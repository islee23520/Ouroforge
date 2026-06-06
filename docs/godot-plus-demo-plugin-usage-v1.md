# Godot-Plus Demo Plugin / Extension Usage v1

Issue: #792
Status: **GPD12.15 plugin-descriptor contract.** This document records the declarative Plugin / Extension System v1 usage for the Godot-Plus Demonstration Game v1 vertical slice (Signal Gate / Collect and Exit), on the canonical fixture `examples/playable-demo-v2/collect-and-exit/`. #1 and #23 remain open.

## Descriptor set

The demo uses three inert local plugin manifests under `examples/playable-demo-v2/collect-and-exit/plugins/`:

| Plugin | Capability | Extension point | Purpose |
| --- | --- | --- | --- |
| `collect-and-exit-dashboard-panel` | `dashboardPanel` | `dashboard.panels.readOnly` | Read-only dashboard metadata for plugin registry display. |
| `collect-and-exit-scenario-template` | `scenarioTemplate` | `scenario.templates.readOnly` | Read-only scenario template metadata for the trusted collect-and-exit route smoke. |
| `collect-and-exit-asset-metadata` | `assetMetadataProvider` | `assets.metadata.readOnly` | Read-only asset metadata labels for checked-in demo assets. |

`plugin-usage-evidence.json` is fixture-scoped evidence tying those descriptors to the demo registry/evidence story. It is intentionally source-like fixture evidence, not generated runtime output.

## Boundaries

The descriptors are declarative metadata only. They add no executable plugin runtime, JavaScript/native extension loading, marketplace, network install/update, dependency install, command bridge, direct trusted source write, auto-apply, auto-merge, publish/deploy/sign/upload, native/mobile/store export, public release, Godot replacement, full parity, or production-ready claim.

Dashboard and Studio surfaces may render the descriptor metadata as escaped read-only text; trusted Rust/local validation owns manifest parsing, registry discovery, and evidence validation.

## Verification

```bash
node --check examples/playable-demo-v2/collect-and-exit/plugin-usage-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/plugin-usage-smoke.test.cjs
cargo test -p ouroforge-core --test godot_plus_demo_plugin_usage_contract -- --test-threads=1
cargo run -p ouroforge-cli -- plugin validate examples/playable-demo-v2/collect-and-exit/plugins
```

Generated plugin registry outputs remain local and ignored unless explicitly fixture-scoped. #1 and #23 remain open.
