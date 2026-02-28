use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AuditProof {
    pub algorithm: String,
    pub record_hash: String,
    pub chain_hash: String,
}

pub fn hash_record(payload: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn chain_hash(previous_chain_hash: Option<&str>, record_hash: &str) -> String {
    let mut hasher = Sha256::new();
    if let Some(previous) = previous_chain_hash {
        hasher.update(previous.as_bytes());
    }
    hasher.update(record_hash.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_deterministic_hashes() {
        let payload = r#"{"test":"value"}"#;
        let hash_a = hash_record(payload);
        let hash_b = hash_record(payload);
        assert_eq!(hash_a, hash_b);
    }
}
