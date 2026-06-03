# Agentic Loop Orchestration v1

Agentic Loop Orchestration v1 is the next local-first control milestone after
Evidence Fidelity & Trust Boundary Hardening v1 and Agentic Review & Regression
Promotion v1.

The completed baseline can already validate project runs, collect evidence,
compare runs, propose mutations, record review decisions, apply accepted
scene-only mutations, promote regression scenarios, summarize regression matrix
state, and expose Journal/Studio review state. Those capabilities are still
separate commands and artifacts. This milestone makes the loop plan and loop
state first-class local artifacts without making Ouroforge fully autonomous.

```text
project select -> run scenario pack -> collect evidence -> compare -> propose
mutation -> record review decision -> apply accepted scene-only mutation ->
rerun -> promote regression -> summarize next action
```

The milestone remains local-first, Rust-trusted, browser-read-only for trusted
state, and generated-state aware.

## Completed baseline

Agentic Loop Orchestration v1 builds on these completed foundations:

- Evidence Fidelity & Trust Boundary Hardening v1
  (`docs/evidence-fidelity-trust-boundary-v1.md`);
- Agentic Review & Regression Promotion v1
  (`docs/agentic-review-regression-promotion-v1.md`);
- project workspace, run, comparison, mutation, journal, and Studio read-model
  docs listed in `docs/roadmap.md`.

The active governance anchors remain:

- #1 — broad vision and roadmap anchor;
- #23 — repo-memory/design context anchor.

Both anchors must remain open unless a separate explicit maintainer-approved
replacement decision exists.

## Milestone goal

Make the evidence-native loop itself inspectable and resumable through local
artifacts:

- loop plan model: what steps are intended and why (`docs/authoring-loop-plan-v1.md`);
- dry-run sequencer: what would run, with no trusted/project-state writes (`docs/authoring-loop-dry-run-v1.md`);
- Rust-trusted step runner: explicit execution for allowed local steps;
- resume/failure recovery: visible incomplete/failed state;
- evidence bundle: portable summary of run, review, comparison, and promotion
  evidence;
- agent handoff contract: enough state for the next agent/human to continue;
- Studio loop cockpit: static read-only display of loop state;
- roadmap/#1 refresh after the milestone completes.

## Follow-up issue dependency order

Implement follow-up issues in this order unless a concrete blocker is documented
in the affected issue before changing scope:

1. #304 — Authoring Loop Plan Model v1.
2. #305 — Authoring Loop Runner v1: Dry-Run Sequencer.
3. #306 — Authoring Loop Execution v1: Rust-Trusted Step Runner.
4. #307 — Loop Resume and Failure Recovery v1.
5. #308 — Loop Evidence Bundle v1.
6. #309 — Agent Handoff Contract v1.
7. #310 — Studio Loop Cockpit v1.
8. #311 — Roadmap and #1 Governance Refresh after Loop Orchestration v1.

The order intentionally defines static plan shape before dry-run sequencing,
dry-run before trusted execution, execution before resume semantics, resume
before evidence bundle/handoff, and backend/read-model state before Studio.

## Trust and execution boundaries

Loop orchestration is a control layer over existing local commands and artifacts.
It does not widen trusted authority.

Allowed in follow-up issues when explicitly scoped:

- create local loop-plan artifacts;
- preview a sequence of local steps without trusted/project-state writes;
- execute only Rust-owned, allowlisted local steps;
- record step state, failure state, and resume metadata;
- collect evidence refs into a generated bundle;
- display loop state in Studio as escaped read-only data.

Not allowed in this milestone:

- browser trusted file writes;
- browser/local command bridge;
- hidden shell execution from browser JavaScript;
- full autonomous apply, promote, merge, or release;
- source-code mutation;
- CI/workflow/dependency/build-script mutation;
- hosted/cloud/server/auth orchestration;
- native export implementation;
- plugin runtime or marketplace;
- distributed QA/Elixir implementation;
- production editor, public launch automation, or Godot replacement claims.

## Authoring Loop Plan Model v1

#304 defines the first concrete artifact in this milestone: Authoring Loop Plan
Model v1 (`docs/authoring-loop-plan-v1.md`). The plan is data-only. It records a
single loop id, project/Seed/scenario-pack context, ordered bounded steps,
expected artifacts, required decisions, rollback refs, per-step status, optional
status transitions, and generated-state roots. Validation rejects duplicate ids,
unknown step kinds, unsafe paths, invalid ordering, missing required inputs, and
invalid status transitions before future orchestration layers can consume the
plan.

The plan model intentionally does not execute commands, mutate scenes, promote
regressions, trigger browser actions, or add a Studio surface by itself. Runtime
loop plans remain generated local state; only explicitly scoped deterministic
fixtures under `examples/authoring-loop-plan-fixtures/` are tracked.

## Dry-run-first policy

Every loop step that could later affect trusted state should have a dry-run or
preview representation before execution is added. A dry-run must show:

- step id and kind;
- required inputs and generated outputs;
- trusted writer, if any, for the future execution path;
- evidence refs expected to justify execution;
- explicit non-goals and skipped actions;
- missing prerequisites as warnings or blocked state.

Dry-run artifacts are generated state and should remain untracked unless a
future fixture-scoped issue explicitly authorizes a tiny deterministic fixture.
Dry-run previews may write explicitly scoped local generated preview artifacts
under ignored/untracked generated-state paths, but they must not execute trusted
actions or write trusted/project state.

## Authoring Loop Dry-Run v1

#305 adds the inert dry-run sequencer (`docs/authoring-loop-dry-run-v1.md`).
`ouroforge loop dry-run <plan>` validates a plan, prints ordered step summaries,
shows prerequisite labels, reports missing project/run/proposal/decision/rollback
artifacts, and displays command text as copyable inert data. Dashboard and Studio
surfaces may render attached `loop_dry_run` data read-only; browser UI does not
execute command text or write trusted state.

Dry-run reports are generated local artifacts unless explicitly fixture-scoped.
The checked-in ready/blocked dry-run examples are deterministic fixtures only.

## Explicit step execution policy

When follow-up issues add execution, every trusted action must remain explicit:

- Rust CLI owns trusted reads/writes;
- browser UI may show escaped copyable commands only;
- review-gated scene application still requires accepted review decision and
  existing validation/provenance checks;
- regression promotion still uses draft/dry-run/promotion through Rust CLI;
- source-code mutation remains out of scope until a later design gate.

## Resume and failure recovery policy

Loop state should make interruption visible, not hidden:

- incomplete steps remain pending or blocked;
- failed steps preserve error text, inputs, and evidence refs;
- reruns should not overwrite prior failure evidence silently;
- resume should require compatible plan/run/project context;
- handoff artifacts should tell the next operator what happened and what remains.

## Evidence bundle and handoff policy

Evidence bundles and handoff records should be generated summaries of existing
trusted artifacts. They should link to evidence, verdicts, proposals, decisions,
applications, comparisons, promotions, journals, and matrix state where present.
They must not invent missing context, hide malformed artifacts, or become a
hosted audit service.

## Verification policy for follow-up issues

Every fixed PR unit in this milestone must include:

- current issue number and PR unit id;
- exact authorized behavior;
- expected changed files;
- explicit non-goals still out of scope;
- focused tests for changed behavior;
- generated-state audit;
- no-browser-writes/no-command-bridge audit;
- #1/#23 state;
- issue-required broad gates.

Do not mark a follow-up issue complete until:

- all fixed PR units are merged in order;
- latest `main` has been pulled;
- issue-level verification passes on latest `main`;
- Definition of Done is audited;
- guardrails/non-goals/over-engineering/drift checks pass;
- generated artifacts remain untracked;
- #1 and #23 remain open;
- final issue comment records PRs, verification, known gaps, and closure
  rationale.

## Generated-state policy

Loop plans, dry-runs, step records, resume files, evidence bundles, and handoff
records are generated local artifacts unless a future issue explicitly scopes a
small deterministic fixture. Keep generated local paths ignored/untracked by
default:

- `runs/`
- `target/`
- `.omx/`
- `.omc/`
- `.claude/`
- `.openchrome/`
- dashboard exports

## Closure criteria for this scope issue

This scope issue is complete when:

- this canonical contract exists;
- follow-up dependency order is explicit;
- dry-run, execution, resume, evidence bundle, handoff, and Studio boundaries are
  documented;
- verification and closure gates are documented;
- no CLI/runtime/browser/product behavior is added;
- #1 and #23 remain open.
