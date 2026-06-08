# Candidate Generation and Curation Cockpit v1 Scope and Contract

Candidate Generation and Curation Cockpit v1 is a scope/contract milestone under
#1 Era J Milestone 57: creation becomes a curation act. The milestone lets an
author ask Ouroforge to generate many candidate variants, inspect them in a
read-only cockpit, and record the human selection as provenance. It does not add
executable behavior in this issue, does not add a new writer, and does not
perform a trusted write from generation.

The contract is intentionally narrow: N-variant generation produces proposals;
humans choose which proposal, if any, enters the existing review/apply/trust-
gradient path. Browser and Studio surfaces may inspect candidate sets and
selection evidence, but trusted validation, persistence, provenance, proposal
classification, and later apply decisions remain Rust/local owned.

Issues #1 and #23 remain open governance anchors. This scope does not close,
modify, or narrow either issue.

## Gate outcome: GO (bounded)

The decision for Candidate Generation and Curation Cockpit v1 is **GO**, bounded
to contract definition and the follow-up sequence below. Any capability not
explicitly listed here remains **DEFER** by default.

GO is justified only because this milestone reuses existing Ouroforge surfaces:

- the Milestone 30 generative front door (`docs/generative-front-door-v1.md`) for
  brief/NL intake and proposal generation;
- the card-roguelite substrate (`docs/card-roguelite-substrate-v1.md`) for
  deckbuilder/card/tuning variants as configuration over one deterministic
  substrate rather than a parallel engine;
- the existing review/apply/trust-gradient path for all trusted writes;
- runtime, evaluator, evolve/campaign, compare, provenance-bundle, asset,
  dashboard, cockpit, and CLI contracts for deterministic evidence and
  inspection;
- static dashboard/cockpit JavaScript only as read-only presentation surfaces;
  and
- Rust/local validation and provenance writing for trusted state.

This GO authorizes proposal and curation contracts only. It does not authorize a
new generator, new runtime, new engine, new language, browser-owned trusted
mutation, hosted/cloud/mobile Layer-3 capability, release automation, or any
automated quality/fun verdict.

## N-variant candidate generation contract

N-variant generation extends the Milestone 30 generation path from a single
proposal to a bounded candidate set:

1. **Input.** A human-authored brief or design intent declares the target game
   class/config, candidate count `N`, allowed knobs, and evidence expectations.
   The request may target cards, tuning, flavor text, store copy, or other
   explicitly-scoped proposal artifacts.
2. **Reuse.** The path reuses Milestone 30 intake and proposal modeling. For
   card/deckbuilder content, variants are expressed as card-roguelite substrate
   configuration, not as a new engine or alternate resolver.
3. **Output.** The output is a candidate set: stable candidate ids, proposal
   payloads, source brief refs, generation parameters, deterministic seeds where
   applicable, and evidence refs for later validation. The output is untrusted
   proposal data.
4. **Trust.** Generation never applies, auto-applies, self-approves, bypasses a
   reviewer, or writes trusted source/project state. A candidate becomes eligible
   for trusted persistence only after the existing review/apply/trust-gradient
   path and its human gates.
5. **Artifacts.** Generated runs, assets, decks, copy, builds, and other outputs
   remain untracked unless a later issue explicitly scopes fixture evidence.

The generated candidate set may contain many possibilities. It does not claim
that any candidate is good, fun, production-ready, shippable, or superior to a
human-authored alternative. It only records that candidates were generated under
bounded, inspectable inputs.

## Curation cockpit contract

The curation cockpit is a read-only inspection and selection-recording surface:

- It may display candidate ids, brief/source refs, proposal summaries,
  card/tuning/flavor/store-copy diffs, deterministic evidence refs, evaluator or
  probe status where available, provenance chain, and current review status.
- It may let a human record a selection decision as provenance: selected,
  rejected, deferred, needs-rework, or superseded, plus human rationale and the
  candidate set/version being judged.
- It must make the selection provenance explicit: who/what actor recorded the
  decision, when, against which candidate set, and which candidate payload hash or
  evidence refs were selected.
- It must not run generation as a trusted writer, execute local commands, mutate
  source, apply candidates, promote content, approve its own output, or hide the
  review/apply/trust-gradient step.
- Browser/Studio presentation remains read-only over Rust/local evidence. Any
  later click target that would create trusted state must route to a Rust/local
  command or proposal record explicitly scoped by a follow-up issue.

Selection is provenance, not application. A human may identify a preferred
candidate, but the selected candidate is still a proposal until the existing
trusted path accepts it.

## Human selection provenance

Every selection record must preserve enough information to audit the decision:

- candidate set id and version;
- selected candidate id or explicit no-selection result;
- source brief/design intent refs;
- proposal payload hash or stable evidence refs;
- generation path and parameter summary, including Milestone 30/substrate reuse
  references where applicable;
- human rationale or selection note;
- actor, timestamp, and local project/run binding; and
- downstream review/apply status if a later issue connects the selection to an
  apply proposal.

A missing, malformed, stale, or mismatched selection record fails closed for any
later promotion gate. The cockpit may show such a record as invalid evidence, but
it must not repair it by making a trusted write from the browser.

## Language boundary

- **Rust** owns trusted validation, persistence-facing schemas, candidate-set and
  selection provenance contracts, substrate/scoring/balance/export/provenance
  logic, evidence writing, run/project binding, review/apply/trust-gradient
  behavior, and CLI behavior.
- **TypeScript/JavaScript** owns deterministic runtime behavior, the
  `window.__OUROFORGE__` probe, in-game UI, juice/feedback, browser-local
  read-only inspection, and static dashboard/cockpit behavior where explicitly
  scoped.
- **Python** may be used only for temporary local tooling or smoke helpers and
  must not own core Era J contracts.
- No new language/runtime is introduced without explicit issue-level
  authorization; distributed/Elixir remains NO-GO per ADR #92
  (`docs/distributed-elixir-design.md`).

## Dependency order and closure gates

The follow-up issues must stay in this order and must reuse the existing
substrate, generation, provenance, dashboard/cockpit, and review/apply surfaces:

1. **#1851 Scope and Contract** — this document.
2. **#1852 N-Variant Candidate Generation v1** — add bounded candidate-set
   proposal generation reusing Milestone 30 and substrate/config paths.
3. **#1853 Read-Only Curation Surface v1** — expose candidate sets and selection
   provenance through a read-only cockpit/dashboard surface.
4. **#1854 Curation Cockpit Demo v1** — fixture-scoped deterministic demo of
   generation-to-curation inspection without trusted browser writes.
5. **#1855 Scenario Coverage v51: Curation Cockpit Regression Suite** — lock the
   proposal-only, read-only, provenance, and generated-state regressions.
6. **#1856 Roadmap and #1 Governance Refresh after Curation Cockpit v1** — update
   roadmap/#1 context only after the above are complete, preserving #1 and #23 as
   open anchors.

```text
#1851 scope -> #1852 -> #1853 -> #1854 -> #1855 -> #1856
```

Closure for every follow-up requires: latest `origin/main`, issue-specific
verification, no direct trusted write from generation or browser/Studio, no
parallel engine, fixture-scoped generated artifacts only, conservative public
wording, final evidence for the implemented surface, and confirmation that #1
and #23 remain open.

## Explicit non-goals

This contract does not authorize:

- direct trusted writes from generation or browser/Studio;
- autonomous apply, self-approval, reviewer bypass, release automation, or hidden
  trusted mutation;
- a new engine, resolver, runtime, language, or hosted/cloud/mobile Layer-3
  surface;
- automated quality, fun, production-readiness, market-demand, or Godot-parity
  claims;
- Steam account creation, code signing, release-button behavior, wishlists, user
  acquisition, discoverability, or market validation;
- committing generated runs/assets/builds unless fixture-scoped by a later issue;
  or
- modifying or closing #1 or #23.

## Generated-state and public-wording audit

Generated outputs are local proposal/evidence artifacts unless a later issue
names them as fixtures. They must not be committed by default. Public wording must
stay conservative: this milestone may say that candidate generation and curation
provenance are scoped, proposal-only, read-only where browser-facing, and locally
verifiable. It must not say that Ouroforge automatically makes good games,
replaces another engine, produces production-ready content, proves fun, or ships
without human gates.
