# Project Workspace Loop v1

Project Workspace Loop v1 is the next local-first consolidation milestone for
Ouroforge. It moves the current run-centered authoring loop into a small local
game project workspace where the project manifest ties together scene files,
seeds, scenario packs, asset roots, runs, evidence, comparisons, journals, and
scene-only mutation lifecycle records.

```text
project manifest -> scaffold/scene/seed/scenario pack -> run -> evidence -> compare -> journal/mutation review
```

The milestone is intentionally conservative. Rust remains the trusted
validation and persistence boundary, browser examples remain read-only displays,
generated evidence remains untracked, and the project does not claim to replace
production engines such as Godot.

## Current baseline

This milestone starts after these completed contracts:

- Runtime v1 (`docs/runtime-v1.md`, `docs/runtime-v1-demo.md`)
- Scenario/Evaluator v1 (`docs/scenario-evaluator-v1.md`,
  `docs/scenario-evaluator-v1-demo.md`)
- Evolve Loop v1 (`docs/evolve-loop-v1.md`, `docs/evolve-loop-v1-demo.md`)
- Studio v1 (`docs/studio-v1.md`, `docs/studio-v1-demo.md`)
- Engine Expansion v1 (`docs/engine-expansion-v1.md`,
  `docs/engine-expansion-v1-demo.md`)
- Authoring Loop v2 (`docs/authoring-loop-v2.md`)
- Studio v2 (`docs/studio-v2-cockpit.md`)
- Public-readiness refresh (`docs/public-readiness-audit.md`,
  `docs/public-demo-evidence.md`, `docs/public-launch-checklist.md`)

The active project anchors remain #1 for the broad vision and #23 for
repo-memory/design context. Project Workspace Loop v1 must not close, replace,
or silently implement either anchor.

## Goal

Ouroforge should support a practical local authoring loop for a small game
workspace:

1. identify a project through a Rust-validated manifest;
2. scaffold a minimal local project without external package/template systems;
3. group QA scenarios as project-level scenario packs;
4. bind QA runs to project manifest, seed, scene hashes, and scenario pack
   provenance;
5. compare project-bound runs semantically enough for authoring and mutation
   review;
6. apply scene-only mutations only to project-authorized scene files;
7. expose the project loop in Studio as read-only state and copyable CLI
   commands;
8. refresh the roadmap/#1 governance state after implementation.

## Dependency order

Implementation should proceed in this dependency order unless code findings are
documented in the issue before a safer order is adopted:

1. #245 — Project Workspace Loop v1 Scope and Contract
2. #246 — Project Manifest v1: Local Game Workspace Contract
3. #247 — Project Scaffold v1: Create Small Game Workspace
4. #248 — Scenario Pack v1: Project-Level QA Contracts
5. #249 — Project Run v1: Workspace-Bound Run Metadata
6. #250 — Project Comparison v1: Cross-Run Project Evidence Summary
7. #251 — Project Mutation Loop v1: Workspace-Safe Scene Mutation
8. #252 — Studio v3: Project Workspace Cockpit
9. #253 — Roadmap and #1 Refresh after Authoring Loop v2

The manifest comes before scaffold, scenario packs, run metadata, comparison,
mutation, and Studio v3 because it is the trusted workspace boundary. Studio v3
comes late because it should display exported/read-model data produced by the
Rust-owned layers rather than inventing trusted browser behavior.

## Project artifact contract

Tracked project source artifacts may include:

- `ouroforge.project.json` manifest files;
- scene JSON files;
- Seed YAML files;
- scenario pack files;
- asset manifests and local asset references;
- source docs and small deterministic fixtures;
- Rust models, CLI code, and tests;
- dashboard/cockpit source and tests.

Generated artifacts must stay untracked, including:

- `runs/` output;
- `target/` build output;
- generated dashboard export data;
- `.openchrome/`, `.omc/`, `.omx/`, and `.claude/` local/runtime state;
- temporary scaffold/run smoke directories under `.omx/tmp/`.

A PR may commit generated-looking data only when the issue explicitly scopes it
as a tiny deterministic fixture and the PR explains why it is source-like test
data rather than local run output.

## Manifest boundary

Project Manifest v1 is the root of trust for workspace resolution. The concrete
field contract, examples, CLI validation command, and non-goals are documented
in `docs/project-manifest-v1.md`. It should identify:

- project id and name;
- schema version;
- scene paths;
- seed paths;
- scenario pack paths;
- asset roots;
- runs root;
- generated-state policy.

Project paths must be project-local and must not escape via absolute paths or
`..`. Project source paths must not point at generated roots, hidden runtime
state, local tool directories, or browser-owned output.

## Verification policy

Every fixed PR unit must run focused checks for the changed behavior plus broad
sanity checks appropriate to the issue. At minimum:

- Rust formatting and tests for Rust changes;
- focused CLI integration tests for manifest/scaffold/run/mutation/compare
  behavior;
- Node syntax or focused Node tests for dashboard/cockpit changes;
- project validate/scaffold/run smoke commands when a PR changes workspace
  integration;
- generated-state audit before commit and before issue closure;
- #1 and #23 state checks before issue closure.

Broad smoke output is never sufficient by itself. Each issue must identify the
exact artifact or assertion that proves its contract.

## Rust-trusted / browser-read-only boundary

Rust CLI/code owns trusted reads, writes, validation, scaffold generation,
manifest resolution, run binding, journal/dashboard project-context export, comparison artifacts, scene-only mutation
application, and rollback metadata. Browser examples may read exported JSON and
display copyable command strings. Browser JavaScript must not directly write
trusted files, execute local commands, or become a command bridge.

## Explicit non-goals

Project Workspace Loop v1 does not authorize:

- native export implementation;
- plugin runtime, dynamic loading, marketplace UI, or extension API
  implementation;
- hosted/cloud/server/database/auth infrastructure;
- distributed QA/Elixir implementation;
- browser-side trusted file writes;
- arbitrary source-code mutation;
- auto-merge, auto-accept, or autonomous production release behavior;
- production editor or Godot replacement claims;
- public visibility changes or launch automation.

Existing design documents for native export, plugin systems, and distributed
evaluation remain design references only.

## Follow-up issue roles

- #246 defines and validates the local project manifest as the Rust-trusted
  workspace contract.
- #247 creates a bounded local scaffold command for a minimal project without an
  external template system. The command and generated file tree are documented
  in `docs/project-scaffold-v1.md`.
- #248 adds project-level scenario packs while preserving existing Scenario DSL
  semantics. The schema, examples, and execution boundary are documented in
  `docs/scenario-pack-v1.md`.
- #249 binds runs to project manifest, scene, seed, scenario pack, and optional
  transaction provenance. The command contract and metadata fields are documented
  in `docs/project-run-v1.md`.
- #250 extends semantic comparison with project-level context. The field contract and limitations are documented in `docs/project-comparison-v1.md`.
- #251 makes scene-only mutation application project-aware and limited to
  manifest-authorized scenes. The command contract, rollback metadata,
  dashboard/cockpit read-only evidence, and closure gates are documented in
  `docs/project-mutation-loop-v1.md`.
- #252 surfaces project workspace state in Studio v3 without browser writes or
  command execution.
- #253 refreshes roadmap/#1 governance after the milestone, preserving #23.

## PR expectations

Each implementation PR should state:

- exact issue and fixed PR unit;
- expected changed files;
- behavior explicitly authorized by that PR unit;
- non-goals that remain out of scope;
- focused verification commands and results;
- generated artifact paths left untracked;
- drift/guardrail/over-engineering audit results.

Issue closure is allowed only after all fixed PR units are merged, post-merge
verification passes on latest `main`, generated state remains untracked, #1 and
#23 are checked, and the final issue comment records DoD/guardrail/drift audit
evidence.
