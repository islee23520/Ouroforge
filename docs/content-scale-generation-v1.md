# Campaign-Scale Generation v1

Campaign-Scale Generation v1 (#1649) is the first implementation slice of
Content-at-Scale Generation and Curation v1 (`docs/content-scale-v1.md`, #1648)
under #1 Era G Milestone 38. It extends the Milestone 30 generative front door
(`docs/generative-front-door-v1.md`) from a single level to **campaign scale** —
a set of level/encounter proposals — and from one genre to two: the grid-puzzle
class already reachable through the front door, plus the deck-roguelike class
(`docs/deck-roguelike-game-class-v1.md`, #1601).

This slice reuses the existing proposal path; it is **not** a new generator.

## What this slice adds

- **A deck-roguelike genre on the front door.** `generative_intake.rs` gains
  `DeckRoguelikeBrief` and `intake_deck_roguelike_brief`, which assemble and
  validate a deck-roguelike artifact (`ouroforge.deck-roguelike.v1`, mirroring
  the Deck-Roguelike Game Class v1 fixture) and wrap it in the existing
  `MutationProposal` / `GenerationProvenance` model — the same shape the
  grid-puzzle intake already produces. No new proposal model, no new writer.
- **Campaign-scale set generation.** `content_scale_generation.rs` turns one
  `CampaignBrief` (campaign metadata plus per-genre item briefs) into a
  `CampaignProposalSet` — a set of proposals, one per item, routed item-by-item
  through the genre's existing front-door intake. The set records the genres it
  covers and is itself validated fail-closed.

## Contract

- **Proposals only.** Campaign-scale generation is proposal-only. Every proposal
  in a campaign set is identical in trust to any other proposal entering the
  review/apply/trust-gradient path: it is `proposed` / `pending` / `unverified`
  and is never promoted here. The set records `proposalOnly: true`. Campaign
  generation performs no trusted write, auto-apply, self-approval, or reviewer
  bypass.
- **No per-game escape hatch.** A single malformed item — an unsupported genre,
  a deck that references an undeclared card, a puzzle with no player — fails the
  whole campaign closed. There is no path that silently drops or specially
  handles one bad item.
- **Engine room is downstream.** A freshly generated campaign set is a batch of
  unverified proposals. Promotion still requires the per-level engine room (four
  gates + solver + over-solution) and the curation gate (#1650–#1652), which are
  out of scope for this slice.
- **Deterministic.** `generate_campaign` takes `now_unix_ms` from the caller and
  never reads the clock, filesystem, or network, so the same brief yields the
  same set.

## Language and Studio boundary

Rust/local owns the new generation-proposal and validation logic. The
deterministic runtime and `window.__OUROFORGE__` probe stay in JavaScript;
browser/Studio surfaces remain read-only and are not changed by this slice. No
new language or runtime is introduced; distributed/Elixir remains NO-GO per
ADR #92 (`docs/distributed-elixir-design.md`).

## Generated-state policy

Generated proposals, campaign sets, runs, and caches remain untracked. The only
committed artifacts are the tiny deterministic fixtures under
`examples/generative-front-door/` (`campaign-scale-brief-v1.json`,
`campaign-scale-brief-invalid.json`) used by the contract test
`crates/ouroforge-core/tests/content_scale_generation_contract.rs`.

## Wording

This slice makes no auto-merge, quality, fun, production-ready, shippable, or
Godot-replacement claim. "Generated" here means "assembled into a validated
proposal awaiting review," not "good" or "balanced."

## #1 / #23 governance

#1 remains open as the roadmap/vision anchor and #23 as the repo-memory/design
context anchor. This slice does not close, narrow, or modify either; any such
change requires a separate explicit governance decision.
