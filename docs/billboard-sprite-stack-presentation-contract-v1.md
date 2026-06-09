# Contract: Billboard, Sprite-Stack, and 2D-in-3D Presentation Layer v1

Status: accepted for Era P / Milestone 98 (#2196)
Date: 2026-06-10 KST
Parent boundary: `docs/2-5d-migration-on-ramp-scope-contract-v1.md`
Related contract: `docs/gltf-geometry-orthographic-camera-import-contract-v1.md`
Scope: documentation/design contract only; no Rust, Elixir, or JavaScript implementation in this milestone.

## Goal

Milestone 98 fixes the contract for importing and re-authoring **billboards, sprite stacks, and 2D-in-3D presentation layers** for Era P's 2.5D on-ramp. The milestone defines what can become Ouroforge-native presentation evidence, what must be re-authored, and what must be handed to Era R as clean-room re-derivation work.

This is a one-way on-ramp from source-project/open-text presentation skeletons to Ouroforge-native artifacts. It is not engine absorption, not a live bridge, not an embedded runtime, and not a finished-game auto-port.

## Accepted inputs

Inputs must be authorized source-project material or open/text presentation data:

- Godot source-project `.tscn` / `.tres` nodes and resources that describe `Sprite2D`, `AnimatedSprite2D`, `Sprite3D`, billboard flags, texture references, z/y sort hints, and import metadata.
- Unity Force-Text YAML plus `.meta` records for sprites, sprite renderers, sorting layers, billboarding hints, camera-facing presentation components, and texture import settings.
- glTF source-project nodes, planes, materials, alpha mode, texture coordinates, and extension metadata that describe presentation sprites or billboard-like quads.
- Pixel-art sprite-stack manifests, layer lists, slice spacing, pivot/origin declarations, palette/filtering metadata, and deterministic draw-order hints.
- Provenance records tying every input to source-project/open-text origins and declared import roots.

Rejected inputs are 🔴 by default:

- shipped-build ripping, binary-only asset extraction from third-party games, or unlicensed dumps,
- copied/decompiled source scripts or translated engine code,
- live runtime bridge descriptors or engine-execution hooks,
- shader/VFX/physics behavior presented as if it were sprite presentation,
- gameplay logic, hit detection, AI, timers, score, trigger, quest, or win/loss state embedded in sprite metadata.

## Importable subset and fidelity grades

| Area | 🟢 Green / importable presentation | 🟡 Yellow / partial or re-author | 🔴 Red / re-derive or defer |
| --- | --- | --- | --- |
| Billboard orientation | Static camera-facing mode, axis lock, pivot, anchor, and deterministic sort hints that do not change gameplay truth | Engine-specific billboard modes that can be approximated as presentation with named tolerance | Runtime scripts that steer camera, aim, hit boxes, collisions, AI, or gameplay state |
| Sprite stacks | Ordered slices/layers, per-slice offsets, pivots, spacing, alpha, and pixel filtering metadata with deterministic draw order | Ambiguous slice spacing, missing pivots, palette/filter mismatch, or unsupported blend modes | Procedural stack generation, gameplay-bearing slice visibility, or physics/collision derived from stack layers |
| 2D-in-3D planes | Quad/sprite plane transform, texture/material refs, UVs, alpha mode, sorting layer, and camera/canvas relationship | Non-standard material extensions or approximate perspective/parallax presentation | Full-3D gameplay mesh logic, engine cameras/scripts, or non-deterministic physics authority |
| Animation | Presentation-only frame list, timing metadata, loop mode, and static atlas refs when deterministic and non-authoritative | Unsupported easing, blend, shader, or timing gaps recorded as fidelity warnings | Animation events that mutate gameplay state or encode triggers/attacks/win conditions |
| Materials and textures | Source-project texture refs, sampler/filtering intent, alpha mode, color-space notes, and atlas/frame refs | Custom shaders, lighting hacks, palette swaps, particles, or VFX recorded as gaps | Shader code, VFX behavior, lighting simulation, or exact reproduction claims |
| Logic coupling | None for imported presentation | Named coupling note that creates an Era R work item | Any claim that behavior was ported, auto-translated, or completed without oracle evidence |

Green means the unit is deterministic presentation evidence only. Yellow means useful presentation facts were preserved but fidelity gaps or re-authoring work remain. Red means the unit is outside M98 and must be rejected, deferred, or sent to Era R.

## Rust-owned outputs

Downstream implementation must produce Rust-owned artifacts only:

- an Ouroforge-native presentation-layer report for billboards, sprite-stack layers, 2D-in-3D planes, draw ordering, texture refs, pivots, anchors, and camera-facing constraints;
- source-project/open-text provenance for each imported unit;
- a fidelity report with explicit 🟢/🟡/🔴 rows and gap reasons;
- deterministic presentation normalization metadata such as axis lock, origin, sort key, pixel filtering, alpha mode, palette/color-space note, slice spacing, and camera relationship;
- deterministic state-hash primary evidence for any authoritative state touched by the candidate artifact;
- perceptual render evidence only as secondary corroboration for presentation fidelity;
- Era R hand-off rows for logic, physics, animation events, shader/VFX behavior, or tacit feel that cannot be imported as presentation-only.

Studio may render these outputs and route gated CLI actions, but it owns no artifact semantics and performs no trusted writes.

## Gated path

M98 implementation and demo milestones must reuse existing Ouroforge gates:

1. Validate source-project/open-text provenance, path containment, and clean-room boundaries in Rust.
2. Parse presentation declarations into adapter/IR records without executing source-engine code.
3. Normalize the accepted subset into Ouroforge-native presentation artifacts.
4. Emit fidelity rows before any user-facing success claim.
5. Route writes through existing `ouroforge` CLI, evaluator, `scene-apply`, or `source-apply` gates; no new write path or data store is allowed.
6. Present evidence in Studio only as a control/presentation surface; no Elixir/Phoenix trusted write or artifact-truth authority is allowed.

## Oracle and no-port rule

M98 can claim a presentation unit is imported or re-authored only within the fidelity grade shown in its report. It must not claim that gameplay behavior, source-engine physics, animation side effects, shader behavior, or a whole game was `ported`, auto-ported, auto-translated, or completed without captured acceptance evidence and a passing oracle.

Behavior-bearing rows become Era R tasks with:

- the source-project reference and presentation unit identifier,
- the observed behavior or missing-observation note,
- the human/interrogated intent needed to re-derive it,
- the oracle scenario or missing-oracle blocker,
- deterministic state expectations for the Ouroforge-native re-expression,
- the reason the unit is not presentation-only.

## Determinism and render evidence

- Authoritative state is deterministic state-hash primary evidence.
- 2.5D render evidence is perceptual secondary evidence, such as SSIM/pixel-diff tolerance reports or fixture-backed pixel smoke checks.
- Sorting, pivots, frame order, layer order, slice spacing, and camera-facing constraints must be deterministic for identical inputs.
- Imported physics is never reproduced; collision or interaction semantics are re-simulated or re-derived through Era R.

## Downstream citation requirement

Milestone 98 implementation, demo, and coverage issues (#2197-#2199) must cite this contract and the parent 2.5D ADR. If downstream work discovers a new billboard/sprite-stack/2D-in-3D case, it must add a bounded fidelity row or explicit DEFER item without weakening the one-way/source-only/clean-room boundary.

## Non-goals

- No finished-game auto-port.
- No live bridge to Godot, Unity, Unreal, glTF, or another source runtime.
- No embedded runtime or source-engine execution.
- No shipped-build ripping or decompiled-code copying.
- No full-3D gameplay/physics import; Era Q remains a DEFER-default gate.
- No shader/VFX/physics 1:1 reproduction promise.
- No new data plane, persistent store, or ungated Studio write path.
- No automation of fun/feel or release go/no-go; those remain human Ring 2 decisions.
