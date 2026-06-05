# GDD Project Scaffold Plan v1

Issue: #649

`gdd-project-scaffold-plan-v1` is a preview-only project scaffold plan artifact for feasible GDD slices. Preview first: it records project identity, planned seed files, scene files, asset manifest placeholders, scenario pack refs, generated-state roots, source-like fixture refs, expected preview commands, target paths, stale targets, and blocked reasons before any later trusted apply path. It is not a prototype generator and provides no direct trusted writes.

## Validation gates

Rust/local validation rejects unsafe paths, generated-root collisions, duplicate file targets, unsupported or deferred template needs without blockers, missing feasibility pass for ready plans, stale targets without blockers, overbroad file generation, and direct-write command wording.

## Compatibility references

The plan references existing project scaffold and validation contracts instead of bypassing them: [`project-scaffold-v1.md`](project-scaffold-v1.md), [`project-manifest-v1.md`](project-manifest-v1.md), [`asset-manifest-v1.md`](asset-manifest-v1.md), [`scenario-evaluator-v1.md`](scenario-evaluator-v1.md), and [`gdd-feasibility-gate-v1.md`](gdd-feasibility-gate-v1.md).

## Boundaries

This contract keeps GDD, extracted requirements, mechanics mapping, feasibility, scaffold plans, drafts, task graph, review, apply, run evidence, and journal artifacts separate. GDD-derived scaffold output remains untrusted until Rust/local validation and later review-gated prototype apply. Browser, dashboard, and Studio consumers remain read-only or draft-only for this read model.

No direct file writes, source/script mutation, browser trusted write, command bridge, auto-apply, auto-merge, generated proprietary asset claim, production game, native export, plugin runtime, hosted/cloud behavior, commercial readiness claim, or current Godot replacement claim is added.

#1 remains the roadmap/governance anchor. #23 remains the memory/governance anchor.
