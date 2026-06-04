# Patch Preview Artifact v1 fixture

This directory contains a tiny deterministic source-like fixture for
`docs/patch-preview-artifact-v1.md`.

- `patch-preview.sample.json` is a data-only patch preview example with an
  artifact hash, target metadata, diff text/summary, evidence refs, required
  tests, rollback expectations, and read-only display prototype.
- The embedded `readModelPrototype` shows the summary a future read-only
  dashboard/Studio surface could display.
- The fixture is not an executable patch, does not write source files, does not
  execute commands, and keeps `sourceMutationApplyStatus` set to `blocked`.

This fixture exists only to validate the schema vocabulary for #358 / SMP1.4.1.
Source mutation apply, sandbox execution, CLI validation, and preview
application remain out of scope.
