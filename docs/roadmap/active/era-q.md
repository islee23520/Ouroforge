### Era Q: Full-3D Migration On-Ramp Re-evaluation (Design-Gate-First, DEFER by Default)

Added 2026-06-09. Full 3D import is possible for *assets/scene* (~30–55% via glTF) but forces near-second-engine additions (deterministic 3D physics, a glTF 3D web runtime, a two-tier evidence model) and **dilutes the bit-exact determinism moat** (3D GPU render is not cross-machine bit-reproducible → state-hash primary + perceptual render secondary). Era Q is therefore a **re-evaluation gate, DEFER by default**, that decides whether Ouroforge should become a partial-3D engine and, only on GO, builds a bounded 3D on-ramp.

**Boundaries (Era Q additions in bold):** all prior boundaries hold; **full 3D is DEFER by default; if GO, evidence becomes two-tier (deterministic state-hash primary, perceptual render secondary); imported 3D physics is re-simulated deterministically and is NOT a reproduction of the source; logic is re-derived (Era R); source-only legal boundary unchanged.**

#### Milestone 101: Full-3D Re-evaluation Gate (GO/DEFER)
Goal: decide, on evidence, whether to build a bounded full-3D on-ramp, given the moat-dilution tradeoff.
Target deliverables: a re-evaluation ADR (the two-tier evidence change, the deterministic-3D-physics requirement, the glTF-3D-runtime requirement, the "re-simulate, not reproduce" physics rule, demand evidence); a per-capability GO/DEFER with DEFER as default.
Success criteria: a documented GO/DEFER with evidence exists; DEFER remains the default absent strong demand; #1/#23 remain open.

#### Milestone 101 Decision: DEFER recorded (#2204)

M101 is recorded as DEFER in `docs/full-3d-on-ramp-reevaluation-gate-v1.md`. M102-M106 remain GO-gated and no implementation work is created by this lane. The decision preserves the 2D bit-exact moat, treats 3D state-hash evidence as primary and perceptual render comparison as secondary, and keeps full-3D physics as native deterministic re-simulation rather than source-runtime reproduction. #1/#23 remain open.

#### Milestone 102: glTF 3D Scene Import and Normalization (GO-gated)
Goal: import 3D scene/geometry/PBR materials via glTF with web normalization.
Target deliverables: a glTF 3D importer (mesh/PBR-metallic-roughness/skeleton/morph/cameras/punctual lights), normalization (axis/handedness/units/color-space), Draco/Meshopt/KTX2 optimization; a Scenario Coverage v86 regression suite.
Success criteria: a 3D scene imports and renders in the web runtime; custom shaders/VFX/baked-GI flagged for re-authoring/re-bake.

#### Milestone 103: Deterministic 3D Physics Re-Simulation (GO-gated)
Goal: re-simulate imported 3D scenes under a determinism-built physics engine (not reproducing the source).
Target deliverables: integration of a deterministic engine (e.g., Rapier `enhanced-determinism`) with fixed timestep, pinned op order, seeded RNG, no FMA/fast-math/uncontrolled transcendentals; a Scenario Coverage v87 regression suite.
Success criteria: physics state is deterministic and cross-platform-hashable; imported dynamics are treated as reference intent, not a reproduction.

#### Milestone 104: Two-Tier 3D Evidence Model (GO-gated)
Goal: extend the evidence model for 3D — deterministic state-hash primary, perceptual render secondary.
Target deliverables: a state-hash (SupCom/OpenTTD-style) primary gate plus perceptual (SSIM/pixel-diff) render corroboration with pinned-GPU exact-hash option; a Scenario Coverage v88 regression suite.
Success criteria: 3D runs gate on cross-platform state-hash; render verified with tolerance; the bit-exact 2D model is preserved where applicable.

#### Milestone 105: 3D Logic Re-Derivation Hand-off and Demo (GO-gated)
Goal: route 3D logic to Era R and demonstrate a bounded 3D import end to end.
Target deliverables: 3D behavioral-unit hand-off to Era R; a bounded 3D import demo with fidelity report; a Scenario Coverage v89 regression suite.
Success criteria: a bounded 3D scene imports, re-simulates deterministically, and its logic is re-derived+verified via Era R; gaps honestly reported.

#### Milestone 106: Era Q Roadmap and #1 Governance Refresh
Goal: record the Era Q outcome (GO scope built or DEFER recorded) and reaffirm the moat boundary.
Target deliverables: roadmap/#1 update; reaffirmation that DEFER is default and that 3D never silently replaces the bit-exact 2D moat; a #1 completion comment.
Success criteria: outcome reflected with evidence; boundaries reaffirmed; #1/#23 remain open.
