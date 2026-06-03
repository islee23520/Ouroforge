# Patch Preview Artifact v1 fixture

This directory contains a tiny deterministic source-like fixture for
`docs/patch-preview-artifact-v1.md`.

- `patch-preview.sample.json` is a data-only patch preview example.
- The embedded `readModelPrototype` shows the summary a future read-only
  dashboard/Studio surface could display.
- The fixture is not an executable patch, does not write source files, does not
  execute commands, and keeps `sourceMutationApplyStatus` set to `blocked`.

This fixture exists only to validate the design vocabulary for #326 / SMG1.4.2.
Source mutation apply remains out of scope.
