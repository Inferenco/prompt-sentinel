import type { ComplianceResponse } from "../domain/types";

interface WorkflowStagesProps {
  response: ComplianceResponse | null;
}

export function WorkflowStages({ response }: WorkflowStagesProps) {
  if (!response) {
    return (
      <section className="panel workflow-stages">
        <h2 className="panel__title">Workflow Stages</h2>
        <p className="panel__hint">Detailed stage outcomes appear after a compliance run.</p>
      </section>
    );
  }

  return (
    <section className="panel workflow-stages">
      <h2 className="panel__title">Workflow Stages</h2>

      <details className="stage" data-testid="stage-firewall">
        <summary>
          <span>1. Prompt Firewall</span>
          <span className="pill pill--neutral">{response.firewall.action}</span>
        </summary>
        <div className="stage__content">
          <p>
            <strong>Severity:</strong> {response.firewall.severity}
          </p>
          <p>
            <strong>Sanitized prompt:</strong> {response.firewall.sanitized_prompt || "(empty)"}
          </p>
          <p>
            <strong>Reasons:</strong> {response.firewall.reasons.join(", ") || "None"}
          </p>
          <p>
            <strong>Matched rules:</strong> {response.firewall.matched_rules.join(", ") || "None"}
          </p>
        </div>
      </details>

      <details className="stage" data-testid="stage-bias">
        <summary>
          <span>2. Bias Detection</span>
          <span className="pill pill--neutral">{response.bias.level}</span>
        </summary>
        <div className="stage__content">
          <p>
            <strong>Score:</strong> {response.bias.score.toFixed(2)}
          </p>
          <p>
            <strong>Categories:</strong> {response.bias.categories.join(", ") || "None"}
          </p>
          <p>
            <strong>Matched terms:</strong> {response.bias.matched_terms.join(", ") || "None"}
          </p>
          <p>
            <strong>Mitigation hints:</strong> {response.bias.mitigation_hints.join(" | ") || "None"}
          </p>
        </div>
      </details>

      <details className="stage" data-testid="stage-input-moderation">
        <summary>
          <span>3. Input Moderation</span>
          <span className="pill pill--neutral">
            {response.input_moderation ? (response.input_moderation.flagged ? "Flagged" : "Clear") : "Not run"}
          </span>
        </summary>
        <div className="stage__content">
          {response.input_moderation ? (
            <>
              <p>
                <strong>Flagged:</strong> {String(response.input_moderation.flagged)}
              </p>
              <p>
                <strong>Severity:</strong> {response.input_moderation.severity.toFixed(2)}
              </p>
              <p>
                <strong>Categories:</strong> {response.input_moderation.categories.join(", ") || "None"}
              </p>
            </>
          ) : (
            <p>No input moderation result is available for this run.</p>
          )}
        </div>
      </details>

      <details className="stage" data-testid="stage-output-moderation">
        <summary>
          <span>4. Output Moderation</span>
          <span className="pill pill--neutral">
            {response.output_moderation ? (response.output_moderation.flagged ? "Flagged" : "Clear") : "Not run"}
          </span>
        </summary>
        <div className="stage__content">
          {response.output_moderation ? (
            <>
              <p>
                <strong>Flagged:</strong> {String(response.output_moderation.flagged)}
              </p>
              <p>
                <strong>Severity:</strong> {response.output_moderation.severity.toFixed(2)}
              </p>
              <p>
                <strong>Categories:</strong> {response.output_moderation.categories.join(", ") || "None"}
              </p>
            </>
          ) : (
            <p>No output moderation result is available for this run.</p>
          )}
        </div>
      </details>

      <details className="stage" data-testid="stage-generation">
        <summary>
          <span>5. Generation Outcome</span>
          <span className="pill pill--neutral">{response.generated_text ? "Available" : "Suppressed"}</span>
        </summary>
        <div className="stage__content">
          {response.generated_text ? (
            <p>{response.generated_text}</p>
          ) : (
            <p>Generated content is not available because compliance controls blocked final output.</p>
          )}
        </div>
      </details>
    </section>
  );
}
