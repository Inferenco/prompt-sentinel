use std::fs;
use std::sync::{Arc, RwLock};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::dtos::{
    ComplianceCheckRequest, ComplianceCheckResponse, ComplianceReportRequest, 
    ComplianceReportResponse, ComplianceConfigurationRequest, ComplianceConfigurationResponse,
    ComplianceConfigurationSummary, RiskKeywordCounts, DocumentationRequirements
};
use super::model::{AiRiskTier, ComplianceFinding};

const DEFAULT_EU_KEYWORDS_PATH: &str = "config/eu_risk_keywords.json";
const EU_KEYWORDS_PATH_ENV: &str = "PROMPT_SENTINEL_EU_KEYWORDS_PATH";

const DEFAULT_UNACCEPTABLE_KEYWORDS: &[&str] = &[
    "social scoring",
    "biometric surveillance",
    "biometric categorization",
    "emotion recognition in workplace",
    "emotion recognition in school",
    "manipulative subliminal",
];

const DEFAULT_HIGH_KEYWORDS: &[&str] = &[
    "employment",
    "hiring",
    "education",
    "credit",
    "insurance",
    "critical infrastructure",
    "law enforcement",
    "migration",
    "asylum",
    "border control",
    "justice",
    "judicial",
    "essential public service",
    "medical triage",
];

const DEFAULT_LIMITED_KEYWORDS: &[&str] = &[
    "chatbot",
    "recommendation",
    "generative assistant",
    "customer support bot",
    "deepfake",
];

#[derive(Clone, Debug, Deserialize, Serialize)]
struct EuRiskKeywordConfig {
    #[serde(default = "default_unacceptable_keywords")]
    unacceptable: Vec<String>,
    #[serde(default = "default_high_keywords")]
    high: Vec<String>,
    #[serde(default = "default_limited_keywords")]
    limited: Vec<String>,
}

impl Default for EuRiskKeywordConfig {
    fn default() -> Self {
        Self {
            unacceptable: default_unacceptable_keywords(),
            high: default_high_keywords(),
            limited: default_limited_keywords(),
        }
    }
}

/// Global configuration manager with runtime reloading support
#[derive(Clone, Debug)]
struct ConfigManager {
    config: Arc<RwLock<EuRiskKeywordConfig>>,
}

impl ConfigManager {
    fn new() -> Self {
        let config = load_risk_keywords();
        Self {
            config: Arc::new(RwLock::new(config)),
        }
    }

    fn get_config(&self) -> EuRiskKeywordConfig {
        let guard = self.config.read().unwrap();
        guard.clone()
    }

    fn update_config(&self, new_config: EuRiskKeywordConfig) -> Result<(), std::io::Error> {
        let mut guard = self.config.write().unwrap();
        
        // Save to file first
        save_risk_keywords(&new_config)?;
        
        // Update in-memory config
        *guard = new_config;
        
        Ok(())
    }
}

// Global configuration instance
lazy_static::lazy_static! {
    static ref CONFIG_MANAGER: ConfigManager = ConfigManager::new();
}

#[derive(Clone, Debug, Default)]
pub struct EuLawComplianceService;

impl EuLawComplianceService {
    pub fn check(&self, request: ComplianceCheckRequest) -> ComplianceCheckResponse {
        let intended_use = request.intended_use.trim();
        let risk_tier = classify_risk(intended_use);
        let mut findings = Vec::new();

        if intended_use.len() < 8 {
            findings.push(ComplianceFinding {
                code: "EU-SCOPE-001".to_owned(),
                detail: "Intended-use description is too short for reliable risk classification."
                    .to_owned(),
            });
        }

        if matches!(risk_tier, AiRiskTier::Unacceptable) {
            findings.push(ComplianceFinding {
                code: "EU-RISK-001".to_owned(),
                detail: "Intended use matches a prohibited-risk category under EU AI Act controls."
                    .to_owned(),
            });
        }

        if matches!(risk_tier, AiRiskTier::High | AiRiskTier::Unacceptable) {
            if !request.technical_documentation_available {
                findings.push(ComplianceFinding {
                    code: "EU-DOC-001".to_owned(),
                    detail: "Technical documentation is missing.".to_owned(),
                });
            }
            if !request.transparency_notice_available {
                findings.push(ComplianceFinding {
                    code: "EU-TRN-001".to_owned(),
                    detail: "Transparency notice is missing.".to_owned(),
                });
            }
            if !request.copyright_controls_available {
                findings.push(ComplianceFinding {
                    code: "EU-CPY-001".to_owned(),
                    detail: "Copyright safeguard documentation is missing.".to_owned(),
                });
            }
        } else if matches!(risk_tier, AiRiskTier::Limited) && !request.transparency_notice_available
        {
            findings.push(ComplianceFinding {
                code: "EU-TRN-002".to_owned(),
                detail: "Limited-risk systems must include a transparency notice.".to_owned(),
            });
        }

        let compliant = !matches!(risk_tier, AiRiskTier::Unacceptable) && findings.is_empty();
        ComplianceCheckResponse {
            risk_tier,
            compliant,
            findings,
        }
    }

    pub fn generate_compliance_report(&self, request: ComplianceReportRequest) -> ComplianceReportResponse {
        let check_response = self.check(ComplianceCheckRequest {
            intended_use: request.intended_use,
            technical_documentation_available: true,
            transparency_notice_available: true,
            copyright_controls_available: true,
        });

        ComplianceReportResponse {
            report_id: format!("COMP-REPORT-{}", request.correlation_id),
            risk_tier: check_response.risk_tier,
            compliant: check_response.compliant,
            findings: check_response.findings,
            generated_at: Utc::now(),
            pdf_available: request.generate_pdf,
            pdf_url: if request.generate_pdf {
                Some(format!("/api/compliance/reports/{}/pdf", request.correlation_id))
            } else {
                None
            },
        }
    }

    pub fn get_current_configuration(&self) -> ComplianceConfigurationSummary {
        let keywords = CONFIG_MANAGER.get_config();
        
        ComplianceConfigurationSummary {
            risk_keyword_counts: RiskKeywordCounts {
                unacceptable: keywords.unacceptable.len(),
                high: keywords.high.len(),
                limited: keywords.limited.len(),
            },
            documentation_requirements: DocumentationRequirements {
                technical_documentation_required: Some(true),
                transparency_notice_required: Some(true),
                copyright_controls_required: Some(true),
            },
        }
    }

    pub fn update_configuration(&self, request: ComplianceConfigurationRequest) -> ComplianceConfigurationResponse {
        // Load current configuration
        let current_config = CONFIG_MANAGER.get_config();
        let mut new_config = current_config.clone();
        
        // Apply updates from request
        if let Some(risk_thresholds) = request.risk_thresholds {
            if let Some(keywords) = risk_thresholds.unacceptable_keywords {
                if !keywords.is_empty() {
                    new_config.unacceptable = keywords;
                }
            }
            if let Some(keywords) = risk_thresholds.high_risk_keywords {
                if !keywords.is_empty() {
                    new_config.high = keywords;
                }
            }
            if let Some(keywords) = risk_thresholds.limited_risk_keywords {
                if !keywords.is_empty() {
                    new_config.limited = keywords;
                }
            }
        }
        
        // Save updated configuration to file and memory
        match CONFIG_MANAGER.update_config(new_config) {
            Ok(_) => {
                ComplianceConfigurationResponse {
                    status: "success".to_string(),
                    message: "Configuration updated successfully".to_string(),
                    current_configuration: self.get_current_configuration(),
                }
            }
            Err(e) => {
                ComplianceConfigurationResponse {
                    status: "error".to_string(),
                    message: format!("Failed to update configuration: {}", e),
                    current_configuration: self.get_current_configuration(),
                }
            }
        }
    }
}

fn classify_risk(intended_use: &str) -> AiRiskTier {
    let text = intended_use.to_ascii_lowercase();
    let keywords = CONFIG_MANAGER.get_config();

    if contains_any(&text, &keywords.unacceptable) {
        AiRiskTier::Unacceptable
    } else if contains_any(&text, &keywords.high) {
        AiRiskTier::High
    } else if contains_any(&text, &keywords.limited) {
        AiRiskTier::Limited
    } else {
        AiRiskTier::Minimal
    }
}

fn load_risk_keywords() -> EuRiskKeywordConfig {
    let path =
        std::env::var(EU_KEYWORDS_PATH_ENV).unwrap_or_else(|_| DEFAULT_EU_KEYWORDS_PATH.to_owned());

    fs::read_to_string(path)
        .ok()
        .and_then(|content| serde_json::from_str::<EuRiskKeywordConfig>(&content).ok())
        .unwrap_or_default()
}

fn contains_any(text: &str, keywords: &[String]) -> bool {
    keywords.iter().any(|keyword| text.contains(keyword))
}

fn save_risk_keywords(config: &EuRiskKeywordConfig) -> Result<(), std::io::Error> {
    let path =
        std::env::var(EU_KEYWORDS_PATH_ENV).unwrap_or_else(|_| DEFAULT_EU_KEYWORDS_PATH.to_owned());
    
    // Create directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    
    fs::write(path, content)
}

fn default_unacceptable_keywords() -> Vec<String> {
    DEFAULT_UNACCEPTABLE_KEYWORDS
        .iter()
        .map(|keyword| (*keyword).to_owned())
        .collect()
}

fn default_high_keywords() -> Vec<String> {
    DEFAULT_HIGH_KEYWORDS
        .iter()
        .map(|keyword| (*keyword).to_owned())
        .collect()
}

fn default_limited_keywords() -> Vec<String> {
    DEFAULT_LIMITED_KEYWORDS
        .iter()
        .map(|keyword| (*keyword).to_owned())
        .collect()
}
