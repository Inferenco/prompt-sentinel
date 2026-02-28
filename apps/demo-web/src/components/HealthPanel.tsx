import type { DemoMode, HealthState } from "../domain/types";
import { formatIsoTime } from "../utils/format";

interface HealthPanelProps {
  mode: DemoMode;
  health: HealthState;
  onRefresh: () => void;
}

function servicePillClass(status: string): string {
  if (status === "healthy") {
    return "pill pill--good";
  }
  if (status === "unhealthy" || status === "error") {
    return "pill pill--bad";
  }
  return "pill pill--neutral";
}

export function HealthPanel({ mode, health, onRefresh }: HealthPanelProps) {
  return (
    <section className="panel health-panel" data-testid="health-panel">
      <div className="panel__head">
        <h2 className="panel__title">Health and Readiness</h2>
        <button
          type="button"
          className="button button--secondary"
          onClick={onRefresh}
          disabled={health.loading}
        >
          {health.loading ? "Refreshing..." : "Refresh"}
        </button>
      </div>

      <div className="health-grid" aria-live="polite">
        <article className="health-metric">
          <span className="health-metric__label">Mode</span>
          <span className="health-metric__value">{mode === "live" ? "Live API" : "Mock"}</span>
        </article>

        <article className="health-metric">
          <span className="health-metric__label">Service health</span>
          <span className={servicePillClass(health.serviceStatus)}>{health.serviceStatus}</span>
        </article>

        <article className="health-metric">
          <span className="health-metric__label">Mistral health</span>
          <span className={servicePillClass(health.mistralStatus)}>{health.mistralStatus}</span>
        </article>

        <article className="health-metric">
          <span className="health-metric__label">Last checked</span>
          <span className="health-metric__value">
            {health.lastCheckedAt ? formatIsoTime(health.lastCheckedAt) : "Not checked yet"}
          </span>
        </article>
      </div>

      <p className="health-panel__message">{health.message}</p>

      {health.models.length > 0 ? (
        <ul className="model-list">
          {health.models.map((model, index) => (
            <li key={`${model ?? "null"}-${index}`} className="model-list__item">
              {model ?? "No moderation model configured"}
            </li>
          ))}
        </ul>
      ) : null}

      {health.error ? <p className="error-text">{health.error}</p> : null}
    </section>
  );
}
