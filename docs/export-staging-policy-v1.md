# Export Artifact Staging and Generated-State Policy v1

Issue: #726
Roadmap anchor: #1 (Build / Export / Packaging milestone).
Parent scope: #719 (`docs/build-export-packaging-v1.md`).
Status: policy + staging-path helper; no publish behavior.

This document defines where local export artifacts are staged and how generated
export state is kept out of source control.

## Staging directory

Each export run stages its artifacts under a run-scoped directory:

```
target/ouroforge/exports/<run-id>/
```

`<run-id>` is a bounded local id (alphanumeric, dash, underscore, dot). The
staging root lives under `target/`, which is already ignored by `.gitignore`
(`/target/`), so **every export artifact is ignored by default** — no
`.gitignore` change is required and no generated artifact reaches a commit
unless a fixture explicitly scopes it.

## Generated-state policy

The following are generated and remain ignored: export bundle outputs, asset
payloads, asset manifests, fingerprints/checksums, verification logs,
screenshots/world-state captures, temp static servers, and local tool state.

The only tracked export artifacts are **fixture-scoped** inputs (for example the
fixture game source under `examples/export-bundle-v1/`) and committed schema
fixtures used by tests. Generated outputs of a run are never committed.

Each follow-up PR includes a generated-state audit (`git status --short --ignored`) in its closure evidence to prove the working tree stays clean.

## Stale export cleanup

Staging is disposable. A new run writes to its own `<run-id>` directory; old run
directories may be pruned at any time because they are fully regenerated from
the validated profile/plan. Recommended policy:

- Keep only the run ids a caller explicitly wants to retain; treat all other
  run directories under the staging root as stale and safe to delete.
- Never rely on a staged artifact as a source of truth — re-run the export to
  regenerate it.

The `export_staging` module exposes the run-scoped staging path and a helper to
partition existing run directories into kept vs stale sets so cleanup is
deterministic and testable.

## Boundary

Staging assembles local, evidence-backed artifacts only; it does not publish,
deploy, sign, upload, or distribute. #1 remains the broad roadmap anchor and
remains open; #23 remains the repo-memory/design anchor and remains open. This
milestone does not close or modify either.
