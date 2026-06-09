# Godot 2D Adapter to IR Contract v1

Issue: #2168 — Era O, Milestone 89.

This contract narrows the general [2D Migration On-Ramp Scope, IR, and
Legal/Fidelity Contract v1](./2d-migration-on-ramp-scope-ir-legal-v1.md) for the
Godot 2D adapter. It fixes what the adapter may read, what neutral IR it may
emit, how fidelity is graded, and what must be handed to Era R for clean-room
semantic re-derivation.

## Decision

The Godot adapter is a **source-project text adapter**. It parses Godot `.tscn`
and `.tres` resources from a user-supplied source tree and emits neutral
migration IR plus a fidelity report. It does not execute Godot, embed Godot,
bridge to a Godot runtime, rip exported games, translate GDScript, or claim a
finished game has been ported.

The adapter is Rust data-plane work. Any Studio/Phoenix surface may only display
Rust-owned evidence and route explicit writes through the existing `ouroforge`
CLI/gates; it owns no source artifact semantics and performs no trusted writes.

## Accepted inputs

The adapter accepts only source-project, open/text Godot artifacts:

- `.tscn` text scenes;
- `.tres` text resources referenced by accepted scenes/resources;
- project-local asset paths referenced from those text files;
- `project.godot` only for bounded metadata such as input-map declarations when
  it is available as text.

Inputs must be rejected or marked 🔴 when they require:

- exported/shipped build extraction;
- binary-only packed resources;
- decompiled GDScript or C# source from a shipped artifact;
- executing Godot editor/runtime code;
- network or package-manager resolution;
- engine plugins or custom importers as trusted code.

## Godot subset mapped to neutral IR

| Godot source fact | Neutral IR output | Grade floor | Notes |
| --- | --- | --- | --- |
| `Node2D` / `CanvasItem` hierarchy, names, ownership, visibility | `IrScene.root` / `IrNode` tree | 🟢 if parsed and provenance is complete | Runtime-spawned nodes are not inferred. |
| 2D transform fields (`position`, `rotation`, `scale`, `z_index`) | `IrNode.transform2d` and presentation order metadata | 🟢 | Engine-specific transform side effects remain out of scope. |
| `Sprite2D` / region/texture refs | `IrNode.presentation.sprite` + `IrAssetRef` | 🟢/🟡 | Atlas/region maps green only when source text is explicit and local. |
| `TileMap` / `TileSet` declarative cells and layers | `IrNode.tilemap` + tile asset refs | 🟡 | Godot terrain/autotile/runtime tile behavior is a gap unless fully explicit. |
| `Camera2D` fields | `IrNode.camera` | 🟢/🟡 | Smoothing/limits may be metadata-only if Ouroforge lacks equivalent behavior. |
| `CollisionShape2D`, `CollisionPolygon2D`, `Area2D`, `StaticBody2D`, `CharacterBody2D`, `RigidBody2D` declarations | `IrNode.collider` and physics metadata | 🟡 | Shapes map; Godot physics behavior is re-simulated, never reproduced. |
| `InputMap` action/binding declarations from source text | `IrInputAction` | 🟢/🟡 | Gameplay reactions to input are logic touchpoints. |
| Signals, script refs, exported variables, animation method tracks/callbacks | `IrLogicTouchpoint` | 🔴 | Era R hand-off only; no GDScript/C# translation. |
| Shaders, particles, custom resources, plugins, editor scripts | `IrUnsupportedFeature` | 🔴 | Preserve provenance and reason; do not silently flatten. |

## Adapter outputs

For each import run, Rust must produce:

1. Neutral `MigrationIr` containing source reference, scenes, nodes, assets,
   input actions, logic touchpoints, unsupported features, and provenance.
2. A fidelity report that lists every accepted, partial, unsupported, and
   re-derivation-required unit.
3. Evidence references for parser coverage and deterministic normalization.
4. Era R hand-off records for all script/signal/callback/input/physics logic
   units.

The adapter output is not an Ouroforge game and not an apply operation. Mapping
and gated application happen in later milestones through existing
source-apply/scene-apply paths.

## Fidelity and oracle rule

Grades use the M88 vocabulary:

- **🟢 Green:** declarative Godot text fact maps to neutral IR with source
  provenance and deterministic parser evidence.
- **🟡 Yellow:** the fact is imported as metadata or partial skeleton with an
  explicit gap in the fidelity report.
- **🔴 Red:** the fact requires unsupported engine behavior, copied/decompiled
  code, or clean-room logic re-derivation. It is not ported.

No Godot-derived unit may be called "ported" or "equivalent" until a later
Ouroforge-native mapping passes its captured oracle. For 2D that oracle includes
bit-exact deterministic state hashes. Render/perceptual comparisons may support
presentation review, but they do not replace deterministic state evidence.

## Era R clean-room hand-off

The Godot adapter must inventory, not translate, logic. Each hand-off item
records:

- source file and node/resource provenance;
- script path or signal/callback/input/action name when present;
- exported variables or declarative configuration visible in source text;
- missing acceptance oracle(s);
- any observed behavior evidence supplied by a later capture flow;
- the instruction: re-implement in Ouroforge from observed behavior and
  interrogated intent, never from copied/decompiled source.

## Trusted boundary and non-goals

- One-way on-ramp only; no live Godot bridge and no embedded Godot runtime.
- Source-project text formats only; no shipped-build ripping.
- Re-derivation, not translation; no GDScript/C# source copying into Ouroforge.
- Rust owns parsing, IR, reports, evidence, and gates.
- Phoenix/LiveView is presentation/control only and may not mutate artifacts
  outside gated CLI flows.
- Fun/feel and release go/no-go remain human Ring 2 decisions.

## Verification anchors

Downstream implementation and demo issues should reference this contract when
they add Godot parsing, IR fixtures, fidelity reports, and Studio display. A
reviewer should be able to audit the implementation by searching for the terms
one-way, on-ramp, re-derivation, fidelity, two-plane, source-project, clean-room,
Godot `.tscn`, and Godot `.tres` in the docs and evidence it emits.
