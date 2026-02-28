import type {
  AuditProof,
  BiasScanResult,
  ComplianceResponse,
  MistralHealthResponse,
  ModerationResponse,
  PromptFirewallResult,
  WorkflowStatus,
} from "./types";

const workflowStatuses = new Set<WorkflowStatus>([
  "Completed",
  "BlockedByFirewall",
  "BlockedByInputModeration",
  "BlockedByOutputModeration",
]);

function expectObject(value: unknown, label: string): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error(`${label} must be an object`);
  }
  return value as Record<string, unknown>;
}

function expectString(value: unknown, label: string): string {
  if (typeof value !== "string") {
    throw new Error(`${label} must be a string`);
  }
  return value;
}

function expectNumber(value: unknown, label: string): number {
  if (typeof value !== "number" || Number.isNaN(value)) {
    throw new Error(`${label} must be a number`);
  }
  return value;
}

function expectBoolean(value: unknown, label: string): boolean {
  if (typeof value !== "boolean") {
    throw new Error(`${label} must be a boolean`);
  }
  return value;
}

function expectStringArray(value: unknown, label: string): string[] {
  if (!Array.isArray(value) || value.some((item) => typeof item !== "string")) {
    throw new Error(`${label} must be a string array`);
  }
  return value;
}

function parseWorkflowStatus(value: unknown): WorkflowStatus {
  const status = expectString(value, "status");
  if (!workflowStatuses.has(status as WorkflowStatus)) {
    throw new Error(`status is invalid: ${status}`);
  }
  return status as WorkflowStatus;
}

function parseFirewall(value: unknown): PromptFirewallResult {
  const raw = expectObject(value, "firewall");
  return {
    action: expectString(raw.action, "firewall.action") as PromptFirewallResult["action"],
    severity: expectString(raw.severity, "firewall.severity") as PromptFirewallResult["severity"],
    sanitized_prompt: expectString(raw.sanitized_prompt, "firewall.sanitized_prompt"),
    reasons: expectStringArray(raw.reasons, "firewall.reasons"),
    matched_rules: expectStringArray(raw.matched_rules, "firewall.matched_rules"),
  };
}

function parseBias(value: unknown): BiasScanResult {
  const raw = expectObject(value, "bias");
  return {
    score: expectNumber(raw.score, "bias.score"),
    level: expectString(raw.level, "bias.level") as BiasScanResult["level"],
    categories: expectStringArray(raw.categories, "bias.categories") as BiasScanResult["categories"],
    matched_terms: expectStringArray(raw.matched_terms, "bias.matched_terms"),
    mitigation_hints: expectStringArray(raw.mitigation_hints, "bias.mitigation_hints"),
  };
}

function parseModeration(value: unknown, label: string): ModerationResponse {
  const raw = expectObject(value, label);
  return {
    flagged: expectBoolean(raw.flagged, `${label}.flagged`),
    categories: expectStringArray(raw.categories, `${label}.categories`),
    severity: expectNumber(raw.severity, `${label}.severity`),
  };
}

function parseAuditProof(value: unknown): AuditProof {
  const raw = expectObject(value, "audit_proof");
  return {
    algorithm: expectString(raw.algorithm, "audit_proof.algorithm"),
    record_hash: expectString(raw.record_hash, "audit_proof.record_hash"),
    chain_hash: expectString(raw.chain_hash, "audit_proof.chain_hash"),
  };
}

export function parseMistralHealthResponse(value: unknown): MistralHealthResponse {
  const raw = expectObject(value, "mistral_health");

  if (!Array.isArray(raw.models)) {
    throw new Error("mistral_health.models must be an array");
  }

  const models = raw.models.map((model) => {
    if (model === null || typeof model === "string") {
      return model;
    }
    throw new Error("mistral_health.models entries must be string or null");
  });

  return {
    status: expectString(raw.status, "mistral_health.status"),
    message: expectString(raw.message, "mistral_health.message"),
    models,
  };
}

export function parseComplianceResponse(value: unknown): ComplianceResponse {
  const raw = expectObject(value, "compliance_response");

  const inputModeration =
    raw.input_moderation === null || raw.input_moderation === undefined
      ? null
      : parseModeration(raw.input_moderation, "input_moderation");

  const outputModeration =
    raw.output_moderation === null || raw.output_moderation === undefined
      ? null
      : parseModeration(raw.output_moderation, "output_moderation");

  const generatedText =
    raw.generated_text === null || raw.generated_text === undefined
      ? null
      : expectString(raw.generated_text, "generated_text");

  return {
    correlation_id: expectString(raw.correlation_id, "correlation_id"),
    status: parseWorkflowStatus(raw.status),
    firewall: parseFirewall(raw.firewall),
    bias: parseBias(raw.bias),
    input_moderation: inputModeration,
    output_moderation: outputModeration,
    generated_text: generatedText,
    audit_proof: parseAuditProof(raw.audit_proof),
  };
}
