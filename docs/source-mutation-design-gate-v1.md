# Source Mutation Design Gate v1

Source Mutation Design Gate v1 is a control milestone, not an implementation
milestone. It defines the evidence, threat model, file-class boundaries, review
requirements, rollback/audit expectations, sandbox limits, and read-only Studio
review design that must exist before Ouroforge can consider any future source
code mutation capability.

This gate exists because source mutation crosses a higher-trust boundary than
scene-only data edits. Existing Ouroforge milestones keep trusted persistence in
Rust and keep browser surfaces read-only. Source code patches could affect build
scripts, tests, runtime behavior, repository history, and user machines, so they
require explicit design controls before any apply path is authorized.

## Completed baseline

The design gate starts from the completed local-first baseline:

- **Project Workspace Loop v1**: project manifests, scaffolding, scenario packs,
  project runs, project comparison, project-level mutation loop framing, and
  Studio project cockpit inspection.
- **Evidence Fidelity & Trust Boundary Hardening v1**: runtime probe contracts,
  input replay evidence, CDP observation boundaries, reproducible command
  context, and Studio evidence warnings that distinguish Rust-trusted artifacts
  from browser observations.
- **Agentic Review & Regression Promotion v1**: proposal quality, review
  decisions, review-gated scene-only mutation application, rerun comparison,
  regression promotion/matrix, Journal v2, and Studio review cockpit state.
- **Agentic Loop Orchestration v1**: data-only loop plans, dry-run sequencing,
  CLI-only trusted step execution, resume/failure recovery, evidence bundles,
  agent handoff contracts, and Studio loop cockpit inspection.
- **Engine Expressiveness v2 / Playable Game Authoring v1 implemented subset**:
  expressive scene components, collision/physics rules, triggers/flags, HUD
  entities, animation/audio event evidence, manifest-declared scene transitions,
  the collect-and-exit demo, scenario coverage v3, and read-only Studio scene
  inspection.

That baseline is evidence-native and local. It does not imply permission to
modify source files automatically.

## Dependency order

Follow-up Source Mutation Design Gate issues must stay design/control-only and
should be completed in this order:

1. **Threat model**: identify assets, actors, trust boundaries, abuse cases,
   failure modes, and explicit mitigations for any future source patch proposal
   workflow.
2. **Allowed source mutation file classes**: define which repository file classes
   could ever be eligible for future proposals, which file classes are always
   forbidden, and what metadata proves a file is in an allowed class.
3. **Patch preview artifact**: define the data-only preview shape for proposed
   changes, including diff metadata, file-class classification, risk labels,
   verification commands, and reviewer-facing rationale. This design gate does
   not implement the schema or producer.
4. **Source patch review gate**: define the explicit human/review decision model
   required before a later milestone may apply any patch, including rejection,
   hold, and stale-preview behavior.
5. **Rollback and audit contract**: define required audit records, provenance,
   reversible checkpoints, rollback evidence, and retention expectations for any
   future patch workflow.
6. **Sandbox/worktree boundary**: define the local isolation, worktree, command
   allowlist, generated-state, and cleanup boundaries that must exist before
   future dry-run evaluation.
7. **Studio source patch review design**: define a read-only Studio review
   surface for patch previews, warnings, evidence, and copyable CLI commands;
   Studio must not write trusted files or execute commands.
8. **Roadmap and #1 governance refresh**: update roadmap/#1 context only after
   the above design/control surfaces are complete, preserving #1 and #23 as open
   anchors.

Later implementation milestones may depend on these designs, but this milestone
alone does not authorize implementation.

## Follow-up issue verification policy

Each follow-up design/control issue should define:

- the exact control surface being specified;
- the files expected to change;
- the non-goals that remain blocked;
- the verification commands or review checks needed for closure;
- generated-state expectations; and
- proof that #1 and #23 remain open.

Issue closure should require:

1. all fixed PR units for the issue merged in order;
2. latest `main` pulled locally;
3. issue-level verification run on latest `main`;
4. Definition of Done, guardrail, drift-prevention, over-engineering, and
   generated-state audits recorded;
5. final issue evidence comment with merged PRs, verification output, known
   gaps, and closure rationale; and
6. #1 and #23 confirmed open.


## Gate outcome and sequencing recommendation

Source Mutation Design Gate v1 is complete when the roadmap/#1 governance
refresh records the gate outcome. The outcome is conservative:

- source mutation apply remains blocked;
- arbitrary patch apply, auto-merge, auto-accept, browser command bridges,
  browser trusted writes, credentialed commands, implicit network, install
  scripts, and CI/workflow mutation remain blocked;
- the completed gate is evidence for future planning only, not an implementation
  authorization;
- Asset Pipeline v1 and Visual Authoring v1 should proceed before Source
  Mutation Preview v1 implementation work; and
- any later Source Mutation Preview v1 implementation must remain inert
  preview/evidence work until a separate explicit governance decision authorizes
  anything broader.

#1 remains the broad roadmap/vision anchor and #23 remains the repo-memory/design
context anchor. This gate does not close, replace, or narrow either issue.

## Source mutation remains blocked

Source mutation apply remains blocked until a later explicit implementation
milestone authorizes it. This design gate does not authorize:

- source mutation application;
- arbitrary patch application;
- patch preview schema implementation beyond design documentation;
- automatic apply, automatic merge, or self-approval;
- browser-side trusted writes;
- browser command bridges or hosted command execution;
- plugin runtime, marketplace, dynamic loading, or extension APIs;
- hosted/cloud/server/database/auth infrastructure;
- production editor, production-ready engine, compatibility-stable engine API,
  Godot replacement, or public-launch automation claims.

Any future source mutation implementation must be separately scoped, review
blocked by default, locally sandboxed, Rust-trusted where persistence is needed,
and backed by focused regression evidence.

## Studio boundary

Studio source patch review, when designed in a later issue, must remain a
read-only evidence surface. It may display patch previews, file-class labels,
risk warnings, verification status, stale evidence, reviewer notes, and copyable
CLI commands. It must not execute commands, write trusted repository files,
auto-approve changes, or become a command bridge.

## Generated-state policy

Generated run state, caches, local worktrees, build outputs, and evidence bundles
remain untracked unless a future issue explicitly scopes a tiny deterministic
fixture as source-like data. Closure audits should use `git status --short
--ignored` or an equivalent check to confirm generated/local artifacts remain
ignored.

## #1 / #23 governance preservation

- #1 remains open as the broad roadmap/vision anchor.
- #23 remains open as the repo-memory/design context anchor.
- This design gate does not replace, close, or narrow either anchor.
