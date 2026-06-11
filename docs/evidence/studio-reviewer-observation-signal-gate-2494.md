# Signal Gate Relay Studio reviewer observation evidence (#2494)

Issue: #2494
M130 phase: #2391 first playable / Signal Gate Relay
Manual gap ledger: `m130-2391-manual-studio-launch`
Stable fixture: `examples/production-usability-gate-v111/studio-reviewer-observation.fixture.json`

## Closure classification

Closure classification: product-observed complete for the bounded Studio reviewer observation handoff only.

This evidence makes the manual Studio launch and reviewer observation step for Signal Gate Relay visible as local product-surface evidence. It records a reproducible read-only Studio handoff with copyable Rust CLI commands and transcript template fields. It does not claim browser trusted writes, command bridges, self-approval, auto-apply, auto-merge, commercial readiness, native export, hosted collaboration, Godot parity, or public release automation.

## Reviewer reproduction steps

1. From the repository root, start the local static server:

   ```bash
   python3 -m http.server 8000 --bind 127.0.0.1 --directory .
   ```

2. Open the Studio authoring cockpit (read-only inspection surface):

   `http://127.0.0.1:8000/examples/authoring-cockpit/`

3. Confirm the cockpit renders project/run/evidence panels from exported dashboard data when available, and that command strings are display-only (not executed by the browser).

4. Open the Signal Gate Relay runtime preview surface:

   `http://127.0.0.1:8000/examples/game-runtime/?scene=/examples/playable-demo-v2/signal-gate-dogfood/scenes/signal-gate-relay.scene.json`

5. Copy and run the Rust CLI commands referenced in the fixture `copyableCommands` array (paths under `examples/authoring-cockpit/commands/`) from a terminal — trusted validation and runs remain Rust/local only:

   - `examples/authoring-cockpit/commands/signal-gate-dogfood-project-validate.json`
   - `examples/authoring-cockpit/commands/signal-gate-dogfood-seed-validate.json`
   - `examples/authoring-cockpit/commands/signal-gate-dogfood-project-run.json`
   - `examples/authoring-cockpit/commands/signal-gate-dogfood-dashboard-export.json`

6. Record reviewer observation using the fixture `manualSteps.fields` transcript template (UTC time, panels visible, project context, commands reviewed, runtime preview opened, explicit `browserTrustedWriteAttempted: false`).

## Gate linkage

- Production usability gate manual gap: `m130-2391-manual-studio-launch` in `examples/production-usability-gate-v111/gate.fixture.json` (phase #2391).
- Dogfood project manifest: `examples/playable-demo-v2/signal-gate-dogfood/ouroforge.project.json` (`projectRef`: `signal-gate-dogfood`).

## Generated-state audit

Generated runs, dashboard exports, screenshots, and browser profiles remain under ignored roots unless a later issue explicitly scopes a tracked fixture change. This handoff indexes stable fixture and documentation paths only.

#1 and #23 remain open governance anchors.