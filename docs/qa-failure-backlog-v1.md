# QA Failure Classification and Mutation Backlog v1

Issue: #692 — Failure Classification and Mutation Backlog v1.

QA Failure Classification and Mutation Backlog v1 is a non-mutating Rust/local
artifact that turns QA/playtest failures into evidence-linked backlog items, not
automatic fixes. Each backlog item records a failure class, severity, evidence
refs, a likely owner lane, a suggested next investigation, a reproduction
context, an optional related scenario/fuzz seed, a triage-only review status,
and blocked reasons.

## Failure classes

`gameplay-logic`, `level-design`, `asset`, `physics-collision`, `input`,
`performance`, `visual`, `runtime-crash`, `console-error`, `probe-failure`,
`flaky`, `unsupported`, and `unknown`. The artifact rolls up to a `classified`,
`blocked`, or `stale` status.

## Boundary

Failure classifications and mutation backlog items are evidence and backlog
inputs only, not automatic fixes; they are also evidence inputs and remain
review-gated until a reviewer triages them. Review statuses are triage-only
(`pending-review`, `triaged`, `deferred`, `rejected`) — there is no applied or
fixed status. No auto-fix, no auto-apply, no auto-merge, no self-approval, no
hidden tasks, no hidden workers, and no remote swarm are authorized. Backlog
items may reference multi-agent task boards/work packages, but they do not create
hidden tasks automatically or write trusted state. Browser/dashboard/Studio
surfaces remain read-only or draft-only unless a separate trusted Rust/local API
is explicitly scoped.

## Validation

Rust/local validation rejects missing evidence, invalid owner lane, unsupported
failure class, unsupported severity, missing reproduction context, stale run refs
outside the declared run matrix, duplicate backlog ids, auto-fix/auto-apply
attempts (including unsupported review statuses), remote/traversal refs, and a
status that disagrees with the computed classification.

## Artifact separation

Run matrix, multi-agent task boards, evidence bundle, dashboard, and Studio read
models remain separate. The backlog may point to existing run matrix and
evidence refs, but it does not mutate them or write trusted state.

## Read model

The read model is display/export data only: item counts, blocked counts,
failure-class/owner-lane/review-status counts, validation notes, compatibility
notes, and the conservative boundary. It has no auto-fix, apply, or command
authority.

## Governance

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open. Public wording stays conservative:
this is bounded evidence-gated QA exploration, not a production game, a
shipped-game claim, a current Godot replacement, or a production-ready claim.
