# Studio QA Swarm Inspection Surface v1

Studio QA Swarm Inspection Surface v1 is the #695 read-only Studio/cockpit contract for inspecting bounded QA/playtest swarm evidence. It lets Studio display scenario candidates, fuzzing plans, worker budgets and assignments, QA run matrices, invariant checks, route attempts, visual/performance/error evidence, flaky rerun policy, failure classification/backlog state, and QA evidence bundle summaries without becoming a worker runner, trusted writer, command bridge, or hosted/cloud orchestration surface.

## Inputs

The surface consumes Rust/local dashboard read-model data exported as `qa_swarm_inspection` plus the underlying evidence refs produced by trusted local validation and run export commands. The browser-facing model is evidence and backlog input only. Missing, partial, stale, or malformed panels must stay visible as escaped read-only warning state rather than being repaired, hidden, persisted, or promoted by Studio.

The nine panel groups are intentionally separate:

- scenario candidates;
- fuzzing plans;
- worker budgets and assignments;
- QA run matrix;
- runtime invariant checks;
- objective route attempts;
- visual, performance, and error evidence;
- flaky rerun policy/results plus failure classification/backlog;
- QA evidence bundle summary.

## Boundary

Studio, dashboard, and cockpit surfaces that render this data must not:

- spawn QA workers, hidden workers, background agents, unbounded workers, local runners, cloud runners, hosted orchestration, or remote swarm services;
- execute commands, turn copyable command text into a button, open a browser command bridge, call a local server bridge, run credentialed commands, install dependencies, mutate dependencies, or invoke network/install commands;
- write trusted browser state, source files, dashboard exports, generated evidence, run artifacts, fuzz inputs, screenshots, videos, traces, backlog items, decision ledgers, or local tool state;
- auto-fix, auto-apply, auto-merge, self-approve, bypass reviewers, promote outputs, publish, deploy, or claim production safety;
- claim that QA/playtest evidence proves fun, subjective quality, market readiness, accessibility compliance, shipped-game readiness, production readiness, current Godot replacement, or a quality guarantee.

Trusted persistence, QA artifact validation, evidence writing, CLI behavior, review decisions, and any later mutation remain Rust/local responsibilities outside this browser inspection surface.

## Rendering requirements

- Escape all rendered data, including panel labels, evidence refs, malformed reasons, blocked reasons, inert command text, and boundary text.
- Keep copyable commands inert text only; never render execution controls, forms, local/cloud runner controls, apply buttons, merge buttons, approval controls, or worker-spawn controls.
- Show budgets, rerun limits, timeout limits, output roots, cleanup policies, and malformed/missing state when present.
- Keep QA/playtest outputs evidence/backlog inputs until reviewed by an explicit trusted process.
- Preserve existing scenario/evaluator/runtime/project/behavior/level/GDD/multi-agent/dashboard/Studio contracts unless a later issue includes an explicit migration note.

## Generated-state policy

Generated QA/playtest runs, fuzz inputs, screenshots, videos, traces, dashboard exports, temporary projects, browser profiles, and local tool state remain ignored/untracked unless a later issue explicitly scopes a deterministic fixture. Studio must not move generated artifacts into tracked source or create browser-owned persistence for them.

## Audit evidence

QA14.14.3 audits this contract with local tests that scan this document, `docs/README.md`, and `examples/authoring-cockpit/README.md` for the required no-worker-runner, no-write, no-command, no-cloud, no-auto-fix/apply/merge, generated-state, conservative-wording, #1/#23 governance, and escaped read-only rendering language. The issue closure evidence must also include live checks that #1 and #23 remain open.
