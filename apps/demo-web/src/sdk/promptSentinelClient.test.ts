import { describe, expect, it, vi } from "vitest";

import { PromptSentinelApiError, PromptSentinelClient } from "./promptSentinelClient";

describe("PromptSentinelClient", () => {
  it("calls all endpoints and parses health + compliance payloads", async () => {
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
      .mockResolvedValueOnce(
        new Response(
          JSON.stringify({
            correlation_id: "corr-1",
            status: "Completed",
            firewall: {
              action: "Allow",
              severity: "Low",
              sanitized_prompt: "safe",
              reasons: ["passed"],
              matched_rules: [],
            },
            bias: {
              score: 0.08,
              level: "Low",
              categories: [],
              matched_terms: [],
              mitigation_hints: [],
            },
            input_moderation: {
              flagged: false,
              categories: [],
              severity: 0,
            },
            output_moderation: {
              flagged: false,
              categories: [],
              severity: 0,
            },
            generated_text: "ok",
            audit_proof: {
              algorithm: "sha256",
              record_hash: "a".repeat(64),
              chain_hash: "b".repeat(64),
            },
          }),
          { status: 200 },
        ),
      );

    const client = new PromptSentinelClient("http://localhost:3000", fetchMock);

    await expect(client.getHealth()).resolves.toBe("OK");
    await expect(client.getMistralHealth()).resolves.toMatchObject({ status: "healthy" });
    await expect(client.checkCompliance({ prompt: "test" })).resolves.toMatchObject({
      status: "Completed",
    });

    expect(fetchMock).toHaveBeenNthCalledWith(1, "http://localhost:3000/health", {
      method: "GET",
    });
    expect(fetchMock).toHaveBeenNthCalledWith(2, "http://localhost:3000/api/mistral/health", {
      method: "GET",
    });
    expect(fetchMock).toHaveBeenNthCalledWith(
      3,
      "http://localhost:3000/api/compliance/check",
      expect.objectContaining({ method: "POST" }),
    );
  });

  it("maps HTTP errors into PromptSentinelApiError", async () => {
    const fetchMock = vi.fn<typeof fetch>().mockResolvedValue(new Response("boom", { status: 503 }));
    const client = new PromptSentinelClient("http://localhost:3000", fetchMock);

    await expect(client.getMistralHealth()).rejects.toBeInstanceOf(PromptSentinelApiError);
  });
});
