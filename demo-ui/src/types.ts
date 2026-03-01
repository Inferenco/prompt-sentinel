export interface ModerationResult {
    flagged: boolean;
    severity: number;
    categories: string[];
}

export interface FirewallResult {
    action: 'Allow' | 'Block' | 'Sanitize';
    severity: 'Low' | 'Medium' | 'High' | 'Critical';
    matched_rules: string[];
    sanitized_prompt: string;
    reasons: string[];
}

export interface SemanticResult {
    risk_score: number;
    risk_level: 'Low' | 'Medium' | 'High';
    nearest_template_id: string | null;
    similarity: number;
    category: string | null;
}

export interface BiasResult {
    score: number;
    level: 'Low' | 'Medium' | 'High';
    categories: string[];
}

export interface DecisionEvidence {
    firewall_action: string;
    firewall_matched_rules: string[];
    semantic_risk_score: number | null;
    semantic_matched_template: string | null;
    semantic_category: string | null;
    moderation_flagged: boolean;
    moderation_categories: string[];
    final_decision: string;
    final_reason: string;
}

export interface AuditProof {
    algorithm: string;
    record_hash: string;
    chain_hash: string;
}

export interface StoredAuditRecord {
    correlation_id: string;
    timestamp: string;
    payload: string;
    proof: AuditProof;
}

export interface AuditTrailRequest {
    limit?: number;
    offset?: number;
    start_time?: string;
    end_time?: string;
    correlation_id?: string;
}

export interface AuditTrailResponse {
    records: StoredAuditRecord[];
    total_count: number;
    limit: number;
    offset: number;
}

export type ObligationStatus = 'Met' | 'Partial' | 'Gap' | 'NotApplicable';

export interface ObligationResult {
    id: string;
    name: string;
    legal_basis: string;
    status: ObligationStatus;
    detail: string | null;
    applicable_from: string | null;
}

export interface EuComplianceResult {
    risk_tier: 'Minimal' | 'Limited' | 'High' | 'Unacceptable';
    compliant: boolean;
    obligations: ObligationResult[];
    findings: { code: string; detail: string }[];
    scope_disclaimer: string;
}

export interface ComplianceResponse {
    correlation_id: string;
    status: 'Completed' | 'BlockedByFirewall' | 'BlockedBySemantic' | 'BlockedByInputModeration' | 'BlockedByOutputModeration' | 'BlockedByEuCompliance' | 'Sanitized';
    firewall: FirewallResult;
    semantic: SemanticResult | null;
    bias: BiasResult;
    input_moderation: ModerationResult | null;
    output_moderation: ModerationResult | null;
    generated_text: string | null;
    audit_proof: AuditProof;
    decision_evidence: DecisionEvidence | null;
    eu_compliance: EuComplianceResult | null;
}

export interface HealthStatus {
    status: 'ok' | 'error';
    version: string;
}
