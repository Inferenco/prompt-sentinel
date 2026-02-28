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
