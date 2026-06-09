# 2D Migration On-Ramp Scope, IR, and Legal/Fidelity Contract v1

Issue: #2167 — Era O, Milestone 88.

This ADR is the design-gate contract for the External-Engine 2D Migration
On-Ramp. Downstream Era O issues must cite and preserve this document's scope,
IR vocabulary, fidelity/oracle rule, and legal boundary.

## Decision

Ouroforge may import a source project's **declarative skeleton** into
Ouroforge-native artifacts and may then support **clean-room semantic
re-derivation** of game logic through Era R. This is a one-way on-ramp, not
engine absorption, not a live bridge, and not an automated finished-game port.

The on-ramp is deliberately two-plane:

- **Rust data plane** owns parsing, source-project validation, neutral IR,
  mapping, deterministic gates, evidence, fidelity reports, and trusted writes.
- **Elixir/Phoenix Studio control + presentation plane** may render Rust-owned
  evidence and route user intent through the `ouroforge` CLI/gates, but it owns
  no artifact semantics and performs no trusted writes.

## Importable 2D subset

The first supported subset is Godot-first, Unity second. It admits only
source-project, open/text inputs:

- Godot: `.tscn` / `.tres` text resources and project-local assets referenced by
  those resources.
- Unity: Force Text YAML scene/prefab/asset metadata plus `.meta` GUID files.

The importable skeleton is limited to declarative 2D presentation and structure:

| Category | In scope | Out of scope / Era R hand-off |
| --- | --- | --- |
| Scene hierarchy | Stable node/entity tree, names, parent-child relationships, enabled/visible flags, transforms | Runtime-created nodes, behavior-only scene mutations |
| Sprites | Texture references, atlas/region metadata when expressible, tint/modulate, draw order/layer | Shader-specific rendering, procedural materials, engine-specific effects |
| Tilemaps | Tile set references, cell coordinates, layer order, collision-shape references when declarative | Runtime tile mutation logic, custom terrain/autotile behavior beyond recorded metadata |
| Camera | 2D camera transform, zoom/viewport hints, smoothing flags as metadata | Engine-specific camera scripts, cinematic logic |
| Collider shapes | Declarative rectangles/circles/polygons/segments, trigger/sensor flags, layer/mask metadata | Attempted reproduction of engine physics solvers; physics is re-simulated in Ouroforge |
| Input | Declarative action names and bindings when present in source text | Input handling scripts and gameplay reactions |
| Animation/presentation metadata | Named clips/timelines only when source text exposes keyframes cleanly | Runtime animation controllers, state machines, script callbacks |
| Logic | Touchpoint inventory only: script references, signals/events, scene callbacks, exported variables | Translation or copying of source code; all logic goes to Era R semantic re-derivation |

Anything outside this subset is represented as a gap, a logic touchpoint, or an
unsupported source feature in the fidelity report. It must not be silently
flattened into a false success.

## Neutral IR schema contract

Adapters normalize source text into an adapter-agnostic neutral IR before any
Ouroforge mapping. The Rust implementation may evolve concrete structs, but the
IR must preserve these semantic groups:

```text
MigrationIr
  source: SourceProjectRef
  legal_boundary: SourceOnlyCleanRoomBoundary
  scenes: Vec<IrScene>
  assets: Vec<IrAssetRef>
  inputs: Vec<IrInputAction>
  logic_touchpoints: Vec<IrLogicTouchpoint>
  unsupported: Vec<IrUnsupportedFeature>
  provenance: Vec<IrProvenanceRef>
```

```text
IrScene
  id, source_path, engine, engine_version?, root: IrNode

IrNode
  id, source_stable_id?, name, kind, parent_id?, order
  transform2d, visibility, presentation?, collider?, camera?, tilemap?
  metadata: bounded key/value facts that do not grant runtime semantics
  provenance: source file/range/resource identifier when available

IrAssetRef
  id, source_path, kind, digest?, import_hint, legal_status

IrInputAction
  name, bindings, source_scope, provenance

IrLogicTouchpoint
  id, source_path, node_id?, trigger_kind, symbol/name?, exported_variables,
  observed_behavior_refs?, era_r_status

IrUnsupportedFeature
  source_path, node_id?, feature_kind, severity, reason, suggested_hand_off
```

The IR is not an execution format. It is a loss-accounted interchange contract
between source adapters, mapping, verification, reports, and Era R hand-off.
Adapters must keep provenance sufficient for a reviewer to trace each imported
fact back to a source-project text artifact without copying decompiled source.

## Fidelity grades and oracle rule

Each imported unit receives one of three fidelity grades:

- **🟢 Green — structurally mapped.** The declarative skeleton fact is supported,
  mapped to Ouroforge-native artifacts, and verified by deterministic evidence.
- **🟡 Yellow — partial / best-effort.** The unit is represented with known gaps
  or metadata-only preservation. The report states the gap and its consequence.
- **🔴 Red — re-derivation required / unsupported.** The unit contains logic,
  engine-specific behavior, unsupported presentation, or legal/format ambiguity.
  It is not ported; it becomes an Era R re-derivation or human decision item.

**Oracle-gated rule:** no unit may be described as "ported", "equivalent", or
"complete" unless captured acceptance evidence passes. For 2D, acceptance gates
must include bit-exact deterministic state hashes for the relevant Ouroforge
state. For 2.5D/3D-adjacent presentation, deterministic state hash remains
primary; perceptual render comparison (SSIM/pixel-diff) is secondary evidence
only. Imported physics is re-simulated in Ouroforge and may receive at most a
best-effort fidelity grade until Ouroforge-native behavior passes its oracle.

The fidelity report is a product artifact, not a marketing claim. It must list:
source files considered, imported units, unsupported units, logic touchpoints,
clean-room hand-offs, evidence references, and explicit non-equivalence notes.

## Legal and clean-room boundary

The on-ramp accepts source projects supplied by a user/operator in source-like
open/text formats only. It rejects shipped-build ripping, embedded proprietary
engine runtime extraction, decompiled source, and binary-only games.

Required legal posture:

1. **One-way import:** source facts become Ouroforge-native artifacts; no live
   bridge to Unity, Unreal, Godot, or any other engine runtime is created.
2. **No embedded engine runtime:** Ouroforge does not ship or execute the source
   engine as part of the imported game.
3. **Clean-room logic:** adapters may inventory script references, exported
   variables, signal names, and behavior touchpoints, but they must not copy,
   translate, or rephrase decompiled source code into Ouroforge logic.
4. **Source-project provenance:** every imported fact must point to an allowed
   source-project artifact and format.
5. **Human-owned fun/feel/release:** Ring 2 human review decides whether the
   re-derived result feels acceptable and whether release is appropriate.

## Era R hand-off contract

Era O hands off **logic touchpoints**, not translated logic. Each hand-off item
must include:

- source scene/node/asset provenance;
- the triggering surface (script reference, signal/event binding, animation
  callback, input action, physics callback, or exported variable);
- observable behavior evidence if captured;
- missing oracle(s) needed to claim equivalence;
- fidelity grade and gap explanation;
- a clean-room instruction to re-implement behavior from observed behavior and
  interrogated intent, never from copied/decompiled source.

Era R returns Ouroforge-native behavior plus oracle evidence. Until that happens,
Era O reports the unit as a skeleton import with a re-derivation hand-off, not as
a completed port.

## Governance invariants for downstream issues

- Godot adapter work precedes Unity adapter work.
- Rust modules under `crates/ouroforge-core` and `crates/ouroforge-evaluator`
  own adapters, IR, mapping, extraction, re-expression, and gates.
- New Rust modules use explicit `pub mod x;` declarations and module-path imports;
  they must not add broad `pub use x::*` exports that reintroduce `lib.rs`
  module-block conflicts.
- Phoenix/LiveView surfaces under `studio/` are local control/presentation only
  and must route every write through `ouroforge` CLI/gates.
- No new data store or trusted write path is introduced for migration UX.
- Gaps remain visible. A yellow or red unit is successful only if the report is
  honest about why it is not fully verified.

## Verification anchors

A downstream reviewer should verify this ADR by checking that documentation and
implementation preserve the following exact concepts: one-way on-ramp,
re-derivation, fidelity report, two-plane architecture, source-project text
formats, oracle-gated claims, and clean-room legal boundary.
