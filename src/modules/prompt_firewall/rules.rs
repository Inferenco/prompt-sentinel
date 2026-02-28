use super::dtos::{FirewallAction, FirewallSeverity, PromptFirewallResult};

const BLOCK_RULES: &[(&str, &str)] = &[
    ("PFW-001", "ignore previous instructions"),
    ("PFW-002", "reveal system prompt"),
    ("PFW-003", "developer instructions"),
    ("PFW-004", "bypass policy"),
    ("PFW-005", "jailbreak"),
];

const SANITIZE_PATTERNS: &[(&str, &str)] = &[
    ("PFW-SAN-001", "```"),
    ("PFW-SAN-002", "<script"),
    ("PFW-SAN-003", "</script>"),
];

pub fn evaluate(prompt: &str, max_input_length: usize) -> PromptFirewallResult {
    if prompt.len() > max_input_length {
        return PromptFirewallResult {
            action: FirewallAction::Block,
            severity: FirewallSeverity::High,
            sanitized_prompt: prompt.chars().take(max_input_length).collect(),
            reasons: vec![format!(
                "input length exceeds configured max ({max_input_length})"
            )],
            matched_rules: vec!["PFW-LENGTH".to_owned()],
        };
    }

    let normalized = prompt.to_ascii_lowercase();
    let mut matched_rules = Vec::new();
    let mut reasons = Vec::new();
    for &(rule_id, pattern) in BLOCK_RULES {
        if normalized.contains(pattern) {
            matched_rules.push(rule_id.to_owned());
            reasons.push(format!("matched high-risk injection pattern: {pattern}"));
        }
    }

    if !matched_rules.is_empty() {
        return PromptFirewallResult {
            action: FirewallAction::Block,
            severity: FirewallSeverity::Critical,
            sanitized_prompt: prompt.to_owned(),
            reasons,
            matched_rules,
        };
    }

    let mut sanitized_prompt = prompt.to_owned();
    for &(_, pattern) in SANITIZE_PATTERNS {
        if sanitized_prompt.to_ascii_lowercase().contains(pattern) {
            sanitized_prompt = sanitized_prompt.replace(pattern, "");
            sanitized_prompt = sanitized_prompt.replace(&pattern.to_ascii_uppercase(), "");
        }
    }

    if sanitized_prompt != prompt {
        let sanitize_rule_ids = SANITIZE_PATTERNS
            .iter()
            .map(|(rule_id, _)| (*rule_id).to_owned())
            .collect::<Vec<_>>();

        return PromptFirewallResult {
            action: FirewallAction::Sanitize,
            severity: FirewallSeverity::Medium,
            sanitized_prompt: sanitized_prompt.trim().to_owned(),
            reasons: vec!["removed suspicious formatting or HTML/script markers".to_owned()],
            matched_rules: sanitize_rule_ids,
        };
    }

    PromptFirewallResult {
        action: FirewallAction::Allow,
        severity: FirewallSeverity::Low,
        sanitized_prompt: prompt.trim().to_owned(),
        reasons: vec!["prompt passed static firewall checks".to_owned()],
        matched_rules: Vec::new(),
    }
}
