# IR to Ouroforge Mapping and Fidelity Classifier Contract v1

Issue: #2172 — Era O, Milestone 90.

This contract narrows the Era O 2D migration on-ramp after adapter-to-IR work.
It defines how neutral migration IR is mapped into Ouroforge-native candidate
artifacts, how the fidelity classifier grades each unit, and where logic is
handed to Era R for clean-room semantic re-derivation. It is documentation only;
no new write path, data store, engine bridge, or runtime is authorized here.

## Decision

Ouroforge may map validated neutral migration IR into **candidate**
Ouroforge-native skeleton artifacts only through the existing gated CLI/review
pipeline. The mapper consumes Rust-owned IR from source-project/open-text
adapters and produces loss-accounted mapping plans, draft scene/asset/input
shapes, fidelity classifications, and Era R hand-off records.

This is a one-way on-ramp. It is not engine absorption, not a finished-game
auto-port, not source-code translation, and not proof of equivalence. A mapped
unit is still a candidate skeleton until the appropriate Ouroforge-native oracle
passes.

## Accepted inputs

The mapper accepts only Rust-validated neutral IR whose adapter contract records
the source-only and clean-room boundary:

- `MigrationIr` / adapter-specific equivalents with source provenance;
- `IrScene`, `IrNode`, `IrAssetRef`, `IrInputAction`, `IrLogicTouchpoint`, and
  `IrUnsupportedFeature` groups;
- source-project/open-text provenance from Godot `.tscn` / `.tres` /
  `project.godot` or Unity Force-Text YAML + `.meta` in later milestones;
- adapter fidelity records and unsupported-feature diagnostics;
- optional captured acceptance evidence references supplied by later oracle
  flows.

Inputs are rejected or classified 🔴 when they require shipped-build ripping,
decompiled source, binary-only resources, live Unity/Unreal/Godot runtime access,
network/package-manager resolution, or trusted writes from Studio/Phoenix.

## Outputs

The mapper/classifier emits Rust-owned evidence and candidate artifacts:

1. **Mapping plan** — source IR ids to Ouroforge candidate scene/entity/asset/input
   targets, including target paths that must be applied only through existing
   `ouroforge` CLI gates.
2. **Candidate skeleton draft** — declarative scene hierarchy, 2D transforms,
   presentation metadata, asset references, input declarations, collider metadata,
   and other deterministic skeleton facts that can be expressed natively.
3. **Fidelity classification report** — one row per mapped, partial, unsupported,
   or re-derivation-required unit with 🟢/🟡/🔴 grade, reason, provenance,
   evidence refs, and oracle status.
4. **Era R hand-off bundle** — every script/signal/callback/input reaction,
   runtime mutation, AI, physics behavior, or gameplay rule that must be
   re-implemented clean-room from observed behavior plus interrogated intent.
5. **Deterministic state hash** — canonical hash for the mapped candidate state;
   2D equivalence later requires bit-exact state-hash oracle evidence.

These outputs do not mutate trusted project state by themselves. Human-facing UI
may display them, but every write must route through the existing gated CLI path.

## Mapping subset and grade floors

| IR source fact | Ouroforge candidate output | Grade floor | Notes |
| --- | --- | --- | --- |
| Scene/node hierarchy, names, parent order, visibility | Scene/entity tree draft | 🟢 when provenance and deterministic normalization are complete | Runtime-created nodes are not inferred. |
| 2D transforms, z/layer order | Entity transform/render order fields | 🟢 | Engine side effects stay out of scope. |
| Sprite, label, camera, tilemap presentation facts | Native presentation components or metadata | 🟢/🟡 | Metadata-only preservation is 🟡 when native behavior differs or is incomplete. |
| Project-local asset references | Asset manifest/import draft | 🟢/🟡 | Missing digest, ambiguous importer, or non-local path lowers grade. |
| Declarative input action names/bindings | Input-action draft | 🟢/🟡 | Gameplay reaction to input is logic and goes to Era R. |
| Collider/physics declarations | Collider metadata + re-simulated physics intent | 🟡 | Physics is re-simulated, never reproduced from the source engine. |
| Unsupported engine-specific rendering, shaders, particles, plugins | Unsupported feature record | 🔴 | Preserve gap; do not flatten to a fake native equivalent. |
| Script refs, signals, callbacks, animation method tracks, runtime behavior | Era R hand-off task | 🔴 | Inventory only; never copy, translate, or rephrase source code. |
| Decompiled/ripped/binary-only source | Rejection/red legal boundary record | 🔴 | No candidate mapping is produced for the unsafe unit. |

## Fidelity classifier rules

- **🟢 Green — mapped candidate skeleton.** A declarative IR fact maps to an
  Ouroforge-native skeleton field with complete provenance, deterministic
  normalization evidence, and no known behavior gap.
- **🟡 Yellow — partial or metadata-only mapping.** A fact is preserved with an
  explicit gap, native approximation, or pending evidence. The report states what
  is missing and why the unit is not equivalent.
- **🔴 Red — unsupported or re-derivation required.** The unit contains logic,
  source-engine behavior, legal ambiguity, unsupported presentation, shipped-build
  material, or copied/decompiled-source risk. It is not ported and must be routed
  to Era R or human decision.

The classifier must be conservative. A lossy import cannot be graded clean; an
unsupported unit cannot disappear; an auto-translated logic claim must fail.

## Oracle rule and determinism

No unit may be described as `ported`, `equivalent`, `complete`, or `verified`
unless captured Ouroforge-native acceptance evidence passes. For 2D, the primary
oracle is a bit-exact deterministic state hash over the relevant Ouroforge state.
For 2.5D/3D-adjacent presentation, deterministic state-hash remains primary and
perceptual render evidence such as SSIM/pixel-diff is secondary only.

Mapping determinism is mandatory:

- identical IR inputs produce identical mapping plans, reports, candidate drafts,
  and state hashes;
- IR or source-provenance drift changes the canonical hash;
- the report records stale, missing, or inconclusive evidence explicitly instead
  of silently passing.

## Gated path

1. Adapter parses source-project/open-text artifacts into neutral IR and fidelity
   records.
2. Mapper/classifier produces candidate skeleton drafts, mapping report, and Era R
   hand-off bundle under Rust data-plane ownership.
3. Existing `ouroforge` CLI gates validate target classes, stale hashes,
   source-apply/scene-apply previews, rollback evidence, and review decisions.
4. Only a passing gated apply path can write trusted Ouroforge artifacts.
5. Later oracle runs determine whether a mapped unit may receive equivalence
   wording; until then it remains a skeleton import or re-derivation task.

Phoenix/LiveView Studio may render evidence and capture user intent, but it owns
no artifact semantics, performs no trusted writes, and introduces no new data
plane.

## Era R hand-off requirements

Every logic-touchpoint hand-off must include:

- source IR id and source-project provenance;
- the trigger surface: script reference, signal, callback, input reaction,
  physics event, runtime animation event, or exported variable;
- visible declarative parameters and any observed behavior evidence refs;
- missing oracle(s) and expected deterministic state-hash evidence;
- fidelity grade 🔴 and gap explanation;
- clean-room instruction: re-implement in Ouroforge from observed behavior plus
  interrogated intent, never from copied or decompiled source.

## Non-goals

- No auto-porting a finished game.
- No live bridge or embedded Unity/Unreal/Godot runtime.
- No copied, translated, or decompiled source code.
- No faithful reproduction of source-engine physics, shaders, particles, or VFX.
- No Studio trusted write path, new database, or new artifact semantics owner.
- No automated fun/feel judgment or release go/no-go.

## Verification anchors

Downstream implementation, demo, and coverage issues for Milestone 90 must cite
this document and preserve the words: one-way on-ramp, source-project/open-text,
clean-room re-derivation, fidelity classifier, two-plane boundary, oracle-gated,
state-hash, Era R hand-off, and no auto-port. #1 and #23 remain open governance
anchors.
