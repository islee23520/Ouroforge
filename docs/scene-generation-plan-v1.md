# Scene Generation Plan v1

Issue: #629 - Scene Generation Plan Artifact v1.

Scene Generation Plan v1 is an advisory and untrusted planning artifact. It
explains how future scene or level drafts are expected to satisfy a validated
Level Intent v1 artifact before any layout change is proposed. It does not generate
scenes, does not write trusted files, does not apply drafts, and does not make
browser or Studio output authoritative.

## Contract

The `scene-generation-plan-v1` artifact includes:

- stable plan and intent ids;
- target scene and optional tilemap refs;
- preview summary text for read-only display;
- proposed zones and their linked intent objectives;
- placement strategy with required and forbidden zone refs;
- required assets and entities inherited from the linked intent;
- scenario checks to generate later;
- expected evidence paths under `evidence/scene-generation-plans/<plan-id>/`;
- target hashes for stale-target detection;
- planned, stale, or blocked status with blocked reasons when stale or blocked.

## Validation Boundary

Rust validation owns trusted interpretation. Validation rejects missing intent
ids, unsupported zone kinds, unsafe target refs, missing required assets or
entities when checked against intent, contradictory placement strategy, stale or
blocked plans without reasons, malformed target hashes, and malformed expected
evidence paths.

The plan remains advisory. It is separate from intent, drafts, evidence, visual
or semantic diffs, review decisions, and apply records. Later issues must add
their own validation before any draft or apply behavior exists.

## Read Model

The read model summarizes plan id, intent id, target scene ref, optional target
tilemap ref, status, blocked reasons, zone count, scenario-check count, expected
evidence refs, target hash refs and count, preview summary, and a boundary
string. It is read-only display data: advisory and untrusted, does not generate
drafts, does not write trusted files, and carries no command execution, browser
write, auto-apply, auto-merge, or quality guarantee authority.
