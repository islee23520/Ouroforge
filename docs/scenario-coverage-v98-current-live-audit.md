# Scenario Coverage v98 — Current collect-and-exit live audit

Scenario Coverage v98 locks Era S M117.2 behavior for the first formal product-observed audit of the current collect-and-exit runtime page.

## Boundary

- Uses M116.1 bundle layout under `runs/live-observability/<run-id>/` for live/manual evidence.
- Uses M117.1 category and severity ids from `docs/product-gap-taxonomy.json`.
- Records `contract-pass` and `product-observed FAIL` separately; no soft pass is allowed when diagnostics or visible usability gaps remain.
- Maps every gap to an owning milestone in M118-M130.
- Keeps generated run bundles, screenshots, browser profiles, package artifacts, and observability output out of tracked source.
- #1 and #23 remain open.

## Coverage ledger

| Row | Locked behavior |
| --- | --- |
| `v98-live-bundle-layout` | The audit references the M116.1 bundle layout at `runs/live-observability/collect-and-exit-2351-live-audit/`. |
| `v98-rendered-product-fail` | The audit records `contract-pass` and `product-observed FAIL` for the same current run. |
| `v98-screenshot-evidence` | Start and final screenshot paths are recorded as generated evidence refs. |
| `v98-replay-state-samples` | Input replay and state samples show start, after-key, and after-exit checkpoints. |
| `v98-gap-taxonomy-enforced` | Gap entries use only category/severity ids from `docs/product-gap-taxonomy.json`. |
| `v98-gap-owner-milestones` | Every gap maps to an owning milestone in M118-M130. |
| `v98-no-soft-pass` | Runtime diagnostics and debug/probe UX remain gap/backlog inputs instead of green product claims. |
| `v98-generated-state-clean` | Source tracks only the audit summary, ledger, and coverage validator; run bundles remain ignored/generated. |

## Source artifacts

- Audit report: `docs/product-observed/collect-and-exit-live-audit-2351.md`
- Gap ledger: `docs/product-observed/collect-and-exit-live-audit-gap-ledger.json`
- Smoke validator: `scripts/scenario-coverage-v98-current-live-audit.test.cjs`

## Verification

```bash
node scripts/scenario-coverage-v98-current-live-audit.test.cjs
CARGO_TARGET_DIR=/Users/jh0927/.cache/ouroforge-targets/session-e-2351 \
  cargo test --manifest-path crates/ouroforge-observability/Cargo.toml
node --check tools/live-observability-runner/runner.mjs
```

## Product-observed evidence summary

The generated live bundle `collect-and-exit-2351-live-audit` validated with ten artifacts. Replay reached `exit_reached=true`, but the rendered verdict remains `product-observed FAIL` because a runtime `missing_asset` diagnostic and visible debug/probe page shape remain. This is the requirements input for M118 runtime shell work and M120 visual rubric work; it is not a product-observed completion claim.
