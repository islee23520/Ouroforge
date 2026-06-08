# Scenario Coverage v40 — Autonomous Producer Regression Suite

Issue: #1687

Scenario Coverage v40 locks Autonomous Producer and Whole-Game Orchestration v1 with state/shape regressions only. It covers producer plan decomposition, orchestration progression and resume, budget/stop/human-gate handling, and a backward-compatibility golden proving the existing single-artifact evolve campaign remains valid.

Fixtures live under `examples/autonomous-producer-v1/scenario-coverage-v40/` and reference the existing producer plan, orchestration, budget-gate, demo, and evolve-campaign fixtures. The suite is Rust/local owned and requires no network or live browser.

Browser, dashboard, and Studio surfaces may inspect read models only. Producer output remains proposal-only through the existing review/apply/trust-gradient path. Generated runs, assets, content, release-candidate artifacts, and coverage outputs stay untracked unless explicitly fixture-scoped.

Conservative wording is preserved: no auto-merge, auto-apply, self-approval, reviewer bypass, production-ready claim, Godot replacement/parity claim, autonomous shipping claim, or quality/fun guarantee is introduced.

Issues #1 and #23 remain open.
