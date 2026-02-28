use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::modules::audit::logger::{AuditError, AuditEvent, AuditLogger};
use crate::modules::audit::proof::AuditProof;
use crate::modules::bias_detection::dtos::{BiasScanRequest, BiasScanResult};
use crate::modules::bias_detection::service::BiasDetectionService;
use crate::modules::mistral_expert::dtos::ModerationResponse;
use crate::modules::mistral_expert::service::{MistralService, MistralServiceError};
use crate::modules::prompt_firewall::dtos::{
    FirewallAction, PromptFirewallRequest, PromptFirewallResult,
};
use crate::modules::prompt_firewall::service::PromptFirewallService;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum WorkflowStatus {
    Completed,
    BlockedByFirewall,
    BlockedByInputModeration,
    BlockedByOutputModeration,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ComplianceRequest {
    pub correlation_id: Option<String>,
    pub prompt: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ComplianceResponse {
    pub correlation_id: String,
    pub status: WorkflowStatus,
    pub firewall: PromptFirewallResult,
    pub bias: BiasScanResult,
    pub input_moderation: Option<ModerationResponse>,
    pub output_moderation: Option<ModerationResponse>,
    pub generated_text: Option<String>,
    pub audit_proof: AuditProof,
}

#[derive(Clone)]
pub struct ComplianceEngine {
    firewall_service: PromptFirewallService,
    bias_service: BiasDetectionService,
    mistral_service: MistralService,
    audit_logger: AuditLogger,
}

impl ComplianceEngine {
    pub fn new(
        firewall_service: PromptFirewallService,
        bias_service: BiasDetectionService,
        mistral_service: MistralService,
        audit_logger: AuditLogger,
    ) -> Self {
        Self {
            firewall_service,
            bias_service,
            mistral_service,
            audit_logger,
        }
    }

    /// Get a reference to the Mistral service for health checks
    pub fn mistral_service(&self) -> &MistralService {
        &self.mistral_service
    }

    pub async fn process(
        &self,
        request: ComplianceRequest,
    ) -> Result<ComplianceResponse, WorkflowError> {
        let correlation_id = request
            .correlation_id
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let firewall = self.firewall_service.inspect(PromptFirewallRequest {
            prompt: request.prompt.clone(),
            correlation_id: Some(correlation_id.clone()),
        });

        let bias = self.bias_service.scan(BiasScanRequest {
            text: firewall.sanitized_prompt.clone(),
            threshold: None,
        });

        if firewall.action == FirewallAction::Block {
            let proof = self.audit_logger.log_event(AuditEvent {
                correlation_id: correlation_id.clone(),
                original_prompt: request.prompt,
                sanitized_prompt: firewall.sanitized_prompt.clone(),
                firewall_action: format!("{:?}", firewall.action),
                firewall_reasons: firewall.reasons.clone(),
                bias_score: bias.score,
                bias_level: format!("{:?}", bias.level),
                input_moderation_flagged: false,
                output_moderation_flagged: false,
                final_status: "blocked_by_firewall".to_owned(),
                model_used: None,
                output_preview: None,
            })?;

            return Ok(ComplianceResponse {
                correlation_id,
                status: WorkflowStatus::BlockedByFirewall,
                firewall,
                bias,
                input_moderation: None,
                output_moderation: None,
                generated_text: None,
                audit_proof: proof,
            });
        }

        let input_moderation = self
            .mistral_service
            .moderate_text(firewall.sanitized_prompt.clone())
            .await?;
        if input_moderation.flagged {
            let proof = self.audit_logger.log_event(AuditEvent {
                correlation_id: correlation_id.clone(),
                original_prompt: request.prompt,
                sanitized_prompt: firewall.sanitized_prompt.clone(),
                firewall_action: format!("{:?}", firewall.action),
                firewall_reasons: firewall.reasons.clone(),
                bias_score: bias.score,
                bias_level: format!("{:?}", bias.level),
                input_moderation_flagged: true,
                output_moderation_flagged: false,
                final_status: "blocked_by_input_moderation".to_owned(),
                model_used: None,
                output_preview: None,
            })?;

            return Ok(ComplianceResponse {
                correlation_id,
                status: WorkflowStatus::BlockedByInputModeration,
                firewall,
                bias,
                input_moderation: Some(input_moderation),
                output_moderation: None,
                generated_text: None,
                audit_proof: proof,
            });
        }

        let generation = self
            .mistral_service
            .generate_text(firewall.sanitized_prompt.clone(), true)
            .await?;

        let output_moderation = self
            .mistral_service
            .moderate_text(generation.output_text.clone())
            .await?;

        if output_moderation.flagged {
            let proof = self.audit_logger.log_event(AuditEvent {
                correlation_id: correlation_id.clone(),
                original_prompt: request.prompt,
                sanitized_prompt: firewall.sanitized_prompt.clone(),
                firewall_action: format!("{:?}", firewall.action),
                firewall_reasons: firewall.reasons.clone(),
                bias_score: bias.score,
                bias_level: format!("{:?}", bias.level),
                input_moderation_flagged: false,
                output_moderation_flagged: true,
                final_status: "blocked_by_output_moderation".to_owned(),
                model_used: Some(generation.model),
                output_preview: Some(generation.output_text.chars().take(160).collect()),
            })?;

            return Ok(ComplianceResponse {
                correlation_id,
                status: WorkflowStatus::BlockedByOutputModeration,
                firewall,
                bias,
                input_moderation: Some(input_moderation),
                output_moderation: Some(output_moderation),
                generated_text: None,
                audit_proof: proof,
            });
        }

        let proof = self.audit_logger.log_event(AuditEvent {
            correlation_id: correlation_id.clone(),
            original_prompt: request.prompt,
            sanitized_prompt: firewall.sanitized_prompt.clone(),
            firewall_action: format!("{:?}", firewall.action),
            firewall_reasons: firewall.reasons.clone(),
            bias_score: bias.score,
            bias_level: format!("{:?}", bias.level),
            input_moderation_flagged: false,
            output_moderation_flagged: false,
            final_status: "completed".to_owned(),
            model_used: Some(generation.model.clone()),
            output_preview: Some(generation.output_text.chars().take(160).collect()),
        })?;

        Ok(ComplianceResponse {
            correlation_id,
            status: WorkflowStatus::Completed,
            firewall,
            bias,
            input_moderation: Some(input_moderation),
            output_moderation: Some(output_moderation),
            generated_text: Some(generation.output_text),
            audit_proof: proof,
        })
    }
}

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("mistral workflow failure: {0}")]
    Mistral(#[from] MistralServiceError),
    #[error("audit workflow failure: {0}")]
    Audit(#[from] AuditError),
}
