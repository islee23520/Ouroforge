# Full Studio Editor Integrated Demo v1

Issue: #774

This fixture is a local static demo read model for the Full Studio Editor v1
sequence. It composes project overview, scene tree/entity inspection, visual
canvas, asset browser, scenario/playtest evidence, export/package inspection,
plugin descriptor inspection, draft edit preview, Safe Source Apply handoff
preview, and the read-only command palette.

Boundary: the demo is fixture-scoped and evidence-backed. It does **not** write
trusted source files from the browser, execute commands, publish, deploy, sign,
upload, install or execute plugins, auto-apply, auto-merge, self-approve, or
claim full Godot parity / replacement / production readiness.

Run from the repository root:

```bash
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Generated local workspace state remains ignored (`runs/`, `target/`, `.omx/`,
`.omc/`, `.openchrome/`). This fixture is intentionally small and tracked so the
integrated smoke can remain deterministic.
