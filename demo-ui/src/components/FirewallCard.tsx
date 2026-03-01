import React from 'react';
import type { FirewallResult } from '../types';

interface FirewallCardProps {
    result: FirewallResult | null;
    loading: boolean;
}

export const FirewallCard: React.FC<FirewallCardProps> = ({ result, loading }) => {
    const getActionColor = (action: string | undefined) => {
        switch (action) {
            case 'Block': return 'danger';
            case 'Sanitize': return 'warning';
            case 'Allow': return 'success';
            default: return 'neutral';
        }
    };

    const actionColor = getActionColor(result?.action);

    return (
        <div className={`card firewall-card ${loading ? 'loading' : ''} ${actionColor}-border`}>
            <div className="card-header">
                <h2>Firewall</h2>
            </div>
            <div className="card-body">
                {!result && !loading && <span className="empty-state">No data</span>}
                {loading && <span className="loading-state">Analyzing...</span>}
                {result && (
                    <div className="result-details">
                        <div className="detail-row">
                            <span className="label">Action:</span>
                            <span className={`badge badge-${actionColor}`}>{result.action}</span>
                        </div>
                        <div className="detail-row">
                            <span className="label">Severity:</span>
                            <span className="value">{result.severity}</span>
                        </div>
                        {result.matched_rules.length > 0 && (
                            <>
                                <div className="detail-row">
                                    <span className="label">Matched Rules:</span>
                                    <span className="value">{result.matched_rules.length}</span>
                                </div>
                                <ul className="rules-list">
                                    {result.matched_rules.map((rule, idx) => (
                                        <li key={idx} className="rule-tag">{rule}</li>
                                    ))}
                                </ul>
                            </>
                        )}
                        {result.reasons.length > 0 && (
                            <div className="detail-row reasons">
                                <span className="label">Reasons:</span>
                                <ul className="reasons-list">
                                    {result.reasons.map((reason, idx) => (
                                        <li key={idx}>{reason}</li>
                                    ))}
                                </ul>
                            </div>
                        )}
                    </div>
                )}
            </div>
        </div>
    );
};
