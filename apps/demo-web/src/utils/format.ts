import type { WorkflowStatus } from "../domain/types";

export function formatWorkflowStatus(status: WorkflowStatus): string {
  switch (status) {
    case "Completed":
      return "Completed";
    case "BlockedByFirewall":
      return "Blocked by Firewall";
    case "BlockedByInputModeration":
      return "Blocked by Input Moderation";
    case "BlockedByOutputModeration":
      return "Blocked by Output Moderation";
    default:
      return status;
  }
}

export function formatIsoTime(timestampIso: string): string {
  return new Date(timestampIso).toLocaleString();
}

export function shortHash(hash: string, keep = 10): string {
  if (hash.length <= keep * 2) {
    return hash;
  }
  return `${hash.slice(0, keep)}...${hash.slice(-keep)}`;
}

export function formatPercent(score: number): string {
  return `${(score * 100).toFixed(1)}%`;
}
