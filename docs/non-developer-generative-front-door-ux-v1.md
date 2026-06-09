# Non-Developer Generative Front-Door UX Scope and Contract v1

Issue: #2078 — Era N, Milestone 82

Status: **scope and contract accepted for implementation planning**. This
milestone is documentation only. It defines how a non-developer-facing
Phoenix LiveView Studio front door may capture game intent while preserving the
existing Ouroforge safety model: agent-first by default, intervention-as-evidence,
read + gated-write Studio posture, Rust-owned gates, and a local-first CLI
fallback.

## Goal

The Non-Developer Generative Front-Door UX lets a person express game intent in
plain language, templates, or guided Studio controls without learning repository
structure or source formats. The UX is a control/presentation surface over the
existing Generative Front Door v1 and Era M active-intervention gates; it is not
a new data plane, writer, validator, artifact store, or authority to apply a
change.

The target outcome is a recorded proposal, constraint, or directive with
provenance that can continue through the existing Rust-owned gates. If no human
input is supplied, the autonomous agent loop continues through the CLI with zero
human input.

## Reused gated paths

Every write-affecting action from this UX must reuse an existing gate. Studio
may render, capture, and route only; the Rust data plane validates and records
trusted outcomes.

| UX action | Captured input shape | Required reused path | Trusted effect only after |
| --- | --- | --- | --- |
| Start from a plain-language game brief | Brief/intake proposal with author/source metadata | Generative Front Door proposal model, review/apply, evidence/provenance | Rust validates the proposal and engine-room evidence is present |
| Choose or adjust a template | Template constraint proposal plus selected parameters | Evaluator/constraint gate, review/apply, evidence/provenance | Constraint compiles to a gate and passes validation |
| Amend a generated proposal before approval | Amendment proposal against a base proposal ref | Proposal amendment re-verify, review/apply, evidence/provenance | Re-verification succeeds against the current base ref |
| Add a human-authored artifact candidate | Artifact intake candidate | Human-authored artifact intake, scene/source-apply, evaluator, provenance | Candidate schema/evaluator checks pass and review accepts it |
| Steer an active campaign from Studio | Directive with bounded scope, target run/task refs, and timing | Live campaign steering directive path, evaluator, evidence/provenance | Directive is accepted by the Rust gate and recorded as evidence |
| Correct a diagnosis or generated rationale | Correction proposal with rationale and target diagnosis ref | Diagnosis correction / re-attribution path, review/apply, provenance | Re-attribution evidence is recorded and stale refs fail closed |
| Take over and hand back a stage | Time-bounded takeover directive and handback summary | Stage takeover/handback gate, review/apply, evaluator, evidence/provenance | Handback validation records scope, result, and gate status |

No row creates a raw Studio write. The UX may help a non-developer describe
intent, but the trusted result remains a gated Rust artifact/evidence transition.

## Intervention-as-evidence invariant

All human input captured by this capability is **intervention-as-evidence**: a
validated, recorded proposal, constraint, or directive with provenance. It must
include enough context for deterministic re-checking: author/source, target ref,
base hash or run/task id when applicable, captured text/parameters, validation
status, gate verdict, and accept/reject/block evidence.

Human input is opt-in and never required for loop completion. A missing human
brief, template choice, amendment, directive, correction, or handback cannot block
the autonomous path by itself. Missing, stale, malformed, or unverifiable human
input fails closed and leaves the autonomous CLI path available.

## Read + gated-write Studio posture

Phoenix LiveView Studio may provide a friendlier non-developer front door with
forms, guided prompts, template pickers, previews, copyable CLI commands, pending
states, validation messages, and evidence/status displays. This is **read +
gated-write** only:

- **Read**: Studio displays Rust-exported state, proposal provenance, gate
  evidence, review state, evaluator status, and CLI fallback commands.
- **Gated-write capture**: Studio may capture a proposed brief, template
  constraint, amendment, artifact candidate, steering directive, correction, or
  stage handback record and route it to the relevant Rust gate.
- **No raw write**: Studio never writes trusted artifacts, ledgers, evidence,
  scenes, sources, release decisions, merge decisions, or evaluator verdicts
  directly.
- **Fail closed**: stale refs, missing evidence, invalid provenance, unsupported
  target types, or unavailable Rust validation keep the item pending/rejected;
  they do not broaden Studio authority.

## Two-plane invariant

This milestone preserves the Era M two-plane boundary:

| Plane | Owns | Non-developer UX responsibility | Forbidden leakage |
| --- | --- | --- | --- |
| Rust data plane | Artifact truth, schemas, deterministic validation, evaluator decisions, review/apply, scene/source-apply, source-apply, evidence, provenance, CLI fallback | Validate, gate, record, and expose deterministic status for captured inputs | None; Rust remains the only trusted artifact/evidence writer |
| Elixir/OTP + Phoenix LiveView control/presentation plane | Local single-user rendering, UI state, capture, routing, supervision, status presentation | Make the front door approachable and route captured inputs to Rust gates | Artifact semantics, direct artifact mutation, evaluator truth, ledger/evidence writes, hidden command bridges |

Elixir can make the path understandable; it cannot make a proposal true,
accepted, verified, applied, releasable, merged, or fun.

## Local-first CLI fallback

The CLI fallback is mandatory and sufficient. A fresh checkout must be able to
complete the generative front-door loop without Phoenix, a browser, a database, a
hosted service, a remote worker, or human input. For every Studio capture path,
there must be an equivalent or narrower local CLI path that can submit or inspect
the same proposal/constraint/directive through Rust-owned gates.

Studio may display copyable commands and evidence locations, but those commands
are informational. Studio does not execute them as a browser command bridge and
does not become required for success.

## Non-developer UX boundaries

Non-developer does not mean no-gate, no-review, no-codebase-truth, or no-human
responsibility. It means the author can express intent without manually editing
source files. The resulting candidate still passes through the same review,
engine-room, evaluator, provenance, and apply gates as developer-authored input.

The UX must use conservative wording:

- say "proposal", "candidate", "captured intent", "pending validation", and
  "verified by gates";
- avoid "no-code builder", "instant game", "shippable", "production-ready",
  "fun guaranteed", "auto-fix", "auto-apply", "auto-merge", or "Godot
  replacement";
- keep fun/taste and release go/no-go explicitly human-owned.

## Deferred scope

The following are not authorized by this contract:

- direct Studio, browser, or Elixir trusted writes;
- a new artifact store, database, queue, hosted service, account model, or
  collaborative/multi-user Studio;
- Elixir-owned artifact schemas, validators, evaluator truth, or provenance
  ledgers;
- prompt-only acceptance rules that silently change generation outcomes without a
  compiled Rust/evaluator gate;
- autonomous apply, auto-merge, self-approval, reviewer bypass, direct release,
  deploy, signing, publishing, Steam account actions, or public launch;
- automated fun/taste verdicts, quality guarantees, market validation, or release
  go/no-go automation.

Hosted/multi-user/collaborative Studio remains Layer-3 DEFER. Fun/taste and
release decisions remain human and evidence-recorded, not automated.

## Guardrail checklist for downstream Era N issues

Downstream implementation issues must prove all of the following before merge:

- Agent-first default preserved; zero-human-input CLI loop still completes.
- Captured human input is intervention-as-evidence with provenance.
- Every write-affecting action routes through review/apply, scene/source-apply,
  source-apply, evaluator, evidence, or provenance gates; no parallel write path.
- Rust owns trusted validation and persistence; Elixir renders/captures/routes
  only.
- Studio posture is read + gated-write and fails closed on missing/stale evidence.
- CLI fallback remains documented, tested, and sufficient.
- No new data store, hosted/multi-user surface, command bridge, release path, or
  fun/taste automation is introduced.
- #1 and #23 remain open governance anchors.

## Verification and evidence expectations

A downstream issue referencing this contract should include evidence for:

1. the captured input shape and provenance fields;
2. the exact reused Rust gate or CLI command;
3. failure behavior for stale/missing evidence;
4. proof that Studio does not write trusted artifacts directly;
5. proof that the CLI path works without Studio or human input; and
6. proof that generated/local state remains ignored unless explicitly
   fixture-scoped.

For this documentation-only milestone, verification is limited to the repository
build plus text evidence that the contract records read + gated-write,
intervention-as-evidence, two-plane, and local-first boundaries.

## ADR outcome

Decision: **GO for scoped non-developer generative front-door UX planning over
existing gates, with local Phoenix LiveView Studio as read + gated-write control
and presentation only.**

Rejected alternatives:

- Build a no-gate no-code writer: rejected because it would bypass review,
  evaluator, evidence, and provenance.
- Let Studio directly apply generated artifacts: rejected because trusted writes
  belong to Rust gates only.
- Require human input for generation: rejected because the autonomous CLI loop
  must still complete with zero human input.
- Add a hosted collaborative Studio or new data store: rejected as Layer-3 scope
  creep and unnecessary for this local milestone.
- Let Elixir own artifact semantics or validation: rejected because Rust remains
  the data plane.

## #1 / #23 governance preservation

- #1 remains open as the broad roadmap and vision anchor.
- #23 remains open as the repository memory/design context anchor.
- This contract does not close, replace, narrow, or modify either anchor. Any
  change to those anchors requires a separate explicit governance decision.
