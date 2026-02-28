import type { DemoScenario } from "../domain/types";

interface ScenarioChipsProps {
  scenarios: DemoScenario[];
  selectedId: string | null;
  disabled?: boolean;
  onSelect: (scenario: DemoScenario) => void;
}

export function ScenarioChips({
  scenarios,
  selectedId,
  disabled,
  onSelect,
}: ScenarioChipsProps) {
  return (
    <section className="panel scenario-chips" aria-label="Preset prompt scenarios">
      <h2 className="panel__title">Preset Demo Scenarios</h2>
      <p className="panel__hint">
        Use deterministic prompts to demonstrate each compliance path quickly.
      </p>
      <div className="chips" role="list">
        {scenarios.map((scenario) => (
          <button
            key={scenario.id}
            type="button"
            role="listitem"
            className={`chip ${selectedId === scenario.id ? "chip--selected" : ""}`}
            onClick={() => onSelect(scenario)}
            disabled={disabled}
            data-testid={`scenario-${scenario.id}`}
          >
            <span className="chip__label">{scenario.label}</span>
            <span className="chip__description">{scenario.description}</span>
          </button>
        ))}
      </div>
    </section>
  );
}
