use super::dtos::{PromptFirewallRequest, PromptFirewallResult};
use super::service::PromptFirewallService;

pub fn handle_prompt(
    service: &PromptFirewallService,
    prompt: impl Into<String>,
    correlation_id: Option<String>,
) -> PromptFirewallResult {
    service.inspect(PromptFirewallRequest {
        prompt: prompt.into(),
        correlation_id,
    })
}
