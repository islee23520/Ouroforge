# Godot-Plus Demo Project Scaffold v1

Issue: #781
Status: **GPD12.4.1–3 scaffold contract.** This document records the project
scaffold for the Godot-Plus Demonstration Game v1 vertical slice. The scaffold
reuses the canonical fixture `examples/playable-demo-v2/collect-and-exit/`; it
does not create a parallel demo tree and does not implement the gameplay loop.
The legacy `examples/godot-plus-demo-v1/` tree is superseded and is not used.

This issue adds the export/plugin placeholders and the generated-state audit that
the GDD (`docs/godot-plus-demo-gdd-v1.md`) and acceptance matrix
(`docs/godot-plus-demo-acceptance-criteria-v1.md`) require. No production release,
native/store export, executable plugin runtime, marketplace, or trusted browser
write is introduced. #1 and #23 remain open.

## Scaffold layout

The collect-and-exit project already declares its manifest, scene, Seed,
scenario pack, asset manifest, and assets. This scaffold issue adds:

| Path | Purpose | Contract |
| --- | --- | --- |
| `ouroforge.project.json` | Project manifest (existing) | `project-manifest-v1`; declares scenes, seeds, scenario packs, asset roots, runs root, and generated roots. |
| `export/export-profile.json` | Local web export **placeholder** | `export-profile-v1`; `web-local` target, `dist/` staging output, demo verification scenario ids. Actual package verification is owned by #791. |
| `export/package-metadata.json` | Export package metadata **placeholder** | `export-package-metadata-v1`; local package descriptor source for #791. |
| `plugins/collect-and-exit-dashboard-panel/ouroforge.plugin.json` | Inert read-only plugin descriptor **placeholder** | `ouroforge.plugin-manifest.v1`; `dashboardPanel` capability, no executable code. Actual descriptor usage is owned by #792. |
| `scaffold-audit.test.cjs` | Generated-state audit smoke | Read-only Node audit; asserts scaffold files parse and no generated roots are tracked. |

Gameplay config and scenario config are the existing scene
(`scenes/collect-and-exit.scene.json`) `gameplayRules`/`componentDefaults` and the
scenario pack (`scenarios/collect-and-exit.json`); the scaffold does not duplicate
them.

## Generated-state boundary

The project manifest declares generated roots `runs`, `target`, and
`dashboard-data`. These plus `dist/` (export staging) are never committed inside
the fixture. The repository `.gitignore` ignores top-level `/runs/`, `/target/`,
and `/dashboard-data/`; nested generated output is written outside the repository
by the demo smokes and removed before exit. `scaffold-audit.test.cjs` fails closed
if any generated root appears inside the fixture.

## Verification

Rust-trusted validation (the scaffold contracts are owned by Rust):

```bash
cargo test -p ouroforge-core --test godot_plus_demo_scaffold_contract
cargo run -p ouroforge-cli -- project validate examples/playable-demo-v2/collect-and-exit/ouroforge.project.json
cargo run -p ouroforge-cli -- asset validate examples/playable-demo-v2/collect-and-exit/asset-manifest.json
cargo run -p ouroforge-cli -- plugin validate examples/playable-demo-v2/collect-and-exit/plugins
```

Generated-state audit smoke:

```bash
node examples/playable-demo-v2/collect-and-exit/scaffold-audit.test.cjs
```

## Boundaries

This scaffold does not authorize gameplay implementation, generated output
tracking, production release, native/mobile/console/store export, signing,
publishing, deployment, executable plugin runtime, marketplace, network plugin
install/update, trusted Studio/browser writes, command bridges, or any full Godot
replacement / full Godot parity / production-ready / commercial-release claim.
#1 and #23 remain open.
