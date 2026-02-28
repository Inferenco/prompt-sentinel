use serde::{Deserialize, Serialize};

use super::model::{AiRiskTier, ComplianceFinding};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ComplianceCheckRequest {
    pub intended_use: String,
    pub technical_documentation_available: bool,
    pub transparency_notice_available: bool,
    pub copyright_controls_available: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ComplianceCheckResponse {
    pub risk_tier: AiRiskTier,
    pub compliant: bool,
    pub findings: Vec<ComplianceFinding>,
}
