import { useState, useEffect } from 'react';
import './App.css';
import { api } from './api';
import type { ComplianceResponse, HealthStatus, FirewallResult, SemanticResult, BiasResult, AuditProof, DecisionEvidence } from './types';
import type { PipelineStatus } from './components/Pipeline';

// Components
import { Header } from './components/Header';
import { PromptInput } from './components/PromptInput';
import { ExampleButtons } from './components/ExampleButtons';
import { Pipeline } from './components/Pipeline';
import { FirewallCard } from './components/FirewallCard';
import { SemanticCard } from './components/SemanticCard';
import { BiasCard } from './components/BiasCard';
import { StatusCard } from './components/StatusCard';
import { ResponseCard } from './components/ResponseCard';
import { AuditCard } from './components/AuditCard';
import { DecisionEvidenceCard } from './components/DecisionEvidenceCard';

function App() {
  const [healthStatus, setHealthStatus] = useState<HealthStatus>({ status: 'unknown', version: 'unknown' } as any);
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

    const startTime = Date.now();

    try {
      // Simulate pipeline steps animation (since API returns all at once)
      const animatePipeline = async (finalData: ComplianceResponse) => {
        setCorrelationId(finalData.correlation_id);
        const steps = [
          { ms: 300, step: 1 },  // Firewall
          { ms: 400, step: 2 },  // Semantic
          { ms: 300, step: 3 },  // Bias
          { ms: 400, step: 4 },  // Input Mod
          { ms: 1000, step: 5 }, // Gen
          { ms: 300, step: 6 },  // Output Mod
          { ms: 200, step: 7 }   // Audit
        ];

        for (let i = 0; i < steps.length; i++) {
          setActiveStep(i);
          await new Promise(r => setTimeout(r, steps[i].ms));

          if (i === 0) setFirewallResult(finalData.firewall);
          if (i === 1) setSemanticResult(finalData.semantic);
          if (i === 2) setBiasResult(finalData.bias);

          // Check for blocks at appropriate steps
          if (i === 0 && finalData.status === 'BlockedByFirewall') {
            setStatus('BlockedByFirewall');
            setAuditProof(finalData.audit_proof);
            setDecisionEvidence(finalData.decision_evidence);
            setLoading(false);
            setTimeMs(Date.now() - startTime);
            return;
          }
          if (i === 1 && finalData.status === 'BlockedBySemantic') {
            setStatus('BlockedBySemantic');
            setAuditProof(finalData.audit_proof);
            setDecisionEvidence(finalData.decision_evidence);
            setLoading(false);
            setTimeMs(Date.now() - startTime);
            return;
          }
          if (i === 3 && finalData.status === 'BlockedByInputModeration') {
            setStatus('BlockedByInputModeration');
            setAuditProof(finalData.audit_proof);
            setDecisionEvidence(finalData.decision_evidence);
            setLoading(false);
            setTimeMs(Date.now() - startTime);
            return;
          }
          if (i === 4 && finalData.generated_text) setResponse(finalData.generated_text);
          if (i === 5 && finalData.status === 'BlockedByOutputModeration') {
            setStatus('BlockedByOutputModeration');
            setResponse(null);
            setAuditProof(finalData.audit_proof);
            setDecisionEvidence(finalData.decision_evidence);
            setLoading(false);
            setTimeMs(Date.now() - startTime);
            return;
          }
        }

        setActiveStep(7);
        setStatus(finalData.status === 'Sanitized' ? 'Sanitized' : 'Completed');
        setAuditProof(finalData.audit_proof);
        setDecisionEvidence(finalData.decision_evidence);
        setLoading(false);
        setTimeMs(Date.now() - startTime);
      };

      const data = await api.checkCompliance(prompt);
      await animatePipeline(data);

    } catch (error) {
      console.error(error);
      setStatus('Idle');
      setLoading(false);
      alert('Failed to analyze prompt. Ensure backend is running.');
    }
  };

  return (
    <div className="app-container">
      <Header healthStatus={healthStatus.status} version={healthStatus.version} />

      <main className="main-content grid">
        {/* Top Row: Input and Pipeline */}
        <section className="top-row">
          <div className="input-section">
            <PromptInput
              value={prompt}
              onChange={setPrompt}
              onSubmit={handleAnalyze}
              loading={loading}
            />
            <ExampleButtons onSelect={setPrompt} disabled={loading} />
          </div>
          <div className="pipeline-section">
            <Pipeline status={status} activeStep={activeStep} timeMs={timeMs} />
          </div>
        </section>

        {/* Middle Row: Result Cards */}
        <section className="middle-row">
          <FirewallCard result={firewallResult} loading={loading && activeStep === 0} />
          <SemanticCard result={semanticResult} loading={loading && activeStep === 1} />
          <BiasCard result={biasResult} loading={loading && activeStep === 2} />
          <StatusCard status={status} loading={loading} />
        </section>

        {/* Bottom Row: Generated Response, Decision Evidence & Audit */}
        <section className="bottom-row">
          <div className="response-column">
            <ResponseCard response={response} status={status} loading={loading && activeStep === 4} />
          </div>
          <div className="evidence-column">
            <DecisionEvidenceCard evidence={decisionEvidence} />
            <AuditCard proof={auditProof} correlationId={correlationId ?? undefined} />
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;
