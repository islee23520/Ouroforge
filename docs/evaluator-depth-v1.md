# Evaluator Depth v1 Scope and Contract

Issue: #1279  
Roadmap anchor: #1 Milestone 4.1, completing Milestone 4 (Scenario DSL and Evaluator) and Architectural Pillar 1 (Evaluate).  
Status: scope contract only; no executable behavior.

Evaluator Depth v1 promotes existing runtime and visual evidence into declarable scenario acceptance gates. It defines four sibling verdict categories — `mechanical`, `runtime`, `visual`, and `semantic` — without adding trusted mutation, auto-fix, auto-apply, auto-merge, or reviewer bypass.

## Gate categories

- `mechanical`: deterministic scenario/evaluator assertions over declared scenario steps and expected outcomes.
- `runtime`: bounded runtime health checks, crashes, console errors, frame-budget evidence, and runtime invariant inputs already owned by Rust/local validation.
- `visual`: threshold-based comparison against declared visual evidence. The gate consumes `VisualComparisonEvidenceArtifact` from #688 and must not introduce a duplicate screenshot diff engine.
- `semantic`: declared invariant-rule evaluation over runtime/world-state evidence. The gate consumes the runtime invariant checker from #686 and must not introduce a second semantic/invariant engine.

The overall verdict passes only when every declared gate passes. An undeclared gate is neutral and never an implicit fail. Unsupported, malformed, missing, or stale declared gate evidence fails visibly for that declared gate and is preserved in the verdict/journal read model.

## Scenario and Seed acceptance declarations

A Seed or scenario may declare visual acceptance with:

- a local baseline screenshot or visual checkpoint reference,
- the captured screenshot/evidence reference produced by the run,
- an explicit threshold or tolerance,
- evidence freshness metadata tying the comparison to the run and scenario id.

A Seed or scenario may declare semantic acceptance with:

- local invariant rule references,
- runtime/world-state evidence references,
- the expected invariant status,
- evidence freshness metadata tying the invariant report to the run and scenario id.

Acceptance declarations are part of the scenario contract. QA-swarm visual comparisons or invariant reports that are not declared acceptance criteria remain evidence-only and do not change the four-gate verdict.

## Reuse and ownership contract

Rust/local validation owns trusted persistence, gate logic, verdict serialization, evidence freshness checks, and CLI contracts. Browser, dashboard, and Studio surfaces may display evidence and read models, but they do not own trusted state or gate decisions unless a later explicitly scoped Rust/local trusted API grants that boundary.

The visual gate reuses `VisualComparisonEvidenceArtifact` (#688). The semantic gate reuses the runtime invariant checker (#686). This issue does not build new diff, visual, semantic, invariant, AI-judgment, or repair engines.

## Additive `verdict.json` extension

Evaluator Depth v1 extends verdict data additively:

```json
{
  "mechanical": [],
  "runtime": [],
  "visual": [],
  "semantic": [],
  "overallStatus": "passed|failed|unsupported|malformed"
}
```

Backward compatibility rule: existing two-gate verdicts remain byte-compatible. Consumers that do not know `visual` or `semantic` continue to read existing fields. New readers treat absent `visual` and `semantic` fields as empty/`unsupported` display state, not as implicit failures.

## Follow-up dependency order

1. Visual Evaluator Gate v1 — #1283
2. Semantic Evaluator Gate v1 — #1284
3. Four-Category Verdict and Journal Integration v1 — #1285
4. Studio Evaluator Depth Inspection Surface v1 — #1286
5. Evaluator Depth Demo v1 — #1287
6. Scenario Coverage v19: Evaluator Depth Regression Suite — #1288
7. Roadmap and #1 Governance Refresh after Evaluator Depth v1 — #1289

Dependency order: #1279 contract -> { #1283 visual, #1284 semantic } -> #1285 verdict and journal -> { #1286 Studio, #1287 demo, #1288 coverage } -> #1289 governance refresh.

## Explicit non-goals

- No new screenshot diff, visual comparison, semantic, or invariant engine.
- No taste, beauty, fun, accessibility, market-readiness, or broad design-quality judgment.
- No claim that semantic gates prove a game is correct beyond declared invariant rules.
- No auto-fix, auto-apply, auto-merge, self-approval, reviewer bypass, or trusted mutation.
- No browser trusted writes, command bridge, hidden local server bridge, arbitrary script execution, dynamic code loading, plugin loading, or source mutation.
- No production game, shipped-game, commercial readiness, broad compatibility, native export, hosted/cloud, or current engine replacement claim.
- No generated runs, screenshots, traces, diff/heatmap artifacts, dashboard exports, or local tool state tracked unless explicitly fixture-scoped.

## Governance anchors

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor. Both remain open through this contract issue.
