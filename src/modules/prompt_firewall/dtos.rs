use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PromptFirewallRequest {
    pub prompt: String,
    pub correlation_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum FirewallAction {
    Allow,
    Sanitize,
    Block,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FirewallSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PromptFirewallResult {
    pub action: FirewallAction,
    pub severity: FirewallSeverity,
    pub sanitized_prompt: String,
    pub reasons: Vec<String>,
    pub matched_rules: Vec<String>,
}
