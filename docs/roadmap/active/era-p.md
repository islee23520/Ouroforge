### Era P: 2.5D Migration On-Ramp (3D Presentation over 2D-Deterministic Logic)

Added 2026-06-09. Era P extends the on-ramp to 2.5D — but keeps the moat by framing 2.5D as **3D *presentation* over 2D-*deterministic* logic** (the common case: isometric/orthographic look, billboards, sprite-stacking, with logic that is fundamentally 2D and deterministic). Geometry and orthographic cameras import via glTF; camera-facing/stacking behavior is presentation re-authored at runtime; logic is re-derived via Era R.

**Boundaries (Era P additions in bold):** all Era O boundaries hold; **2.5D logic stays 2D-deterministic (no non-deterministic 3D physics is introduced here); 3D presentation (billboards/stacking) is runtime presentation, not gameplay truth; full 3D is deferred to the Era Q gate.**

#### Milestone 96: 2.5D On-Ramp Scope & Contract (Design-Gate-First)
Goal: define 2.5D as "3D presentation + 2D-deterministic logic" and what imports vs re-authors.
Target deliverables: `docs/2-5d-migration-on-ramp-scope-contract-v1.md`, a contract (ortho/iso camera + glTF geometry + billboard/stack presentation; logic stays 2D-deterministic), the import-vs-re-author split, the Era R hand-off contract, and the fidelity-grade/oracle rule.
Success criteria: the 2.5D boundary and split are specified; the determinism moat is preserved; #1/#23 remain open.

#### Milestone 97: glTF Geometry and Orthographic-Camera Import
Goal: import 2.5D geometry and orthographic/isometric cameras via glTF into the presentation layer.
Target deliverables: `docs/gltf-geometry-orthographic-camera-import-contract-v1.md`, then a glTF importer (geometry, PBR-standard materials→metallic-roughness, orthographic camera) with coordinate/unit/color-space normalization; a demo and a Scenario Coverage v83 regression suite. The importer must cite the M97 contract and the parent 2.5D ADR.
Success criteria: 2.5D geometry + ortho camera import and render; custom shaders are flagged for baking/re-authoring; deterministic logic untouched.

#### Milestone 98: Billboard, Sprite-Stack, and 2D-in-3D Presentation Layer
Goal: provide the runtime presentation behaviors (camera-facing billboards, sprite stacking) over deterministic logic.
Target deliverables: `docs/billboard-sprite-stack-presentation-contract-v1.md`, then presentation primitives (billboard, sprite-stack, unlit/alpha-mode handling, pixel-art nearest filtering); a demo and a Scenario Coverage v84 regression suite. Implementation/demo/coverage issues must cite the M98 contract and parent 2.5D ADR.
Success criteria: 2.5D presentation renders correctly; presentation is decoupled from (and never mutates) the deterministic logic/evidence.

#### Milestone 99: 2.5D Import Verification and Fidelity Report
Goal: verify imported 2.5D and report fidelity (state-hash primary, perceptual render secondary).
Target deliverables: verification with deterministic logic/state-hash as the primary gate and perceptual (SSIM/pixel-diff) render corroboration; a fidelity report; a Scenario Coverage v85 regression suite.
Success criteria: 2.5D imports verify on state-hash; render checked with tolerance; gaps attributed.

#### Milestone 100: Era P Roadmap and #1 Governance Refresh
Goal: record Era P completion and reaffirm the 2.5D=presentation/2D-logic boundary.
Target deliverables: roadmap/#1 update; boundary reaffirmation; #1 completion comment.
Success criteria: completion reflected with evidence; boundary reaffirmed; #1/#23 remain open.
