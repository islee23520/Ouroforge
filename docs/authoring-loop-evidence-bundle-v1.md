# Authoring Loop Evidence Bundle v1

Authoring Loop Evidence Bundle v1 is a generated local index for one authoring loop. It ties the loop plan to step outputs and generated evidence without moving, packaging, or mutating the underlying artifacts.

Generated bundle files live under:

```text
runs/authoring-loop-bundles/<loop-id>/bundle.json
```

The bundle schema is `authoring-loop-evidence-bundle-v1`.

## What the bundle records

A bundle may reference:

- the loop plan;
- per-step outputs;
- source and after runs;
- comparison artifacts;
- proposal records;
- review-decision records;
- transaction/application records;
- regression-promotion records;
- matrix snapshots;
- journal summaries.

Partial and failed loops are valid bundle states. Missing or stale references are surfaced through `missingRefs`; undeclared stale references fail validation when bundles are read.

## Generation boundary

Bundles are generated after explicit CLI loop-step execution. Generation writes only the bundle index under ignored/generated state. It does not move source artifacts, rewrite run artifacts, apply mutations, package files, upload anything, start a server, or add a storage backend.

## Dashboard and cockpit boundary

Dashboard export includes generated bundles as `loop_evidence_bundles`. The dashboard and authoring cockpit render those bundles as escaped read-only evidence. Browser surfaces do not write bundle data, execute loop commands, package artifacts, repair references, apply mutations, promote regressions, or bridge UI actions to CLI execution.

## Retention boundary

Bundles are local generated state. They are useful for inspection and handoff, but they are not a production retention policy, remote sharing service, native export format, or durable storage backend. Keep `runs/`, dashboard exports, and bundle files untracked unless a later issue explicitly changes the policy.
