# ADR: Distribution Delegation Decision Gate v1

## Status

Accepted for Era X Milestone 136 (#2540, parent SSOT #2517). This ADR is a
design-only decision record: it authorizes no packaging, export, signing,
store, upload, or release implementation. Any implementation requires its own
future milestone with explicit governance.

## Context

Ouroforge's roadmap non-goals exclude native export, hosted services, and
public-launch automation, while the engine's practical loop increasingly
produces playable local web games (Signal Gate Relay, Crate Garden Puzzle)
with local web package evidence (Build/Export/Packaging v1, the #2393 local
package handoff, the #2498 inspection-handoff increment). This leaves an
unresolved tension: what is the sanctioned story when a dogfood game is good
enough that someone wants to hand it to another person?

Three candidate paths were evaluated, informed by the externally verified gap
research recorded in #2517:

- **(A) Web-first local package distribution as the canonical path.** Extend
  the existing fixture-scoped local web package/export evidence lineage
  (deterministic staging, asset manifests, checksums/provenance, runtime probe
  preservation) so a verified local web bundle is the one supported
  distribution artifact. External calibration: the browser is a first-class
  commercial target now (Unity 6 web push), so web-only distribution is not a
  dead end; and the existing Era I "Steam-shaped desktop export" artifacts
  remain local evidence contracts, not shipping paths.
- **(B) One-way external export handoff.** Emit an importable project skeleton
  for an external engine (directionally consistent with the Era O one-way
  philosophy, reversed in direction). This would delegate native packaging,
  certification, and store submission to a mature ecosystem, at the cost of a
  second adapter surface to govern, fidelity-honesty obligations in the export
  direction, and a standing temptation toward engine-bridge scope creep.
- **(C) Explicit deferral.** Record no distribution stance and revisit later;
  cheapest now, but it leaves every future dogfood postmortem without a
  sanctioned answer to "how would this reach a player," and invites ad-hoc
  improvisation — the exact failure mode the M115 rebaseline corrected.

## Decision

**Ouroforge adopts (A): web-first local package distribution is the canonical
distribution path.** The verified local web bundle — deterministic staging,
asset manifest, checksums, provenance, preserved runtime probe — is the one
artifact Ouroforge stands behind for handing a game to another person, served
or shared locally. The existing Build/Export/Packaging v1 and local package
handoff contracts are the substrate; future distribution work extends that
lineage rather than opening native paths.

Option (B) is **not rejected permanently**; it is parked behind the
re-evaluation triggers below. Option (C) is rejected: a named stance is itself
the deliverable of this gate.

Signing, store submission, upload, credentialed release flows, hosted
distribution, and public-launch automation remain out of scope regardless of
this decision, per the standing roadmap non-goals.

## Consequences

- Dogfood postmortems (M135 and later) may cite the local web package as the
  sanctioned distribution answer, and package smoke evidence stays a
  first-class part of product-observed cycles.
- No new adapter surface or export-direction fidelity obligations are created
  now; governance attention stays on the Era X iteration-experience milestones.
- Anyone needing desktop/store distribution today is outside Ouroforge's
  supported scope, and docs must say so honestly rather than implying a
  roadmap promise.

## Re-evaluation triggers

Open a new design issue to reconsider option (B) if any of the following is
observed and recorded in evidence:

1. A completed dogfood cycle's postmortem records a concrete, blocking need
   for desktop or store distribution that the web package demonstrably cannot
   satisfy.
2. The web runtime hits a verified capability wall (performance budget,
   platform API, input class) that materially degrades a shipped dogfood game
   and has no web-side remediation.
3. An external user/maintainer decision explicitly prioritizes a distribution
   channel that requires native packaging.

Reconsidering (B) requires the Era O invariants restated in the export
direction: one-way emission, no live bridge, no embedded foreign runtime, and
honest fidelity reporting on what the exported skeleton does not carry.

## Boundaries

This ADR does not authorize: native/mobile/console export implementation,
signing, store submission, upload, credentialed operations, hosted/cloud
distribution, public release automation, engine bridges, or any change to the
trusted-write gates. #1 and #23 remain open governance anchors.
