# Production 2D Studio Inspection Surface v1

Issue: #593 — Studio 2D Engine Inspection Surface v1.

This document records the P2D8.13.3 boundary and generated-state audit for the
Production 2D read-only Studio/cockpit inspection surfaces. It documents the
state after the read-model normalization PR and Studio panel PR; it does not add
a trusted writer, command bridge, production editor, hosted service, native
export path, plugin runtime, visual scripting system, or current Godot
replacement claim.

## Display contract

The cockpit may render Rust/local exported Production 2D evidence from
`dashboard-data.json` as escaped, read-only HTML:

- renderer/layer/camera state;
- physics/collision/contact state;
- input/action/replay state;
- animation, VFX, audio, asset-loading, and performance evidence;
- save/load snapshots, runtime state digests, and replay digest comparisons;
- profiler/frame-budget summaries;
- copyable command text that remains inert display-only text.

Every browser-rendered field is treated as untrusted display input. Missing,
malformed, stale, or hostile data must be visible as empty/warning text without
crashing, and must be escaped before insertion into markup.

## Authority boundary

Rust/local workflows own trusted validation, generated evidence writing,
scenario execution, run/project binding, save/load persistence, replay digest
comparison, source-like fixture validation, and CLI behavior. Browser dashboard
and Studio surfaces only inspect already-exported evidence unless a later issue
explicitly scopes a Rust/local trusted API for persistence.

The Production 2D Studio inspection surfaces must not:

- write source, scene, tilemap, asset, save, project, dashboard, run, or evidence
  files from browser JavaScript;
- persist trusted state through browser storage, native file APIs, hidden local
  servers, or command bridges;
- execute, rerun, schedule, merge, apply, auto-accept, or auto-promote commands;
- mutate saves or replay baselines;
- claim to be a production editor, visual scripting system, native export path,
  plugin runtime, hosted/cloud Studio, public launch surface, production-ready
  engine, secure sandbox, broad compatibility-stable API, shipped-game proof, or
  Godot replacement.

Copyable command strings are reproducibility hints for a human terminal/Rust CLI
flow. They are not buttons, browser authority, or implicit approval.

## Generated-state audit

P2D8.13.3 adds documentation/tests only. It intentionally does not track new
`runs/`, dashboard exports, screenshots, temp projects, browser profiles,
`.omx/`, `.omc/`, `.claude/`, `.openchrome/`, or build output. Generated local
state remains ignored unless a later issue explicitly scopes a tiny deterministic
source-like fixture.

Reviewers should verify generated-state isolation with:

```bash
git status --short --ignored
```

Expected ignored local-state categories include `runs/`, `target/`, `.omx/`,
`.omc/`, `.claude/`, and `.openchrome/` when present. They must remain untracked
for #593 closure evidence.

## Wording audit

P2D8.13.3 keeps public language conservative:

- Production 2D means a bounded Milestone 8 engineering target for a small 2D
  vertical-slice game class, not a public maturity or shipped-game claim.
- Studio/cockpit inspection is local-first and read-only; it is not a hosted
  Studio, production editor, release pipeline, marketplace, plugin runtime,
  native export flow, or public launch feature.
- #1 remains open as the broad roadmap/vision anchor.
- #23 remains open as the repo-memory/design context anchor.

## Verification evidence expected for #593 closure

Before closing #593, record evidence that:

- #593, #1, and #23 were re-checked live before closure;
- `cargo fmt --check`, `cargo test`, and
  `cargo clippy --all-targets --all-features -- -D warnings` passed;
- dashboard and cockpit JavaScript syntax/tests passed;
- `git diff --check` passed;
- `git status --short --ignored` showed no tracked generated-state drift;
- no browser trusted write API, command execution API, command bridge, or local
  server bridge was introduced;
- final issue evidence states #1/#23 remain open.
