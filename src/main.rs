use prompt_sentinel::FrameworkConfig;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting Prompt Sentinel Framework");
    
    // Use default configuration (port 3000, sled db at "prompt_sentinel_data")
    let config = FrameworkConfig::default();
    
    // Initialize the framework
    let server = config.initialize()?;
    
    // Start the server
    server.start().await?;
    
    Ok(())
}
