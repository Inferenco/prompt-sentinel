import type { EuComplianceResult, ObligationStatus } from '../types';
import './EuComplianceCard.css';

interface EuComplianceCardProps {
  result: EuComplianceResult | null;
  loading?: boolean;
}

const statusIcon = (status: ObligationStatus): string => {
  switch (status) {
    case 'Met': return '✓';
    case 'Partial': return '◐';
    case 'Gap': return '✗';
    case 'NotApplicable': return '—';
  }
};

const statusClass = (status: ObligationStatus): string => {
  switch (status) {
    case 'Met': return 'status-met';
    case 'Partial': return 'status-partial';
    case 'Gap': return 'status-gap';
    case 'NotApplicable': return 'status-na';
  }
};

const riskTierClass = (tier: string): string => {
  switch (tier) {
    case 'Unacceptable': return 'tier-unacceptable';
    case 'High': return 'tier-high';
    case 'Limited': return 'tier-limited';
    case 'Minimal': return 'tier-minimal';
    default: return '';
  }
};

export function EuComplianceCard({ result, loading }: EuComplianceCardProps) {
  if (loading) {
    return (
      <div className="eu-compliance-card loading">
        <h3>EU AI Act Compliance</h3>
        <div className="loading-indicator">Analyzing...</div>
      </div>
    );
  }

  if (!result) {
    return (
      <div className="eu-compliance-card empty">
        <h3>EU AI Act Compliance</h3>
        <p className="empty-state">Submit a prompt to see compliance analysis</p>
      </div>
    );
  }

  return (
    <div className={`eu-compliance-card ${result.compliant ? 'compliant' : 'non-compliant'}`}>
      <div className="card-header">
        <h3>EU AI Act Compliance</h3>
        <span className={`risk-tier-badge ${riskTierClass(result.risk_tier)}`}>
          {result.risk_tier} Risk
        </span>
      </div>

      <div className="compliance-status">
        <span className={`status-badge ${result.compliant ? 'compliant' : 'non-compliant'}`}>
          {result.compliant ? 'Compliant' : 'Non-Compliant'}
        </span>
      </div>

      <div className="obligations-section">
        <h4>Obligations Status</h4>
        <table className="obligations-table">
          <thead>
            <tr>
              <th>Status</th>
              <th>Obligation</th>
              <th>Legal Basis</th>
              <th>Applicable</th>
            </tr>
          </thead>
          <tbody>
            {result.obligations.map((ob) => (
              <tr key={ob.id} className={statusClass(ob.status)}>
                <td className="status-cell">
                  <span className={`status-icon ${statusClass(ob.status)}`}>
                    {statusIcon(ob.status)}
                  </span>
                </td>
                <td>
                  <strong>{ob.name}</strong>
                  {ob.detail && <div className="obligation-detail">{ob.detail}</div>}
                </td>
                <td className="legal-basis">{ob.legal_basis}</td>
                <td className="applicable-date">{ob.applicable_from || '—'}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {result.findings.length > 0 && (
        <div className="findings-section">
          <h4>Findings</h4>
          <ul className="findings-list">
            {result.findings.map((finding, idx) => (
              <li key={idx} className="finding-item">
                <code className="finding-code">{finding.code}</code>
                <span className="finding-detail">{finding.detail}</span>
              </li>
            ))}
          </ul>
        </div>
      )}

      <div className="disclaimer-section">
        <p className="disclaimer">{result.scope_disclaimer}</p>
      </div>
    </div>
  );
}
