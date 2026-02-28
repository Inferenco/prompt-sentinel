import { useEffect, useState } from "react";

import { AuditProofCard } from "./components/AuditProofCard";
import { ComplianceSummary } from "./components/ComplianceSummary";
import { HealthPanel } from "./components/HealthPanel";
import { PromptComposer } from "./components/PromptComposer";
import { RawJsonPanel } from "./components/RawJsonPanel";
import { RequestHistory } from "./components/RequestHistory";
import { ScenarioChips } from "./components/ScenarioChips";
import { WorkflowStages } from "./components/WorkflowStages";
import { DesktopSidePanel } from "./layout/DesktopSidePanel";
import { AppShell } from "./layout/AppShell";
import { MobileActionBar } from "./layout/MobileActionBar";
import { SCENARIO_PRESETS } from "./sdk/mockFixtures";
import { useDemoState } from "./state/useDemoState";

function useDesktopLayout(breakpoint = "(min-width: 64rem)"): boolean {
  const [isDesktop, setIsDesktop] = useState<boolean>(() =>
    typeof window === "undefined" ? false : window.matchMedia(breakpoint).matches,
  );

  useEffect(() => {
    if (typeof window === "undefined") {
      return () => {};
    }

    const media = window.matchMedia(breakpoint);
    const listener = () => setIsDesktop(media.matches);
    listener();

    media.addEventListener("change", listener);
    return () => media.removeEventListener("change", listener);
  }, [breakpoint]);

  return isDesktop;
}

export default function App() {
  const state = useDemoState();
  const isDesktop = useDesktopLayout();

  const activeRecord = state.activeRecord;
  const activeResponse = activeRecord?.response ?? null;

  const header = (
    <header className="app-hero">
      <h1>Prompt Sentinel SDK Demo</h1>
      <p>
        Interactive frontend for judges and developers to verify runtime safety layers: firewall,
        bias detection, moderation, and audit proof-chain output.
      </p>
      <div className="hero-tags" aria-label="Demo scope tags">
        <span className="hero-tag">Runtime contract aligned</span>
        <span className="hero-tag">Mobile and web parity</span>
        <span className="hero-tag">Live plus mock mode</span>
      </div>
    </header>
  );

  const main = (
    <>
      <HealthPanel mode={state.mode} health={state.health} onRefresh={() => void state.refreshHealth()} />

      <PromptComposer
        mode={state.mode}
        apiBaseUrl={state.apiBaseUrl}
        prompt={state.prompt}
        correlationId={state.correlationId}
        running={state.isRunning}
        runError={state.runError}
        onModeChange={state.setMode}
        onApiBaseUrlChange={state.setApiBaseUrl}
        onPromptChange={state.setPrompt}
        onCorrelationIdChange={state.setCorrelationId}
        onRun={() => void state.runCompliance()}
        onClear={state.clearComposer}
      />

      <ScenarioChips
        scenarios={SCENARIO_PRESETS}
        selectedId={state.selectedScenarioId}
        onSelect={state.applyScenario}
        disabled={state.isRunning}
      />

      <ComplianceSummary record={activeRecord} />
      <WorkflowStages response={activeResponse} />
      <AuditProofCard proof={activeResponse?.audit_proof ?? null} />

      {!isDesktop ? (
        <>
          <RawJsonPanel
            request={activeRecord?.request ?? null}
            response={activeRecord?.response ?? null}
            error={activeRecord?.error ?? null}
          />

          <RequestHistory
            history={state.history}
            selectedId={activeRecord?.id ?? null}
            onSelect={state.selectHistoryRecord}
          />
        </>
      ) : null}
    </>
  );

  const side = isDesktop ? (
    <DesktopSidePanel>
      <RawJsonPanel
        request={activeRecord?.request ?? null}
        response={activeRecord?.response ?? null}
        error={activeRecord?.error ?? null}
      />

      <RequestHistory
        history={state.history}
        selectedId={activeRecord?.id ?? null}
        onSelect={state.selectHistoryRecord}
      />
    </DesktopSidePanel>
  ) : null;

  const mobileFooter = !isDesktop ? (
    <MobileActionBar
      running={state.isRunning}
      disabled={state.isRunning}
      onRun={() => void state.runCompliance()}
      onClear={state.clearComposer}
    />
  ) : null;

  return <AppShell header={header} main={main} side={side} mobileFooter={mobileFooter} />;
}
