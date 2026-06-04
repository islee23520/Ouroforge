# 3D Animation Playback Evidence v1

Issue #602 begins bounded 3D animation playback evidence with a source-like
scene model for transform clip references and playback state. This first slice
adds schema and validation only; runtime playback, probe emission, and scenario
assertion integration are staged in later #602 PR units.

## Scene Contract

`scene3d.animationClips[]` defines a bounded local transform clip catalog. Each
clip records:

- `id`;
- `targetNodeId` referencing an existing `scene3d.nodes[]` id;
- `channel`: `translation`, `rotation`, or `scale`;
- positive `durationFrames`;
- optional `looped` state;
- strictly increasing `keyframes[]`, each with a bounded integer `frame` and
  vector `value`.

`scene3d.animationStates[]` records read-only playback state for evidence and
inspection. Each state records:

- `clipId`;
- `targetNodeId`;
- `channel`;
- `currentFrame` and `currentTimeMs`;
- `playing` and `looped` booleans;
- optional `missingClipWarning` when a state intentionally preserves a warning
  for a missing clip;
- optional `malformedClipWarnings` for bounded display-only diagnostics.

Rust validation rejects duplicate clip ids, missing target nodes, unsupported
channels, non-positive durations, empty or oversized keyframes, non-increasing
keyframes, out-of-range frames, zero scale keyframe axes, state/clip target or
channel drift, negative playback counters, and missing clip states that do not
carry an explicit warning.

## Boundary

This is transform clip/state evidence for small deterministic local 3D scenes.
It does not add skeletal animation authoring, animation graph editing,
retargeting, IK, broad importer behavior, production animation tooling, native
export, plugin runtime, hosted/cloud behavior, source apply, auto-merge, or a
Godot replacement claim.

Generated runtime playback evidence, screenshots, run outputs, dashboard data,
and local tool state remain untracked unless a later issue explicitly scopes a
tiny deterministic fixture as source-like data.
