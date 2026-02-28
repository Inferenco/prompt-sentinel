import React from 'react';
import type { FirewallResult } from '../types';

interface FirewallCardProps {
    result: FirewallResult | null;
    loading: boolean;
}

export const FirewallCard: React.FC<FirewallCardProps> = ({ result, loading }) => {
    const actionColor = result?.action === 'ALLOW' ? 'success' : result?.action === 'BLOCK' ? 'danger' : 'neutral';

    return (
        <div className={`card firewall-card ${loading ? 'loading' : ''} ${actionColor}-border`}>
            <div className="card-header">
                <h2>🛡️ FIREWALL</h2>
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
                        <div className="detail-row">
                            <span className="label">Matched Rules:</span>
                            <span className="value">{result.matched_rules.length}</span>
                        </div>
                        {result.matched_rules.length > 0 && (
                            <ul className="rules-list">
                                {result.matched_rules.map((rule, idx) => (
                                    <li key={idx}>{rule}</li>
                                ))}
                            </ul>
                        )}
                        <div className="detail-row">
                            <span className="label">Sanitized:</span>
                            <span className="value">{result.sanitized ? 'Yes' : 'No'}</span>
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};
