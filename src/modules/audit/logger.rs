use std::sync::Arc;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::proof::{AuditProof, chain_hash, hash_record};
use super::storage::{AuditStorage, AuditStorageError, StoredAuditRecord};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditEvent {
    pub correlation_id: String,
    pub original_prompt: String,
    pub sanitized_prompt: String,
    pub firewall_action: String,
    pub firewall_reasons: Vec<String>,
    /// Semantic risk score (0.0 - 1.0)
    pub semantic_risk_score: Option<f32>,
    /// ID of matched attack template
    pub semantic_template_id: Option<String>,
    /// Category of matched attack template
    pub semantic_category: Option<String>,
    pub bias_score: f32,
    pub bias_level: String,
    pub input_moderation_flagged: bool,
    pub output_moderation_flagged: bool,
    pub final_status: String,
    /// Human-readable explanation of the decision
    pub final_reason: String,
    pub model_used: Option<String>,
    pub output_preview: Option<String>,
}

#[derive(Clone)]
pub struct AuditLogger {
    storage: Arc<dyn AuditStorage>,
}

impl AuditLogger {
    pub fn new(storage: Arc<dyn AuditStorage>) -> Self {
        Self { storage }
    }

    pub fn log_event(&self, event: AuditEvent) -> Result<AuditProof, AuditError> {
        let payload = serde_json::to_string(&event)?;
        let record_hash = hash_record(&payload);
        let previous_chain = self.storage.latest_chain_hash()?;
        let chain_hash = chain_hash(previous_chain.as_deref(), &record_hash);

        let proof = AuditProof {
            algorithm: "sha256".to_owned(),
            record_hash,
            chain_hash,
        };

        let record = StoredAuditRecord {
            correlation_id: event.correlation_id,
            timestamp: Utc::now(),
            payload,
            proof: proof.clone(),
        };
        self.storage.append(record)?;

        Ok(proof)
    }

    pub fn records(&self) -> Result<Vec<StoredAuditRecord>, AuditError> {
        self.storage.all().map_err(Into::into)
    }

    pub fn storage(&self) -> &Arc<dyn AuditStorage> {
        &self.storage
    }
}

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("failed to serialize audit event: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("audit storage failure: {0}")]
    Storage(#[from] AuditStorageError),
}
