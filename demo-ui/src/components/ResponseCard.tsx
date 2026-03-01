import React from 'react';
import type { PipelineStatus } from './Pipeline';

interface ResponseCardProps {
    response: string | null;
    status: PipelineStatus;
    loading: boolean;
}

export const ResponseCard: React.FC<ResponseCardProps> = ({ response, status, loading }) => {
    const isBlocked = status.includes('Blocked');

    return (
        <div className="card response-card full-width">
            <div className="card-header">
                <h2>💬 GENERATED RESPONSE</h2>
            </div>
            <div className="card-body min-h-[150px]">
                {loading && (
                    <div className="typing-indicator flex gap-1">
                        <span className="dot"></span>
                        <span className="dot"></span>
                        <span className="dot"></span>
                    </div>
                )}
                {!loading && status === 'Idle' && (
                    <span className="empty-state">Waiting for prompt...</span>
                )}
                {!loading && isBlocked && (
                    <div className="blocked-message text-danger p-4 border border-danger-light rounded bg-danger-transparent">
                        <h3>Response Blocked</h3>
                        <p>The generation was halted because the prompt or response violated security/compliance policies.</p>
                    </div>
                )}
                {!loading && response && (status === 'Completed' || status === 'Sanitized') && (
                    <div className="response-text">
                        {response.split('\\n').map((line, i) => (
                            <React.Fragment key={i}>
                                {line}
                                <br />
                            </React.Fragment>
                        ))}
                    </div>
                )}
            </div>
        </div>
    );
};
