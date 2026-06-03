# Playable Demo v2: Collect and Exit

This source fixture is the first implementation slice for issue #319. It defines
a deterministic one-screen project with a player, key trigger, door/exit trigger,
minimal HUD values, animation metadata, audio intent metadata, a Seed, and a
scenario pack.

The fixture is intentionally source-like only. Do not commit generated `runs/`,
`dashboard-data/`, screenshots, or temporary smoke outputs for this demo.

Suggested validation from a full Rust toolchain:

```bash
cargo run -p ouroforge-cli -- project validate examples/playable-demo-v2/collect-and-exit/ouroforge.project.json
cargo run -p ouroforge-cli -- seed validate examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml
```

## Evidence smoke

PR EE2.8.3 adds a source-controlled smoke harness that drives the runtime through
key collection and exit completion, evaluates the scenario pack against bounded
evidence paths, writes temporary evidence outside the repository, and removes it
before exit:

```bash
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
```

The smoke intentionally leaves `runs/`, `dashboard-data/`, screenshots, and other
generated artifacts untracked.

Canonical documentation: `docs/playable-demo-v2-collect-and-exit.md`.
