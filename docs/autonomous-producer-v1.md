# Autonomous Producer and Whole-Game Orchestration v1

Issue: **#1682** (Era H Milestone 43 scope and contract)

Autonomous Producer and Whole-Game Orchestration v1 is the Milestone 43 scope contract for driving an entire small-but-complete game from a human design intent to a release candidate as one **bounded, auditable campaign**. It is an accountability, decomposition, and approval framework, not an autonomous unrestricted game-completion system, hidden worker runtime, hosted/cloud orchestrator, browser command bridge, release pipeline, current Godot replacement, or production-ready claim.

This document is the control contract for follow-up issues #1683 through #1688. It defines what later decomposition, orchestration-state, budget/approval, demo, coverage, and governance work may implement without re-opening the milestone boundary. **This contract issue adds no executable behavior.** The producer never performs a direct trusted write or release.

## Bounded target

The v1 target is a deterministic, local-first **producer campaign** that composes existing surfaces — it adds no parallel engine. A producer campaign is describable through four contracts:

1. **Design-intent decomposition** — turn a single human design intent into a structured production plan, reusing Milestone 30 generation and the GDD / design-brief surfaces.
2. **Whole-game orchestration state** — a long-horizon plan and campaign state that extends the Milestone 23 campaign model and the Milestone 42 multi-agent production pipeline.
3. **Game-scale budgets and stop conditions** — bounded resource/iteration/time ceilings that fail closed, with explicit stop conditions.
4. **Mandatory human approval gates** — gates a human must clear before any apply, promotion, or release-candidate transition.

All artifacts are source-like contracts or fixture-scoped examples unless a follow-up issue explicitly scopes a trusted Rust/local writer. Generated production plans, orchestration state, budget ledgers, runs, traces, screenshots, dashboard exports, temporary projects, release artifacts, and local tool state remain **untracked** unless explicitly fixture-scoped.

## Design-intent decomposition contract

The producer decomposes a single human design intent into a production plan by **reusing existing generation and design surfaces**; it introduces no new generator:

- Intent is captured through the existing GDD / design-brief surfaces (Milestone 30 generation).
- The plan is a structured, ordered set of bounded work packages mapped to the Milestone 42 production task board, role model, ownership, and acceptance contracts.
- Every plan item references its originating intent and the surface that produces its proposal; the decomposition itself is a **proposal**, not a trusted write.
- Decomposition is deterministic and re-derivable from the recorded intent and the existing surfaces; it stores no hidden authority.

Generated/decomposed plan output is proposal-only and reaches trusted state only through the existing review/apply/trust-gradient path.

## Whole-game orchestration state contract

Whole-game orchestration **extends, and does not replace**, the Milestone 23 campaign model and the Milestone 42 pipeline:

- A producer campaign is a long-horizon plan over multiple bounded rungs/work packages, each owned and reviewed exactly as in Milestone 42.
- Campaign state is additive: existing campaign, run, and pipeline records remain valid without a producer envelope.
- Orchestration state tracks plan progress, outstanding budgets, pending approval gates, and stop-condition status; it records references, not new trusted artifacts.
- Growth stays demand-driven (Milestone 24): the campaign only spans the engine/content/system breadth a specific loop-produced rung justifies.

The orchestration layer is composition by reference over existing campaign/pipeline/run/provenance state. It is not a new scheduler, worker pool, or runtime.

## Game-scale budgets and stop conditions contract

Producer campaigns are never unbounded:

- Every campaign declares **game-scale budgets** (for example iteration count, wall-clock/time, run/compute, and review ceilings). Budgets are bounded and **fail closed**: exceeding a budget halts further proposals rather than escalating authority.
- **Stop conditions** are explicit and enumerated in follow-up issues (for example budget-exhausted, blocked-on-approval, repeated-failure, invalid-input, missing-license). Reaching a stop condition halts the campaign and surfaces the reason; it never auto-applies, auto-merges, or self-approves to make progress.
- Budget and stop-condition accounting is owned by Rust/local logic and recorded as evidence; browser/Studio surfaces render it read-only.

## Human approval gate contract

Human approval gates are **mandatory** and cannot be bypassed:

- No apply, promotion, or release-candidate transition occurs without an explicit human approval recorded through the existing review/apply/trust-gradient path.
- High-risk and source-affecting changes are **never** auto-applied; generation, role agents, the producer, and any browser/Studio surface emit proposals only.
- Generated assets/audio/content require license/provenance plus the function-specific QA gate before any promotion; `looks good / sounds good / is fun` and art/audio/UX/narrative direction remain human decisions.
- Humans retain vision, taste, legal, and release go/no-go. The producer prepares evidence and proposals; it does not decide.

## Trusted boundary

Producer outputs are untrusted proposals until Rust/local validation and review-gated apply or promotion accept them. The trusted side owns:

- schema and invariant validation of plans, orchestration state, and budgets;
- project/run/campaign binding;
- artifact path and generated-state checks;
- budget/stop-condition accounting and evidence writing when explicitly scoped;
- CLI contracts and local persistence;
- review/promotion/approval decision validation.

Browser, dashboard, and Studio surfaces are read-only consumers of trusted state. They may render plans, orchestration state, budgets, stop conditions, approval gates, and gaps, but they must not spawn workers, execute commands, write trusted files, apply source changes, auto-promote outputs, merge PRs, alter visibility, or bypass review.

## Dependency order and closure gates

Follow-up issues proceed in this dependency-safe order unless a later issue body explicitly proves a narrower independent slice:

| Order | Issue | Dependency purpose |
| --- | --- | --- |
| 1 | #1682 | Scope contract and milestone boundary (this issue). |
| 2 | #1683 | Design-intent decomposition and production plan v1. |
| 3 | #1684 | Whole-game orchestration state v1. |
| 4 | #1685 | Budgets, stop conditions, and human approval gates v1. |
| 5 | #1686 | Autonomous producer demo v1. |
| 6 | #1687 | Scenario coverage v40: autonomous producer regression suite. |
| 7 | #1688 | Roadmap and #1 governance refresh after autonomous producer v1. |

```text
#1682 scope -> #1683 -> #1684 -> #1685 -> #1686 -> #1687 -> #1688
```

Each issue and PR unit records evidence for its own boundary:

- live GitHub checks for the issue and for #1/#23 remaining open;
- focused Rust tests or Node smokes for new schemas, read models, dashboard/cockpit display, or fixture contracts;
- `cargo fmt --check`, relevant `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings` when Rust contracts change;
- `node --check` and dashboard/cockpit tests when browser display surfaces change;
- `git diff --check` and `git status --short --ignored` for whitespace and generated-state hygiene;
- issue closure comment listing PRs, commits, verification commands, generated-state policy, conservative wording audit, and #1/#23 status.

A closure gate passes only when the issue-specific acceptance criteria are met without broadening authority beyond this document. Scenario coverage numbering continues from v33 (Era F) onward; this milestone's regression suite is v40.

## Reuse and compatibility

- **Compose by reference** to existing runtime, probe, evaluator (`declared-gate-and`), visual gate, `compare`, evolve/campaign (`evolve_campaign.rs`), provenance (`provenance_bundle.rs`), asset-manifest, `source_apply_*` / `trust_gradient_*`, QA-swarm, dashboard, cockpit, and CLI surfaces. **No parallel engine, runtime, or writer.**
- **Backward-compatible** and **additive**; preserve existing project, scaffold, scene, tilemap, asset, behavior, level, GDD, scenario, campaign, pipeline, provenance, dashboard, and Studio contracts unless a PR includes an explicit migration note and targeted compatibility tests.
- Source-like fixtures may be added only when deterministic and issue-scoped; generated runs/assets/content/release artifacts stay out of git unless explicitly fixture-scoped.

## Language boundary

- **Rust/local** owns trusted validation, persistence, decomposition/orchestration/budget/approval/provenance/compliance logic, evidence writing, run/project/campaign binding, the review/apply/trust-gradient path, and CLI behavior.
- **TypeScript/JavaScript** owns the deterministic runtime (including in-game UI/HUD/menus), the `window.__OUROFORGE__` probe, browser-local read-only inspection, and static dashboard/cockpit behavior where explicitly scoped.
- **Python** may be used only for temporary local tooling or smoke helpers and must not own core Era G/H contracts.
- No new language/runtime is introduced without explicit issue-level authorization; distributed/Elixir remains **NO-GO** per ADR #92 (`docs/distributed-elixir-design.md`).

## Conservative wording policy

Public wording describes v1 as a local-first, evidence-gated, human-gated producer campaign. Do not claim autonomous arbitrary game completion, automated quality/fun/taste judgement, production readiness, commercial readiness, current Godot replacement/parity, hosted orchestration, cloud worker pools, native export shipping, release automation, or publication support. Shipping (native/store export), hosted/cloud, real-player telemetry, and live-ops are **DEFER** absent an explicit Layer-3 GO (Milestone 26 / #1508).

## Explicit non-goals

Out of scope for v1:

- direct trusted writes from generation, role agents, the producer, or any browser/Studio surface; proposals only, through the existing review/apply/trust-gradient path;
- autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden trusted writes; high-risk and source-affecting changes are never auto-applied;
- unbounded autonomy; budgets, stop conditions, and human approval gates are mandatory;
- promoting any unlicensed, uncredited, or unverified-style generated asset/audio/content; license/provenance and the function-specific QA gate are mandatory;
- automated quality/fun/taste claims; art/audio/UX/narrative direction remains a human decision;
- shipping (native/store export), hosted/cloud, real-player telemetry, or live-ops absent an explicit Layer-3 GO; distributed/Elixir remains NO-GO per ADR #92;
- engine/content/system breadth beyond what a specific loop-produced rung (Milestone 24) justifies;
- a new engine/runtime/writer/scheduler/worker pool, or any claim of a production-ready engine, Godot replacement/parity, or autonomous shipping of finished games.

## Definition of done for the milestone

The milestone is done when the follow-up issues produce validated local decomposition, orchestration-state, and budget/approval contracts plus a deterministic demo, regression coverage, and governance refresh that reuse Milestone 23/30/42 surfaces — with no new orchestrator engine, the producer never performing a direct trusted write or release, mandatory human approval gates, generated-state hygiene, conservative wording, and #1/#23 governance preservation recorded in final evidence.

**#1 and #23 remain open.**
