# Authoring Loop Plan Model v1

Authoring Loop Plan Model v1 is a local, data-only description of one
Ouroforge evidence-native authoring loop. It records what a loop intends to do,
which artifacts each step needs, which artifacts each step is expected to
produce, and which manual/review decisions are required before gated work can be
considered complete.

The plan model does not execute commands, mutate scenes, promote regressions,
trigger browser actions, bridge to a shell, or decide review outcomes. It is a
validated local artifact that later dry-run, runner, resume, evidence-bundle,
handoff, and Studio cockpit work may read.

## Schema identity

Tracked fixture examples live in
`examples/authoring-loop-plan-fixtures/`. Generated loop plans outside explicit
fixture scope are local generated state and should remain untracked.

Top-level fields:

| Field | Required | Meaning |
| --- | --- | --- |
| `schemaVersion` | yes | Must be `authoring-loop-plan-v1`. |
| `loopId` | yes | Stable local id for this single loop plan. |
| `project.projectId` | yes | Project manifest id expected by the loop. |
| `project.manifestPath` | yes | Project-relative manifest path. |
| `seed` | yes | `{ id, path }` reference for the Seed selected by the loop. |
| `scenarioPack` | yes | `{ id, path }` reference for the project scenario pack. |
| `steps[]` | yes | Ordered bounded loop steps. |
| `generatedState.roots[]` | yes | Project-relative generated-state roots such as `runs`. |
| `generatedState.trackedFixtureOnly` | yes | Indicates checked-in examples are fixtures only, not runtime output. |

Path-like fields use the same conservative project-local path rules as project
manifest paths: no absolute paths, no `..` traversal, no hidden local tool
components, and only bounded ASCII path characters.

## Step model

Each step has:

| Field | Meaning |
| --- | --- |
| `id` | Stable step id, unique inside the plan. |
| `kind` | One of the bounded step kinds below. |
| `status` | Current per-step state. |
| `dependsOn[]` | Earlier step ids that must complete before running/completing this step. |
| `inputs[]` | Earlier expected artifacts consumed by this step. |
| `expectedArtifacts[]` | Artifact refs this step is expected to produce later. |
| `requiredDecisions[]` | Explicit review/acceptance/promotion decisions needed by gated steps. |
| `rollbackRefs[]` | Rollback/provenance artifacts for accepted mutation application. |
| `statusTransition` | Optional `{ from, to }` transition record; `to` must match `status`. |

Allowed step kinds are intentionally bounded:

1. `run-scenario-pack`
2. `compare-runs`
3. `generate-proposal`
4. `record-review-decision`
5. `apply-accepted-scene-mutation`
6. `rerun`
7. `promote-regression`
8. `summarize`

The order is deterministic. A plan may skip future kinds when it only models a
small pending/partial fixture, but it may not move backwards in the sequence.
Dynamic plugin-defined step kinds are out of scope.

## Status and transitions

A step status may be:

- `pending`
- `running`
- `blocked`
- `failed`
- `completed`

Valid transition edges are deliberately small:

- same-state preservation;
- `pending -> running`;
- `pending -> blocked`;
- `running -> blocked`;
- `running -> failed`;
- `running -> completed`;
- `blocked -> pending`;
- `failed -> pending`.

A plan with `statusTransition.to` that does not match the step `status` is
invalid. A direct `pending -> completed` transition is invalid because later
runner/resume work must make in-progress execution or explicit recovery visible
instead of hiding it inside the plan artifact.

Running or completed steps must not depend on incomplete earlier steps. Blocked
and failed steps preserve visible interruption state without implying execution.

## Validation rules

The Rust model validates that:

- schema version is exact;
- ids are bounded path components;
- paths are conservative project-relative paths;
- step ids are unique;
- expected artifact ids are unique;
- step ordering follows the bounded kind sequence;
- `dependsOn[]` references earlier steps;
- required input artifacts reference earlier `expectedArtifacts[]`;
- decision-gated steps include decision references;
- accepted scene mutation application includes rollback refs;
- optional status transitions are allowed edges and end at the current status;
- unknown fields and unknown enum values are rejected by serde.

This validation is not a workflow engine. It rejects malformed plan data before
future orchestration layers read it, but it does not execute, rerun, promote, or
apply anything.

## Generated-state policy

Runtime loop plans, dry-run previews, step records, resume files, evidence
bundles, handoff records, and Studio exports are generated local artifacts. Keep
them under ignored/untracked generated roots such as `runs/`, `target/`, or a
future explicitly documented loop-output directory.

Checked-in files under `examples/authoring-loop-plan-fixtures/` are small
deterministic fixtures authorized by this issue. They are not examples of
runtime output that should be committed during normal use.

Generated-state boundaries:

- fixture files may be tracked only when an issue explicitly scopes them;
- local run/evidence/dashboard/loop outputs remain untracked;
- generated outputs must not overwrite project source files, scene files, Seeds,
  scenario packs, or manifests;
- browser UI remains read-only for trusted state and may not write trusted plan
  files.

## Compatibility and boundaries

Existing project manifests, runs, scenario packs, comparison artifacts, mutation
records, review decisions, regression promotions, journals, and dashboards remain
compatible. The plan model adds a new artifact contract; it does not require
legacy runs to contain loop plans.

Explicit non-goals:

- no CLI command for loop plans in this issue;
- no dry-run sequencer yet;
- no step runner;
- no Studio UI;
- no browser trusted writes;
- no browser/local command bridge;
- no scene mutation by the plan model;
- no regression promotion writes by the plan model;
- no source-code mutation;
- no hosted orchestration.

## Verification

Focused model verification:

```bash
cargo test authoring_loop_plan
```

Issue-level verification also runs:

```bash
cargo fmt --check
cargo test project_manifest
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```
