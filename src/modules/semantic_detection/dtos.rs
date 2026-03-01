use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct SemanticScanRequest {
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SemanticScanResult {
    /// Risk score from 0.0 to 1.0
    pub risk_score: f32,
    /// Risk level classification
    pub risk_level: SemanticRiskLevel,
    /// ID of the nearest matching attack template
    pub nearest_template_id: Option<String>,
    /// Cosine similarity to the nearest template
    pub similarity: f32,
    /// Category of the matched attack template
    pub category: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum SemanticRiskLevel {
    Low,
    Medium,
    High,
}

impl SemanticScanResult {
    pub fn low_risk() -> Self {
        Self {
            risk_score: 0.0,
            risk_level: SemanticRiskLevel::Low,
            nearest_template_id: None,
            similarity: 0.0,
            category: None,
        }
    }
}

/// Attack template loaded from JSON
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttackTemplate {
    pub id: String,
    pub category: String,
    pub text: String,
}

/// Template bank schema
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttackTemplateBank {
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    pub templates: Vec<AttackTemplate>,
}

/// Cached template with pre-computed embedding
#[derive(Clone, Debug)]
pub struct CachedTemplate {
    pub id: String,
    pub category: String,
    pub text: String,
    pub embedding: Vec<f32>,
}
