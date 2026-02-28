use axum::{routing::post, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::config::settings::AppSettings;
use crate::modules::audit::storage::SledAuditStorage;
use crate::modules::workflow::handler;

/// Framework server builder
pub struct PromptSentinelServer {
    config: AppSettings,
    audit_storage: SledAuditStorage,
}

impl PromptSentinelServer {
    /// Create a new server instance
    pub fn new(config: AppSettings, audit_storage: SledAuditStorage) -> Self {
        Self {
            config,
            audit_storage,
        }
    }

    /// Build the axum router with all endpoints
    fn build_router(&self) -> Router {
        Router::new()
            .route("/api/compliance/check", post(handler::check_compliance))
            .route("/health", post(|| async { "OK" }))
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            )
    }

    /// Start the server
    pub async fn start(self) -> Result<(), std::io::Error> {
        let app = self.build_router();
        let addr = SocketAddr::from(([0, 0, 0, 0], self.config.server_port));
        
        info!("Prompt Sentinel Server starting on {}", addr);
        info!("Using sled for audit storage");
        info!("Framework version: {}", env!("CARGO_PKG_VERSION"));

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
    }
}

/// Framework configuration for easy setup
pub struct FrameworkConfig {
    pub server_port: u16,
    pub sled_db_path: String,
}

impl Default for FrameworkConfig {
    fn default() -> Self {
        Self {
            server_port: 3000,
            sled_db_path: "prompt_sentinel_data".to_string(),
        }
    }
}

impl FrameworkConfig {
    /// Initialize the framework with default or custom configuration
    pub fn initialize(&self) -> Result<PromptSentinelServer, Box<dyn std::error::Error>> {
        let config = AppSettings {
            server_port: self.server_port,
            // Other config fields
        };
        
        let audit_storage = SledAuditStorage::new(&self.sled_db_path)?;
        
        Ok(PromptSentinelServer::new(config, audit_storage))
    }
}
