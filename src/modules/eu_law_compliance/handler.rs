use super::dtos::{ComplianceCheckRequest, ComplianceCheckResponse};
use super::service::EuLawComplianceService;

pub fn handle_compliance_check(
    service: &EuLawComplianceService,
    request: ComplianceCheckRequest,
) -> ComplianceCheckResponse {
    service.check(request)
}
