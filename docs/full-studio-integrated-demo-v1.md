# Full Studio Editor Integrated Demo v1

Issue #774 defines a fixture-scoped integrated demo for the Full Studio Editor
v1 milestone. The demo is a local static cockpit over tracked fixture data and
generated-state references; it is not full Godot editor parity and not a
production editor.

The tracked fixture lives at
`examples/authoring-cockpit/integrated-demo-v1.fixture.json`.

## Covered Surfaces

The smoke composes the same surfaces the cockpit page uses:

- project overview and project-bound run context;
- scene tree and entity inspector;
- live browser preview canvas area;
- asset inspector;
- scenario and playtest evidence panel;
- export/package inspection panel;
- plugin/extension inspection panel;
- command palette;
- Studio draft authoring preview;
- visual diff preview;
- Safe Source Apply handoff preview metadata.

## Boundaries

The demo is read-only or draft-only. It may show copyable command text and
generated evidence references, but it must not execute commands, write trusted
source files, persist browser draft state, publish, deploy, sign, upload, install
plugins, execute plugin code, browse a marketplace, mutate CI/workflows, or
perform credentialed operations.

Trusted source mutation remains gated by Safe Source Apply: validated preview,
sandbox evidence, independent review, stale-target checks, rollback metadata,
allowlisted verification, and post-apply comparison.

Generated demo outputs belong under ignored roots such as
`runs/generated/full-studio-integrated-demo/` unless a future issue explicitly
scopes another tiny deterministic fixture.

#1 and #23 remain open as roadmap and repo-memory anchors.
