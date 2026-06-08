# Whole-Game Orchestration State v1

Issue: #1684. Status: additive Rust/local contract.

`producer-orchestration-v1` extends the Milestone 23 campaign state and the
Milestone 42 production pipeline to a whole-game horizon. It records resumable
state over an existing `producer-plan-v1` task graph and dispatches function/role
agents in deterministic plan order. It is not a new orchestrator engine, hidden
worker runtime, hosted orchestrator, writer, release system, or browser command
bridge.

Each dispatch is proposal-only and carries the current plan task's role, inputs,
outputs, expected verification, and completion evidence ref. Completing a
dispatch advances the completed-prefix and updates a deterministic resume token,
so long-horizon state can be serialized and resumed without hidden state.

Browser, dashboard, and Studio surfaces may display the read model only. They do
not write trusted state, spawn workers, run commands, apply changes, merge,
release, or approve work. Any trusted write remains routed through the existing
review/apply/trust-gradient path with human gates.

Generated orchestration state, runs, assets, content, coverage, and local
artifacts remain untracked unless explicitly fixture-scoped. Public wording stays
conservative: no auto-apply, auto-merge, self-approval, reviewer bypass,
production-ready claim, current Godot replacement claim, quality/fun guarantee,
or autonomous shipping claim is introduced.

Issues #1 and #23 remain open.
