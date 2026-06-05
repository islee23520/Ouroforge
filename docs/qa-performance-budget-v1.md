# QA Performance Budget v1

Issue: #689 — Performance Budget Swarm Evaluation v1.

QA Performance Budget v1 is a non-mutating Rust/local artifact for evaluating
performance metrics across bounded QA/playtest runs and classifying regressions
as evidence inputs. It records a budget id, run matrix refs, stale run refs,
baseline refs, profiling refs, frame/update/render/probe/evidence timings,
entity/draw/collision counts, thresholds, a verdict, blocked reasons, and trust
warnings.

## States

The budget classifies into `pass`, `fail`, `inconclusive`, `missing`,
`unsupported`, and `stale`. Classification precedence is stale, then
unsupported, then missing, then fail/pass, with inconclusive reserved for the
case where only advisory (browser-sourced) thresholds are evaluable.

## Boundary

Performance metrics are evidence inputs, not trusted truth. Browser-sourced
metrics stay advisory and are not trusted truth; only present, trusted Rust/local
profiler or frame budget evidence can fail a budget. No auto-fix, no auto-apply,
no auto-merge, no self-approval, no hidden workers, no remote swarm, no command
bridge, no local server bridge, and no production performance guarantee are
authorized. Outputs feed failure classification and review but are review-gated
and never perform trusted mutation. Browser/dashboard/Studio surfaces remain
read-only or draft-only unless a separate trusted Rust/local API is explicitly
scoped.

## Validation

Rust/local validation rejects missing metrics, malformed metrics (absent,
non-finite, negative, or non-integer counts), stale run refs outside the
declared run matrix, unsupported thresholds without blockers, missing required
baselines, browser-sourced metrics without a browser trust warning, remote refs,
traversal refs, status that disagrees with the computed classification, and
overbroad v1 budgets.

## Artifact separation

Run matrix, runtime frame budget, profiling evidence, failure classification,
evidence bundle, dashboard, and Studio read models remain separate. The budget
may point to existing run matrix and profiling refs, but it does not mutate them
or write trusted state.

## Read model

The read model is display/export data only: ref counts, metric and threshold
counts, trusted-threshold and violation counts, low-trust metric counts, blocked
counts, metric-kind counts, validation notes, compatibility notes, and the
conservative boundary. It has no auto-fix, apply, or command authority.

## Governance

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open. Public wording stays conservative:
this is bounded evidence-gated QA exploration, not a production game, a
shipped-game claim, a production performance guarantee, a current Godot
replacement, or a production-ready claim.
