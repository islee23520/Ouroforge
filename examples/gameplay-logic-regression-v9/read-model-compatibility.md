# Scenario Coverage v9 Read-Model Compatibility

Scope for #624 GL10.14.3: document coverage matrix and dashboard/Studio read-model compatibility for the Gameplay Logic Regression Suite.

Run:

```bash
node examples/gameplay-logic-regression-v9/coverage-read-model-compatibility.test.cjs
```

The compatibility fixture stays read-only. It does not execute scripts, run command bridges, write trusted files, auto-apply behavior changes, auto-merge, self-approve, introduce hosted/cloud behavior, claim a production-stable scripting API, or claim Godot replacement maturity. Generated `runs/`, `dashboard-data/`, `target/`, `tmp/`, screenshots, and local tool state remain untracked.
