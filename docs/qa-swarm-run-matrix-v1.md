# QA Swarm Run Matrix v1

Issue: #693 — QA Swarm Run Matrix v1.

QA Swarm Run Matrix v1 is a non-mutating Rust/local artifact that represents
QA/playtest results across scenarios, fuzz seeds, workers, attempts, reruns,
verdicts, failure classes, flake status, and evidence refs. Each row records a
scenario id, candidate id, optional fuzz seed, worker id, attempt, rerun group,
verdict, optional failure class, flake status, evidence refs, the bounded budget
used, an optional start timestamp, and blocked reasons.

## Verdicts and status

Row verdicts cover `passed`, `failed`, `flaky`, `inconclusive`, `skipped`,
`unsupported`, `timed_out`, `crashed`, and `missing_evidence`. The matrix rolls
up to a `complete`, `partial`, `blocked`, or `stale` status: `complete` when all
rows are resolved (passed/failed), `partial` when any row is unresolved or
carries blockers, `blocked` when top-level blockers exist, and `stale` when
evidence refs are stale.

## Boundary

The run matrix is evidence inputs only, not trusted truth, and remains
review-gated until reviewed. No auto-fix, no auto-apply, no auto-merge, no
self-approval, no hidden workers, and no remote swarm are authorized.
Browser/dashboard/Studio surfaces remain read-only or draft-only unless a
separate trusted Rust/local API is explicitly scoped.

## Validation

Rust/local validation rejects duplicate rows (by id and by
scenario/candidate/seed/worker/attempt), stale evidence refs without blockers,
invalid worker/candidate ids, missing run refs, malformed verdicts, missing or
unbounded budgets, inconsistent rerun groups (mismatched scenario/candidate or
duplicate attempts), missing evidence for evidence-bearing verdicts, a dashboard
surface that is not read-only, remote/traversal refs, and a status that
disagrees with the computed classification.

## Dashboard/Studio compatibility

The artifact carries a `dashboardCompat` block (read-only flag, surface, and
columns) so dashboard/Studio surfaces can render the matrix read-only or
draft-only without owning trusted persistence.

## Read model

The read model is display/export data only: row counts, verdict counts, rerun
group counts, blocked counts, validation notes, compatibility notes, and the
conservative boundary. It has no auto-fix, apply, or command authority.

## Governance

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open. Public wording stays conservative:
this is bounded evidence-gated QA exploration, not a production game, a
shipped-game claim, a current Godot replacement, or a production-ready claim.
