# Proposal Workbench Model v1

Proposal Workbench Model v1 is the #2375/M125 data contract for evidence-linked
agent gameplay proposals.

## Required fields

A proposal must include:

- problem evidence refs by `runId`, local artifact path, and digest;
- finding category from the #2350 taxonomy lane;
- hypothesis;
- bounded scene/asset/behavior diff scope;
- expected impact;
- mandatory risk level, rationale, and mitigations;
- mandatory rollback plan and rollback refs;
- reviewer requirements;
- non-goals; and
- guardrails.

## Safe Source Apply compatibility

`safe_source_apply_review_section()` converts the proposal into a review-section
shape containing proposal id, evidence refs, target paths, reviewer requirements,
risk, rollback, and forbidden authority. M126 can embed this section without a
translation layer.

## Forbidden authority

The schema has no self-apply flag and no command surface. Validation rejects
hidden command, browser trusted write, self-apply, self-approval, auto-apply,
auto-merge, dependency install, publish/deploy/upload, remote URL, production
maturity, and Godot-replacement claims.

## Boundary

This issue is contract-complete only. UI, live dogfood proposal capture, and
quality-gate review are deferred to later M125 issues.
