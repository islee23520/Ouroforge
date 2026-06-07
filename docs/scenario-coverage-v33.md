# Scenario Coverage v33 — Evidence-Native Marketplace Regression Suite

Issue: **#1616** (Era F Milestone 33). Part of Evidence-Native Marketplace v1
(#1612); locks the behavior of the Local Verifiable-Asset Registry v1 (#1613)
and the Evidence-Native Marketplace Demo v1 (#1615).

Scenario Coverage v33 is an enumerated, fixture-scoped regression suite. It
asserts **states and shapes only** — no flaky or timing-based assertions — so a
breaking change to the marketplace registry fails CI.

## What is covered

The matrix is `examples/evidence-marketplace-v1/scenario-coverage-v33/matrix.fixture.json`,
driven by `crates/ouroforge-core/tests/scenario_coverage_v33_evidence_marketplace.rs`.

- **Registry validation** (`registryCases`): a valid asset publishes (with its
  replay proof and a complete provenance status); a proof-less asset is rejected;
  a provenance-gapped asset is rejected. Fail-closed.
- **Verify-on-consume + tamper detection** (`verifyCases`): a valid asset
  verifies on consume (its proof re-runs — the recorded template digest matches
  and the bound provenance bundle resolves to a complete chain); a tampered asset
  (template digest mismatch) is rejected.
- **Provenance lineage** (`provenanceLineageCases`): the valid asset's bound
  Milestone 25 provenance bundle resolves to a **complete** chain against the
  local evidence root.
- **Backward compatibility** (`backwardCompatibility`): the Milestone 25
  provenance bundle remains **valid standalone** — it parses and evaluates as a
  plain `provenance-bundle-v1`, with no marketplace wrapper. The registry is
  additive over the existing bundle; prior behavior is unchanged.

## Reproduce

```bash
cargo test -p ouroforge-core --test scenario_coverage_v33_evidence_marketplace
```

The suite reuses the registry (#1613), `provenance_bundle.rs` (#1500), and
`export_hash` surfaces — it is a regression suite, not a new engine, runtime, or
writer.

## Boundaries and governance

- **Local only.** No hosted, paid, or network marketplace capability; any
  transaction layer or take-rate stays DEFER until a #1508 Layer-3 GO (Elixir
  NO-GO per ADR #92).
- **Proposal-only.** Adoption of a consumed asset flows through the existing
  review/apply/trust-gradient path, never a direct trusted write; browser/Studio
  surfaces stay read-only.
- **Additive and backward-compatible.** New fixtures, a runner, and this doc
  only; no change to existing contracts.
- Generated state remains untracked unless explicitly fixture-scoped; the v33
  matrix and fixtures are tracked source-like regression artifacts.
- Conservative wording: this locks a local verification workflow over fixture
  evidence only. It makes no production, quality, or Godot-comparison claim, and
  does not assert that generated games are good, fun, or shippable.

**#1 remains open. #23 remains open.**
