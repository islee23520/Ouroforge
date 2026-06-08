# Card-Roguelite Substrate v1 Scope and Design Gate

## Decision

**GO** for an additive Card-Roguelite Substrate v1 under #1 Era I Milestone 47.
The substrate generalizes the deterministic deck-roguelike class from #1601 and
reuses the seeded-RNG/replay surfaces from #1600 so future deckbuilder variants
are authored as configuration over one shared substrate rather than as separate
engines.

The decision is **DEFER** for any browser- or Studio-owned trusted mutation,
cloud/mobile Layer-3 runtime, automated fun/quality scoring, release automation,
or generated asset/run output outside fixture-scoped evidence.

## Boundary

Rust/local owns the trusted substrate contract, validation, persistence-facing
schemas, deterministic balance/scoring logic, replay/provenance evidence, and CLI
entry points. JavaScript/browser surfaces may inspect substrate outputs and run
read-only deterministic demos where explicitly scoped, but they do not perform
trusted writes or bypass review/apply/trust-gradient paths. Python remains local
temporary tooling only and does not own any Era I contract.

The substrate owns these stable concepts:

- **Cards**: immutable card identity, tags, cost, action/effect references, and
  upgrade/meta-progression hooks.
- **Modifiers**: additive or multiplicative rule fragments applied through a
  deterministic ordering contract, never through ambient runtime state.
- **Deterministic resolution**: a seeded, replayable step function that accepts a
  run state, player choice, and config rules, then emits state plus evidence.
- **Run/ante**: escalating quota, ante, encounter, reward, loss, and run-end
  structure expressed as data/configuration.
- **Shop**: deterministic offer generation, pricing, reroll/removal/upgrade
  actions, and audit metadata tied to the run seed.
- **Seed**: every stochastic choice is derived from the existing seeded-RNG
  surface; configs do not introduce ad hoc randomness.
- **Meta-progression**: unlock and progression declarations are config-owned
  inputs validated by the substrate; they are not hidden global state.

A game config supplies theme, card catalog, encounters, scoring constants,
modifier catalogs, shop tables, quota/ante tuning, unlock lists, fixture names,
and presentation copy. A config may not fork the step function, bypass seeded
randomness, introduce trusted browser writes, or claim an automated fun verdict.

## Reuse Contract

Card-Roguelite Substrate v1 composes existing Ouroforge surfaces instead of
creating a parallel engine:

- #1601 remains the source for the existing deterministic deck-roguelike class
  behavior and its authoring vocabulary.
- #1600 remains the source for seeded-RNG determinism, replayability, and
  evidence expectations.
- Existing runtime, probe, evaluator gate, evolve/campaign, compare,
  provenance-bundle, asset, dashboard, cockpit, and CLI contracts are preserved
  unless a later issue records an explicit migration note.
- Browser/Studio surfaces remain read-only inspection or proposal-only authoring
  surfaces; trusted application continues through the existing review/apply path.

## Golden-Byte Backward-Compatibility Contract

The existing deck-roguelike class must be expressible as a substrate config with
**golden-byte parity**: for each fixture-scoped seed and scripted choice sequence
that is already covered by the class, the migrated substrate config must produce
byte-identical canonical JSON evidence for the approved observable fields.

The parity fixture set must include, at minimum:

1. initial deck/run state,
2. deterministic draw/offer order,
3. encounter resolution,
4. reward selection,
5. shop offer/pricing/reroll behavior,
6. quota/ante escalation,
7. run-loss and run-win terminal evidence, and
8. provenance tying the output to the same seed/config version.

Any intentional field rename, canonicalization change, or evidence-version bump
must be recorded as a migration note in the implementing issue and must not
silently replace existing evidence. Until such a note exists, the compatibility
gate is byte-for-byte parity for the existing class.

## Existing Deck-Roguelike as a Config

The current deck-roguelike class becomes the `deck-roguelike-classic` substrate
config. It supplies the same cards, encounter tables, reward tables, shop tables,
quota curve, and meta-progression declarations currently reachable through the
class front door. It does not own an alternate resolver. Its implementation gate
is the golden-byte contract above, so user-visible behavior remains unchanged.

## Engine-Builder Deckbuilder as a Second Config

The engine-builder deckbuilder becomes a second substrate config over the same
model. It may provide engine-part themed card catalogs, build-combo modifiers,
quota/shop tuning, and progression declarations, but it must use the same
substrate-owned step function, deterministic modifier ordering, seeded offer
selection, run/ante model, shop model, and evidence contract.

The second config is accepted only if it proves that the substrate extension is a
configuration boundary: new content and tuning without a new engine, hidden
trusted writes, or automated fun/quality claims.

## Dependency Order and Closure Gates

1. **#1791 Scope and Design Gate**: this document records GO/DEFER, boundary,
   reuse, config, and compatibility contracts.
2. **#1792 Substrate Core Model v1**: add trusted Rust model types and validators
   only after this gate exists.
3. **#1793 Deck-Roguelike-as-Substrate-Config Migration v1**: map the existing
   class to config and prove golden-byte parity.
4. **#1794 Engine-Builder Deckbuilder Config v1**: add the second config over the
   same model.
5. **#1795 Card-Roguelite Substrate Demo v1**: demonstrate read-only/demo usage
   without trusted browser writes.
6. **#1796 Scenario Coverage v42**: add regression coverage for the substrate and
   both configs.
7. **#1797 Roadmap and #1 Governance Refresh**: update public roadmap/governance
   wording while keeping #1 and #23 open.

Closure gates for every follow-up:

- no parallel engine,
- no behavior change for the existing class unless an explicit migration note is
  approved,
- Rust/local trusted ownership,
- browser/Studio read-only or proposal-only behavior,
- fixture-scoped generated artifacts only,
- conservative wording with no production-ready, Godot-parity, automated fun, or
  auto-merge claim, and
- #1 and #23 remain open anchors.
