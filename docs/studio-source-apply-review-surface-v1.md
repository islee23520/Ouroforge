# Studio Source Apply Review Surface v1

Issue: #712
Roadmap anchor: #1 (Milestone 15: Safe Source Mutation Apply).
Status: read-only Studio inspection surface; no apply behavior.

The Studio source apply review surface lets a reviewer inspect review-gated
source apply readiness and results **read-only**. It renders an exported,
Rust-produced read model only; it never applies patches, accepts reviews,
bypasses review gates, merges branches, rolls back, executes commands, or writes
trusted files.

## What it shows

A single review-readiness aggregation over the source-apply gate families:

- Apply transaction
- Review decision enforcement
- Stale target guard
- Sandbox-to-trusted promotion
- Rollback snapshot / recovery
- Verification
- Rerun / comparison
- Audit ledger
- Evidence bundle

Each gate shows present/absent status, any exported blocked reasons, and inert
evidence references. The surface also lists the forbidden browser actions
(`apply_patch`, `merge_branch`, `self_approve`, `reviewer_bypass`,
`execute_command`, `write_trusted_file`, `browser_command_bridge`) and an
emergency-hold notice when one is exported. Malformed read models surface their
reasons instead of failing silently.

## Boundary

- **No apply / no command / no merge / no browser write.** The surface is pure
  display of escaped exported JSON. There are no apply buttons, command runners,
  or trusted-write controls. Copyable commands, where present elsewhere, remain
  inert display text.
- Rust/local validation owns trusted persistence, source apply validation,
  trusted writes, rollback metadata, generated evidence writing, and CLI
  contracts. The browser/Studio surface remains read-only.
- Source apply remains review-gated for explicitly allowed source-like file
  classes; this surface makes no claim of unrestricted source mutation, secure
  sandbox, autonomous source repair, production-ready mutation, or Godot
  replacement.

Generated previews, sandbox outputs, rollback snapshots, verification logs,
runs, dashboard exports, temp worktrees, and local tool state remain untracked
unless explicitly fixture-scoped. #1 and #23 remain open.
