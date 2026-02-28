import React from 'react';

export type PipelineStatus = 'Idle' | 'Pending' | 'Completed' | 'BlockedByFirewall' | 'BlockedByInputModeration' | 'BlockedByOutputModeration';

interface PipelineProps {
    status: PipelineStatus;
    activeStep: number;
    timeMs: number | null;
}

const STEPS = [
    'Firewall Check',
    'Bias Detection',
    'Input Moderation',
    'Generation',
    'Output Moderation',
    'Audit Logging'
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
            if (status === 'Completed') return 'completed';
            return 'active';
        }
        return 'idle';
    };

    return (
        <div className="card pipeline-card">
            <div className="card-header">
                <h2>🔄 COMPLIANCE PIPELINE</h2>
            </div>
            <div className="card-body">
                <div className="pipeline-steps">
                    {STEPS.map((step, index) => {
                        const stepStatus = getStepStatus(index);
                        return (
                            <div key={step} className={`pipeline-step ${stepStatus}`}>
                                <div className="step-indicator">
                                    {stepStatus === 'completed' && '✓'}
                                    {stepStatus === 'active' && <span className="spinner"></span>}
                                    {stepStatus === 'blocked' && '✗'}
                                    {stepStatus === 'idle' && '○'}
                                </div>
                                <span className="step-label">{step}</span>
                            </div>
                        );
                    })}
                </div>
                <div className="pipeline-footer">
                    <span className="time-label">Time: {timeMs ? `${timeMs}ms` : '--ms'}</span>
                </div>
            </div>
        </div>
    );
};
