# Local Studio + Kernel Packaging and Install UX v1

Issue #2096 implements the first local packaging/install UX for the M86 Studio delivery line. The package is a local single-user developer bundle made from the Phoenix/OTP Studio control/presentation plane plus the Rust `ouroforge` CLI/kernel data plane. It is not a hosted service, collaboration feature, installer, updater, app-store artifact, signing path, deploy path, release channel, or new write authority.

## Install from a Fresh Checkout

```bash
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-target}
cargo build --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1)
[ -n "$STU" ] && (cd "$(dirname "$STU")" && mix deps.get && mix compile)
```

## Run UX

The CLI fallback is canonical and remains sufficient for a zero-human autonomous loop:

```bash
cargo run -p ouroforge-cli -- loop status
```

The local Studio can be started as an optional local control/presentation process after the Mix app compiles:

```bash
cd studio/executor
mix run --no-halt
```

The Studio may render Rust-owned read models and capture human proposals, constraints, directives, corrections, amendments, takeover/handback records, and review evidence. Every write-affecting action remains read + gated-write through existing Rust-owned review/apply, scene/source-apply, evaluator, evidence/provenance, and related gate families. Elixir/Phoenix never owns artifact semantics and never writes trusted artifacts directly.

## Built-Artifact Smoke

After the build commands complete, run:

```bash
scripts/studio-local-package-smoke.sh
```

The smoke checks that the Rust `ouroforge` binary and compiled Studio BEAM app exist, then writes generated evidence under `runs/studio-local-package-smoke-v1/smoke.json`. That generated file is smoke evidence only; it is not a trusted source artifact, release artifact, package store, ledger append, or bypass.

## Boundary Assertions

- Agent-first default preserved: no human is required for the autonomous loop.
- Local-first and single-user only; hosted/multi-user/collaborative Studio remains Layer-3 DEFER.
- Rust remains the data plane for truth, validation, determinism, evidence, provenance, and writes.
- Elixir/OTP + Phoenix LiveView remains the local control/presentation plane.
- No command bridge, new data store, raw bypass, trusted Elixir write, deploy, publish, signing, updater, installer, app-store path, or release channel.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- #1 and #23 remain open.
