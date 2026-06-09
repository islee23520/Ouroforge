# Ouroforge: Final Goal and Implementation Direction

**Project name:** Ouroforge  
**Tagline:** *an evidence-native game engine built on Ouroboros loops.*  
**Visibility:** Private during early architecture and MVP validation.

## Final Goal

Ouroforge aims to become an evidence-native, agentic game engine where every game change is produced, tested, judged, and evolved through an Ouroboros loop:

> **Seed → Build → Observe → Verify → Journal → Evolve**

> **Detailed documentation**: See [`docs/roadmap/`](docs/roadmap/) for full vision, architecture, milestone history, and progress tracking.

## Roadmap Index and Status

> Navigation for this document. Eras run in order A→K; each Era section below states its **guiding principle**, **boundaries**, and **milestones** (in `#1` each milestone is Goal / Target deliverables / Success criteria; the derived GitHub issues carry the full 13-section template). Status as of 2026-06-08.
> Legend: ✅ complete · 🔄 in progress · 🟦 issues open · 📋 planned (not yet issued).

| Era | Theme | Milestones | Coverage | Issues | Status |
| --- | --- | --- | --- | --- | --- |
| A | Evidence-Native Foundation (subsumes legacy Milestone Plan M0–M7) | M0–M7, A.H | v0–v26 era | — | ✅ |
| B | Godot-Class Engine Core | Era B set | — | — | ✅ |
| C | Agentic Game Builder | Era C set | — | — | ✅ |
| D | Real Game Shipping | Era D set | — | — | ✅ |
| E | Loop Generalization & Trustworthy Autonomy | …M26 | …v26 | — | ✅ |
| — | Alignment Addendum (2026-06-05): Evaluator Depth / Evolve Depth / Foundation Hardening | M4.1, M5.1, A.H | — | — | 🔄 (A.H core-crate decomposition advanced by the `ouroforge-core-types`/`gdd`/`source-apply` split; M4.1/M5.1 partial) |
| F | Accessible Authoring & Genre Verticalization | M27–M35 | v27–v33 | #1573–#1619 | ✅ |
| G | Specialized Production Functions | M36–M41 | v34–v38 | #1634–#1673 | ✅ |
| H | Autonomous Production & Shipping | M42–M46 | v39–v41 | #1674–#1698 | 🔄 (M42 done; M43–M46 open) |
| I | Genre Verticalization to a Shippable Title (Engine-Builder Deckbuilder) | M47–M56 | v42–v50 | #1791–#1850 | 🟦 |
| J | Creative Co-Pilot & Release Decision | M57–M61 | v51–v54 | #1851–#1875 | 🟦 |
| K | Production Orchestration Executor (Studio Layer) | M62–M66 | v55–v58 | #1933–#1951 | ✅ complete on merged evidence (#1951 / PR #1985) |
| L | Autonomous Self-Validation & Improvement Loop (Real-Title Dogfooding) | M68–M73 | v60–v65 | #2023–#2048 | ✅ complete on merged evidence (#2047 / PR #2145; #2048 fix-proposal evidence) |
| M | Active Human Intervention (Agent-First, Human-Steerable) | M74–M81 | v66–v71 | #2052–2077 | ✅ complete on merged evidence (#2077 / PR #2245) |
| N | Human-Grade Studio & Adoption UX (Newcomer-Accessible) | M82–M87 | v72–v77 | #2078–2099 | ✅ complete on merged evidence (#2099 / PR #2292) |
| O | External-Engine 2D Migration On-Ramp (Godot/Unity) | M88–M95 | v78–v82 | #2167–#2190 | ✅ complete on merged evidence (#2167–#2190 / PR #2309); one-way source-only skeleton import + clean-room re-derivation hand-off, no auto-port/live bridge/runtime embedding |
| P | 2.5D Migration On-Ramp (3D presentation / 2D-deterministic logic) | M96–M100 | v83–v85 | #2191–#2203 | 🔄 M96–M98 contracts/early implementation merged (#2191–#2196 / PR #2323); M98 implementation/demo/coverage and M99–M100 remain open |
| Q | Full-3D On-Ramp Re-evaluation (DEFER by default) | M101–M106 | v86–v89 | #2204–#2219 | 🟦 gate #2204 open; M102–M106 GO-gated (DEFER default) |
| R | Interrogated Semantic Re-Derivation (legacy logic → verified deterministic behavior) | M107–M114 | v90–v95 | #2220–#2241 | ✅ complete on merged evidence (#2220–#2241 / PR #2318); clean-room semantic re-derivation only, no auto-port/live bridge/runtime embedding |
| K+1 | Executor Operator Cockpit and Read-Only Runbook UX | M67 | v59 | #2002–#2006, #2008–#2011 | 🟦 candidate: read-only operator UX; no new execution/trust authority |

## Active Eras

### Era P: 2.5D Migration On-Ramp

 2.5D Migration On-Ramp (3D Presentation over 2D-Deterministic Logic)

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

### Era Q: Full-3D On-Ramp Re-evaluation

 Full-3D Migration On-Ramp Re-evaluation (Design-Gate-First, DEFER by Default)

Added 2026-06-09. Full 3D import is possible for *assets/scene* (~30–55% via glTF) but forces near-second-engine additions (deterministic 3D physics, a glTF 3D web runtime, a two-tier evidence model) and **dilutes the bit-exact determinism moat** (3D GPU render is not cross-machine bit-reproducible → state-hash primary + perceptual render secondary). Era Q is therefore a **re-evaluation gate, DEFER by default**, that decides whether Ouroforge should become a partial-3D engine and, only on GO, builds a bounded 3D on-ramp.

**Boundaries (Era Q additions in bold):** all prior boundaries hold; **full 3D is DEFER by default; if GO, evidence becomes two-tier (deterministic state-hash primary, perceptual render secondary); imported 3D physics is re-simulated deterministically and is NOT a reproduction of the source; logic is re-derived (Era R); source-only legal boundary unchanged.**

#### Milestone 101: Full-3D Re-evaluation Gate (GO/DEFER)
Goal: decide, on evidence, whether to build a bounded full-3D on-ramp, given the moat-dilution tradeoff.
Target deliverables: a re-evaluation ADR (the two-tier evidence change, the deterministic-3D-physics requirement, the glTF-3D-runtime requirement, the "re-simulate, not reproduce" physics rule, demand evidence); a per-capability GO/DEFER with DEFER as default.
Success criteria: a documented GO/DEFER with evidence exists; DEFER remains the default absent strong demand; #1/#23 remain open.

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

## Architecture Summary

### Two-Plane Architecture (Kernel vs Studio Executor)

A persistent invariant across all Eras, made explicit here because later Eras add orchestration:

- **Rust verification kernel = data plane.** Owns artifact truth: seed/run/ledger/evidence/verdict/mutation schemas, the evaluator gates, the deterministic runtime, and all trusted-write validation. This never moves and is never self-certifying.
- **Studio executor = control plane (Era K).** May schedule, supervise, retry, budget, and observe the producer/role agents, but **never owns or defines any artifact's meaning** and **never performs a trusted write or release directly** — those flow only through review/apply/trust-gradient and the human go/no-go.
- **Posture shift (Era M, 2026-06-09):** Studio surfaces move from *read-only* to *read + gated-write* — humans may actively intervene at defined points, but every intervention is a validated, recorded proposal/constraint/directive through the existing gates, never a raw bypass. The interactive Studio is **local-first Phoenix LiveView** (Elixir control + presentation plane); hosted/multi-user is Layer-3 DEFER.

Eras A–J build the kernel, the genre-vertical game, the safety/provenance envelope, the agent-coordination *scaffolding* (Milestones 42–43, modeled as auditable artifacts and gates), and the human creative loop. The component that actually *drives* the multi-agent loop end-to-end — the orchestration **executor** — is currently external and ad-hoc; **Era K** makes it a first-class, supervised runtime. The executor's control-plane language is **Elixir/OTP — decided 2026-06-08** (control plane only; the kernel stays Rust). Milestone 62 decides only the build **GO/DEFER timing** by evidence (see ADR #92 and the Language Boundary Charter).

## Language Boundary Charter

Ouroforge should use each language where it is strongest, but the first private MVP must remain local-first and Rust-first.

### Rust owns initial core work

Rust is the default language for:

- harness kernel
- Seed / Run / Ledger / Evidence / Journal / Verdict models
- CLI
- evaluator
- mutation proposal storage
- evolve loop v0
- deterministic runtime core when native/runtime code begins
- file artifact integrity
- local orchestration and testable core logic

### TypeScript / JavaScript owns browser and UI surfaces

TypeScript or JavaScript is appropriate for:

- browser runtime probe scripts
- browser demo pages
- dashboard UI
- Godot-like authoring UI
- Chrome/CDP glue only if the implementing issue proves it is simpler and does not weaken Rust-owned artifact contracts

### Elixir owns the Studio executor control plane (Era K)

Decision (2026-06-08): Elixir/OTP is the chosen language for the **Studio executor control plane** — the production-orchestration runtime that schedules, supervises, budgets, retries, and observes the producer/role agents (Era K). This fixes the *direction*; the *build timing* stays evidence-gated at Milestone 62 (ADR #92 revisit). The Rust kernel is unchanged.

Elixir/OTP (control plane only) may own:

- producer/role-agent worker supervision (spawn, crash isolation, restart)
- task scheduling over the production plan and its dependency graph
- runtime enforcement of budgets and stop conditions
- retry/backoff, backpressure, and concurrency control
- live telemetry/progress fanout derived from kernel artifacts

Elixir must **never** own — these stay in the Rust kernel, reached only via the `ouroforge` CLI:

- rendering, physics, frame loop, deterministic simulation core
- seed/run/ledger/evidence/verdict/mutation schemas and validation
- the evaluator gates, trusted-write acceptance, and releases
- local file artifact contracts and the harness kernel

Still deferred (Layer-3, ADR #92 / Milestone 45): distributed/multi-machine orchestration, hosted/cloud, and live-ops. Era K is **local single-machine and local-first**; the manual Rust-CLI loop remains a tested fallback.

### Python is non-core unless explicitly justified

Python may be used only for one-off tooling, research scripts, or temporary migration helpers. It must not own core runtime contracts, harness semantics, or evaluator logic unless a later issue explicitly changes this charter.

### Language drift rule

If an issue does not explicitly authorize a language/runtime, default to Rust for core/harness work and TypeScript/JavaScript for browser/UI work. Any new language introduction requires an issue update explaining why the current authorized languages are insufficient.

---

---

> **Completed Eras (A–O, R)**: Full milestone details archived in [`docs/roadmap/milestones/`](docs/roadmap/milestones/)
> **Machine-readable progress**: [`docs/roadmap/progress.json`](docs/roadmap/progress.json)
> **Comment history**: [`docs/roadmap/CHANGELOG.md`](docs/roadmap/CHANGELOG.md)
