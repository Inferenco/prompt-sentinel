export type DemoMode = "live" | "mock";

export type WorkflowStatus =
  | "Completed"
  | "BlockedByFirewall"
  | "BlockedByInputModeration"
  | "BlockedByOutputModeration";

export type FirewallAction = "Allow" | "Sanitize" | "Block";

export type FirewallSeverity = "Low" | "Medium" | "High" | "Critical";

export type BiasLevel = "Low" | "Medium" | "High";

export type BiasCategory =
  | "Gender"
  | "RaceEthnicity"
  | "Age"
  | "Religion"
  | "Disability"
  | "SocioEconomic";

export interface ComplianceRequest {
  correlation_id?: string;
  prompt: string;
}

export interface PromptFirewallResult {
  action: FirewallAction;
  severity: FirewallSeverity;
  sanitized_prompt: string;
  reasons: string[];
  matched_rules: string[];
}

export interface BiasScanResult {
  score: number;
  level: BiasLevel;
  categories: BiasCategory[];
  matched_terms: string[];
  mitigation_hints: string[];
}

export interface ModerationResponse {
  flagged: boolean;
  categories: string[];
  severity: number;
}

export interface AuditProof {
  algorithm: string;
  record_hash: string;
  chain_hash: string;
}

export interface ComplianceResponse {
  correlation_id: string;
  status: WorkflowStatus;
  firewall: PromptFirewallResult;
  bias: BiasScanResult;
  input_moderation: ModerationResponse | null;
  output_moderation: ModerationResponse | null;
  generated_text: string | null;
  audit_proof: AuditProof;
}

export interface MistralHealthResponse {
  status: "healthy" | "unhealthy" | string;
  message: string;
  models: Array<string | null>;
}

export interface HealthState {
  serviceStatus: "unknown" | "healthy" | "error";
  mistralStatus: "unknown" | "healthy" | "unhealthy" | "error";
  message: string;
  models: Array<string | null>;
  loading: boolean;
  error: string | null;
  lastCheckedAt: string | null;
}

export interface DemoScenario {
  id: string;
  label: string;
  description: string;
  prompt: string;
  correlationId?: string;
}

export interface DemoRunRecord {
  id: string;
  timestampIso: string;
  mode: DemoMode;
  request: ComplianceRequest;
  response: ComplianceResponse | null;
  error: string | null;
}
