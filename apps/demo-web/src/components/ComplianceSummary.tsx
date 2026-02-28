import type { DemoRunRecord } from "../domain/types";
import { formatIsoTime, formatPercent, formatWorkflowStatus } from "../utils/format";

interface ComplianceSummaryProps {
  record: DemoRunRecord | null;
}

function statusClass(status: string): string {
  if (status === "Completed") {
    return "pill pill--good";
  }
  if (status.startsWith("Blocked")) {
    return "pill pill--bad";
  }
  return "pill pill--neutral";
}

export function ComplianceSummary({ record }: ComplianceSummaryProps) {
  if (!record) {
    return (
      <section className="panel summary" aria-live="polite">
        <h2 className="panel__title">Outcome Summary</h2>
        <p className="panel__hint">Run a compliance check to inspect status, bias, moderation, and audit proof.</p>
      </section>
    );
  }

  return (
    <section className="panel summary" aria-live="polite">
      <h2 className="panel__title">Outcome Summary</h2>
      <div className="summary-grid">
        <article className="summary-metric">
          <span className="summary-metric__label">Status</span>
          {record.response ? (
            <span className={statusClass(record.response.status)} data-testid="summary-status">
              {formatWorkflowStatus(record.response.status)}
            </span>
          ) : (
            <span className="pill pill--bad" data-testid="summary-status">
              Request Failed
            </span>
          )}
        </article>

        <article className="summary-metric">
          <span className="summary-metric__label">Correlation ID</span>
          <span className="summary-metric__value">
            {record.response?.correlation_id ?? record.request.correlation_id ?? "(auto-generated)"}
          </span>
        </article>

        <article className="summary-metric">
          <span className="summary-metric__label">Bias score</span>
          <span className="summary-metric__value">
            {record.response ? formatPercent(record.response.bias.score) : "N/A"}
          </span>
        </article>

        <article className="summary-metric">
          <span className="summary-metric__label">Run time</span>
          <span className="summary-metric__value">{formatIsoTime(record.timestampIso)}</span>
        </article>
      </div>

      {record.error ? (
        <p className="error-text">{record.error}</p>
      ) : record.response?.generated_text ? (
        <article className="generated-text">
          <h3>Generated text</h3>
          <p>{record.response.generated_text}</p>
        </article>
      ) : (
        <article className="generated-text">
          <h3>No generated output</h3>
          <p>This request was blocked before or after generation based on compliance controls.</p>
        </article>
      )}
    </section>
  );
}
