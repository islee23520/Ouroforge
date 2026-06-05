# Evolve Loop Depth v1 Scope and Contract

Issue: #1290
Roadmap anchor: #1 Milestone 5.1, completing the original Milestone 5 ("Evolve Loop").
Status: complete after #1298 / PR evidence; this document is the scope and boundary contract for the completed Milestone 5.1 sequence.

Evolve Loop Depth v1 deepens the existing v0 evolve path into evidence-linked, domain-aware mutation proposals, and adds the four-gate before/after rerun comparison that scene-only apply (#215) explicitly deferred. The original Milestone 5 thesis requires that failed evidence drive concrete, evidence-linked mutation proposals with before/after rerun comparison; this milestone completes that thesis.

It reuses existing engines and adds none from scratch. It consumes the scene-only mutation apply path (#215), the failure classification and mutation backlog model (#692), and the visual/semantic verdicts from Evaluator Depth v1 (#1279). It does NOT introduce auto-accept, auto-apply, auto-merge, arbitrary source-code patching, or any unsupervised mutation; proposals remain manual-review inputs.

## Naming disambiguation

This milestone is distinct from the closed #215 ("Evolve Loop v2: Scene-Only Safe Mutation Application"). It is named "Evolve Loop Depth v1" specifically to avoid collision with #215. #215 delivered scene-only safe mutation application and explicitly deferred the visual/semantic rerun comparison; Evolve Loop Depth v1 consumes #215 rather than replacing it.

## Evidence-linked proposal contract

Every mutation proposal must cite:

- the specific failing gate it responds to — one of `mechanical`, `runtime`, `visual`, or `semantic`,
- the evidence artifact that justified it (the artifact reference, not a restatement),
- a confidence value derived from that evidence rather than a hardcoded constant.

Confidence is an evidence-derived, bounded signal. It is not a claim that the proposed change is correct, fun, or production-safe.

## Domain-aware selection contract

Proposal selection consumes the #692 failure classification taxonomy to map a failure class to a bounded mutation type (`data`, `scene`, or `scenario`) and to choose the next proposal from the backlog. Consuming the #692 classification is read-only; no hidden tasks or workers are created automatically. Mutation types remain bounded to data/scene/scenario level; arbitrary source-code patching remains unsupported.

## Four-gate rerun comparison contract

A before/after comparison artifact reports per-gate deltas across all four gates — `mechanical`, `runtime`, `visual`, and `semantic` — including the visual/semantic gates that #215 deferred. The comparison reuses the Evaluator Depth v1 (#1279) verdicts for the visual and semantic gates and the existing evolve rerun path for the mechanical and runtime gates.

## Reuse and ownership contract

This milestone deepens `evolve_run`, `MutationProposalRationale`, and `build_mutation_proposal_rationale`; it does not rebuild them. It consumes #215 scene-only apply, #692 classification, and #1279 (4.1) verdicts. It introduces no duplicate proposal, mutation, classification, comparison, or verdict engine.

Rust/local validation owns trusted persistence, proposal/comparison logic, and CLI contracts. Browser, dashboard, and Studio surfaces remain read-only for trusted state unless a later explicitly scoped Rust/local trusted API owns persistence. Artifact changes are additive and backward-compatible; existing evolve/mutation runs stay compatible.

## Prerequisite

Evaluator Depth v1 (#1279, especially #1283/#1284/#1285) is a prerequisite for the visual/semantic citation, since the visual and semantic gate verdicts originate there.

## Follow-up issue sequence

1. Evolve Loop Depth v1 Scope and Contract (this issue) — #1290
2. Evidence-Linked Mutation Proposal v1 — #1292
3. Failure-Classification-Driven Proposal Selection v1 — #1293
4. Four-Gate Rerun Comparison and Evolve Journal v2 — #1294
5. Studio Evolve Depth Inspection Surface v1 — #1295
6. Evolve Depth Demo v1 — #1296
7. Scenario Coverage v20: Evolve Depth Regression Suite — #1297
8. Roadmap and #1 Governance Refresh after Evolve Loop Depth v1 — #1298

```text
(prereq: 4.1 #1283/#1284/#1285)
#1290 contract -> { #1292 proposal, #1293 selection } -> #1294 rerun-compare + journal
       -> { #1295 studio, #1296 demo, #1297 coverage v20 } -> #1298 governance
```

The contract lands first; the evidence-linked proposal and classification-driven selection proceed in parallel; the rerun comparison and journal join them; the surface, demo, and coverage proceed in parallel; the governance refresh closes the milestone.


## Completion evidence

Evolve Loop Depth v1 is complete for #1 Roadmap Alignment Addendum Milestone 5.1. The merged evidence chain is:

- #1290 — scope and naming contract for Evolve Loop Depth v1, disambiguated from #215.
- #1292 — evidence-linked mutation proposal rationale with per-gate citations and bounded confidence.
- #1293 — failure-classification-driven proposal selection with bounded data/scene/scenario mutation types and backlog-only unsupported/unknown/flaky classes.
- #1294 — four-gate rerun comparison and Evolve Journal v2, including visual/semantic deltas.
- #1295 — read-only Studio Evolve Depth inspection surface.
- #1296 — deterministic fixture-scoped demo showing visual fail-to-pass through a scene-only #215 apply operation.
- #1297 — Scenario Coverage v20 regression fixtures and legacy evolve v0 golden compatibility.
- #1298 — roadmap and #1 governance refresh.

The milestone completed the original Milestone 5 thesis mechanically: failed evidence can now drive evidence-linked, manually reviewed proposals and a before/after rerun comparison can record four-gate deltas. This is not a claim that proposals are correct, fun, production-safe, auto-applicable, or source-patch authorized.

## Verification and closure gates

This is a scope/contract issue and adds no executable behavior. PR evidence must include the canonical `docs/evolve-loop-depth-v1.md`, the explicit reuse/no-rebuild statement against `evolve_run`/#215/#692/#1279, a naming-disambiguation note versus closed #215, a wording audit, and a #1/#23 governance audit. Follow-up implementation PRs must additionally pass the standard repository gates (`cargo fmt --check`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, the dashboard/cockpit node smokes, `git diff --check`, and a clean `git status --short --ignored`) and add focused tests for the behavior they implement.

## Explicit non-goals

- No auto-accept, auto-apply, auto-merge, self-approval, reviewer bypass, or unsupervised mutation; the manual-review lifecycle from #215 is preserved.
- No arbitrary source-code patching; mutation types remain bounded to data/scene/scenario.
- No second proposal, mutation, classification, comparison, or verdict engine; existing engines are deepened, not rebuilt.
- No confidence claim that a proposed change is correct, fun, or production-safe.
- No browser trusted writes; the browser remains read-only for trusted state.
- No generated runs, proposals, comparison artifacts, or traces tracked unless explicitly fixture-scoped.
- No production-ready or current Godot replacement claim.

## Governance anchors

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor. Both remain open through this contract issue; this milestone does not close or modify either without a separate explicit governance decision.
