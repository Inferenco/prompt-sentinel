import React from 'react';
import type {
    FirewallResult,
    SemanticResult,
    BiasResult,
    AuditProof,
    DecisionEvidence,
    EuComplianceResult
} from '../types';
import type { PipelineStatus } from './Pipeline';

// Components
import { PromptInput } from './PromptInput';
import { ExampleButtons } from './ExampleButtons';
import { Pipeline } from './Pipeline';
import { FirewallCard } from './FirewallCard';
import { SemanticCard } from './SemanticCard';
import { BiasCard } from './BiasCard';
import { StatusCard } from './StatusCard';
import { ResponseCard } from './ResponseCard';
import { AuditCard } from './AuditCard';
import { DecisionEvidenceCard } from './DecisionEvidenceCard';
import { EuComplianceCard } from './EuComplianceCard';

interface DashboardProps {
    prompt: string;
    setPrompt: (value: string) => void;
    loading: boolean;
    status: PipelineStatus;
    activeStep: number;
    timeMs: number | null;
    firewallResult: FirewallResult | null;
    semanticResult: SemanticResult | null;
    biasResult: BiasResult | null;
    response: string | null;
    auditProof: AuditProof | null;
    decisionEvidence: DecisionEvidence | null;
    correlationId: string | null;
    euCompliance: EuComplianceResult | null;
    handleAnalyze: () => void;
}

export const Dashboard: React.FC<DashboardProps> = (props) => {
    return (
        <div className="dashboard-grid">
            <div className="dashboard-top">
                {/* Left Panel: Inputs and Core Analysis */}
                <div className="left-panel">
                    <PromptInput
                        value={props.prompt}
                        onChange={props.setPrompt}
                        onSubmit={props.handleAnalyze}
                        loading={props.loading}
                    />
                    <ExampleButtons onSelect={props.setPrompt} disabled={props.loading} />

                    <div className="analysis-cards">
                        <FirewallCard result={props.firewallResult} loading={props.loading && props.activeStep === 0} />
                        <SemanticCard result={props.semanticResult} loading={props.loading && props.activeStep === 1} />
                        <BiasCard result={props.biasResult} loading={props.loading && props.activeStep === 2} />
                    </div>
                </div>

                {/* Right Panel: Pipeline and Status */}
                <div className="right-panel">
                    <Pipeline status={props.status} activeStep={props.activeStep} timeMs={props.timeMs} />
                    <StatusCard status={props.status} loading={props.loading} />
                </div>
            </div>

            {/* Compliance Row: EU AI Act Compliance */}
            <div className="compliance-row">
                <EuComplianceCard result={props.euCompliance} loading={props.loading} />
            </div>

            {/* Bottom Row: Generated Response, Decision Evidence & Audit */}
            <div className="bottom-row">
                <div className="response-column">
                    <ResponseCard
                        response={props.response}
                        status={props.status}
                        loading={props.loading && props.activeStep === 4}
                    />
                </div>
                <div className="evidence-column">
                    <DecisionEvidenceCard evidence={props.decisionEvidence} />
                    <AuditCard proof={props.auditProof} correlationId={props.correlationId ?? undefined} />
                </div>
            </div>
        </div>
    );
};
