export interface ModerationResult {
    flagged: boolean;
    severity: number;
    categories: string[];
}

export interface FirewallResult {
    action: 'ALLOW' | 'BLOCK';
    severity: 'Low' | 'Medium' | 'High' | 'None';
    matched_rules: string[];
    sanitized: boolean;
}

export interface BiasResult {
    score: number;
    level: 'Low' | 'Medium' | 'High';
    categories: string[];
}

export interface AuditProof {
    correlation_id: string;
    record_hash: string;
    chain_hash: string;
}

export interface ComplianceResponse {
    correlation_id: string;
    status: 'Completed' | 'BlockedByFirewall' | 'BlockedByInputModeration' | 'BlockedByOutputModeration';
    firewall: FirewallResult;
    bias: BiasResult;
    input_moderation: ModerationResult;
    output_moderation: ModerationResult;
    generated_text: string | null;
    audit_proof: AuditProof;
}

export interface HealthStatus {
    status: 'ok' | 'error';
    version: string;
}
