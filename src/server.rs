use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::config::settings::AppSettings;
use crate::modules::audit::logger::AuditLogger;
use crate::modules::audit::storage::{AuditStorage, SledAuditStorage};
use crate::modules::bias_detection::service::BiasDetectionService;
use crate::modules::mistral_expert::client::{HttpMistralClient, MistralClient};
use crate::modules::mistral_expert::service::MistralService;
use crate::modules::prompt_firewall::service::PromptFirewallService;
use crate::workflow::{ComplianceEngine, ComplianceRequest, ComplianceResponse};

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<ComplianceEngine>,
}

/// Framework server builder
pub struct PromptSentinelServer {
    config: AppSettings,
    state: AppState,
}

impl PromptSentinelServer {
    /// Create a new server instance
    pub fn new(config: AppSettings, engine: ComplianceEngine) -> Self {
        Self {
            config,
            state: AppState {
                engine: Arc::new(engine),
            },
        }
    }

    /// Build the axum router with all endpoints
    fn build_router(&self) -> Router {
        Router::new()
            .route("/api/compliance/check", post(check_compliance))
            .route("/health", get(health_check))
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            )
            .with_state(self.state.clone())
    }

    /// Start the server
    pub async fn start(self) -> Result<(), std::io::Error> {
        let app = self.build_router();
        let addr = format!("0.0.0.0:{}", self.config.server_port);

        info!("Prompt Sentinel Server starting on {}", addr);
        info!("Using sled for audit storage");
        info!("Framework version: {}", env!("CARGO_PKG_VERSION"));

        let listener = TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await
    }
}

async fn health_check() -> &'static str {
    "OK"
}

async fn check_compliance(
    State(state): State<AppState>,
    Json(request): Json<ComplianceRequest>,
) -> Result<Json<ComplianceResponse>, (StatusCode, String)> {
    state
        .engine
        .process(request)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Framework configuration for easy setup
pub struct FrameworkConfig {
    pub server_port: u16,
    pub sled_db_path: String,
    pub mistral_api_key: Option<String>,
}

impl Default for FrameworkConfig {
    fn default() -> Self {
        Self {
            server_port: 3000,
            sled_db_path: "prompt_sentinel_data".to_string(),
            mistral_api_key: std::env::var("MISTRAL_API_KEY").ok(),
        }
    }
}

impl FrameworkConfig {
    /// Initialize the framework with default or custom configuration
    pub fn initialize(self) -> Result<PromptSentinelServer, Box<dyn std::error::Error>> {
        let settings = AppSettings::from_env().unwrap_or_else(|_| AppSettings {
            server_port: self.server_port,
            mistral_api_key: self.mistral_api_key.clone(),
            mistral_base_url: "https://api.mistral.ai".to_string(),
            generation_model: "mistral-large-latest".to_string(),
            moderation_model: None,
            embedding_model: "mistral-embed".to_string(),
            bias_threshold: 0.35,
            max_input_length: 4096,
        });

        let audit_storage: Arc<dyn AuditStorage> =
            Arc::new(SledAuditStorage::new(&self.sled_db_path)?);
        let audit_logger = AuditLogger::new(audit_storage);

        let firewall_service = PromptFirewallService::new(settings.max_input_length);
        let bias_service = BiasDetectionService::new(settings.bias_threshold);
        let mistral_client: Arc<dyn MistralClient> = Arc::new(HttpMistralClient::new(
            settings.mistral_base_url.clone(),
            settings.mistral_api_key.clone().unwrap_or_default(),
        ));
        let mistral_service = MistralService::new(
            mistral_client,
            settings.generation_model.clone(),
            settings.moderation_model.clone(),
            settings.embedding_model.clone(),
        );

        let engine = ComplianceEngine::new(
            firewall_service,
            bias_service,
            mistral_service,
            audit_logger,
        );

        Ok(PromptSentinelServer::new(settings, engine))
    }
}
