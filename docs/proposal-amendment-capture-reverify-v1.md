# Proposal Amendment Capture and Re-Verify v1

Issue: #2054 — Era M, Milestone 75

Status: **implemented capability contract** for amend-before-approve capture and
re-verification.

## Implemented boundary

Ouroforge now has an additive proposal-amendment data-plane contract and local
Studio capture surface:

- Rust validates `ouroforge.proposal-amendment.v1` artifacts.
- The artifact records a human edit as **intervention-as-evidence** with before,
  after, and provenance refs.
- A proposal amendment is `ready-for-review-apply` only when review/apply,
  scene/source-apply, evaluator, and design-integrity gates all pass.
- Blocked, rejected, stale, missing, or failed gates remain visible and prevent
  review/apply readiness.
- The CLI fallback is `ouroforge proposal-amendment validate <artifact.json>`.
- The local Elixir control/presentation surface captures an amendment request as
  inert read + gated-write data and routes it to the Rust validation family.

## Two-plane invariant

Rust remains the data plane for validation, determinism, evidence/provenance,
re-verification, and review/apply readiness. Elixir/Phoenix LiveView Studio is
control + presentation only: it captures amendment intent and routes it to Rust;
it does not own artifact semantics, write trusted artifacts, run evaluator gates,
or apply changes.

## Safety properties

- Agent-first default is preserved; amendments are opt-in and never required for
the autonomous loop to complete.
- Studio capture forbids raw writes, raw apply bypasses, trusted write authority,
and mandatory human intervention.
- Human edits are proposal amendments, not direct artifact mutations.
- Missing or stale evidence fails closed instead of broadening authority.
- Hosted/multi-user/collaborative Studio remains Layer-3 DEFER.
- #1 and #23 remain open.
