# Evidence Fidelity & Trust Boundary Hardening v1

Evidence Fidelity & Trust Boundary Hardening v1 is the next local-first
consolidation milestone after Project Workspace Loop v1. Ouroforge now has an
engine demo, scenario/evaluator loop, evolve lifecycle, Studio cockpit, Authoring
Loop v2, and project workspace flow. The next bottleneck is not more engine
surface area or public launch mechanics; it is making each authored action's
evidence, trusted writes, runtime observations, replay inputs, and rerun context
harder to corrupt and easier to audit.

The milestone preserves the current loop:

> project action -> trusted artifact write -> browser/runtime observation ->
> replayable scenario evidence -> semantic verification -> journal/mutation
> review -> reproducible rerun context

It keeps Ouroforge local-first, Rust-trusted, and browser-read-only for trusted
state.

## Completed Baseline

Evidence Fidelity v1 builds on these completed or refreshed foundations:

- Runtime v1 (`docs/runtime-v1.md`, `docs/runtime-v1-demo.md`) — deterministic
  local run execution and evidence capture for seeds.
- Scenario/Evaluator v1 (`docs/scenario-evaluator-v1.md`,
  `docs/scenario-evaluator-v1-demo.md`) — scenario validation and verdict
  evidence.
- Evolve Loop v1 (`docs/evolve-loop-v1.md`, `docs/evolve-loop-v1-demo.md`) —
  local proposal lifecycle and journal evidence.
- Studio v1/v2/v3 (`docs/studio-v1.md`, `docs/studio-v2-cockpit.md`,
  `docs/studio-v3-project-workspace-cockpit.md`) — static browser cockpit
  surfaces that display exported read models without trusted writes or command
  execution.
- Engine Expansion v1 (`docs/engine-expansion-v1.md`,
  `docs/engine-expansion-v1-demo.md`) — broader playable-template engine
  features with scenario coverage.
- Authoring Loop v2 (`docs/authoring-loop-v2.md`,
  `docs/scene-edit-transactions.md`, `docs/run-comparison-v2.md`,
  `docs/scene-only-mutation-v2.md`) — scene edit transactions, QA binding,
  semantic comparison, and scene-only mutation lifecycle.
- Project Workspace Loop v1 (`docs/project-workspace-loop-v1.md`,
  `docs/project-manifest-v1.md`, `docs/project-run-v1.md`,
  `docs/project-comparison-v1.md`, `docs/project-mutation-loop-v1.md`) —
  scaffolded project manifests, project-scoped runs, comparison, mutation
  review, and Studio v3 integration.
- Public-readiness refresh (`docs/public-readiness-audit.md`,
  `docs/public-demo-evidence.md`, `docs/public-launch-checklist.md`) —
  governance evidence and conservative public wording without launch automation.

The active project anchors remain #1 and #23. This milestone must not close,
replace, or silently implement either anchor.

## Milestone Goal

Make the local authoring loop's evidence chain trustworthy enough that future
agents and maintainers can answer:

1. What trusted state changed?
2. Which layer was allowed to write it?
3. Which artifacts are generated evidence rather than source-like project data?
4. Which runtime/browser observations support the verdict?
5. How can the run be reproduced with the same project, seed, scenario pack,
   environment, and command context?
6. Which gaps remain explicitly non-blocking or assigned to a later issue?

## Follow-up Issue Dependency Order

Implement follow-up issues in this order unless a concrete blocker is documented
in the affected issue before changing scope:

1. #285 — Evidence Fidelity & Trust Boundary Hardening v1 Scope and Contract.
2. #286 — Shared Transaction Output Safety Guard.
3. #287 — Trusted Artifact Write Policy.
4. #288 — Runtime Probe Contract v2.
5. #289 — Input Replay and Deterministic Scenario Evidence v2.
6. #290 — Openchrome/CDP Evidence Fidelity v2.
7. #291 — Reproducible Run Command Context v1.
8. #292 — Studio Evidence Fidelity Surfaces.
9. #293 — Roadmap/#1 Governance Refresh after Evidence Fidelity v1.

The order intentionally hardens trusted writes before runtime/browser evidence,
then adds reproducibility context, then updates read-only Studio surfaces and
roadmap governance.

## Known Non-blocking Gap Assigned to #286

Project Workspace Loop v1 left a known non-blocking safety gap around
`scene edit --transaction-output`: aliasing or hard-link style output paths can
make an edit transaction appear to write a separate artifact while actually
referencing an unsafe or ambiguous destination. Evidence Fidelity v1 treats that
as a first-class follow-up, but this scope issue does not implement the guard.
#286 owns the behavior change, focused tests, and closure evidence.

## Trust Boundary Categories

### Rust-trusted source-like writes

Rust CLI commands own trusted writes to source-like project data:

- project manifests;
- scene JSON/YAML files;
- scenario packs;
- scene-only mutation application targets;
- explicit transaction outputs;
- comparison or journal artifacts that are source-like only when a command
  explicitly declares them as tracked project data.

Every trusted write must validate destination policy, reject generated-state
roots, and record enough provenance for later audit.

### Generated evidence and local runtime state

Generated evidence remains local and untracked by default:

- `runs/` outputs;
- dashboard export JSON;
- comparison outputs;
- generated screenshots or browser observations;
- replay artifacts;
- temporary scaffold projects;
- tool cache/state under `.omx/` or other local runtime folders.

Generated artifacts may be referenced in issue/PR comments as evidence, but they
must not become tracked source unless an issue explicitly authorizes a small,
deterministic fixture.

### Browser read models

Browser/Studio surfaces may display exported read models, command strings,
provenance, warnings, and evidence links. They must not:

- write trusted files;
- execute commands;
- start a local command bridge;
- auto-rerun tests;
- auto-apply proposals;
- auto-accept mutations;
- merge or rebase source changes.

Browser evidence is advisory observation unless a Rust-owned command validates
and records it.

### Runtime and probe observations

Runtime probes and openchrome/CDP observations may enrich evidence with frame,
DOM, screenshot, or interaction metadata. They must remain reproducible,
structured, and clearly labeled as observation data. They do not replace Rust
validation or scenario verdicts.

### Source mutation boundary

Source mutation preview/design work remains blocked from applying patches to the
trusted main worktree. Preview artifacts may be generated or displayed only when
an issue explicitly scopes them as inert, reviewable evidence.

## Verification Policy for Follow-up Issues

Each PR unit must include:

- the current issue number and PR unit id;
- the exact authorized behavior;
- expected changed files;
- explicit non-goals still out of scope;
- generated-state audit;
- guardrail audit;
- drift audit;
- over-engineering audit;
- #1/#23 state;
- focused tests for the changed behavior;
- broad gates required by the issue body.

Issue-level closure must run on latest `main`, not just the feature branch. When
relevant, closure evidence should include:

```bash
gh issue view <issue-number> --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Node, dashboard, Studio, browser, cargo audit, or project-run checks are required
when the issue body scopes those surfaces. If a check cannot run, the issue
comment must record why, the next-best validation used, and whether the gap is a
closure blocker.

## Closure Gates

Do not close any Evidence Fidelity v1 issue until:

- every fixed PR unit from that issue is merged in order;
- latest `main` is pulled;
- issue-level verification passes on latest `main`;
- Definition of Done is audited;
- guardrails and explicit non-goals are still true;
- generated artifacts remain untracked;
- drift-prevention and over-engineering checklists are answered;
- #1 and #23 remain open unless a separate maintainer-approved governance
  replacement decision explicitly allows otherwise;
- the issue has a final evidence comment with merged PRs, verification, known
  gaps, generated-state audit, and closure rationale.

## Explicit Non-goals

Evidence Fidelity & Trust Boundary Hardening v1 does not authorize:

- native export implementation;
- plugin runtime, dynamic loading, marketplace, or extension API;
- hosted/cloud/server/database/auth infrastructure;
- distributed QA/Elixir implementation;
- browser-side trusted file writes;
- browser command bridge or local server command bridge;
- auto-apply, auto-accept, auto-promote, auto-rerun, auto-merge, or hidden
  command execution;
- source patch apply to the trusted main worktree;
- branch merge/rebase automation for source mutation;
- dependency, CI/workflow, or build-script mutation unless a later design issue
  explicitly authorizes discussion only;
- production editor, visual scripting system, public launch automation,
  compatibility guarantee, secure sandbox claim, production-ready claim, or
  Godot replacement claim;
- repository visibility change, package publishing, binary release, or actual
  public launch action.

## Follow-up Issue Roles

- #286 hardens transaction-output destination safety.
- #287 defines and enforces trusted artifact write policy across source-like and
  generated-state paths.
- #288 updates the runtime probe contract so observations are structured,
  bounded, and attributable.
- #289 strengthens replay and deterministic scenario evidence through `docs/input-replay-evidence-v2.md`.
- #290 improves openchrome/CDP evidence fidelity through `docs/openchrome-cdp-evidence-fidelity-v2.md` while keeping browser
  observation non-authoritative.
- #291 records reproducible command context for reruns and audits through `docs/reproducible-run-command-context-v1.md`.
- #292 exposes the hardened evidence chain in Studio without browser writes or
  command execution.
- #293 refreshes roadmap and #1 governance after the milestone without closing
  #1 or #23 by accident.

## Implementation Discipline

This document is a control artifact. It authorizes no product behavior changes.
Follow-up PRs should prefer small, reviewable, test-backed changes and reuse
existing evidence, validation, and read-model patterns before adding new
abstractions. Any scope movement must be documented in the affected issue before
implementation proceeds.
