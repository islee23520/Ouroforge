# Scenario Coverage v13: QA Swarm Regression Suite

Issue: #697
Roadmap anchor: #1 remains open.
Memory/governance anchor: #23 remains open.

This coverage note aggregates the QA/playtest swarm contracts into one regression map. It documents fixture-level coverage and known gaps; it does not introduce a hidden worker pool, remote/cloud swarm, browser command bridge, auto-fix, auto-apply, auto-merge, reviewer bypass, or trusted mutation.

## Coverage matrix

The fixture matrix lives at `examples/qa-swarm-regression-suite-v13/coverage-matrix.fixture.json` and covers:

- scenario candidates
- adversarial input fuzzing plans
- worker assignments and budgets
- runtime invariant models/evidence
- objective route attempts
- visual comparison evidence
- performance budget evidence
- console/crash/runtime error classification
- flake/rerun policy
- failure classification and backlog entries
- run matrix rows
- evidence bundles
- Studio/dashboard read-model boundaries
- malformed, missing, stale, and unresolved-output failure states

Each row points at an existing positive fixture and a fail-closed negative fixture or synthetic guardrail. The Rust regression test validates those rows with the same public validators that own the individual contracts.

## Trusted-state boundary

QA/playtest outputs are evidence and backlog inputs until reviewed. Rust/local validation owns trusted persistence, artifact validation, generated evidence writing, and CLI contracts. Browser dashboard and Studio surfaces remain read-only or draft-only unless a separately scoped Rust/local trusted API owns persistence.

Generated runs, fuzz inputs, screenshots, videos, traces, dashboard exports, temp projects, and local tool state remain untracked unless explicitly fixture-scoped.

## Known gaps

- This is aggregate regression coverage over deterministic fixtures, not a live autonomous playtest service.
- Studio/dashboard compatibility is audited at the documented read-model boundary; it does not grant browser-side trusted file writes or command execution.
- Heuristic visual, performance, route, error, and failure classifiers are triage evidence, not proof of fun, subjective quality, market readiness, production safety, accessibility compliance, a current Godot replacement, or production-ready status.
