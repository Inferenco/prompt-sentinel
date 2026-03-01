import { useState, useEffect } from 'react';
import './App.css';
import { api } from './api';
import type { HealthStatus, FirewallResult, SemanticResult, BiasResult, AuditProof, DecisionEvidence, EuComplianceResult } from './types';
import type { PipelineStatus } from './components/Pipeline';

// Components
import { Header } from './components/Header';
import { TransparencyBanner } from './components/TransparencyBanner';
import { Dashboard } from './components/Dashboard';
import { AuditLogsPage } from './components/AuditLogsPage';

function App() {
  const [healthStatus, setHealthStatus] = useState<HealthStatus>({ status: 'unknown', version: 'unknown' } as any);
  const [currentPage, setCurrentPage] = useState<'dashboard' | 'audit_logs'>('dashboard');

  // Dashboard state
  const [prompt, setPrompt] = useState('');
  const [loading, setLoading] = useState(false);

  // Pipeline state
  const [status, setStatus] = useState<PipelineStatus>('Idle');
  const [activeStep, setActiveStep] = useState(0);
  const [timeMs, setTimeMs] = useState<number | null>(null);

  // Results
  const [firewallResult, setFirewallResult] = useState<FirewallResult | null>(null);
  const [semanticResult, setSemanticResult] = useState<SemanticResult | null>(null);
  const [biasResult, setBiasResult] = useState<BiasResult | null>(null);
  const [response, setResponse] = useState<string | null>(null);
  const [auditProof, setAuditProof] = useState<AuditProof | null>(null);
  const [decisionEvidence, setDecisionEvidence] = useState<DecisionEvidence | null>(null);
  const [correlationId, setCorrelationId] = useState<string | null>(null);
  const [euCompliance, setEuCompliance] = useState<EuComplianceResult | null>(null);

  useEffect(() => {
    // Check health on load
    api.checkHealth().then(setHealthStatus).catch(console.error);
    const interval = setInterval(() => {
      api.checkHealth().then(setHealthStatus).catch(console.error);
    }, 10000);
    return () => clearInterval(interval);
  }, []);

  const handleAnalyze = async () => {
    if (!prompt.trim()) return;

    setLoading(true);
    setStatus('Pending');
    setActiveStep(0);
    setTimeMs(null);
    setFirewallResult(null);
    setSemanticResult(null);
    setBiasResult(null);
    setResponse(null);
    setAuditProof(null);
    setDecisionEvidence(null);
    setCorrelationId(null);
    setEuCompliance(null);

    const startTime = Date.now();
    let progressInterval: ReturnType<typeof setInterval> | null = null;

    try {
      progressInterval = setInterval(() => {
        setActiveStep((prev) => (prev + 1) % 8);
      }, 400);

      const data = await api.checkCompliance(prompt);
      if (progressInterval) {
        clearInterval(progressInterval);
      }

      setCorrelationId(data.correlation_id);
      setFirewallResult(data.firewall);
      setSemanticResult(data.semantic);
      setBiasResult(data.bias);
      setResponse(data.generated_text ?? null);
      setAuditProof(data.audit_proof);
      setDecisionEvidence(data.decision_evidence);
      setEuCompliance(data.eu_compliance ?? null);
      setStatus(data.status);

      const finalStep =
        data.status === 'BlockedByFirewall' ? 1 :
          data.status === 'BlockedByEuCompliance' ? 2 :
            data.status === 'BlockedBySemantic' ? 3 :
              data.status === 'BlockedByInputModeration' ? 5 :
                data.status === 'BlockedByOutputModeration' ? 7 :
                  8;
      setActiveStep(finalStep);

      setLoading(false);
      setTimeMs(Date.now() - startTime);

    } catch (error) {
      console.error(error);
      setStatus('Idle');
      setLoading(false);
      alert('Failed to analyze prompt. Ensure backend is running.');
    } finally {
      if (progressInterval) {
        clearInterval(progressInterval);
      }
    }
  };

  return (
    <div className="app-container">
      <Header
        healthStatus={healthStatus.status}
        version={healthStatus.version}
        currentPage={currentPage}
        onNavigate={setCurrentPage}
      />
      <TransparencyBanner />

      <main className="main-content">
        {currentPage === 'dashboard' ? (
          <Dashboard
            prompt={prompt}
            setPrompt={setPrompt}
            loading={loading}
            status={status}
            activeStep={activeStep}
            timeMs={timeMs}
            firewallResult={firewallResult}
            semanticResult={semanticResult}
            biasResult={biasResult}
            response={response}
            auditProof={auditProof}
            decisionEvidence={decisionEvidence}
            correlationId={correlationId}
            euCompliance={euCompliance}
            handleAnalyze={handleAnalyze}
          />
        ) : (
          <AuditLogsPage />
        )}
      </main>
    </div>
  );
}

export default App;
