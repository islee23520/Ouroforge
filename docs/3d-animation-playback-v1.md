# 3D Animation Playback Evidence v1

Issue #602 adds bounded 3D animation playback evidence with a source-like
scene model for transform clip references and playback state. The current
runtime slice advances transform-only clips deterministically in the browser
runtime and exposes read-only `scene3dAnimation` world-state evidence. Dedicated
scenario assertion integration remains staged in a later #602 PR unit.

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

## Runtime Playback

`examples/game-runtime/runtime.js` advances `scene3d.animationStates[]` during
`step()` using fixed-frame integer playback. The runtime applies the active
clip value to the target node's `localTransform` channel (`translation`,
`rotation`, or `scale`) and records `scene3dAnimation` evidence with:

- schema version `ouroforge.scene3d-animation-evidence.v1`;
- frame id, scene id, clip/state counts, active state count, warning count;
- per-state clip id, target node, channel, current frame/time, playing/looped
  state, interpolated vector value, status, and warnings;
- `runtime.scene3d.animation.state` events for read-only probe inspection.

Playback is intentionally local and deterministic. It does not fetch assets,
execute commands, persist trusted files, or infer animation quality.

## Boundary

This is transform clip/state evidence for small deterministic local 3D scenes.
It does not add skeletal animation authoring, animation graph editing,
retargeting, IK, broad importer behavior, production animation tooling, native
export, plugin runtime, hosted/cloud behavior, source apply, auto-merge, or a
Godot replacement claim.

Generated runtime playback evidence, screenshots, run outputs, dashboard data,
and local tool state remain untracked unless a later issue explicitly scopes a
tiny deterministic fixture as source-like data.
