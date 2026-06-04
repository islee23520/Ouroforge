# 3D Collision and Physics Capability v1

Issue #601 adds bounded 3D collision evidence for explicit `sceneKind: "3d"`
demo scenes. The path supports deterministic overlap checks for scoped box
colliders and trigger volumes. It is capability-gate evidence, not a full 3D
physics engine.

## Scene Contract

`scene3d.colliders[]` defines the trusted source-like collider catalog. Each
collider has:

- `id`;
- `shape: "box"`;
- `body`: `static`, `dynamic`, `kinematic`, or `trigger`;
- positive `size` with `x`, `y`, and `z` axes;
- optional `offset`;
- optional `trigger`, `sensor`, `disabled`, `collisionGroup`, and
  `collisionMask` fields.

Nodes attach colliders through `colliderRef` or a bounded `collider` component
reference. Rust validation rejects missing catalog references, unsupported
shapes, unsupported bodies, non-positive box sizes, unsafe group/mask strings,
and malformed metadata.

## Runtime Evidence

The browser runtime emits `scene3dCollision` with schema version
`ouroforge.scene3d-collision-evidence.v1`. The summary records collider counts,
active/disabled counts, contact and trigger counts, invalid collider warnings,
event rows, and a conservative boundary string.

Runtime event types are:

- `runtime.scene3d.collision.contact`;
- `runtime.scene3d.collision.trigger`.

3D collision events are exported in world-state `collisions` so existing
`collision_evidence` scenario assertions can catch contact and trigger
regressions. The 2D grounded and trigger processors continue to consume only 2D
collision events.

## Inspection Surfaces

Dashboard exports use the snake_case `scene3d_collision` read model. Browser
dashboard and Studio/cockpit surfaces render exported 3D contact, trigger, and
invalid-collider evidence as escaped read-only inspection data. They do not gain
trusted persistence, command execution, source mutation, or local bridge
authority.

## Boundary

This capability is limited to deterministic overlap/contact evidence for small
local 3D demo scenes. It does not add a full 3D physics engine, rigidbody
simulation parity, ragdolls, joints/constraints, vehicles, character-controller
maturity, broad 3D engine compatibility, native export, hosted/cloud behavior,
plugin runtime, source apply, or a Godot replacement claim.

Generated runs, screenshots, dashboard data, temp projects, and local tool state
remain untracked unless a later issue explicitly scopes a tiny deterministic
source-like fixture.
