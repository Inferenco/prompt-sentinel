use prompt_sentinel::FrameworkConfig;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting Prompt Sentinel Framework");

    // Use default configuration (port 3000, sled db at "prompt_sentinel_data")
    let config = FrameworkConfig::default();

    // Initialize the framework (now async)
    let server = config.initialize().await?;

    // Start the server
    server.start().await?;

    Ok(())
}
