# Source Mutation Preview v1

Source Mutation Preview v1 is a documentation/control contract for making source
patch suggestions inspectable before any trusted source tree is changed. It is a
preview-only milestone. It does not implement source patch application, merge
automation, branch mutation, dependency mutation, arbitrary command execution,
browser command bridges, hosted services, or production editor behavior.

The trusted boundary remains local-first and Rust-owned. Browser, dashboard, and
Studio surfaces may display exported preview evidence as escaped read-only data
and inert command text only. They must not write trusted files, execute commands,
apply patches, accept mutations, merge branches, install dependencies, or start a
local command bridge.

## Completed Baseline

This preview scope builds only on baseline behavior already documented in this
repository:

- Agentic Review & Regression Promotion v1
  (`docs/agentic-review-regression-promotion-v1.md`) records evidence-linked
  proposal rationale, review decision ledgers, review-gated scene-only
  application, rerun comparison, regression promotion, Journal v2, and read-only
  Studio review cockpit state. It explicitly does not authorize arbitrary source
  patch application, dependency mutation, CI/workflow mutation, branch merge or
  rebase automation, browser command bridges, or auto-merge.
- Agentic Loop Orchestration v1 (`docs/agentic-loop-orchestration-v1.md`)
  records data-only loop plans, inert dry-run sequencing, CLI-only allowlisted
  local step execution, explicit recovery preflight, generated evidence bundles,
  advisory handoff contracts, and read-only Studio loop cockpit inspection.
- Trusted Artifact Write Policy v1 (`docs/artifact-write-policy-v1.md`) records
  generated preview/sandbox artifact categories while preserving the blocked
  source-apply boundary for the trusted main worktree.
- Evidence Fidelity & Trust Boundary Hardening v1
  (`docs/evidence-fidelity-trust-boundary-v1.md`) records the Rust-trusted,
  browser-read-only, generated-state-aware boundary and keeps source mutation
  preview/design work blocked from applying patches to the trusted main
  worktree.
- Governance handoff docs identify Source Mutation Design Gate v1, Asset
  Pipeline v1 / Content Authoring Foundation, and Visual Authoring v1 / Safe
  Local Edit Cockpit as prerequisite roadmap candidates. This document inherits
  only their conservative boundary language where supported locally: design
  gates before implementation, bounded local asset/content handling, safe
  read-only visual inspection, generated-state audits, and no production editor
  claims.

The active project anchors remain #1 and #23. This milestone must not close,
replace, edit, or silently supersede either anchor unless a separate explicit
governance decision exists.

## Milestone Goal

Define the smallest safe path from an evidence-backed source change suggestion
to a reviewable source patch preview artifact:

```text
source file class validation
  -> patch diff integrity checks
  -> inert source patch preview artifact
  -> test command allowlist
  -> sandbox-only apply and allowlist-only dry-run evidence
  -> review decision integration
  -> evidence bundle
  -> read-only Studio review
  -> preview demo
  -> regression/scenario coverage
  -> roadmap/#1 refresh
```

The target outcome is not source mutation application. The target is a bounded
artifact contract that lets maintainers answer:

1. Which source files are eligible for preview?
2. Which files, hunks, and tests would be affected?
3. Which integrity checks prove the preview is inert and reviewable?
4. Which commands are allowed to be shown or later dry-run, and why?
5. Which evidence supports a review decision?
6. Which gaps remain blockers before any future source-apply design can be
   considered?

## Dependency Order

Implement follow-up slices in this order unless a later issue documents a
specific blocker and maintainer-approved replacement ordering:

1. **Source File Class Validator** — classify eligible source-like files and
   reject generated, ignored, dependency, workflow, build-script, lockfile, tool
   state, and unsafe path targets before any patch preview is created. Implemented
   report shape, hard-block behavior, fixtures, and verification notes are
   documented in `docs/source-file-class-validator-v1.md`.
2. **Patch Diff Integrity** — validate unified diff shape, file paths, hunk
   consistency, maximum size, allowed file classes, and no hidden binary or
   generated-state changes. Implemented report and validation read-model shapes
   are documented in `docs/patch-diff-integrity-v1.md`.
3. **Source Patch Preview Artifact** — write an inert generated preview artifact
   that records proposed file changes, hashes, rationale, evidence refs,
   unsupported claims, and review prerequisites without applying the patch.
4. **Test Command Allowlist** — define the narrow local command vocabulary that
   may be displayed with a preview, such as specific cargo or Node checks already
   used by the repository. The allowlist is data for review; it is not a shell
   execution surface.
5. **Sandbox Dry-Run Evaluator** — implemented only as a bounded Rust-owned
   preview evaluator documented in
   `docs/source-patch-sandbox-dry-run-evaluator-v1.md`: validated preview diffs may be copied/applied inside an
   isolated generated sandbox/worktree, and declared required tests may run only
   when their normalized `argv` matches the repository source-patch test command
   allowlist. It must not mutate the trusted main worktree, broaden into a shell
   runner, or claim secure sandboxing.
6. **Source Patch Review Decision Integration** — connect preview artifacts to
   review decisions without making a decision apply, merge, or execute anything.
7. **Source Patch Evidence Bundle** — collect preview, integrity, allowlist,
   review, and optional dry-run evidence refs into a generated local bundle with
   explicit missing/stale/malformed states.
8. **Studio Source Patch Review** — render preview/review/bundle state in Studio
   as escaped read-only exported data and inert copyable command text only.
9. **Source Mutation Preview Demo** — compose the merged preview slices into a
   deterministic local demonstration without applying patches to the trusted
   main worktree.
10. **Scenario Coverage v6** — add focused coverage for supported preview
    contracts, malformed preview data, guardrail rejections, and read-only
    Studio/dashboard behavior.
11. **Roadmap/#1 refresh** — update top-level docs and #1 governance after the
    milestone is implemented and verified, preserving #1 and #23 as open anchors
    unless a separate explicit replacement decision exists.

No follow-up slice may skip ahead by applying source patches, running arbitrary
commands, mutating branches, changing dependencies, or giving browser/Studio
surfaces trusted authority.

## Safety Boundaries

Source Mutation Preview v1 allows only inert preview and evidence contracts when
explicitly scoped by follow-up issues:

- classify source file eligibility;
- validate patch diff shape and integrity;
- write generated preview artifacts under ignored/local generated-state roots;
- display preview state in CLI, dashboard, Journal, or Studio as read-only data;
- record review decisions that reference preview artifacts;
- record allowlisted command text as inert data;
- evaluate a preview in an isolated generated sandbox/worktree through explicit
  Rust-owned allowlisted dry-run commands when the governing issue scopes it.

Source Mutation Preview v1 does not authorize:

- applying source patches to the trusted main worktree;
- committing, merging, rebasing, auto-merging, or mutating branches;
- dependency, lockfile, CI/workflow, build-script, install, or arbitrary shell
  mutation;
- browser trusted writes, browser command bridges, local server command bridges,
  or hidden command execution;
- hosted/cloud/server/database/auth infrastructure;
- public launch automation, repository visibility changes, package publishing,
  binary releases, or production editor behavior;
- native export implementation, plugin runtime, marketplace, dynamic loading, or
  extension API;
- secure-sandbox, production-ready, broad compatibility, or Godot replacement
  claims.

## Source File Class Validator Boundary

The first implementation slice must reject unsupported targets before preview
artifacts exist. Eligible files should be narrow and explicit. At minimum, the
validator must distinguish:

- tracked source-like code and tests that a later issue explicitly allows;
- docs and fixtures, when the issue explicitly allows them;
- generated/local roots such as `runs/`, `target/`, `.omx/`, `.omc/`,
  `.openchrome/`, `.claude/`, dashboard exports, and temporary sandboxes;
- dependency, lockfile, CI/workflow, build-script, package-manager, generated,
  binary, and ignored files;
- path traversal, symlink/alias, absolute path, and out-of-worktree targets.

Unsupported files must fail closed with explicit reasons. Missing or ambiguous
classification must not be inferred as safe.

## Patch Diff Integrity Boundary

Patch previews are inert data. Integrity validation should prove that the data is
well-formed enough for review, not that it is safe to apply. A valid preview
should record:

- schema/version and preview id;
- source proposal, run, journal, comparison, or review evidence refs;
- target file paths and file classes;
- old/new hashes or documented missing-hash state where available;
- parsed hunks and affected line ranges;
- expected added/removed line counts;
- unsupported claims and manual review notes;
- generated-state and trusted-worktree non-mutation assertions.

Validation must reject malformed diffs, unknown file classes, hidden binary
patches, path aliases, oversized previews, generated-state writes, and stale or
missing required evidence according to the follow-up issue contract.

## Test Command Allowlist Boundary

Allowlisted commands are review metadata until a later issue explicitly scopes
execution. The allowlist must be narrow, repository-specific, and argument-aware.
It may include only commands that are already normal local verification surfaces
for the affected files, for example focused `cargo`, `node --check`, or static
test commands. It must reject shell metacharacter composition, package
installation, network fetches, arbitrary scripts, dependency updates, build
script mutation, Git operations, and browser/server command bridges.

Studio/dashboard may show allowed command text read-only. They must not execute
the command text.

## Sandbox Dry-Run Boundary

Sandbox dry-run evaluation is implemented only for the bounded preview path and
must remain allowlist-only. The evaluator must:

- create or use an isolated generated sandbox/worktree, never the trusted main
  worktree;
- apply the preview only inside that isolated generated location;
- run only explicit allowlisted `argv` commands through Rust-owned code after
  command text normalization, forbidden-command classification, and policy-id
  checks;
- record apply logs, command stdout/stderr summaries, exit status, changed-file
  lists, hashes, run ids, and comparison refs as generated evidence;
- report failed apply or failed tests as evidence, not as permission to retry
  destructively;
- cleanly distinguish "not run", "blocked", "failed", "passed", and "stale";
- avoid secure-sandbox claims, remote execution, cloud workers, servers,
  databases, auth, package installs, and network-dependent evaluation.

The current runner is not an arbitrary shell surface: it uses `Command` directly
with normalized `argv`, sets `current_dir` to the sandbox worktree, writes a
bounded JSON test-execution report under sandbox evidence, and fails closed on
blocked commands or test failures.

## Review, Evidence Bundle, and Studio Boundaries

Review decisions may reference source patch preview artifacts, integrity reports,
allowlisted command text, and optional sandbox dry-run evidence. A decision
record does not apply a patch, rerun tests, merge a branch, or accept a mutation
automatically.

Evidence bundles should collect existing refs and expose missing, stale, or
malformed state. They must not invent proof, hide unsupported claims, become a
storage backend, or package source patches for production apply.

Studio Source Patch Review may display:

- preview metadata, target file classes, diff summaries, and integrity state;
- review decisions and rationale;
- missing/stale/malformed evidence warnings;
- inert allowlisted command text;
- optional sandbox dry-run evidence generated by the Rust-owned sandbox
  evaluator.

Studio must remain read-only for trusted state.

## Verification Policy

Each follow-up implementation PR must define focused verification for the slice
it changes and include at least:

- issue number and PR unit id;
- exact authorized behavior and explicit non-goals;
- expected changed files;
- focused Rust tests for validators, artifacts, review integration, CLI surfaces,
  or generated-state policy touched by the slice;
- Node static checks/tests for dashboard or Studio changes;
- `git diff --check`;
- generated-state audit proving `runs/`, `target/`, dashboard exports,
  sandbox roots, sandbox evidence reports, and local tool state remain
  ignored/untracked unless explicitly fixture-scoped;
- no-browser-writes/no-command-bridge audit;
- no-source-apply/no-main-worktree-mutation audit;
- #1/#23 state checks where governance is in scope;
- latest-main issue closure verification before marking a follow-up issue done.

Full `cargo fmt`, `cargo test`, `cargo clippy`, browser smoke tests, scenario
runs, and dashboard exports are required only when a follow-up issue touches
those surfaces or explicitly scopes them. If a check cannot run, closure evidence
must say why, what next-best validation ran, and whether the gap blocks closure.

## Closure Gates

Do not mark Source Mutation Preview v1 or any follow-up slice complete until:

- all fixed PR units for that issue are merged in dependency order;
- latest `main` has been pulled;
- issue-level verification passes on latest `main`;
- guardrails, non-goals, drift checks, and over-engineering checks remain true;
- generated preview, sandbox, run, dashboard, and local tool artifacts remain
  untracked unless an issue explicitly scopes a tiny deterministic fixture;
- browser/dashboard/Studio surfaces remain read-only;
- the trusted main worktree was not modified by preview evaluation;
- #1 and #23 remain open unless a separate explicit governance replacement
  decision exists;
- the issue has a final evidence comment with merged PRs, verification, known
  gaps, generated-state audit, and closure rationale.

## Closure Criteria for #355

Issue #355 is satisfied when:

- this canonical scope document exists;
- the dependency order for Source Mutation Preview v1 slices is explicit;
- non-goals, safety boundaries, verification gates, and closure gates are
  documented;
- browser/dashboard/Studio surfaces are constrained to read-only preview
  inspection;
- sandbox dry-run remains isolated, generated-state-scoped, and allowlist-only;
- no product code, source patch application, command bridge, hosted service,
  branch/dependency mutation, or production editor behavior is added;
- #1 and #23 remain preserved as open roadmap/governance anchors unless a
  separate explicit governance decision exists.

This document is documentation only. It authorizes no product behavior changes by
itself.

## Source patch review read model v1

SMP1.7.3 exposes source patch review decisions as read-only evidence state. A
review read model may show the review decision id, patch preview id, review
status, file-class report, diff-integrity report, sandbox report, required test
commands, and blocked reasons. It is display-only: dashboard, cockpit, and Studio
surfaces may inspect evidence and copy required test commands, but they must not
apply patches, merge branches, execute commands, write trusted files, or add a
browser command bridge.

A `reviewed` source patch review status is not source apply authorization. Later
source apply work must use a separate explicitly-scoped design and review gate.
