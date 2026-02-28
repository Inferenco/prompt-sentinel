use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::proof::AuditProof;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoredAuditRecord {
    pub correlation_id: String,
    pub timestamp: DateTime<Utc>,
    pub payload: String,
    pub proof: AuditProof,
}

pub trait AuditStorage: Send + Sync {
    fn append(&self, record: StoredAuditRecord) -> Result<(), AuditStorageError>;
    fn latest_chain_hash(&self) -> Result<Option<String>, AuditStorageError>;
    fn all(&self) -> Result<Vec<StoredAuditRecord>, AuditStorageError>;
}

#[derive(Clone, Default)]
pub struct InMemoryAuditStorage {
    inner: Arc<Mutex<Vec<StoredAuditRecord>>>,
}

impl InMemoryAuditStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AuditStorage for InMemoryAuditStorage {
    fn append(&self, record: StoredAuditRecord) -> Result<(), AuditStorageError> {
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| AuditStorageError::LockPoisoned)?;
        guard.push(record);
        Ok(())
    }

    fn latest_chain_hash(&self) -> Result<Option<String>, AuditStorageError> {
        let guard = self
            .inner
            .lock()
            .map_err(|_| AuditStorageError::LockPoisoned)?;
        Ok(guard.last().map(|entry| entry.proof.chain_hash.clone()))
    }

    fn all(&self) -> Result<Vec<StoredAuditRecord>, AuditStorageError> {
        let guard = self
            .inner
            .lock()
            .map_err(|_| AuditStorageError::LockPoisoned)?;
        Ok(guard.clone())
    }
}

#[derive(Debug, Error)]
pub enum AuditStorageError {
    #[error("audit storage lock poisoned")]
    LockPoisoned,
}
