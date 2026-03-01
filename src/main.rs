use prompt_sentinel::FrameworkConfig;
use prompt_sentinel::modules::telemetry::metrics::TelemetryMetrics;
use prompt_sentinel::modules::telemetry::tracing::init_tracing;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file FIRST
    dotenvy::dotenv().ok();

    // Initialize enhanced tracing with correlation support
    init_tracing();

    info!("Starting Prompt Sentinel Framework");

    // Start metrics server on port 9090
    info!("Starting metrics server on 0.0.0.0:9090");
    TelemetryMetrics::start_metrics_server("0.0.0.0:9090")?;

    // Use default configuration (reads from env vars)
    let config = FrameworkConfig::default();

    // Initialize the framework
    let server = config.initialize().await?;

    // Start the server
    server.start().await?;

    Ok(())
}
