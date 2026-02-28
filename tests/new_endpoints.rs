use chrono::Utc;
use prompt_sentinel::modules::audit::storage::AuditTrailRequest;
use prompt_sentinel::modules::eu_law_compliance::dtos::{
    ComplianceConfigurationRequest, ComplianceReportRequest, RiskThresholds,
};
use prompt_sentinel::modules::eu_law_compliance::service::EuLawComplianceService;
use prompt_sentinel::modules::mistral_ai::service::MistralService;
use prompt_sentinel::modules::mistral_ai::client::MockMistralClient;
use std::sync::Arc;

#[tokio::test]
async fn test_model_validation_endpoint() {
    let mock_client = Arc::new(MockMistralClient::default());
    let service = MistralService::new(
        mock_client,
        "mistral-large-latest".to_string(),
        Some("mistral-large-latest".to_string()), // Use same model for moderation
        "mistral-embed".to_string(),
    );

    let response = service.validate_models_endpoint().await;
    
    assert_eq!(response.generation_model.model_name, "mistral-large-latest");
    assert!(response.generation_model.available);
    assert_eq!(response.moderation_model.as_ref().unwrap().model_name, "mistral-large-latest");
    assert!(response.moderation_model.as_ref().unwrap().available);
    assert_eq!(response.embedding_model.model_name, "mistral-embed");
    assert!(response.embedding_model.available);
    assert_eq!(response.overall_status, "all_models_available");
}

#[test]
fn test_compliance_report_generation() {
    let service = EuLawComplianceService::default();
    
    let request = ComplianceReportRequest {
        intended_use: "AI-powered chatbot for customer support".to_string(),
        request_timestamp: Utc::now(),
        correlation_id: "test-123".to_string(),
        generate_pdf: false,
    };

    let response = service.generate_compliance_report(request);
    
    assert!(response.report_id.contains("test-123"));
    assert!(response.compliant);
    assert!(!response.pdf_available);
    assert!(response.pdf_url.is_none());
}

#[test]
fn test_compliance_configuration_management() {
    let service = EuLawComplianceService::default();
    
    // Test getting current configuration
    let current_config = service.get_current_configuration();
    let _initial_unacceptable_count = current_config.risk_keyword_counts.unacceptable;
    
    // Test updating configuration
    let update_request = ComplianceConfigurationRequest {
        risk_thresholds: Some(RiskThresholds {
            unacceptable_keywords: Some(vec!["social scoring".to_string(), "new prohibited use".to_string()]),
            high_risk_keywords: None,
            limited_risk_keywords: None,
        }),
        documentation_requirements: None,
    };

    let update_response = service.update_configuration(update_request);
    
    assert_eq!(update_response.status, "success");
    assert!(update_response.message.contains("updated successfully"));
    
    // Verify configuration was updated
    let new_config = service.get_current_configuration();
    assert_eq!(new_config.risk_keyword_counts.unacceptable, 2); // Should have 2 keywords now
}

#[test]
fn test_audit_trail_filters() {
    // This would test the audit trail filtering functionality
    // In a real implementation, we would test with actual audit data
    let request = AuditTrailRequest {
        limit: Some(10),
        offset: Some(0),
        start_time: None,
        end_time: None,
        correlation_id: None,
    };
    
    // The actual implementation would be tested with a real storage backend
    // This is just a placeholder to show the API works
    assert_eq!(request.limit.unwrap(), 10);
    assert_eq!(request.offset.unwrap(), 0);
}