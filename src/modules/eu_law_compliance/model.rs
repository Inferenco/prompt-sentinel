use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AiRiskTier {
    Minimal,
    Limited,
    High,
    Unacceptable,
}

impl AiRiskTier {
    /// Returns the applicable EU AI Act articles for this risk tier
    pub fn applicable_articles(&self) -> Vec<&'static str> {
        match self {
            AiRiskTier::Unacceptable => vec!["Article 5 (Prohibited AI Practices)"],
            AiRiskTier::High => vec![
                "Article 6 (Classification Rules)",
                "Article 9 (Risk Management)",
                "Article 10 (Data Governance)",
                "Article 11 (Technical Documentation)",
                "Article 13 (Transparency)",
                "Article 14 (Human Oversight)",
            ],
            AiRiskTier::Limited => vec!["Article 50 (Transparency Obligations)"],
            AiRiskTier::Minimal => vec!["Article 95 (Voluntary Codes of Conduct)"],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ComplianceFinding {
    pub code: String,
    pub detail: String,
}

/// Compliance status for individual obligations
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ObligationStatus {
    /// Requirement fully satisfied
    Met,
    /// Requirement partially satisfied, action needed
    Partial,
    /// Requirement not satisfied, blocking gap
    Gap,
    /// Not applicable to this risk tier
    NotApplicable,
}

/// Individual obligation with status and legal basis
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ObligationResult {
    /// Unique identifier for this obligation
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Legal basis (EU AI Act article reference)
    pub legal_basis: String,
    /// Current compliance status
    pub status: ObligationStatus,
    /// Detailed explanation
    pub detail: Option<String>,
    /// Applicable date (ISO 8601 format)
    pub applicable_from: Option<String>,
}

/// Structured EU AI Act compliance result
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct EuComplianceResult {
    /// Classified risk tier
    pub risk_tier: AiRiskTier,
    /// Whether the use case is compliant overall
    pub compliant: bool,
    /// Individual obligation statuses
    pub obligations: Vec<ObligationResult>,
    /// Legacy findings for backward compatibility
    pub findings: Vec<ComplianceFinding>,
    /// Scope limitation disclaimer
    pub scope_disclaimer: String,
}
