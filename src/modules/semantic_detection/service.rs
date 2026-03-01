use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;
use tracing::{debug, error, info, warn};

use crate::modules::mistral_ai::service::{MistralService, MistralServiceError};
use super::dtos::{
    AttackTemplate, AttackTemplateBank, CachedTemplate, SemanticRiskLevel, SemanticScanRequest,
    SemanticScanResult,
};

#[derive(Clone)]
pub struct SemanticDetectionService {
    mistral_service: MistralService,
    cached_templates: Arc<RwLock<Vec<CachedTemplate>>>,
    initialized: Arc<RwLock<bool>>,
    /// Threshold for Low/Medium boundary
    medium_threshold: f32,
    /// Threshold for Medium/High boundary
    high_threshold: f32,
}

impl SemanticDetectionService {
    pub fn new(mistral_service: MistralService, medium_threshold: f32, high_threshold: f32) -> Self {
        Self {
            mistral_service,
            cached_templates: Arc::new(RwLock::new(Vec::new())),
            initialized: Arc::new(RwLock::new(false)),
            medium_threshold,
            high_threshold,
        }
    }

    /// Initialize the service by loading templates and computing embeddings
    pub async fn initialize(&self) -> Result<(), SemanticDetectionError> {
        let templates = self.load_templates()?;
        info!("Loaded {} attack templates from bank", templates.len());

        let mut cached = Vec::with_capacity(templates.len());
        for template in templates {
            debug!("Computing embedding for template {}", template.id);
            let embedding = self.compute_embedding(&template.text).await?;
            cached.push(CachedTemplate {
                id: template.id,
                category: template.category,
                text: template.text,
                embedding,
            });
        }

        let mut cache = self.cached_templates.write().await;
        *cache = cached;
        let mut init = self.initialized.write().await;
        *init = true;

        info!("Semantic detection service initialized with {} templates", cache.len());
        Ok(())
    }

    /// Check if service is initialized
    pub async fn is_initialized(&self) -> bool {
        *self.initialized.read().await
    }

    /// Scan text for semantic similarity to attack templates
    pub async fn scan(&self, request: SemanticScanRequest) -> Result<SemanticScanResult, SemanticDetectionError> {
        if !self.is_initialized().await {
            warn!("Semantic detection service not initialized, returning low risk");
            return Ok(SemanticScanResult::low_risk());
        }

        // Translate to English if needed for semantic analysis
        let text_to_analyze = self.translate_if_needed(&request.text).await;

        let input_embedding = self.compute_embedding(&text_to_analyze).await?;
        let cache = self.cached_templates.read().await;

        if cache.is_empty() {
            debug!("No templates cached, returning low risk");
            return Ok(SemanticScanResult::low_risk());
        }

        // Find highest similarity match
        let mut best_match: Option<(&CachedTemplate, f32)> = None;
        for template in cache.iter() {
            let similarity = cosine_similarity(&input_embedding, &template.embedding);
            if best_match.is_none() || similarity > best_match.unwrap().1 {
                best_match = Some((template, similarity));
            }
        }

        let (template, similarity) = best_match.unwrap();
        let risk_level = self.classify_risk(similarity);
        let risk_score = similarity;

        debug!(
            "Semantic scan: similarity={:.3}, template={}, category={}, risk={:?}",
            similarity, template.id, template.category, risk_level
        );

        Ok(SemanticScanResult {
            risk_score,
            risk_level,
            nearest_template_id: Some(template.id.clone()),
            similarity,
            category: Some(template.category.clone()),
        })
    }

    fn load_templates(&self) -> Result<Vec<AttackTemplate>, SemanticDetectionError> {
        let config_path = std::env::var("SEMANTIC_ATTACK_BANK_PATH")
            .unwrap_or_else(|_| "config/semantic_attack_bank.json".to_string());

        let path = Path::new(&config_path);
        if !path.exists() {
            error!("Attack template bank not found at {:?}", path);
            return Err(SemanticDetectionError::ConfigNotFound(config_path));
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| SemanticDetectionError::IoError(e.to_string()))?;

        let bank: AttackTemplateBank = serde_json::from_str(&content)
            .map_err(|e| SemanticDetectionError::ParseError(e.to_string()))?;

        Ok(bank.templates)
    }

    async fn compute_embedding(&self, text: &str) -> Result<Vec<f32>, SemanticDetectionError> {
        let response = self.mistral_service.embed_text(text).await?;
        Ok(response.vector)
    }

    /// Classify risk level based on similarity score using configured thresholds
    fn classify_risk(&self, similarity: f32) -> SemanticRiskLevel {
        if similarity > self.high_threshold {
            SemanticRiskLevel::High
        } else if similarity > self.medium_threshold {
            SemanticRiskLevel::Medium
        } else {
            SemanticRiskLevel::Low
        }
    }

    async fn translate_if_needed(&self, text: &str) -> String {
        // Always translate to English for consistent semantic analysis
        let Ok(translation) = self.mistral_service
            .translate_text(text.to_owned(), "English")
            .await
        else {
            return text.to_owned();
        };
        
        translation.translated_text
    }
}

/// Compute cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}

#[derive(Debug, Error)]
pub enum SemanticDetectionError {
    #[error("Attack template bank not found: {0}")]
    ConfigNotFound(String),
    #[error("Failed to read config: {0}")]
    IoError(String),
    #[error("Failed to parse config: {0}")]
    ParseError(String),
    #[error("Embedding service error: {0}")]
    Embedding(#[from] MistralServiceError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let a = vec![1.0, 0.0];
        let b = vec![-1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - (-1.0)).abs() < 0.0001);
    }
}
