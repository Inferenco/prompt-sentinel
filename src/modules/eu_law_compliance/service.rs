use std::fs;
use std::sync::LazyLock;

use serde::Deserialize;

use super::dtos::{ComplianceCheckRequest, ComplianceCheckResponse};
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

#[derive(Clone, Debug, Deserialize)]
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

static EU_RISK_KEYWORDS: LazyLock<EuRiskKeywordConfig> = LazyLock::new(load_risk_keywords);

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
}

fn classify_risk(intended_use: &str) -> AiRiskTier {
    let text = intended_use.to_ascii_lowercase();
    let keywords = &*EU_RISK_KEYWORDS;

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
