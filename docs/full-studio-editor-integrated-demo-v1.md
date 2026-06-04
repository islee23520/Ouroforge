# Full Studio Editor Integrated Demo v1

Issue: #774
Roadmap anchor: #1 (Full Studio Editor v1). Memory anchor: #23.

## Scope

The integrated demo composes already-bounded Studio v1 surfaces over the tracked
fixture `examples/full-studio-editor-demo-v1/demo.fixture.json`:

- project overview;
- scene tree and entity/component inspector;
- visual scene canvas;
- asset browser and metadata inspector;
- scenario/playtest evidence panel;
- export/package inspection panel;
- plugin/extension descriptor panel;
- draft edit preview;
- Safe Source Apply handoff preview;
- read-only command palette.

## Guardrails

This is a local static, fixture-scoped demonstration. It is not a production
editor, hosted Studio, native desktop editor, executable plugin runtime,
marketplace, command bridge, native export path, Godot replacement, or full Godot
editor parity claim.

The demo does not apply trusted writes, run commands, publish, deploy, sign,
upload, install plugins, execute plugin code, auto-apply, auto-merge,
self-approve, bypass review gates, or mutate CI/workflows/dependencies. Trusted
source mutation remains gated by Safe Source Apply: validated preview, sandbox
evidence, accepted independent review, stale-target checks, rollback metadata,
allowlisted verification, and post-apply comparison.

## Verification

Focused smoke:

```bash
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Required issue gate also includes `cargo fmt --check`, full `cargo test`,
`cargo clippy --all-targets --all-features -- -D warnings`, dashboard smoke,
`git diff --check`, live #774/#1/#23 checks, and final confirmation that #1 and
#23 remain open.
