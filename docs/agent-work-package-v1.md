# Agent Work Package v1

Agent Work Package v1 is a fixture-scoped, evidence-linked assignment contract for the multi-agent production pipeline. It gives one supported role a bounded task, expected artifacts, acceptance criteria, verification command text, ownership evidence, and a handoff target.

The artifact is an **untrusted assignment**. It does not execute commands, spawn workers, apply source changes, merge pull requests, repair evidence, grant browser write authority, or bypass review gates. Rust/local validation owns trusted interpretation.

## Required fields

- `schemaVersion: "agent-work-package-v1"`
- `workPackageId`, `taskId`, `role`, and `status`
- `objective`
- `scope.summary`, `scope.inScope`, and `scope.outOfScope`
- `allowedArtifacts`
- `forbiddenActions`
- `acceptanceCriteria[].evidenceRefs`
- `verificationCommands` as inert display text
- `expectedEvidence`
- `ownershipRefs`
- `handoffTarget`
- `blockedReasons` when status is `blocked` or `stale`
- optional `stateSnapshotRef`
- `generatedState`
- `boundary`

Supported statuses are `assigned`, `blocked`, `stale`, `ready-for-review`, and `completed`.

## Fixture matrix

- `agent-work-package.valid.fixture.json` — normal assigned package.
- `agent-work-package.blocked.fixture.json` — blocked package with explicit blocker.
- `agent-work-package.stale.fixture.json` — stale package tied to snapshot drift.
- `agent-work-package.ready-for-review.fixture.json` — accepted evidence shape ready for reviewer inspection.
- `agent-work-package.invalid.fixture.json` — missing acceptance criteria for MAP13.6.2 validation coverage.
- `agent-work-package.overbroad.fixture.json` — negative overbroad-scope example.
- `agent-work-package.unsafe.fixture.json` — negative unsafe-path/browser-bridge example.

## Read-model compatibility

`agent-work-package-read-model-v1` is the normalized display summary for dashboards and Studio/cockpit surfaces. It reports:

- package id, task id, role, and status;
- allowed artifact, acceptance criterion, verification command, and expected evidence counts;
- blocker text for `blocked` or `stale` packages;
- ownership ref paths, handoff target path, and optional state snapshot path;
- `malformedReasons` when Rust/local validation rejects a malformed package.

The read model is still display-only. It does not execute verification command text, spawn hidden agents, apply work, write trusted browser state, auto-merge, or self-approve. Dashboard and cockpit surfaces accept both full fixture-shaped work packages (`agent_work_package`, `agent_work_packages`, and camelCase aliases) and read-model-shaped malformed summaries so missing or malformed state remains visible instead of being silently repaired.

## Boundary

Work packages are displayable planning/evidence data only. They are not command runners and do not create hidden background agents, auto-apply changes, auto-merge, self-approve, write trusted browser state, or claim autonomous arbitrary game completion.
