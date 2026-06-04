# Studio 2D Engine Inspection Surface v1

This document closes the remaining documentation and boundary-audit portion of
#593. The Studio cockpit may inspect Production 2D runtime and evidence state
from Rust-exported dashboard data, but it remains a static read-only browser
surface. It is not a trusted writer, command bridge, local server bridge,
production editor, visual scripting system, native export path, plugin runtime,
hosted service, or Godot replacement claim.

## Inspection Panels

The Studio cockpit composes these Production 2D panels from
`engine_summaries`, generated run evidence, and existing dashboard read models:

- **Camera/layer inspection** shows active camera, viewport, layer ordering,
  parallax, camera participation, and world-to-screen samples.
- **Render breakdown inspection** shows renderable draw order, render queue
  rows, tilemap draw counts, missing tile refs, and absence diagnostics.
- **Collision/transition/event inspection** shows collision rules, physics
  contact summaries, transition events, animation entities, VFX events, audio
  intent events, and browser audio limitation warnings.
- **Input action mapping** shows mapped actions, active actions, raw-key
  evidence, missing/unmapped/duplicate action diagnostics, and binding
  conflicts.
- **Runtime save/state inspection** shows state ids, digests, snapshots,
  save/load events, and replay digest comparisons.
- **Runtime profiler inspection** shows frame id, scene/scenario id, timing
  samples, budget samples, entity/render/collision/media counts, and budget
  violations.

All rendered values are escaped before insertion into the page. Missing,
malformed, empty, and hostile read-model data must render as warnings or empty
states instead of crashing or becoming executable HTML.

## Trusted Boundary

Trusted validation, persistence, source-like fixture validation, generated
evidence writing, project/run binding, and CLI behavior remain Rust/local
responsibilities. The cockpit can display inert command text, but browser
JavaScript must not execute commands, write files, persist trusted state,
control the runtime as trusted authority, upload/fetch assets, apply mutations,
merge branches, rerun tests, or bridge to a local server.

Generated runs, dashboard exports, screenshots, temp projects, preview outputs,
and local tool state remain untracked unless a later issue explicitly scopes a
tiny deterministic fixture as source-like data.

## Verification

Focused coverage lives in `examples/authoring-cockpit/cockpit.test.cjs` and
asserts the Production 2D panels render read-only data, escape hostile values,
show malformed/empty states, and preserve inert command boundaries. The
dashboard companion coverage remains in
`examples/evidence-dashboard/dashboard.test.cjs`.

Before closing #593, run:

```bash
gh issue view 593 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

The closure evidence must confirm #1 and #23 remain open and must avoid current
Godot replacement, production-ready, broad compatibility-stable, secure-sandbox,
native export, plugin runtime, hosted/cloud, autonomous launch, browser trusted
write, command bridge, or unrestricted source-apply claims.
