use std::env;
use std::num::ParseFloatError;
use std::num::ParseIntError;

use thiserror::Error;

#[derive(Clone, Debug)]
pub struct AppSettings {
    pub server_port: u16,
    pub mistral_api_key: Option<String>,
    pub mistral_base_url: String,
    pub generation_model: String,
    pub moderation_model: Option<String>,
    pub embedding_model: String,
    pub bias_threshold: f32,
    pub max_input_length: usize,
    /// Threshold for semantic Low/Medium boundary (default: 0.70)
    pub semantic_medium_threshold: f32,
    /// Threshold for semantic Medium/High boundary (default: 0.80)
    pub semantic_high_threshold: f32,
}

impl AppSettings {
    pub fn from_env() -> Result<Self, SettingsError> {
        let server_port = parse_env_u16("SERVER_PORT", 3000)?;
        let bias_threshold = parse_env_f32("BIAS_THRESHOLD", 0.35)?;
        let max_input_length = parse_env_usize("MAX_INPUT_LENGTH", 4096)?;
        let semantic_medium_threshold = parse_env_f32("SEMANTIC_MEDIUM_THRESHOLD", 0.70)?;
        let semantic_high_threshold = parse_env_f32("SEMANTIC_HIGH_THRESHOLD", 0.80)?;

        Ok(Self {
            server_port,
            mistral_api_key: env::var("MISTRAL_API_KEY").ok().filter(|v| !v.is_empty()),
            mistral_base_url: env::var("MISTRAL_BASE_URL")
                .unwrap_or_else(|_| "https://api.mistral.ai".to_owned()),
            generation_model: env::var("MISTRAL_GENERATION_MODEL")
                .unwrap_or_else(|_| "mistral-large-latest".to_owned()),
            moderation_model: Some(env::var("MISTRAL_MODERATION_MODEL")
                .unwrap_or_else(|_| "mistral-moderation-latest".to_owned())),
            embedding_model: env::var("MISTRAL_EMBEDDING_MODEL")
                .unwrap_or_else(|_| "mistral-embed".to_owned()),
            bias_threshold,
            max_input_length,
            semantic_medium_threshold,
            semantic_high_threshold,
        })
    }
}

fn parse_env_f32(key: &str, default: f32) -> Result<f32, SettingsError> {
    match env::var(key) {
        Ok(value) => value
            .parse::<f32>()
            .map_err(|source| SettingsError::ParseFloat {
                key: key.to_owned(),
                source,
            }),
        Err(_) => Ok(default),
    }
}

fn parse_env_usize(key: &str, default: usize) -> Result<usize, SettingsError> {
    match env::var(key) {
        Ok(value) => value
            .parse::<usize>()
            .map_err(|source| SettingsError::ParseInt {
                key: key.to_owned(),
                source,
            }),
        Err(_) => Ok(default),
    }
}

fn parse_env_u16(key: &str, default: u16) -> Result<u16, SettingsError> {
    match env::var(key) {
        Ok(value) => value
            .parse::<u16>()
            .map_err(|source| SettingsError::ParseInt {
                key: key.to_owned(),
                source,
            }),
        Err(_) => Ok(default),
    }
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("failed to parse floating-point setting {key}: {source}")]
    ParseFloat {
        key: String,
        source: ParseFloatError,
    },
    #[error("failed to parse integer setting {key}: {source}")]
    ParseInt { key: String, source: ParseIntError },
}
