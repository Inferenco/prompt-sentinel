import React from 'react';
import type { PipelineStatus } from './Pipeline';

interface StatusCardProps {
    status: PipelineStatus;
    loading: boolean;
}

export const StatusCard: React.FC<StatusCardProps> = ({ status, loading }) => {
    const isBlocked = status.includes('Blocked');

    const getStatusDisplay = () => {
        if (loading) return { text: 'Analyzing...', icon: '⏳', color: 'neutral' };
        if (status === 'Idle') return { text: 'Ready', icon: '⚡', color: 'neutral' };
        if (status === 'Completed') return { text: 'Allowed', icon: '✓', color: 'success' };
        if (status === 'Sanitized') return { text: 'Sanitized', icon: '⚠️', color: 'warning' };
        return { text: 'Blocked', icon: '✗', color: 'danger' };
    };

    const display = getStatusDisplay();

    return (
        <div className={`card status-card ${display.color}-border`}>
            <div className="card-header">
                <h2>Status</h2>
            </div>
            <div className="card-body">
                <div className={`status-main-badge bg-${display.color}-light text-${display.color}`}>
                    <span className="icon">{display.icon}</span>
                    <span className="text">{display.text}</span>
                </div>

                {isBlocked && (
                    <div className="blocked-details mt-4">
                        <span className="label text-danger">Blocked by:</span>
                        <div className="block-list">
                            <div className="block-item">
                                <span className="checkbox">{status === 'BlockedByFirewall' ? '☑' : '□'}</span>
                                <span>Firewall</span>
                            </div>
                            <div className="block-item">
                                <span className="checkbox">{status === 'BlockedBySemantic' ? '☑' : '□'}</span>
                                <span>Semantic Detection</span>
                            </div>
                            <div className="block-item">
                                <span className="checkbox">{status === 'BlockedByInputModeration' ? '☑' : '□'}</span>
                                <span>Input Moderation</span>
                            </div>
                            <div className="block-item">
                                <span className="checkbox">{status === 'BlockedByOutputModeration' ? '☑' : '□'}</span>
                                <span>Output Moderation</span>
                            </div>
                        </div>
                    </div>
                )}

                {status === 'Sanitized' && (
                    <div className="sanitized-details mt-4">
                        <span className="label text-warning">Elevated Risk:</span>
                        <p className="detail-text">
                            Request was allowed but with caution due to semantic similarity to known attack patterns.
                        </p>
                    </div>
                )}
            </div>
        </div>
    );
};
