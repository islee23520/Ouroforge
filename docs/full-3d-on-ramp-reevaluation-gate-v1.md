# Full-3D On-Ramp Re-evaluation Gate v1

Issue: #2204. Era Q, Milestone 101.

## Decision

**DEFER.** Era Q does not create M102-M106 implementation work in this lane. Full
3D remains a gated future capability, not an active build path, unless a later
explicit governance issue records a new GO decision with stronger demand evidence
and keeps the same clean-room/on-ramp boundaries.

The decision is conservative because the current merged evidence shows a useful
2D and 2.5D migration on-ramp, but not enough demand to justify the moat dilution
and near-second-engine cost of full 3D. Era P already covers the common bounded
case: 3D presentation over 2D-deterministic logic. Full 3D would require a new
runtime surface, deterministic 3D physics, and a two-tier evidence model whose
render evidence can only corroborate state rather than replace deterministic
truth.

## Evidence considered

- **Era O** completed source-project/open-text 2D skeleton import with clean-room
  re-derivation hand-offs, honest fidelity reports, and no auto-port claims.
- **Era P** completed 2.5D presentation import for glTF geometry, orthographic
  cameras, billboards, sprite stacks, 2D-in-3D planes, state-hash-primary
  verification, and perceptual render secondary corroboration.
- **Era R** completed the clean-room re-derivation path for behavior-bearing
  units through interrogation, oracle capture, deterministic re-expression,
  differential verification, semantic-port coverage, and human intent/feel
  escalation.
- Full 3D import would add material new requirements: a glTF 3D web runtime,
  deterministic 3D physics re-simulation, cross-platform state-hash evidence,
  perceptual render comparison as secondary evidence, shader/VFX re-authoring or
  baking, and more unsupported/gap attribution.
- No merged evidence in this lane proves demand strong enough to outweigh those
  costs or to justify creating M102-M106 work now.

## Per-capability GO/DEFER record

| Capability | Decision | Evidence / condition | Downstream effect |
| --- | --- | --- | --- |
| glTF 3D scene import and normalization (M102) | **DEFER** | 2.5D glTF presentation import is complete; full 3D scenes need a separate runtime, broader materials/lights/cameras/skeleton/morph support, and stronger demand evidence. | Do not create implementation work. If later GO, build source-project/open-text import only and preserve fidelity gaps. |
| Deterministic 3D physics re-simulation (M103) | **DEFER** | Imported physics cannot be reproduced; a deterministic fixed-step 3D physics stack with pinned operation order is required before any full-3D claim. | Do not create implementation work. If later GO, re-simulate intent natively; never reproduce foreign runtime physics. |
| Two-tier 3D evidence model (M104) | **DEFER** | 2D bit-exact hashes remain the moat. 3D can only use deterministic state-hash primary evidence with perceptual render secondary corroboration. | Do not create implementation work. If later GO, render diff/SSIM never becomes trusted state authority. |
| 3D logic re-derivation hand-off and demo (M105) | **DEFER** | Era R can handle behavior, but no full-3D import should claim behavior without captured oracles and passing differential evidence. | Do not create implementation work. If later GO, every behavior-bearing unit routes to Era R and remains unported until oracle-gated. |
| Era Q outcome governance (M106) | **DEFER until a future GO** | With M101 DEFER, there is no Q build-out to close. | Keep #1/#23 open; record this gate and stop Q implementation creation. |

## Requirements if a future GO is recorded

A later GO must be explicit and must cite this ADR. It must include all of the
following before any implementation issue is created:

1. **Demand evidence** strong enough to outweigh the added engine/runtime surface.
2. **Source-only legal inputs**: source-project/open-text assets only; no shipped
   build ripping, binary extraction, player-data scraping, or decompiled-code
   copying.
3. **One-way import** into Ouroforge-native artifacts only; no live bridge and no
   embedded Unity/Unreal/Godot runtime.
4. **Clean-room re-derivation** for behavior-bearing units; re-derivation is not
   translation and no unit is called `ported` without captured passing oracle
   evidence.
5. **Deterministic 3D physics re-simulation** with fixed timestep, pinned
   operation order, seeded randomness, no uncontrolled nondeterminism, and a
   clear statement that imported physics is re-simulated, never reproduced.
6. **Two-tier evidence** where deterministic state-hash evidence is primary and
   perceptual render comparison (SSIM/pixel-diff, with any pinned-GPU exact-hash
   mode clearly optional) is secondary only.
7. **Honest fidelity grades** where gaps remain Yellow/Red with attribution and
   unsupported shaders/VFX/materials/cameras/physics/logic are not laundered into
   Green.
8. **Two-plane ownership**: Rust owns artifact semantics, validation,
   determinism, fidelity reports, evidence, and gated writes; Elixir/Phoenix
   Studio remains local control + presentation only and routes writes through the
   existing `ouroforge` CLI/gates.
9. **No new trusted write path or data store** from this gate.
10. **Human Ring 2 authority** for fun/feel, creative acceptance, and release
    go/no-go.

## Era R hand-off contract

Full-3D candidate inputs, if ever GO-gated, split as follows:

- Declarative scene/presentation assets may enter a source-only adapter and
  fidelity report.
- Behavior-bearing units, gameplay scripts, animation events that mutate state,
  physics authority, shader/VFX behavior, tacit feel, and source-engine runtime
  semantics become Era R re-derivation tasks.
- A re-derivation task must carry provenance, observed behavior or interrogated
  intent, oracle requirements, deterministic state-hash expectations, and known
  fidelity gaps.
- No downstream artifact may use `ported`, complete, equivalent, or done wording
  until its captured oracle and deterministic differential evidence pass.

## Completion statement

M101 records a DEFER decision for full 3D. M102-M106 remain GO-gated and should
not be created or implemented from this lane. The bounded on-ramp remains Era O/P
source-project/open-text skeleton and presentation import plus Era R clean-room
behavior re-derivation. #1 and #23 remain open governance anchors.
