import type { SemanticResult } from '../types';

interface SemanticCardProps {
  result: SemanticResult | null;
  loading?: boolean;
}

export function SemanticCard({ result, loading }: SemanticCardProps) {
  const getRiskColor = (level: string | undefined) => {
    switch (level) {
      case 'High': return 'var(--danger)';
      case 'Medium': return 'var(--warning)';
      default: return 'var(--success)';
    }
  };

  const getRiskIcon = (level: string | undefined) => {
    switch (level) {
      case 'High': return '🔍';
      case 'Medium': return '⚠️';
      default: return '✓';
    }
  };

  return (
    <div className="card semantic-card">
      <div className="card-header">
        <h3>🧠 Semantic Detection</h3>
        <span className="badge" style={{
          backgroundColor: result ? getRiskColor(result.risk_level) : 'var(--bg-secondary)'
        }}>
          {loading ? '...' : result?.risk_level || 'N/A'}
        </span>
      </div>

      <div className="card-content">
        {loading ? (
          <div className="loading-placeholder">Analyzing semantic similarity...</div>
        ) : result ? (
          <>
            <div className="stat-row">
              <span className="stat-label">Risk Score</span>
              <span className="stat-value">{(result.risk_score * 100).toFixed(0)}%</span>
            </div>
            <div className="stat-row">
              <span className="stat-label">Similarity</span>
              <span className="stat-value">{(result.similarity * 100).toFixed(1)}%</span>
            </div>
            {result.nearest_template_id && (
              <div className="stat-row">
                <span className="stat-label">Matched Template</span>
                <span className="stat-value code">{result.nearest_template_id}</span>
              </div>
            )}
            {result.category && (
              <div className="stat-row">
                <span className="stat-label">Attack Category</span>
                <span className="stat-value">{result.category.replace(/_/g, ' ')}</span>
              </div>
            )}
            <div className="result-indicator">
              <span className="icon">{getRiskIcon(result.risk_level)}</span>
              <span className="text">
                {result.risk_level === 'High' ? 'Attack pattern detected' :
                 result.risk_level === 'Medium' ? 'Elevated risk detected' :
                 'No attack pattern detected'}
              </span>
            </div>
          </>
        ) : (
          <div className="empty-state">No semantic analysis yet</div>
        )}
      </div>
    </div>
  );
}
