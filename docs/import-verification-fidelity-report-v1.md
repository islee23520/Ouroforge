# Import Verification and Fidelity Report v1

Era O M92 verifies the imported Godot source-text skeleton and emits an honest fidelity report. This is a one-way on-ramp report, not engine absorption, a live bridge, or a finished-game auto-port.

## Rust-owned data shapes

All artifact truth lives in `crates/ouroforge-core`:

| Shape | Code location | Purpose |
| --- | --- | --- |
| Godot IR nodes | `crates/ouroforge-core/src/godot_2d_adapter_ir.rs::GodotIrNode` | Source-project/open-text `.tscn` / `.tres` / `project.godot` skeleton facts, provenance, and adapter fidelity. |
| Mapping records | `crates/ouroforge-core/src/ir_mapping_fidelity_classifier.rs::MappingRecord` | Ouroforge-native candidate skeleton mapping, green/yellow/red fidelity, and explicit gap reasons. |
| Behavioral units | `crates/ouroforge-core/src/logic_touchpoint_handoff.rs::LogicBehavioralUnitRecord` | Behavior-bearing scripts/signals/input/physics/rendering touchpoints routed to Era R. |
| Oracle records | `crates/ouroforge-core/src/import_verification_report.rs::ImportOracleRecord` | Captured-oracle requirements and the fail-closed no-port-claim rule. |
| Composed report | `crates/ouroforge-core/src/import_verification_report.rs::ImportVerificationReport` | Skeleton verification evidence, provenance, asset license status, fidelity totals, re-derivation tasks, and deterministic report hash. |

The CLI entrypoint is `ouroforge migration verify-demo`, implemented in `crates/ouroforge-cli/src/main.rs`. It writes generated report JSON under `examples/godot-2d-adapter-v1/generated/` by default.

## Verification evidence

The report records `openchrome-local-skeleton-smoke` evidence: a read-only imported skeleton smoke that checks scene/entity/asset/input shape and deterministic state hashes. Perceptual render evidence is secondary; logic behavior remains Era R re-derivation work until an Ouroforge-native oracle passes.

## Fidelity rules

- `clean` / Green: declarative skeleton facts mapped without known behavior or unsupported gaps.
- `flagged` / Yellow: partial or metadata-only skeleton facts that remain visible in the report.
- `rederive` / Red: behavior-bearing or unsupported source-engine facts that must become Era R tasks.
- `claimed_ported_units` must stay empty until captured oracle evidence passes.
- Source/decompiled code is never copied or translated; only source-project/open-text formats are accepted.
- Asset provenance records `origin=godot` and license status; missing explicit license remains `source-project-provenance-recorded-license-unverified` rather than silently clean.
- #1 and #23 remain open governance anchors.

## Verification

```bash
cargo fmt --all
cargo build --workspace --jobs 2
cargo test -p ouroforge-core -p ouroforge-evaluator --jobs 2
cargo run -p ouroforge-cli -- migration verify-demo --project examples/godot-2d-adapter-v1/sample-project --output examples/godot-2d-adapter-v1/generated/import-verification-report.json
cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers 2 || true
```
