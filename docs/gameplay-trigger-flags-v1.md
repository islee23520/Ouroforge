# Gameplay Trigger and Flag Evidence v1

Gameplay Trigger and Flag System v1 is a local-first, evidence-native gameplay objective layer. Scenes may declare `gameplayRules.flags`, entities may use `trigger`, `goalFlag`, `status.flags`, and HUD `uiText.bindFlag`, and runtime evidence exposes outcomes through `goalFlags`, trigger/collision events, and bounded scenario assertions.

## Evidence contract

- Rust validation owns the source schema. When `gameplayRules` is present, trigger flag references must point at declared flags.
- The browser runtime remains observable-only: it initializes declared flags, applies trigger actions during local runtime stepping, and records bounded runtime events. It does not write trusted files or execute local commands.
- Scenario assertions should prefer `world_flags` for objective state, and `runtime_events` or collision evidence for event presence.
- Dashboard, journal, and Studio surfaces display Rust-authored summaries from generated world-state evidence. They are read-only projections and do not infer semantic game quality.

## Summary fields

`engine_summaries.gameplay` contains:

- `declaredFlagCount` — flags declared in `gameplayRules.flags`.
- `worldFlagCount`, `trueFlagCount`, `falseFlagCount` — observed runtime `goalFlags` state.
- `triggerEntityCount`, `goalFlagEntityCount` — component counts from the runtime component model or entity fallback.
- `triggerCollisionEventCount` — observed `runtime.collision.trigger` events in world-state collision history.
- `trueFlags` and `flags` — bounded flag IDs and values for review.

## Non-goals

This is not a production editor, visual scripting system, plugin API, native export path, hosted service, account system, or Godot replacement. Browser dashboard and Studio views remain static/read-only and must not gain trusted writes or command bridges.
