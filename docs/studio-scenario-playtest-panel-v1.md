# Studio Scenario and Playtest Panel v1

Issue: #765
Roadmap anchor: #1 (Milestone: Full Studio Editor v1 — integrated local
authoring UX foundation, not full Godot editor parity).
Status: read-only scenario/playtest evidence inspection; no autonomous run
controls.

The scenario and playtest panel exposes GDD/prototype/QA swarm scenarios and
playtest evidence in Studio **read-only**. It inspects existing
scenario/playtest/evidence data and never launches runs, starts scenarios, runs
commands, or orchestrates QA beyond existing allowlisted trusted-runner paths.

## What it shows

Per scenario: id, template source, run status, pass/fail verdict, failure
classification, screenshot references, world-state, logs, and evidence links.

Diagnostics surface stale evidence (re-run required), missing evidence links for
completed verdicts, and broken/missing evidence references.

## Boundary

- **No autonomous run / start / command.** Run controls are disabled. The panel
  is pure display of escaped exported JSON; there are no run/start buttons or
  command runners. Existing QA/GDD/evidence contracts remain backward-compatible.
- Rust/local trusted code and existing allowlisted trusted runners own scenario
  execution, evidence writing, and trusted file boundaries.
- No claim of production-ready editor, Godot replacement, or full Godot editor
  parity; this is not a new execution orchestrator.
- Governance issues #1 and #23 remain open.
