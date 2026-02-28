use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AiRiskTier {
    Minimal,
    Limited,
    High,
    Unacceptable,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ComplianceFinding {
    pub code: String,
    pub detail: String,
}
