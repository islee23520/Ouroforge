# Asset Generation and QA Demo v1

Issue: **#1638** (Era G Milestone 36). Part of Asset Generation and Asset-QA v1
(#1634). This is a **deterministic, fixture-scoped** demo: it composes the
already-merged Asset Generation Proposal Model (#1635), the Asset-QA Gate
(#1636), and the Asset Import/Atlas Path (#1637) into one end-to-end walkthrough.
It adds **no** new engine, runtime, or writer — it sequences existing surfaces.

The demo reproduces from a fresh clone with no network and no live browser. It
asserts **behavior and gate states**, never subjective quality: it does not claim
an asset "looks good" or "is fun".

## Fixtures

All inputs are fixture-scoped under `examples/asset-pipeline-v1/demo/`:

| Fixture | Stage | Role |
| --- | --- | --- |
| `asset-brief.json` | Generation (#1635) | A licensed `AssetGenerationBrief` for a 32×32 sprite carrying license/provenance. |
| `asset-qa-blocked.json` | Asset-QA (#1636) | An `AssetQaCheck` that is **style-inconsistent** (`styleConsistency: "fail"`). |
| `asset-qa-pass.json` | Asset-QA (#1636) | An `AssetQaCheck` that passes all four dimensions. |

No generated binary assets, runs, or release artifacts are committed.

## Walkthrough

### 1. Generate (proposal-only)

`asset_generation_proposal::generate_asset_proposal` turns `asset-brief.json`
into a `MutationProposal` carrying license/provenance, routed through the existing
review/apply/trust-gradient path. A freshly generated proposal is **proposed /
pending / unverified** — it is never a direct trusted write and never
auto-promoted.

### 2. Asset-QA gate — blocked

`asset_qa_gate::evaluate_asset_qa_check` on `asset-qa-blocked.json` returns
`style-inconsistent`. The gate **fails closed**: a style-inconsistent (or
unlicensed) asset is **not** promotable. Composed into the evaluator's
`declared-gate-and` aggregation, the asset-QA category is `fail`.

### 3. Asset-QA gate — pass — promotion routing

`evaluate_asset_qa_check` on `asset-qa-pass.json` returns `pass`. Only then can the
asset be imported: `asset_import::enforce_asset_qa` (the import promotion gate
from #1637) **requires** a passing asset-QA verdict for the generated asset. The
demo shows the import is **blocked** with no/failing verdict and **promotable**
with the passing verdict — the promotion routes through the asset-QA gate, never
around it.

```text
brief -> generate (proposal-only) -> asset-QA gate
                                       |-- blocked (style-inconsistent) -> not promoted
                                       `-- pass -> import requires the QA pass -> promotable
```

## Reproduce

The deterministic smoke test
`crates/ouroforge-core/tests/asset_generation_qa_demo.rs` asserts the block, the
pass, and the promotion routing:

```bash
cargo test -p ouroforge-core --test asset_generation_qa_demo
```

## Governance

- Generation stays proposal-only through the existing review/apply/trust-gradient
  path; never a direct trusted write. Browser/Studio surfaces stay read-only.
- License/provenance and the asset-QA gate are mandatory before any promotion; the
  gate fails closed. No unlicensed/uncredited/unverified-style asset is promoted.
- Additive and backward-compatible; fixture-scoped; no generated artifacts
  committed; conservative wording (no auto-merge/quality/fun/production/Godot
  claim).
- Rust/local owns the logic; no new language; Elixir NO-GO per ADR #92.

**#1 and #23 remain open.**
