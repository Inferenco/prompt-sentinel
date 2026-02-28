use tracing::{info, debug, error, warn, span, Level};
use tracing_subscriber::{fmt, EnvFilter};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_tracing() {
    INIT.call_once(|| {
        let filter = EnvFilter::new("info,prompt_sentinel=debug,tower_http=debug");

        fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
    });
}

pub fn log_with_correlation(correlation_id: &str, level: Level, message: &str) {
    match level {
        Level::ERROR => error!(correlation_id = %correlation_id, "{}", message),
        Level::WARN => warn!(correlation_id = %correlation_id, "{}", message),
        Level::INFO => info!(correlation_id = %correlation_id, "{}", message),
        Level::DEBUG => debug!(correlation_id = %correlation_id, "{}", message),
        Level::TRACE => tracing::trace!(correlation_id = %correlation_id, "{}", message),
    }
}

pub fn create_span_with_correlation(correlation_id: &str, _name: &str) -> tracing::Span {
    span!(Level::INFO, "request", correlation_id = %correlation_id)
}