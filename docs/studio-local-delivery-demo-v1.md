# Studio Packaging and Local Delivery Demo v1

Issue #2097 demonstrates the M86 local Studio delivery path end to end. The demo composes the local packaging/install manifest, a Rust-owned generated smoke evidence read model, and one optional human packaging constraint captured through the existing Studio intervention panels.

## Demo Flow

1. Build the Rust workspace and compile the local Studio Mix app.
2. Run `scripts/studio-local-package-smoke.sh` to check the built `ouroforge` binary and compiled Studio BEAM app.
3. Render the generated smoke evidence in the local Studio shell as a Rust-owned read model.
4. Capture an optional human packaging constraint as intervention-as-evidence.
5. Route the constraint to the existing Rust human-constraint validation gate; do not write trusted artifacts from Elixir.
6. Prove the autonomous fallback completes through CLI commands with no Studio and no human.

## Verification

```bash
cargo build --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1)
[ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test --only demo)
```

## Invariants Demonstrated

- Studio delivery remains read + gated-write: presentation may capture intent, but trusted effects stay behind existing Rust gates.

- Agent-first default: the CLI fallback completes without the human surface.
- Read + gated-write: the human packaging constraint is queued for Rust gates, never applied directly.
- Two-plane: Rust owns data-plane truth; Elixir/Phoenix renders, captures, and routes only.
- Local-first: the demo is single-user and local; hosted/multi-user/collaborative Studio remains Layer-3 DEFER.
- Generated smoke evidence under `runs/` is not a trusted source artifact, release artifact, store, deploy path, or new data store.
- Fun/taste verdict and release go/no-go remain human; #1 and #23 remain open.
