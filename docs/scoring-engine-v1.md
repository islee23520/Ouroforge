# Multiplicative Scoring-Engine and Modifier Composition v1

Issue: #1798  
Parent: #1 Era I Milestone 48  
Status: scope and contract only

## Purpose

Multiplicative Scoring-Engine and Modifier Composition v1 defines the bounded
contract for the card-roguelite genre's mechanical scoring moat: simple,
readable card and modifier effects that combine deterministically into emergent,
hard-to-solve scoring states. This document is a scope gate only. It adds no
executable behavior, no new runtime, and no parallel engine.

Milestone 48 builds on Card-Roguelite Substrate v1 / Milestone 47. A deckbuilder
or engine-builder variant remains a configuration over the existing
card-roguelite substrate. The scoring engine extends substrate configuration and
Rust/local validation rather than introducing a separate engine or browser-owned
trusted writer.

## Reuse and ownership statement

- Trusted model, validation, deterministic resolution, score digesting,
  persistence, evidence writing, run/project binding, review/apply/
  trust-gradient integration, and CLI behavior are Rust/local owned.
- JavaScript/browser/Studio/dashboard/cockpit surfaces may inspect exported state
  or run deterministic browser-local previews where explicitly scoped, but they
  remain read-only for trusted state.
- The existing card-roguelite substrate, seeded RNG contract, runtime probe,
  evaluator gates, evolve/campaign, compare, provenance bundle, asset surfaces,
  dashboard, cockpit, and CLI contracts are reused.
- Generated runs, builds, assets, score traces, reports, screenshots, temp
  servers, and other local artifacts remain untracked unless a later issue
  explicitly scopes deterministic fixture data.

## Modifier and effect model

A modifier is an individually readable, one-line effect declaration over a
substrate card, run, shop, or scoring context. The contract requires each effect
piece to be understandable in isolation while allowing the combined score state
to become non-obvious.

Required shape for follow-up implementation issues:

- **Stable id**: every modifier and effect has a deterministic, repo-stable id.
- **Readable text**: every effect has concise player-facing wording such as
  `double score from starter cards` or `add +3 before multipliers`.
- **Explicit scope**: every effect declares whether it applies to a card, tag,
  hand/deck group, ante/run context, shop offer, or scoring phase.
- **Explicit operation**: additive, multiplicative, clamp/floor, selector, or
  gate effects are declared as data, not hidden code paths.
- **Explicit order key**: every effect resolves in a deterministic phase/order.
- **No hidden nondeterminism**: random-looking outcomes must derive from the
  existing seeded RNG and must be captured in trusted state/evidence.
- **No hidden writes**: effects may transform a computed score state; they do not
  write trusted files, bypass review, or mutate browser/Studio state.

Readable parts are the design rule. A modifier may be strong or surprising in
composition, but no single effect should require reading a solver or hidden
runtime path to understand its local contribution.

## Deterministic multiplicative resolution contract

The resolution engine for follow-up issues must be deterministic, seed-stable,
and explicit about ordering. A valid implementation resolves the same substrate
config and seed to the same score state, score trace, and digest across runs.

Required resolution phases:

1. **Validate config**: fail closed on unknown ids, unsafe refs, invalid phases,
   duplicate order keys where uniqueness is required, malformed selectors,
   non-finite numeric values, overflow risk, and boundary drift.
2. **Collect candidates**: select cards, tags, modifiers, and run/shop context
   from the validated substrate state only.
3. **Apply additive phase**: apply declared additions in stable `(phase, order,
   id)` order.
4. **Apply multiplicative phase**: apply declared multipliers in stable `(phase,
   order, id)` order after additions unless the effect explicitly declares a
   different bounded phase.
5. **Apply clamps/floors/caps**: apply bounded post-processing in stable order.
6. **Emit trace/read model**: emit a Rust/local read model that explains the
   ordered state transitions without exposing trusted write authority.
7. **Digest result**: hash canonical score inputs, ordered effects, and final
   state for replay/debug evidence.

The model must use checked arithmetic or an explicitly bounded equivalent. If a
score cannot be resolved safely, the trusted result is a failed validation or
blocked score state, not a silent pass or browser-side fallback.

## Combinatorial composition model

The milestone's moat is composition: individually readable effects produce a
large, deterministic, difficult-to-solve scoring space when combined.

Follow-up implementation must keep these constraints:

- **Readable parts**: each effect remains simple enough to explain in one line.
- **Unsolved whole**: the aggregate state may contain many interacting additive,
  multiplicative, selector, and clamp effects.
- **Deterministic traceability**: every aggregate result can be replayed and
  traced from ordered effect applications.
- **No subjective fun verdict**: mechanical score complexity and balance signals
  are descriptive; they do not assert that the game is fun or good.
- **Human gate preserved**: fun/feel, release readiness, Steam/account/signing,
  market demand, content survey, and release go/no-go remain human/Ring-3 or
  later Era J decisions.
- **Substrate compatibility**: existing deck-roguelike and card-roguelite
  substrate configs remain backward-compatible unless a later issue explicitly
  records a migration note and regression evidence.

## Dependency order

Milestone 48 must land in this order so each step has a bounded verification
surface:

1. #1798 — Scope and contract (`docs/scoring-engine-v1.md`), no executable
   behavior.
2. #1799 — Modifier and Effect Model v1 over the existing substrate.
3. #1800 — Deterministic Multiplicative Resolution Engine v1 with explicit
   ordering and score trace/digest behavior.
4. #1801 — Combinatorial Composition Model v1 for readable parts and unsolved
   wholes.
5. #1802 — Scoring-Engine Demo v1 using fixture-scoped deterministic evidence.
6. #1803 — Scenario Coverage v43: Scoring-Engine Regression Suite.
7. #1804 — Roadmap and #1 governance refresh after merged evidence.

## Closure gates

Milestone 48 is not complete until later issues prove all of the following on
merged evidence:

- Modifier/effect model reuses the substrate and rejects malformed or unsafe
  effect declarations fail-closed.
- Multiplicative resolution is deterministic, seed-stable, explicitly ordered,
  and replayable by trace/digest.
- Composition demonstrates readable individual effects with non-trivial combined
  score states.
- Demo and regression coverage are fixture-scoped and state/shape-only.
- Existing substrate, deck-roguelike golden parity, runtime/probe/evaluator,
  evolve/campaign, compare, provenance, dashboard, cockpit, and CLI contracts
  remain backward-compatible.
- #1 and #23 remain open governance anchors.

## Conservative wording and non-goals

This scope does not authorize:

- a parallel engine instead of a substrate config;
- direct trusted writes from generation, browser, dashboard, cockpit, or Studio;
- browser command bridges, shell execution, dependency installs, credentialed
  operations, publish/deploy/sign/upload behavior, or CI/workflow mutation;
- autonomous apply, auto-merge, self-approval, reviewer bypass, hidden trusted
  writes, or unreviewed source mutation;
- automated fun/feel, quality, shippability, production-readiness, market-demand,
  or Godot replacement/parity claims;
- hosted/cloud/mobile Layer-3 capability, distributed orchestration, or Elixir
  runtime ownership.

The contract is descriptive and mechanical. It defines how score mechanics must
be modeled, ordered, and verified; it does not claim the resulting title is good,
fun, shippable, commercially viable, production-ready, or an engine replacement.
