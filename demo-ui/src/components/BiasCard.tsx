import React from 'react';
import type { BiasResult } from '../types';

interface BiasCardProps {
    result: BiasResult | null;
    loading: boolean;
}

export const BiasCard: React.FC<BiasCardProps> = ({ result, loading }) => {
    const getLevelColor = (level: string) => {
        switch (level) {
            case 'Low': return 'success';
            case 'Medium': return 'warning';
            case 'High': return 'danger';
            default: return 'neutral';
        }
    };

    const levelColor = result ? getLevelColor(result.level) : 'neutral';
    const scorePct = result ? Math.min(100, Math.max(0, result.score * 100)) : 0;

    return (
        <div className={`card bias-card ${loading ? 'loading' : ''} ${levelColor}-border`}>
            <div className="card-header">
                <h2>⚖️ BIAS ANALYSIS</h2>
            </div>
            <div className="card-body">
                {!result && !loading && <span className="empty-state">No data</span>}
                {loading && <span className="loading-state">Analyzing...</span>}
                {result && (
                    <div className="result-details">
                        <div className="score-container">
                            <div className="detail-row">
                                <span className="label">Score:</span>
                                <span className="value">{result.score.toFixed(2)}</span>
                            </div>
                            <div className="progress-bar-bg">
                                <div
                                    className={`progress-bar-fill bg-${levelColor}`}
                                    style={{ width: `${scorePct}%` }}
                                ></div>
                            </div>
                        </div>
                        <div className="detail-row">
                            <span className="label">Level:</span>
                            <span className={`badge badge-${levelColor}`}>{result.level}</span>
                        </div>
                        <div className="categories-section">
                            <span className="label">Categories detected:</span>
                            {result.categories.length === 0 ? (
                                <span className="value none-detected">None</span>
                            ) : (
                                <div className="chips-container">
                                    {result.categories.map((cat, idx) => (
                                        <span key={idx} className="chip">{cat}</span>
                                    ))}
                                </div>
                            )}
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};
