# Deduplication and Novelty Metrics v1

Deduplication and Novelty Metrics v1 (#1650) is the second implementation slice
of Content-at-Scale Generation and Curation v1 (`docs/content-scale-v1.md`,
#1648) under #1 Era G Milestone 38. It computes descriptive **dedup** and
**novelty/variety** metrics over a generated proposal set (Campaign-Scale
Generation v1, `docs/content-scale-generation-v1.md`, #1649) so that scale does
not collapse into repetitive churn.

This slice reuses existing artifacts and evidence; it is **not** a new analysis
engine.

## What this slice adds

`content_novelty.rs` computes a `NoveltyReport` over a
`CampaignProposalSet`:

- a per-item **signature** — a deterministic digest (the existing `export_hash`
  helper) over the normalized generated artifact (the proposal's `to` payload,
  with the identity-only `id` field dropped so two items that differ only by id
  collide). When the caller supplies a Milestone 28 difficulty/solver signal for
  an item, it is folded into the signature so two items with identical content
  but a different measured difficulty are not treated as duplicates;
- **duplicate detection** — items that share a signature are grouped into
  duplicate clusters and each later member is flagged `isDuplicate` with its
  `duplicateOf`;
- a **novelty ratio** — `distinctCount / itemCount` — and a `lowNovelty` flag
  raised when the ratio falls below a declared threshold
  (`DEFAULT_NOVELTY_THRESHOLD = 0.5`).

## Contract

- **Computed from evidence, not asserted.** Every number is derived from the
  existing generated artifacts (and optional difficulty/solver evidence), so the
  report can be re-derived and audited. No similarity engine, embedding model, or
  external service is introduced.
- **Read/measure-only, never destructive.** Dedup identifies and flags
  duplicate/near-duplicate items; it never deletes evidence, runs, or prior
  content to manufacture novelty.
- **Descriptive only.** "Distinct," "duplicate," and "novel" are measurements
  against a declared, evidence-backed threshold — not a quality, fun, or taste
  claim. "Sufficiently novel" means "above the declared threshold."
- **Deterministic.** The same set and inputs always yield the same report.
- **Fail closed.** A malformed artifact or an out-of-range threshold is an error.

## Language and Studio boundary

Rust/local owns the metric logic. No JS/Studio changes — browser/Studio surfaces
remain read-only. No new language or runtime is introduced; distributed/Elixir
remains NO-GO per ADR #92 (`docs/distributed-elixir-design.md`).

## Generated-state policy

No generated runs/assets/content/release artifacts are committed. The only added
data files are the tiny deterministic fixtures under
`examples/generative-front-door/` (`content-novelty-campaign-v1.json`,
`content-novelty-campaign-low-v1.json`) consumed by the contract test
`crates/ouroforge-core/tests/content_novelty_contract.rs`.

## Wording

This slice makes no auto-merge, quality, fun, production-ready, shippable, or
Godot-replacement claim.

## #1 / #23 governance

#1 remains open as the roadmap/vision anchor and #23 as the repo-memory/design
context anchor. This slice does not close, narrow, or modify either; any such
change requires a separate explicit governance decision.
