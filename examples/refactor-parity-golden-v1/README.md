# Refactor Parity Golden Baseline v1 fixtures

Committed, fixture-scoped golden snapshots for the Refactor Parity Golden
Baseline (#1302) under Foundation Hardening v1 (#1301).

Each `*.verdict.golden.json` file is the byte-exact `verdict.json` produced by
`evaluate_run` for a representative deterministic demo run:

- `mechanical-pass.verdict.golden.json` — one passing scenario; verdict `passed`.
- `mechanical-fail.verdict.golden.json` — one failing scenario; verdict `failed`.
- `visual-gates.verdict.golden.json` — passing scenario with three declared
  visual acceptance comparisons; verdict `failed`.

These are guarded by `crates/ouroforge-core/tests/refactor_parity_golden.rs`,
which regenerates each verdict and fails on any byte difference. Do not edit
these files inside a crate-extraction PR. Regenerate only via an authorized,
separately reviewed behavior change:

```bash
OUROFORGE_PARITY_CAPTURE=1 cargo test --test refactor_parity_golden
```

See `docs/refactor-parity-golden-baseline-v1.md` for the full procedure.
