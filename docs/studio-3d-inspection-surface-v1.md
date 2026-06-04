# Studio 3D Inspection Surface v1

Issue: #607 — Studio 3D Inspection Surface v1. This document records the
no-write, no-command, no-3D-editor boundary for the Studio panels that inspect
3D capability evidence from exported dashboard data.

Studio 3D inspection is a read-only evidence surface for #1 Milestone 9. It is
not a 3D editor, production 3D engine, broad 3D compatibility promise, secure
sandbox, native export path, plugin runtime, hosted/cloud service, visual
scripting system, autonomous launch system, or Godot replacement claim.

## Read-only inspection scope

The Studio cockpit may display these normalized `engine_summaries` read models
when they are present in `dashboard-data.json`:

- `scene3d_hierarchy`: scene graph rows, root/parented counts, and transform
  summaries.
- `scene3d_camera`: active camera id, camera count, projection values, and
  viewport evidence.
- `scene3d_render`: mesh/material refs, renderable rows, visible/skipped counts,
  render fallback reasons, and screenshot artifact refs when exported.
- `scene3d_collision`: contact/trigger counts, collision events, invalid
  collider diagnostics, and bounded collision boundary text.
- `scene3d_animation`: animation state counts, clip/channel rows, and emitted
  animation evidence.
- `scene3d_probe`: scene kind, node/camera/animation counts, and runtime-probe
  status.
- `scene3d_scenario_verdicts`: scenario ids, assertion counts, pass/fail status,
  and evaluator verdict evidence.

All values must be escaped before insertion into HTML. Missing, empty,
malformed, hostile, stale, or partial values must render as visible empty states
or warnings instead of executable markup or hidden failure.

## Trusted authority boundary

Rust/local code owns trusted validation, source-like fixture validation,
persistence, generated evidence artifact writing, project/run binding, dashboard
export shape, and CLI behavior. Browser Studio code may display evidence and
copyable inert text only.

Studio 3D inspection must not:

- write source, scene, project, tilemap, asset, dashboard, run, or evidence
  files;
- persist trusted browser state, viewport manipulation, camera edits, scene graph
  edits, timeline edits, visual-scripting state, or local tool state;
- execute commands, open a local server bridge, call a browser trusted-write API,
  rerun tests, apply mutations, promote regressions, merge branches, auto-apply,
  auto-merge, self-approve, publish, export, package, sign, or deploy;
- import broad 3D formats, fetch remote assets, run plugins, load third-party
  code, claim PBR/material-graph/advanced-lighting support, or claim skeletal
  authoring support;
- claim production-ready 3D, shipped-game maturity, broad engine parity, secure
  sandboxing, native export readiness, hosted/cloud support, or Godot replacement
  status.

Generated 3D screenshots, run directories, preview outputs, dashboard exports,
temp projects, browser profiles, and local tool outputs remain ignored local
generated state unless a later issue explicitly scopes a tiny deterministic
source-like fixture.

## Audit evidence for #607

The 3D Studio inspection implementation is split into small PR units:

1. `3D9.12.1` normalized the dashboard read-model fields.
2. `3D9.12.2` added escaped read-only Studio panels and smoke tests.
3. `3D9.12.3` documents this boundary and adds documentation/wording/generated-
   state audit evidence.

Before closing #607, evidence should confirm:

- #607 is still tied to #1 Milestone 9 and #1 remains open;
- #23 remains open as the memory/governance anchor;
- the issue-specific PRs merged with post-merge verification;
- `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, dashboard/cockpit Node syntax and smoke tests, and `git diff --check` passed;
- `git status --short --ignored` showed only expected ignored generated state
  such as `target/`;
- no dependency, CI, workflow, build-script, browser trusted-write, command
  bridge, viewport persistence, visual scripting, production/Godot-replacement,
  native export, plugin runtime, hosted/cloud, auto-apply, or auto-merge scope was
  added.

## Verification commands

```bash
gh issue view 607 --repo shaun0927/Ouroforge
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

The closure comment for #607 should reference the merged PRs and the post-merge
verification logs, then close only #607. It must not close or modify #1 or #23.
