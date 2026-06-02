# Evolve Loop v1 integration demo evidence

This document records the Evolve Loop v1 integration demo for issue #82. The demo
composes completed Evolve Loop v1 capabilities only:

1. controlled failed scenario evidence;
2. mutation classification;
3. patch draft generation;
4. sandbox-only draft application;
5. rerun comparison;
6. explicit manual review command boundary.

It does **not** apply patches to the primary working tree, merge patches, create
branches, create GitHub PRs, run autonomous agents, or add UI review workflow.

## Demo seed

The demo seed is:

```text
seeds/evolve-v1-demo.yaml
```

It intentionally asserts an incorrect player position after a deterministic move
step. The expected purpose is a controlled failed verdict that can drive the
evolve lifecycle.

## Exact commands

Run from the repository root:

```bash
cargo fmt --check
cargo test
cargo run -p ouroforge-cli -- seed validate seeds/evolve-v1-demo.yaml
cargo run -p ouroforge-cli -- run seeds/evolve-v1-demo.yaml --workers 4
```

The run command prints a run directory such as:

```text
runs/run-1780378946430-53503
```

Then compose lifecycle evidence:

```bash
cargo run -p ouroforge-cli -- evolve demo runs/run-1780378946430-53503
```

The demo command writes:

```text
runs/run-1780378946430-53503/mutation/evolve-v1-demo-summary.json
```

Manual review remains explicit and separate:

```bash
cargo run -p ouroforge-cli -- mutation review runs/run-1780378946430-53503 --accept --reason "manual accept based on rerun comparison evidence"
cargo run -p ouroforge-cli -- mutation review runs/run-1780378946430-53503 --reject --reason "manual reject records alternate reviewer concern"
```

Use one manual decision for a real review record; both commands are shown here to
demonstrate the two allowed terminal decision states.

## Fresh EV7.1 verification evidence

EV7.1 verification used run:

```text
runs/run-1780378946430-53503
```

Lifecycle summary fields:

```json
{
  "status": "lifecycle_evidence_ready",
  "classification_artifact_path": "mutation/classifications.json",
  "patch_draft_artifact_path": "mutation/patch-drafts.json",
  "sandbox_result_path": "sandbox/patch-draft-1/evidence/result.json",
  "rerun_evidence_path": "mutation/rerun-orchestration.json",
  "comparison_artifact_path": "mutation/run-comparison-run-1780378946430-53503--run-1780378946430-53503--sandbox-patch-draft-1.json",
  "manual_review_state": "pending_review",
  "lifecycle_summary_path": "mutation/evolve-v1-demo-summary.json"
}
```

Primary working tree safety was verified by comparing tracked `git status` before
and after the demo lifecycle command. The generated run and sandbox artifacts are
ignored under `runs/`.

## Lifecycle evidence map

| Lifecycle step | Evidence path |
| --- | --- |
| Controlled failure run | `run.json`, `verdict.json`, `evidence/index.json` under the generated run |
| Classification | `mutation/classifications.json` |
| Patch draft | `mutation/patch-drafts.json` |
| Sandbox apply | `sandbox/patch-draft-1/evidence/result.json` |
| Rerun comparison | `mutation/rerun-orchestration.json` and `mutation/run-comparison-*.json` |
| Lifecycle summary | `mutation/evolve-v1-demo-summary.json` |
| Manual review | `mutation/review-decisions.json` after an explicit `mutation review` command |

## Omitted features / boundaries

- Manual review is separate from `evolve demo`; the demo records
  `pending_review` until a reviewer runs `mutation review`.
- Accepted review decisions do not apply, commit, merge, or publish patches.
- No autonomous agent loop is introduced.
- No GitHub PR/review automation is introduced.
- No server, database, cloud, Elixir, or distributed orchestration is introduced.
- No reviewer UI is introduced.
- Subjective gameplay quality is not inferred; comparison is limited to existing
  Scenario/Evaluator evidence deltas.
