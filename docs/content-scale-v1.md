# Content-at-Scale Generation and Curation v1

Content-at-Scale Generation and Curation v1 is a scope/contract milestone, not an
implementation milestone. It defines the decision, trust boundaries, promotion
rules, and follow-up sequence under #1 Era G Milestone 38 that let generation
move from single levels to **campaign scale** — many levels, large pools, and
whole-game shape — while curation guarantees that scale does not become slop.

Campaign-scale generation is the **front door at scale**. The deterministic
verification loop — the four-gate evaluator aggregation (`declared-gate-and`),
the solver, over-solution detection (Milestone 28), and synthetic-player balance
evidence (Milestone 32) — is the **engine room** that makes output non-slop. They
are layers, not alternatives: the front door lets an author request a campaign;
the engine room is what every level, and the campaign as a whole, must pass
before it can be promoted. Scaling generation never replaces verification and
never bypasses it.

This contract exists because scaling a generation entry point is tempting to wire
as a shortcut — a path that emits many artifacts at once, auto-applies its own
output, dedups by deleting evidence, or claims its campaigns are good. Ouroforge
keeps trusted persistence in Rust and keeps browser/Studio surfaces read-only. A
content-at-scale path must stay inside those boundaries: it emits proposals only,
routed through the existing review/apply/trust-gradient path, and no campaign — and
no level within it — is promoted unless it passes the engine room and the curation
gate.

## Gate outcome: GO (bounded)

The decision for Content-at-Scale Generation and Curation v1 is **GO**, bounded
strictly to the scope below. The default for any capability not enumerated here
remains **DEFER**.

GO is justified only because content-at-scale reuses surfaces that already exist
and are already governed:

- the Milestone 30 generative front door (`docs/generative-front-door-v1.md`,
  brief/NL intake and proposal model) already turns intent into proposals on the
  existing path; campaign scale extends it rather than replacing it;
- the review/apply/trust-gradient path (Milestone 15 / Milestone 22) already
  carries proposals through human review before any trusted write;
- the four-gate evaluator aggregation (`declared-gate-and`), solver, and
  over-solution detection (Milestone 28) already define the per-level engine-room
  promotion guard;
- synthetic-player balance evidence (Milestone 32,
  `docs/synthetic-player-balance-v1.md`) already produces the difficulty/balance
  signal that whole-game curve verification reads;
- `evolve_campaign.rs`, `provenance_bundle.rs`, and the asset manifest already
  provide campaign iteration, provenance, and asset/license tracking; and
- the deterministic runtime (`examples/game-runtime/runtime.js`) and the
  `window.__OUROFORGE__` probe already provide read-only browser inspection.

GO authorizes **only** the design contracts and the follow-up issue sequence in
this document. It does not authorize a new engine, a new runtime, a new writer, a
new generator, a new language, or any executable generation behavior. Each
follow-up issue is separately scoped and must reuse the named surfaces above
rather than build a parallel system.

### GO/DEFER criteria

A capability is **GO** under Content-at-Scale Generation and Curation v1 only if
all of the following hold; otherwise it is **DEFER** by default:

1. **Proposal-only.** It emits a proposal (or set of proposals) routed through
   the existing review/apply/trust-gradient path. It never performs a direct
   trusted write.
2. **Engine-room gated.** No content it produces can be promoted unless every
   level passes the engine room (four gates + solver + over-solution) and the
   campaign passes the curation gate.
3. **Surface reuse.** It extends an existing surface (front door, runtime/probe,
   evaluator, evolve/campaign, compare, provenance, asset manifest, review/apply,
   source-apply, dashboard/cockpit, QA swarm, CLI) rather than adding a parallel
   generator, engine, or runtime.
4. **Boundary preserving.** Rust/local owns the new trusted logic; browser/Studio
   stays read-only; the change is additive and backward-compatible.
5. **Conservative wording.** It makes no auto-merge, quality, fun,
   production-ready, shippable, or Godot-replacement claim.
6. **Governance preserving.** It does not close, narrow, or modify #1 or #23.

Anything that fails any criterion is DEFER and requires a separate explicit
governance decision before it may be scoped.

## Non-slop at scale is a process guarantee

"Verified," "curated," and "non-slop" in this milestone are **process
guarantees, not quality or fun claims**. A campaign is *curated* exactly when
every admitted level has passed the engine room (four gates + solver +
over-solution), the campaign's difficulty curve has been verified against its
declared shape, and the admitted content meets the novelty/variety threshold.
"Non-slop at scale" means "passed the engine room per level, passed curve
verification, and cleared the novelty gate."

It does **not** mean the generated campaign is good, fun, shippable, balanced in
a subjective sense, or production-ready. No public wording in this milestone may
imply otherwise. Difficulty/balance/novelty are measured against declared,
evidence-backed thresholds, not against a taste judgement.

## Campaign-scale generation contract

Campaign-scale generation extends the Milestone 30 front door from a single level
to a campaign:

- The unit of a campaign request is still a brief/NL intent (Milestone 30 intake,
  #1593). The output is a **campaign proposal**: an ordered or pooled set of level
  proposals plus campaign-level metadata (declared difficulty shape, intended
  size, target game class).
- Campaign scale targets the deck-roguelike game class
  (`docs/deck-roguelike-game-class-v1.md`) and many-level / large-pool scale, in
  addition to the grid-puzzle class already reachable through the front door. The
  game class is declared in the request, not inferred by a new engine.
- Generation reuses `evolve_campaign.rs` for campaign iteration and
  `provenance_bundle.rs` for per-level and per-campaign provenance. It does not
  add a parallel campaign generator or a new iteration loop.
- A campaign proposal is identical in trust to any other proposal entering the
  review/apply/trust-gradient path. Generation never applies, auto-applies,
  auto-merges, self-approves, or bypasses review for a campaign or any level in
  it.
- Provenance (machine-generated, from what intent, through what path, with what
  per-level engine-room evidence) is attached to the campaign proposal and to
  each level proposal so reviewers can see what produced them.

## Deduplication and novelty/variety contract

Dedup and novelty exist so that scale does not collapse into repetition:

- Dedup and novelty/variety metrics are computed **over existing solver and
  difficulty evidence** — the solver trace, difficulty/balance signals
  (Milestone 32), and per-level engine-room evidence. They do not introduce a new
  similarity engine, embedding model, or external service.
- Dedup is a **read/measure-and-gate** operation, not a destructive one. It
  identifies duplicate or near-duplicate levels by their existing evidence and
  marks them for exclusion from admission; it never deletes evidence, runs, or
  prior content to manufacture novelty.
- Novelty/variety is measured against a declared, evidence-backed threshold. A
  campaign whose admitted levels fall below the threshold fails the curation gate;
  the gate fails closed.
- Novelty is a process metric over evidence, not a taste claim. "Sufficiently
  novel" means "above the declared evidence threshold," not "interesting" or
  "fun."

## Whole-game difficulty-curve contract

Difficulty-curve verification raises balance verification from per-level to
whole-game:

- A campaign declares an intended **difficulty curve** (the shape of difficulty
  across its ordered levels) as part of the campaign proposal. The curve is
  declared and verified, not auto-tuned by a new engine.
- Curve verification reads the existing per-level difficulty/balance evidence
  (Milestone 32 synthetic-player balance, solver difficulty signal) and checks
  the realized curve against the declared shape within declared tolerances.
- Whole-game verification is additive to per-level verification: every level must
  still individually pass the engine room. Curve verification is a campaign-level
  precondition layered on top, not a replacement for per-level gates.
- Curve verification fails closed: a campaign whose realized curve does not match
  its declared shape within tolerance, or whose curve evidence is missing or
  stale, is not promotable.

## Curation gate (campaign promotion guard)

A campaign **cannot be promoted** unless it passes the curation gate, which is the
campaign-level analogue of the per-level engine-room guard:

- **Per-level engine room.** Every admitted level must pass the four-gate
  aggregation (`declared-gate-and`), be demonstrably solvable by the solver, and
  pass over-solution detection. A single failed level fails the campaign closed.
- **Solvable.** No unsolvable level is admitted.
- **Balanced.** The campaign's realized difficulty curve matches its declared
  shape within tolerance (whole-game curve verification).
- **Sufficiently novel.** The admitted content meets the declared
  novelty/variety threshold; duplicates and near-duplicates are excluded.

The curation gate fails closed: a campaign with a missing, failing, or stale
engine-room, curve, or novelty evidence is not promotable. Curation gating is
**mandatory before campaign promotion**. Promotion is the existing
review/apply/trust-gradient action; the curation gate adds a precondition, it does
not add a new write path. This gate is the boundary between content at scale and
slop at scale: generation can produce many candidates, but only the curation gate
plus human review can let a campaign through.

## Language boundary

- **Rust** owns trusted validation, persistence, the generation-proposal /
  asset-QA / curation / dedup / novelty / curve / orchestration / provenance /
  compliance logic, evidence writing, run/project binding, the
  review/apply/trust-gradient path, and CLI behavior.
- **TypeScript/JavaScript** owns the deterministic runtime (including in-game
  UI/HUD/menus), the `window.__OUROFORGE__` probe, browser-local read-only
  inspection, and static dashboard/cockpit behavior where explicitly scoped.
- **Python** may be used only for temporary local tooling or smoke helpers and
  must not own core Era G contracts.
- No new language or runtime is introduced without explicit issue-level
  authorization; distributed/Elixir remains NO-GO per ADR #92
  (`docs/distributed-elixir-design.md`).

## Studio boundary

Studio's content-at-scale surface, when designed in a later issue, must remain a
**read-only evidence surface**. It may display campaign proposals, per-level and
per-campaign provenance, engine-room status (gates/solver/over-solution),
dedup/novelty metrics, difficulty-curve verification status, curation-gate
status, review state, and copyable CLI commands. It must not generate campaigns
through a trusted path, apply or promote campaigns or levels, auto-approve,
execute commands, or become a command bridge.

## Asset, license, and provenance boundary

Campaign scale does not relax the asset and provenance rules:

- Any generated asset, audio, or content carried by a campaign requires
  license/provenance metadata (the asset manifest) and must pass the
  function-specific QA gate before promotion. Scale never waives this.
- No unlicensed, uncredited, or unverified-style generated asset/audio/content is
  ever promoted, regardless of how many levels reference it.
- Provenance is attached per level and per campaign; a campaign with missing
  asset provenance fails the curation gate closed.

## Dependency order and closure gates

The follow-up issues stay scoped to reuse of existing surfaces and must be
completed in this order:

1. **Scope and Contract** — this issue (#1648).
2. **Campaign-Scale Generation v1** — #1649. Extend the Milestone 30 front door
   to deck-roguelike and to many-level/large-pool scale via `evolve_campaign.rs`,
   emitting campaign proposals on the existing path. Depends on #1593, #1601,
   #1580, #1581, and #1607.
3. **Deduplication and Novelty Metrics v1** — #1650. Compute dedup and
   novelty/variety metrics over existing solver/difficulty evidence.
4. **Whole-Game Difficulty-Curve Verification v1** — #1651. Verify a campaign's
   realized difficulty curve against its declared shape using existing balance
   evidence.
5. **Content Curation Gate v1** — #1652. Implement the mandatory campaign
   promotion guard (per-level engine room + solvable + balanced + novel) on the
   existing path.
6. **Content-at-Scale Generation and Curation Demo v1** — #1653. A deterministic
   demo of campaign generation through curation, reusing
   runtime/probe/evaluator/evolve/provenance.
7. **Scenario Coverage v36: Content-at-Scale Regression Suite** — #1654. Continue
   Scenario Coverage numbering from v33 (Era F) onward.
8. **Roadmap and #1 Governance Refresh after Content-at-Scale Generation and
   Curation v1** — #1655. Refresh roadmap/#1 context only after the above are
   complete, preserving #1 and #23 as open anchors.

```text
#1648 scope -> #1649 -> #1650 -> #1651 -> #1652 -> #1653 -> #1654 -> #1655
```

Each follow-up issue must define the exact surface it extends, the files it
changes, the non-goals that stay blocked, the verification commands or checks for
closure, generated-state expectations, and proof that #1 and #23 remain open.
Issue closure requires all fixed PR units merged in order, latest `main` pulled,
issue-level verification run on latest `main`, the Definition of Done / guardrail
/ drift-prevention / over-engineering / generated-state audits recorded, a final
evidence comment, and #1 and #23 confirmed open.

## Explicit non-goals

This contract does not authorize, now or implicitly through any follow-up:

- a direct trusted write from generation, role agents, the producer, or any
  browser/Studio surface; proposals only, through the existing
  review/apply/trust-gradient path;
- autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden trusted
  writes; high-risk and source-affecting changes are never auto-applied;
- promotion of any campaign or level that has not passed the engine room (four
  gates + solver + over-solution) and the curation gate;
- destructive dedup that deletes evidence, runs, or prior content to manufacture
  novelty;
- a new engine, runtime, writer, or parallel generator where an existing surface
  should be reused;
- a browser command bridge, arbitrary shell execution, dependency install,
  CI/workflow mutation, credentialed operation, network install/update, or
  publish/deploy/sign/upload;
- unlicensed, uncredited, or unverified-style generated asset/audio/content
  promotion; license/provenance and the function-specific QA gate are mandatory;
- any automated quality/fun/taste claim; "looks good / sounds good / is fun" and
  art/audio/UX/narrative direction remain human decisions;
- hosted/cloud/paid capability, marketplace transaction layer, real-player
  telemetry, live-ops, or distributed/Elixir orchestration (Layer-3; DEFER per
  Milestone 26 / #1508, NO-GO per ADR #92);
- engine/content/system breadth beyond what a specific loop-produced rung
  (Milestone 24) justifies;
- generated runs/genre/evidence/registry/release artifacts committed unless
  explicitly fixture-scoped;
- any claim of a production-ready engine, Godot replacement/parity, or autonomous
  shipping of finished games.

## Generated-state policy

Generated run state, caches, local worktrees, build outputs, campaign artifacts,
and evidence bundles remain untracked unless a future issue explicitly scopes a
tiny deterministic fixture. Closure audits should use `git status --short
--ignored` or an equivalent check to confirm generated/local artifacts remain
ignored.

## #1 / #23 governance preservation

- #1 remains open as the broad roadmap/vision anchor.
- #23 remains open as the repo-memory/design context anchor.
- This contract does not replace, close, or narrow either anchor. Any change to
  #1 or #23 requires a separate explicit governance decision.
