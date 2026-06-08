# Escalating Run Structure and Shop Economy v1 Scope and Contract

Issue: #1805  
Parent: #1 Era I Milestone 49  
Status: scope and contract only

## Purpose

Escalating Run Structure and Shop Economy v1 defines the bounded contract for a
replayable card-roguelite run loop over the existing substrate: a run advances
through rising quota/ante targets, then uses a deterministic shop economy to
create draft variance and player-controlled probability levers between scoring
rounds.

This document is a scope gate only. It adds no executable behavior, no new
runtime, no new language surface, and no parallel engine. Follow-up issues must
reuse Card-Roguelite Substrate v1 and Multiplicative Scoring-Engine v1 rather
than introducing a separate run/shop engine or browser-owned trusted writer.

## Reuse and ownership statement

- Trusted run/shop model, validation, deterministic offer generation,
  quota/ante persistence, scoring integration, evidence writing, run/project
  binding, review/apply/trust-gradient integration, and CLI behavior are
  Rust/local owned.
- JavaScript/browser/Studio/dashboard/cockpit surfaces may inspect exported
  run/shop state or run explicitly scoped deterministic previews, but they remain
  read-only for trusted state and do not apply purchases, rerolls, removals, or
  run advancement.
- The existing card-roguelite substrate, scoring engine, seeded RNG contract,
  runtime probe, evaluator gates, evolve/campaign, compare, provenance bundle,
  asset surfaces, dashboard, cockpit, and CLI contracts are reused.
- Generated runs, shop traces, score reports, screenshots, temp servers, builds,
  and other local artifacts remain untracked unless a later issue explicitly
  scopes deterministic fixture data.

## Escalating quota/ante run model

A run is a bounded sequence of scoring rounds grouped by ante. Each round has a
validated target quota and a deterministic transition rule. The run ends only in
an explicit terminal state such as `won`, `lost`, `abandoned`, or
`blocked-invalid-state`.

Required shape for follow-up implementation issues:

- **Stable run id**: every run state carries a deterministic id, config id,
  config version, seed, ante, round index, and evidence refs.
- **Bounded length**: max antes, max rounds per ante, max shop actions, and max
  generated offers are explicit validated constants, not ambient runtime policy.
- **Rising targets**: score quotas increase by an explicit curve or table tied to
  ante/round state. A flat target is permitted only as a documented fixture or
  compatibility case.
- **Ante transitions**: reaching or exceeding the current quota advances to the
  next round/ante according to validated data; failing a quota records an
  explicit loss or blocked state.
- **Seed reproducibility**: all stochastic-looking decisions derive from the
  existing seeded RNG surface and are captured in trusted state/evidence.
- **Score integration**: scoring uses the existing substrate and scoring-engine
  resolution trace/digest; the run model does not fork scoring rules.
- **No hidden writes**: run advancement transforms trusted Rust/local state only
  through validated logic; browser/Studio surfaces do not commit trusted run
  changes.

The run model is mechanical evidence. It may report quota success/failure and
score progression, but it does not assert fun, quality, release readiness,
market demand, or production maturity.

## Shop economy contract

The shop provides per-run draft variance and player-controlled probability
levers between scoring rounds. It is deterministic for a given run state and
seed, but not luck-only: valid player actions must expose bounded ways to alter
future odds and scoring potential.

Required shop actions:

- **Buy**: spend declared currency to add a card, modifier, upgrade, or other
  approved substrate item from the current offer set.
- **Sell**: convert eligible owned items to declared currency or other explicit
  value according to a validated table.
- **Reroll**: spend declared currency to generate a new offer set from the seeded
  shop table while recording the reroll count and RNG draw.
- **Remove**: spend declared currency or consume a declared resource to remove an
  eligible card/item from the run state.

Required validation and read model:

- prices, sell values, reroll costs, removal costs, and inventory limits are
  validated data;
- offers are generated from declared pools, weights, eligibility gates, and the
  existing seeded RNG only;
- unavailable, duplicate, unaffordable, unsafe, or out-of-bound actions fail
  closed with actionable diagnostics;
- every accepted action emits an ordered shop trace and digest suitable for
  replay/debug evidence;
- the read model shows offers, prices, action availability, and probability
  levers without granting trusted write authority to browser/Studio surfaces.

The shop must provide levers over probability: reroll/removal/buy/sell choices
should let a player shape deck composition, modifier exposure, and future offer
odds. The contract rejects a luck-only loss model where failure is explained
solely by hidden RNG rather than replayable decisions and visible levers.

## Dependency order

Milestone 49 must land in this order so each step has a bounded verification
surface:

1. #1805 — Scope and contract (`docs/run-shop-v1.md`), no executable behavior.
2. #1806 — Escalating Quota/Ante Run Model v1 over the existing substrate and
   scoring engine.
3. #1807 — Shop Economy v1 with buy/sell/reroll/remove validation and
   probability levers.
4. #1808 — Run and Shop Demo v1 using fixture-scoped deterministic evidence.
5. #1809 — Scenario Coverage v44: Run and Shop Regression Suite.
6. #1810 — Roadmap and #1 governance refresh after merged evidence.

## Closure gates

Milestone 49 is not complete until later issues prove all of the following on
merged evidence:

- The run model is bounded, seed-reproducible, and fails closed on malformed or
  out-of-bound quota/ante state.
- The shop economy supports buy, sell, reroll, and remove with deterministic
  offer generation, validated costs, and replayable traces.
- Shop decisions provide visible probability levers; losses are not attributed
  to hidden luck-only outcomes.
- Demo and regression coverage are fixture-scoped and state/shape-only.
- Existing substrate, scoring engine, deck-roguelike golden parity,
  runtime/probe/evaluator, evolve/campaign, compare, provenance, dashboard,
  cockpit, and CLI contracts remain backward-compatible.
- #1 and #23 remain open governance anchors.

## Conservative wording and non-goals

This scope does not authorize:

- executable behavior in this issue;
- a parallel run/shop engine instead of substrate configuration and Rust/local
  validation;
- direct trusted writes from generation, browser, dashboard, cockpit, or Studio;
- browser command bridges, shell execution, dependency installs, credentialed
  operations, publish/deploy/sign/upload behavior, or CI/workflow mutation;
- autonomous apply, auto-merge, self-approval, reviewer bypass, hidden trusted
  writes, or unreviewed source mutation;
- automated fun/feel, quality, shippability, production-readiness, market-demand,
  or Godot replacement/parity claims;
- hosted/cloud/mobile Layer-3 capability, distributed orchestration, or Elixir
  runtime ownership.

The contract is descriptive and mechanical. It defines how bounded runs and shop
variance must be modeled, ordered, and verified; it does not claim the resulting
title is good, fun, shippable, commercially viable, production-ready, or an
engine replacement.
