# Synthetic Player Balance v1

Synthetic Player Balance v1 is a scope and contract document for #1 Era F
Milestone 32. It defines pre-launch balance telemetry as interpretable,
human-in-the-loop evidence: synthetic players modeled as human-like personas
play a small deck-roguelike game class over seeded runs, their play is
aggregated into descriptive balance telemetry, and the result is presented in a
read-only cockpit that a human reads — never an auto-applied nerf.

This document adds no executable behavior, fixtures, runtime features, Studio
surfaces, browser authority, or engine capability. It is a governance contract
for the follow-up implementation issues it sequences.

Synthetic players are modeled as human-like personas with skill and style
parameters, not win-maximizers. This follows the finding that balance signal
comes from how a range of real player skill/style profiles experience a game,
not from a single superhuman optimizer; the output is a cockpit of
interpretable evidence, never an automatic balance change.

## Scope

The contract covers four artifacts and the way they reuse existing surfaces:

1. the synthetic-player persona model (skill/style parameters; human-like, not
   win-maximizing);
2. the balance-telemetry aggregation contract (pick-rate, win-deck inclusion,
   degenerate-combo flags, dead-item flags, difficulty curve);
3. the read-only balance cockpit contract (interpretable evidence plus
   counterexample/replay; human-in-the-loop, never auto-nerf);
4. the seeded re-run/diff contract that reuses `compare` to re-run an identical
   seed distribution after a balance change and diff the impact.

The contract applies to balance-telemetry claims, cockpit wording, roadmap
wording, and any future request to grow persona, telemetry, or cockpit
behavior. Each follow-up issue must still scope concrete Rust/local changes and
verification; this document does not authorize implementation by itself.

All metrics defined here are **descriptive observations of synthetic seeded
runs**. They are not a balance guarantee, a quality or fun judgment, or a
production-readiness claim.

## Non-goals

Synthetic Player Balance v1 does not authorize:

- auto-applied nerfs or buffs; the cockpit is read-only and human-in-the-loop;
- win-maximizing, superhuman, or optimizer agents; human-like personas only;
- live or network telemetry; synthetic seeded runs only;
- a parallel comparison engine, runtime, or writer; it reuses `compare` and the
  deck-roguelike game class;
- direct trusted writes from generation or any browser/Studio surface; proposals
  flow only through the existing review/apply/trust-gradient path;
- autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden
  trusted writes;
- a browser command bridge, arbitrary shell execution, dependency install,
  CI/workflow mutation, credentialed operation, network install/update, or
  publish/deploy/sign/upload;
- hosted/cloud/paid capability, a marketplace transaction layer, or
  distributed/Elixir orchestration (Layer-3; DEFER per #1508, NO-GO per ADR
  #92);
- engine breadth (renderer/3D/audio/physics depth) beyond what this rung's gate
  justifies;
- committing generated runs/genre/evidence/registry artifacts unless explicitly
  fixture-scoped;
- any claim of a production-ready engine, Godot replacement/parity, or that
  generated games are good, fun, or shippable;
- closing or modifying #1 or #23 without a separate explicit governance
  decision.

## Reused surfaces

This contract reuses existing surfaces and adds no parallel engine:

- the deck-roguelike game class and deterministic runtime (#1601) as the game
  under test;
- the deterministic runtime `examples/game-runtime/runtime.js` and the
  `window.__OUROFORGE__` probe for browser-local read-only inspection;
- the evaluator's four-gate aggregation and `declared-gate-and` evidence shape
  for any pass/fail verdict;
- the evolve loop / campaign surface for driving repeated seeded runs;
- the `compare` / `evolve compare` semantic-compare path for the seeded
  re-run/diff contract;
- the provenance and evidence-bundle surfaces for recording run provenance;
- the review/apply/trust-gradient path (`source_apply_*`) as the only route for
  any proposed balance change to become a trusted write;
- the read-only dashboard and authoring-cockpit surfaces for presentation.

If a follow-up issue believes an existing surface is insufficient, it must say
why in that issue before introducing anything new.

## Synthetic-player persona model

A synthetic player is a **human-like persona**, described by interpretable
skill and style parameters, that drives the deck-roguelike runtime through the
existing runtime/probe contract. A persona is a data description consumed by the
runtime, not a new engine or AI service.

A persona has at least:

- a **skill** dimension (for example, decision quality or planning depth) bounded
  to a human-plausible range, never a superhuman optimizer;
- one or more **style** dimensions (for example, aggressive vs. cautious,
  greedy vs. economical, explorer vs. rusher) that bias choices without
  maximizing win rate;
- a deterministic **seed binding** so a persona's play is reproducible.

Personas are explicitly **not** win-maximizers. A persona that always plays the
mathematically optimal line is out of scope; the value of the model is that a
spread of human-like personas reveals where a design is frustrating,
degenerate, or dead for real players, not where a solver can break it.

A follow-up issue (#1606) implements the persona agents. This contract only
fixes the persona shape and the rule that personas are human-like and seeded.

## Balance-telemetry aggregation contract

Balance telemetry aggregates many seeded persona runs into descriptive signals.
The v1 signal set is:

- **pick-rate** — how often each card/item/option is chosen across runs, by
  persona where useful;
- **win-deck inclusion** — how often each card/item appears in runs that reach a
  win/success state;
- **degenerate-combo flags** — descriptive flags where a combination dominates
  outcomes beyond a declared threshold, surfaced for human review (not an
  automatic nerf trigger);
- **dead-item flags** — descriptive flags where a card/item is effectively never
  picked or never contributes to success;
- **difficulty curve** — a per-stage/per-segment descriptive view of success and
  failure across personas.

Every signal is a descriptive observation of synthetic seeded runs and carries
that framing in its wording. A flag is an invitation for a human to look, not a
verdict that the design is wrong. Aggregation reuses the evaluator/evidence
surfaces; it does not introduce a parallel metrics engine.

A follow-up issue (#1607) implements the aggregation. This contract fixes the
v1 signal set and the descriptive-only framing.

## Read-only balance cockpit contract

The balance cockpit is a **read-only** presentation surface over the telemetry
and its underlying runs. It is built on the existing read-only dashboard /
authoring-cockpit surfaces and the `window.__OUROFORGE__` probe; it has no
trusted-write authority.

The cockpit must:

- present the v1 telemetry signals as interpretable evidence with conservative
  wording;
- for any flag, link to a **counterexample / replay** — a specific seeded run a
  human can re-watch through the runtime — so the evidence is checkable, not a
  black-box score;
- keep the human in the loop: it never applies, proposes-as-applied, or
  auto-merges a balance change.

Any balance change a human decides to make is authored and applied only through
the existing review/apply/trust-gradient path, never from the cockpit or any
browser/Studio surface. The cockpit's job ends at presenting evidence.

A follow-up issue (#1608) implements the cockpit surface and the re-run diff
view. This contract fixes its read-only, replay-backed, human-in-the-loop
boundary.

## Seeded re-run / diff contract

To measure the impact of a balance change, the re-run contract:

1. records the seed distribution and persona set used for a baseline telemetry
   run;
2. after a balance change is applied through the normal review/apply path,
   re-runs the **identical** seed distribution and persona set;
3. diffs the two telemetry results using the existing `compare` /
   `evolve compare` semantic-compare path, surfacing how each signal moved.

The re-run/diff must reuse `compare`; it does not add a parallel comparison
engine. The diff is descriptive — it shows how the telemetry moved, not whether
the change was good. Interpretation stays with the human reading the cockpit.

A follow-up issue (#1608) implements the re-run/diff surface alongside the
cockpit.

## Language boundary

- Rust owns trusted validation, persistence, the persona/telemetry/aggregation
  logic, evidence and provenance writing, run/project binding, the
  review/apply/trust-gradient path, and CLI behavior.
- TypeScript/JavaScript owns the deterministic runtime, the
  `window.__OUROFORGE__` probe, browser-local read-only inspection, and static
  dashboard/cockpit behavior where explicitly scoped.
- Python may be used only for temporary local tooling or smoke helpers and must
  not own core Era F contracts.
- No new language/runtime is introduced without explicit issue-level
  authorization; distributed/Elixir remains NO-GO per ADR #92
  (`docs/distributed-elixir-design.md`).

## Dependency and closure order

Milestone 32 follow-up work should remain in this order unless a later issue
documents a concrete blocker and replacement ordering:

1. #1605 — Synthetic Player Balance v1 Scope and Contract (this issue).
2. #1606 — Synthetic Player Persona Agents v1 (depends on the deck-roguelike
   game class and runtime, #1601).
3. #1607 — Balance Telemetry Aggregation v1.
4. #1608 — Balance Cockpit Read-Only Surface and Re-Run Diff v1.
5. #1609 — Synthetic Player Balance Demo v1.
6. #1610 — Scenario Coverage v32: Synthetic Player Balance Regression Suite.
7. #1611 — Roadmap and #1 Governance Refresh after Synthetic Player Balance v1.

```text
#1605 scope -> #1606 -> #1607 -> #1608 -> #1609 -> #1610 -> #1611
```

This ordering records the governed follow-up chain only; it does not authorize
implementing, modifying, closing, or claiming any later issue from this contract
alone.

Closure gates for each follow-up issue:

- the prior issue in the dependency order is merged or explicitly documented as
  superseded by a maintainer-approved governance decision;
- the issue scopes one bounded artifact (persona, telemetry, cockpit, re-run
  diff, demo, coverage, or governance);
- any engine or surface growth cites the specific gate and evidence gap;
- four-gate evidence and a loop-coverage verdict are present for any executable
  claim;
- telemetry wording stays descriptive (no balance/quality/fun guarantee);
- generated-state and trust-boundary audits pass;
- #1 and #23 are checked and remain open anchors unless a separate explicit
  governance decision exists.

## Governance audit for #1 and #23

#1 remains the broad vision and roadmap anchor. #23 remains the repo-memory and
design context anchor. This contract preserves both as open anchors and does not
modify, close, replace, or narrow either issue.

Future work that proposes to change either anchor must be a separate explicit
governance decision. That decision must identify the replacement source of
truth, record maintainer approval, and avoid implying that this Synthetic
Player Balance contract itself authorized the change.
