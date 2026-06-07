# Asset Replay-Proof and Provenance Binding v1 (#1614)

Part of Evidence-Native Marketplace v1 (#1612) under #1 Era F Milestone 33, building
on the Local Verifiable-Asset Registry (#1613).

## What this adds

Each published marketplace asset is bound to two pieces of existing evidence:

1. A **deterministic replay proof** — the provenance bundle's `replayInputs`,
   re-run on consume with the existing `evaluate_run` evaluator via
   `provenance_replay.rs` (#1502).
2. A **provenance lineage** — the Milestone 25 provenance bundle
   (`provenance_bundle.rs`, #1500), traversed in order and re-checked for
   tampering with its existing digest-bound evaluation.

`verify_asset_proof` reuses both surfaces; it is **not** a new provenance engine
and re-runs no command of its own. It reuses `evaluate_run` for the replay and
the digest-bound bundle evaluation for tamper detection.

## On consume

- **Re-runs the proof to verify the asset works**: the bound replay proof is
  reconstructed in a caller-provided workspace and re-evaluated. Only a
  `reproduced` verdict yields `verified`.
- **Detects tampering**: a mutated provenance ref fails its `expectedDigest`
  check (reported `stale`), a missing ref is `dangling`, and a diverged re-run
  is `replay-diverged` — each is a non-verified, fail-closed status, never a
  silent pass.
- **Traverses provenance lineage**: the eight ordered chain links are reported
  with their resolved state; the lineage is traceable only when every link is
  `present`.

## Boundaries and guardrails

- Owned by Rust/local. Browser/Studio surfaces stay read-only. The behavior is
  additive and backward-compatible with `evidence-marketplace-registry-v1`.
- A verified asset is still adopted only through the existing
  review/apply/trust-gradient path, never a direct trusted write.
- No autonomous apply, auto-merge, self-approval, or reviewer bypass.
- No hosted, paid, or network capability; the marketplace transaction layer
  stays Layer-3 (DEFER per #1508; distributed/Elixir NO-GO per ADR #92).
- This rung makes no claim of replacing or matching Godot, of a production
  engine, or that generated assets are good, fun, or shippable.
- Generated replay outputs remain untracked unless fixture-scoped.

## Governance

- #1 remains open.
- #23 remains open.
- Downstream Evidence-Native Marketplace rungs (sign-off/export/dashboard/
  governance) are unchanged by this binding.
