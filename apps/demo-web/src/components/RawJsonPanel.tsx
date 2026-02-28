import type { ComplianceRequest, ComplianceResponse } from "../domain/types";

interface RawJsonPanelProps {
  request: ComplianceRequest | null;
  response: ComplianceResponse | null;
  error: string | null;
}

export function RawJsonPanel({ request, response, error }: RawJsonPanelProps) {
  const hasData = Boolean(request || response || error);

  return (
    <section className="panel raw-json" data-testid="raw-json-panel">
      <h2 className="panel__title">Raw JSON Inspector</h2>
      <p className="panel__hint">Runtime payloads for technical verification and contract debugging.</p>

      {hasData ? (
        <div className="json-grid">
          <article>
            <h3>Request</h3>
            <pre data-testid="json-request">{JSON.stringify(request, null, 2)}</pre>
          </article>

          <article>
            <h3>Response</h3>
            <pre data-testid="json-response">
              {error ? JSON.stringify({ error }, null, 2) : JSON.stringify(response, null, 2)}
            </pre>
          </article>
        </div>
      ) : (
        <p className="panel__hint">No payloads captured yet.</p>
      )}
    </section>
  );
}
