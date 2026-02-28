use super::dtos::{ChatCompletionResponse, ModerationResponse};
use super::service::{MistralService, MistralServiceError};

pub async fn handle_moderation(
    service: &MistralService,
    input: impl Into<String>,
) -> Result<ModerationResponse, MistralServiceError> {
    service.moderate_text(input).await
}

pub async fn handle_chat_completion(
    service: &MistralService,
    prompt: impl Into<String>,
) -> Result<ChatCompletionResponse, MistralServiceError> {
    service.generate_text(prompt, true).await
}
