### Era G: Specialized Production Functions

Added 2026-06-07. Eras A–F bring the evidence loop to where it produces and verifies *small complete games* in two high-iteration 2D genres (grid puzzle end-to-end including non-developer natural-language generation; deck roguelike including pre-launch synthetic balancing), with a verification engine room (solvability, over-solution, balance, regression), a generative front door, and a local verifiable-asset registry. That covers the *mechanics, balance, and verification* functions of production. It does **not** cover the other specialized studio functions a *large or long-form* game needs: art, audio, UX, narrative, content at campaign scale, and production-scale QA. Era G fills those functions — each as a specialized capability with its own design-gate and verification gate — so the loop can produce a larger, longer game, not only a vertical slice.

**Guiding principle for Era G.** Each missing studio function becomes a *specialized, verified* capability, never an unverified generator. Generation (assets, content, systems) stays proposal-only through the existing review/apply/trust-gradient path; every generated artifact must pass a function-specific verification gate (asset-QA, content-curation, systems contract, production-QA) before promotion — the same "front door + engine room" pattern as Era F, extended to each function. Growth stays demand-driven (Milestone 24 ladder): a function is added only when a loop-produced game class needs it, and each rung needs a loop-produced demo. North-star, extended: `loop coverage × game complexity × trust × accessibility × production coverage` — and now also *how many specialized production functions the loop covers with verification*, at increasing game scale.

**Boundaries (Era A–F boundaries unchanged; Era G additions in bold):**
- Rust-first and local-first; Studio/browser surfaces read-only; generation never performs a trusted write (proposals only).
- **Generated assets (art/audio) must carry license/provenance and pass asset-QA; no unlicensed, uncredited, or unverified-style asset slop is ever promoted.** "Looks good / sounds good / is fun" remains human taste, not an automated claim.
- **"Quality/fun" is never an automated guarantee.** Public wording stays conservative; no production-ready, Godot-replacement, or "makes good games" claim.
- Engine/content/system breadth grows only to satisfy a specific loop-produced rung (Milestone 24); no pre-authorized breadth.
- Shipping, hosted/cloud, and real-player liveops remain Layer-3 (DEFER per Milestone 26 / #1508) and are out of Era G scope; Era G ships nothing to stores.

#### Milestone 36: Asset Generation and Asset-QA Gate (Design-Gate-First)
Goal: give the loop a *verified* visual asset function (sprites, tilesets, UI art, animation, VFX) without introducing asset slop or license risk.
Target deliverables: a design-gate ADR (`docs/asset-pipeline-design.md`) defining the generated-asset policy (license/provenance, attribution, allowed sources) and GO/DEFER; if GO, asset generation that emits *proposals* with attached license/provenance; an asset-QA gate (style-consistency, resolution/format validity, visual-regression vs baseline, license/provenance completeness) composing with the existing four gates; an asset import/atlas path.
Success criteria: an asset can be generated as a proposal, must pass asset-QA (including license/provenance) before promotion, and is fully evidence-linked; unlicensed/inconsistent assets are blocked fail-closed; art-direction/taste remains a human decision.

#### Milestone 37: Audio Generation and Audio-QA
Goal: extend the asset function to audio (SFX, music, adaptive audio) under the same verified, license-safe pattern.
Target deliverables: audio generation emitting proposals with license/provenance; an audio-QA check (format/loudness validity, license/provenance, regression vs baseline); adaptive-audio hooks in the runtime; audio evidence in the bundle.
Success criteria: audio can be generated as a proposal, passes audio-QA before promotion, and is evidence-linked; unlicensed audio is blocked; sound-direction remains human.

#### Milestone 38: Content-at-Scale Generation and Curation
Goal: move generation from single levels to *campaign-scale* content, with curation so scale does not become slop.
Target deliverables: campaign-scale generation (hundreds of puzzle levels / large card-and-relic pools), extending the Milestone 30 front door to the deck-roguelike genre; deduplication and novelty/variety metrics (to avoid "challenging but uninteresting" churn); whole-game difficulty-curve authoring and verification (not only per-level); curation gate that admits only solvable/balanced, sufficiently-novel content.
Success criteria: a campaign of many levels/cards is generated, curated, and verified (solvability/balance/novelty/whole-game difficulty curve) before promotion; both genres support campaign-scale generation with comparable evidence; generation gaps surface as explicit findings, not silent manual patches.

#### Milestone 39: Long-Form Game Systems (Demand-Driven Ladder)
Goal: add the systems a long-form game needs, each as a gated ladder rung verified by the loop.
Target deliverables, each a Milestone 24 rung with a loop-produced demo: meta-progression and unlocks; economy/currency; save/profile and run history at scale; UI/UX flow (menus, HUD, settings), onboarding, and accessibility; an optional narrative/dialogue/event system for genres that need it.
Success criteria: each climbed system rung has a loop-produced demo with passing verification and a loop-coverage verdict; systems compose into a longer playable game; UX/narrative tone remains a human decision; no system breadth lands without a rung justification.

#### Milestone 40: Production-Scale QA Matrix
Goal: scale QA from per-artifact checks to whole-game production QA.
Target deliverables: a regression matrix across content variants, seeds, and supported targets (extending the QA/playtest swarm and Milestone 14); visual-regression at scale; performance/soak testing for long sessions and large content; crash/flaky detection at scale; accessibility and asset/UX QA; a consolidated production-QA verdict per game build.
Success criteria: a large game build is exercised by a production-scale QA matrix with visual/soak/performance/accessibility coverage; regressions across content/seeds/targets are caught with replayable evidence; the QA verdict is descriptive, not a quality/fun guarantee.

#### Milestone 41: Era G Roadmap and #1 Governance Refresh
Goal: record Era G completion and assess production-function coverage.
Target deliverables: update `docs/roadmap.md` to record Milestones 36–40 as completed only after merged evidence, with realized functions and remaining gaps; assess against the extended north-star (production coverage across functions and game scale, with the license/provenance and human-taste boundaries intact); a #1 comment marking Era G complete; reaffirm all Era G boundaries.
Success criteria: the roadmap and #1 reflect actual Era G completion with evidence; boundaries reaffirmed; #1 and #23 remain open.
