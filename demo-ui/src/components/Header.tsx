import React from 'react';

interface HeaderProps {
    healthStatus: 'unknown' | 'ok' | 'error';
    version: string;
}

export const Header: React.FC<HeaderProps> = ({ healthStatus, version }) => {
    return (
        <header className="app-header">
            <div className="logo-container">
                <h1>⚡ Prompt Sentinel</h1>
                <span className="version-badge">v{version || '1.0.0'}</span>
            </div>
            <div className="header-controls">
                <div className="health-indicator">
                    <span className="health-label">API Health:</span>
                    <div className={`health-dot ${healthStatus}`}></div>
                </div>
                <button className="settings-btn" title="Settings">⚙️ Settings</button>
            </div>
        </header>
    );
};
