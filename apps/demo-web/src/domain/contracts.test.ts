import { describe, expect, it } from "vitest";

import { parseComplianceResponse } from "./contracts";

describe("parseComplianceResponse", () => {
  it("accepts blocked responses with null moderation fields", () => {
    const parsed = parseComplianceResponse({
      correlation_id: "corr-123",
      status: "BlockedByFirewall",
      firewall: {
        action: "Block",
        severity: "Critical",
        sanitized_prompt: "unsafe",
        reasons: ["rule"],
        matched_rules: ["PFW-001"],
      },
      bias: {
        score: 0.1,
        level: "Low",
        categories: [],
        matched_terms: [],
        mitigation_hints: [],
      },
      input_moderation: null,
      output_moderation: null,
      generated_text: null,
      audit_proof: {
        algorithm: "sha256",
        record_hash: "a".repeat(64),
        chain_hash: "b".repeat(64),
      },
    });

    expect(parsed.status).toBe("BlockedByFirewall");
    expect(parsed.input_moderation).toBeNull();
    expect(parsed.output_moderation).toBeNull();
  });

  it("rejects invalid status values", () => {
    expect(() =>
      parseComplianceResponse({
        correlation_id: "corr-123",
        status: "Unknown",
        firewall: {
          action: "Allow",
          severity: "Low",
          sanitized_prompt: "safe",
          reasons: [],
          matched_rules: [],
        },
        bias: {
          score: 0,
          level: "Low",
          categories: [],
          matched_terms: [],
          mitigation_hints: [],
        },
        input_moderation: null,
        output_moderation: null,
        generated_text: null,
        audit_proof: {
          algorithm: "sha256",
          record_hash: "a".repeat(64),
          chain_hash: "b".repeat(64),
        },
      }),
    ).toThrow(/status is invalid/);
  });
});
