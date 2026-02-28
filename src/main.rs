use prompt_sentinel::FrameworkConfig;
use prompt_sentinel::modules::telemetry::tracing::init_tracing;
use prompt_sentinel::modules::telemetry::metrics::TelemetryMetrics;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize enhanced tracing with correlation support
    init_tracing();

    info!("🚀 Starting Prompt Sentinel Framework");

    // Start metrics server on port 9090
    info!("📊 Starting metrics server on 0.0.0.0:9090");
    TelemetryMetrics::start_metrics_server("0.0.0.0:9090")?;

    // Use default configuration (port 3000, sled db at "prompt_sentinel_data")
    let config = FrameworkConfig::default();

    // Initialize the framework (now async)
    let server = config.initialize().await?;

    // Start the server
    server.start().await?;

    Ok(())
}
