# GDD Prototype Scope and Feasibility Gate v1

Issue: #648  
Status: feasibility gate artifact, deterministic state rules, fixtures, and display read model contract.

`gdd-feasibility-gate-v1` prevents overbroad or unsupported GDD-derived work from becoming implementation work. Prototype planning starts only after feasibility passes or an accepted bounded slice is recorded. The gate is not a prototype generator and is not prototype generation authority and does not implement scaffold, scene, behavior, asset, scenario, apply, run, or Studio behavior.

## Artifact shape

Top-level fields are `schemaVersion`, `gateId`, `state`, `mechanicsMappingRef`, `targetPrototypeSize`, `supportedMechanics`, `requiredPriorMilestones`, `acceptanceCriteriaRefs`, `scenarioPlanRefs`, `riskFlags`, `knownGaps`, optional `sliceRecommendation`, and `boundary`. `targetPrototypeSize` carries the scope limits (`maxScenes`, `maxLevels`, `maxEntities`, `maxAssets`, `maxMechanics`).

Allowed states are `pass`, `fail`, `defer`, `downgrade`, and `blocked`. The deterministic state rule defines pass/fail/defer behavior. Passing gates must have complete prior milestones and no blocking risk flags. Deferred or downgraded gates require visible risk flags or known gaps plus a downgrade, defer, or bounded slice recommendation.

## Validation gates

Rust/local validation rejects missing mechanics mappings, unsupported or overbroad limits, unclear acceptance criteria, blocking risk flags on passing gates, missing scenario plans, missing prior capability prerequisites, deferred/downgraded gates without recommendations, and unsafe authority wording.

## Fixtures

Valid/visible-state fixtures:

- `examples/gdd-feasibility-gate-v1/feasibility.feasible.fixture.json` (`pass`)
- `examples/gdd-feasibility-gate-v1/feasibility.infeasible.fixture.json` (`fail`)
- `examples/gdd-feasibility-gate-v1/feasibility.deferred.fixture.json` (`defer`)
- `examples/gdd-feasibility-gate-v1/feasibility.downgraded.fixture.json` (`downgrade`)
- `examples/gdd-feasibility-gate-v1/feasibility.overbroad.fixture.json` (`fail`)
- `examples/gdd-feasibility-gate-v1/feasibility.blocked.fixture.json` (`blocked`)

Invalid fixtures under `examples/gdd-feasibility-gate-v1/invalid/` cover missing mappings, unsupported mechanics without risk, overbroad scope without risk, unclear acceptance, missing scenario plan, missing prior prerequisite, defer without slice recommendation, and unsafe wording.

## Boundaries

This contract keeps GDD, extracted requirements, mechanics mapping, feasibility, plans, drafts, task graph, review, apply, run evidence, and journal artifacts separate. GDD-derived output remains untrusted until Rust/local validation and later review-gated apply. Browser, dashboard, and Studio surfaces remain read-only or draft-only for this read model.

No prototype generation, hidden implementation work, source/script mutation, command bridge, browser trusted write, auto-apply, auto-merge, native export, plugin runtime, hosted/cloud behavior, generated proprietary asset claim, commercial readiness claim, or current Godot replacement claim is added.

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor.
