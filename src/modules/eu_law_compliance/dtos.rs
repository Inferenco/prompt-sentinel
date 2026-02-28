use chrono::{DateTime, Utc};
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ComplianceReportRequest {
    pub intended_use: String,
    pub request_timestamp: DateTime<Utc>,
    pub correlation_id: String,
    pub generate_pdf: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ComplianceReportResponse {
    pub report_id: String,
    pub risk_tier: AiRiskTier,
    pub compliant: bool,
    pub findings: Vec<ComplianceFinding>,
    pub generated_at: DateTime<Utc>,
    pub pdf_available: bool,
    pub pdf_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ComplianceConfigurationRequest {
    pub risk_thresholds: Option<RiskThresholds>,
    pub documentation_requirements: Option<DocumentationRequirements>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct RiskThresholds {
    pub unacceptable_keywords: Option<Vec<String>>,
    pub high_risk_keywords: Option<Vec<String>>,
    pub limited_risk_keywords: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DocumentationRequirements {
    pub technical_documentation_required: Option<bool>,
    pub transparency_notice_required: Option<bool>,
    pub copyright_controls_required: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ComplianceConfigurationResponse {
    pub status: String,
    pub message: String,
    pub current_configuration: ComplianceConfigurationSummary,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ComplianceConfigurationSummary {
    pub risk_keyword_counts: RiskKeywordCounts,
    pub documentation_requirements: DocumentationRequirements,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct RiskKeywordCounts {
    pub unacceptable: usize,
    pub high: usize,
    pub limited: usize,
}
