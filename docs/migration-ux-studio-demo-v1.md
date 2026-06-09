# Migration UX Studio Demo v1

Issue: #2188. Milestone: Era O M94.

This demo proves the Studio migration UX path without widening the on-ramp. It reuses the Rust-owned Godot and Unity source-project adapters, renders the fidelity report through the Studio control/presentation model, and routes unsupported behavior to Era R clean-room re-derivation. It is not engine absorption, not a live bridge, not an embedded runtime, and not a finished-game auto-port.

## Runnable script

```bash
examples/migration-ux-studio-demo/run-demo.sh
```

The script runs:

1. `cargo run -p ouroforge-cli -- migration verify-demo` for the Godot source-text sample project;
2. `cargo run -p ouroforge-cli -- migration unity-demo` for the Unity Force-Text sample project;
3. `cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers 2 || true` to exercise the seed path used by the issue verification block;
4. `mix test test/ouroforge_executor/migration_ux_demo_test.exs` to verify the Studio demo model.

Reports are written under `${TMPDIR:-/tmp}/ouroforge-migration-ux-demo/` so the demo does not create a tracked artifact store.

## Evidence shape

The committed fixture [`examples/migration-ux-studio-demo/demo-summary.fixture.json`](../examples/migration-ux-studio-demo/demo-summary.fixture.json) records the reviewable summary:

- 🟢 one clean declarative skeleton import row;
- 🟡 one best-effort presentation row with a visible caveat;
- 🔴 one behavior row routed to Era R;
- zero claimed ported units;
- source-only / clean-room / one-way boundary;
- no trusted Studio write authority;
- deterministic `sha256:` state-hash evidence.

## Oracle and determinism rule

No row is called `ported` or behavior-equivalent in this demo. A 🔴 logic row remains `ported_claim_allowed=false` until later Era R evidence provides a captured oracle, an Ouroforge-native clean-room re-expression, and passing deterministic verification. For 2D, state hash is bit-exact primary evidence. For 2.5D/3D, deterministic state hash is primary and perceptual render comparison is secondary.

## Studio boundary

Studio/Phoenix renders and routes only. Rust in `crates/ouroforge-core` and `crates/ouroforge-evaluator` owns adapter IR, mapping records, oracle requirements, fidelity grades, state hashes, and any gated writes. Elixir owns no artifact semantics, adds no data store, and performs no trusted write.
