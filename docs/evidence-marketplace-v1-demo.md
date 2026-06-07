# Evidence-Native Marketplace Demo v1

Issue: **#1615** (Era F Milestone 33 demo). Part of Evidence-Native Marketplace
v1 (#1612); builds on the Local Verifiable-Asset Registry v1 (#1613).

This demo shows, deterministically and from a fresh clone, that a verifiable
asset can be **published**, **consumed**, and **verified locally** — its proof
re-runs to confirm the asset works — and that a **tampered asset is rejected**
on verification. It runs with no network and no live browser; all evidence is
fixture-scoped under `examples/evidence-marketplace-v1/demo/`.

## What the demo shows

1. **Publish.** A `collect-and-exit` grid-puzzle template is published into a
   local `LocalAssetRegistry` together with its acceptance suite, deterministic
   replay proof, and a complete Milestone 25 provenance lineage
   (`provenance_bundle.rs`, #1500).
2. **Consume.** The asset is consumed by id; the registry re-verifies its shape
   (acceptance suite + replay proof + complete provenance) fail-closed.
3. **Verify (proof re-runs).** On consume, the asset's proof re-runs locally:
   the recorded template digest is recomputed from the on-disk template and must
   match (a tamper check), and the bound provenance bundle is re-evaluated
   against the local evidence root and must resolve to a **complete** chain.
   When both hold, the asset **reproduces** and is verified.
4. **Tamper rejection.** A tampered asset whose recorded template digest no
   longer matches its on-disk template is **rejected** on verification, even
   though its shape is intact. Verification trusts the re-run, not the claim.

## Reproduce

```bash
cargo test -p ouroforge-core --test evidence_marketplace_demo_contract
```

The smoke test asserts verify-on-consume for the valid asset and tamper
rejection for the tampered asset. It is deterministic and reuses the existing
registry (#1613) and provenance bundle (#1500) surfaces — no new engine,
runtime, or writer.

## Fixtures

- `demo/asset.valid.fixture.json` — a valid published asset; verifies on consume.
- `demo/asset.tampered.fixture.json` — a tampered asset (template digest does not
  match the published template); rejected on verification.
- `demo/refs/` — the template, acceptance suite, and the eight provenance
  chain-link evidence files the bundle references.

## Boundaries and governance

- **Local only.** No hosted, paid, or network marketplace capability. Any
  transaction layer or take-rate stays DEFER until a #1508 Layer-3 GO (Elixir
  NO-GO per ADR #92).
- **Proposal-only.** Adoption of a consumed asset flows through the existing
  review/apply/trust-gradient path, never a direct trusted write; browser/Studio
  surfaces stay read-only.
- **Additive and backward-compatible.** New fixtures and a smoke test only; no
  change to existing contracts.
- Generated state remains untracked unless explicitly fixture-scoped; the demo
  evidence here is fixture-scoped.
- Conservative wording: this demonstrates a local verification workflow over
  fixture evidence only. It makes no production, quality, or Godot-comparison
  claim, and does not assert that generated games are good, fun, or shippable.

**#1 remains open. #23 remains open.**
