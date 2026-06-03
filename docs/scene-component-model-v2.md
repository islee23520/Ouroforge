# Scene Component Model v2

Scene Component Model v2 is an additive Runtime v1 scene extension for small, deterministic 2D examples. It keeps the existing `schemaVersion: "1"` scene shape and adds optional component payloads that are validated, hash-stable, and observable through the browser runtime probe.

## Fixture

The canonical fixture is:

```text
examples/game-runtime/scene-components-v2.json
```

Validate it locally with:

```bash
cargo run -p ouroforge-cli -- scene validate examples/game-runtime/scene-components-v2.json
node examples/game-runtime/components-v2.test.cjs
```

The fixture covers:

- `status`: bounded hit point fields plus string `flags` and `states`
- `input`: keyboard/arrows/WASD scheme metadata, `moveSpeed`, optional `jumpImpulse`, and `allowedActions`
- `trigger`: deterministic overlap/interact/enter metadata with bounded `onEnter` actions
- `goalFlag`: named objective flag plus optional display label
- `cameraTarget`: numeric weight plus optional dead-zone size
- `uiText`: bounded display text with optional role and flag binding

Component defaults may provide `status` and `input`. Runtime normalization applies those defaults to entities that omit the component, just like existing transform/velocity/size defaults.

## Validation and edit compatibility

Rust scene validation rejects malformed component v2 values before runtime use:

- trigger/action kinds outside the supported vocabularies
- duplicate or unsafe path-like flag/action identifiers
- negative hit points, non-positive maximum hit points, or hit points above max
- non-positive input speeds or camera target weights/dead zones
- oversized or control-character display text

The scene edit API exposes only explicit bounded scalar paths:

```text
components.status.hitPoints
components.status.maxHitPoints
components.input.moveSpeed
components.input.jumpImpulse
components.cameraTarget.weight
components.uiText.text
```

Those paths edit existing optional components only. They do not create missing components, do not mutate source code, and remain covered by post-edit validation plus deterministic scene hashing.

## Runtime and probe compatibility

`examples/game-runtime/runtime.js` preserves v2 component payloads in `window.__OUROFORGE__.getWorldState()` and adds a probe-facing `componentModel` summary:

```json
{
  "version": "2",
  "counts": { "status": 3, "input": 3, "trigger": 1, "goalFlag": 1, "cameraTarget": 1, "uiText": 1 },
  "entities": [
    { "entityId": "player", "components": { "status": {}, "input": {}, "cameraTarget": {} } }
  ],
  "goalFlags": { "alive": true, "coin_collected": false }
}
```

Runtime behavior stays intentionally small and evidence-first:

- player movement uses `components.input.moveSpeed` when `allowedActions` permits `move`
- collision trigger events may set/clear goal flags or hide entities according to bounded `onEnter` actions
- `components.uiText.text` is rendered as simple canvas text when the owning sprite/layer is visible
- `status`, `goalFlag`, and `cameraTarget` are preserved for probe/debug evidence

## Non-goals and guardrails

Scene Component Model v2 is not visual scripting, a native export pipeline, a plugin/runtime marketplace, a hosted/cloud/server/auth feature, or a Godot replacement claim. It introduces no trusted browser command bridge and no browser-side source-write path. Generated run evidence remains local under `runs/` and untracked.
