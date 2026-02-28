import type { ReactNode } from "react";

interface DesktopSidePanelProps {
  children: ReactNode;
}

export function DesktopSidePanel({ children }: DesktopSidePanelProps) {
  return <div className="desktop-side-panel">{children}</div>;
}
