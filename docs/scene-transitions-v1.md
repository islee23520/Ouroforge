# Scene Transitions v1

Scene Transitions v1 lets a local project declare bounded scene/level links and
observe transition behavior through Rust-authored evidence.

## Scene schema

Scenes may include `sceneTransitions`:

```json
{
  "sceneTransitions": [
    { "id": "to_level_2", "toScene": "scenes/level-2.scene.json", "label": "Level 2" }
  ]
}
```

Validation rules:

- `id` is a bounded path-component-style identifier and must be unique within
  the scene.
- `toScene` must be a safe project-relative JSON path.
- Project manifest validation rejects transition targets that are not listed in
  `ouroforge.project.json` `scenes`.
- `label` is optional bounded display text.

## Runtime and probe evidence

The browser runtime exposes declared transitions and transition events through
`window.__OUROFORGE__.getWorldState()`:

- `sceneTransitions`: the current scene's declared transition rows.
- `transitionEvents`: bounded success/failure rows from transition attempts.
- `sceneId`: the current scene id after a successful transition.

The runtime API is `window.__OUROFORGE__.transition(id)`. It only follows a
transition declared by the current scene, records a transition event, and does
not write files, execute commands, load plugins, or stream arbitrary content.

## Scenario assertions and read models

Scenario DSL supports:

```yaml
steps:
  - transition:
      id: to_level_2
assertions:
  - transition_evidence:
      path: 0.status
      equals: succeeded
```

Scenario execution writes a `transition_evidence` artifact with:

- `currentSceneId`
- `declaredTransitions`
- `transitionEvents`

Dashboard/Studio read models expose these as read-only transition summaries:

- `declaredTransitionCount`
- `declaredTransitions`
- `transitionEventCount`
- `transitions`
- reload status fields

## Non-goals

Scene Transitions v1 does not add native export, hosted runtime streaming,
plugin loading, browser-side trusted writes, source mutation, visual scripting,
asset bundles, or a production editor workflow.
