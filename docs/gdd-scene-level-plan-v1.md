# GDD Scene/Level Plan v1

Issue: #650

`gdd-scene-level-plan-v1` is a non-mutating bridge from feasible GDD requirements and mechanics mapping into existing Agentic Scene and Level Designer contracts. It links requirement ids and mechanics mapping ids to `level-intent-v1`, `scene-generation-plan-v1`, objective placement plans, progression ordering, target scene/tilemap refs, and expected scenario/evidence proof refs before any draft bundle or apply step exists.

## Validation gates

Rust/local validation rejects missing requirement or mechanics mapping ids, unsafe refs, unsupported or deferred level-design capabilities without visible blockers, contradictory level goals, missing objective proof expectations, stale targets without blockers, duplicate ids/targets, overbroad plans, and wording that implies direct writes or unrestricted generation.

## Compatibility references

The contract reuses existing level-design artifacts rather than replacing them: [`level-intent-v1.md`](level-intent-v1.md), [`scene-generation-plan-v1.md`](scene-generation-plan-v1.md), [`agent-generated-level-draft-v1.md`](agent-generated-level-draft-v1.md), [`review-gated-level-apply-v1.md`](review-gated-level-apply-v1.md), [`gdd-mechanics-mapping-v1.md`](gdd-mechanics-mapping-v1.md), [`gdd-feasibility-gate-v1.md`](gdd-feasibility-gate-v1.md), and [`gdd-project-scaffold-plan-v1.md`](gdd-project-scaffold-plan-v1.md).

## Boundaries

This artifact keeps GDD, extracted requirements, mechanics mapping, feasibility, scaffold plans, scene/level plans, draft bundles, task graph, review, apply, run evidence, and journal artifacts separate. GDD-derived output remains untrusted until Rust/local validation and later review-gated apply. Browser, dashboard, and Studio consumers remain read-only or draft-only for this read model.

No direct scene or tilemap writes, arbitrary source mutation, arbitrary script execution, dynamic code loading, plugin loading, browser trusted write, command bridge, local server bridge, auto-apply, auto-merge, self-approval, uncontrolled asset generation, generated proprietary asset claim, production game, shipped-game, commercial readiness, current Godot replacement, production-ready engine, native export, plugin runtime, hosted/cloud behavior, or autonomous unrestricted game creation is added.

#1 remains the roadmap/governance anchor. #23 remains the memory/governance anchor.
