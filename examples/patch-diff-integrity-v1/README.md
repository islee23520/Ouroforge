# Patch Diff Integrity v1 Fixtures

Focused parser, rejection, and read-model fixtures for unified diff integrity checks.

These fixtures are inert review/test data. They are not patch-apply inputs and do
not authorize source mutation, sandbox execution, merge automation, browser
writes, command bridges, dependency changes, CI changes, or generated tracked
state.

## Valid fixtures

- `valid/two-file-basic.diff` — two ordinary unified diff file sections with
  context, additions, and removals.
- `valid/new-file-basic.diff` — parser-level new-file shape with `/dev/null`
  old path and added lines.

## Validation output

The implemented report and preview-validation read-model shapes are documented in
`docs/patch-diff-integrity-v1.md`. Validation is read-only: it may return
`passed` or `blocked`, but it never applies a patch, runs a sandbox, writes
trusted source files, merges branches, or executes commands.

## Malformed fixtures

- `invalid/missing_new_file_header.diff` — file section has `---` but no `+++`
  before the hunk.
- `invalid/hunk_count_mismatch.diff` — hunk header line counts do not match the
  hunk body.
- `invalid/orphan_hunk.diff` — hunk appears before any `diff --git` file
  section.

## Unsafe fixtures

- `unsafe/generated-target.diff` — generated/local root target that must block before preview.
- `unsafe/traversal-target.diff` — parent-dir target that must block before preview.
- `unsafe/binary-target.diff` — binary patch marker that must block before preview.
- `unsafe/mode-change.diff` — file mode metadata that must block before preview.
- `unsafe/critical-delete.diff` — critical file deletion that must block before preview.

SMP1.3.1 covered valid unified diff parsing and malformed parser failures.
SMP1.3.2 added unsafe path, mode, size, generated-target, and critical deletion
rejection. SMP1.3.3 documents the validation/read-model output shape without
relaxing file class guardrails or adding source patch apply.
