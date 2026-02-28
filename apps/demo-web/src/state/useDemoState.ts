import { useCallback, useEffect, useMemo, useState } from "react";

import type {
  ComplianceRequest,
  DemoMode,
  DemoRunRecord,
  DemoScenario,
  HealthState,
} from "../domain/types";
import { PromptSentinelClient } from "../sdk/promptSentinelClient";
import { getMockHealth, runMockCompliance } from "../sdk/mockFixtures";

const DEFAULT_API_BASE_URL =
  (import.meta.env.VITE_API_BASE_URL as string | undefined) ?? "http://localhost:3000";

const initialHealth: HealthState = {
  serviceStatus: "unknown",
  mistralStatus: "unknown",
  message: "No health check run yet.",
  models: [],
  loading: false,
  error: null,
  lastCheckedAt: null,
};

export interface DemoState {
  mode: DemoMode;
  apiBaseUrl: string;
  prompt: string;
  correlationId: string;
  selectedScenarioId: string | null;
  health: HealthState;
  history: DemoRunRecord[];
  activeRecord: DemoRunRecord | null;
  isRunning: boolean;
  runError: string | null;
  setMode: (mode: DemoMode) => void;
  setApiBaseUrl: (apiBaseUrl: string) => void;
  setPrompt: (prompt: string) => void;
  setCorrelationId: (correlationId: string) => void;
  applyScenario: (scenario: DemoScenario) => void;
  clearComposer: () => void;
  refreshHealth: () => Promise<void>;
  runCompliance: () => Promise<void>;
  selectHistoryRecord: (recordId: string) => void;
}

export function useDemoState(): DemoState {
  const [mode, setMode] = useState<DemoMode>("live");
  const [apiBaseUrl, setApiBaseUrl] = useState<string>(DEFAULT_API_BASE_URL);
  const [prompt, setPrompt] = useState<string>("");
  const [correlationId, setCorrelationId] = useState<string>("");
  const [selectedScenarioId, setSelectedScenarioId] = useState<string | null>(null);
  const [health, setHealth] = useState<HealthState>(initialHealth);
  const [history, setHistory] = useState<DemoRunRecord[]>([]);
  const [activeRecordId, setActiveRecordId] = useState<string | null>(null);
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [runError, setRunError] = useState<string | null>(null);

  const client = useMemo(() => new PromptSentinelClient(apiBaseUrl), [apiBaseUrl]);

  const appendHistory = useCallback((record: DemoRunRecord) => {
    setHistory((previous) => [record, ...previous].slice(0, 10));
    setActiveRecordId(record.id);
  }, []);

  const refreshHealth = useCallback(async () => {
    setHealth((previous) => ({ ...previous, loading: true, error: null }));

    try {
      if (mode === "mock") {
        const mock = getMockHealth();
        setHealth({
          serviceStatus: "healthy",
          mistralStatus: mock.status === "healthy" ? "healthy" : "unhealthy",
          message: mock.message,
          models: mock.models,
          loading: false,
          error: null,
          lastCheckedAt: new Date().toISOString(),
        });
        return;
      }

      await client.getHealth();
      const mistral = await client.getMistralHealth();

      setHealth({
        serviceStatus: "healthy",
        mistralStatus: mistral.status === "healthy" ? "healthy" : "unhealthy",
        message: mistral.message,
        models: mistral.models,
        loading: false,
        error: null,
        lastCheckedAt: new Date().toISOString(),
      });
    } catch (error) {
      setHealth({
        serviceStatus: "error",
        mistralStatus: "error",
        message: "Health check failed.",
        models: [],
        loading: false,
        error: error instanceof Error ? error.message : String(error),
        lastCheckedAt: new Date().toISOString(),
      });
    }
  }, [client, mode]);

  const runCompliance = useCallback(async () => {
    const normalizedPrompt = prompt.trim();
    if (!normalizedPrompt) {
      setRunError("Prompt is required.");
      return;
    }

    setRunError(null);
    setIsRunning(true);

    const request: ComplianceRequest = {
      prompt: normalizedPrompt,
      ...(correlationId.trim() ? { correlation_id: correlationId.trim() } : {}),
    };

    try {
      const response =
        mode === "mock"
          ? await runMockCompliance(request)
          : await client.checkCompliance(request);

      appendHistory({
        id: `${response.correlation_id}:${Date.now()}`,
        timestampIso: new Date().toISOString(),
        mode,
        request,
        response,
        error: null,
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      setRunError(message);

      appendHistory({
        id: `error:${Date.now()}`,
        timestampIso: new Date().toISOString(),
        mode,
        request,
        response: null,
        error: message,
      });
    } finally {
      setIsRunning(false);
    }
  }, [appendHistory, client, correlationId, mode, prompt]);

  const applyScenario = useCallback((scenario: DemoScenario) => {
    setSelectedScenarioId(scenario.id);
    setPrompt(scenario.prompt);
    setCorrelationId(scenario.correlationId ?? "");
    setRunError(null);
  }, []);

  const clearComposer = useCallback(() => {
    setPrompt("");
    setCorrelationId("");
    setSelectedScenarioId(null);
    setRunError(null);
  }, []);

  const activeRecord = useMemo(() => {
    if (!activeRecordId && history.length > 0) {
      return history[0];
    }
    return history.find((record) => record.id === activeRecordId) ?? null;
  }, [activeRecordId, history]);

  const selectHistoryRecord = useCallback((recordId: string) => {
    setActiveRecordId(recordId);
  }, []);

  useEffect(() => {
    void refreshHealth();
  }, [refreshHealth]);

  return {
    mode,
    apiBaseUrl,
    prompt,
    correlationId,
    selectedScenarioId,
    health,
    history,
    activeRecord,
    isRunning,
    runError,
    setMode,
    setApiBaseUrl,
    setPrompt,
    setCorrelationId,
    applyScenario,
    clearComposer,
    refreshHealth,
    runCompliance,
    selectHistoryRecord,
  };
}
