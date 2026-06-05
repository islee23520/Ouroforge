# Studio Entity / Component Inspector v1

Issue: #760
Roadmap anchor: #1 (Milestone: Full Studio Editor v1 — integrated local
authoring UX foundation, not full Godot editor parity).
Status: read-only inspection plus draft-only primitive field editing; no apply
behavior.

The entity/component inspector lets a Studio user inspect the primitive fields of
a selected entity's components **read-only** and generate **draft-only** edit
operations for valid primitive field changes. It renders an exported,
Rust-produced read model only; it never writes trusted component files, executes
commands, merges, self-approves, or applies edits. Any trusted write must go
through the Safe Source Apply review gates outside the browser.

## What it shows

- Entities with a read-only selection indicator.
- Each component's primitive fields with type, current value, and an
  editable/read-only/blocked status.
- Supported primitive field types: `string`, `number`, `boolean`, `enum`, and
  `vector` (a tuple of finite numbers). Fields of other types, fields marked
  read-only, and fields flagged unsafe are displayed but blocked from draft
  editing.

## Draft edit generation

`entityComponentDraftEdit(target, field, newValue)` validates a proposed
primitive field change and returns a neutral draft operation
(`set_component_field`) with `validationStatus: "validated"` when the value is
type-valid, the field is editable, and the target path is an allowlisted
in-project source path. The draft is shaped so it can be consumed by the Studio
draft authoring model (`studio_draft_authoring`).

Validation fails closed (`validationStatus: "blocked"`, no proposed operations)
for:

- Unsupported field types (e.g. references, scripts, objects).
- Read-only or unsafe-flagged fields.
- Type-invalid values (non-finite numbers, non-boolean booleans, out-of-set
  enum values, non-numeric vector tuples, non-string strings).
- Target paths that are absolute, contain `..` traversal, or contain shell
  metacharacters.

Every produced draft carries `requiresSafeSourceApplyHandoff: true` and
`applyCapability: false`.

## Boundary

- **No trusted write / no command / no apply / no merge.** The surface is pure
  display of escaped exported JSON plus draft-only operations. Inputs are
  disabled and copy-only; there are no apply buttons, command runners, or
  trusted-write controls.
- Rust/local trusted code owns validation, persistence, source-apply handoff,
  evidence writing, and trusted file boundaries.
- TypeScript/JavaScript owns read-only inspection, draft interaction, and
  draft-only browser state.
- No claim of production-ready editor, Godot replacement, secure sandbox, full
  Godot editor parity, or production-ready collaborative editor.
- Governance issues #1 and #23 remain open.
