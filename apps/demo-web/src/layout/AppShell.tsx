import type { ReactNode } from "react";

interface AppShellProps {
  header: ReactNode;
  main: ReactNode;
  side: ReactNode;
  mobileFooter?: ReactNode;
}

export function AppShell({ header, main, side, mobileFooter }: AppShellProps) {
  return (
    <div className="app-shell">
      {header}
      <div className="app-content">
        <main className="app-main">{main}</main>
        {side ? <aside className="app-side">{side}</aside> : null}
      </div>
      {mobileFooter ? <div className="mobile-footer">{mobileFooter}</div> : null}
    </div>
  );
}
