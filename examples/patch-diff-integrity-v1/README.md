# Patch Diff Integrity v1 Fixtures

Focused SMP1.3.1 parser fixtures for unified diff integrity checks.

These fixtures are inert review/test data. They are not patch-apply inputs and do
not authorize source mutation, sandbox execution, merge automation, browser
writes, command bridges, dependency changes, CI changes, or generated tracked
state.

## Valid fixtures

- `valid/two-file-basic.diff` — two ordinary unified diff file sections with
  context, additions, and removals.
- `valid/new-file-basic.diff` — parser-level new-file shape with `/dev/null`
  old path and added lines.

## Malformed fixtures

- `invalid/missing_new_file_header.diff` — file section has `---` but no `+++`
  before the hunk.
- `invalid/hunk_count_mismatch.diff` — hunk header line counts do not match the
  hunk body.
- `invalid/orphan_hunk.diff` — hunk appears before any `diff --git` file
  section.

SMP1.3.1 covers valid unified diff parsing and malformed parser failures only.
Unsafe path, mode, size, generated-target, and critical deletion rejection belong
to later SMP1.3.2 work.
