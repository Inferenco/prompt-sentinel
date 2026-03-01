import type { ComplianceResponse, HealthStatus, AuditTrailRequest, AuditTrailResponse } from './types';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3200';

export const api = {
    checkCompliance: async (prompt: string): Promise<ComplianceResponse> => {
        const response = await fetch(`${API_BASE_URL}/api/compliance/check`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ prompt }),
        });

        if (!response.ok) {
            throw new Error(`API error: ${response.statusText}`);
        }

        return response.json();
    },

    getAuditLogs: async (request: AuditTrailRequest): Promise<AuditTrailResponse> => {
        const response = await fetch(`${API_BASE_URL}/api/audit/trail`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(request),
        });

        if (!response.ok) {
            throw new Error(`API error: ${response.statusText}`);
        }

        return response.json();
    },

    checkHealth: async (): Promise<HealthStatus> => {
        try {
            const response = await fetch(`${API_BASE_URL}/health`);
            if (response.ok) {
                const text = await response.text();
                return { status: text === 'OK' ? 'ok' : 'error', version: '0.1.0' };
            }
            return { status: 'error', version: 'unknown' };
        } catch {
            return { status: 'error', version: 'unknown' };
        }
    },
};
