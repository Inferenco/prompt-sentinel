import "@testing-library/jest-dom/vitest";

// React 19 expects explicit act environment configuration in test runners.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
(globalThis as any).IS_REACT_ACT_ENVIRONMENT = true;
