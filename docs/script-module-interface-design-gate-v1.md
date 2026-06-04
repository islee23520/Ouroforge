# Script Module Interface Design Gate v1

Issue: #615 — Script Module Interface Design Gate v1. This is a design-gate
control artifact for a future gameplay script module interface under
[`Gameplay Scripting / Logic System v1`](gameplay-scripting-logic-system-v1.md).
It defines constraints for future proposals and intentionally does not implement
an executable script runtime, loader, sandbox, plugin runtime, dynamic import,
command bridge, browser trusted write path, source apply path, native export, or
current Godot replacement capability.

## Design-gate status

This gate is **design-only**. Structured gameplay behavior, event/signal,
state-machine, ability/action, draft/apply, evidence, and Studio inspection
contracts remain separate from arbitrary executable scripting. Any future script
runtime proposal must pass a separate governance issue, review-gated source
mutation controls, sandbox evidence, and deterministic replay/evidence checks.

## Future module interface shape

A future script module may be proposed only as a bounded interface over existing
validated artifacts. It may eventually request these capability classes after a
separate approval gate:

- read validated world, scene, entity/component, state-machine, ability/action,
  event/signal, scenario, and run evidence state;
- request bounded actions through a Rust/local trusted API that validates targets,
  payloads, review state, rollback metadata, and evidence links before any
  persistence;
- emit structured events/signals through validated event/signal contracts;
- declare required verification commands from an allowlisted command catalog;
- expose metadata and evidence links for dashboard/Studio display.

The interface does not grant direct filesystem, process, network, browser, source
mutation, dependency, CI, workflow, credential, or secret access.

## Module metadata requirements

Future module descriptors must be structured data with at least:

- `id`: stable path-safe module id.
- `version`: semver-like design version.
- `status`: `draft`, `review_pending`, `sandbox_only`, `blocked`, or
  `unsupported`.
- `capabilities`: bounded capability declarations such as `read_world_state`,
  `request_bounded_action`, `emit_event`, `declare_required_tests`, and
  `expose_metadata`.
- `targetSystems`: bounded references to behavior, event/signal, state-machine,
  ability/action, scenario, dashboard, or Studio surfaces.
- `requiredTests`: allowlisted verification command ids and evidence refs; no raw
  shell text or browser command bridge fields.
- `evidenceRefs`: repo-relative evidence or design refs.
- `author` / `source`: display-only provenance; no credential or secret fields.
- `blockedReasons`: required when status is blocked or unsupported.

## Forbidden APIs and fields

Future script module proposals must reject and audit any direct or disguised
access to:

- filesystem reads/writes outside explicitly validated generated or sandbox roots;
- process spawning, shell execution, command execution, local server bridges, or
  hidden command runners;
- network, hosted/cloud/server/auth/account behavior;
- secrets, environment variables, credentials, tokens, or account state;
- dependency, CI, workflow, build-script, package-manager, or install mutation;
- `eval`, dynamic import, dynamic code loading, plugin loading, extension loading,
  marketplace loading, WASM/native loading, or arbitrary JS/Rust/Python/Lua/WASM
  execution;
- browser trusted writes, browser-to-local command bridges, or trusted file writes
  from dashboard/Studio;
- source apply, auto-apply, auto-merge, auto-accept, or self-approval.

Forbidden concepts must be normalized across casing and separators. For example,
`plugin_loader`, `Plugin-Loader`, `dynamicImport`, `trusted_write`, and
`commandBridge` are all forbidden.

## Review, sandbox, and evidence requirements

Before any future runtime implementation can be considered, it must define:

1. A review decision artifact proving accepted human/independent review for the
   exact module and target scope.
2. A sandbox execution plan that cannot mutate trusted source, dependencies, CI,
   workflows, credentials, or browser trusted state.
3. Deterministic inputs, bounded outputs, and replayable evidence refs.
4. Rollback metadata for any trusted persistence proposal.
5. Allowlisted verification commands represented as argv/catalog data, not shell
   text.
6. Missing/malformed/stale evidence behavior that fails visibly instead of
   inferring runtime success.
7. Dashboard/Studio surfaces that remain read-only or draft-only unless a
   Rust/local trusted API owns persistence.

## Deterministic execution expectations

A future approved runtime must be deterministic by construction: stable module
ids, stable inputs, bounded time/step budgets, bounded outputs, explicit random
seeds where applicable, ordered event emission, and replayable evidence bundles.
It must not depend on wall-clock time, network state, host environment variables,
implicit filesystem discovery, browser privileged APIs, or unreviewed dynamic
imports.

## Compatibility and generated-state notes

This design gate is additive to existing Seeds, scenes, project manifests, runs,
scenarios, dashboard exports, Studio read models, behavior/event/state-machine,
ability/action, 2D/3D fixtures, and source-like fixtures. It does not migrate or
weaken existing contracts. Generated module drafts, reviews, sandbox outputs,
run artifacts, dashboard data, screenshots, and local tool state remain ignored
unless explicitly fixture-scoped by a later issue.

## Public wording guardrail

Public wording must stay conservative: no production-stable scripting API,
broad compatibility-stable engine API, secure-sandbox guarantee, current Godot
replacement, production-ready engine, shipped-game maturity, hosted/cloud/auth
claim, native export claim, plugin runtime claim, or autonomous launch claim.

## Non-goals

This gate does not authorize arbitrary JS/Rust/Python/Lua/WASM script execution,
`eval`, dynamic import, plugin runtime, extension loader, marketplace, command
bridge, browser trusted writes, local server bridge, unrestricted source mutation
apply, auto-merge, auto-accept, self-approval, native export, hosted/cloud/server
behavior, or current Godot replacement claims.

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open.
