interface MobileActionBarProps {
  running: boolean;
  disabled: boolean;
  onRun: () => void;
  onClear: () => void;
}

export function MobileActionBar({
  running,
  disabled,
  onRun,
  onClear,
}: MobileActionBarProps) {
  return (
    <div className="mobile-action-bar" role="toolbar" aria-label="Mobile compliance actions">
      <button
        type="button"
        className="button button--secondary"
        onClick={onClear}
        disabled={disabled}
        data-testid="mobile-clear-btn"
      >
        Clear
      </button>
      <button
        type="button"
        className="button button--primary"
        onClick={onRun}
        disabled={disabled}
        data-testid="mobile-run-check-btn"
      >
        {running ? "Running..." : "Run check"}
      </button>
    </div>
  );
}
