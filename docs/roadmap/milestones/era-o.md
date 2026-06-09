### Era O: External-Engine 2D Migration On-Ramp (Godot- and Unity-first)

Added 2026-06-09. Eras A–N build a deterministic, evidence-native, agent-first 2D/2.5D production+verification engine. Era O opens a **migration on-ramp**: import the *declarative skeleton* (assets + scene + presentation) of an external 2D engine project so a team can start iterating inside Ouroforge instead of from scratch. **The honest framing (a permanent boundary): this is an on-ramp + value moat, not engine absorption and not finished-game auto-porting.** Content (the "nouns" — meshes/sprites/tilemaps/audio) imports at high fidelity; behavior (the "verbs" — logic/physics/shaders) is *re-derived*, not translated (see Era R). The loop's evidence honestly reports what imported clean vs what must be re-derived.

**Guiding principle for Era O.** One-way import to Ouroforge-native artifacts, never a live bridge to a non-deterministic engine and never embedding their runtime. Reuse the existing pipeline (scene/tilemap model, asset QA/provenance, evidence loop). A fidelity report turns the importer's unavoidable gaps into an auditable re-authoring plan. Sequencing note: Era O (skeleton) pairs with **Era R** (semantic re-derivation) to yield a *working* port; build order is **O → R → P → (Q gate)**.

**Boundaries (all prior boundaries unchanged; Era O additions in bold):**
- **One-way import only; no live bridge, no embedding of Unity/Unreal/Godot runtimes; imported artifacts become Ouroforge-native and deterministic.**
- **Legal: source-project + open/text formats only (Godot `.tscn`/`.tres`; Unity Force-Text YAML + `.meta`); no shipped-build ripping of others' games, no copying of decompiled code.**
- **Content imports best-effort with a fidelity report; logic/physics/shaders are re-derived (Era R), not auto-translated; no "auto-port a finished game" claim.**
- Bounded 2D subset (range-limited to prevent scope explosion); Rust parses/maps, Elixir/Phoenix surfaces; deterministic by construction.

#### Milestone 88: 2D Migration On-Ramp Scope, IR, and Legal/Fidelity Contract (Design-Gate-First)
Goal: define the importable 2D subset, the neutral intermediate representation (IR), the fidelity-grade model, and the source-only legal boundary.
Target deliverables: a design-gate ADR (importable subset, IR schema, 🟢/🟡/🔴 fidelity grades, one-way/no-embed/source-only legal contract, engine priority Godot→Unity); the on-ramp↔Era-R handoff contract.
Success criteria: the subset, IR, fidelity model, and legal boundary are specified; Godot-first priority recorded; #1 and #23 remain open.

#### Milestone 89: Godot 2D Adapter → IR
Goal: parse Godot 2D projects into the neutral IR.
Target deliverables: a read-only parser for `.tscn`/`.tres`/`project.godot` (scene tree, Sprite2D, TileMap/TileSet, Camera2D, Area2D/CollisionShape2D, Label, input map) → IR; a demo and a Scenario Coverage v78 regression suite.
Success criteria: a real Godot 2D project parses to a faithful IR; unsupported nodes are flagged, not dropped silently; deterministic.

#### Milestone 90: IR → Ouroforge Mapping and Fidelity Classifier
Goal: map the IR to Ouroforge scene/tilemap/sprite/behavior artifacts and grade each element's fidelity.
Target deliverables: IR→native mapping with coordinate/unit/color-space normalization; a 🟢/🟡/🔴 fidelity classifier per element; a demo and a Scenario Coverage v79 regression suite.
Success criteria: mapped artifacts render and validate in Ouroforge; every element carries a fidelity grade; 🔴 items become re-derivation tasks (Era R).

#### Milestone 91: Logic Touchpoint Detection and Re-Derivation Hand-off
Goal: identify the project's logic/engine-API touchpoints and hand them to Era R as re-derivation tasks (do not auto-translate here).
Target deliverables: detection of GDScript/engine-API behavioral units and their coupling; structured re-derivation tasks emitted to Era R; a Scenario Coverage v80 regression suite.
Success criteria: behavioral units are catalogued with their engine coupling; tasks route to Era R; no code is auto-translated in Era O.

#### Milestone 92: Import Verification and Fidelity Report
Goal: verify the imported skeleton through the loop and produce an honest fidelity report.
Target deliverables: openchrome verification of the imported skeleton; a fidelity report (clean vs flagged vs re-derive) with evidence/provenance links; a demo.
Success criteria: the imported skeleton runs and is verified; the report accurately attributes gaps; provenance (origin=godot, asset licenses) recorded.

#### Milestone 93: Unity 2D Adapter → IR
Goal: parse Unity 2D source projects into the same IR.
Target deliverables: a read-only parser for Force-Text `.unity`/`.prefab`/`.asset` + `.meta` (GUID+fileID), SpriteRenderer/Tilemap/Collider2D/Camera, with prefab-override flattening; runtime not bundled; a demo and a Scenario Coverage v81 regression suite.
Success criteria: a real Unity 2D source project parses to the IR with references resolved via `.meta`; shipped-build ripping is explicitly out of scope.

#### Milestone 94: Migration UX (Studio)
Goal: an import wizard + fidelity report + fix-forward surface in Studio, governed first by the M94 contract in `docs/migration-ux-studio-contract-v1.md`.
Target deliverables: a Phoenix LiveView import wizard, the Rust-owned fidelity report view, and fix-forward links into Era R/Era M; a demo and a Scenario Coverage v82 regression suite. Studio remains control/presentation only: one-way/source-project import, no live engine bridge, no embedded runtime, no new data plane, and no trusted Elixir write path.
Success criteria: a user can import an allowed Godot/Unity 2D source project, read honest 🟢/🟡/🔴 fidelity rows, and route 🔴 logic gaps to clean-room re-derivation; no unit is claimed ported without captured passing oracle evidence and deterministic state-hash validation.

#### Milestone 95: Era O Roadmap and #1 Governance Refresh
Goal: record Era O completion and the on-ramp boundaries.
Target deliverables: a roadmap/#1 update (Godot+Unity 2D on-ramp complete on merged evidence); reaffirmation of one-way/no-embed/source-only/no-auto-port boundaries; a #1 completion comment.
Success criteria: the roadmap reflects actual completion with evidence; boundaries reaffirmed; #1 and #23 remain open.
