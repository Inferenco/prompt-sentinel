import { parseComplianceResponse, parseMistralHealthResponse } from "../domain/contracts";
import type {
  ComplianceRequest,
  ComplianceResponse,
  MistralHealthResponse,
} from "../domain/types";

type FetchFn = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

export class PromptSentinelApiError extends Error {
  status: number;
  responseBody: string;

  constructor(status: number, responseBody: string) {
    super(`HTTP ${status}: ${responseBody || "Request failed"}`);
    this.name = "PromptSentinelApiError";
    this.status = status;
    this.responseBody = responseBody;
  }
}

export class PromptSentinelClient {
  private readonly baseUrl: string;
  private readonly fetchImpl: FetchFn;

  constructor(baseUrl = "http://localhost:3000", fetchImpl: FetchFn = fetch) {
    this.baseUrl = baseUrl.replace(/\/$/, "");
    // Some browsers require `window.fetch` to be invoked with window context.
    this.fetchImpl = (input, init) => fetchImpl.call(globalThis, input, init);
  }

  async getHealth(): Promise<"OK"> {
    const response = await this.fetchImpl(this.url("/health"), {
      method: "GET",
    });

    const payload = await response.text();
    if (!response.ok) {
      throw new PromptSentinelApiError(response.status, payload);
    }

    const normalized = payload.trim();
    if (normalized !== "OK") {
      throw new Error(`Unexpected health payload: ${normalized}`);
    }

    return "OK";
  }

  async getMistralHealth(): Promise<MistralHealthResponse> {
    const json = await this.requestJson("/api/mistral/health", {
      method: "GET",
    });
    return parseMistralHealthResponse(json);
  }

  async checkCompliance(request: ComplianceRequest): Promise<ComplianceResponse> {
    const json = await this.requestJson("/api/compliance/check", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(request),
    });
    return parseComplianceResponse(json);
  }

  private async requestJson(path: string, init: RequestInit): Promise<unknown> {
    const response = await this.fetchImpl(this.url(path), init);
    const payload = await response.text();

    if (!response.ok) {
      throw new PromptSentinelApiError(response.status, payload);
    }

    try {
      return JSON.parse(payload);
    } catch (error) {
      throw new Error(`Invalid JSON response: ${String(error)}`);
    }
  }

  private url(path: string): string {
    return `${this.baseUrl}${path}`;
  }
}
