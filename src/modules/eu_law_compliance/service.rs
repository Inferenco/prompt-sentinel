use super::dtos::{ComplianceCheckRequest, ComplianceCheckResponse};
use super::model::{AiRiskTier, ComplianceFinding};

#[derive(Clone, Debug, Default)]
pub struct EuLawComplianceService;

impl EuLawComplianceService {
    pub fn check(&self, request: ComplianceCheckRequest) -> ComplianceCheckResponse {
        let risk_tier = classify_risk(&request.intended_use);
        let mut findings = Vec::new();

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
    if text.contains("social scoring") || text.contains("biometric surveillance") {
        AiRiskTier::Unacceptable
    } else if text.contains("employment")
        || text.contains("education")
        || text.contains("credit")
        || text.contains("law enforcement")
    {
        AiRiskTier::High
    } else if text.contains("chatbot") || text.contains("recommendation") {
        AiRiskTier::Limited
    } else {
        AiRiskTier::Minimal
    }
}
