import type { ComplianceResponse, HealthStatus } from './types';

const API_BASE_URL = 'http://localhost:3000';

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

    checkHealth: async (): Promise<HealthStatus> => {
        try {
            const response = await fetch(`${API_BASE_URL}/health`);
            if (!response.ok) {
                throw new Error('Health check failed');
            }
            return response.json();
        } catch (e) {
            return { status: 'error', version: 'unknown' };
        }
    },
};
