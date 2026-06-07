# Production-Scale QA Matrix v1 Scope and Contract

Issue: #1665
Roadmap anchor: #1 Era G Milestone 40 (Production-Scale QA Matrix).
Status: scope and contract only; adds no executable behavior.

Production-Scale QA Matrix v1 scales QA from per-artifact checks to whole-game
production QA. It defines a regression matrix across content variants, seeds,
and supported targets; visual-regression at scale; soak and performance testing;
crash, flaky, accessibility, and asset-UX QA; and a single consolidated
production-QA verdict per game build. Every capability reuses the existing
QA / playtest swarm, runtime, evaluator, compare, evolve/campaign, provenance,
and asset surfaces. It does not authorize a new test engine, a parallel runtime,
auto-fix, auto-apply, auto-merge, browser trusted writes, or any quality
guarantee. The verdict is descriptive evidence only; it is never a guarantee of
quality, fun, accessibility compliance, market readiness, or shipped-game
readiness.

This document is the canonical contract for the Production-Scale QA Matrix v1
follow-up issues (#1666–#1672). Each follow-up slice remains bounded by the
trust, reuse, and wording boundaries defined here.

## Bounded target

The milestone covers the following bounded capabilities, each implemented and
verified independently and each reusing an existing runner rather than adding a
new engine:

- Regression matrix (content x seed x target): a bounded matrix binding content
  variants, deterministic seeds, and supported targets to QA runs, extending the
  existing QA / playtest swarm run matrix and Milestone 14 contracts and the
  existing `regression-run-matrix-v1` and export target matrix surfaces.
- Visual-regression at scale: threshold-based visual comparison evidence across
  the matrix, reusing the existing visual gate and `compare` artifacts; no new
  image-diff engine and no subjective "looks good" judgement.
- Soak and performance testing: bounded, repeated-run soak evidence and
  frame/time/memory budget evaluation reusing the existing performance budget
  surface; explicit run-count, duration, and budget limits; no unbounded loops.
- Crash / flaky / accessibility / asset-UX QA: deterministic crash and runtime
  error classification, bounded flake confirmation under the existing rerun
  policy, structural accessibility checks, and asset-UX checks; each is a
  bounded evidence signal, not a compliance certification.
- Consolidated production-QA verdict: a single descriptive verdict per game
  build that aggregates the matrix, visual, soak/performance, and
  crash/flaky/accessibility/asset-UX evidence, composing existing gate outcomes
  (for example via the evaluator `declared-gate-and` pattern) rather than
  introducing a new judgement engine.

## Trusted boundary

- QA matrix outputs are descriptive evidence inputs only until reviewed. They
  never perform a trusted mutation, auto-fix, auto-apply, auto-merge,
  self-approval, or reviewer bypass.
- Rust/local validation owns the trusted matrix/verdict logic, QA artifact
  validation, generated-evidence writing, run/project binding, the existing
  review/apply/trust-gradient path, and CLI contracts.
- Browser, dashboard, and Studio surfaces remain read-only for trusted state.
  They cannot run commands, bridge to a local server, write trusted files,
  install dependencies, or reach the network.
- The consolidated verdict is a descriptive aggregation of bounded signals, not
  proof of fun, subjective quality, accessibility compliance, market readiness,
  production safety, current Godot replacement/parity, or shipped-game
  readiness.
- Generated assets, audio, or content referenced by a QA run are never promoted
  without license/provenance and the function-specific QA gate; art/audio/UX/
  narrative "looks good / sounds good / is fun" decisions remain human.

## Budgets, limits, and cleanup policy

Every Production-Scale QA Matrix follow-up issue must declare explicit, bounded
budgets and reuse the existing QA swarm worker/budget policy:

- Matrix dimensions: explicit, bounded sets of content variants, seeds, and
  supported targets; no unbounded combinatorial expansion.
- Worker budget: a maximum number of local-first workers per run, reusing the
  existing worker pool; no hidden background workers and no unbounded spawning.
- Soak limits: a maximum repeated-run count and a maximum wall-clock duration
  per soak run.
- Rerun limits: a maximum bounded rerun count for flake confirmation under the
  existing rerun policy; no unsupervised long-running loops.
- Timeout limits: a maximum wall-clock budget per worker and per run.
- Output roots: a declared output root for generated runs, screenshots, traces,
  and dashboard exports.
- Cleanup policy: generated runs, assets, content, and release artifacts remain
  ignored and cleaned up unless explicitly fixture-scoped.

## Dependency order for follow-up issues

1. Scope and Contract (this issue, #1665) lands first.
2. Regression Matrix (Content x Seed x Target) v1 — #1666.
3. Visual-Regression at Scale v1 — #1667.
4. Performance and Soak Testing v1 — #1668.
5. Crash/Accessibility QA and Consolidated Verdict v1 — #1669.
6. Production-Scale QA Matrix Demo v1 — #1670.
7. Scenario Coverage v38: Production-Scale QA Matrix Regression Suite — #1671.
8. Roadmap and #1 Governance Refresh after Production-Scale QA Matrix v1 — #1672.

```text
#1665 scope -> #1666 -> #1667 -> #1668 -> #1669 -> #1670 -> #1671 -> #1672
```

Each follow-up issue must verify its slice independently and must not combine the
matrix, visual, soak/performance, crash/accessibility, verdict, demo, coverage,
and governance behavior into a single PR when they can be verified separately.

## Verification and closure gates

Every follow-up PR must pass the standard repository gates (`cargo fmt --check`,
`cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, the
dashboard/cockpit node smokes, `git diff --check`, and a clean
`git status --short --ignored`) and must add focused tests/smokes for the exact
QA matrix behavior it implements. Closure evidence must include generated-state,
conservative-wording, backward-compatibility, bounded-budget, reuse (no new test
engine), and no-auto-fix audits, and must reconfirm that #1 and #23 remain open.
Scenario Coverage numbering continues from v33 (Era F) onward; the Production-Scale
QA Matrix regression suite lands as Scenario Coverage v38.

## Explicit non-goals

- No new test engine, parallel runtime, or judgement engine; reuse the existing
  QA / playtest swarm, runtime, evaluator, compare, evolve/campaign, provenance,
  and asset surfaces.
- No direct trusted writes from generation, role agents, the producer, or any
  browser/Studio surface; proposals only, through the existing
  review/apply/trust-gradient path.
- No autonomous apply, auto-fix, auto-apply, auto-merge, self-approval, reviewer
  bypass, or hidden trusted write; high-risk and source-affecting changes are
  never auto-applied.
- No unlicensed, uncredited, or unverified-style generated asset/audio/content
  promotion; license/provenance and the function-specific QA gate are mandatory.
- No automated quality/fun/taste claim; "looks good / sounds good / is fun" and
  art/audio/UX/narrative direction remain human decisions.
- No shipping (native/store export), hosted/cloud, real-player telemetry, or
  live-ops absent an explicit Layer-3 GO (DEFER per Milestone 26 / #1508);
  distributed/Elixir remains NO-GO per ADR #92.
- No real-player data (Layer-3); synthetic runs only.
- No flaky or timing-based assertions in tests.
- No claim of production-ready engine, Godot replacement/parity, or autonomous
  shipping of finished games.
- No generated runs, assets, content, or release artifacts tracked unless
  explicitly fixture-scoped.

## Governance anchors

#1 remains the broad roadmap/final-goal anchor. #23 remains the
repo-memory/design context anchor. Both remain open through this contract issue;
this milestone does not close or modify either without a separate explicit
governance decision.
