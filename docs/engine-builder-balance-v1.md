# Engine-Builder Balance Verification v1

Engine-Builder Balance Verification v1 is a scope and contract document for #1
Era I Milestone 50. It extends Synthetic Player Balance v1 from Milestone 32
(`docs/synthetic-player-balance-v1.md`) to the engine-builder/deckbuilder shape:
synthetic seeded runs should produce descriptive evidence about whether an
economy appears solved, unfair, or mechanically unwinnable under declared
conditions.

This document adds no executable behavior, fixtures, runtime features, Studio
surfaces, browser authority, generated run outputs, or engine capability. It is
a governance contract for the follow-up implementation issues it sequences.

Engine-builder/deckbuilder work is a configuration over the existing
card-roguelite substrate. It is not a separate engine, analyzer, runtime, or
balance authority. The v1 verdicts defined here are descriptive observations
over synthetic runs; they are not a quality judgment, a fun judgment, or a
shipping claim.

## Scope

The contract covers four Milestone 50 artifacts and the way they extend
Milestone 32:

1. combo-explosion and degenerate-build detection over synthetic seeded runs;
2. dominant-build analysis that extends Milestone 32 pick-rate and win-rate
   telemetry;
3. fairness verification through loss-attribution and winnable-with-skill
   evidence;
4. daily-seed solvability verification for declared persona and seed sets.

The contract applies to balance-verification claims, roadmap wording, cockpit
and dashboard wording, evidence-bundle wording, and any future request to grow
engine-builder verification behavior. Each follow-up issue must still scope
concrete Rust/local changes and verification. This document does not authorize
implementation by itself.

All metrics defined here are descriptive observations of synthetic seeded runs.
They are evidence for human review, not automatic design changes and not a claim
that a title is good, fun, or complete.

## Non-goals

Engine-Builder Balance Verification v1 does not authorize:

- a parallel engine, analyzer, solver, runtime, or persistence path;
- replacing Milestone 32 Synthetic Player Balance v1;
- direct trusted writes from generation or any browser/Studio surface;
- autonomous apply, self-approval, reviewer bypass, or hidden trusted writes;
- win-maximizing, superhuman, or opaque optimizer agents as the primary balance
  authority;
- browser command bridges, arbitrary shell execution, dependency install,
  credentialed operation, network update, publish, deploy, upload, or signing;
- hosted/cloud/mobile Layer-3 capability, which remains deferred per Milestone
  26 / #1508;
- distributed/Elixir orchestration, which remains NO-GO per ADR #92;
- generated runs, assets, builds, or evidence outputs committed to git unless
  they are explicitly fixture-scoped;
- claims of engine equivalence, shipping readiness, automated quality, or
  automated fun;
- Steam account creation, signing, market-demand checks, wishlists, user
  acquisition, discoverability, or release-button decisions;
- closing or modifying #1 or #23 without a separate explicit governance
  decision.

## Reused surfaces and substrate

This contract reuses the Milestone 32 balance substrate and existing Ouroforge
surfaces. Engine-builder/deckbuilder is modeled as configuration over the
card-roguelite substrate, with different economy, card, relic, upgrade, and
progression definitions. It is not a new engine family.

The follow-up implementation issues must prefer the existing surfaces:

- the card-roguelite game class and deterministic runtime as the game substrate;
- synthetic player personas and seeded runs from Milestone 32;
- evaluator aggregation and declared gate evidence shapes;
- the evolve loop and campaign surfaces for repeated seeded runs;
- `compare` / `evolve compare` for seeded re-run and semantic diff evidence;
- provenance and evidence-bundle surfaces for recording run provenance;
- asset, dashboard, cockpit, and CLI surfaces where already scoped;
- the review/apply/trust-gradient path as the only route for trusted changes.

Browser and Studio surfaces are read-only inspection and presentation surfaces
for this contract. They may display evidence, filters, examples, and replay
links when a follow-up issue scopes that work, but they do not own trusted
validation, persistence, or apply behavior.

If a follow-up issue believes an existing surface is insufficient, it must
document the gap before introducing new structure. New work must remain
additive and backward-compatible with Milestone 32 evidence.

## Combo-explosion and degenerate-build detection contract

Combo-explosion detection identifies seeded runs where a build's economic or
combat output grows beyond declared mechanical bounds. The signal is meant to
answer whether a configuration appears solved by an explosive loop, not whether
the design is enjoyable.

The v1 contract covers descriptive flags for:

- unbounded or near-unbounded resource growth across turns, rooms, fights, or
  shops;
- repeated-action loops that produce more draw, currency, energy, damage,
  block, scaling, or upgrade value than their declared costs consume;
- deterministic win paths that require little adaptation once a small set of
  pieces appears;
- build states where failure becomes mechanically implausible across the
  declared seed/persona set;
- degenerate avoidance, stall, or farming loops that prevent normal run
  pressure from mattering.

Each flag must be tied to a specific seed, persona, build state, and evidence
trace that a human can inspect. Thresholds must be declared by the follow-up
issue that implements the check. A flag is not an automatic nerf, rejection, or
apply instruction.

The detection extends Milestone 32 degenerate-combo flags. It must not introduce
a parallel analyzer; it should reuse synthetic runs, evaluator aggregation,
compare output, provenance, and replay/counterexample evidence.

## Dominant-build analysis contract

Dominant-build analysis extends Milestone 32 pick-rate and win-deck inclusion
telemetry to engine-builder/deckbuilder configurations. It asks whether one
build family appears to crowd out alternatives across synthetic seeded runs.

The v1 signal set is:

- **pick-rate concentration** - how often cards, upgrades, relics, economy
  nodes, or build-defining choices are selected across personas and seeds;
- **win-rate concentration** - how often those choices appear in successful
  runs compared with their availability and pick rate;
- **archetype dominance** - whether declared build families occupy a
  disproportionate share of wins, high scores, or late-run survivals;
- **counter-pick absence** - whether available alternatives rarely improve
  outcomes against the dominant family;
- **persona skew** - whether the dominance appears only for a narrow persona
  band or persists across human-like skill/style profiles.

Dominant-build verdicts are descriptive. They should say that evidence suggests
concentration, skew, or crowd-out under a declared run set. They must not say
that the game is solved for all players or that a change should be applied.

This analysis extends Milestone 32 balance-telemetry aggregation. It reuses the
same seeded-run and aggregation surfaces, adding engine-builder dimensions such
as economy node, build family, upgrade path, shop pattern, and progression
choice where a follow-up issue scopes those fields.

## Fairness and daily-seed solvability contract

Fairness verification asks whether losses can be attributed to player decisions
under the declared synthetic personas, rather than to unavoidable luck in the
seed. For this contract, fairness means a skilled player can attribute losses to
decisions. It does not mean every run is winnable by every persona.

The v1 fairness evidence should cover:

- **loss attribution** - the run records enough decision context to identify
  plausible decision points that contributed to failure;
- **winnable-with-skill evidence** - stronger human-like personas, or the same
  persona with better decisions within declared bounds, can reach success on a
  seed that weaker play loses;
- **unavoidable-loss flags** - descriptive flags for seeds where declared
  skilled personas cannot find a plausible successful line under the v1 model;
- **pressure curve evidence** - stage, encounter, shop, reward, and economy
  pressure are correlated with failure points rather than hidden as a final
  binary result;
- **counterexample replay** - any fairness or solvability flag points to a seed,
  persona, and replay/evidence trace that a human can inspect.

Daily-seed solvability verification applies the same evidence contract to the
declared daily-seed set. It should report whether the declared skilled persona
set found at least one plausible successful line, which seeds were flagged, and
which loss-attribution evidence was available. It must not guarantee that a
human will win, enjoy, or approve the daily.

Fairness and solvability evidence must remain additive to Milestone 32. The
follow-up issues may broaden telemetry dimensions for engine-builder economy
shape, but they must reuse the existing runtime, evaluator, evolve, compare,
provenance, dashboard, cockpit, and CLI contracts where applicable.

## Language boundary

- Rust owns future trusted validation, persistence, substrate/scoring/balance
  logic, evidence writing, provenance, run/project binding, review/apply,
  trust-gradient behavior, and CLI behavior.
- TypeScript/JavaScript owns the deterministic runtime, runtime probe,
  in-game/browser-local inspection, dashboard, cockpit, and read-only
  presentation behavior where explicitly scoped.
- Browser and Studio surfaces are read-only for this contract. They may inspect
  and present evidence; they do not own trusted writes or persistence.
- Python may be used only for temporary local tooling or smoke helpers and must
  not own core Era I balance-verification contracts.
- No new runtime or language is introduced without explicit issue-level
  authorization. Distributed/Elixir remains NO-GO per ADR #92.

## Dependency and closure order

Milestone 50 follow-up work should remain in this order unless a later issue
documents a concrete blocker and replacement ordering:

1. #1811 - Engine-Builder Balance Verification v1 Scope and Contract (this
   issue).
2. #1812 - combo-explosion and degenerate-build detection over synthetic runs.
3. #1813 - dominant-build analysis extending Milestone 32.
4. #1814 - fairness and daily-seed solvability verification.
5. #1815 - Engine-Builder Balance Demo v1.
6. #1816 - Scenario Coverage v45: Engine-Builder Balance Regression Suite.
7. #1817 - Roadmap and #1 Governance Refresh after Engine-Builder Balance v1.

```text
#1811 scope -> #1812 -> #1813 -> #1814 -> #1815 -> #1816 -> #1817
```

This ordering records the governed follow-up chain only. It does not authorize
implementing, modifying, closing, or claiming any later issue from this contract
alone.

Closure gates for each follow-up issue:

- the prior issue in the dependency order is merged or explicitly documented as
  superseded by a maintainer-approved governance decision;
- the issue scopes one bounded artifact and cites how it extends Milestone 32;
- engine-builder/deckbuilder remains configuration over the card-roguelite
  substrate;
- any new field, surface, or gate cites the specific evidence gap it fills;
- verdict wording stays descriptive and avoids quality, fun, or shipping
  claims;
- Rust/local ownership is preserved for trusted validation and persistence;
- browser/Studio surfaces remain read-only;
- generated runs and outputs remain untracked unless explicitly fixture-scoped;
- #1 and #23 are checked and remain open anchors unless a separate explicit
  governance decision exists.

## Governance audit for #1 and #23

#1 remains the broad Era roadmap and milestone anchor. #1811 is scoped under #1
Era I Milestone 50 and does not replace or narrow that anchor.

#23 remains the repo-memory and design-context anchor. #1811 does not modify,
close, replace, or narrow #23.

Future work that proposes to change either anchor must be a separate explicit
governance decision. That decision must identify the replacement source of
truth, record maintainer approval, and avoid implying that this
Engine-Builder Balance Verification contract itself authorized the change.

## Conservative wording

Public and in-repo wording for this work should use conservative phrases such
as:

- "descriptive evidence from synthetic seeded runs";
- "flagged for human review";
- "suggests concentration under the declared run set";
- "winnable-with-skill evidence for declared personas";
- "extends Synthetic Player Balance v1 / Milestone 32";
- "configuration over the existing card-roguelite substrate";
- "read-only browser/Studio presentation".

Wording should avoid implying autonomous trusted changes, automated quality or
fun judgments, mature-engine equivalence, or universal solvability. The contract
verifies mechanical and balance surfaces only; final feel, taste, market, and
release decisions remain human-owned.
