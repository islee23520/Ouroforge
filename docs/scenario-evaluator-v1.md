# Scenario/Evaluator v1 scope and contract

Scenario/Evaluator v1 upgrades Ouroforge's evidence-native QA loop from MVP
smoke checks into deterministic game-behavior evaluation. It exists to make Seeds
stronger executable contracts while preserving the project loop described in the
README: Seed -> Run -> Evidence -> Evaluation -> Journal -> Mutation proposal.

This document is a control artifact for issues #68 through #75. It authorizes no
product behavior by itself; each follow-up issue remains the implementation
contract for its own PRs.

## 1. Purpose and relation to the final goal

Issue #1 defines Ouroforge as an evidence-native game engine built around local,
inspectable Ouroboros loops. Scenario/Evaluator v1 supports that goal by making
QA results reproducible and artifact-linked. It does not attempt to become a
generic test framework, AI judge, visual testing platform, distributed QA system,
or public compatibility promise.

The v1 evaluator must answer bounded questions:

- What deterministic scenario ran?
- Which runtime evidence artifacts were captured?
- Which assertions passed or failed?
- Which verdict links explain the result?
- Which follow-up journal or mutation inputs can cite the evidence?

## 2. Current MVP baseline

The current MVP can validate Seeds, run local browser workers, capture runtime
world state and frame stats, execute simple scenario steps/assertions, produce a
deterministic verdict, update a journal, and produce mutation proposals for
failed runs. Runtime v1 (#60-#67) adds the local browser-game capabilities that
Scenario/Evaluator v1 may consume: scene schema v1, deterministic replay,
collision, snapshot/restore, local assets, animation state, audio event evidence,
and the composed Runtime v1 demo seed.

Scenario/Evaluator v1 must build on this baseline instead of duplicating Runtime
v1 behavior.

## 3. Scenario/Evaluator v1 capabilities

Allowed v1 capabilities are bounded to:

- replay binding between Scenario DSL steps and Runtime v1 input replay;
- console log, performance metric, and bounded CDP trace evidence capture;
- richer deterministic assertions over captured artifacts;
- visual regression hooks that persist artifacts and metadata without becoming a
  full visual testing platform;
- multi-scenario suite execution summaries;
- before/after run comparison summaries;
- an integration demo that proves these capabilities through the existing
  Seed -> Run -> Evidence -> Verdict -> Journal path.

All capabilities must remain deterministic, local-first, evidence-linked, and
reviewable through generated run artifacts.

## 4. Issue order and dependency graph

Implement in this order:

1. #68 Scenario/Evaluator v1 Scope and Contract — documentation only.
2. #69 Scenario Replay Binding — depends on Runtime v1 replay (#61) and current
   Scenario DSL execution.
3. #70 Console, Performance, and CDP Trace Evidence — depends on the existing CDP
   boundary and local browser workers.
4. #71 Richer Assertion Model — depends on stable artifact paths and evidence
   targets from #69/#70.
5. #72 Visual Regression Hooks — depends on browser screenshots and richer
   assertion/evidence contracts; hook-only, not a complete visual platform.
6. #73 Multi-Scenario Suite Execution — depends on deterministic scenario result
   aggregation from #69-#72.
7. #74 Before/After Run Comparison — depends on stable run summaries and suite
   outputs from #73.
8. #75 Scenario/Evaluator v1 Integration Demo — composes #69-#74 and closes the
   Scenario/Evaluator v1 milestone.

No later issue may be implemented early. If a dependency is not merged, the next
issue must stop or use the approved PR/merge discipline rather than stacking
hidden scope.

## 5. Runtime v1 boundary

Runtime v1 owns game behavior and probe state. Scenario/Evaluator v1 may invoke
and inspect Runtime v1 features, but it must not reimplement them.

Runtime v1 responsibilities:

- browser runtime state transitions;
- scene schema v1 and scene validation;
- deterministic input effects;
- collision, snapshot/restore, asset loading, animation, and audio event state;
- local demo surfaces under `examples/game-runtime`.

Scenario/Evaluator v1 responsibilities:

- scenario syntax and execution contracts;
- evidence capture requests and artifact registration;
- deterministic assertion evaluation;
- verdict summaries and evidence links;
- suite and comparison summaries.

## 6. Evidence categories and verdict-linking expectations

Allowed evidence categories:

- replay evidence: serialized replay inputs and execution references;
- console logs: bounded logs captured from the browser/CDP session;
- performance metrics: deterministic metric summaries suitable for local QA;
- bounded CDP trace summaries: small, local summaries rather than full tracing
  infrastructure;
- world-state snapshots: probe JSON and snapshot/restore artifacts;
- visual regression artifacts: screenshots, metadata, and hook outputs;
- multi-scenario suite summaries: per-scenario and aggregate pass/fail data;
- before/after comparison summaries: deterministic diffs over selected run
  artifacts.

Evaluator outputs must link to concrete evidence artifact paths. A claim without
an artifact reference is not a v1 evaluator claim. Verdicts remain deterministic
JSON summaries; no AI semantic judgment is authorized.

## 7. Determinism requirements

Scenario/Evaluator v1 must be reproducible under local execution:

- input replays are explicit and ordered;
- assertions compare concrete values or bounded summaries;
- captured evidence paths are registered in run artifacts;
- generated timestamps may appear in filenames, but evaluator meaning must not
  depend on wall-clock timing;
- browser/device output that is flaky or environment-dependent must be reduced to
  bounded evidence summaries before it influences a verdict.

## 8. Language and runtime boundary

Default implementation language is Rust for Scenario DSL schema, evaluator logic,
artifact contracts, CLI, suite execution, and deterministic comparison. JavaScript
is allowed only where browser/runtime capture requires page-side probes or CDP
interaction. No Playwright, Elixir/distributed orchestration, server, database,
cloud service, or hosted QA system is authorized in Scenario/Evaluator v1.

## 9. Non-goals and drift risks

Non-goals:

- no generic test framework;
- no AI semantic evaluator or LLM judge;
- no distributed QA architecture;
- no visual testing platform beyond bounded hooks and artifacts;
- no Runtime v1 feature implementation;
- no UI/dashboard implementation unless a follow-up issue explicitly authorizes
  a read-only display surface;
- no public compatibility or production QA claim.

Primary drift risks and countermeasures:

- Drift into AI judgment: require deterministic assertions and artifact links.
- Drift into Runtime v1 duplication: keep runtime behavior in Runtime v1 and only
  consume probe/evidence outputs.
- Drift into visual platform scope: persist hooks/artifacts only; leave advanced
  diffing policies to explicit future issues.
- Drift into distributed QA: keep execution local-first and single-repo.

## 10. PR decomposition summary for follow-up issues

- #69 should add replay binding in the smallest schema/execution PRs needed to
  connect Scenario DSL to Runtime v1 replay evidence.
- #70 should add bounded console/performance/CDP trace evidence capture without a
  tracing platform.
- #71 should extend deterministic assertion targets/operators while preserving
  clear failure output.
- #72 should add visual regression hooks and artifacts only; no full visual test
  platform.
- #73 should add multi-scenario suite aggregation and summary evidence.
- #74 should add before/after run comparison over selected deterministic outputs.
- #75 should add an integration demo and documentation proving #69-#74 through the
  existing evidence-native loop.

Every follow-up PR must include verification output and guardrail checks in the
PR body. Generated run artifacts remain local state and must not be committed.

## 11. Integration demo evidence

The Scenario/Evaluator v1 integration demo is documented in
`docs/scenario-evaluator-v1-demo.md` and uses
`seeds/scenario-evaluator-v1-demo.yaml`. The demo composes the completed #69-#74
capabilities through the existing local evidence-native loop; it does not add new
Scenario/Evaluator behavior or broaden the maturity claim.
