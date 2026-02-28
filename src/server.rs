use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde_json;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, error, info};

use crate::config::settings::AppSettings;
use crate::modules::audit::logger::AuditLogger;
use crate::modules::audit::storage::{AuditStorage, SledAuditStorage, AuditTrailRequest, AuditTrailResponse};
use crate::modules::bias_detection::service::BiasDetectionService;
use crate::modules::eu_law_compliance::dtos::{ComplianceReportRequest, ComplianceReportResponse, ComplianceConfigurationRequest, ComplianceConfigurationResponse};
use crate::modules::eu_law_compliance::service::EuLawComplianceService;
use crate::modules::mistral_ai::client::{HttpMistralClient, MistralClient};
use crate::modules::mistral_ai::dtos::ModelValidationResponse;
use crate::modules::mistral_ai::service::MistralService;
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
            .route("/api/mistral/health", get(mistral_health_check))
            .route("/v1/models", get(validate_models))
            .route("/api/audit/trail", post(get_audit_trail))
            .route("/api/compliance/report", post(generate_compliance_report))
            .route("/api/compliance/config", get(get_compliance_config))
            .route("/api/compliance/config", post(update_compliance_config))
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

async fn mistral_health_check(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    debug!("Received Mistral health check request");
    
    let mistral_service = state.engine.mistral_service();
    
    match mistral_service.health_check().await {
        Ok(_) => {
            info!("Mistral health check passed");
            Ok(Json(serde_json::json!({
                "status": "healthy",
                "message": "Mistral API integration is operational",
                "models": [
                    mistral_service.generation_model(),
                    mistral_service.moderation_model(),
                    mistral_service.embedding_model()
                ]
            })))
        }
        Err(e) => {
            error!("Mistral health check failed: {}", e);
            Err((StatusCode::SERVICE_UNAVAILABLE, format!("Mistral API unhealthy: {}", e)))
        }
    }
}

async fn validate_models(
    State(_state): State<AppState>,
) -> Result<Json<ModelValidationResponse>, (StatusCode, String)> {
    debug!("Received model validation request");
    
    let mistral_service = _state.engine.mistral_service();
    
    match mistral_service.validate_models_endpoint().await {
        response => {
            info!("Model validation completed");
            Ok(Json(response))
        }
    }
}

async fn get_audit_trail(
    State(_state): State<AppState>,
    Json(request): Json<AuditTrailRequest>,
) -> Result<Json<AuditTrailResponse>, (StatusCode, String)> {
    debug!("Received audit trail request");
    
    let audit_logger = _state.engine.audit_logger();
    let storage = audit_logger.storage();
    
    match storage.get_with_filters(
        request.limit,
        request.offset,
        request.start_time,
        request.end_time,
        request.correlation_id,
    ) {
        Ok(response) => {
            info!("Audit trail retrieved successfully");
            Ok(Json(response))
        }
        Err(e) => {
            error!("Failed to retrieve audit trail: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to retrieve audit trail: {}", e)))
        }
    }
}

async fn generate_compliance_report(
    State(_state): State<AppState>,
    Json(request): Json<ComplianceReportRequest>,
) -> Result<Json<ComplianceReportResponse>, (StatusCode, String)> {
    debug!("Received compliance report generation request");
    
    let eu_service = EuLawComplianceService::default();
    let response = eu_service.generate_compliance_report(request);
    
    info!("Compliance report generated successfully");
    Ok(Json(response))
}

async fn get_compliance_config(
    State(_state): State<AppState>,
) -> Result<Json<ComplianceConfigurationResponse>, (StatusCode, String)> {
    debug!("Received compliance configuration request");
    
    let eu_service = EuLawComplianceService::default();
    let response = eu_service.get_current_configuration();
    
    let config_response = ComplianceConfigurationResponse {
        status: "success".to_string(),
        message: "Current compliance configuration retrieved".to_string(),
        current_configuration: response,
    };
    
    info!("Compliance configuration retrieved successfully");
    Ok(Json(config_response))
}

async fn update_compliance_config(
    State(_state): State<AppState>,
    Json(request): Json<ComplianceConfigurationRequest>,
) -> Result<Json<ComplianceConfigurationResponse>, (StatusCode, String)> {
    debug!("Received compliance configuration update request");
    
    let eu_service = EuLawComplianceService::default();
    let response = eu_service.update_configuration(request);
    
    info!("Compliance configuration update processed");
    Ok(Json(response))
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
    pub async fn initialize(self) -> Result<PromptSentinelServer, Box<dyn std::error::Error>> {
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

        // Perform model validation at startup
        info!("Validating Mistral models at startup...");
        mistral_service.validate_all_models().await.map_err(|e| {
            error!("Model validation failed: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;
        info!("All Mistral models validated successfully");

        let engine = ComplianceEngine::new(
            firewall_service,
            bias_service,
            mistral_service,
            audit_logger,
        );

        Ok(PromptSentinelServer::new(settings, engine))
    }
}
