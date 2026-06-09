### Era I: Genre Verticalization to a Shippable Title (Engine-Builder Deckbuilder)

Added 2026-06-08. Eras A–H prove the evidence loop can produce and verify small game classes (grid puzzle, deck roguelike) up to a web release candidate, with art/audio/systems/QA, autonomous production, provenance, and compliance. Era I takes the loop the last mile for one concrete, commercially-validated genre: a single-player, premium **engine-builder deckbuilder roguelite** (Balatro/Slay-the-Spire lineage) shipped on Steam. The strategy is **minimal first, then specialize, generalized to a degree**: extract a general card-roguelite substrate so the engine-builder deckbuilder is a thin configuration over it, then deepen it, so future deckbuilder variants are configs rather than rebuilds.

**Guiding principle for Era I.** Generalize the genre, don't hard-code the game: a single **card-roguelite substrate** (cards, modifiers, deterministic resolution, escalating run/ante, shop, seed, meta-progression, balance) carries every deckbuilder variant; the token-themed engine-builder is the first config over it. The durable advantage remains the evidence-backed loop — the substrate must remain fully deterministic, synthetic-balance-verifiable, and provenance-tracked. North-star, extended: `loop coverage × game complexity × trust × accessibility × production coverage × autonomy × **shippability**` — whether a loop-produced title can actually reach a real storefront, verified end to end.

**Boundaries (all prior boundaries unchanged; Era I additions in bold):**
- Rust-first and local-first; deterministic and seeded (daily-seed leaderboards are a feature, not a liability); synthetic-balance-verifiable by construction.
- **The substrate is the unit of reuse: a new deckbuilder variant must be a configuration over the substrate, not a parallel engine.**
- **"Fun/feel" is never an automated claim and is reserved for Era J (human-judged); Era I verifies the *mechanical and balance* surface only.**
- **Steam shipping is a *desktop export* (Electron/Steamworks), distinct from the deferred Layer-3 mobile/cloud scope; the Steam account, code signing, store sign-off, the "Release" button, and all market demand (wishlists/UA) remain human/external (Ring 3) and out of engine scope.**
- Public wording conservative; no production-ready/Godot-replacement/"makes fun games" claim.

#### Milestone 47: Card-Roguelite Substrate (Generalization of the Deck-Roguelike Class)
Goal: extract a general, deterministic card-roguelite substrate from the Milestone 31 deck-roguelike class so deckbuilder variants are configurations, not rebuilds (design-gate-first).
Target deliverables: a design-gate ADR defining the substrate boundary (what the substrate owns vs what a game config supplies) and a backward-compatibility contract; the substrate core model (cards, modifiers, deterministic resolution, run/ante, shop, seed, meta) reusing the existing deck-roguelike and seeded-RNG work; the existing deck-roguelike re-expressed as a config over the substrate with no behavior change; a thin engine-builder-deckbuilder config; a loop-produced demo and a Scenario Coverage v42 regression suite.
Success criteria: the deck-roguelike class is preserved as a substrate config (golden bytes unchanged); a second config (engine-builder) runs over the same substrate; everything stays deterministic and synthetic-balance-verifiable.

#### Milestone 48: Multiplicative Scoring-Engine and Modifier Composition
Goal: deliver the genre's defining mechanic — simple, readable modifiers that combine multiplicatively into an emergent, hard-to-solve scoring engine.
Target deliverables: a modifier/effect model (one-line readable effects); a deterministic multiplicative resolution engine with explicit ordering; a combinatorial composition model that keeps parts readable but wholes unsolved; a demo and Scenario Coverage v43.
Success criteria: modifiers compose deterministically; degenerate/broken combos are reproducible and surfaced; the resolution is seed-stable and replayable.

#### Milestone 49: Escalating Run Structure and Shop Economy
Goal: bound and escalate a run, and add the draft/shop economy that creates per-run variance.
Target deliverables: an escalating quota/ante run model; a shop economy (buy/sell/reroll/remove) with per-run draft variance; a demo and Scenario Coverage v44.
Success criteria: a bounded run escalates to a loss/win condition; the shop gives levers over probability; runs are seed-reproducible.

#### Milestone 50: Engine-Builder Balance Verification
Goal: extend synthetic balance to the engine-builder shape so "is the economy solved/unfair?" is answered by the loop, not by vibes.
Target deliverables: combo-explosion/degenerate-build detection; dominant-build (pick-rate/win-rate) analysis extending Milestone 32; fairness (loss-attribution / winnable-with-skill) and daily-seed solvability verification; a demo and Scenario Coverage v45.
Success criteria: a planted dominant build and a degenerate combo are detected; an unfair (unwinnable) seed is flagged; the verdict is descriptive, not a fun guarantee.

#### Milestone 51: Game-Feel and Juice Toolkit
Goal: give the runtime the feedback layer that makes the score cascade satisfying, with a responsiveness guarantee.
Target deliverables: juice primitives (easing/tween, shake, hit-stop, SFX hooks) for the runtime; a score-cascade payoff feedback system; a sub-100ms responsiveness verification check; a demo and Scenario Coverage v46.
Success criteria: the cascade payout produces deterministic, replayable feedback events; the responsiveness check fails when feedback latency exceeds budget; juice is verified mechanically (feel itself remains a human judgment in Era J).

#### Milestone 52: Deckbuilder UI Kit
Goal: the genre-specific UI surface, extending the existing read-only UI.
Target deliverables: card/hand/pipeline UI; shop and run-map UI; number-cascade/score display and tooltips; a demo and Scenario Coverage v47.
Success criteria: the UI renders substrate state and is probe-observable; it stays read-only/draft-only with trusted writes through the existing path; deterministic.

#### Milestone 53: Localization Pipeline
Goal: multi-language support for a global Steam release.
Target deliverables: string externalization; multi-language generation and validation; a demo and Scenario Coverage v48.
Success criteria: a title can be localized to multiple languages with validated, complete strings; localization is additive and verifiable.

#### Milestone 54: Steam Desktop Export and Steamworks (Design-Gate-First)
Goal: turn a web release candidate into a shippable Steam desktop build — the actual ship gate.
Target deliverables: a design-gate ADR (export architecture: web→desktop wrapper via Electron + steamworks.js; what is in scope vs human, e.g. account/signing/release-button); a build pipeline producing a Windows executable and depot upload via SteamPipe; Steamworks integration (overlay, achievements, cloud, daily-seed leaderboard); store-asset generation reusing the Milestone 36 asset pipeline; a built-artifact demo with a smoke test and Scenario Coverage v49.
Success criteria: a deterministic web build packages to a runnable desktop artifact with Steamworks features wired; store assets generate; the human-only steps (Steam account, code signing, content survey, the "Release" button) are explicitly documented as out of engine scope.

#### Milestone 55: Post-Launch Patch, Re-Verify, and Save-Migration Loop
Goal: support iterative updates after launch without breaking players' saves.
Target deliverables: an update → re-verify → re-package loop; a save-migration/version-compatibility model; a demo and Scenario Coverage v50.
Success criteria: a patched build re-verifies through the full gate set and re-packages; an older save migrates forward with verified compatibility; the loop is reproducible and provenance-tracked.

#### Milestone 56: Era I Roadmap and #1 Governance Refresh
Goal: record Era I completion and assess shippability.
Target deliverables: update the roadmap to record Milestones 47–55 as completed only after merged evidence; assess against the extended north-star (a loop-produced deckbuilder reaching a verified Steam-shippable artifact); a #1 completion comment; reaffirm Era I boundaries.
Success criteria: the roadmap and #1 reflect actual Era I completion with evidence; boundaries reaffirmed; #1 and #23 remain open.
