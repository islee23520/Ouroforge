# GDD Prototype Draft Bundle v1

Issue: #655

`gdd-prototype-draft-bundle-v1` is a review surface only for a scoped GDD-to-prototype proposal. It composes the GDD ref, requirement extraction, feasibility gate, scaffold plan, scene/level plan, behavior plan, asset plan, scenario drafts, task graph, expected evidence refs, source/license note refs, target hashes, validation status, and blocked reasons before any later review-gated apply path.

## Validation gates

Rust/local validation rejects unsafe refs, missing required components, duplicate components or target hashes, stale targets without blockers, missing/unsupported/contradictory components without blockers, missing scenario draft refs, missing expected evidence refs, missing asset/source notes, overbroad bundles, malformed hashes, and unsafe authority wording.

## Compatibility references

The bundle references existing contracts instead of merging their responsibilities: [`gdd-requirement-extraction-v1.md`](gdd-requirement-extraction-v1.md), [`gdd-mechanics-mapping-v1.md`](gdd-mechanics-mapping-v1.md), [`gdd-feasibility-gate-v1.md`](gdd-feasibility-gate-v1.md), [`gdd-project-scaffold-plan-v1.md`](gdd-project-scaffold-plan-v1.md), [`agent-generated-level-draft-v1.md`](agent-generated-level-draft-v1.md), [`behavior-draft-v1.md`](behavior-draft-v1.md), [`asset-manifest-v1.md`](asset-manifest-v1.md), and [`scenario-evaluator-v1.md`](scenario-evaluator-v1.md).

## Boundaries

This contract keeps GDD, extracted requirements, mechanics mapping, feasibility, scaffold plans, scene/level plans, behavior plans, asset plans, scenario drafts, task graph, review, apply, run evidence, and journal artifacts separate. Generated prototype bundles remain untrusted until Rust/local validation and later review-gated apply. Browser, dashboard, and Studio consumers remain read-only or draft-only for this read model.

No direct trusted writes, arbitrary source mutation, arbitrary script execution, dynamic code loading, command bridge, browser trusted write, auto-apply, auto-merge, asset generation, native export, plugin runtime, hosted/cloud behavior, production-ready claim, commercial readiness claim, current Godot replacement claim, or autonomous unrestricted game creation is added.

#1 remains the roadmap/governance anchor. #23 remains the memory/governance anchor.
