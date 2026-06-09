# Contract: glTF Geometry and Orthographic-Camera Import v1

Status: accepted for Era P / Milestone 97 (#2192)  
Date: 2026-06-10 KST  
Parent boundary: `docs/2-5d-migration-on-ramp-scope-contract-v1.md`  
Scope: design contract only; no implementation in this milestone.

## Goal

Milestone 97 imports the 2.5D **presentation skeleton** for geometry and orthographic/isometric cameras through glTF-compatible, source-project inputs while preserving Ouroforge's deterministic 2D logic moat.

This contract fixes the importable subset, outputs, fidelity grades, oracle rule, and Era R hand-off before implementation. It is a one-way on-ramp to Ouroforge-native artifacts, not engine absorption, not a live bridge, and not a finished-game auto-port.

## Accepted inputs

Inputs must come from an authorized source project and open/text or inspectable source artifacts:

- glTF 2.0 `.gltf` JSON and `.glb` only when the asset is source-project material the user is authorized to import.
- Referenced buffers, images, textures, and material metadata that resolve within the source-project/import root.
- Node transforms, scene hierarchy, mesh primitive geometry, standard material references, and orthographic camera definitions.
- Source-project provenance records linking the imported glTF to the allowed Godot/Unity/source workspace when applicable.

Rejected or red inputs:

- shipped-build ripping, packed third-party game binaries, or unlicensed asset dumps,
- decompiled source or translated engine code,
- paths escaping the declared source/import root,
- runtime bridge descriptors or engine-embedded execution hooks,
- gameplay scripts or source-engine callbacks masquerading as geometry metadata.

## Importable subset

| Area | 🟢 Importable when valid | 🟡 Partial / re-author | 🔴 Re-derive / defer |
| --- | --- | --- | --- |
| Geometry | Mesh positions, normals, tangents, UVs, vertex colors, indices, primitive topology supported by the runtime | Unsupported attributes preserved as metadata when safe | Procedural engine geometry or binary-only generated mesh sources |
| Transforms | Translation/rotation/scale and static node hierarchy normalized into Ouroforge scene/presentation IR | Ambiguous coordinate/unit conventions require an explicit normalization note | Runtime-authored transforms from scripts/physics |
| Materials | glTF-standard PBR/unlit references, textures, alpha mode, sampler/filter hints | Non-standard extensions recorded as partial fidelity gaps | Source-engine shaders/VFX translated as behavior or claiming exact reproduction |
| Cameras | Orthographic cameras, isometric camera orientation, near/far/aspect intent | Perspective camera used for 2.5D look may be represented as presentation-only with warning | Full-3D gameplay camera/physics truth or engine runtime camera scripts |
| Animation | Static pose imports only for M97 unless explicitly presentation metadata | Keyframes that only affect presentation may be preserved as unsupported metadata | Animation-driven gameplay logic, hit timing, state transitions |
| Logic | None | Logic-bearing references become red hand-off rows | Any claim that gameplay logic was imported/ported |

## Outputs

The Rust data plane owns the outputs:

- an Ouroforge-native presentation skeleton artifact for geometry, node hierarchy, camera intent, and material references,
- a provenance record tying every imported artifact to source-project/open-format origins,
- a fidelity report with per-unit 🟢/🟡/🔴 grades,
- deterministic normalization metadata (coordinate system, unit scale, color-space assumptions, camera projection parameters),
- red/yellow Era R hand-off items for logic-bearing, shader/VFX, physics, or tacit-behavior gaps,
- evidence suitable for downstream state-hash primary and perceptual-render secondary checks.

No output may be a live reference to a source engine runtime. Imported content becomes Ouroforge-native or is reported as a gap.

## Gated path

Implementation milestones must route import writes through existing gated paths:

1. Parse and validate source-project provenance, open/text boundary, path containment, and glTF shape in Rust.
2. Normalize geometry, transforms, camera projection, material references, and render metadata into IR.
3. Produce a fidelity report before any user-facing claim of success.
4. Route artifact writes through the existing CLI/gate path (`scene-apply` / `source-apply` / evaluator gates as applicable).
5. Present results in Studio only as Rust-owned evidence; Elixir/Phoenix performs no trusted writes and owns no artifact semantics.

## Fidelity grades and oracle rule

- 🟢 **Green / imported presentation:** source is allowed, normalized output is deterministic, no behavior is claimed, and the artifact is presentation-only.
- 🟡 **Yellow / partial or re-author needed:** the import preserves useful presentation data but has unsupported material/camera/extension/animation gaps that must be named in the fidelity report.
- 🔴 **Red / re-derive or defer:** the unit is logic-bearing, physics-bearing, shader/VFX behavior, unsafe provenance, shipped-build-derived, decompiled, or otherwise outside M97. It becomes an Era R task or a DEFER item.

Nothing is called "ported" without captured acceptance evidence and a passing oracle. M97's geometry/camera import can be green for presentation skeletons only; gameplay logic remains re-derivation work.

## Determinism and verification

M97 preserves the Era P two-tier evidence rule:

- authoritative logic verification stays deterministic state-hash primary,
- 2.5D render verification uses perceptual render comparison only as secondary corroboration,
- camera and geometry normalization must be deterministic for identical inputs,
- imported physics is never reproduced; physics-like effects become deterministic re-simulation/re-derivation tasks.

## Era R hand-off

Every red or logic-bearing yellow row must carry enough context for semantic re-derivation:

- source-project provenance and glTF node/material/camera identifier,
- observed behavior or expected presentation/logic coupling if available,
- why it cannot be imported as presentation-only,
- required oracle or missing-oracle note,
- deterministic state expectations once re-expressed.

The hand-off wording must say re-derive/re-author, not translate or port.

## Non-goals

- No runtime bridge or embedded Godot/Unity/Unreal execution.
- No new data plane or persistent store.
- No decompiled-code copying or shipped-build ripping.
- No full-3D gameplay import; Era Q remains DEFER by default.
- No shader/physics/VFX 1:1 reproduction promise.
- No Studio trusted write path.

## Downstream citation requirement

Milestone 97 implementation, demo, and scenario coverage issues must cite this contract and the parent 2.5D ADR. If implementation discovers additional glTF cases, they must be added as bounded fidelity grades or explicit DEFER rows without weakening the one-way/source-only/clean-room boundary.
