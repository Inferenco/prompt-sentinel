use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sled::Db;
use thiserror::Error;

use super::proof::AuditProof;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrailRequest {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub correlation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrailResponse {
    pub records: Vec<StoredAuditRecord>,
    pub total_count: usize,
    pub limit: usize,
    pub offset: usize,
}

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
    fn get_with_filters(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        correlation_id: Option<String>,
    ) -> Result<AuditTrailResponse, AuditStorageError>;
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

    fn get_with_filters(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        correlation_id: Option<String>,
    ) -> Result<AuditTrailResponse, AuditStorageError> {
        let all_records = self.all()?;

        // Apply time filters
        let filtered_records: Vec<StoredAuditRecord> = all_records
            .into_iter()
            .filter(|record| {
                let in_time_range = start_time
                    .as_ref()
                    .map(|start| record.timestamp >= *start)
                    .unwrap_or(true)
                    && end_time
                        .as_ref()
                        .map(|end| record.timestamp <= *end)
                        .unwrap_or(true);

                let matches_correlation = correlation_id
                    .as_ref()
                    .map(|cid| record.correlation_id == *cid)
                    .unwrap_or(true);

                in_time_range && matches_correlation
            })
            .collect();

        // Apply pagination
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        let total_count = filtered_records.len();
        let paginated_records: Vec<StoredAuditRecord> = filtered_records
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        Ok(AuditTrailResponse {
            records: paginated_records,
            total_count,
            limit,
            offset,
        })
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

    fn get_with_filters(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        correlation_id: Option<String>,
    ) -> Result<AuditTrailResponse, AuditStorageError> {
        let all_records = self.all()?;

        // Apply time filters
        let filtered_records: Vec<StoredAuditRecord> = all_records
            .into_iter()
            .filter(|record| {
                let in_time_range = start_time
                    .as_ref()
                    .map(|start| record.timestamp >= *start)
                    .unwrap_or(true)
                    && end_time
                        .as_ref()
                        .map(|end| record.timestamp <= *end)
                        .unwrap_or(true);

                let matches_correlation = correlation_id
                    .as_ref()
                    .map(|cid| record.correlation_id == *cid)
                    .unwrap_or(true);

                in_time_range && matches_correlation
            })
            .collect();

        // Apply pagination
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        let total_count = filtered_records.len();
        let paginated_records: Vec<StoredAuditRecord> = filtered_records
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        Ok(AuditTrailResponse {
            records: paginated_records,
            total_count,
            limit,
            offset,
        })
    }
}
