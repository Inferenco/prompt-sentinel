use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub safe_prompt: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LanguageDetectionRequest {
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LanguageDetectionResponse {
    pub language: String,
    pub confidence: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TranslationRequest {
    pub text: String,
    pub target_language: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TranslationResponse {
    pub translated_text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ChatCompletionResponse {
    pub model: String,
    pub output_text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ModerationRequest {
    pub model: Option<String>,
    pub input: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ModerationResponse {
    pub flagged: bool,
    pub categories: Vec<String>,
    pub severity: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct EmbeddingResponse {
    pub model: String,
    pub vector: Vec<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModelListResponse {
    pub models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelValidationResponse {
    pub generation_model: ModelValidationStatus,
    pub moderation_model: Option<ModelValidationStatus>,
    pub embedding_model: ModelValidationStatus,
    pub overall_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelValidationStatus {
    pub model_name: String,
    pub available: bool,
    pub message: String,
}
