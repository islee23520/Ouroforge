### Era H: Autonomous Production and Shipping

Added 2026-06-07. Eras A–G give the loop verified capability across nearly all production *functions* (mechanics, balance, level/content at scale, art, audio, UX, narrative, production QA) for two genres. What remains is making those functions run as one *autonomous production*, from concept to an actual release, with humans only on core judgment (vision, taste, IP/legal/ethics, release go/no-go, monetization) — and the shipping and live-operations surfaces that an actual release requires. Era H closes that distance. It is the project's highest-risk era and is the most heavily gated.

**Guiding principle for Era H.** The unit of autonomy moves from a single change to a *whole game*: a producer agent orchestrates the specialized function agents (Era G) from a human-given design intent through to a release candidate, with every trusted write still flowing through the review/apply/trust-gradient path. Autonomy is broadened only as verification strength and reversibility scale with it; the human-judgment boundary is never removed. North-star, extended: `loop coverage × game complexity × trust × accessibility × production coverage × autonomy` — the fraction of the concept→release pipeline the loop runs without human action, with humans retained on intent, taste, legal, and release approval.

**Boundaries (all prior boundaries unchanged; Era H additions in bold):**
- **Humans retain, permanently: creative vision and the "is it fun" judgment; art/audio/UX direction (taste); IP, legal, ethics, content-policy, and age-rating decisions; release go/no-go; and monetization.** The producer agent proposes and produces; humans give intent and approve gates.
- **The producer agent never performs a trusted write or a release directly;** trusted writes go through review/apply/trust-gradient, and a release requires an explicit human go/no-go gate.
- **Shipping (native/store export), hosted/cloud, and real-player liveops are Layer-3 and remain DEFER until an explicit GO** (the Milestone 45 re-evaluation, tied to #1508). Absent a GO, autonomy ends at a *web* release candidate with synthetic evidence; no store submission, no real-player data.
- Rust-first and local-first; conservative public wording; no production-ready/Godot-replacement/"ships great games autonomously" claim.

#### Milestone 42: Multi-Agent Production Pipeline (Design-Gate-First)
Goal: realize the role-specialized agent collaboration that Milestone 13 scoped, now that the functions exist.
Target deliverables: a design-gate ADR; role agents (designer, gameplay/systems, level/content, artist, audio, UX, QA, build, reviewer, critic) with explicit artifact ownership, handoff artifacts, conflict resolution, shared state, approvals, and observability; evaluator-gated promotion and regression blocking between roles.
Success criteria: multiple role agents work on one game project without hidden state or unreviewed trusted writes; every handoff and decision is recorded as evidence/journal; reviewer/critic agents can block promotion before trusted apply.

#### Milestone 43: Autonomous Producer and Whole-Game Orchestration
Goal: drive an entire small-but-complete game from a human design intent to a release candidate as one bounded, auditable campaign.
Target deliverables: a producer agent that decomposes a design intent into a production plan and orchestrates the Milestone 42 role agents through it; long-horizon plan/state for the whole game (extending the Milestone 23 campaign model); cost/iteration budgets and stop conditions at game scale; gate-driven progression with human approval points.
Success criteria: from a human design intent, the producer drives concept → content → assets → systems → QA → release-candidate with a complete audit trail, stopping safely at budget or at a human gate; non-convergence ends with an evidence-linked diagnosis, never an unbounded loop.

#### Milestone 44: Scaled Trust Gradient, Release Provenance, and Compliance Gate
Goal: scale safety and audit from per-change to per-release.
Target deliverables: a broadened-but-bounded auto-apply tier backed by stronger verification and game-scale rollback and kill switch (extending Milestone 22); a per-release provenance/audit bundle spanning the whole game's intent→content→assets→QA→release chain (extending Milestone 25); a compliance reviewer gate for content policy, age-rating signals, and asset license/provenance completeness.
Success criteria: a release candidate carries a complete, replayable per-release provenance bundle; broadened auto-apply remains reversible and audited with high-risk and source-affecting changes still manual; the compliance gate blocks release candidates with missing license/provenance or policy issues; humans retain the release go/no-go.

#### Milestone 45: Shipping and LiveOps — Layer-3 Re-evaluation and Build-Out
Goal: decide, on Era F–H evidence, whether to build the actual shipping and live-operations surfaces, then build only what the gate authorizes.
Target deliverables: a Layer-3 re-evaluation ADR (paired with #1508) producing a per-capability GO/DEFER for native/store export, real-player telemetry, live balancing, and update/patch pipelines; if GO, bounded build-out of the authorized capability with smoke evidence and release governance; if DEFER, autonomy remains web-release-candidate-only with synthetic evidence.
Success criteria: a documented per-capability GO/DEFER exists, justified by evidence; DEFER remains the default; any built shipping/liveops capability is governed by explicit readiness, rollback, and human go/no-go; Rust-first/local-first preserved absent a GO.

#### Milestone 46: Era H Roadmap and #1 Refresh, and Final Autonomy Assessment
Goal: record Era H completion and assess end-to-end autonomy against the human-judgment boundary.
Target deliverables: update `docs/roadmap.md` and comment on #1 recording Milestones 42–45 with merged evidence; a final assessment of `loop coverage × game complexity × trust × accessibility × production coverage × autonomy` for the two genres, reporting what fraction of concept→release runs autonomously and exactly which decisions remained human; reaffirm the permanent human-judgment boundary and all Layer-3 gating.
Success criteria: the roadmap and #1 reflect actual Era H completion with evidence and an autonomy assessment; the human-judgment boundary and Layer-3 gating are reaffirmed; #1 and #23 remain open.

---
