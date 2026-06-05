# Studio Draft Authoring Model v1

Issue: #761
Roadmap anchor: #1 (Milestone: Full Studio Editor v1 — integrated local
authoring UX foundation, not full Godot editor parity).
Status: neutral, reviewable draft operation model; no apply behavior.

The draft authoring model represents Studio edits as **reviewable draft
operations** rather than trusted source writes. It is a neutral data + validation
model that UI controls can produce and that can later be converted into Safe
Source Apply preview input. It never writes trusted files, executes commands,
merges, self-approves, publishes/deploys, or applies edits.

## Supported operations

- `set_component_field` — requires a field path and a value.
- `add_component` — requires a component type.
- `remove_component` — requires a component type.
- `rename_entity` — requires a new name.
- `reorder_child` — requires a child reference and a target index.

## Validation and diagnostics

`validateStudioDraftOperation(operation)` returns `{ kind, valid, reasons }`.
`studioDraftOperationModel(draft)` aggregates per-operation validation, checks the
draft schema version against an allowlist (`visual-edit-draft-v1`), and produces a
review-only preview diff via `studioDraftOperationPreviewDiff`.

Operations fail closed when they:

- Use an unsupported operation kind.
- Use a source-mutation/publish bypass kind (`apply`, `apply_patch`, `merge`,
  `merge_branch`, `write_file`, `write_trusted_file`, `self_approve`,
  `bypass_review_gate`, `execute_command`, `run_command`, `publish`, `deploy`,
  `sign`, `upload`, `install`, `network_install`).
- Contain a command/network field (`command`, `cmd`, `exec`, `shell`, `hook`,
  `script`, `url`, `href`).
- Contain a string with an unsafe path (absolute, `..` traversal, drive-letter)
  or shell metacharacters.
- Omit required fields for their kind.
- Belong to a draft with an unsupported schema version.

A preview diff is produced only when the whole draft validates; otherwise the
diff output states the draft is blocked.

## Boundary

- **No trusted write / no command / no apply / no merge / no publish.** The model
  is data + validation only; rendering is escaped display.
- Rust/local trusted code owns trusted persistence, source-apply handoff, and
  evidence writing. Drafts are suitable for Safe Source Apply preview handoff.
- Draft outputs remain generated/ignored unless explicitly fixture-scoped.
- No claim of production-ready editor, Godot replacement, secure sandbox, or full
  Godot editor parity.
- Governance issues #1 and #23 remain open.
