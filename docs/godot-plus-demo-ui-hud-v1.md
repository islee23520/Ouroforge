# Godot-Plus Demo UI / HUD / Feedback v1

Issue: #785
Status: **GPD12.8 HUD/feedback contract.** This document records the UI/HUD and
feedback model for the Godot-Plus Demonstration Game v1 vertical slice
(Signal Gate / Collect and Exit), on the canonical fixture
`examples/playable-demo-v2/collect-and-exit/`. It builds on the GDD (#780),
scaffold (#781), core loop (#782), level set (#783), and behavior (#784). The
legacy `examples/godot-plus-demo-v1/` tree is superseded and is not used. #1 and
#23 remain open.

## HUD model

`hud-model.json` (schema `demo-hud-model-v1`) is a declarative read-only model
over the existing scene HUD entities. Each row binds an objective flag to a HUD
display; the live runtime exposes the bound state through
`componentModel.hudValues[*].flagValue`.

| Row | Entity | Binds | False → True display |
| --- | --- | --- | --- |
| Goal | `hud_goal` | `exit_reached` | "Collect key and exit" → "Exit reached" |
| Key | `hud_key` | `key_collected` | `0/1` → `1/1` |
| HP | `hud_health` | `player_alive` | `3/3` → `0/3` |

## Win / lose visibility

Game-state is derived from the same objective flags (no extra state needed):

| Condition | State |
| --- | --- |
| `player_alive == false` | `lost` (HP shows `0/3`) |
| `exit_reached == true` | `won` (Goal shows "Exit reached") |
| otherwise | `in-progress` |

So the HUD makes win (Goal complete), lose (HP zero), and objective progress (Key
count) visible to both players and agents.

## Feedback (no new assets)

To avoid asset bloat, feedback reuses existing runtime paths:

- **Visual** — the key entity is hidden on pickup via the existing `hideEntity`
  trigger action.
- **Audio** — the existing `collect_sound` (`collect.ogg`) audio intent fires
  (intent-only, muted browser playback); no new audio asset is added.

## Verification

```bash
node examples/playable-demo-v2/collect-and-exit/hud-smoke.test.cjs
```

`hud-smoke.test.cjs` asserts: every HUD model row maps to a live HUD row and
binds the right flag; the HUD reflects key collection and objective completion on
the win path; the lose-state display is derivable (`HP 0/3`); the key is hidden on
pickup and the `collect_sound` intent fires; and the read-only dashboard renders
the HUD evidence. It writes only to a temp dir outside the repository and fails
closed on any committed generated root.

## Boundaries

The HUD/feedback model is read-only and reuses existing scene HUD entities and
runtime feedback paths. It adds no new assets, no engine surface, no trusted
browser write, no production/native/store export, and no full Godot parity /
replacement / production-ready / commercial-release claim. #1 and #23 remain open.
