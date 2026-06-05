# Studio Demo Authoring Walkthrough v1

Issue: **#790**  
Demo root: `examples/playable-demo-v2/collect-and-exit` (Signal Gate / collect-and-exit)

This walkthrough documents how a reviewer inspects the Godot-Plus demo game through **read-only and draft-only** Studio surfaces. It does not grant trusted browser writes, command bridges, auto-apply, auto-merge, self-approval, publish/deploy, or plugin execution.

## What the walkthrough covers

1. **Project overview** — bounded project/scene/scenario counts and validation summary for the demo fixture.
2. **Scene tree and entity inspector** — entity selection and component inspection against the collect-and-exit scene fixture.
3. **Visual scene canvas** — read-only canvas with draft-only transform handles (no trusted scene writes).
4. **Asset browser** — manifest-backed asset metadata inspection only (no import/upload/fetch).
5. **Scenario and playtest panel** — existing scenario verdicts, logs, and evidence links (no run-start controls).
6. **Evidence timeline** — linked run/evidence artifacts for the demo iteration story.
7. **Export / package inspection** — local web package evidence and verification (publish/deploy remains blocked).
8. **Plugin / extension panel** — descriptor inspection for demo plugin metadata (no install/execute).
9. **Draft edit preview** — Studio draft authoring preview against source fixtures.
10. **Safe Source Apply handoff preview** — review-gated apply handoff text only; Studio does not execute apply.

## Deterministic fixture

- Fixture: `examples/godot-plus-demo-studio-walkthrough-v790/walkthrough-v790.fixture.json`
- Smoke: `node examples/godot-plus-demo-studio-walkthrough-v790/godot-plus-studio-walkthrough-smoke.test.cjs`

Generated `runs/`, `target/`, dashboard exports, and temp servers remain **untracked** unless explicitly scoped as source-like fixtures.

## Boundaries

- Rust/local tooling owns validation, evidence, and trusted apply.
- Browser/Studio surfaces render exported read models only.
- Wording stays conservative: scoped Godot-plus workflow proof, not full Godot parity, production readiness, or Godot replacement.
- **#1 and #23 remain open.**
