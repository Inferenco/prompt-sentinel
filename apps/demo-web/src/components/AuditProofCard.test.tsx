import { act, fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { AuditProofCard } from "./AuditProofCard";

describe("AuditProofCard", () => {
  it("renders proof fields and copies hash values", async () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText },
    });

    render(
      <AuditProofCard
        proof={{
          algorithm: "sha256",
          record_hash: "a".repeat(64),
          chain_hash: "b".repeat(64),
        }}
      />,
    );

    await act(async () => {
      fireEvent.click(screen.getAllByRole("button", { name: "Copy" })[0]);
    });
    expect(writeText).toHaveBeenCalledWith("a".repeat(64));
  });
});
