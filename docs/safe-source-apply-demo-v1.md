# Safe Source Apply Demo v1

Issue: #713 — #1 Milestone 15: Safe Source Mutation Apply.

## What this demo is

A single deterministic plan artifact (`safe-source-apply-demo-v1`) that composes
the existing safe-source-apply chain over **one explicitly allowed low-risk
source-like fixture**. It proves the safety chain composes end to end; it does
**not** enable unrestricted source mutation. The artifact references each chain
stage and its evidence path by reference only — it executes no commands, applies
no patches, and writes no trusted files.

This milestone enables review-gated trusted source apply for explicitly allowed
source-like file classes, not unrestricted source mutation.

## Chain stages

Pre-apply gates (must all pass before the apply transaction):

1. `preview` — deterministic patch preview over the allowed fixture.
2. `file-class` — confirms an allowed source-like class.
3. `diff-integrity` — preview hunks match the recorded base hash.
4. `worktree-context` — isolated local worktree boundary.
5. `stale-guard` — recorded target hash still matches the live fixture.
6. `high-risk-blocker` — no forbidden/high-risk class is touched.
7. `sandbox-dry-run` — allowlisted local checks, with no secure-sandbox claim.
8. `sandbox-promotion` — sandbox evidence present before any trusted apply.
9. `independent-review` — reviewer differs from author; decision accepted.

Post-apply evidence (recorded only after a trusted apply):

10. `apply-transaction`
11. `rollback-snapshot`
12. `verification`
13. `rerun-comparison`
14. `audit-ledger`
15. `evidence-bundle`

A `ready` demo requires the full chain with no blockers, stale targets, or
failed stages. Forbidden file classes, high-risk targets, missing or self
review, stale targets, missing rollback, and failed verification all fail closed.

## Commands

```bash
cargo test -p ouroforge-core safe_source_apply_demo
```

The contract test validates the bundled fixtures and asserts that the invalid
fixtures fail closed.

## Expected evidence

The demo references an evidence bundle index and a rollback snapshot under
`examples/safe-source-apply-demo-v1/evidence/`. Real runs would generate these
under an untracked staging directory; only the fixture-scoped JSON in this folder
is committed.

## Known gaps

- The demo references chain artifacts by path; it does not re-run the underlying
  preview/sandbox/review/apply engines.
- Apply scope is intentionally limited to allowed source-like file classes; the
  high-risk classes stay blocked for v1.
- Browser/dashboard/Studio surfaces remain read-only or draft-only.

## Rollback notes

Every demo that records an apply transaction must also record a rollback
snapshot before the trusted write, so the change can be reverted. A demo that
reaches `ready` without rollback evidence fails validation.

## Cleanup policy

Generated previews, sandbox outputs, rollback snapshots, verification logs, and
evidence bundles stay untracked and git-ignored. Only fixture-scoped JSON under
`examples/safe-source-apply-demo-v1/` is committed.

## Governance

- #1 remains open.
- #23 remains open.
- No auto-apply, no self-approval, no reviewer bypass, and no browser-side
  trusted writes.
