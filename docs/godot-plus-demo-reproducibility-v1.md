# Godot-Plus Demo Reproducibility v1

Issue: #795  
Status: documentation and reproducibility guide for the bounded Godot-Plus Demo v1 track.

This guide explains how a contributor can reproduce the local demo workflow from committed source-like fixtures and generated local evidence. It documents commands and inspection surfaces only. It does not add gameplay, trusted browser writes, direct Studio source mutation, executable plugin runtime, marketplace behavior, publishing, deployment, native/mobile/console/store export, signing, release automation, credentialed operation, or public visibility changes.

The demo remains a scoped evidence-native agentic workflow for a small local 2D vertical slice. It is not a full Godot replacement, not full Godot parity, not production-ready, not commercial release ready, and not a secure sandbox claim. #1 and #23 remain open.

## How to run locally

Use the committed local fixtures and Rust-owned validation surfaces:

```bash
cargo run -p ouroforge-cli -- project validate examples/playable-demo-v2/collect-and-exit
cargo run -p ouroforge-cli -- scene validate examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
```

For a browser inspection pass, serve the repository locally and open the runtime fixture:

```bash
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

The local server is for static inspection. It does not create a command bridge, write trusted source files, publish a build, install dependencies, or grant credentialed authority.

## How to inspect in Studio

Studio inspection uses the static authoring cockpit and exported read models. The cockpit may render project metadata, scene summaries, evidence links, draft/review/source-apply status, export/package status, plugin descriptor metadata, and known blockers as escaped read-only or draft-only text.

```bash
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Studio walkthrough evidence should show inspect, draft, review, and comparison surfaces without direct Studio trusted source writes, no browser command bridge, no auto-apply, no auto-merge, no self-approval, and no reviewer bypass. Source mutation remains review-gated through Safe Source Apply evidence.

## How to run scenarios and QA

Scenario and QA checks stay deterministic and local:

```bash
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/evidence-read-model-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/asset-evidence-smoke.test.cjs
```

The expected evidence shape links scenario assertions, input/replay context, runtime probe data, HUD/objective state, asset integrity, QA classification, and generated run refs. Generated QA runs, screenshots, temp servers, local browser profiles, and evidence bundles remain ignored unless a later issue explicitly scopes a deterministic fixture.

## How to export a local package

The export/package path is local web package verification only:

```bash
cargo test -p ouroforge-core --test scenario_coverage_v15_build_export_packaging
```

Export evidence may include source fixture refs, package manifest refs, checksum/fingerprint refs, runtime probe compatibility, and local staging policy. It does not authorize no publish, no deploy, no sign, no upload, no native export ready claim, no mobile/console/store export, and no public release automation.

## How to view evidence and journal

Dashboard and journal inspection use exported local read models:

```bash
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
```

Evidence should be linked from committed fixtures or ignored generated roots such as `runs/`, `target/`, `dashboard-data/`, `.openchrome/`, `.omc/`, `.omx/`, `.claude/`, screenshots, browser profiles, temp servers, and package output directories. Do not commit generated demo outputs unless the issue explicitly calls them fixture-scoped.

Agentic iteration evidence should show draft/preview refs, review decisions, source-apply transactions, rollback metadata, rerun comparison, journal entries, and final verification. It must remain review-gated and auditable; it must not become autonomous unreviewed source mutation.

## Known limitations

- The demo is a small local 2D vertical slice, not a production-ready engine/editor.
- The workflow is a scoped evidence-native agentic workflow, not broad superiority over Godot.
- Browser and Studio surfaces remain read-only or draft-only except existing allowlisted runtime interaction.
- Safe Source Apply remains review-gated; there are no direct Studio trusted source writes.
- There is no browser command bridge, local server command bridge, arbitrary shell execution, dependency install, CI/workflow mutation, network install/update, credentialed operation, auto-apply, auto-merge, self-approval, or reviewer bypass.
- Plugin descriptors are inert metadata; there is no executable plugin runtime, no marketplace, no plugin network install/update, no dynamic loading, and no remote asset loading.
- Local export/package evidence is for reproducible local web inspection; there is no publish, no deploy, no signing, no upload, no app-store/Steam/itch publishing, and no commercial release.
- Generated demo outputs, exports, QA runs, evidence, screenshots, videos, temp servers, package bundles, local state, and tool outputs stay ignored unless explicitly fixture-scoped.

## Generated-state policy

Run documentation must never require committing `runs/`, `target/`, `dashboard-data/`, `examples/evidence-dashboard/dashboard-data.json`, `.openchrome/`, `.omc/`, `.omx/`, `.claude/`, browser profiles, screenshots, package outputs, or temp-server state. These paths are local evidence or generated state unless a future issue names a deterministic fixture artifact.

## Governance verification

Closure evidence for this documentation pass should include:

```bash
gh issue view 795 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

The final record must confirm #1 and #23 remain open and that public wording stays conservative: no Godot replacement, no full parity, no production-ready claim, no commercial release readiness, no secure sandbox claim, and no universal superiority claim.
