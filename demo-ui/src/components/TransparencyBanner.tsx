import './TransparencyBanner.css';

export function TransparencyBanner() {
  return (
    <div className="transparency-banner">
      <div className="banner-icon">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <circle cx="12" cy="12" r="10" />
          <path d="M12 16v-4M12 8h.01" />
        </svg>
      </div>
      <div className="banner-content">
        <strong>AI Transparency Notice (EU AI Act Article 50)</strong>
        <span className="banner-text">
          You are interacting with an AI-powered system. This demo showcases EU AI Act compliance controls.
        </span>
      </div>
    </div>
  );
}
