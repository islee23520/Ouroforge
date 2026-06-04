# Agent-Generated Level Draft v1

Issue: #637 - Agent-Generated Level Draft v1.

Agent-Generated Level Draft v1 records a complete level proposal assembled from
a level intent, a scene generation plan, tilemap and placement drafts,
reachability evidence, objective proof expectations, difficulty/pacing evidence,
and visual/semantic diff evidence.

The artifact is generated and untrusted. It is a review input only:
review-gated apply is required before any trusted state can change.

## Artifact Shape

The `agent-generated-level-draft-v1` artifact includes:

- stable draft, intent, plan, target scene, tilemap draft, placement draft,
  reachability, objective proof, heuristic, and diff refs;
- target hashes for stale-target detection;
- author/source metadata with `untrusted: true`;
- tilemap, entity placement, objective placement, and encounter placement
  sections;
- proposed operations with partial, missing-evidence, and unsupported states;
- expected constraint, reachability, objective proof, preview, and read-model
  evidence under `evidence/agent-level-drafts/<draft-id>/`;
- validation status, blocked reasons, and guardrails.

Malformed summaries, duplicate operations, unsafe refs, contradictory sections,
missing evidence expectations, unsupported draft kinds, stale targets, and
blocked drafts are rejected or represented as explicit visible states.

## Read Model

The read model reports section count, operation count, partial count,
missing-evidence count, unsupported count, linked evidence refs, target hash
refs, blocked reasons, and boundary text.

The boundary is read-only untrusted generated level draft data: review-gated
apply required, no scene writes, no trusted apply, no browser command bridge, no
auto-apply, no auto-merge, no autonomous full game generation, and no production
editor claim.

## Non-Goals

- No autonomous full game generation.
- No production editor or full visual level editor claim.
- No browser trusted writes, command bridge, local server bridge, auto-apply, or
  auto-merge.
- No trusted apply or unrestricted source mutation.
- No native export, plugin runtime, hosted/cloud behavior, production-ready
  claim, or Godot replacement claim.
