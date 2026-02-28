use serde::{Deserialize, Serialize};

use super::model::{BiasCategory, BiasLevel};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BiasScanRequest {
    pub text: String,
    pub threshold: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BiasScanResult {
    pub score: f32,
    pub level: BiasLevel,
    pub categories: Vec<BiasCategory>,
    pub matched_terms: Vec<String>,
    pub mitigation_hints: Vec<String>,
}
