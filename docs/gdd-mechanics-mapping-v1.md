# GDD Mechanics and Core Loop Mapping v1

Issue: #647

GDD Mechanics and Core Loop Mapping v1 maps extracted GDD requirement ids to supported Ouroforge capability references before prototype planning. It is not a prototype generator, not a source/apply contract, and not an asset creation contract. Its job is to make supported mechanics, partial support, unsupported mechanics, deferred work, contradictions, scene/level needs, asset needs, scenario needs, dependencies, gaps, and blocked reasons visible for review.

## Artifact boundary

The artifact is fixture-scoped under `examples/gdd-mechanics-mapping-v1/` and is validated by `GddMechanicsMappingArtifact` in `crates/ouroforge-core/src/gdd_mechanics_mapping.rs`.

Top-level fields include `schemaVersion`, `mappingId`, `status`, `sourceExtractionRef`, `requirementRefs`, `capabilityRefs`, `coreLoops`, `mappings`, and `boundary`. Each mapping row records requirement ids, support status, engine capability refs, behavior model refs, scene/level needs, asset needs, scenario needs, dependencies, conflicts, unsupported gaps, blocked reasons, recommendations, core loop refs, and evidence boundary.

## Validation rules

Rust/local validation fails closed when a mapping references an unknown requirement, capability, mapping, or core-loop id; a supported mapping lacks engine, behavior, scene/level, asset, or scenario needs; a supported mapping references a non-supported capability; an unsupported or deferred mapping lacks gaps, blocked reasons, or a downgrade/defer/placeholder recommendation; a partial mapping lacks visible gaps and recommendations; a contradictory mapping lacks blocked reasons; a core loop is overbroad; or boundary wording grants source/script mutation, asset generation, browser trusted writes, auto-apply, auto-merge, or broad game-generation authority.

The validator can produce a display-only read model summarizing counts and compatibility notes. The read model does not grant trusted persistence or review approval.

## Capability links

Mechanics mapping links to existing capability contracts rather than inventing new mechanics:

- Production 2D references: [`production-2d-engine-core-v1.md`](production-2d-engine-core-v1.md), [`scene-component-model-v2.md`](scene-component-model-v2.md), [`collision-physics-v2.md`](collision-physics-v2.md), and [`engine-expressiveness-v2.md`](engine-expressiveness-v2.md)
- 3D gate references: [`3d-capability-gate-v1.md`](3d-capability-gate-v1.md), [`3d-scene-graph-v1.md`](3d-scene-graph-v1.md), and [`3d-render-smoke-v1.md`](3d-render-smoke-v1.md)
- Gameplay logic references: [`gameplay-scripting-logic-system-v1.md`](gameplay-scripting-logic-system-v1.md), [`gameplay-behavior-model-v1.md`](gameplay-behavior-model-v1.md), [`gameplay-event-signal-system-v1.md`](gameplay-event-signal-system-v1.md), [`gameplay-state-machine-v1.md`](gameplay-state-machine-v1.md), and [`gameplay-ability-action-v1.md`](gameplay-ability-action-v1.md)
- Level designer references: [`agentic-scene-level-designer-v1.md`](agentic-scene-level-designer-v1.md), [`agentic-level-design-demo-v1.md`](agentic-level-design-demo-v1.md), and [`scenario-coverage-v10-agentic-level-design.md`](scenario-coverage-v10-agentic-level-design.md)
- Asset and scenario references: [`asset-manifest-v1.md`](asset-manifest-v1.md), [`asset-reference-integrity-v1.md`](asset-reference-integrity-v1.md), [`scenario-evaluator-v1.md`](scenario-evaluator-v1.md), and [`gdd-requirement-extraction-v1.md`](gdd-requirement-extraction-v1.md)

These links are evidence references only. Unsupported mechanics block, defer, or force visible downgrades; they are not silently implemented inside the mapping.

## Fixtures

Valid fixtures include supported, unsupported, partially supported, contradictory, and deferred mappings:

- `examples/gdd-mechanics-mapping-v1/mechanics.supported.fixture.json`
- `examples/gdd-mechanics-mapping-v1/mechanics.unsupported.fixture.json`
- `examples/gdd-mechanics-mapping-v1/mechanics.partial.fixture.json`
- `examples/gdd-mechanics-mapping-v1/mechanics.contradictory.fixture.json`
- `examples/gdd-mechanics-mapping-v1/mechanics.deferred.fixture.json`

Invalid fixtures cover missing behavior capability, unsupported mapping without recommendation, contradictory mapping without blocker, overbroad core loops, unsafe boundary wording, and unknown capability references.

## Governance and trust boundaries

This contract preserves the Milestone 12 separation between GDD, extracted requirements, mechanics mapping, feasibility, plans, drafts, task graph, review, apply, run evidence, and journal artifacts. GDD-derived output remains untrusted until Rust/local validation and later review-gated apply. Browser, dashboard, and Studio surfaces remain read-only or draft-only unless a separately scoped Rust/local trusted API owns persistence.

#1 remains the roadmap/governance anchor. #23 remains the memory/governance anchor. This issue does not close or modify either anchor.
