# Collision and Physics Rules v2

Collision and Physics Rules v2 is a bounded Runtime v1 expressiveness increment for the local 2D demo runtime. It adds an optional collision layer catalog, deterministic gravity/jump behavior, and scenario evidence hooks while preserving local-first, Rust-trusted, browser-observable boundaries.

## Fixtures

Canonical fixtures and focused checks:

```bash
cargo run -p ouroforge-cli -- scene validate examples/game-runtime/physics-rules-v2.json
node examples/game-runtime/physics.test.cjs
```

`examples/game-runtime/physics-rules-v2.json` contains a dynamic `player` collider, a static `floor`, and opt-in `collisionRules` layers (`world`, `actors`) used by collider groups/masks.

## Scene schema and validation

Scenes may opt into a collision layer catalog:

```json
{
  "collisionRules": {
    "version": "2",
    "defaultLayer": "world",
    "layers": [
      { "id": "world", "solid": true, "collidesWith": ["actors"] },
      { "id": "actors", "solid": true, "collidesWith": ["world"] },
      { "id": "triggers", "solid": false, "triggerOnly": true, "collidesWith": ["actors"] }
    ]
  }
}
```

Rust validation enforces:

- `collisionRules.version` is `"2"`
- layer IDs, `defaultLayer`, and `collidesWith` entries are safe path-like identifiers
- layer IDs and `collidesWith` entries are unique
- `defaultLayer`, collider `collisionGroup`, and collider `collisionMask` references point at declared layers when `collisionRules` exists
- each declared layer is either `solid` or `triggerOnly`

Legacy scenes without `collisionRules` remain backward-compatible: existing freeform collider groups/masks still validate as bounded identifiers.

## Runtime rules

The browser runtime applies a fixed-step, deterministic platformer subset:

- gravity (`1`) applies only to dynamic AABB collider entities
- fall speed is capped at `8`
- `components.input.jumpImpulse` works only when `allowedActions` includes `jump` and the entity is grounded
- grounded state is derived from solid contact normals and exposed as `getWorldState().physics.grounded`
- jump emits a bounded `runtime.physics.jump` event through `getEvents()`
- trigger/solid contacts remain observable through `getWorldState().collisions` and `collisionEvents`

This is not a general physics engine: there are no forces, rigid-body materials, rotations, slopes, continuous collision detection, joints, 3D physics, wall-clock simulation, or user-configurable physics plugins.

## Scenario and evidence hooks

Scenario runs now capture runtime events as a bounded artifact and support these relevant assertion targets:

```yaml
assertions:
  - collision_evidence:
      path: 0.type
      equals: runtime.collision.contact
  - physics_evidence:
      path: grounded.player
      equals: true
  - runtime_events:
      path: events
      countGreaterThan: 0
```

`collision_evidence` reads `world_state.collisions`, `physics_evidence` reads `world_state.physics`, and `runtime_events` reads the captured `getEvents()` artifact. All assertions remain bounded JSON-path checks against captured artifacts; no arbitrary expression evaluation or browser command bridge is introduced.

## Guardrails

Collision and Physics Rules v2 does not implement native export, source mutation, plugin runtime, hosted/cloud/server/auth, visual scripting, production editor behavior, public launch automation, or Godot replacement claims. Generated run evidence stays local under ignored `runs/`.
