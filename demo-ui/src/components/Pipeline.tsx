import React from 'react';

export type PipelineStatus = 'Idle' | 'Pending' | 'Completed' | 'Sanitized' | 'BlockedByFirewall' | 'BlockedBySemantic' | 'BlockedByInputModeration' | 'BlockedByOutputModeration' | 'BlockedByEuCompliance';

interface PipelineProps {
    status: PipelineStatus;
    activeStep: number;
    timeMs: number | null;
}

const STEPS = [
    { name: 'Firewall', icon: '🛡️' },
    { name: 'EU Compliance', icon: '🇪🇺' },
    { name: 'Semantic', icon: '🧠' },
    { name: 'Bias', icon: '⚖️' },
    { name: 'Input Mod', icon: '📥' },
    { name: 'Generate', icon: '✨' },
    { name: 'Output Mod', icon: '📤' },
    { name: 'Audit', icon: '📋' }
];

export const Pipeline: React.FC<PipelineProps> = ({ status, activeStep, timeMs }) => {
    const getStepStatus = (index: number) => {
        if (status === 'Idle') return 'idle';
        if (index < activeStep) {
            if (status.includes('Blocked') && index === activeStep - 1) {
                return 'blocked';
            }
            return 'completed';
        }
        if (index === activeStep) {
            if (status.includes('Blocked')) return 'blocked';
            if (status === 'Completed' || status === 'Sanitized') return 'completed';
            return 'active';
        }
        return 'idle';
    };

    const getStatusMessage = () => {
        switch (status) {
            case 'Idle': return 'Ready';
            case 'Pending': return 'Processing...';
            case 'Completed': return 'Allowed';
            case 'Sanitized': return 'Caution & Allowed';
            case 'BlockedByFirewall': return 'Blocked by Firewall';
            case 'BlockedByEuCompliance': return 'Blocked by EU AI Act (Article 5)';
            case 'BlockedBySemantic': return 'Blocked by Semantic Detection';
            case 'BlockedByInputModeration': return 'Blocked by Input Moderation';
            case 'BlockedByOutputModeration': return 'Blocked by Output Moderation';
            default: return status;
        }
    };

    const getStatusColor = () => {
        if (status === 'Idle' || status === 'Pending') return 'neutral';
        if (status.includes('Blocked')) return 'danger';
        if (status === 'Sanitized') return 'warning';
        return 'success';
    };

    return (
        <div className="card pipeline-card">
            <div className="card-header">
                <h2>Defense-in-Depth Pipeline</h2>
                <span className={`status-badge ${getStatusColor()}`}>{getStatusMessage()}</span>
            </div>
            <div className="card-body">
                <div className="pipeline-steps">
                    {STEPS.map((step, index) => {
                        const stepStatus = getStepStatus(index);
                        return (
                            <div key={step.name} className={`pipeline-step ${stepStatus}`}>
                                <div className="step-indicator">
                                    {stepStatus === 'completed' && '✓'}
                                    {stepStatus === 'active' && <span className="spinner"></span>}
                                    {stepStatus === 'blocked' && '✗'}
                                    {stepStatus === 'idle' && step.icon}
                                </div>
                                <span className="step-label">{step.name}</span>
                            </div>
                        );
                    })}
                </div>
                <div className="pipeline-footer">
                    <span className="time-label">Time: {timeMs ? `${timeMs}ms` : '--'}</span>
                </div>
            </div>
        </div>
    );
};
