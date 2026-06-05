# Scenario Coverage v17: Full Studio Editor Regression Suite

Issue: #775
Roadmap anchor: #1 (Full Studio Editor v1). Memory anchor: #23.

## Scope

This suite locks the Full Studio Editor v1 fixture behavior introduced for the
integrated demo. It is a regression inventory and smoke fixture, not a new editor
feature. The coverage matrix lives at
`examples/full-studio-editor-regression-v17/coverage-matrix.fixture.json`.

Success scenarios cover:

- project overview;
- scene tree;
- entity inspector;
- draft edit preview;
- Safe Source Apply handoff preview;
- visual canvas;
- asset browser;
- scenario/evidence panel;
- evidence timeline;
- export panel;
- plugin panel;
- workspace persistence;
- command palette.

Failure scenarios cover:

- invalid scene reference;
- missing asset;
- malformed plugin descriptor;
- stale source apply target;
- blocked direct trusted write;
- blocked publish/deploy command;
- blocked shell command;
- broken evidence bundle;
- invalid workspace state;
- large fixture budget exceeded.

## Guardrails

The suite is fixture-scoped and generated-state aware. It does not add browser
trusted writes, command execution, publish/deploy/sign/upload behavior, plugin
execution, marketplace/network install behavior, auto-apply, auto-merge,
self-approval, reviewer bypass, or CI/workflow mutation.

Safe Source Apply remains a preview handoff only: any trusted source mutation
still requires validated preview, sandbox evidence, accepted independent review,
stale-target checks, rollback metadata, allowlisted verification, and post-apply
comparison outside the browser.

This document makes no current full Godot parity, replacement, secure sandbox,
production editor, native export, marketplace, hosted/cloud, or release-readiness
claim. #1 and #23 remain open governance anchors.

## Verification

Focused smoke:

```bash
node --check examples/full-studio-editor-regression-v17/coverage-smoke.test.cjs
node examples/full-studio-editor-regression-v17/coverage-smoke.test.cjs
```

The cockpit regression smoke also imports this matrix so the required authoring
cockpit test covers Scenario Coverage v17.
