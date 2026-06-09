# Import Verification and Fidelity Report Demo v1

This Era O M92 demo reuses `ImportVerificationReport` and `ouroforge migration verify-demo` to produce a local fidelity/coverage summary for the Godot source-text sample project.

## Run

```bash
examples/godot-2d-adapter-v1/import-verification-demo/run-demo.sh
```

The script performs two bounded local steps:

1. `cargo run -p ouroforge-cli -- migration verify-demo --project examples/godot-2d-adapter-v1/sample-project --output examples/godot-2d-adapter-v1/generated/import-verification-report.json`
2. `cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers 2 || true`

Generated reports and run evidence stay under ignored generated/run state. The committed fixture `examples/godot-2d-adapter-v1/import-verification-demo/demo-summary.fixture.json` records the expected demo shape.

## What the demo proves

- The source project is Godot source text and imports as a best-effort declarative skeleton.
- The report records `openchrome-local-skeleton-smoke` verification evidence for the imported skeleton shape.
- Fidelity is honest: clean, flagged, and re-derive counts are all visible.
- Asset provenance records `origin=godot`; license status is explicit rather than silently clean.
- `claimed_ported_units` is empty, and oracle-missing records do not allow port claims.
- Deterministic state hashes are required and validated.
- Logic is not translated or claimed complete; behavior-bearing units remain Era R clean-room re-derivation tasks until captured oracle evidence passes.
- No Elixir/Phoenix trusted write path or artifact semantics authority is introduced.
- #1 and #23 remain open.

## Non-goals

No finished-game auto-port, no foreign runtime bridge, no embedded Godot runtime, no shipped-build ripping, no decompiled-code copying, and no fun/feel or release go/no-go automation.
