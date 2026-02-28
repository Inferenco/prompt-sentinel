import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { useDemoState } from "./useDemoState";

describe("useDemoState", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("supports loading and success states in mock mode", async () => {
    vi.stubGlobal("fetch", vi.fn(async () => new Response("OK")));

    const { result } = renderHook(() => useDemoState());

    act(() => {
      result.current.setMode("mock");
      result.current.setPrompt("Summarize this update");
    });

    let promise: Promise<void> | undefined;
    act(() => {
      promise = result.current.runCompliance();
    });

    await waitFor(() => expect(result.current.isRunning).toBe(true));

    await act(async () => {
      await promise;
    });

    expect(result.current.isRunning).toBe(false);
    expect(result.current.activeRecord?.response?.status).toBe("Completed");
  });

  it("captures live API errors in history", async () => {
    const fetchMock = vi
      .fn<typeof fetch>()
      .mockResolvedValueOnce(new Response("OK", { status: 200 }))
      .mockResolvedValueOnce(
        new Response(
          JSON.stringify({
            status: "healthy",
            message: "ok",
            models: ["mistral-large-latest", "mistral-moderation-latest", "mistral-embed"],
          }),
          { status: 200 },
        ),
      )
      .mockResolvedValueOnce(new Response("upstream unavailable", { status: 503 }));

    vi.stubGlobal("fetch", fetchMock);

    const { result } = renderHook(() => useDemoState());

    await waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(2));

    act(() => {
      result.current.setPrompt("Test prompt");
    });

    await act(async () => {
      await result.current.runCompliance();
    });

    expect(result.current.activeRecord?.error).toContain("HTTP 503");
  });
});
