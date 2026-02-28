use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sled::Db;
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
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("serialization error: {0}")]
    SerializationError(String),
}

#[derive(Clone)]
pub struct SledAuditStorage {
    db: Db,
}

impl SledAuditStorage {
    pub fn new(db_path: &str) -> Result<Self, AuditStorageError> {
        let db =
            sled::open(db_path).map_err(|e| AuditStorageError::DatabaseError(e.to_string()))?;
        Ok(Self { db })
    }
}

impl AuditStorage for SledAuditStorage {
    fn append(&self, record: StoredAuditRecord) -> Result<(), AuditStorageError> {
        let serialized = serde_json::to_string(&record)
            .map_err(|e| AuditStorageError::SerializationError(e.to_string()))?;

        // Use timestamp-prefixed key for chronological ordering
        // Format: {timestamp_nanos}_{correlation_id}
        let key = format!(
            "{:020}_{}",
            record.timestamp.timestamp_nanos_opt().unwrap_or(0),
            record.correlation_id
        );
        self.db
            .insert(key, serialized.as_bytes())
            .map_err(|e| AuditStorageError::DatabaseError(e.to_string()))?;

        self.db
            .flush()
            .map_err(|e| AuditStorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    fn latest_chain_hash(&self) -> Result<Option<String>, AuditStorageError> {
        // Iterate in reverse to get the chronologically latest record
        let last_record = self
            .db
            .iter()
            .next_back()
            .transpose()
            .map_err(|e| AuditStorageError::DatabaseError(e.to_string()))?;

        match last_record {
            Some((_, data)) => {
                let record: StoredAuditRecord = serde_json::from_slice(&data)
                    .map_err(|e| AuditStorageError::SerializationError(e.to_string()))?;
                Ok(Some(record.proof.chain_hash))
            }
            None => Ok(None),
        }
    }

    fn all(&self) -> Result<Vec<StoredAuditRecord>, AuditStorageError> {
        let mut records = Vec::new();

        for result in self.db.iter() {
            let (_, data) = result.map_err(|e| AuditStorageError::DatabaseError(e.to_string()))?;
            let record: StoredAuditRecord = serde_json::from_slice(&data)
                .map_err(|e| AuditStorageError::SerializationError(e.to_string()))?;
            records.push(record);
        }

        Ok(records)
    }
}
