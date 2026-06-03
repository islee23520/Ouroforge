# Authoring Loop Dry-Run v1

Authoring Loop Dry-Run v1 reads an Authoring Loop Plan v1 artifact and renders a
deterministic summary of what would be needed to continue the loop. It is an
inspect-only command and read-model surface: command text is inert, copyable text
for a human/operator, not an execution bridge.

## CLI

```bash
cargo run -p ouroforge-cli -- loop dry-run examples/authoring-loop-dry-run-fixtures/ready/loop-plan.json
cargo run -p ouroforge-cli -- loop dry-run examples/authoring-loop-plan-fixtures/blocked/missing-prereq.json
```

The command prints JSON with:

- `schemaVersion: authoring-loop-dry-run-v1`;
- loop/project/Seed/scenario-pack context copied from the validated plan;
- overall `status` (`ready` or `blocked`);
- ordered step summaries;
- inert `commandText` for each bounded step kind;
- prerequisite labels and expected artifacts;
- missing prerequisites and decision gates;
- safety gates and a boundary statement.

## Read-only surfaces

The evidence dashboard and authoring cockpit render `loop_dry_run` (or
`loopDryRun`) objects when present in exported dashboard data. The surfaces:

- escape all dry-run ids, prerequisite text, missing-prerequisite text, and
  command text;
- display command text as inert text only;
- do not execute browser-side commands;
- do not write plan files, reports, runs, decisions, scenes, scenario packs, or
  promotion records;
- show missing prerequisites and blocked state explicitly.

The current Rust dashboard exporter does not automatically write a dry-run
report into dashboard data. Operators may inspect CLI output directly, and later
issues may generate explicit local dry-run report artifacts under generated
state.

## Fixture policy

`examples/authoring-loop-dry-run-fixtures/ready/` contains tiny deterministic
fixture-scoped placeholder artifacts so the ready path can be exercised without
creating local generated state. `examples/authoring-loop-plan-fixtures/blocked/`
contains a blocked plan fixture. These are checked-in fixtures only; runtime
`runs/`, dashboard exports, and dry-run reports remain generated/untracked.

## Non-goals

Dry-run v1 does not:

- execute commands;
- create missing artifacts;
- mutate scenes;
- write scenario packs or regression promotion records;
- record review decisions;
- add a step runner;
- add resume/retry behavior;
- add browser execution controls;
- bridge browser UI to local command execution;
- implement hosted orchestration.
