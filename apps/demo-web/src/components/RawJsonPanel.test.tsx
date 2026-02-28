import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { RawJsonPanel } from "./RawJsonPanel";

describe("RawJsonPanel", () => {
  it("renders request and response payloads", () => {
    render(
      <RawJsonPanel
        request={{ prompt: "test" }}
        response={{
          correlation_id: "corr-1",
          status: "Completed",
          firewall: {
            action: "Allow",
            severity: "Low",
            sanitized_prompt: "test",
            reasons: ["ok"],
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
          generated_text: "done",
          audit_proof: {
            algorithm: "sha256",
            record_hash: "a".repeat(64),
            chain_hash: "b".repeat(64),
          },
        }}
        error={null}
      />,
    );

    expect(screen.getByTestId("json-request")).toHaveTextContent("prompt");
    expect(screen.getByTestId("json-response")).toHaveTextContent("Completed");
  });
});
