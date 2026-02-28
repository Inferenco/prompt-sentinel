use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn generate_correlation_id() -> String {
    let counter = REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let uuid = Uuid::new_v4();
    format!("{}-{}", uuid, counter)
}

pub fn generate_correlation_id_from_request(request_id: Option<String>) -> String {
    match request_id {
        Some(id) if !id.is_empty() => id,
        _ => generate_correlation_id(),
    }
}
