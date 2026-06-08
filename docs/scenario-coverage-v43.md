# Scenario Coverage v43 — Scoring-Engine Regression Suite

Issue: #1803

Scenario Coverage v43 locks Multiplicative Scoring-Engine v1 behavior with state/shape regressions only. It covers the readable modifier/effect model, deterministic resolution order and digest stability, combinatorial composition and reproducible degenerate combo surfacing, Scoring-Engine Demo v1 manifest replay, and a backward-compatibility golden proving the existing card-roguelite substrate determinism surface remains valid.

Fixtures live under `examples/scoring-engine-v1/scenario-coverage-v43/` and reference the existing scoring-engine fixture set plus the fixture-scoped demo added for #1802. The runner is Rust/local owned and requires no timing assertions, no network, no live browser, no browser command bridge, no trusted writes, and no generated run output.

Browser, dashboard, cockpit, and Studio surfaces may inspect read-only scoring evidence only. Generation remains proposal-only through the existing review/apply/trust-gradient path. Generated runs, assets, builds, coverage output, and other artifacts stay untracked unless explicitly fixture-scoped.

Conservative wording is preserved: no auto-merge, no auto-apply, self-approval, reviewer bypass, production-ready claim, Godot replacement/parity claim, autonomous shipping claim, quality verdict, or automated fun score is introduced. The fun/feel verdict remains a human Era J gate.

The regression runner is:

```bash
cargo test -p ouroforge-core --test scenario_coverage_v43_scoring_engine --jobs 2
```

Issues #1 and #23 remain open governance anchors.
