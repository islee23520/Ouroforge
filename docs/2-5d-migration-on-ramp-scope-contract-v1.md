# ADR: 2.5D Migration On-Ramp Scope and Contract v1

Status: accepted for Era P / Milestone 96 (#2191)  
Date: 2026-06-10 KST  
Scope: 2.5D migration on-ramp design gate only; no implementation in this milestone.

## Decision

Ouroforge defines **2.5D** as:

> **3D presentation over 2D-deterministic logic.**

Era P is a one-way migration **on-ramp** for source-project, open/text 2.5D content. It imports and normalizes the declarative presentation skeleton into Ouroforge-native artifacts, then re-derives behavior clean-room through the existing semantic re-derivation/oracle workflow. It is not engine absorption, not a live bridge, not an embedded Unity/Unreal/Godot runtime, and not a finished-game auto-port.

The determinism moat remains the product boundary:

- **Logic/gameplay truth stays 2D-deterministic.** Gameplay state, rules, collision intent, triggers, timers, scores, inventory, and acceptance oracles are represented as deterministic Ouroforge logic and evidence, not as imported engine runtime behavior.
- **3D is presentation only in Era P.** glTF meshes, orthographic/isometric cameras, billboards, and sprite stacks may affect what the player sees, but they do not own authoritative game state.
- **Verification is two-tier for rendered 2.5D evidence.** Deterministic state-hash remains primary; perceptual render comparison (SSIM/pixel-diff) is secondary corroboration for presentation fidelity.
- **Physics is re-simulated, never reproduced.** If a source project used engine physics to create a 2.5D feel, Ouroforge records that as a re-derivation task and re-expresses deterministic behavior from observed acceptance evidence and interrogated intent.

## Import vs re-author split

| Source-project material | Era P treatment | Truth owner | Fidelity rule |
| --- | --- | --- | --- |
| glTF geometry, meshes, node transforms, static scene hierarchy | Import and normalize to Ouroforge-native presentation artifacts | Rust data plane | Best-effort import with provenance and fidelity grade |
| Orthographic/isometric cameras | Import normalized projection/camera intent for presentation | Rust data plane | State-hash unaffected; render comparison is secondary |
| Standard materials/textures compatible with glTF-style PBR or unlit workflows | Import when source/text/open and deterministic metadata is available | Rust data plane | Honest fidelity grade; unsupported shader features become gaps |
| Billboards, camera-facing sprites, sprite stacks, pixel-art filtering, alpha sorting | Re-author as Ouroforge presentation primitives over deterministic state | Rust data plane for artifacts; runtime presentation for rendering | Presentation evidence only; cannot mutate gameplay truth |
| Gameplay scripts, engine callbacks, triggers, AI, timers, scoring, win/lose logic | Re-derive clean-room through Era R semantic re-derivation | Rust deterministic logic/evaluator | Nothing is claimed ported without captured oracle evidence |
| Source engine physics, shaders, VFX, animation side effects | Re-author or mark as a fidelity gap/re-derivation task | Rust evidence/fidelity ledger | Never copy decompiled source; never promise 1:1 reproduction |
| Studio wizard and review UI | Present Rust-owned evidence and route gated operations | Elixir/Phoenix control + presentation plane | No trusted writes and no artifact semantics in Studio |

## Allowed source boundary

Era P inherits the Era O legal/source boundary:

- Allowed: source projects and open/text formats, including Godot `.tscn`/`.tres` and Unity Force-Text YAML plus `.meta`, and glTF assets that the user is authorized to import.
- Not allowed: shipped-build ripping, binary asset extraction from third-party games, decompiled source copying, or translating engine source code into Ouroforge code.
- Clean-room rule: behavior is re-implemented from observed behavior, user/interrogated intent, and acceptance oracles; source engine code is not copied.

## Two-plane contract

Era P preserves the existing two-plane architecture:

- **Rust data plane:** `crates/ouroforge-core` and `crates/ouroforge-evaluator` own adapters, IR, mapping, extraction, re-expression, evidence, fidelity reports, deterministic hashes, and gates.
- **Elixir/Phoenix control + presentation plane:** `studio/` may render Rust-owned evidence, help humans review gaps, and invoke the `ouroforge` CLI/gates. It performs no trusted writes, owns no artifact semantics, and introduces no new data store.

Every write that changes artifacts or evidence must flow through the existing gated write path (`source-apply` / `scene-apply` / evaluator gates as applicable). Studio may request, display, or route; it may not bypass.

## Era R hand-off contract

Era P creates two categories of downstream work:

1. **Imported presentation skeleton artifacts** for meshes/cameras/material references/billboard definitions/sprite-stack presentation. These flow forward as Ouroforge-native presentation artifacts with provenance and fidelity grades.
2. **Re-derivation work items** for logic/physics/shader/VFX/tacit-feel gaps. These flow to Era R with:
   - the source-project reference and open/text provenance,
   - observed behavior captures where available,
   - interrogated human intent or tacit acceptance notes,
   - oracle scenarios and expected deterministic state outcomes,
   - an explicit fidelity grade / coverage verdict.

A re-derivation unit is not done until it has captured acceptance evidence and passes the relevant oracle. If the oracle is absent, incomplete, or failing, the fidelity report must say so. The correct label is a gap or re-derivation task, not "ported".

## Verification and fidelity rules

Era P verification is intentionally asymmetric:

- **Primary gate:** deterministic state-hash for the authoritative 2D logic state.
- **Secondary gate:** perceptual render evidence for 2.5D presentation (SSIM/pixel-diff or equivalent tolerance-based comparison).
- **Coverage verdict:** every imported or re-authored unit is graded honestly (for example, clean/imported, partial/re-authored, blocked/re-derivation required, unsupported/deferred).
- **Oracle-gated language:** user-facing reports must not claim a unit is ported without a captured passing oracle and supporting evidence.

This preserves bit-exact 2D determinism while allowing 3D-looking presentation to be assessed with practical visual tolerances.

## Non-goals

- No finished-game auto-port.
- No live bridge to Godot, Unity, Unreal, or another source engine.
- No embedded source-engine runtime.
- No shipped-build ripping or decompiled-code translation.
- No full-3D gameplay/physics import in Era P; the full-3D decision remains Era Q and is DEFER by default.
- No new data plane, persistent store, or ungated Studio write path.
- No automation of fun/feel or release go/no-go; those remain human Ring 2 decisions.

## Downstream citation requirement

Milestones 97-100 should cite this ADR when implementing glTF geometry/camera import, billboard and sprite-stack presentation, 2.5D fidelity reports, and Era P governance. If a downstream issue needs behavior not covered here, it must record it as a bounded extension or a DEFER item rather than weakening the determinism/on-ramp boundary.
