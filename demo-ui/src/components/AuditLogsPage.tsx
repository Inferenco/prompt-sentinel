import React, { useState, useEffect } from 'react';
import { api } from '../api';
import type { AuditTrailRequest, StoredAuditRecord } from '../types';

export const AuditLogsPage: React.FC = () => {
    const [records, setRecords] = useState<StoredAuditRecord[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [totalCount, setTotalCount] = useState(0);

    const [limit] = useState(10);
    const [offset, setOffset] = useState(0);

    // Filter controls
    const [correlationId, setCorrelationId] = useState('');

    const fetchLogs = async () => {
        setLoading(true);
        setError(null);
        try {
            const req: AuditTrailRequest = {
                limit,
                offset,
                ...(correlationId.trim() ? { correlation_id: correlationId.trim() } : {})
            };
            const response = await api.getAuditLogs(req);
            setRecords(response.records);
            setTotalCount(response.total_count);
        } catch (err: any) {
            setError(err.message || 'Failed to fetch audit logs');
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchLogs();
    }, [limit, offset]); // Fetch when pagination changes

    const handleSearch = (e: React.FormEvent) => {
        e.preventDefault();
        setOffset(0); // Reset to first page when searching
        fetchLogs();
    };

    const handleNextPage = () => {
        if (offset + limit < totalCount) {
            setOffset(offset + limit);
        }
    };

    const handlePrevPage = () => {
        if (offset - limit >= 0) {
            setOffset(offset - limit);
        }
    };

    return (
        <div className="audit-logs-page">
            <div className="audit-header">
                <h2>Audit Logs</h2>
                <p className="subtitle">View and verify system operation trails</p>
            </div>

            <div className="audit-controls">
                <form className="search-form" onSubmit={handleSearch}>
                    <input
                        type="text"
                        placeholder="Search by Correlation ID..."
                        value={correlationId}
                        onChange={(e) => setCorrelationId(e.target.value)}
                        className="search-input"
                    />
                    <button type="submit" className="search-btn">Search</button>
                    <button type="button" className="refresh-btn" onClick={fetchLogs} title="Refresh logs">↻</button>
                </form>
            </div>

            {error && (
                <div className="audit-error">
                    {error}
                </div>
            )}

            <div className="audit-table-container">
                <table className="audit-table">
                    <thead>
                        <tr>
                            <th>Timestamp</th>
                            <th>Correlation ID</th>
                            <th>Chain Hash</th>
                            <th>Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        {loading && records.length === 0 ? (
                            <tr>
                                <td colSpan={4} className="loading-state">Loading logs...</td>
                            </tr>
                        ) : records.length === 0 ? (
                            <tr>
                                <td colSpan={4} className="empty-state">No audit logs found.</td>
                            </tr>
                        ) : (
                            records.map((record, index) => (
                                <tr key={index}>
                                    <td>{new Date(record.timestamp).toLocaleString()}</td>
                                    <td className="font-mono text-sm break-all">{record.correlation_id}</td>
                                    <td className="font-mono text-xs text-slate-400 break-all">{record.proof.chain_hash.substring(0, 32)}...</td>
                                    <td>
                                        <button
                                            className="view-btn"
                                            onClick={() => alert(`Payload:\n${JSON.stringify(JSON.parse(record.payload), null, 2)}`)}
                                        >
                                            View DTO
                                        </button>
                                    </td>
                                </tr>
                            ))
                        )}
                    </tbody>
                </table>
            </div>

            <div className="pagination">
                <span className="pagination-info">
                    Showing {records.length > 0 ? offset + 1 : 0} to {Math.min(offset + limit, totalCount)} of {totalCount}
                </span>
                <div className="pagination-controls">
                    <button
                        disabled={offset === 0 || loading}
                        onClick={handlePrevPage}
                        className="page-btn"
                    >
                        Previous
                    </button>
                    <button
                        disabled={offset + limit >= totalCount || loading}
                        onClick={handleNextPage}
                        className="page-btn"
                    >
                        Next
                    </button>
                </div>
            </div>
        </div>
    );
};
