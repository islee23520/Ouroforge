# Studio Behavior Inspection Surface v1

Issue: #622 — Studio Behavior Inspection Surface v1.

Studio Behavior Inspection Surface v1 lets the local authoring cockpit inspect
structured gameplay behavior evidence without granting trusted write or execution
authority to the browser. The surface is an escaped read-only composition of
existing data-first gameplay logic read models:

- behavior list rows for validated structured behavior definitions;
- event/signal rows for deterministic, bounded event queue inspection;
- state-machine rows for states, transitions, guards, and display-only effects;
- ability/action rows for bounded action metadata, costs, cooldowns, effects,
  and runtime status;
- behavior draft status rows from Rust validation/preview;
- review/apply status rows that report accepted, blocked, stale, or ready
  metadata without performing any apply.

The surface does **not** create an executable scripting runtime, script editor,
visual scripting system, plugin runtime, marketplace, command bridge, local
server bridge, hosted/cloud/backend service, browser trusted-write path,
source-apply authority, auto-apply path, auto-merge path, self-approval path,
credentialed/network/install behavior, native export path, production-stable
scripting API, production editor, or Godot replacement claim.

## Trust boundary

Rust/local code remains the trusted authority for validation, persistence,
behavior draft/apply validation, generated evidence writing, project/run
binding, rollback metadata, and CLI behavior. Browser Studio code may only render
escaped dashboard/cockpit data and inert copyable text. It must not execute CLI
commands, mutate source files, write trusted project state, write generated run
state, persist browser draft state, dispatch gameplay events, activate abilities,
transition state machines, approve reviews, apply behavior transactions, repair
missing evidence, install dependencies, access credentials, publish, deploy, or
open a command/local-server bridge.

Generated behavior drafts, review/apply records, runs, dashboard exports,
screenshots, browser profiles, temp projects, and local tool state stay ignored
unless a later issue explicitly scopes tiny fixture data under `examples/`.

## Panel contracts

### Behavior list panel

The behavior list panel renders ids, status, labels/summaries, target refs,
trigger refs, condition/action counts, blocked reasons, and evidence refs from
structured behavior read models. Missing or malformed input remains visible as an
empty/warning state. The panel must never create edit controls, script bodies,
`eval` snippets, dynamic import hooks, plugin loader hooks, command buttons,
trusted-write controls, or auto-apply controls.

### Event/signal panel

The event/signal panel renders deterministic event order, event types, signal
names, source/target refs, tick/order values, consumed/unconsumed state,
blocked reasons, consumers, and evidence refs. It is queue inspection only: the
browser does not emit events, dispatch signals, run scenario probes, rerun tests,
or mutate runtime state.

### State machine panel

The state-machine panel renders state ids, initial state, transition summaries,
trigger kinds, blocked reasons, and evidence refs. It is a display surface only:
the browser does not transition machines, execute entry/exit actions, interpret
scripts, write runtime state, or persist state changes.

### Ability/action panel

The ability/action panel renders ability/action ids, runtime status, target,
trigger, effect, cost/cooldown metadata, blocked reasons, and evidence refs. It
must not activate abilities, run effects, dispatch commands, or treat an action
row as trusted executable behavior.

### Draft and review/apply status panels

Behavior draft status may show target hash freshness, validation status,
behavior/evidence/scenario-impact counts, blocked reasons, diagnostics, and
inert preview command text. Review/apply status may show review decision ids,
reviewers, transaction ids, rollback refs, blocked reasons, and evidence refs.
These panels are status-only. They do not approve, apply, merge, self-review,
execute commands, write files, or convert preview metadata into trusted state.

## Audit checklist

Each Studio behavior inspection change must verify:

1. #622, #1, and #23 are still open before work, before PR merge, and before
   closure of #622.
2. Rendered behavior/event/state/ability/draft/review/apply data is escaped,
   including malformed and adversarial strings.
3. Missing and malformed read models are visible as empty/warning states and do
   not crash the cockpit.
4. The browser adds no `eval`, dynamic import, plugin loader, command bridge,
   local server bridge, direct file write, localStorage/indexedDB persistence,
   showSaveFilePicker path, apply button, merge button, self-approval control,
   network/install/credential behavior, or hidden command execution.
5. Copyable command text remains inert and points back to Rust/local validation
   where applicable.
6. Generated behavior drafts, transactions, runs, dashboard exports, screenshots,
   browser profiles, temp projects, and local tool state remain untracked unless
   fixture-scoped.
7. Public wording stays conservative: no production-stable scripting API,
   production editor, secure sandbox, native export, plugin runtime, hosted/cloud
   feature, current Godot replacement, or production-ready engine claim.

## Compatibility notes

The surface extends existing dashboard/cockpit read-model composition. It must be
additive and preserve existing Seeds, scenes, project manifests, runs,
scenarios, dashboard exports, Studio read models, 2D/3D fixtures, behavior
contracts, source-like fixtures, and runtime probe shapes unless a later issue
adds an explicit migration note and focused compatibility tests.
