# Provenance Replayability v1

#1502 adds a local Rust replay path for provenance bundles. A replay reconstructs the referenced run evidence in a caller-provided workspace and reuses `evaluate_run`; it does not execute arbitrary commands, mutate source, or add browser write authority.

Replay results are conservative:

- `reproduced`: the referenced expected verdict matches the current replay verdict.
- `diverged`: the verdict differs and the result carries a JSON diff.
- `not-replayable`: required replay inputs are absent, non-deterministic inputs lack deterministic metadata refs, or bundle refs are stale or dangling.

Generated replay outputs remain untracked unless fixture-scoped. The fixture bundles keep `generatedState.fixtureScoped` true because they are contract evidence.

Compatibility is additive. Bundles without `replayInputs` remain valid provenance bundles, but replay reports them as `not-replayable`.

Governance boundaries remain unchanged: #1 remains open, #23 remains open, and this does not implement #1503 sign-off/audit, #1504 export, #1505 dashboard UI, or #1506 governance.
