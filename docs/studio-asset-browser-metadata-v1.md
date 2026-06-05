# Studio Asset Browser and Metadata Inspector v1

Issue: #764
Roadmap anchor: #1 (Milestone: Full Studio Editor v1 — integrated local
authoring UX foundation, not full Godot editor parity).
Status: read-only asset manifest and plugin-metadata inspection; no import,
generation, upload, or mutation.

The asset browser lets Studio inspect asset manifests and plugin-provided
metadata descriptors **read-only**. It reuses Build / Export / Packaging asset
manifest data and Plugin / Extension System metadata descriptors as inputs. It
never imports, generates, uploads, fetches remote assets, writes manifests, runs
commands, or applies edits.

## What it shows

- An asset list/grid with id, type, source path, output path, hash, size, and
  status.
- Read-only filtering/search via `filterStudioAssets(assets, { type, query })`.
- Diagnostics: missing output artifacts, duplicate asset hashes, duplicate asset
  ids, and unsafe source/output paths.
- A metadata inspector that renders plugin-provided descriptors read-only. Only
  allowlisted descriptor types (`asset-metadata`, `texture-metadata`,
  `audio-metadata`, `model-metadata`, `generic-metadata`) are rendered; other
  descriptor types are shown as blocked.

## Boundary

- **No import / generation / upload / mutation / command / network.** The surface
  is pure display of escaped exported JSON. There are no import buttons, file
  pickers, upload controls, or asset generators, and no import/upload/generate
  functions are exported.
- Rust/local trusted code owns asset manifest production, validation, and trusted
  file boundaries.
- No claim of full asset import pipeline, production-ready editor, Godot
  replacement, or full Godot editor parity.
- Governance issues #1 and #23 remain open.
