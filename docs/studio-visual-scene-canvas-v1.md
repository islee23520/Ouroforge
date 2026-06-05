# Studio Visual Scene Canvas v1

Issue: #763
Roadmap anchor: #1 (Milestone: Full Studio Editor v1 — integrated local
authoring UX foundation, not full Godot editor parity).
Status: read-only visual preview plus draft-only transform operations; no apply
behavior, not full WYSIWYG/Godot parity.

The visual scene canvas renders a 2D preview of exported scene/runtime data and
lets transform interactions produce **draft-only** operations. It never writes
trusted scene files, runs commands, or applies edits. Trusted apply must go
through the Safe Source Apply review gates.

## What it shows

- A read-only SVG preview of the scene at the exported canvas size.
- Optional grid lines with a snap indicator (`grid.size`, `grid.snap`).
- The selected entity highlighted with an emphasized stroke.
- Authored state drawn solid; runtime state drawn as a dashed ghost so the two
  are visually distinguished.
- A node legend listing position, rotation, and scale per node.

## Draft transform generation

`studioCanvasTransformDraft(target, transform)` validates a proposed transform
and returns a draft with `set_component_field` operations for `transform.position`
(finite `[x, y]`), `transform.rotation` (finite number), and `transform.scale`
(finite `[x, y]`). It fails closed (`validationStatus: "blocked"`, no operations)
for non-finite values, malformed tuples, no supported transform, or
non-allowlisted target paths (absolute, `..` traversal, shell metacharacters).
Every produced draft carries `requiresSafeSourceApplyHandoff: true` and
`applyCapability: false`.

## Boundary

- **No trusted write / no command / no apply / no merge.** The canvas is pure
  display of escaped exported JSON plus draft-only operations. There are no apply
  buttons, command runners, or trusted-write controls.
- Rust/local trusted code owns validation, persistence, source-apply handoff, and
  evidence writing.
- No claim of full WYSIWYG editor, production-ready editor, Godot replacement,
  secure sandbox, or full Godot editor parity.
- Governance issues #1 and #23 remain open.
