import React from 'react';

interface HeaderProps {
    healthStatus: 'unknown' | 'ok' | 'error';
    version: string;
    currentPage: 'dashboard' | 'audit_logs';
    onNavigate: (page: 'dashboard' | 'audit_logs') => void;
}

export const Header: React.FC<HeaderProps> = ({ healthStatus, version, currentPage, onNavigate }) => {
    return (
        <header className="app-header">
            <div className="logo-container">
                <h1>⚡ Prompt Sentinel</h1>
                <span className="version-badge">v{version || '1.0.0'}</span>
            </div>
            <nav className="header-nav">
                <button
                    className={`nav-btn ${currentPage === 'dashboard' ? 'active' : ''}`}
                    onClick={() => onNavigate('dashboard')}
                >
                    Dashboard
                </button>
                <button
                    className={`nav-btn ${currentPage === 'audit_logs' ? 'active' : ''}`}
                    onClick={() => onNavigate('audit_logs')}
                >
                    Audit Logs
                </button>
            </nav>
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
