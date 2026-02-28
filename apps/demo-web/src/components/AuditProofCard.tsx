import { useState } from "react";

import type { AuditProof } from "../domain/types";
import { shortHash } from "../utils/format";

interface AuditProofCardProps {
  proof: AuditProof | null;
}

export function AuditProofCard({ proof }: AuditProofCardProps) {
  const [copiedField, setCopiedField] = useState<string | null>(null);

  async function copyValue(field: string, value: string): Promise<void> {
    if (typeof navigator !== "undefined" && navigator.clipboard?.writeText) {
      await navigator.clipboard.writeText(value);
      setCopiedField(field);
      window.setTimeout(() => setCopiedField((current) => (current === field ? null : current)), 1000);
    }
  }

  if (!proof) {
    return (
      <section className="panel audit-proof">
        <h2 className="panel__title">Audit Proof</h2>
        <p className="panel__hint">Run a compliance request to inspect immutable proof-chain fields.</p>
      </section>
    );
  }

  return (
    <section className="panel audit-proof" data-testid="audit-proof">
      <h2 className="panel__title">Audit Proof</h2>
      <p className="panel__hint">Hash-chain metadata returned by runtime response contract.</p>

      <div className="proof-grid">
        <article className="proof-row">
          <span className="proof-row__label">Algorithm</span>
          <code>{proof.algorithm}</code>
        </article>

        <article className="proof-row">
          <span className="proof-row__label">Record hash</span>
          <code title={proof.record_hash}>{shortHash(proof.record_hash, 14)}</code>
          <button type="button" className="button button--ghost" onClick={() => void copyValue("record", proof.record_hash)}>
            Copy
          </button>
          {copiedField === "record" ? <span className="copy-note">Copied</span> : null}
        </article>

        <article className="proof-row">
          <span className="proof-row__label">Chain hash</span>
          <code title={proof.chain_hash}>{shortHash(proof.chain_hash, 14)}</code>
          <button type="button" className="button button--ghost" onClick={() => void copyValue("chain", proof.chain_hash)}>
            Copy
          </button>
          {copiedField === "chain" ? <span className="copy-note">Copied</span> : null}
        </article>
      </div>
    </section>
  );
}
