# Authoring Loop v2 / MVP Consolidation

Authoring Loop v2 is the next local-first consolidation milestone for Ouroforge. It turns the current evidence-native engine demo into a bounded authoring loop for small game-data changes:

```text
scene edit -> Rust validation -> run -> evidence -> compare -> journal/mutation review
```

The milestone is intentionally conservative. Rust remains the trusted persistence boundary, browser examples remain read-only displays, generated evidence remains untracked, and the project does not claim to replace production engines such as Godot.

## Current baseline

This milestone starts after these completed contracts:

- Runtime v1 (`docs/runtime-v1.md`, `docs/runtime-v1-demo.md`)
- Scenario/Evaluator v1 (`docs/scenario-evaluator-v1.md`, `docs/scenario-evaluator-v1-demo.md`)
- Evolve Loop v1 (`docs/evolve-loop-v1.md`, `docs/evolve-loop-v1-demo.md`)
- Studio v1 (`docs/studio-v1.md`, `docs/studio-v1-demo.md`)
- Engine Expansion v1 (`docs/engine-expansion-v1.md`, `docs/engine-expansion-v1-demo.md`)

The active project anchors remain #1 for the broad vision and #23 for distributed/evaluator design exploration. Authoring Loop v2 must not close, replace, or silently implement either anchor.

## Goal

Ouroforge should support a practical local authoring loop for a small scene change:

1. record the scene edit as a transaction;
2. validate it through Rust-owned scene semantics;
3. bind subsequent QA runs to the edit provenance;
4. compare evidence semantically enough for mutation review;
5. expose the loop in Studio as read-only state and copyable CLI commands;
6. preserve manual accept/reject boundaries for mutation lifecycle decisions.

## Dependency order

Implementation should proceed in this dependency order unless code findings prove a safer order:

1. #210 — Authoring Loop v2 Scope and Contract
2. #213 — Scenario Coverage v2: Engine Feature Regression Suite
3. #211 — Authoring Loop v2: Scene Edit Transaction Model
4. #212 — Authoring Loop v2: Scene Edit to QA Run Binding
5. #214 — Run Comparison v2: Semantic Evidence Diff
6. #215 — Evolve Loop v2: Scene-Only Safe Mutation Application
7. #216 — Studio v2: Evidence-Backed Authoring Cockpit
8. #217 — Public Readiness Refresh after Engine Expansion v1

Coverage comes before transaction work so the existing engine expansion features have a regression safety net before the authoring loop changes run, comparison, and mutation metadata.

## Artifact contract

Tracked artifacts may include:

- Rust models and tests;
- CLI command handling and focused test fixtures;
- deterministic seed/scenario fixtures;
- dashboard/cockpit source and tests;
- documentation that references generated evidence by run id or path.

Generated artifacts must stay untracked, including:

- `runs/` output;
- `target/` build output;
- `.openchrome/`, `.omc/`, `.omx/`, and `.claude/` local/runtime state;
- generated dashboard export data unless a tracked sample already exists and the PR explicitly updates it as a fixture.

## Verification policy

Every PR unit must run focused checks for the changed behavior plus broad sanity checks. At minimum:

- Rust formatting and tests for Rust changes;
- focused CLI tests for transaction/run/mutation/compare behavior;
- Node syntax or focused Node tests for dashboard/cockpit changes;
- smoke commands only when the PR changes run/evidence/dashboard integration;
- generated-state audit before commit and before issue closure.

Broad smoke output is never sufficient by itself. Each issue must identify the exact artifact or assertion that proves its contract.

## Rust-trusted / browser-read-only boundary

Rust CLI/code owns trusted reads, writes, validation, transaction emission, run binding, comparison artifacts, mutation application, and rollback metadata. Browser examples may read exported JSON and display copyable command strings. Browser JavaScript must not directly write trusted files, execute local commands, or become a command bridge.

## Explicit non-goals

Authoring Loop v2 does not authorize:

- native export implementation;
- plugin runtime or marketplace implementation;
- hosted/cloud/server/database/auth infrastructure;
- distributed QA/Elixir implementation;
- browser-side trusted file writes;
- arbitrary source-code mutation;
- auto-merge, auto-accept, or autonomous production release behavior;
- production editor or Godot replacement claims.

Existing design documents for native export, plugin systems, and distributed evaluation remain design references only.

## Follow-up issue roles

- #213 creates deterministic feature regression coverage so later metadata changes do not hide Engine Expansion regressions.
- #211 defines scene edit transactions with before/after hashes, validation result, rollback metadata, and provenance.
- #212 binds optional transaction provenance into runs, journals, and dashboard read models without breaking legacy runs.
- #214 adds deterministic semantic evidence diff sections for run comparison and mutation review.
- #215 safely applies scene-only mutations through validation and transaction emission while preserving manual lifecycle boundaries.
- #216 surfaces authoring-loop state in the cockpit without browser writes or command execution.
- #217 refreshes public-readiness evidence and wording after the milestone, without making a public-launch decision.

## PR expectations

Each implementation PR should state:

- exact issue and PR unit;
- expected changed files;
- what is explicitly out of scope;
- focused verification commands and results;
- generated artifact paths left untracked;
- checklist audit before closing the issue.

Issue closure is allowed only after the relevant PR units are merged, verification evidence is recorded in an issue comment, generated state remains untracked, and the issue Definition of Done is satisfied.
