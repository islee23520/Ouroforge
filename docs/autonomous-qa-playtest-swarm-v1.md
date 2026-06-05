# Autonomous QA / Playtest Swarm v1 Scope and Contract

Issue: #682
Roadmap anchor: #1 Milestone 14 (Autonomous QA and Playtest Swarm).
Status: complete as bounded local QA/playtest evidence contracts after #698 governance refresh; no hidden workers, remote/cloud swarm, auto-fix/apply/merge, browser trusted writes, quality guarantee, current Godot replacement, or production-ready claim.

Autonomous QA / Playtest Swarm v1 enables bounded, local-first QA/playtest workers to generate, run, classify, and report adversarial gameplay evidence. It does not authorize hidden workers, unbounded autonomous agents, remote/cloud swarms, browser command bridges, auto-fix, auto-apply, auto-merge, or any quality guarantee. QA/playtest outputs are evidence/backlog inputs only until reviewed.

This document is the canonical contract for the completed QA/playtest swarm issues. Each completed slice remains bounded by the trust and wording boundaries defined here.

## Bounded target

The milestone covers the following bounded capabilities, each implemented and verified independently:

- QA scenario generation: deterministic candidate scenarios derived from existing scenario/level/GDD contracts.
- Adversarial input fuzzing: bounded fuzz plans over declared input surfaces with explicit input limits.
- Worker assignment and budget policy: local-first worker assignment with explicit worker, input, rerun, and timeout budgets.
- Runtime invariant checking: consumption of the existing runtime invariant checker as evidence, not a second engine.
- Objective route attempt evidence: bounded objective/route attempt records as evidence.
- Visual regression evidence: threshold-based visual comparison evidence reusing existing visual comparison artifacts.
- Performance budget evaluation: bounded frame/time/memory budget evidence.
- Console/crash/runtime error classifier: deterministic classification of captured console, crash, and runtime errors.
- Flaky evidence and bounded rerun policy: explicit, bounded rerun limits with flake evidence; no unbounded retry loops.
- Failure classification and mutation backlog: deterministic failure classes feeding a review-gated backlog.
- QA swarm run matrix: a bounded matrix binding scenarios, workers, and budgets to runs.
- QA swarm evidence bundle: an aggregated, fixture-scopable evidence bundle.
- Studio inspection, demo, and regression suite: read-only inspection surfaces, a bounded demo, and a regression suite.

## Trusted boundary

- QA/playtest outputs are evidence/backlog inputs only until reviewed. They never perform trusted mutation, auto-fix, auto-apply, or auto-merge.
- Rust/local validation owns trusted persistence, QA artifact validation, generated-evidence writing, run/project binding, and CLI contracts.
- Browser, dashboard, and Studio surfaces remain read-only or draft-only for trusted state unless a later explicitly scoped Rust/local trusted API owns persistence. They cannot run commands, bridge to a local server, write trusted files, install dependencies, or reach the network.
- Heuristic route/visual/performance/failure classifiers are bounded evidence signals, not proof of fun, subjective quality, market readiness, production safety, accessibility compliance, or shipped-game readiness.

## Budgets, limits, and cleanup policy

Every QA/playtest follow-up issue must declare explicit, bounded budgets:

- Worker budget: a maximum number of local-first workers per run; no hidden background workers and no unbounded spawning.
- Input limits: a maximum count/size for generated and fuzzed inputs.
- Rerun limits: a maximum bounded rerun count for flake confirmation; no unsupervised long-running loops.
- Timeout limits: a maximum wall-clock budget per worker and per run.
- Output roots: a declared output root for generated runs, fuzz inputs, screenshots, videos, traces, and dashboard exports.
- Cleanup policy: generated artifacts remain ignored and are cleaned up unless explicitly fixture-scoped.

## Dependency order for follow-up issues

1. This scope and contract issue (#682) lands first.
2. Scenario generation, adversarial fuzzing, worker/budget policy, and invariant/route/visual/performance/error evidence proceed as bounded, independently verifiable slices.
3. Flake policy, failure classification, and the mutation backlog build on the evidence slices.
4. The run matrix and evidence bundle aggregate the slices above.
5. Studio inspection, the demo, and the regression suite proceed in parallel once evidence and aggregation exist.
6. A roadmap and #1 governance refresh closes the milestone.

Each follow-up issue must verify its slice independently and must not combine scenario generation, fuzzing, workers, invariants, visual/performance/error evidence, flake policy, backlog, bundle, Studio, and demo behavior into a single PR when they can be verified separately.

## Verification and closure gates

Every follow-up PR must pass the standard repository gates (`cargo fmt --check`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, the dashboard/cockpit node smokes, `git diff --check`, and a clean `git status --short --ignored`) and must add focused tests/smokes for the exact QA/playtest behavior it implements. Closure evidence must include generated-state, conservative-wording, backward-compatibility, bounded-budget, no-auto-fix, and no-hidden-worker audits, and must reconfirm that #1 and #23 remain open.

## Explicit non-goals

- No hidden background workers, unbounded agent spawning, remote/cloud swarm, hosted orchestration, or unsupervised long-running loop.
- No auto-fix, auto-apply, auto-merge, self-approval, reviewer bypass, or trusted mutation.
- No browser trusted writes, command bridge, local server bridge, hidden command execution, credentialed commands, network/install commands, or dependency mutation.
- No unrestricted source mutation, arbitrary script execution, dynamic code loading, plugin loading, or visual scripting implementation.
- No claim that QA proves fun, subjective quality, market readiness, production safety, accessibility compliance, current Godot replacement, or production-ready status.
- No native export, release automation, publish action, or public visibility change.
- No generated runs, fuzz inputs, screenshots, videos, traces, dashboard exports, or local tool state tracked unless explicitly fixture-scoped.

## Governance anchors

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor. Both remain open through this contract issue; this milestone does not close or modify either without a separate explicit governance decision.
