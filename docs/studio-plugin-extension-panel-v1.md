# Studio Plugin / Extension Panel Integration v1

Issue: #768
Roadmap anchor: #1 (Milestone: Full Studio Editor v1 — integrated local
authoring UX foundation, not full Godot editor parity).
Status: read-only plugin registry and descriptor inspection; no plugin
execution.

The plugin / extension panel integrates Plugin / Extension System registry and
descriptor panels into the Full Studio Editor **read-only**, without plugin
execution. It reuses Plugin / Extension System descriptor validation and keeps
Studio rendering allowlisted and read-only.

## What it shows

Per plugin: registry id/name, validation status, compatibility, capabilities,
extension points, and blocked reasons. Each plugin may contribute read-only
panels.

Descriptor-contributed panels are rendered using only allowlisted descriptor
types (`info-panel`, `metadata-panel`, `readonly-table`, `key-value-panel`,
`text-panel`). Non-allowlisted descriptor types are shown as blocked. All
descriptor-provided content is rendered as escaped text, so descriptor content
cannot become arbitrary HTML/JS execution.

## Boundary

- **No plugin execution / install / update / delete / enable / run.** The panel
  is pure display of escaped exported JSON. There are no install/update/delete/
  enable/run controls, and no install/update/run/enable functions are exported.
- No marketplace, network install/update, or executable plugin runtime.
- Rust/local trusted code owns plugin descriptor validation and trusted file
  boundaries.
- No claim of production-ready editor, Godot replacement, secure sandbox, or full
  Godot editor parity.
- Governance issues #1 and #23 remain open.
