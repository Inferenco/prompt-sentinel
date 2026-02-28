use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use thiserror::Error;

use super::dtos::{
    ChatCompletionRequest, ChatCompletionResponse, EmbeddingRequest, EmbeddingResponse,
    ModelListResponse, ModerationRequest, ModerationResponse,
};

#[async_trait]
pub trait MistralClient: Send + Sync {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, MistralClientError>;
    async fn moderate(
        &self,
        request: ModerationRequest,
    ) -> Result<ModerationResponse, MistralClientError>;
    async fn embeddings(
        &self,
        request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, MistralClientError>;
    async fn list_models(&self) -> Result<ModelListResponse, MistralClientError>;
}

#[derive(Clone)]
pub struct HttpMistralClient {
    http: Client,
    base_url: String,
    api_key: String,
}

impl HttpMistralClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.into(),
            api_key: api_key.into(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }
}

#[async_trait]
impl MistralClient for HttpMistralClient {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, MistralClientError> {
        let response = self
            .http
            .post(self.url("/v1/chat/completions"))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;

        let json: Value = response.json().await?;
        let output_text = extract_content(&json)?;
        let model = json
            .get("model")
            .and_then(Value::as_str)
            .unwrap_or(request.model.as_str())
            .to_owned();

        Ok(ChatCompletionResponse { model, output_text })
    }

    async fn moderate(
        &self,
        request: ModerationRequest,
    ) -> Result<ModerationResponse, MistralClientError> {
        let response = self
            .http
            .post(self.url("/v1/moderations"))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;

        let json: Value = response.json().await?;
        let result = json
            .get("results")
            .and_then(Value::as_array)
            .and_then(|results| results.first())
            .ok_or_else(|| {
                MistralClientError::InvalidResponse("missing moderation results".to_owned())
            })?;

        let flagged = result
            .get("flagged")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let mut categories = Vec::new();
        if let Some(map) = result.get("categories").and_then(Value::as_object) {
            for (category, value) in map {
                if value.as_bool().unwrap_or(false) {
                    categories.push(category.clone());
                }
            }
        }

        let severity = if flagged {
            (categories.len() as f32 / 5.0).min(1.0)
        } else {
            0.0
        };

        Ok(ModerationResponse {
            flagged,
            categories,
            severity,
        })
    }

    async fn embeddings(
        &self,
        request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, MistralClientError> {
        let response = self
            .http
            .post(self.url("/v1/embeddings"))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;

        let json: Value = response.json().await?;
        let vector_values = json
            .get("data")
            .and_then(Value::as_array)
            .and_then(|data| data.first())
            .and_then(|item| item.get("embedding"))
            .and_then(Value::as_array)
            .ok_or_else(|| {
                MistralClientError::InvalidResponse("missing embedding vector".to_owned())
            })?;

        let vector = vector_values
            .iter()
            .map(|value| value.as_f64().unwrap_or_default() as f32)
            .collect::<Vec<_>>();

        Ok(EmbeddingResponse {
            model: request.model,
            vector,
        })
    }

    async fn list_models(&self) -> Result<ModelListResponse, MistralClientError> {
        let response = self
            .http
            .get(self.url("/v1/models"))
            .bearer_auth(&self.api_key)
            .send()
            .await?
            .error_for_status()?;

        let json: Value = response.json().await?;
        let models = json
            .get("data")
            .and_then(Value::as_array)
            .ok_or_else(|| MistralClientError::InvalidResponse("missing model list".to_owned()))?
            .iter()
            .filter_map(|model| model.get("id").and_then(Value::as_str))
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();

        Ok(ModelListResponse { models })
    }
}

#[derive(Clone, Debug)]
pub struct MockMistralClient {
    chat_response: ChatCompletionResponse,
    moderation_responses: Arc<Mutex<Vec<ModerationResponse>>>,
    embedding_response: EmbeddingResponse,
    models: Vec<String>,
}

impl Default for MockMistralClient {
    fn default() -> Self {
        Self {
            chat_response: ChatCompletionResponse {
                model: "mistral-large-latest".to_owned(),
                output_text: "Mock response".to_owned(),
            },
            moderation_responses: Arc::new(Mutex::new(vec![
                ModerationResponse {
                    flagged: false,
                    categories: Vec::new(),
                    severity: 0.0,
                },
                ModerationResponse {
                    flagged: false,
                    categories: Vec::new(),
                    severity: 0.0,
                },
            ])),
            embedding_response: EmbeddingResponse {
                model: "mistral-embed".to_owned(),
                vector: vec![0.1, 0.2, 0.3],
            },
            models: vec![
                "mistral-large-latest".to_owned(),
                "mistral-embed".to_owned(),
            ],
        }
    }
}

impl MockMistralClient {
    pub fn with_moderation_sequence(
        sequence: Vec<ModerationResponse>,
    ) -> Result<Self, MistralClientError> {
        if sequence.is_empty() {
            return Err(MistralClientError::InvalidResponse(
                "moderation sequence cannot be empty".to_owned(),
            ));
        }
        let mut client = Self::default();
        client.moderation_responses = Arc::new(Mutex::new(sequence));
        Ok(client)
    }

    pub fn with_chat_response(mut self, response: ChatCompletionResponse) -> Self {
        self.chat_response = response;
        self
    }
}

#[async_trait]
impl MistralClient for MockMistralClient {
    async fn chat_completion(
        &self,
        _request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, MistralClientError> {
        Ok(self.chat_response.clone())
    }

    async fn moderate(
        &self,
        _request: ModerationRequest,
    ) -> Result<ModerationResponse, MistralClientError> {
        let mut guard = self.moderation_responses.lock().map_err(|_| {
            MistralClientError::InvalidResponse("moderation queue poisoned".to_owned())
        })?;

        if guard.len() > 1 {
            Ok(guard.remove(0))
        } else {
            Ok(guard[0].clone())
        }
    }

    async fn embeddings(
        &self,
        _request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, MistralClientError> {
        Ok(self.embedding_response.clone())
    }

    async fn list_models(&self) -> Result<ModelListResponse, MistralClientError> {
        Ok(ModelListResponse {
            models: self.models.clone(),
        })
    }
}

fn extract_content(response: &Value) -> Result<String, MistralClientError> {
    let message_content = response
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|choices| choices.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .ok_or_else(|| {
            MistralClientError::InvalidResponse("missing response content".to_owned())
        })?;

    if let Some(content) = message_content.as_str() {
        return Ok(content.to_owned());
    }

    if let Some(items) = message_content.as_array() {
        let combined = items
            .iter()
            .filter_map(|item| item.get("text").and_then(Value::as_str))
            .collect::<Vec<_>>()
            .join("\n");
        if !combined.is_empty() {
            return Ok(combined);
        }
    }

    Err(MistralClientError::InvalidResponse(
        "unsupported response content shape".to_owned(),
    ))
}

#[derive(Debug, Error)]
pub enum MistralClientError {
    #[error("mistral request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("mistral response contract invalid: {0}")]
    InvalidResponse(String),
}
