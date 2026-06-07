# Scenario Coverage v34 — Asset Pipeline Regression Suite

Issue: **#1639** (Era G Milestone 36). Part of Asset Generation and Asset-QA v1
(#1634). Scenario Coverage v34 locks the behavior of the Asset Generation
Proposal Model (#1635), the Asset-QA Gate (#1636), and the Asset Import/Atlas
Path (#1637), and guards the backward compatibility of the existing four-gate
aggregation and visual gate.

Scenario Coverage v34 is an enumerated, fixture-scoped regression suite. It
asserts **states and shapes only** — no flaky or timing-based assertions — so a
breaking change to the asset pipeline fails CI. Coverage numbering continues from
v33 (Era F) onward.

## What is covered

The matrix `examples/asset-pipeline-v1/scenario-coverage-v34/matrix.fixture.json`
enumerates every case; the runner
`crates/ouroforge-core/tests/scenario_coverage_v34_asset_pipeline.rs` executes
them against the real merged surfaces.

| Area | Case | Expected |
| --- | --- | --- |
| Asset proposal (#1635) | `V34.proposal.valid` | generates a proposal (proposed / pending / proposal-only) |
| Asset proposal | `V34.proposal.missing-license` | rejected fail-closed (missing license) |
| Asset proposal | `V34.proposal.malformed` | rejected fail-closed (out-of-bounds resolution) |
| Asset-QA gate (#1636) | `V34.qa.pass` | `pass` |
| Asset-QA gate | `V34.qa.style-fail` | `style-inconsistent` |
| Asset-QA gate | `V34.qa.provenance-fail` | `missing-provenance` |
| Asset import/atlas (#1637) | `V34.import.valid` | imported (5 assets) |
| Asset import/atlas | `V34.import.atlas-integrity` | rejected (atlas frame outside image bounds) |
| Asset import/atlas | `V34.import.hash-mismatch` | rejected (content-hash mismatch) |
| Backward compatibility | `V34.compat.four-gate-and-visual` | the four-gate `declared-gate-and` aggregation and visual gate remain valid; the asset-QA gate composes additively |

The import cases reuse the existing Asset Pipeline v1 regression fixtures
(`examples/asset-pipeline-v1-regression/`, real assets and hashes). The proposal
and asset-QA cases are fixture-scoped under
`examples/asset-pipeline-v1/scenario-coverage-v34/`.

## Backward compatibility

The suite builds the existing four-gate categories via `evaluation_gate_categories`
with a passing visual gate and asserts the base aggregation is unchanged
(`operator: declared-gate-and`, `undeclaredGatePolicy: neutral`, `visual: pass`).
It then composes the asset-QA gate and asserts the composition is **additive**: the
base aggregation is preserved and a new declared `assetQa` category is ANDed in.
The asset-QA gate is therefore a composition, not a replacement of the four gates.

## Reproduce

```bash
cargo test -p ouroforge-core --test scenario_coverage_v34_asset_pipeline
```

## Governance

- The suite asserts states and shapes only; no flaky or timing-based assertions;
  no auto-merge or self-approval.
- Generation stays proposal-only through the existing review/apply/trust-gradient
  path; license/provenance and the asset-QA gate are mandatory before promotion;
  the gate fails closed. No unlicensed/uncredited/unverified-style asset is
  promoted.
- Additive and backward-compatible; fixture-scoped; no generated runs/assets/
  release artifacts committed; conservative wording (no auto-merge/quality/fun/
  production/Godot-replacement claim).
- Rust/local owns the suite; browser/Studio surfaces read-only; no new language;
  Elixir NO-GO per ADR #92.

**#1 and #23 remain open.**
