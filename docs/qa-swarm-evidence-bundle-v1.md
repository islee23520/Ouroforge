# QA Swarm Evidence Bundle v1

Issue: #694 — QA Swarm Evidence Bundle v1.

QA Swarm Evidence Bundle v1 is a non-mutating Rust/local artifact that bundles
QA scenario candidates, fuzz plans, worker assignments, invariant checks, route
attempts, visual comparisons, performance budgets, error classifications, flake
policy/results, failure classifications, mutation backlog, and run matrix into
one auditable artifact with a final summary and blocked reasons.

## Components and status

The bundle references each clearly-separated QA/playtest artifact by ref and
records whether it is present, stale, and resolved. It rolls up to `complete`,
`partial`, `blocked`, or `stale`: `stale` when any component is stale, `partial`
when any component is missing/unresolved or flaky states are unresolved,
`blocked` when top-level blockers exist, and `complete` otherwise. Cleanup
confirmation, budget confirmation, and matrix-row consistency are explicit.

## Boundary

The QA evidence bundle is evidence inputs only, not trusted truth, and remains
review-gated until reviewed. Generated outputs follow an explicit cleanup policy
and bounded budgets with disjoint output roots. No auto-fix, no auto-apply, no
auto-merge, no self-approval, no hidden workers, and no remote swarm are
authorized. Browser/dashboard/Studio surfaces remain read-only or draft-only
unless a separate trusted Rust/local API is explicitly scoped.

## Validation

Rust/local validation rejects missing component refs, missing required
components, stale artifacts without blockers, unresolved flaky states without
blockers, missing budgets, missing cleanup confirmation, unresolved (overlapping)
output roots, inconsistent matrix rows, malformed artifacts (unsupported
component types), a dashboard export that is not read-only, remote/traversal
refs, and a status that disagrees with the computed classification.

## Dashboard export / read-model compatibility

The artifact carries a `dashboardExport` block (read-only flag, surface, and
fields) so dashboard/Studio surfaces can export the bundle read-only or
draft-only without owning trusted persistence.

## Read model

The read model is display/export data only: component counts, present/resolved
counts, per-type component status, blocked counts, validation notes,
compatibility notes, and the conservative boundary. It has no auto-fix, apply, or
command authority.

## Generated-state policy

Generated runs, fuzz inputs, screenshots, videos, traces, dashboard exports, and
local tool state remain untracked unless explicitly fixture-scoped; the bundle's
output roots and cleanup confirmation make this explicit.

## Governance

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open. Public wording stays conservative:
this is bounded evidence-gated QA exploration, not a production game, a
shipped-game claim, a current Godot replacement, or a production-ready claim.
