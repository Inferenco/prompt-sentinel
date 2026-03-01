use std::time::Duration;

use prompt_sentinel::modules::telemetry::correlation::{
    generate_correlation_id, generate_correlation_id_from_request,
};
use prompt_sentinel::modules::telemetry::metrics::{RequestTimer, get_metrics};
use prompt_sentinel::modules::telemetry::tracing::{
    create_span_with_correlation, init_tracing, log_with_correlation,
};
use tracing::Level;

#[test]
fn test_correlation_id_generation() {
    init_tracing();

    let id1 = generate_correlation_id();
    let id2 = generate_correlation_id();

    assert!(!id1.is_empty());
    assert!(!id2.is_empty());
    assert_ne!(id1, id2);

    // Test that IDs contain UUID and counter
    assert!(id1.contains('-'));
    assert!(id2.contains('-'));
}

#[test]
fn test_correlation_id_from_request() {
    init_tracing();

    let custom_id = "test-request-123".to_string();
    let id1 = generate_correlation_id_from_request(Some(custom_id.clone()));
    let id2 = generate_correlation_id_from_request(Some("".to_string()));
    let id3 = generate_correlation_id_from_request(None);

    assert_eq!(id1, custom_id);
    assert_ne!(id2, custom_id);
    assert!(!id2.is_empty());
    assert_ne!(id3, custom_id);
    assert!(!id3.is_empty());
}

#[test]
fn test_tracing_with_correlation() {
    init_tracing();

    let correlation_id = generate_correlation_id();

    log_with_correlation(&correlation_id, Level::INFO, "Test info message");
    log_with_correlation(&correlation_id, Level::DEBUG, "Test debug message");
    log_with_correlation(&correlation_id, Level::WARN, "Test warn message");
    log_with_correlation(&correlation_id, Level::ERROR, "Test error message");
}

#[test]
fn test_span_with_correlation() {
    init_tracing();

    let correlation_id = generate_correlation_id();
    let span = create_span_with_correlation(&correlation_id, "test_span");

    let _enter = span.enter();
    log_with_correlation(&correlation_id, Level::INFO, "Message inside span");
}

#[test]
fn test_request_timer() {
    init_tracing();

    let timer = RequestTimer::new();

    // Simulate some work
    std::thread::sleep(Duration::from_millis(10));

    let elapsed = timer.elapsed_seconds();

    assert!(elapsed >= 0.01);
    assert!(elapsed < 1.0);
}

#[test]
fn test_metrics_incrementation() {
    init_tracing();

    let metrics = get_metrics();

    // Test counters
    metrics.increment_requests("GET", "/test");
    metrics.increment_requests("POST", "/test");
    metrics.increment_errors("test_error");

    // Test gauges
    metrics.increment_active_requests();
    metrics.decrement_active_requests();

    // Test timer
    let timer = RequestTimer::new();
    std::thread::sleep(Duration::from_millis(5));
    let duration = timer.elapsed_seconds();

    metrics.record_latency("GET", "/test", duration);
}
