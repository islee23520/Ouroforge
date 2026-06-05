# Scenario Coverage v14: Source Apply Regression Suite

Issue: #714 — Scenario Coverage v14: Source Apply Regression Suite.

Scenario Coverage v14 locks the Safe Source Mutation Apply v1 milestone as a
local-only, evidence-backed regression suite. It exercises each source-apply
safety gate independently so a critical failure cannot hide inside one demo. The
suite is read-only: it validates and evaluates the existing gate contracts and
applies nothing, runs nothing, and writes no trusted files.

## Regression matrix

| Scenario id | Coverage | Evidence / focused check | Expected result |
| --- | --- | --- | --- |
| `SA14.success-valid-apply-transaction` | Allowed source-like target with accepted independent review and a ready sandbox promotion. | `SourceApplyHighRiskBlocker::is_allowed`, `SourceApplyReviewEnforcement::is_ready`, `SourceApplySandboxPromotion::evaluate`. | All gates report ready/allowed; no apply or merge command is emitted. |
| `SA14.block-missing-review` | A `Missing` decision blocks apply. | `SourceApplyReviewEnforcement::evaluate` with `decision_state = Missing`. | Not ready; apply stays blocked until an accepted decision exists. |
| `SA14.block-self-review` | Reviewer equal to proposer is not independent. | `SourceApplyReviewEnforcement::evaluate` with `reviewer_id == proposer_id`. | Not ready; self-approval is rejected. |
| `SA14.block-stale-target` | A sandbox base revision drifting from the transaction base revision is stale. | `SourceApplySandboxPromotion::evaluate` with mismatched base revisions. | Not ready; the stale base is surfaced before any trusted apply. |
| `SA14.block-sandbox-mismatch` | A sandbox after-hash disagreeing with the expected after-hash is a mismatch. | `SourceApplySandboxPromotion::evaluate` with `sandbox_after_hash != expected_after_hash`. | Not ready; the mismatched diff is surfaced. |
| `SA14.block-forbidden-file-class` | A dependency lockfile is a forbidden high-risk class. | `SourceApplyHighRiskBlocker::is_allowed` over a `Cargo.lock` candidate. | Blocked; the forbidden class is named and apply is refused. |
| `SA14.block-missing-rollback` | A target with no before-content reference and no reverse patch has no recovery path. | `SourceApplyRollbackSnapshot::evaluate` with no recovery references. | Not complete; a recovery gap is recorded before apply is allowed. |
| `SA14.record-verification-failure` | A failed allowlisted verification command is recorded, not hidden. | `SourceApplyVerificationRun::evaluate` with a `Failed` command. | Not passed; the failure is visible as evidence. |
| `SA14.record-rerun-regression` | A regressed rerun dimension blocks a promotion claim. | `SourceApplyRerunComparison::promotion_claim_allowed` with a `Regressed` dimension. | Promotion claim not allowed; the regression is recorded. |
| `SA14.block-emergency-hold` | A global emergency hold blocks all apply locally. | `SourceApplyHold::evaluate_against` while a global, disabled hold is active. | Apply blocked; the local hold cannot be bypassed and has no remote kill switch. |
| `SA14.success-audit-append-only` | The audit ledger is append-only and rejects history rewrites. | `SourceApplyAuditLedger::validate_is_append_of` for a valid append and a rewritten/truncated history. | Valid appends pass; rewritten or truncated histories fail closed. |
| `SA14.success-evidence-bundle` | A complete evidence bundle aggregates every gate reference read-only. | `SourceApplyEvidenceBundle::is_complete` for a complete bundle. | Complete and consistent; it applies nothing. |
| `SA14.block-malformed-evidence` | A bundle with an unresolved rollback and a failed component is incomplete. | `SourceApplyEvidenceBundle::evaluate` with `rollback_resolved = false` and a failed component. | Not complete; the unresolved/missing/stale evidence is surfaced. |
| `SA14.success-read-only-inspection` | Gate evaluations are read-only for dashboard/Studio inspection. | Serialized review/rollback evaluation has no `applyCommand`/`mergeCommand` keys and lists forbidden actions. | Browser surfaces inspect JSON only; no accept/apply/write/merge/publish/deploy controls. |
| `SA14.block-malformed-bundle-ref` | A bundle missing a required reference fails structural validation. | `SourceApplyEvidenceBundle::validate` with an empty required ref. | Validation fails closed before the bundle is treated as evidence. |

## Fixture policy

The fixture under `examples/source-apply-regression-v14/` is source-like scenario
coverage metadata. It is intentionally small, deterministic, fixture-scoped, and
tracked. Generated previews, sandbox outputs, rollback snapshots, verification
logs, runs, and evidence bundles remain generated and ignored unless a future
issue explicitly scopes a small fixture-scoped source-like fixture.

## Non-goals and guardrails

Scenario Coverage v14 does not authorize unrestricted source mutation. It is
read-only and adds no auto-apply, no self-approval, no reviewer bypass, no hidden
trusted writes, no command bridge, no network/install command, no credential use,
and no publish/deploy/sign/upload. Browser/dashboard/Studio surfaces remain
read-only. No production-ready claim, no secure sandbox guarantee, and no current
Godot replacement claim is made.

## Known gaps

- The suite exercises the gate contracts in isolation; it does not re-run a full
  end-to-end apply, which is covered by the Safe Source Apply Demo (#713).
- Apply scope stays limited to allowed source-like classes; high-risk classes
  stay blocked for v1.

## Governance

- #1 remains open.
- #23 remains open.
