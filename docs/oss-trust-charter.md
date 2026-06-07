# OSS Trust Charter and Paid-Cloud Boundary Design Gate v1

Status: **Charter adopted — third-rails affirmed; paid-cloud boundary DEFER on
every surface (each tied to a #1508 Layer-3 hosted/cloud GO).**

Decision date: 2026-06-07

This is a design-gate ADR under #1 Era F Milestone 34. It follows the project
idiom for posture/boundary decisions (Distributed / Elixir Design Review ADR #92
in `docs/distributed-elixir-design.md`, the Native Export Design Gate #168, and
the Layer-3 Re-evaluation Design Gate #1508 in `docs/layer3-reevaluation-v1.md`).
**No cloud, hosted, paid, or marketplace capability is implemented by this issue;
it only writes the trust charter and records a per-surface GO/DEFER decision.**

## What this gate decides

1. The canonical trust charter for the open-source core: the licensing pledge, the
   third-rails, and a foundation/governance consideration.
2. A paid-cloud boundary: which operational/team/scale surfaces could ever be
   monetized, and a GO/DEFER for each — every one tied to a #1508 Layer-3
   hosted/cloud GO, DEFER by default.
3. A reaffirmation that absent a Layer-3 GO, only the free local OSS core exists
   and no creative primitive is ever paywalled.

## The trust charter

The Ouroforge core is open-source software and stays that way.

- **Permissive OSS licensing.** The core is licensed under permissive OSS terms
  (MIT today; see [`LICENSE`](../LICENSE)) and any future licensing stays within
  the permissive MIT/Apache-2.0 family. The license file is the source of truth;
  this charter records the pledge, not a license change.
- **No-relicense pledge.** The core is never relicensed away from permissive OSS.
  There is no open-core bait-and-switch, no proprietary relicensing of already-OSS
  code, and no source-available/non-compete license substituted for the core.
- **No-runtime-fee pledge.** Running the local OSS core — generation proposals,
  the deterministic verification loop, the runtime, probe, evaluator, evolve,
  compare, provenance, dashboard, cockpit, source-apply, and CLI surfaces — never
  carries a runtime fee.
- **No-install-fee pledge.** Installing or using the local OSS core never carries
  an install fee, license key, seat charge, or activation gate.
- **No-revenue-share pledge.** Games, projects, evidence, or artifacts produced
  with the OSS core never owe a revenue share, royalty, or take-rate to the
  project.
- **No creative-primitive paywall.** A creative primitive — anything in the
  generate→verify→evolve loop that authors or improves a game — is never
  paywalled. The third-rails below are non-negotiable.

### Third-rails (non-negotiable)

- Never a runtime fee or install fee on the local OSS core.
- Never a revenue share, royalty, or take-rate on what users create with it.
- Never a relicense of the core away from permissive OSS.
- Never a creative primitive behind a paywall.

These third-rails hold regardless of any future Layer-3 GO. A paid-cloud surface,
if one is ever GO, may only charge for an operational/team/scale convenience that
sits beside the OSS core — never for a creative primitive, and never by degrading,
fee-gating, or relicensing the free local core.

### Foundation / governance consideration

To keep the no-relicense pledge credible over time, the project records a
foundation/governance consideration: as adoption warrants, move stewardship of the
core license and trademark toward a neutral, multi-stakeholder governance body
(e.g. a foundation) so that no single party can unilaterally relicense the core or
revoke the third-rails. This is a recorded consideration and direction of travel,
not an implemented governance change and not a commitment with a date; it is
revisited as a separate explicit governance decision.

## Paid-cloud boundary design gate

Monetization, if it ever happens, may touch only the operational/team/scale layer
— hosting, collaboration, and compute convenience around the loop — and never a
creative primitive. Every surface below is **DEFER by default**. Any GO is gated
on a corresponding #1508 Layer-3 hosted/cloud GO (`docs/layer3-reevaluation-v1.md`
records hosted/cloud as **DEFER**), which does not exist today.

| Paid-cloud surface (operational/team/scale only) | Decision | Gated on |
| --- | --- | --- |
| Hosted, searchable evidence-history service | **DEFER** | a #1508 Layer-3 hosted/cloud GO |
| Multi-seat / team collaboration accounts | **DEFER** | a #1508 Layer-3 hosted/cloud GO |
| Managed CI runners for the verification loop | **DEFER** | a #1508 Layer-3 hosted/cloud GO |
| Managed agent compute (hosted generation/evolve runners) | **DEFER** | a #1508 Layer-3 hosted/cloud GO |
| Marketplace take-rate (optional listing/distribution convenience) | **DEFER** | a #1508 Layer-3 hosted/cloud GO |

**DEFER is the default and remains in force.** No surface above is GO. None is
implemented, scaffolded, or scoped by this gate.

### Per-surface rationale, blockers, and revisit criteria

Each surface is operational/team/scale convenience layered beside the OSS core.
The free local equivalent always remains: local evidence/provenance on disk, local
single-user use, the local verification loop, local generation/evolve, and local
distribution of what you build. A paid surface, if ever GO, is an opt-in
convenience and never a precondition for the core.

- **Hosted, searchable evidence-history service — DEFER.** The OSS core writes
  evidence and provenance locally and reads them through read-only surfaces.
  Blockers to a future GO: a #1508 Layer-3 hosted/cloud GO; an authn/authz and
  data-isolation model; preservation of Rust/local trusted ownership and the
  read-only browser boundary; the local evidence path staying fully functional and
  free. Revisit when: a milestone evidences demand for hosted, searchable history
  that the local path cannot meet, with the above in scope.
- **Multi-seat / team collaboration accounts — DEFER.** The core is single-user
  and local-first. Blockers: a #1508 Layer-3 hosted/cloud GO; an account/identity
  and data-ownership model; no degradation of free single-user local use. Revisit
  when: a milestone evidences team-collaboration demand beyond local use.
- **Managed CI runners for the verification loop — DEFER.** The verification loop
  runs locally on the Rust worker pool. Blockers: a #1508 Layer-3 hosted/cloud GO;
  a hosted-execution trust/isolation model; the local loop staying free and
  complete. Revisit when: a milestone evidences demand for managed run capacity the
  local pool cannot meet.
- **Managed agent compute (hosted generation/evolve runners) — DEFER.** Generation
  and evolve run locally as proposals through the existing review/apply/
  trust-gradient path. Blockers: a #1508 Layer-3 hosted/cloud GO; an execution
  trust/isolation and provenance model for hosted runs; proposals staying
  proposal-only with no hosted trusted write; the local path staying free. Revisit
  when: a milestone evidences demand for hosted compute the local path cannot meet.
- **Marketplace take-rate (optional listing/distribution convenience) — DEFER.**
  Distributing what you build stays free and unencumbered; any marketplace would be
  an optional listing convenience, never a tax on creation. Blockers: a #1508
  Layer-3 hosted/cloud GO; a transaction/identity model; an explicit guarantee that
  the take-rate never reaches a creative primitive and that free local distribution
  is unaffected. Revisit when: a milestone evidences optional-distribution demand
  with the above in scope.

## Absent a Layer-3 GO: free local OSS core only

Absent a #1508 Layer-3 hosted/cloud GO, **only the free local OSS core remains.**
No hosted, multi-seat, managed-compute, managed-runner, or marketplace surface
exists, and **no creative primitive is ever paywalled.** The local
generate→verify→evolve loop — generation as proposal-only through the existing
review/apply/trust-gradient path, the deterministic verification loop as the engine
room — stays fully functional and free, with Rust/local retaining trusted ownership
and browser/Studio surfaces read-only. Generation is the front door and the
deterministic verification loop is the engine room: layers, not alternatives.

## Rust-first / local-first preserved

This gate is additive and backward-compatible. Rust/local owns the trusted logic;
browser/Studio/dashboard/cockpit surfaces remain read-only. No runtime, probe,
evaluator four-gate aggregation (`declared-gate-and`), evolve/campaign, compare,
provenance-bundle, dashboard, cockpit, source-apply/trust-gradient, or CLI contract
is changed. No new engine, runtime, writer, language, or dependency is introduced;
distributed/Elixir remains NO-GO per ADR #92. Generated runs/artifacts stay ignored
unless fixture-scoped.

## Conservative wording

This charter makes no production-readiness, quality, fun, shippability,
commercial-readiness, broad genre/engine-breadth, or current Godot
replacement/parity claim. It introduces no auto-merge, auto-apply, self-approval,
reviewer-bypass, or hidden trusted write. The operational/team/scale surfaces it
names are recorded as DEFER possibilities, not offerings, capabilities, or
commitments.

## #1 / #23 governance audit

- #1 (north-star) and #23 (anchor) remain **open** and are neither modified nor
  closed by this gate. #1 remains open; #23 remains open.
- This gate adds a single charter document and authorizes no implementation. It
  records the third-rails and a per-surface DEFER tied to #1508, consistent with
  the Layer-3 hosted/cloud DEFER already on record.

## Revisit / stop criteria

- Re-run this gate at the next milestone that proposes any paid-cloud surface, or
  when a surface's documented blockers above — beginning with a #1508 Layer-3
  hosted/cloud GO — are concretely met by evidence.
- Until then, DEFER stands, the third-rails hold, and no cloud/hosted/paid/
  marketplace implementation may be opened on the basis of this gate.
