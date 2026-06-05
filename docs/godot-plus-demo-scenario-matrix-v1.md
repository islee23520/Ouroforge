# Godot-Plus demo scenario matrix v1 (#787)

This document records the bounded scenario matrix for the Collect-and-Exit demo.
The machine-readable source is `examples/playable-demo-v2/collect-and-exit/scenarios/demo-scenario-matrix.json`.

The matrix covers: start game, player movement, level completion, fail/restart,
enemy interaction, objective update, UI/HUD state, runtime probe state, local export
smoke, read-only Studio walkthrough, inert plugin validation, and evidence bundle
coverage. Each scenario declares pass criteria, fail criteria, expected evidence
artifacts, and checked-in verification references.

Boundaries remain unchanged: no commercial/public release, no signing or store
publishing, no direct Studio trusted source writes, no auto-apply/auto-merge,
no reviewer bypass, no browser command bridge, no arbitrary shell execution,
no dependency install, no credentials, no executable plugin runtime, no marketplace,
and no network plugin install/update. "Godot-plus" remains a scoped
evidence-native workflow demonstration, not a full Godot replacement or production
readiness claim.

Verification:

```bash
node --check examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs
```

Future QA swarm/regression issues may consume scenario IDs from the matrix for
read-only planning and evidence checks only; this matrix grants no authority to
mutate trusted source, execute command bridges, install dependencies, publish,
release, or run executable plugins.
