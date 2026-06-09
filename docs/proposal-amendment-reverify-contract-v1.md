# Proposal Amendment and Re-Verify Contract v1

Issue: #2053 — Era M, Milestone 75

Status: **scope and contract gate**. This document fixes the contract for
proposal amendment and re-verify before implementation. It is documentation
only: it adds no Studio endpoint, no Rust schema, no evaluator rule, no data
store, and no alternate write path.

## Decision summary

Ouroforge supports an opt-in **amend-before-approve** intervention point for
pending proposals. A human may propose an amendment while a proposal is still
under review, but the amendment is **intervention-as-evidence**: a recorded
proposal input that must be validated and re-verified by the existing Rust-owned
gates before any trusted artifact can change.

The Studio posture for this capability is **read + gated-write**. Studio may
show proposal state and capture an amendment request, but it must route that
request to the existing gates. Studio, Phoenix, and Elixir never write trusted
artifacts directly and never own artifact semantics.

The autonomous loop remains agent-first by default. If no human amendment is
provided, the existing autonomous review/apply flow continues and can complete
with zero human input.

## Two-plane invariant

This milestone inherits the Era M two-plane invariant:

- **Rust data plane**: owns proposal artifacts, review/apply validation,
  scene/source-apply, evaluator decisions, deterministic re-verification,
  evidence/provenance, trusted writes, and CLI fallback.
- **Elixir/OTP + Phoenix LiveView control/presentation plane**: may render
  proposal state, capture an amendment request, route it to Rust gates, and show
  pending/accepted/rejected/blocked evidence. It never mutates artifacts,
  bypasses gates, certifies evaluator truth, or stores canonical proposal state.

A captured amendment is not a trusted write. It becomes actionable only after the
Rust data plane validates, records, and re-verifies it through the existing gate
sequence.

## Amendment lifecycle

| Step | Actor/surface | Contract | Gate/evidence reused |
| --- | --- | --- | --- |
| 1. Display pending proposal | CLI or Studio | Shows proposal id, base refs, current review state, linked evidence, stale status, and explicit read + gated-write boundary. | Existing proposal/review/evidence read models. |
| 2. Capture amendment request | CLI command or local Phoenix LiveView Studio | Captures bounded amendment text plus target proposal id, author/source metadata, base proposal ref, rationale, and optional evidence refs. | Evidence/provenance ledger records the request as intervention evidence. |
| 3. Normalize as proposal evidence | Rust gate entrypoint | Treats the amendment as a candidate proposal delta, not as an artifact mutation. Missing/stale base refs fail closed. | Review/apply gate and proposal evidence validation. |
| 4. Re-run validation | Rust evaluator/source/scene gate | Re-runs the same checks that the original proposal must satisfy, plus any scoped scene/source-apply preflight when the amended proposal targets those artifacts. | Existing evaluator, scene/source-apply, source-apply, review/apply checks. |
| 5. Decide amended proposal | Rust-owned decision artifact | Records accepted, rejected, deferred, blocked, or needs-fix state with reasons and evidence refs. | Review decision and provenance ledger. |
| 6. Continue autonomous loop | Agent loop/CLI | Applies only if the governing existing gate accepts the amended proposal. If blocked or absent, the loop follows existing autonomous behavior. | Existing trusted apply path only. |

## Gated write path reused

Amend-before-approve never creates a parallel write path. It reuses existing
capability boundaries:

- **Review/apply gate** for proposal acceptance, rejection, deferral, blocker
  visibility, reviewer independence where required, and decision recording.
- **Scene/source-apply gates** when the amended proposal targets scene or source
  artifacts; preflight and apply remain Rust-owned and evidence-linked.
- **Evaluator gates** for deterministic re-verification of declared acceptance
  conditions and regression dimensions.
- **Evidence/provenance ledger** for recording the amendment request, base refs,
  author/source metadata, validation output, and decision refs.

A Phoenix LiveView control may submit the amendment request to the same local
Rust/CLI path that a CLI command uses. It may not directly edit proposal files,
scene files, source files, ledgers, evaluator verdicts, or generated artifacts.

## Read + gated-write Studio contract

A future local Studio panel for this milestone may include:

- pending proposal summary;
- amendment form bounded to the proposal under review;
- stale base/ref warnings;
- linked evidence display;
- submit-to-gate action that creates an amendment request for Rust validation;
- status display for pending, accepted, rejected, blocked, stale, and malformed
  outcomes.

The panel must also display boundary copy near the amendment action:

> Amendment requests are intervention-as-evidence. Studio captures and routes the
> request; Rust gates validate, record, and re-verify it before any trusted apply.

The panel must not include direct artifact editors, browser file writes, command
bridges, hidden execution, auto-approve, auto-merge, source/scene apply buttons
that bypass the Rust gates, or hosted/collaborative authority.

## CLI fallback

The local-first CLI fallback is first-class. A fresh checkout must be able to run
the full autonomous loop without Studio, Phoenix, Elixir, a database, hosted
service, or human input.

When this capability is later implemented, the CLI path must be sufficient to:

1. inspect pending proposal state;
2. record an amendment request as evidence;
3. re-run the same Rust validation/evaluator gates;
4. record the amendment decision and provenance;
5. continue or block the autonomous loop deterministically.

Studio is an optional local control/presentation surface over that Rust-owned
path, not a prerequisite for completion.

## Failure and hold states

The amendment contract fails closed in these cases:

- target proposal missing or already terminal;
- base proposal ref, run ref, source ref, scene ref, or evidence ref is stale;
- amendment text is empty, unbounded, malformed, or references unsupported target
  classes;
- required review/apply, scene/source-apply, evaluator, or provenance evidence is
  missing;
- amendment attempts to change forbidden governance anchors (#1/#23) outside an
  explicit governance issue;
- Studio or Elixir attempts direct trusted artifact mutation;
- hosted/multi-user/collaborative authority is required;
- the amendment would make human intervention mandatory for autonomous loop
  completion.

Blocked and rejected amendments must keep the blocker evidence visible. They may
not be silently dropped, rewritten, or treated as accepted because the proposal
was otherwise low risk.

## Non-goals

- Raw human writes to trusted artifacts.
- Mandatory human approval or mandatory human amendment.
- New artifact store, database, hosted service, remote collaboration, or account
  model.
- Elixir/Phoenix validation authority or canonical proposal storage.
- New evaluator semantics beyond reusing existing gates.
- Automating fun/taste verdicts or release go/no-go.
- Closing or modifying #1 or #23 outside explicit governance scope.

## Downstream implementation references

Downstream Milestone 75 implementation, demo, and regression issues should treat
this contract as their source of truth:

- #2054 implements the Rust/Studio capture and re-verify path under this gated
  write contract.
- #2055 demonstrates the CLI/Studio amend-before-approve flow while preserving
  zero-human-input autonomous fallback.
- #2056 adds regression coverage for accepted, rejected, blocked, stale, and
  no-human-input cases.

## Acceptance checklist for future code

- Agent-first default preserved; no amendment is required for completion.
- Amendment request is recorded as intervention-as-evidence with provenance.
- Rust owns validation, evaluator re-checks, decisions, and trusted writes.
- Studio is read + gated-write only; Elixir renders/captures/routes.
- Existing review/apply, scene/source-apply, evaluator, and evidence gates are
  reused.
- Missing/stale evidence fails closed.
- CLI fallback remains sufficient and tested.
- Hosted/multi-user Studio remains Layer-3 DEFER.
- Fun/taste and release go/no-go remain human Ring-2 decisions.
- #1 and #23 remain open.
