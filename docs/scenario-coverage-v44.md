# Scenario Coverage v44 — Run and Shop Regression Suite

Issue: #1809, under Escalating Run Structure and Shop Economy v1 (#1805) and #1 Era I Milestone 49.

Scenario Coverage v44 locks the Run and Shop v1 mechanical surface with state/shape regressions only. It covers bounded escalating run shape, terminal win and loss states, shop buy/sell/reroll/remove transaction shapes, and a backward-compatibility golden proving the existing card-roguelite substrate run and shop economy remain valid.

Fixtures live under `examples/run-shop-v1/scenario-coverage-v44/` and reference the existing `examples/run-shop-v1/fixtures/` configs. The runner is Rust/local owned and requires no timing assertions, no network, no live browser, no browser command bridge, no trusted writes, and no generated run output.

Browser, dashboard, cockpit, and Studio surfaces may inspect read-only run/shop evidence only. Generation remains proposal-only through the existing review/apply/trust-gradient path. Generated runs, assets, builds, coverage output, and other artifacts stay untracked unless explicitly fixture-scoped.

#1 and #23 remain open governance anchors.

## Coverage matrix

- `v44-run-escalation-quota-curve` — escalating quota curve shape.
- `v44-run-terminal-win` — terminal `won` state, score, gold, and passed rounds.
- `v44-run-terminal-loss` — terminal `lost` state and budget exhaustion.
- `v44-shop-buy-integrity` — buy transaction shape, acquired card, gold delta, deck growth.
- `v44-shop-sell-integrity` — sell transaction shape, restored deck, no profit fabrication.
- `v44-shop-reroll-determinism` — deterministic reroll variance by seed.
- `v44-shop-remove-probability-lever` — removal lever trims `wound` from the seeded deck.
- `v44-backcompat-substrate-run-economy-golden` — existing substrate run/economy golden remains stable.
- `v44-generated-state-and-governance-audit` — conservative wording, generated-state policy, and #1/#23 anchors.

## Focused verification

```bash
cargo test -p ouroforge-core --test scenario_coverage_v44_run_shop --jobs 2
```

This coverage suite includes no auto-merge, no auto-apply, and no trusted-write path. It does not claim production-ready engine status, Godot replacement/parity, release readiness, auto-merge authority, self-approval authority, reviewer bypass, market demand, or an automated fun score. The fun/feel verdict remains human-owned.
