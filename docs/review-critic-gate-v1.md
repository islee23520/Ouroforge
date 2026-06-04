# Review/Critic Gate v1

Review/Critic Gate v1 records independent review and critic evidence before a
multi-agent work package can be promoted. It is an inert artifact and validation
contract, not an apply mechanism, hidden worker runtime, command queue, browser
command bridge, release pipeline, production-readiness claim, or Godot
replacement claim.

## Artifact shape

A gate uses `schemaVersion: review-critic-gate-v1` and includes:

- `gateId`, `milestone`, `taskId`, and `decision`;
- implementer, reviewer, and critic role/actor metadata;
- links to `workPackageRef`, `handoffRef`, `stateSnapshotRefs`, optional
  `qaEvidenceRefs`, optional `regressionEvidenceRefs`, and `decisionLedgerRef`;
- `evidenceReviewed`, `risks`, `requiredFixes`, `promotionRecommendation`,
  `confidence`, `blockedReasons`, and `staleStateIndicators`;
- generated-state roots, guardrails, forbidden actions, and boundary text.

Supported decisions are `accepted`, `rejected`, `deferred`, `needs-fix`, and
`blocked`. Accepted gates require a `promote` recommendation and no hidden
blocked reasons or required fixes. Rejected, deferred, needs-fix, and blocked
gates keep their risk, fix, stale-state, or blocker evidence visible instead of
repairing or bypassing it.

## Trust boundary

The gate is inert local evidence:

- it does not execute commands;
- it does not spawn agents or hidden/background workers;
- it does not apply changes, promote outputs by itself, does not merge PRs, publish,
  sign, deploy, or change visibility;
- it does not write trusted browser state or provide a browser/local command
  bridge;
- it does not auto-apply, auto-merge, self-approve, bypass reviewers, or hide
  critic findings;
- it does not use credentialed commands, network/install commands, dependency
  mutation, CI/workflow/build-script mutation, release automation, or dynamic
  code loading;
- it does not claim production readiness, commercial readiness, arbitrary game
  completion, or Godot replacement capability.

Rust/local validation owns trusted gate validation. Browser, dashboard, Studio,
and cockpit surfaces may display escaped read-only gate summaries only. Agent
outputs remain untrusted until review-gated apply or promotion is separately
accepted where scoped.


## Generated-state policy

Generated review gates live under local generated roots such as
`runs/multi-agent-pipeline`. Generated task boards, handoffs, work packages,
snapshots, review gates, evidence bundles, runs, dashboard exports, temporary
projects, and local tool state remain untracked unless explicitly
fixture-scoped. The tracked fixtures under `examples/multi-agent-pipeline-v1/`
are schema and regression examples only.

## Fixture examples

- `review-critic-gate.valid.fixture.json` — accepted review/critic gate;
- `review-critic-gate.rejected.fixture.json` — rejected promotion;
- `review-critic-gate.deferred.fixture.json` — deferred pending more evidence;
- `review-critic-gate.self-review-blocked.fixture.json` — self-review is
  visible as a blocked gate fixture;
- `review-critic-gate.missing-evidence.fixture.json` — missing evidence blocks
  promotion;
- `review-critic-gate.stale.fixture.json` — stale state indicators block
  promotion.

Issue #671 MAP13.8.1 defines the gate artifact, states, docs, and fixtures.
MAP13.8.2 adds deeper independence/evidence validation for self-review,
reviewer bypass, missing evidence, stale refs, and conflicts. Accepted gates
with self-review, reviewer-role drift, missing reviewed evidence, or stale-state
promotion attempts are rejected; blocked gates must keep those blockers visible.
MAP13.8.3 adds a read-only `review-critic-gate-read-model-v1` summary that
links the gate to its work package, handoff, shared state snapshots, QA evidence,
regression/evidence bundle, and decision ledger paths. The read model exposes
reviewer/critic/implementer actor IDs, decision and recommendation state,
blockers, stale indicators, generated roots, and malformed validation reasons for
Studio/dashboard display. It is display-only: it cannot promote work, apply
changes, execute commands, spawn agents, write trusted browser state, merge,
auto-apply, auto-merge, self-approve, or bypass the reviewer/critic gate.
Dashboard and Studio compatibility tests assert escaped rendering and no command
bridge/browser trusted write controls.

Issues #1 and #23 must remain open unless a separate explicit governance
decision says otherwise.
