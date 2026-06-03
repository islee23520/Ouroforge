# Studio Authoring Surface v2: Expressive Scene Inspection

Issue #321 adds read-only expressive scene inspection to the static authoring cockpit.

## Evidence source

The panels consume `engine_summaries` from exported dashboard data. The Rust read model owns extraction from world-state evidence; browser JavaScript only renders escaped summaries.

## Surfaces

- Expressive scene inspection: component counts, entity components, trigger bindings, required flags, and HUD values.
- Collision/transition/event inspection: collision rules and events, scene transitions/reload status, animation entities, and audio/runtime events.

## Guardrails

- No browser file writes, local storage, IndexedDB, native shell, command bridge, hosted backend, or auto-apply path.
- No source scene mutation; persistent changes remain routed through Rust CLI validation.
- Missing or malformed summaries render warning/empty states.
- All dashboard-derived content is escaped before insertion into static markup.

## Verification

The authoring cockpit smoke test covers navigation, integrated rendering, malformed summary warnings, empty-state escaping, and XSS escaping for both expressive panels. Node syntax/tests and CI fast checks are the reproducible verification gate for this static surface.
