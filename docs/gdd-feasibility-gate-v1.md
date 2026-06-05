# GDD Prototype Scope and Feasibility Gate v1

Issue: #648  
Status: feasibility gate artifact, deterministic state rules, fixtures, and display read model contract.

`gdd-feasibility-gate-v1` prevents overbroad or unsupported GDD-derived work from becoming implementation work. Prototype planning starts only after feasibility passes or an accepted bounded slice is recorded. The gate is not prototype generation authority and does not implement scaffold, scene, behavior, asset, scenario, apply, run, or Studio behavior.

## Artifact shape

Top-level fields include `schemaVersion`, `gateId`, `status`, `sourceRequirementExtractionRef`, `sourceMechanicsMappingRef`, `targetPrototypeSize`, `limits`, `supportedMechanics`, `requiredPriorMilestones`, `acceptanceCriteria`, `scenarioPlanRefs`, `assetSourceRisks`, `riskFlags`, `blockedRisks`, `knownGaps`, optional `sliceRecommendation`, `deterministicRule`, and `boundary`.

Allowed states are `pass`, `fail`, `defer`, `slice`, and `blocked`. The deterministic rule must define pass/fail/defer behavior. Passing gates must have complete prior milestones and no blocked or asset-source risks. Deferred or sliced gates require visible risks/gaps plus a downgrade, defer, or bounded slice recommendation.

## Validation gates

Rust/local validation rejects missing mechanics mappings, unsupported or overbroad limits, unclear acceptance criteria, asset/source risks on passing gates, missing scenario plans, missing prior capability prerequisites, deferred/sliced gates without recommendations, and unsafe authority wording.

## Fixtures

Valid/visible-state fixtures:

- `examples/gdd-feasibility-gate-v1/feasibility.pass.fixture.json`
- `examples/gdd-feasibility-gate-v1/feasibility.fail.fixture.json`
- `examples/gdd-feasibility-gate-v1/feasibility.defer.fixture.json`
- `examples/gdd-feasibility-gate-v1/feasibility.slice.fixture.json`
- `examples/gdd-feasibility-gate-v1/feasibility.blocked.fixture.json`

Invalid fixtures under `examples/gdd-feasibility-gate-v1/invalid/` cover missing mappings, overbroad scope, unclear acceptance, asset risk on pass, missing scenario plan, missing prior prerequisite, defer without slice recommendation, and unsafe wording.

## Boundaries

This contract keeps GDD, extracted requirements, mechanics mapping, feasibility, plans, drafts, task graph, review, apply, run evidence, and journal artifacts separate. GDD-derived output remains untrusted until Rust/local validation and later review-gated apply. Browser, dashboard, and Studio surfaces remain read-only or draft-only for this read model.

No prototype generation, hidden implementation work, source/script mutation, command bridge, browser trusted write, auto-apply, auto-merge, native export, plugin runtime, hosted/cloud behavior, generated proprietary asset claim, commercial readiness claim, or current Godot replacement claim is added.

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor.
