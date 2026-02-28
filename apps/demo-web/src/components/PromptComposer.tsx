import type { DemoMode } from "../domain/types";

interface PromptComposerProps {
  mode: DemoMode;
  apiBaseUrl: string;
  prompt: string;
  correlationId: string;
  running: boolean;
  runError: string | null;
  onModeChange: (mode: DemoMode) => void;
  onApiBaseUrlChange: (value: string) => void;
  onPromptChange: (value: string) => void;
  onCorrelationIdChange: (value: string) => void;
  onRun: () => void;
  onClear: () => void;
}

export function PromptComposer({
  mode,
  apiBaseUrl,
  prompt,
  correlationId,
  running,
  runError,
  onModeChange,
  onApiBaseUrlChange,
  onPromptChange,
  onCorrelationIdChange,
  onRun,
  onClear,
}: PromptComposerProps) {
  return (
    <section className="panel composer" aria-label="Compliance request composer">
      <h2 className="panel__title">Compliance Check</h2>
      <p className="panel__hint">
        Submit a prompt and inspect each safety stage from firewall to audit proof.
      </p>

      <fieldset className="mode-toggle" aria-label="Data mode">
        <legend>Data Mode</legend>
        <label htmlFor="mode-live" className="mode-toggle__option">
          <input
            id="mode-live"
            name="mode"
            type="radio"
            checked={mode === "live"}
            onChange={() => onModeChange("live")}
          />
          Live mode
        </label>
        <label htmlFor="mode-mock" className="mode-toggle__option">
          <input
            id="mode-mock"
            name="mode"
            type="radio"
            checked={mode === "mock"}
            onChange={() => onModeChange("mock")}
          />
          Mock mode
        </label>
      </fieldset>

      {mode === "live" ? (
        <label className="field" htmlFor="api-base-url">
          <span className="field__label">API base URL</span>
          <input
            id="api-base-url"
            type="url"
            value={apiBaseUrl}
            onChange={(event) => onApiBaseUrlChange(event.target.value)}
            placeholder="http://localhost:3000"
            autoComplete="off"
            spellCheck={false}
          />
        </label>
      ) : (
        <p className="composer__mode-note">
          Mock mode returns deterministic responses for all workflow statuses.
        </p>
      )}

      <label className="field" htmlFor="correlation-id">
        <span className="field__label">Correlation ID (optional)</span>
        <input
          id="correlation-id"
          type="text"
          value={correlationId}
          onChange={(event) => onCorrelationIdChange(event.target.value)}
          placeholder="demo-correlation-001"
          autoComplete="off"
          spellCheck={false}
        />
      </label>

      <label className="field" htmlFor="prompt-input">
        <span className="field__label">Prompt</span>
        <textarea
          id="prompt-input"
          data-testid="prompt-input"
          value={prompt}
          onChange={(event) => onPromptChange(event.target.value)}
          placeholder="Type a prompt to evaluate through Prompt Sentinel..."
          rows={7}
          required
        />
      </label>

      {runError ? <p className="error-text">{runError}</p> : null}

      <div className="composer__actions composer__actions--desktop">
        <button
          type="button"
          className="button button--primary"
          onClick={onRun}
          disabled={running}
          data-testid="desktop-run-check-btn"
        >
          {running ? "Running compliance check..." : "Run compliance check"}
        </button>
        <button
          type="button"
          className="button button--secondary"
          onClick={onClear}
          disabled={running}
          data-testid="desktop-clear-btn"
        >
          Clear
        </button>
      </div>
    </section>
  );
}
