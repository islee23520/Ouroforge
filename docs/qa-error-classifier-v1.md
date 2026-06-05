# QA Error Classifier v1

Issue: #690 — Console, Crash, and Runtime Error Classifier v1.

QA Error Classifier v1 is a non-mutating Rust/local artifact for classifying
console errors, crashes, runtime exceptions, probe failures, asset load
failures, and timeouts consistently across bounded QA/playtest runs. Each entry
records an error id, kind, failure class, severity, message payload, optional
console level, evidence refs, the affected worker/run/scenario, and blocked
reasons.

## States

Entry failure classes are `warning`, `error`, `crash`, `timeout`,
`probe-failure`, `asset-failure`, `unknown`, and `inconclusive`. The artifact
rolls up to a `classified`, `blocked`, or `stale` status: `stale` when run refs
are stale, `blocked` when blockers are present, and `classified` otherwise.

## Boundary

Error classifications are evidence inputs, not trusted truth, and remain
evidence and backlog inputs only until reviewed. No auto-fix, no auto-apply, no
auto-merge, no self-approval, no hidden workers, no remote swarm, no command
bridge, and no production safety guarantee are authorized. Classifier output may
feed run matrix, failure classification, evidence bundle, and journal summaries,
but it is review-gated and never performs trusted mutation. Browser/dashboard/
Studio surfaces remain read-only or draft-only unless a separate trusted
Rust/local API is explicitly scoped.

## Validation

Rust/local validation rejects missing console/probe evidence, malformed error
payloads, unknown severity, unsupported error kinds, missing classification,
failure classes inconsistent with the error kind, stale run refs outside the
declared run matrix, remote refs, traversal refs, duplicate error ids, status
that disagrees with the computed classification, and overbroad v1 classifiers.

## Artifact separation

Run matrix, failure classification, mutation backlog, evidence bundle, journal,
dashboard, and Studio read models remain separate. The classifier may point to
existing run matrix and evidence refs, but it does not mutate them or write
trusted state.

## Read model

The read model is display/export data only: entry counts, blocked counts,
failure-class and kind counts, validation notes, compatibility notes, and the
conservative boundary. It has no auto-fix, apply, or command authority.

## Governance

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open. Public wording stays conservative:
this is bounded evidence-gated QA exploration, not a production game, a
shipped-game claim, a production safety guarantee, a current Godot replacement,
or a production-ready claim.
