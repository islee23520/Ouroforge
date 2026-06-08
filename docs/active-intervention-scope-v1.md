# Active-Intervention Scope and Studio Stack Decision v1

Issue: #2052 — Era M, Milestone 74

Status: **design gate / ADR accepted for implementation planning**. This document
records the already-made posture and Studio stack decisions before any
active-intervention implementation work. It is documentation only: it adds no
write path, data store, Studio endpoint, evaluator rule, or artifact schema.

## Decision summary

Ouroforge remains **agent-first by default**. The autonomous loop must continue to
complete with zero human input through the existing CLI and Rust-owned gates.
Human intervention is opt-in at defined points, and every intervention is treated
as **intervention-as-evidence**: a validated, recorded proposal, constraint, or
directive that is re-checked by existing gates before it can affect trusted
artifacts.

Studio surfaces may shift from read-only inspection to **read + gated-write**
where a milestone explicitly scopes it, but gated-write never means raw browser
or Elixir artifact writes. It means Studio can capture an intervention request
and route it to the existing Rust-owned review/apply, scene/source-apply,
evaluator, evidence, and provenance gates.

The interactive Studio stack decision is **Phoenix LiveView, local single-user**.
Elixir/OTP + Phoenix LiveView is the control and presentation plane only. The
Rust kernel remains the data plane and owns artifact truth, validation,
determinism, gate decisions, ledger/evidence/provenance, and CLI fallback.
Hosted, multi-user, collaborative, and real-time remote Studio remain Layer-3
**DEFER**.

## Two-plane invariant

The Era M intervention architecture has a strict two-plane invariant:

| Plane | Runtime | Owns | Must not own |
| --- | --- | --- | --- |
| Data plane | Rust kernel, evaluator, CLI, scene/source apply, evidence/provenance | Seed/run/artifact schemas, deterministic validation, gate decisions, trusted writes, ledger/evidence, provenance, CLI fallback | Presentation state, browser sessions, Phoenix UI lifecycle |
| Control + presentation plane | Elixir/OTP + Phoenix LiveView Studio, local single-user | Rendering, capture of user intent, routing to Rust commands/gates, local telemetry display, supervised UI/control tasks | Artifact semantics, direct artifact mutation, evaluator truth, gate bypass, hidden writes, hosted authority |

Elixir may render, capture, supervise, and route. Elixir never writes trusted
artifacts directly and never certifies artifact semantics. A Studio action is not
a trusted write until the Rust data plane validates and records it through the
appropriate gate.

## Active intervention points and reused gates

| Intervention point | Human input shape | Existing gated path reused | Recorded evidence requirement | Raw bypass explicitly forbidden |
| --- | --- | --- | --- | --- |
| Amend a proposal before approval | Amendment proposal against a pending proposal/review item | Review/apply gate, proposal re-verification, evidence/provenance ledger | Amendment text, author/source metadata, base proposal ref, re-verify result, decision ref | Editing accepted artifacts or proposal outputs in-place |
| Author intake | Human-authored artifact candidate | Scene/source-apply gate, review/apply gate, schema/evaluator validation | Original author artifact, normalized candidate, validation report, provenance link | Treating uploaded/freeform authoring text as trusted artifact state |
| Steer a live campaign | Directive or steering request | Existing orchestration review gate plus evaluator and evidence checks before apply | Directive text, scope, timing, run/task refs, evaluator impact, accept/reject decision | Runtime mutation that skips ledger/evaluator evidence |
| Constrain future outputs | Constraint proposal | Evaluator gate compilation/validation and evidence/provenance recording | Constraint source, compiled gate/rule ref, test/evaluator result, active/inactive decision | Unvalidated prompt-only policy that silently changes acceptance |
| Correct diagnosis | Correction proposal against a diagnosis/attribution | Review/apply plus evaluator/provenance re-attribution evidence | Original diagnosis, correction rationale, re-attribution result, new decision evidence | Overwriting diagnosis history or hiding the previous attribution |
| Take over and hand back a stage | Time-bounded directive and handback record | Review/apply, stage state evidence, evaluator gate before any trusted apply | Takeover reason, scope, start/end refs, handback summary, validation result | Permanent manual mode or untracked human task execution |

Each point preserves autonomous completion: if no human supplies input, the agent
loop continues with the existing default behavior. Human input can add evidence
for the gates; it cannot become a mandatory dependency for successful execution.

## Posture shift: read-only to read + gated-write

Earlier Studio surfaces were read-only inspection aids. Era M allows a narrower
posture: **read + gated-write**. The shift is intentionally small:

- Studio may display existing Rust-exported state and gate evidence.
- Studio may capture opt-in human intervention requests.
- Studio may submit those requests to an existing or explicitly scoped Rust gate.
- Studio must show pending/accepted/rejected/blocked evidence states.
- Studio must fail closed when evidence, validation, provenance, or base refs are
  missing or stale.

Read + gated-write excludes browser trusted writes, direct repository/artifact
mutation, hidden command bridges, direct evaluator decisions in Elixir, direct
ledger editing, direct scene/source apply from Phoenix, auto-merge, deployment,
release go/no-go automation, and any hosted/collaborative authority.

## Local-first Studio stack decision

Phoenix LiveView is selected for the interactive Studio because it fits the Era K
control-plane decision while preserving the two-plane boundary:

- local single-user UI with server-rendered interaction and no hosted account
  model;
- OTP supervision for local control/presentation tasks;
- live telemetry display derived from Rust artifacts and events;
- clear routing boundary to CLI/Rust kernel commands;
- no requirement for the autonomous loop or CLI fallback.

The Studio remains optional. A fresh checkout must still run the full loop via
the CLI without Phoenix, Elixir, a database, a hosted service, a remote worker,
or human input.

## Deferred scope

The following remain out of scope and require later Layer-3 governance before any
implementation:

- hosted Studio;
- multi-user or collaborative editing;
- remote real-time intervention;
- accounts, auth, cloud deployment, or shared databases;
- Elixir-owned artifact schemas or validation;
- browser or Phoenix direct writes into trusted artifacts;
- automated fun/taste verdicts or automated release go/no-go.

Fun/taste verdict and release go/no-go remain human Ring-2 decisions. Those
human decisions still must be recorded as evidence; they do not become raw
artifact bypasses.

## Guardrail checklist for future implementation issues

Future Era M/N implementation issues must satisfy this checklist before merge:

- Agent-first default preserved; zero-human-input CLI loop remains valid.
- Every intervention is a proposal, constraint, or directive with provenance.
- Every intervention reuses the existing review/apply, scene/source-apply,
  evaluator, evidence, or provenance gates.
- Rust validates and records trusted state; Elixir renders/captures/routes only.
- Studio write affordances are read + gated-write, never raw artifact writes.
- Missing/stale evidence blocks application rather than broadening authority.
- Hosted/multi-user/collaborative Studio remains deferred.
- #1 and #23 remain open unless a separate explicit governance issue says
  otherwise.

## ADR outcome

Decision: **GO for scoped active-intervention planning and local Phoenix LiveView
Studio read + gated-write surfaces, constrained by intervention-as-evidence and
the two-plane invariant.**

Rejected alternatives:

- Keep Studio permanently read-only: rejected because Era M requires opt-in active
  intervention surfaces, but only through gates.
- Allow direct Studio artifact writes: rejected because it breaks determinism,
  evidence, provenance, and Rust data-plane ownership.
- Require humans in the loop: rejected because it violates the agent-first default
  and CLI fallback.
- Build hosted/collaborative Studio now: rejected as Layer-3 scope creep.
- Let Elixir own artifact semantics: rejected because Rust remains the data
  plane and validation authority.
