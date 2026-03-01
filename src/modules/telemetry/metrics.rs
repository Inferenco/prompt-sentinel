use std::time::Instant;

use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct TelemetryMetrics {
    request_counter: AtomicU64,
    error_counter: AtomicU64,
    active_requests_gauge: AtomicU64,
}

impl Default for TelemetryMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetryMetrics {
    pub fn new() -> Self {
        Self {
            request_counter: AtomicU64::new(0),
            error_counter: AtomicU64::new(0),
            active_requests_gauge: AtomicU64::new(0),
        }
    }

    pub fn increment_requests(&self, method: &str, endpoint: &str) {
        self.request_counter.fetch_add(1, Ordering::SeqCst);
        counter!("requests_total", "method" => method.to_string(), "endpoint" => endpoint.to_string()).increment(1);
    }

    pub fn increment_errors(&self, error_type: &str) {
        self.error_counter.fetch_add(1, Ordering::SeqCst);
        counter!("errors_total", "error_type" => error_type.to_string()).increment(1);
    }

    pub fn record_latency(&self, method: &str, endpoint: &str, duration: f64) {
        histogram!("request_latency_seconds", "method" => method.to_string(), "endpoint" => endpoint.to_string()).record(duration);
    }

    pub fn increment_active_requests(&self) {
        self.active_requests_gauge.fetch_add(1, Ordering::SeqCst);
        gauge!("active_requests").increment(1.0);
    }

    pub fn decrement_active_requests(&self) {
        self.active_requests_gauge.fetch_sub(1, Ordering::SeqCst);
        gauge!("active_requests").decrement(1.0);
    }

    pub fn start_metrics_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let builder = PrometheusBuilder::new();
        let socket_addr: std::net::SocketAddr = addr.parse()?;
        let builder = builder.with_http_listener(socket_addr);

        // Install the recorder
        builder.install()?;

        Ok(())
    }
}

pub struct RequestTimer {
    start: Instant,
}

impl RequestTimer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed_seconds(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

static METRICS: Lazy<TelemetryMetrics> = Lazy::new(TelemetryMetrics::new);

pub fn get_metrics() -> &'static TelemetryMetrics {
    &METRICS
}
