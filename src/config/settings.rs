use std::env;
use std::num::ParseFloatError;
use std::num::ParseIntError;

use thiserror::Error;

#[derive(Clone, Debug)]
pub struct AppSettings {
    pub mistral_api_key: Option<String>,
    pub mistral_base_url: String,
    pub generation_model: String,
    pub moderation_model: Option<String>,
    pub embedding_model: String,
    pub bias_threshold: f32,
    pub max_input_length: usize,
}

impl AppSettings {
    pub fn from_env() -> Result<Self, SettingsError> {
        let bias_threshold = parse_env_f32("BIAS_THRESHOLD", 0.35)?;
        let max_input_length = parse_env_usize("MAX_INPUT_LENGTH", 4096)?;

        Ok(Self {
            mistral_api_key: env::var("MISTRAL_API_KEY").ok().filter(|v| !v.is_empty()),
            mistral_base_url: env::var("MISTRAL_BASE_URL")
                .unwrap_or_else(|_| "https://api.mistral.ai".to_owned()),
            generation_model: env::var("MISTRAL_GENERATION_MODEL")
                .unwrap_or_else(|_| "mistral-large-latest".to_owned()),
            moderation_model: env::var("MISTRAL_MODERATION_MODEL")
                .ok()
                .filter(|v| !v.is_empty()),
            embedding_model: env::var("MISTRAL_EMBEDDING_MODEL")
                .unwrap_or_else(|_| "mistral-embed".to_owned()),
            bias_threshold,
            max_input_length,
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
