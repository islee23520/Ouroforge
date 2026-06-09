# Live Campaign Steering Directives Scope & Contract v1

Issue: #2061 — Era M, Milestone 77

Status: **contract accepted for implementation planning**. This document fixes the
scope for live campaign steering directives before any implementation work. It is
documentation only: it adds no Studio endpoint, Elixir write path, Rust artifact
schema, evaluator rule, database, or new data store.

## Goal

Live campaign steering lets a human optionally provide a time-bounded directive
to an already-running autonomous campaign. The directive is not a raw edit and is
not a mandatory dependency. It is **intervention-as-evidence**: a validated,
recorded proposal/constraint/directive that may affect trusted state only after
existing gates accept it.

The default remains agent-first. If no human intervenes, the autonomous loop must
still complete through the local CLI with zero human input.

## Contract

A live steering directive is a request to influence campaign behavior, not an
artifact mutation. It may express intent such as:

- prioritize, pause, resume, or de-prioritize a bounded task or stage;
- narrow or expand a campaign objective within an approved scope;
- add a temporary constraint that future outputs must satisfy;
- request an evaluator-backed re-check before a stage proceeds;
- ask for a handback summary after a bounded intervention window closes.

A directive is accepted only when the Rust data plane can validate its scope,
base references, provenance, evaluator impact, and decision evidence. A directive
is rejected or blocked when any required evidence is missing, stale, ambiguous,
or outside the approved campaign scope.

## Gated path reused

Live steering reuses the existing gated paths. It must not introduce a parallel
write path.

| Directive effect | Existing gated path reused | Required evidence before apply |
| --- | --- | --- |
| Change a pending proposal or approval target | Review/apply gate | Directive text, base proposal ref, scope, re-verification result, accept/reject decision |
| Influence scene or source-affecting output | Scene/source-apply gate plus review/apply | Base artifact refs, normalized candidate, validation report, evaluator result, provenance |
| Add or change a future-output constraint | Evaluator gate compilation/validation plus evidence/provenance | Constraint source, compiled rule or evaluator ref, test result, active/inactive decision |
| Pause, resume, prioritize, or hand back a campaign stage | Orchestration review gate plus evaluator and stage-state evidence | Run/task refs, timing, requested action, stage state before/after, evaluator impact, decision record |
| Correct campaign diagnosis or attribution | Review/apply plus evaluator/provenance re-attribution | Original diagnosis, correction rationale, re-attribution evidence, new decision evidence |

The accepted directive becomes recorded evidence. Trusted artifacts, ledgers,
source files, scene files, and evaluator verdicts change only through the Rust
kernel/evaluator/apply paths that already own those semantics.

## Intervention-as-evidence invariant

Every human steering action must satisfy all of these conditions:

1. **Opt-in:** the human action is optional; lack of human input never blocks the
autonomous campaign from reaching its normal terminal path.
2. **Recorded:** the raw directive, source metadata, timing, and target refs are
captured as provenance/evidence.
3. **Validated:** the directive is checked by the relevant Rust-owned gate before
it can affect trusted state.
4. **Decided:** accepted, rejected, and blocked outcomes are explicit evidence
states, not hidden UI state.
5. **Reversible/accountable:** applied effects reference the directive evidence
and can be audited through the existing ledger/evidence trail.
6. **No bypass:** neither Studio nor Elixir may write trusted artifacts, ledgers,
scene/source state, evaluator truth, release decisions, or merge/deploy state
outside the existing gates.

## Read + gated-write Studio posture

Studio may provide **read + gated-write** affordances for steering directives:

- read campaign, stage, task, evidence, and evaluator state exported by Rust;
- capture a local single-user directive request;
- route that request to the existing Rust CLI/kernel gate;
- render pending, accepted, rejected, or blocked evidence states;
- fail closed when evidence, provenance, target refs, or validation are missing.

Studio must not provide browser trusted writes, direct artifact edits, direct
ledger/evidence mutation, direct scene/source apply, Elixir-owned evaluator
verdicts, hidden command bridges, auto-merge, deploy, release go/no-go, or hosted
collaboration authority.

## Two-plane invariant

Live steering preserves the Era M two-plane invariant:

| Plane | Runtime | Owns | Must not own |
| --- | --- | --- | --- |
| Data plane | Rust kernel, evaluator, CLI, review/apply, scene/source-apply, evidence/provenance | Artifact truth, schemas, deterministic validation, gate decisions, trusted writes, evaluator verdicts, ledger/evidence, CLI fallback | Presentation lifecycle, browser session state, Phoenix UI authority |
| Control + presentation plane | Elixir/OTP + Phoenix LiveView Studio, local single-user | Rendering, local interaction capture, routing to Rust gates, supervised local UI/control tasks | Artifact semantics, direct trusted writes, evaluator truth, gate bypass, hosted/multi-user authority |

Elixir renders, captures, supervises, and routes. Rust validates, decides,
records, and applies. A Phoenix/LiveView event is never trusted state until the
Rust data plane accepts and records it.

## Local-first CLI fallback

The capability must remain local-first. A fresh checkout must be able to run the
full campaign loop from the CLI without Phoenix, Elixir, a browser session, a
database, a hosted service, a remote worker, or a human.

Implementation issues may add Studio presentation for convenience, but the CLI
path remains the canonical fallback and must expose enough status/evidence for a
local operator or autonomous agent to continue without Studio.

## Deferred and forbidden scope

The following remain out of scope for this contract:

- mandatory human steering;
- raw human edits that bypass review/apply, scene/source-apply, evaluator,
  evidence, or provenance gates;
- hosted, multi-user, collaborative, or real-time-remote Studio;
- accounts, auth, cloud deployment, shared databases, or remote workers;
- Elixir/Phoenix-owned artifact schemas, evaluator semantics, or trusted writes;
- automated fun/taste verdicts or automated release go/no-go;
- any claim that Studio is a no-code replacement for the autonomous loop.

Fun/taste and release go/no-go remain human Ring-2 decisions and must be recorded
as evidence when they occur. They are not raw artifact write authority.

## Implementation checklist for downstream issues

Downstream Milestone 77 implementation issues must demonstrate:

- agent-first default preserved and zero-human-input loop still works;
- steering directive is captured as intervention-as-evidence;
- accepted effects route through existing review/apply, scene/source-apply,
  evaluator, evidence, or provenance gates;
- Studio surface, if touched, is read + gated-write only;
- Rust remains the data plane and Elixir remains control + presentation only;
- no new data store or parallel trusted write path;
- local-first CLI fallback remains usable;
- hosted/multi-user/collaborative Studio remains Layer-3 DEFER.

## Relationship to prior contract

This contract specializes the active-intervention ADR in
[`docs/active-intervention-scope-v1.md`](active-intervention-scope-v1.md). The
prior ADR defines the broad Era M posture and Studio stack decision; this file
fixes the live campaign steering directive contract for Milestone 77.

#1 and #23 remain open governance anchors.
