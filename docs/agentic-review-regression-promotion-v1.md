# Agentic Review & Regression Promotion v1

Agentic Review & Regression Promotion v1 is the next local-first consolidation
milestone after Evidence Fidelity & Trust Boundary Hardening v1. Ouroforge now
has stronger evidence provenance, trusted artifact boundaries, runtime probe
contracts, replay evidence, Openchrome/CDP observation fidelity, reproducible run
command context, and static Studio evidence-fidelity surfaces. The next
bottleneck is turning that evidence into auditable human/agent review decisions
and durable regression coverage without making the system autonomous in unsafe
ways.

The milestone preserves the current loop while adding review governance:

```text
failed/passed run evidence -> mutation proposal quality -> review decision ->
review-gated scene-only application -> rerun/compare -> regression scenario
promotion -> journal/Studio handoff
```

It keeps Ouroforge local-first, Rust-trusted, and browser-read-only for trusted
state. Review decisions are trusted records, not automatic action. Regression
promotion is a Rust-validated draft/preview/commit-through-CLI flow, not a
browser write path or automatic scenario-pack merge.

## Completed Baseline

Agentic Review & Regression Promotion v1 builds on these completed foundations:

- Runtime v1 (`docs/runtime-v1.md`, `docs/runtime-v1-demo.md`) — local run
  execution and generated evidence capture.
- Scenario/Evaluator v1 (`docs/scenario-evaluator-v1.md`,
  `docs/scenario-evaluator-v1-demo.md`) — scenario contracts, replayable steps,
  and verdict evidence.
- Evolve Loop v1 (`docs/evolve-loop-v1.md`, `docs/evolve-loop-v1-demo.md`) —
  local mutation proposal lifecycle and journal evidence.
- Studio v1/v2/v3 (`docs/studio-v1.md`, `docs/studio-v2-cockpit.md`,
  `docs/studio-v3-project-workspace-cockpit.md`) — static browser inspection
  surfaces over exported evidence/read models.
- Engine Expansion v1 (`docs/engine-expansion-v1.md`,
  `docs/engine-expansion-v1-demo.md`) — broader playable-template engine
  features with scenario coverage.
- Authoring Loop v2 (`docs/authoring-loop-v2.md`,
  `docs/scene-edit-transactions.md`, `docs/run-comparison-v2.md`,
  `docs/scene-only-mutation-v2.md`, `docs/studio-v2-cockpit.md`) — scene edit
  transactions, QA binding, semantic comparison, and scene-only safe mutation.
- Project Workspace Loop v1 (`docs/project-workspace-loop-v1.md`,
  `docs/project-manifest-v1.md`, `docs/project-run-v1.md`,
  `docs/project-comparison-v1.md`, `docs/project-mutation-loop-v1.md`) — local
  project manifests, workspace-bound runs, project comparison, and
  project-aware scene mutation.
- Evidence Fidelity & Trust Boundary Hardening v1
  (`docs/evidence-fidelity-trust-boundary-v1.md`,
  `docs/runtime-probe-contract-v2.md`,
  `docs/input-replay-evidence-v2.md`,
  `docs/openchrome-cdp-evidence-fidelity-v2.md`,
  `docs/reproducible-run-command-context-v1.md`,
  `docs/studio-evidence-fidelity-surfaces.md`) — hardened trusted-write,
  observation, replay, command-context, and Studio evidence surfaces.

The active project anchors remain #1 and #23. This milestone keeps both anchors
open unless a separate explicit replacement decision exists.

## Milestone Goal

Make review and regression promotion auditable enough that future agents and
maintainers can answer:

1. Which evidence supports this mutation proposal?
2. What risk, scope, and regression implications were reviewed?
3. Who or what recorded the accept/reject/defer decision?
4. Did accepted scene-only application remain Rust-validated and review-gated?
5. Which failure or accepted behavior should become a durable scenario?
6. Which rerun/compare evidence proves the decision did or did not improve the
   project?
7. How does the journal and Studio surface the decision without inventing trusted
   browser authority?

## Follow-up Issue Dependency Order

Implement follow-up issues in this order unless a concrete blocker is documented
in the affected issue before changing scope:

1. #294 — Agentic Review & Regression Promotion v1 Scope and Contract.
2. #295 — Mutation Proposal Quality v2: Evidence-Linked Change Rationale.
3. #296 — Review Decision Ledger v1.
4. #297 — Accepted Mutation Application v2: Review-Gated Scene Change.
5. #298 — Regression Promotion v1: Failure Evidence to Scenario Pack.
6. #299 — Regression Run Matrix v1.
7. #300 — Evidence-Backed Journal v2.
8. #301 — Studio Review Cockpit v1.
9. #302 — Roadmap and #1 Governance Refresh after Review/Regression v1.

The order intentionally improves proposal quality before decisions, decisions
before review-gated application, application before regression promotion,
regression promotion before matrices, and backend/read-model evidence before
Journal/Studio governance surfaces.

## Review Governance Boundary

Review decisions are append-only, evidence-linked records. They may authorize a
later Rust command to apply a scene-only mutation, but the decision record itself
must not directly write scene files, execute commands, rerun tests, merge
branches, or accept proposals automatically.

A review decision should record enough context for later audit:

- decision id and schema version;
- proposal id and evidence refs;
- decision kind, such as accept, reject, defer, or needs-more-evidence;
- reviewer identity or local agent identity when available;
- rationale and risk notes;
- expected follow-up verification or regression promotion hints;
- timestamps or deterministic local ordering where the issue permits them.

Accepted decisions are trusted records because Rust validates and writes them.
They are not trusted proof that the change is safe until a review-gated apply,
rerun, comparison, and issue-level verification pass.

## Review-Gated Scene Application Boundary

This milestone may extend scene-only mutation application only when follow-up
issues explicitly scope it. Any accepted application path must:

- operate on scene data or project-authorized scene files only;
- require a valid accepted review decision;
- require existing scene validation, hash/provenance, and rollback metadata;
- reject stale proposal, stale scene, missing evidence, or mismatched project
  context;
- write explicit application evidence and ledger/journal context;
- avoid source-code mutation and trusted main-worktree patch application.

No issue in this milestone authorizes arbitrary source patch apply, dependency
changes, CI/workflow mutation, build-script mutation, branch merge/rebase
automation, or automatic application from browser UI.

## Regression Promotion Boundary

Regression promotion converts evidence-backed failures or accepted behavior into
scenario-pack candidates through a Rust-owned draft/preview/validation flow.
It does not make the browser a trusted writer and does not automatically merge
new scenarios into project packs.

A safe regression promotion path should:

- identify source run/proposal/decision/comparison evidence;
- draft a scenario or scenario-pack entry with explicit provenance;
- validate the generated scenario through existing scenario-pack rules;
- preview the diff or generated candidate before any tracked write;
- require a Rust CLI action for any tracked scenario-pack update;
- record promotion evidence and generated-state policy.

Current v1 CLI flow:

```bash
cargo run -p ouroforge-cli -- scenario promote-draft <run-dir> \
  --project <project>/ouroforge.project.json \
  --scenario <scenario-id> \
  --output runs/drafts/<scenario-id>.json
cargo run -p ouroforge-cli -- scenario promote runs/drafts/<scenario-id>.json \
  --project <project>/ouroforge.project.json \
  --scenario-pack <pack-id> \
  --dry-run
cargo run -p ouroforge-cli -- scenario promote runs/drafts/<scenario-id>.json \
  --project <project>/ouroforge.project.json \
  --scenario-pack <pack-id>
```

The draft command writes generated draft state only. Dry-run reports before/after
pack hashes without writing. Promotion writes only the project-manifest
authorized scenario pack and a run-local `regression-promotions/*.json` record.
Dashboard and cockpit surfaces display those records read-only with copyable
dry-run commands; browser JavaScript must not generate drafts, promote, write
scenario packs, or execute CLI commands.

The milestone should prefer small deterministic fixtures and focused tests over
large generated run artifacts.

## Regression Run Matrix Boundary

Regression Run Matrix v1 summarizes project-bound scenario outcomes from local
generated run evidence. It groups by project, scenario pack, scenario id, and run
id, then exposes current status, last pass, last fail, evidence refs, and
available mutation/review/promotion context ids.

The matrix is a read model only. It skips legacy or malformed runs with explicit
reasons instead of inferring project context, and it treats missing scenario
results for declared pack scenarios as `pending`. It must not schedule CI, rerun
scenarios, promote scenarios, write scenario packs, add hosted analytics, or
store remote run state. Browser surfaces may render the exported matrix as
escaped read-only HTML only. Detailed semantics and generated-state policy are
recorded in `docs/regression-run-matrix-v1.md`.

## Journal and Studio Boundary

Journal and Studio surfaces may show proposal quality, review decisions,
review-gated application status, regression promotion candidates, regression
matrix results, and copyable CLI commands. They must remain read-only for
trusted state:

- no browser-side trusted file writes;
- no local command bridge;
- no auto-rerun;
- no auto-apply or auto-accept;
- no branch merge/rebase automation;
- no production editor or public compatibility claim.

Browser surfaces should display missing or malformed review/regression evidence
as warnings, not inferred passes.

Evidence-Backed Journal v2 adds the deterministic `journal-authoring-governance-v2`
section to connect proposal quality, decisions, applications, comparisons, and
regression promotions in generated `journal.md` files. See
`docs/evidence-backed-journal-v2.md` for missing/partial/malformed data
semantics and generated-state policy.

## Verification Policy for Follow-up Issues

Every fixed PR unit in this milestone must include:

- current issue number and PR unit id;
- exact authorized behavior;
- expected changed files;
- explicit non-goals still out of scope;
- focused tests for the changed behavior;
- generated-state audit;
- guardrail audit;
- drift audit;
- over-engineering audit;
- #1/#23 state;
- broad gates required by the issue body.

Issue-level verification must run on latest `main`, not only on the feature
branch. At minimum, closure evidence should include:

```bash
gh issue view <issue-number> --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Node dashboard/cockpit checks, project validate/run commands, scenario-pack
validation, compare commands, cargo audit, or public-readiness gates are required
only when the issue body scopes those surfaces. If a check cannot run, the final
issue comment must record why, the next-best validation used, and whether the
gap blocks completion.

## Closure Gates

Do not mark an Agentic Review & Regression Promotion v1 issue complete until:

- every fixed PR unit from that issue is merged in order;
- latest `main` has been pulled;
- issue-level verification passes on latest `main`;
- Definition of Done is audited;
- guardrails and explicit non-goals remain true;
- generated artifacts remain untracked;
- drift-prevention and over-engineering checklists are answered;
- #1 and #23 remain open unless a separate explicit replacement decision exists;
- the issue has a final evidence comment with merged PRs, verification, known
  gaps, generated-state audit, and completion rationale.

## Explicit Non-goals

Agentic Review & Regression Promotion v1 does not authorize:

- source-code mutation or source patch apply to the trusted main worktree;
- arbitrary source, dependency, CI/workflow, or build-script mutation;
- branch merge/rebase automation;
- browser-side trusted file writes;
- browser/local command bridge;
- auto-apply, auto-accept, auto-promote, auto-rerun, auto-merge, or hidden
  command execution;
- native export implementation;
- plugin runtime, dynamic loading, marketplace, or extension API;
- hosted/cloud/server/database/auth infrastructure;
- distributed QA/Elixir implementation;
- production editor, visual scripting system, public launch automation,
  compatibility guarantee, secure sandbox claim, production-ready claim, or
  Godot replacement claim;
- repository visibility change, package publishing, binary release, or actual
  public launch action.

## Follow-up Issue Roles

- #295 improves mutation proposals through `docs/mutation-proposal-quality-v2.md` so rationale, evidence refs, expected impact,
  and review hints are explicit before review decisions are recorded.
- #296 defines the review decision ledger for accept/reject/defer records without
  applying changes by itself.
- #297 makes accepted scene-only application require a valid review decision and
  preserves Rust validation, rollback metadata, and evidence provenance.
- #298 drafts and validates regression scenario candidates from failure or
  accepted-change evidence without browser writes or automatic pack merge.
- #299 runs or summarizes regression matrices across promoted scenarios and
  project contexts using generated evidence, not tracked run output.
- #300 upgrades journal/read-model context so proposal, decision, apply,
  rerun/compare, and regression promotion evidence stays connected.
- #301 surfaces review/regression status in Studio as read-only exported data and
  copyable commands only.
- #302 refreshes roadmap and #1 governance after the milestone while preserving
  #1 and #23 as open anchors unless a separate explicit replacement decision
  exists.

## Implementation Discipline

This document is a control artifact. It authorizes no product behavior changes.
Follow-up PRs should prefer narrow schemas, existing ledger/evidence helpers,
existing scenario-pack validation, and existing dashboard/Studio read-model
patterns before adding abstractions. Any scope movement must be documented in
the affected issue before implementation proceeds.

## Review Decision Ledger v1

See [Review Decision Ledger v1](review-decision-ledger-v1.md).
