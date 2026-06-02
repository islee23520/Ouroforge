# Studio v1 Scope and Contract

Studio v1 is the bounded upgrade path from Ouroforge's current static authoring
cockpit into a local, evidence-native editor surface. It lets a human inspect and
act on the same artifacts used by agents without becoming a full IDE, hosted app,
native shell, unsafe browser file writer, or Godot replacement.

This document is planning/control only. It authorizes no product behavior by
itself; each follow-up GitHub issue remains the implementation contract for its
own PRs.

Related context:

- `README.md` for the current local MVP workflow and maturity boundaries;
- `docs/architecture.md` for the Seed -> Run -> Evidence -> Verdict -> Journal
  -> Mutation Proposal loop and the current UI boundaries;
- `docs/runtime-v1.md` and `docs/runtime-v1-demo.md` for the completed runtime
  capability and demo evidence baseline;
- `docs/scenario-evaluator-v1.md` and
  `docs/scenario-evaluator-v1-demo.md` for scenario, evidence, suite, and run
  comparison contracts;
- `docs/evolve-loop-v1.md` and `docs/evolve-loop-v1-demo.md` for mutation
  lifecycle artifacts and manual review boundaries.

## 1. Purpose and relation to #1 final goal

Issue #1 defines Ouroforge as an evidence-native game engine built around local,
inspectable Ouroboros loops. Studio v1 supports that final goal by making the
loop visible and reviewable in a static browser UI:

```text
Seed -> Run -> Evidence -> Evaluation -> Journal -> Mutation proposal -> Review
```

Studio v1 must answer bounded human questions:

- Which runs exist and what evidence did they produce?
- Which journal findings explain a verdict?
- Which mutation lifecycle records are pending review?
- Which replay or preview state is currently being inspected?
- Which scene edits are preview-only, and which have passed Rust validation?
- How do two runs differ according to deterministic comparison artifacts?

Studio v1 must not answer unbounded questions such as whether a game is
production-ready, whether a mutation should be merged automatically, or whether
Ouroforge is a general-purpose replacement for established engines.

## 2. Current static cockpit baseline

The current repository already has two local static UI examples:

- `examples/evidence-dashboard` is a read-only browser surface over exported run
  data. It does not mutate run artifacts.
- `examples/authoring-cockpit` is a static cockpit prototype for the minimal
  game-runtime scene. Browser edits are in-memory previews and show the Rust
  `ouroforge scene edit` command needed for validated persistence.

Studio v1 builds on this baseline by adding bounded artifact inspection and
validated action surfaces. It should reuse existing static UI patterns and Rust
artifact/export contracts before introducing any new abstraction.

## 3. Studio v1 surfaces and dependency graph

Implement Studio v1 in this order:

1. #83 Studio v1 Scope and Contract — this documentation/control issue only.
2. #84 Studio v1: Run and Evidence Browser — read-only run/evidence navigation
   over exported local data.
3. #85 Studio v1: Journal Viewer — read-only journal/verdict/mutation context
   display built on the run browser.
4. #86 Studio v1: Mutation Review UI — local static review controls that surface
   Evolve Loop v1 manual review commands; no automatic apply/merge.
5. #87 Studio v1: Replay Controls — deterministic replay inspection controls
   backed by Runtime/Scenario replay evidence.
6. #88 Studio v1: Live Preview Controls — preview-only browser controls for the
   local runtime surface; no trusted persistence from browser JavaScript.
7. #89 Studio v1: Scene Editing with Rust Validation — persistent scene edits
   routed through Rust validation and the existing scene edit contract.
8. #90 Studio v1: Run Comparison UI — read-only display of before/after
   comparison artifacts.
9. #91 Studio v1: Integration Demo — composes #84-#90 after they are merged and
   verified; it must not backfill missing feature work.

Dependency meaning:

- #84 is the data/navigation foundation for later Studio surfaces.
- #85 depends on #84 because journals and verdict context are run-scoped.
- #86 depends on #85 and Evolve Loop v1 because mutation review decisions must be
  explained by journal/verdict/evidence artifacts.
- #87 depends on Runtime v1 replay and Scenario/Evaluator v1 replay binding.
- #88 depends on the local runtime preview surface and must remain preview-only
  until a Rust-validated persistence path is explicitly invoked.
- #89 depends on the existing Rust scene read/edit model and must preserve Rust as
  the trusted validator for persistent edits.
- #90 depends on Scenario/Evaluator before/after comparison artifacts.
- #91 is an integration proof only; it should not introduce new core capability.

Sequential execution rule: even when dependencies conceptually allow parallel
work, implementation follows the active workflow's one-PR-at-a-time merge
discipline unless a later issue explicitly changes that rule.

## 4. Static data/export architecture

Studio v1 remains local and static:

```text
Rust CLI / core reads local artifacts
  -> validates and exports bounded JSON/read models
  -> static browser UI loads exported JSON and local demo pages
  -> trusted persistence, when authorized, goes back through Rust validation
```

Allowed data sources:

- run metadata under `runs/<run-id>/run.json`;
- evidence indexes and evidence artifacts under `runs/<run-id>/evidence/`;
- verdicts, scenario results, suite summaries, and comparison artifacts;
- journals and mutation lifecycle artifacts;
- validated scene files and Rust-generated scene read/edit outputs;
- generated dashboard or Studio export JSON that is ignored unless a specific
  issue authorizes committing a fixture.

The browser UI may load static JSON exports and local demo pages. It must not own
a database, hidden backend state, account model, or direct trusted project file
writer.

## 5. Rust validation boundary

Rust owns trusted validation and persistence for Studio v1:

- scene schema validation;
- scene edit command validation;
- run/evidence/journal/mutation read models;
- mutation review command generation and decision artifact validation;
- comparison artifact parsing and summarization;
- any future persistent edit path explicitly authorized by an issue.

Browser JavaScript may:

- display exported data;
- filter, select, sort, and expand local read models;
- preview runtime state in memory;
- prepare a command or request payload for Rust validation;
- show the result of a Rust-validated edit or review command.

Browser JavaScript must not:

- directly write trusted scene/project/run files;
- bypass Rust validation for persistent edits;
- imply that preview-only changes are saved;
- auto-apply mutation patch drafts;
- create commits, merges, GitHub actions, or release actions.

## 6. Evidence-native UI principles

Every Studio v1 surface should preserve evidence-native behavior:

- show artifact paths or ids for claims;
- distinguish generated local state from committed repository state;
- distinguish preview-only UI state from Rust-validated persistent state;
- preserve deterministic ordering where it affects verification;
- prefer small explicit read models over broad editor abstractions;
- cite unsupported claims instead of presenting them as verdicts;
- keep the UI inspectable through Node syntax/tests and local static serving.

A Studio v1 UI claim without an artifact reference is only UI context, not an
Ouroforge evidence claim.

## 7. Generated/local state rules

Do not commit generated or local runtime/tool state unless an active issue
explicitly authorizes a fixture:

- `runs/`;
- `target/`;
- `examples/evidence-dashboard/dashboard-data.json`;
- generated Studio export JSON;
- `.openchrome/`;
- `.omc/`;
- `.omx/`.

Generated artifacts may be used as verification evidence in PR bodies and issue
comments. They should stay local and ignored.

## 8. Non-goals and drift risks

Studio v1 does not authorize:

- a full IDE;
- hosted Studio, cloud service, server, database, auth, or accounts;
- native app shell or desktop packaging;
- direct browser file writes for trusted state;
- visual scripting;
- plugin systems or marketplaces;
- collaborative editing;
- Playwright adoption;
- Elixir/distributed orchestration;
- automatic mutation patch application, commit, merge, or GitHub automation;
- Godot replacement, production editor, public compatibility, or launch claims.

Primary drift risks and countermeasures:

- Drift into unsafe persistence: require Rust validation for every persistent
  edit and label browser-only changes as preview-only.
- Drift into hosted/editor platform scope: keep architecture static/local and
  reject server/database/auth concepts.
- Drift into Godot replacement language: describe Studio v1 as a local prototype
  over evidence artifacts, not a mature engine editor.
- Drift into autonomous mutation acceptance: keep Evolve Loop v1 manual review
  commands explicit and never auto-merge.
- Drift into hidden state: route claims through local artifacts and generated
  read models.

## 9. PR decomposition summary for follow-up issues

Each follow-up issue controls its own PR decomposition. The expected shape is:

- #84 should add the smallest run/evidence read model and static browser surface
  needed for read-only navigation.
- #85 should add journal/verdict/mutation context display over the run browser;
  it must not review or apply mutations.
- #86 should add mutation review UI affordances that surface explicit Rust CLI
  review commands; it must not auto-accept, apply, commit, or merge.
- #87 should add replay inspection controls backed by deterministic replay
  artifacts; it must not implement recording UX or unrelated runtime behavior.
- #88 should add live preview controls for local runtime state; it must not write
  trusted project files directly from the browser.
- #89 should add Rust-validated scene editing persistence and UI integration; it
  must not broaden the scene schema beyond the issue contract.
- #90 should display deterministic run comparison artifacts; it must not invent
  subjective visual/gameplay judgments.
- #91 should add an integration demo and documentation proving #84-#90 through
  the local evidence-native loop; it must not add missing upstream behavior.

Every Studio v1 PR must include verification output, guardrail checks,
over-engineering checks, and drift-prevention checks in the PR body. Generated
run/export artifacts remain local state unless the active issue explicitly
commits a fixture.

## 10. Closing checklist for Studio v1 planning

Before closing #83, verify:

- this file exists as the canonical Studio v1 scope contract;
- #84-#91 are ordered with bounded dependencies and acceptance targets;
- the UI architecture remains local/static;
- every persistent edit path is Rust-validation-gated;
- generated/local state rules are documented;
- no product implementation code was added;
- no server/database/cloud/auth/native shell/Playwright/Elixir scope was
  introduced;
- Studio v1 remains framed as a local evidence-native editor surface, not a full
  IDE or production engine editor.
