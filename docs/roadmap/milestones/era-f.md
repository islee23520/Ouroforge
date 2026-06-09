### Era F: Accessible Authoring and Genre Verticalization

Added 2026-06-07. Eras A–E brought the evidence loop to where it can produce, evaluate (four gates), and evolve small game classes with measured loop coverage (M20), proven generalization across two classes (M21), a trust gradient with bounded auto-apply (M22), multi-iteration evolve campaigns (M23), a demand-driven complexity ladder (M24), and an end-to-end provenance/audit bundle (M25). That proves the loop *works, generalizes, and is auditable*. It does **not** yet make the loop *accessible to non-developers*, and it has not *verticalized* the loop into a genre where automated generation, verification, and balancing decisively beat editor-only workflows. Era F closes that distance.

**Guiding principle for Era F.** Generation is the **front door** (access and adoption); the deterministic verification/balancing loop is the **engine room** that makes generated output non-slop. They are layers, not alternatives. The durable advantage remains the evidence-backed loop as the primary development primitive — generation only earns its place because every generated artifact must pass the engine room (four gates + solver/over-solution + balance) before promotion. The thesis-level reason this is tractable: high-iteration 2D genres (grid puzzle, deck roguelike, arcade) are **machine-checkable** (solvability is decidable, balance is measurable, simulation is cheap and deterministic), so generation in these genres can be verified rather than merely asserted — which is the precise gap that "describe-a-game" tools leave open.

**North-star, extended:** `loop coverage × game complexity × trust × accessibility` — what fraction of a game's trusted changes the loop produces and verifies, at increasing complexity, with reversible/audited autonomy, and now also for authors who **describe intent** (brief / natural language) rather than write code.

**Boundaries (Era A–E boundaries unchanged; Era F additions in bold):**
- Rust-first and local-first; Studio/browser surfaces remain read-only.
- **Generation never performs a trusted write.** It emits *proposals* that flow through the existing review / apply / trust-gradient path (M15 Safe Source Mutation Apply, M22 Trust Gradient). The browser read-only boundary is preserved.
- **"Non-slop" is a *process* guarantee** (a generated game must pass the engine room before promotion), **not** a quality, fun, or correctness guarantee. Public wording stays conservative: no "makes good/fun games", no Godot-replacement, no production claim.
- **Genre growth is demand-driven** (reuses the M24 ladder): each genre rung requires a loop-produced, evidence-backed demo with a loop-coverage verdict before the next rung is claimed; engine capability grows only to satisfy a rung.
- **Cloud / hosted / marketplace *monetization* intersects Layer-3 (hosted/cloud) scope, which is DEFER by default per M26 / #1508.** Absent an explicit Layer-3 GO, only the free local OSS core, the engine room, and a *local* registry are in scope; paid cloud is design-gated, not implemented.

#### Milestone 27: Genre Vertical Design Gate and Grid-Puzzle Game Class
Goal: choose the beachhead genre on evidence and add the first machine-checkable genre to the loop.
Target deliverables: a design-gate ADR (`docs/genre-vertical-design.md`) selecting grid puzzle (PuzzleScript-compatible) as the beachhead, with the genre's machine-checkable acceptance shape defined; a deterministic, probe-exposed grid-puzzle game class in the runtime (grid state, rule model, win condition); a PuzzleScript-compatible DSL ingest path (zero cold-start from an existing corpus); a loop-produced grid-puzzle demo with four-gate evidence and a loop-coverage verdict (honors the M24 ladder).
Success criteria: a grid-puzzle game runs deterministically and is fully observable via the probe API; an imported PuzzleScript-style game validates and runs; the demo carries a passing four-gate + loop-coverage verdict.

#### Milestone 28: Deterministic Solver and Over-Solution Detector (Design-Integrity Gate)
Goal: deliver the moat capability — prove a level has exactly the intended solution.
Target deliverables: a deterministic solver over grid state (solvability yes/no); designer intent capture (intended solution path / mechanic taught); an over-solution detector that exhaustively searches for alternative or shorter solutions bypassing intent; an auditable difficulty-metric artifact (solution length, branching factor, dead-end density, mechanic-introduction order); a new **design-integrity gate** added to the evaluator (intent satisfied AND no unintended over-solutions), composing with the existing four gates.
Success criteria: the solver decides solvability deterministically; the detector surfaces an over-solution as a replayable counterexample trace; the difficulty metrics are computed by Rust/local, not asserted; the design-integrity gate fails closed and links to evidence.

#### Milestone 29: Design Regression Harness (CI for Game Design)
Goal: make every content/rule edit re-prove the whole game, turning the loop into design CI.
Target deliverables: a regression harness that re-runs the full solver + over-solution + difficulty suite on each edit and diffs results, flagging newly opened exploits or regressions with evidence links; reuse of the existing evolve-campaign and `compare` infrastructure; a regression verdict surfaced read-only in the dashboard.
Success criteria: a rule tweak that opens a new over-solution elsewhere is detected as a regression with a replayable trace; a clean edit passes with no false regression; the harness reuses existing campaign/compare contracts (no duplicate engine).

#### Milestone 30: Generative Front Door v1 — Brief/NL to Verified Game Proposal (Design-Gate-First)
Goal: let an author describe intent and receive a *verified* game proposal, without weakening the safety model.
Target deliverables: a design-gate ADR (`docs/generative-front-door-design.md`) producing GO/DEFER and defining that generation emits **proposals only**, routed through the existing review / apply / trust-gradient path (never a direct trusted write); if GO, a brief/NL → grid-puzzle artifact *proposal* path validated by Rust and required to pass the engine room (four gates + solver + over-solution) before it can be promoted; an accessibility path so a non-developer can describe a puzzle and obtain a verified-solvable proposal.
Success criteria: a clear GO/DEFER decision exists; if GO, a generated puzzle proposal cannot be promoted unless it passes the engine room; the browser/Studio read-only boundary is preserved; every promoted proposal links to its generation provenance and verdicts.

#### Milestone 31: Seeded Stochastic Determinism and Deck-Roguelike Game Class
Goal: climb the next genre rung (deck roguelike), which requires deterministic randomness.
Target deliverables: a seeded RNG / deterministic stochastic simulation layer in the runtime (currently absent — the runtime is deterministic only because it has no randomness); a deck-roguelike game class (cards/relics/runs) that is probe-exposed and seed-reproducible; a loop-produced deck-roguelike demo (M24 rung gate) with four-gate + loop-coverage evidence.
Success criteria: identical seed → identical run (digest-stable); a deck-roguelike game runs and is observable; the demo carries a passing rung verdict; randomness never breaks replay/regression.

#### Milestone 32: Synthetic Player Telemetry and Balance Cockpit
Goal: deliver "pre-launch balance telemetry" — the second wow — as interpretable, human-in-the-loop evidence.
Target deliverables: synthetic player agents modeled as **human-like personas, not win-maximizers** (per the EA "Winning Isn't Everything" skill/style finding) running thousands of seeded runs; balance telemetry aggregation (pick-rate, win-deck inclusion, degenerate-combo flags, dead-item flags, difficulty curve); a read-only balance cockpit surfacing interpretable evidence plus a counterexample/replay; seeded re-run + diff on a balance change (reuses `compare`); optionally MAP-Elites/quality-diversity illumination of build archetypes.
Success criteria: a pre-launch balance report is produced from deterministic simulated runs; a degenerate combo is flagged with a replayable seed; a nerf can be re-run on the identical seed distribution and its win-rate impact diffed; output is descriptive (a cockpit), never an auto-applied nerf.

#### Milestone 33: Evidence-Native Asset/Template Marketplace v1 (Design-Gate-First, Layer-3-Adjacent)
Goal: make accumulated evidence compound into an ecosystem moat via verifiable assets.
Target deliverables: a design-gate ADR (`docs/evidence-marketplace-design.md`) defining the registry/marketplace boundary, permissions, and the OSS-core-vs-paid take-rate boundary; templates that ship *with* their acceptance suite + deterministic replay proof + provenance lineage (a verifiable asset); publish/consume from the free OSS core into a **local** registry; reuse of the M25 provenance bundle and existing evidence contracts.
Success criteria: a template can be published and consumed locally carrying replayable proof it works; provenance lineage is traceable; the registry works from the free core; any *hosted/paid* marketplace remains gated on a Layer-3 GO (#1508) and is not implemented absent it.

#### Milestone 34: OSS Trust Charter and Paid-Cloud Boundary Design Gate (Layer-3-Gated)
Goal: lock in the trust posture before monetization, and design (not build) the paid-cloud boundary.
Target deliverables: a written trust charter (`docs/oss-trust-charter.md`): MIT/Apache core, a no-relicense pledge, a no-runtime-fee / no-install-fee / no-revenue-share pledge, and a foundation/governance consideration; a paid-cloud boundary design-gate that, *only upon a Layer-3 hosted/cloud GO from #1508*, would monetize the operational/team/scale layer (hosted/searchable evidence history, multi-seat, CI runners, managed agent compute, marketplace take-rate) and **never** a creative primitive.
Success criteria: the charter is documented and reaffirms the trust third-rails; the paid-cloud boundary is specified with a GO/DEFER tied to #1508; absent a Layer-3 GO, only the free local OSS core remains and no cloud capability is introduced.

#### Milestone 35: Era F Roadmap and #1 Governance Refresh
Goal: record Era F completion and assess against the extended north-star.
Target deliverables: update `docs/roadmap.md` to record M27–M35 as completed only after their issues have merged evidence, with realized capabilities and remaining gaps; assess against `loop coverage × game complexity × trust × accessibility`, citing evidence (genres climbed, generation acceptance rate through the engine room, balance-cockpit coverage, trust posture); a #1 comment marking Era F complete with merged PRs and known gaps; reaffirm all Era F boundaries.
Success criteria: the roadmap and #1 reflect actual Era F completion with evidence and a north-star assessment; boundaries are reaffirmed; #1 and #23 remain open.

---
