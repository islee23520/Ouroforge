# Multi-Agent Prototype Production Demo v1

This document is the MAP13.15.3 demo guide for issue #678. The demo is a deterministic, fixture-scoped evidence chain for accountable multi-agent collaboration. It is not an autonomous production pipeline, not a hosted worker system, not a browser command bridge, not a release flow, not a current Godot replacement, and not a production-ready claim.

## Demo artifacts

Tracked fixtures live under `examples/multi-agent-pipeline-v1/`:

- `agent-roles.fixture.json` — role accountability model and separation requirements.
- `demo-task-board.fixture.json` — bounded task board and work-package chain.
- `demo-handoff-v2.fixture.json` — advisory handoff fixture with inert allowed command text.
- `demo-ownership-conflicts.fixture.json` — ownership and unresolved conflict report.
- `demo-state-snapshot.fixture.json` — stale/read-only state snapshot.
- `demo-qa-review-flow.fixture.json` — QA queue, performance/regression lane, reviewer/critic gate, and append-only decision metadata.
- `demo-evidence-bundle.fixture.json` — completed fixture-scoped evidence bundle with runs, comparisons, proposals, review decisions, transactions, regression promotions, matrix snapshots, and journal summaries.

Generated task boards, handoffs, work packages, snapshots, evidence bundles, runs, dashboard data, temporary projects, and local tool state remain untracked unless explicitly fixture-scoped like the files above.

## Commands

Use these local commands to validate or inspect the demo evidence chain:

```sh
gh issue view 678 --repo shaun0927/Ouroforge --json number,state,title
gh issue view 1 --repo shaun0927/Ouroforge --json number,state,title
gh issue view 23 --repo shaun0927/Ouroforge --json number,state,title
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ouroforge-core multi_agent_demo_pipeline
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

Dashboard and Studio/cockpit surfaces render this evidence as read-only display. They may show allowed command text, forbidden actions, missing refs, and generated-state boundaries, but they do not execute commands, spawn agents, apply mutations, merge branches, package artifacts, write trusted files, or promote regressions.

## Expected evidence

A passing demo inspection proves only that the fixture chain is internally consistent and safely displayed:

- role assignments and work packages are separated;
- handoff evidence is advisory and command-inert;
- ownership conflicts are visible and unresolved conflicts block promotion;
- stale state is surfaced instead of silently repaired;
- QA queue and performance/regression lane evidence are fixture metadata and do not spawn workers;
- reviewer and critic identities are independent;
- decision ledger events are append-only metadata;
- production evidence bundle categories are present for inspection;
- dashboard display and Studio/cockpit display escape fixture text and render no trusted controls.

## Cleanup policy

No cleanup is required for tracked fixtures. Any local generated output from experimentation must stay under ignored generated roots such as `runs/multi-agent-pipeline/`, `dashboard-data/`, `target/`, or local OMX state. Do not commit generated runs, screenshots, traces, dashboard exports, temporary projects, or local tool state unless a future issue explicitly scopes a fixture.

## Known gaps and out-of-scope behavior

The demo intentionally does not provide:

- hidden background agents or unbounded spawning;
- autonomous unrestricted project mutation or arbitrary game completion;
- auto-apply, auto-merge, self-approval, reviewer bypass, or hidden promotion;
- browser trusted writes, command bridge, local server bridge, hidden command execution, credentialed commands, network/install commands, or dependency mutation;
- hosted/cloud/server orchestration, account system, remote worker pool, production CI/CD automation, release automation, publish action, signing, or public visibility change;
- unrestricted source mutation, arbitrary script execution, dynamic code loading, plugin loading, or visual scripting implementation;
- native export or platform packaging implementation;
- current Godot replacement, production-ready, shipped-game, or commercial readiness claim.

Issues #1 and #23 must remain open after this demo evidence is recorded.
