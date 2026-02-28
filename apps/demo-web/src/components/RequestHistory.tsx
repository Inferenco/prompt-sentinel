import type { DemoRunRecord } from "../domain/types";
import { formatIsoTime, formatWorkflowStatus } from "../utils/format";

interface RequestHistoryProps {
  history: DemoRunRecord[];
  selectedId: string | null;
  onSelect: (recordId: string) => void;
}

export function RequestHistory({ history, selectedId, onSelect }: RequestHistoryProps) {
  return (
    <section className="panel request-history">
      <h2 className="panel__title">Recent Requests</h2>
      <p className="panel__hint">Up to 10 most recent runs. Select one to inspect details.</p>

      {history.length === 0 ? (
        <p className="panel__hint">No requests have been run yet.</p>
      ) : (
        <ul className="history-list" data-testid="history-list">
          {history.map((record) => (
            <li key={record.id}>
              <button
                type="button"
                className={`history-item ${selectedId === record.id ? "history-item--selected" : ""}`}
                onClick={() => onSelect(record.id)}
              >
                <span className="history-item__line">
                  <strong>{record.response ? formatWorkflowStatus(record.response.status) : "Request Failed"}</strong>
                  <span>{record.mode === "live" ? "Live" : "Mock"}</span>
                </span>
                <span className="history-item__line">
                  <span>{formatIsoTime(record.timestampIso)}</span>
                  <span>{record.response?.correlation_id ?? record.request.correlation_id ?? "auto"}</span>
                </span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
