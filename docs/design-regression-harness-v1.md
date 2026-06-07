# Design Regression Harness v1 Scope and Contract

Issue: **#1587**

The Design Regression Harness v1 defines how Ouroforge re-proves a whole game when a
content or rule edit lands: **CI for game design**. On an edit, the harness re-runs the
existing solver, over-solution detector, and difficulty suite across the affected levels
and **diffs** the results against the prior baseline, classifying each level as
`unchanged`, `improved`, or `newly-broken`, with every classification linked to replayable
evidence. The harness is a **composition** of surfaces that already exist — it reuses the
`compare` run-comparison surface and the evolve-campaign stop/convergence shape. It builds
**no second comparison engine** and adds **no executable behavior** in this issue.

This is a scope/contract document for Era F Milestone 29. It adds no executable behavior,
no auto-fix, no auto-apply, no auto-merge, no self-approval, no reviewer bypass, and no
trusted browser/Studio mutation. It defines contracts, boundaries, and the follow-up
sequence only.

## Goals

- Define the harness model: which inputs trigger a re-prove, what is re-run, and what is
  diffed against the baseline.
- Define the outcome classification (`unchanged`, `improved`, `newly-broken`) and the
  evidence each verdict must link.
- Define the reuse contract: compose `compare` and the evolve-campaign stop/convergence
  shape; no parallel comparison engine.
- Define the follow-up dependency order and closure gates.
- Preserve #1 and #23 as open governance anchors.

## Non-goals

- No implementation of the Rust/local harness model, diff computation, CLI command,
  dashboard panel, Studio panel, or demo behavior in this issue. Those belong to
  #1588 (model and diff) and later follow-ups.
- No new comparison engine, solver, over-solution detector, difficulty metric, or runtime.
  The harness reuses the Milestone 28 solver/over-solution/difficulty surfaces and the
  `compare` run-comparison surface.
- No claim that a passing harness run means a game is good, fun, accessible,
  production-ready, releasable, commercially viable, or a Godot replacement/parity. The
  harness detects design regressions; it does not assert quality.
- No auto-apply of fixes. The harness is regression **detection** only; remediation stays
  human-in-the-loop through the existing review/apply/trust-gradient path.
- No direct trusted writes from generation or any browser/Studio surface. Generation is
  proposal-only; browser/dashboard/Studio surfaces remain read-only for trusted state.
- No public release, deployment, hosted telemetry, cloud service, database, auth, signing,
  store upload, CI/workflow mutation, or native/mobile/console export behavior.
- No hosted/cloud/paid capability, marketplace transaction layer, or distributed/Elixir
  orchestration (Layer-3; DEFER per #1508, NO-GO per ADR #92).

## Harness model

The harness re-proves the affected slice of a game whenever a content or rule edit changes
its design-integrity inputs. It does not re-prove the whole project blindly; it scopes the
re-run to the **affected levels** and diffs against the **prior baseline** run.

| Step | What happens | Reused surface |
| --- | --- | --- |
| 1. Trigger | A content/rule edit arrives as a proposal through the existing review/apply/trust-gradient path. It is never a direct trusted write. | `source_apply_*` / `trust_gradient_*` review/apply path |
| 2. Affected-set resolution | The harness resolves which levels the edit can affect (the levels whose rules/content changed, plus any sharing the edited mechanic). | Milestone 28 intent-capture and level/rule references |
| 3. Re-run | For each affected level the harness re-runs the existing solver (solvability), over-solution detector (alternative/shorter solutions bypassing intent), and difficulty suite (solution length, branching factor, dead-end density, mechanic-introduction order). | Milestone 28 solver / over-solution detector / difficulty-metric artifact |
| 4. Baseline diff | The harness diffs the re-run results against the prior baseline run using the existing run-comparison surface. | `compare` (`compare_runs`, `write_run_comparison_artifact`, `RunComparison`) |
| 5. Classify | Each affected level is classified `unchanged`, `improved`, or `newly-broken`, each linked to a replayable trace. | classification contract below |
| 6. Verdict + stop | The harness aggregates per-level classifications into a run verdict and terminates on a finite, bounded condition. | evolve-campaign stop/convergence shape (`evolve_campaign.rs`) |
| 7. Surface | The verdict is surfaced **read-only** in the dashboard. | dashboard / cockpit read-only panels |

The harness does not introduce a new evolve, verdict, journal, or comparison engine. It
sequences existing surfaces and records the diff as an additive artifact.

### Baseline

The baseline is the prior accepted run for the same project/level scope (a previous
harness run or a golden fixture run). A missing, malformed, or non-comparable baseline does
**not** silently pass: it produces an explicit `insufficient-data` or `unsupported` verdict
following the comparability rules already enforced by the `compare` surface
(`RunComparisonComparability`). The harness fails closed on blocked writes and invalid
inputs.

## Outcome classification

Each affected level receives exactly one classification per harness run. Every
classification links to a replayable trace and to the underlying solver/over-solution/
difficulty evidence.

| Classification | Meaning | Minimum evidence |
| --- | --- | --- |
| `unchanged` | Solvability, the over-solution set, and the difficulty metrics are equivalent to the baseline within the declared tolerance. | A run-comparison artifact showing no design-integrity delta, linked to both baseline and current solver/detector/difficulty evidence. |
| `improved` | A previously-detected over-solution is closed, an unsolvable level becomes solvable as intended, or a difficulty metric moves toward the declared target — with no new regression introduced. | A run-comparison artifact showing the closed exploit / restored solvability / improved metric, plus a replayable trace, and confirmation that no `newly-broken` condition co-occurs. |
| `newly-broken` | The edit opens a **new over-solution**, makes a previously-solvable level **unsolvable**, or causes a **difficulty regression** away from the declared target. | A run-comparison artifact showing the regression, plus a **replayable counterexample trace** (the over-solution path, the unsolvable proof, or the metric delta) linked to evidence. |

Rules:

- A run is a regression if **any** affected level is `newly-broken`. The harness fails
  closed: the design-integrity regression is surfaced, not suppressed.
- `improved` and `newly-broken` are not mutually exclusive at the run level; if both occur
  on different levels, the run verdict is a regression and both are reported.
- A `newly-broken` over-solution discovered **elsewhere** than the edited level (a rule
  tweak that opens an exploit on a different level sharing the mechanic) is the canonical
  regression case and must be detected, not just edits to the directly-changed level.
- Unknown, malformed, stale, or non-comparable inputs are never coerced into `unchanged`.
  They produce an explicit `insufficient-data` or `unsupported` verdict.

## Reuse contract

The harness composes existing surfaces. It introduces no parallel engine.

| Concern | Reused surface | Boundary |
| --- | --- | --- |
| Solvability / over-solution / difficulty | Milestone 28 solver, over-solution detector, difficulty-metric artifact, and the design-integrity gate. | The harness **calls** these; it does not re-implement solving, detection, or metric computation. |
| Run-to-baseline diff | The `compare` surface: `compare_runs`, `write_run_comparison_artifact`, `RunComparison`, `RunComparisonSnapshot`, `RunComparisonComparability`. | The harness reuses run comparison and comparability gating; it adds **no second comparison engine**. |
| Bounded, finite termination | The evolve-campaign stop/convergence shape (`evolve_campaign.rs`): ordered, finite, budgeted iterations with explicit stop conditions and the four-gate verdict vocabulary. | The harness reuses the stop-condition + budget + verdict shape; it introduces **no new evolve or campaign engine**. |
| Evidence and provenance | The provenance bundle (`provenance_bundle.rs`) and existing evidence/trace writers. | The harness links evidence; it does not invent a new provenance format. |
| Trigger path | The review/apply/trust-gradient path (`source_apply_*` / `trust_gradient_*`). | Edits enter as proposals; the harness never performs a direct trusted write. |
| Surface | The dashboard/cockpit read-only panels and the `window.__OUROFORGE__` probe / runtime. | The regression verdict is surfaced **read-only**; no browser/Studio trusted write. |

**Reuse, not re-implementation.** If a future follow-up appears to need a new comparison
engine, solver, detector, difficulty metric, evolve engine, or provenance format, that is a
signal to extend the existing surface — not to add a parallel system — unless the issue
includes an explicit, justified migration note.

## Artifact shape

Future implementation issues (#1588 onward) own the Rust structs and serializers. This
contract fixes the additive artifact shape they must preserve. The harness artifact
**references** existing run-comparison, solver, over-solution, and difficulty artifacts; it
does not duplicate their contents.

```json
{
  "schemaVersion": "design-regression-harness-v1",
  "projectId": "collect-and-exit",
  "runId": "run-456",
  "baselineRef": "previous-harness-run-or-golden-ref",
  "trigger": {
    "kind": "content-edit | rule-edit",
    "proposalRef": "review/apply proposal reference (never a direct trusted write)"
  },
  "affectedLevels": [
    {
      "levelId": "level-03",
      "classification": "unchanged | improved | newly-broken | insufficient-data | unsupported",
      "runComparisonRef": "runs/run-456/evidence/compare/level-03.json",
      "solverRef": "runs/run-456/evidence/solver/level-03.json",
      "overSolutionRef": "runs/run-456/evidence/over-solution/level-03.json",
      "difficultyRef": "runs/run-456/evidence/difficulty/level-03.json",
      "replayTraceRef": "runs/run-456/evidence/traces/level-03.json"
    }
  ],
  "verdict": "clean | regression | insufficient-data | unsupported",
  "stopCondition": "reused evolve-campaign stop/convergence descriptor"
}
```

The artifact is additive and backward-compatible. It introduces no breaking change to the
existing run, comparison, solver, evolve-campaign, provenance, evidence, or dashboard
contracts.

## Language boundary

- **Rust/local** owns the harness model, affected-set resolution, the diff computation, the
  classification logic, verdict aggregation, evidence/provenance writing, and any CLI
  behavior. Rust owns all trusted validation and persistence.
- **TypeScript/JavaScript** owns the deterministic runtime, the `window.__OUROFORGE__`
  probe, browser-local read-only inspection, and the static dashboard/cockpit read-only
  surfacing of the regression verdict where explicitly scoped.
- **Python** may be used only for temporary local tooling or smoke helpers and must not own
  any core Era F contract.
- No new language/runtime is introduced. Distributed/Elixir remains NO-GO per ADR #92
  (`docs/distributed-elixir-design.md`).

## Compatibility

- The harness is **additive**. Existing runtime, probe, evaluator four-gate aggregation,
  Milestone 28 solver/over-solution/difficulty surfaces, evolve/campaign, `compare`,
  provenance-bundle, dashboard, cockpit, and CLI contracts remain backward-compatible
  unless a later issue includes an explicit migration note.
- Generated runs, harness artifacts, comparison artifacts, traces, temp files, screenshots,
  videos, browser state, and local dashboard exports remain **untracked** unless
  intentionally fixture-scoped.
- Existing manual and proposal workflows remain valid; the harness describes design
  regressions, it does not change how edits are authored or applied.

## Follow-up dependency order and closure gates

| Order | Issue | Closure gate |
| --- | --- | --- |
| 1 | #1587 Design Regression Harness v1 Scope and Contract (this issue) | This document exists; defines the harness model, classification, reuse contract, artifact shape, and follow-up sequence; passes the governance/wording audit; #1 and #23 remain open. |
| 2 | #1588 Design Regression Harness Model and Diff v1 | Adds the Rust/local harness model, affected-set resolution, and baseline diff/classification computation reusing `compare` and the evolve-campaign stop shape. No new engine. |
| 3 | #1589 Design Regression Harness Demo v1 | Adds fixture-scoped demo evidence using the #1588 model: a rule tweak that opens a new over-solution elsewhere is detected as a regression with a replayable trace; a clean edit passes with no false regression. |
| 4 | #1590 Scenario Coverage v29: Design Regression Harness Regression Suite | Adds scenario coverage for unchanged, improved, newly-broken (new over-solution / unsolvable / difficulty regression), insufficient-data, unsupported, and boundary cases. |
| 5 | #1591 Roadmap and #1 Governance Refresh after Design Regression Harness v1 | Updates roadmap/#1 governance after the prior gates are merged and verified; confirms #1 and #23 remain open. |

Dependency chain:

```text
#1587 contract -> #1588 model/diff -> #1589 demo -> #1590 coverage v29 -> #1591 governance
```

## Wording audit

Allowed wording:

- “CI for game design”
- “design regression detection”
- “re-prove the affected levels and diff against the baseline”
- “reuses `compare` and the evolve-campaign stop/convergence shape; no second engine”
- “regression detection only, human-in-the-loop”
- “replayable counterexample trace”
- “fixture-scoped regression guard”
- “Rust/local owns the harness model and verdict serialization”

Disallowed wording:

- “the harness proves a game is good/fun/accessible”
- “production-ready harness” / “release readiness score”
- “Godot replacement/parity evidence”
- “automatic fix/apply/merge authority”
- “browser/Studio trusted write authority”
- “new comparison/solver/evolve engine”

## Governance

- Before starting, before merge or closure, and after merge or closure, verify #1587 state
  and confirm #1 and #23 remain open.
- #1 remains the roadmap anchor and #23 remains the memory anchor; this issue must not
  close or modify either anchor.
- Generation stays proposal-only through the existing review/apply/trust-gradient path;
  never a direct trusted write. Browser/Studio surfaces stay read-only.
- Genre/engine growth stays demand-driven (Milestone 24): no engine breadth beyond this
  rung's gate; cloud/hosted/marketplace monetization stays Layer-3 (DEFER per #1508).
- Future follow-up issues must cite this contract when changing the harness model, diff,
  classification, surfaces, demos, regression suites, or governance.

**#1 and #23 remain open.**
