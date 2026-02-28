use prompt_sentinel::modules::eu_law_compliance::dtos::ComplianceCheckRequest;
use prompt_sentinel::modules::eu_law_compliance::model::AiRiskTier;
use prompt_sentinel::modules::eu_law_compliance::service::EuLawComplianceService;

#[test]
fn unacceptable_use_is_not_compliant_even_with_documentation() {
    let service = EuLawComplianceService;
    let response = service.check(ComplianceCheckRequest {
        intended_use: "Biometric surveillance in public spaces".to_owned(),
        technical_documentation_available: true,
        transparency_notice_available: true,
        copyright_controls_available: true,
    });

    assert_eq!(response.risk_tier, AiRiskTier::Unacceptable);
    assert!(!response.compliant);
    assert!(response.findings.iter().any(|f| f.code == "EU-RISK-001"));
}

#[test]
fn high_risk_use_requires_core_controls() {
    let service = EuLawComplianceService;
    let response = service.check(ComplianceCheckRequest {
        intended_use: "Automated screening for employment candidates".to_owned(),
        technical_documentation_available: false,
        transparency_notice_available: false,
        copyright_controls_available: false,
    });

    assert_eq!(response.risk_tier, AiRiskTier::High);
    assert!(!response.compliant);
    assert!(response.findings.iter().any(|f| f.code == "EU-DOC-001"));
    assert!(response.findings.iter().any(|f| f.code == "EU-TRN-001"));
    assert!(response.findings.iter().any(|f| f.code == "EU-CPY-001"));
}

#[test]
fn hiring_keyword_is_classified_as_high_risk() {
    let service = EuLawComplianceService;
    let response = service.check(ComplianceCheckRequest {
        intended_use: "AI hiring assistant that ranks candidates".to_owned(),
        technical_documentation_available: true,
        transparency_notice_available: true,
        copyright_controls_available: true,
    });

    assert_eq!(response.risk_tier, AiRiskTier::High);
    assert!(response.compliant);
}

#[test]
fn limited_risk_with_transparency_can_pass() {
    let service = EuLawComplianceService;
    let response = service.check(ComplianceCheckRequest {
        intended_use: "Customer support chatbot for order updates".to_owned(),
        technical_documentation_available: false,
        transparency_notice_available: true,
        copyright_controls_available: false,
    });

    assert_eq!(response.risk_tier, AiRiskTier::Limited);
    assert!(response.compliant);
    assert!(response.findings.is_empty());
}

#[test]
fn missing_intended_use_context_is_flagged() {
    let service = EuLawComplianceService;
    let response = service.check(ComplianceCheckRequest {
        intended_use: " ".to_owned(),
        technical_documentation_available: true,
        transparency_notice_available: true,
        copyright_controls_available: true,
    });

    assert!(!response.compliant);
    assert!(response.findings.iter().any(|f| f.code == "EU-SCOPE-001"));
}
