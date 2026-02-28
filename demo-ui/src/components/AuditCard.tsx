import React, { useState } from 'react';
import type { AuditProof } from '../types';

interface AuditCardProps {
    proof: AuditProof | null;
}

export const AuditCard: React.FC<AuditCardProps> = ({ proof }) => {
    const [expanded, setExpanded] = useState(false);

    if (!proof) return null;

    return (
        <div className="card audit-card full-width bg-slate-900 border-slate-700">
            <div
                className="card-header cursor-pointer flex justify-between items-center"
                onClick={() => setExpanded(!expanded)}
            >
                <h2>📜 AUDIT PROOF</h2>
                <span className="toggle-icon text-slate-400">{expanded ? '▲' : '▼'}</span>
            </div>
            {expanded && (
                <div className="card-body font-mono text-sm bg-black/50 p-4 rounded mt-2 border border-slate-800">
                    <div className="audit-row grid grid-cols-1 md:grid-cols-4 gap-2 mb-2 pb-2 border-b border-slate-800">
                        <span className="text-slate-400 font-bold self-center">Correlation ID:</span>
                        <span className="text-emerald-400 col-span-3 break-all bg-emerald-900/20 p-1 px-2 rounded">{proof.correlation_id}</span>
                    </div>
                    <div className="audit-row grid grid-cols-1 md:grid-cols-4 gap-2 mb-2 pb-2 border-b border-slate-800">
                        <span className="text-slate-400 font-bold self-center">Record Hash:</span>
                        <span className="text-slate-300 col-span-3 break-all text-xs bg-slate-800 p-1 px-2 rounded opacity-80">{proof.record_hash}</span>
                    </div>
                    <div className="audit-row grid grid-cols-1 md:grid-cols-4 gap-2">
                        <span className="text-slate-400 font-bold self-center">Chain Hash:</span>
                        <span className="text-slate-300 col-span-3 break-all text-xs bg-slate-800 p-1 px-2 rounded opacity-80">{proof.chain_hash}</span>
                    </div>
                </div>
            )}
        </div>
    );
};
