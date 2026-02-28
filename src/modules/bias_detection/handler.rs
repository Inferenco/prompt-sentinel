use super::dtos::{BiasScanRequest, BiasScanResult};
use super::service::BiasDetectionService;

pub fn handle_bias_scan(
    service: &BiasDetectionService,
    text: impl Into<String>,
    threshold: Option<f32>,
) -> BiasScanResult {
    service.scan(BiasScanRequest {
        text: text.into(),
        threshold,
    })
}
