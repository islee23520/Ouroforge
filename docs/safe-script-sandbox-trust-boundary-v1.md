# Safe Script Sandbox and Trust Boundary v1

Issue: #616 — Safe Script Sandbox and Trust Boundary v1. This document is a
policy/design gate for any future executable gameplay scripting proposal under
[`Gameplay Scripting / Logic System v1`](gameplay-scripting-logic-system-v1.md)
and the [`Script Module Interface Design Gate v1`](script-module-interface-design-gate-v1.md).
It does **not** implement or authorize arbitrary script execution, a sandbox
runtime, dynamic import, plugin loading, command bridges, browser trusted writes,
source apply, dependency/CI mutation, native export, hosted services, or a
current Godot replacement claim.

## Design-gate status

This issue is design-gate only. Structured behavior, events/signals,
state-machines, abilities/actions, drafts, review gates, evidence bundles, and
Studio/dashboard inspection remain separate from executable scripting. Any future
runtime must be separately scoped, reviewed, sandbox-verified, and evidence-gated
before it can affect trusted state.

## Policy-level operation matrix

| Operation class | Policy stance before future runtime approval | Required future authority |
| --- | --- | --- |
| Read validated world/scene/run/evidence state | Allowed only as inert snapshot input | Rust/local validator provides bounded read model refs |
| Request bounded gameplay actions | Design-allowed only, not executable here | Reviewed module + sandbox dry-run + trusted Rust/local apply gate |
| Emit events/signals | Design-allowed only as structured output | Event/signal schema validation and deterministic ordering evidence |
| Declare required tests | Allowed as metadata/catalog ids | Allowlisted argv/catalog command ids, not shell text |
| Write generated evidence bundle | Future sandbox output only | Generated root policy + hash/evidence manifest |
| Mutate source or trusted project state | Blocked by default | Separate review-gated source apply issue with rollback and human/independent acceptance |
| Filesystem access | Blocked except explicit sandbox/generated output roots | Canonical root checks, no traversal, no symlink/hardlink escape |
| Network/server/auth/account access | Blocked | Separate governance issue; #616 does not authorize it |
| Process/shell/command execution | Blocked | No direct module authority; only external allowlisted verifier may run outside module |
| Dependency/CI/workflow/build-script mutation | Blocked | Separate governance issue; not module-accessible |
| Secrets/env/credentials | Blocked | Never exposed to modules or browser surfaces |
| `eval`, dynamic import, plugin loading, native/WASM loading | Blocked | Not authorized by this gate |
| Browser trusted writes or command bridges | Blocked | Browser/Studio remains read-only or draft-only |

## Deterministic execution expectations

A future approved sandbox must define deterministic behavior before execution:

- stable module id, version, capability list, target system refs, and input refs;
- immutable validated input snapshot with hashes for world, scene, state,
  ability/action, event/signal, run, scenario, and evidence refs;
- explicit random seed if randomness is ever allowed;
- bounded step count, wall-time budget, CPU budget, memory budget, output byte
  budget, event count, action request count, and evidence artifact count;
- deterministic event ordering and stable serialization;
- no wall-clock, network, host environment, implicit filesystem discovery,
  browser privileged API, dynamic import, or plugin discovery dependence;
- replay evidence proving the same input snapshot produces the same structured
  outputs or the same failure classification.

## Resource limits and timeout behavior

Future runtime proposals must publish conservative limits before use. At minimum:

- `maxStepCount`: upper bound on interpreter/runtime steps;
- `timeoutMs`: hard timeout that fails closed;
- `maxMemoryBytes`: hard memory ceiling;
- `maxOutputBytes`: bounded serialized output size;
- `maxEventEmits`, `maxActionRequests`, and `maxEvidenceRefs`;
- `maxLogEntries` and redaction policy for display logs;
- failure status for timeout, memory exceeded, output exceeded, invalid target,
  invalid action, malformed output, stale input, missing evidence, forbidden API,
  and non-deterministic replay.

Timeouts and limit violations must produce structured failure evidence and must
not partially mutate trusted state.

## Input/output contract

Future script input must be a validated inert snapshot. It may include only
repo-relative refs and bounded data derived from trusted Rust/local validation.
It must not include secrets, credentials, environment variables, local absolute
paths, browser privileged handles, command text, file descriptors, sockets, or
mutable references.

Future script output must be structured data only:

- requested bounded action records;
- emitted event/signal records;
- metadata/status records;
- declared verification refs;
- generated evidence refs under explicit generated/sandbox roots;
- failure records with reason, phase, bounded logs, and input/output hashes.

Outputs remain untrusted until Rust/local validation accepts them. Browser,
dashboard, and Studio consumers may display them as read-only or draft-only data
but must not treat them as trusted persistence.

## Filesystem, network, process, credential, and dependency restrictions

A future sandbox must fail closed on:

- filesystem reads/writes outside an explicit sandbox/generated root;
- path traversal, backslashes where not allowed, absolute paths, symlink escape,
  hardlink aliasing, hidden generated roots not explicitly allowed, and opaque
  binary writes without a declared fixture scope;
- network access, local server bridges, sockets, fetch/XHR/websocket, cloud,
  hosted, auth, account, telemetry, or external service calls;
- process spawning, shell execution, command execution, package-manager calls,
  install/update/download behavior, hidden command runners, or browser-to-local
  command bridges;
- environment/secrets/credentials/tokens/keychain access;
- dependency, CI, workflow, build-script, lockfile, or package metadata mutation;
- `eval`, dynamic import, runtime plugin loading, extension loading, marketplace
  loading, WASM/native loading, or arbitrary JS/Rust/Python/Lua/WASM execution
  outside a separately approved sandbox runtime.

## Review, rollback, sandbox dry-run, required tests, and evidence bundle

A future executable proposal must require:

1. Accepted review decision for the exact module, target refs, capabilities, and
   generated/trusted output boundary.
2. Sandbox dry-run evidence before trusted persistence.
3. Rollback snapshot/metadata before any future trusted apply path.
4. Required tests declared as allowlisted argv/catalog command ids, not shell
   text embedded in module data.
5. Evidence bundle containing input hashes, output hashes, policy version,
   capability decision, limit settings, deterministic replay result, failure
   classification if any, and linked review decision refs.
6. Missing/malformed/stale evidence behavior that fails visibly and blocks
   trusted persistence.
7. Explicit proof that #1 and #23 remain open for governance continuity.

## Future behavior evidence path

Future script output could become behavior evidence only after these gates pass:

1. validate module metadata against the #615 design gate;
2. validate sandbox policy and trust-boundary settings against this #616 gate;
3. run sandbox dry-run with deterministic input snapshot;
4. validate structured output as untrusted generated evidence;
5. link output to behavior/event/state/ability/scenario evidence refs;
6. require review-gated promotion before any trusted state mutation;
7. write generated evidence only under explicit fixture/generated roots.

Until then, script-like output is advisory evidence only and cannot mutate trusted
source, project state, runtime state, browser state, dependencies, CI, workflows,
or release artifacts.

## Compatibility and generated-state notes

This policy is additive to existing Seeds, scenes, project manifests, runs,
scenarios, dashboard exports, Studio read models, behavior/event/state-machine,
ability/action, source-like fixtures, and 2D/3D fixtures. It does not introduce a
new dependency, runtime, language, plugin system, server, browser bridge, source
apply path, native export, or migration requirement. Generated sandbox outputs,
reviews, dry-run bundles, run artifacts, dashboard data, screenshots, and local
tool state remain ignored unless explicitly fixture-scoped by a later issue.

## Public wording guardrail

Public wording must stay conservative: no production-stable scripting API,
secure-sandbox guarantee, current Godot replacement, production-ready engine,
shipped-game maturity, hosted/cloud/auth claim, native export claim, plugin
runtime claim, autonomous launch claim, or broad engine compatibility claim.

## Non-goals

This gate does not authorize arbitrary JS/Rust/Python/Lua/WASM script execution,
`eval`, dynamic import, plugin runtime, extension loader, marketplace, command
bridge, browser trusted writes, local server bridge, unrestricted source mutation
apply, auto-merge, auto-accept, self-approval, native export, hosted/cloud/server
behavior, dependency/CI/workflow mutation, or current Godot replacement claims.

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open.
