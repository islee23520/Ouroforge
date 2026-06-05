# QA Flake and Rerun Policy v1

Issue: #691 — Flaky Evidence and Rerun Policy v1.

QA Flake and Rerun Policy v1 is a non-mutating Rust/local artifact for detecting
and handling flaky QA evidence with bounded reruns and explicit inconclusive
states. It records a policy id, run matrix refs, max reruns, reruns used, a
consistency threshold, a cleanup policy, output roots, the original evidence
ref, rerun evidence refs, observed outcomes, divergent fields, and blocked
reasons.

## States

Classification covers `stable-pass`, `stable-fail`, `flaky`, `inconclusive`,
`exhausted`, `unsupported`, and `stale`. Flaky covers pass-then-fail,
fail-then-pass, and divergent evidence; exhausted covers a used-up rerun budget;
unsupported covers reruns that are not supported; inconclusive covers undecided
evidence within budget.

## Boundary

Flake and rerun results are evidence inputs and remain review-gated until
triaged. Reruns are bounded reruns with an explicit cleanup policy and disjoint
output roots. No auto-fix, no auto-apply, no auto-merge, no self-approval, no
hidden workers, no remote swarm, and no quality guarantee are authorized. The
policy may feed run matrix, failure classification, and evidence bundle reads,
but it is review-gated and never performs trusted mutation. Browser/dashboard/
Studio surfaces remain read-only or draft-only unless a separate trusted
Rust/local API is explicitly scoped.

## Validation

Rust/local validation rejects unbounded reruns (maxReruns outside 1..=10),
missing or out-of-range consistency thresholds, overlapping output roots,
missing cleanup policy, stale run refs outside the declared run matrix,
malformed comparisons (outcome/rerun count mismatch or unsupported outcomes),
missing original evidence, remote/traversal refs, and a classification that
disagrees with the computed classification.

## Artifact separation

Run matrix, failure classification, and evidence bundle read models remain
separate. The policy may point to existing run matrix and evidence refs, but it
does not mutate them or write trusted state.

## Read model

The read model is display/export data only: classification, rerun budget and
usage, outcome counts, divergent field counts, blocked counts, validation notes,
compatibility notes, and the conservative boundary. It has no auto-fix, apply, or
command authority.

## Governance

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open. Public wording stays conservative:
this is bounded evidence-gated QA exploration, not a production game, a
shipped-game claim, a quality guarantee, a current Godot replacement, or a
production-ready claim.
