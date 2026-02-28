use std::sync::Arc;

use thiserror::Error;

use super::client::{MistralClient, MistralClientError};
use super::dtos::{
    ChatCompletionRequest, ChatCompletionResponse, ChatMessage, EmbeddingRequest,
    EmbeddingResponse, ModerationRequest, ModerationResponse,
};

#[derive(Clone)]
pub struct MistralService {
    client: Arc<dyn MistralClient>,
    generation_model: String,
    moderation_model: Option<String>,
    embedding_model: String,
}

impl MistralService {
    pub fn new(
        client: Arc<dyn MistralClient>,
        generation_model: impl Into<String>,
        moderation_model: Option<String>,
        embedding_model: impl Into<String>,
    ) -> Self {
        Self {
            client,
            generation_model: generation_model.into(),
            moderation_model,
            embedding_model: embedding_model.into(),
        }
    }

    pub async fn validate_generation_model(&self) -> Result<(), MistralServiceError> {
        let models = self.client.list_models().await?;
        if models
            .models
            .iter()
            .any(|model| model == &self.generation_model)
        {
            return Ok(());
        }
        Err(MistralServiceError::UnknownModel(
            self.generation_model.clone(),
        ))
    }

    pub async fn moderate_text(
        &self,
        input: impl Into<String>,
    ) -> Result<ModerationResponse, MistralServiceError> {
        let request = ModerationRequest {
            model: self.moderation_model.clone(),
            input: input.into(),
        };
        self.client.moderate(request).await.map_err(Into::into)
    }

    pub async fn generate_text(
        &self,
        prompt: impl Into<String>,
        safe_prompt: bool,
    ) -> Result<ChatCompletionResponse, MistralServiceError> {
        let request = ChatCompletionRequest {
            model: self.generation_model.clone(),
            messages: vec![ChatMessage {
                role: "user".to_owned(),
                content: prompt.into(),
            }],
            safe_prompt,
        };
        self.client
            .chat_completion(request)
            .await
            .map_err(Into::into)
    }

    pub async fn embed_text(
        &self,
        text: impl Into<String>,
    ) -> Result<EmbeddingResponse, MistralServiceError> {
        let request = EmbeddingRequest {
            model: self.embedding_model.clone(),
            input: text.into(),
        };
        self.client.embeddings(request).await.map_err(Into::into)
    }
}

#[derive(Debug, Error)]
pub enum MistralServiceError {
    #[error("mistral client error: {0}")]
    Client(#[from] MistralClientError),
    #[error("configured generation model is unavailable: {0}")]
    UnknownModel(String),
}
