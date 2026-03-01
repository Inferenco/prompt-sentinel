import type { DecisionEvidence } from '../types';

interface DecisionEvidenceCardProps {
  evidence: DecisionEvidence | null;
}

export function DecisionEvidenceCard({ evidence }: DecisionEvidenceCardProps) {
  if (!evidence) {
    return null;
  }

  const getDecisionColor = () => {
    switch (evidence.final_decision) {
      case 'block': return 'var(--danger)';
      case 'sanitize': return 'var(--warning)';
      default: return 'var(--success)';
    }
  };

  const getDecisionIcon = () => {
    switch (evidence.final_decision) {
      case 'block': return '🚫';
      case 'sanitize': return '⚠️';
      default: return '✅';
    }
  };

  return (
    <div className="card decision-evidence-card">
      <div className="card-header">
        <h3>📋 Decision Evidence</h3>
        <span className="badge" style={{ backgroundColor: getDecisionColor() }}>
          {evidence.final_decision.toUpperCase()}
        </span>
      </div>

      <div className="card-content">
        <div className="evidence-reason">
          <span className="icon">{getDecisionIcon()}</span>
          <span className="reason-text">{evidence.final_reason}</span>
        </div>

        <div className="evidence-details">
          <div className="evidence-section">
            <h4>Detection Layers</h4>
            <table className="evidence-table">
              <tbody>
                <tr>
                  <td className="label">Firewall</td>
                  <td className="value">{evidence.firewall_action}</td>
                </tr>
                {evidence.firewall_matched_rules.length > 0 && (
                  <tr>
                    <td className="label">Matched Rules</td>
                    <td className="value">
                      {evidence.firewall_matched_rules.map((rule, i) => (
                        <span key={i} className="rule-tag">{rule}</span>
                      ))}
                    </td>
                  </tr>
                )}
                {evidence.semantic_risk_score !== null && (
                  <tr>
                    <td className="label">Semantic Risk</td>
                    <td className="value">{(evidence.semantic_risk_score * 100).toFixed(0)}%</td>
                  </tr>
                )}
                {evidence.semantic_matched_template && (
                  <tr>
                    <td className="label">Matched Pattern</td>
                    <td className="value code">{evidence.semantic_matched_template}</td>
                  </tr>
                )}
                {evidence.semantic_category && (
                  <tr>
                    <td className="label">Attack Category</td>
                    <td className="value">{evidence.semantic_category.replace(/_/g, ' ')}</td>
                  </tr>
                )}
                <tr>
                  <td className="label">Moderation</td>
                  <td className="value">
                    {evidence.moderation_flagged ? (
                      <span className="flagged">Flagged: {evidence.moderation_categories.join(', ')}</span>
                    ) : (
                      <span className="passed">Passed</span>
                    )}
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}
