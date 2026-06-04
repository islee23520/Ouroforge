# Performance Regression Lane v1

Performance Regression Lane v1 records evidence-backed baseline/comparison
results for the multi-agent production pipeline. It is an inert local artifact
and validation contract, not a worker runtime, browser command bridge, hidden
agent loop, trusted write path, release pipeline, production-readiness claim, or
Godot replacement claim.

## Artifact shape

A lane uses `schemaVersion: performance-regression-lane-v1` and includes:

- `laneId`, `milestone`, risk area, assigned role/agent, and classification;
- `baselineRuns` and `comparisonRuns` evidence refs;
- `metrics` with metric source, baseline, comparison, delta, unit, and notes;
- `thresholds` with metric refs, comparison operators, threshold values,
  `classificationIfExceeded`, and rationale;
- `evidenceLinks` to run comparison, frame budget, scenario matrix, QA queue,
  and review/critic gate artifacts;
- browser metric warnings, blocked reasons, generated-state roots, guardrails,
  forbidden actions, and boundary text.

Supported classifications are `improved`, `unchanged`, `regressed`,
`inconclusive`, `missing-baseline`, `unsupported`, and `stale`. Regressed,
inconclusive, missing-baseline, unsupported, and stale lane states keep blocked
reasons visible so promotion cannot silently trust incomplete or unsafe evidence.

## Trust boundary

The lane is inert local evidence:

- it does not execute commands;
- it does not spawn agents, hidden/background workers, remote worker pools, or
  unbounded performance loops;
- it does not mutate trusted files, apply changes, merge PRs, publish, sign,
  deploy, or change visibility;
- it does not write trusted browser state or provide a browser/local command
  bridge;
- it does not auto-apply, auto-merge, self-approve, reviewer bypass, or hide
  regressions;
- it does not use credentialed commands, network/install commands, dependency
  mutation, CI/workflow/build-script mutation, release automation, or dynamic
  code loading;
- it does not claim production readiness, commercial readiness, arbitrary game
  completion, or Godot replacement capability.

Browser metrics are advisory evidence inputs only. Rust/local validation owns the
trusted artifact contract and regression classification checks. Browser-advisory
metric sources require warnings that state advisory status and trusted
Rust/local ownership. Dashboard,
Studio, and cockpit surfaces may display escaped read-only summaries only.

## MAP13.10.1 scope

MAP13.10.1 defines the lane artifact, classification enum, docs, and fixture
examples. MAP13.10.2 adds validation for missing baselines, stale run refs, malformed
metrics, unsupported thresholds, missing evidence, and browser metric trust
warnings. MAP13.10.3 owns dashboard/Studio linkage.

## Generated-state policy

Generated performance/regression lane output lives under local generated roots
such as `runs/multi-agent-pipeline`. Generated task boards, handoffs, work
packages, snapshots, review gates, QA queues, regression lane outputs, evidence
bundles, runs, dashboard exports, temporary projects, and local tool state remain
untracked unless explicitly fixture-scoped. The tracked fixtures under
`examples/multi-agent-pipeline-v1/` are schema and regression examples only.

## Fixture examples

- `performance-regression-lane.valid.fixture.json` — unchanged comparison with
  run comparison, frame budget, scenario matrix, QA queue, and review gate refs;
- `performance-regression-lane.improved.fixture.json` — improved classification;
- `performance-regression-lane.regressed.fixture.json` — regressed
  classification with blocked reason;
- `performance-regression-lane.inconclusive.fixture.json` — incomplete evidence;
- `performance-regression-lane.missing-baseline.fixture.json` — explicit missing
  baseline state;
- `performance-regression-lane.unsupported.fixture.json` — unsupported metric or
  threshold state;
- `performance-regression-lane.stale.fixture.json` — stale run or matrix state;
- `performance-regression-lane.malformed.fixture.json` — intentionally malformed
  lane fixture for validation tests.

Issues #1 and #23 must remain open unless a separate explicit governance
decision says otherwise.
