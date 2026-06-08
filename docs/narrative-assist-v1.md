# Narrative and Theme-Arc Authoring Assist v1 Scope and Contract

Narrative and Theme-Arc Authoring Assist v1 is a scope/contract milestone under
#1 Era J Milestone 59. It assists the "soul" of a title by generating candidate
narrative, theme-wit, and flavor material for human selection. Tone, soul,
meaning, and final integration remain human decisions.

This issue adds no executable behavior. It defines the contract for candidate
narrative/theme material extending the Milestone 39 Long-Form Game Systems
narrative/dialogue/event surface (`docs/long-form-systems-v1.md`) and the
human-curated integration path. It does not add a new engine, a new runtime, a
new writer, or any automated tone/quality/fun verdict. Issues #1 and #23 remain
open governance anchors.

## Gate outcome: GO (bounded)

The decision for Narrative and Theme-Arc Authoring Assist v1 is **GO**, bounded
to the contract and follow-up sequence below. Any capability not explicitly
listed here remains **DEFER** by default.

GO is justified only because this milestone reuses existing Ouroforge surfaces:

- Milestone 39 Long-Form Game Systems narrative/dialogue/event contracts for
  optional narrative state, events, and dialogue shape;
- the card-roguelite substrate for deckbuilder/theme variants as configuration
  over one deterministic substrate rather than a parallel engine;
- Milestone 30 proposal-generation intake where narrative/flavor requests enter
  as proposal data;
- runtime, evaluator, evolve/campaign, compare, provenance-bundle, asset,
  dashboard, cockpit, and CLI surfaces for local evidence and read-only
  inspection; and
- the existing review/apply/trust-gradient path for every later trusted write.

This GO authorizes proposal-only candidate generation and human-curated
integration contracts. It does not authorize browser/Studio trusted mutation,
autonomous apply, release automation, hosted/cloud/mobile Layer-3 behavior, or an
automated assessment that a title has the right tone, soul, fun, or market fit.

## Narrative/theme candidate generation contract

Narrative assist extends the Milestone 39 narrative system by producing bounded
candidate material, not by replacing the system:

1. **Input.** A human-authored brief declares the target title/config, desired
   phase shift, narrative arc, theme/wit/flavor goals, constraints, and candidate
   count. The brief may cite candidate-generation or curation cockpit evidence,
   but remains human-authored intent.
2. **Candidate classes.** Supported candidate material may include theme-arc
   beats, dialogue variants, event hooks, card or encounter flavor text,
   store-copy drafts, onboarding text, moment-to-moment wit, and tone notes.
3. **Milestone 39 extension.** Candidates map to the existing narrative/dialogue/
   event vocabulary from Long-Form Game Systems. They do not create a separate
   narrative engine, hidden state service, or alternate runtime.
4. **Substrate reuse.** Deckbuilder/card variants express theme and flavor as
   substrate config/copy references over the existing card-roguelite substrate,
   not a forked resolver.
5. **Output.** The output is an untrusted candidate set with stable ids,
   candidate payload hashes or evidence refs, source brief refs, generation path
   and parameters, and compatibility notes for the target narrative/config
   surface.
6. **Trust.** Generation never applies, auto-applies, self-approves, mutates
   source, promotes content, or bypasses review. Candidate material remains a
   proposal until a human selects it and the existing trusted path accepts it.

The engine may help create many possible voices. It must not claim it found the
right voice. "Tone match," "soul," "funny," "moving," and "good" are human
judgments.

## Human-curated selection and integration contract

Human-curated integration is a read-only assist plus explicit human selection:

- A reviewer may inspect candidate ids, payload summaries, source brief refs,
  narrative/dialogue/event compatibility, substrate/config refs, provenance,
  evaluator/read-model status, and diffs against current copy.
- A reviewer may record a human selection decision as provenance: selected,
  rejected, deferred, needs-rework, superseded, or no-selection, with rationale.
- A selected candidate is still not trusted source. It becomes eligible for
  integration only through the existing review/apply/trust-gradient path and any
  issue-specific gate scoped later.
- Browser/Studio/dashboard/cockpit surfaces may display candidate material and
  selection provenance read-only. They must not execute commands, write trusted
  state, integrate copy, approve their own output, or hide human judgment.
- Any future integration command must be Rust/local owned, evidence-bound,
  review-gated, and scoped by a follow-up issue.

A missing, malformed, stale, or non-human selection record fails closed for any
later integration gate. The system may surface that failure; it must not repair it
with a trusted browser write.

## Tone and soul boundary

Tone/soul is a human decision boundary:

- Automated generation may suggest material.
- Automated checks may validate schema, stale refs, unsafe paths, compatibility,
  and whether required evidence exists.
- Automated summaries may group candidates by declared theme or target moment.
- Automated systems must not assert that a candidate is funny, emotionally right,
  narratively good, fun, production-ready, shippable, or marketable.

The human verdict is allowed to be subjective; the machine-owned part is only the
auditability of the proposal, selection, and integration path.

## Language boundary

- **Rust** owns trusted validation, persistence-facing schemas, narrative/theme
  candidate validation, selection provenance validation, substrate/scoring/
  balance/export/provenance logic, evidence writing, run/project binding,
  review/apply/trust-gradient behavior, and CLI behavior.
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

The follow-up issues must stay in this order and must reuse existing Milestone 39
narrative/dialogue/event, substrate, runtime, evaluator, provenance,
dashboard/cockpit, CLI, and review/apply surfaces:

1. **#1863 Scope and Contract** — this document.
2. **#1864 Narrative/Theme-Arc Candidate Generation v1** — define and validate
   bounded phase-shift narrative/theme-wit/flavor candidate sets extending
   Milestone 39.
3. **#1865 Human-Curated Narrative Integration v1** — record human selection and
   route selected material through the trusted review/apply path.
4. **#1866 Narrative Assist Demo v1** — fixture-scoped deterministic demo of
   narrative candidate inspection and human-curated integration evidence.
5. **#1867 Scenario Coverage v53: Narrative Assist Regression Suite** — lock
   proposal-only generation, human tone/soul selection, read-only display,
   generated-state, and stale-ref regressions.
6. **#1868 Roadmap and #1 Governance Refresh after Narrative Assist v1** — update
   roadmap/#1 context only after the above are complete, preserving #1 and #23 as
   open anchors.

```text
#1863 scope -> #1864 -> #1865 -> #1866 -> #1867 -> #1868
```

Closure for every follow-up requires latest `origin/main`, issue-specific
verification, no new narrative engine, no direct trusted write from generation or
browser/Studio, Rust/local trusted ownership, browser/Studio read-only behavior,
fixture-scoped generated artifacts only, conservative public wording, final
evidence for the implemented surface, and confirmation that #1 and #23 remain
open.

## Explicit non-goals

This contract does not authorize:

- direct trusted writes from generation or browser/Studio;
- autonomous apply, self-approval, reviewer bypass, release automation, or hidden
  trusted mutation;
- a new narrative engine, substrate fork, resolver, runtime, language, or
  hosted/cloud/mobile Layer-3 surface;
- automated tone, soul, quality, fun, production-readiness, market-demand,
  shippability, or Godot-parity claims;
- Steam account creation, code signing, release-button behavior, wishlists, user
  acquisition, discoverability, or market validation;
- committing generated runs/assets/builds/copy outside fixture-scoped evidence;
  or
- modifying or closing #1 or #23.

## Generated-state and wording audit

Generated narrative candidates, copy drafts, playtest notes, screenshots, logs,
local builds, and run artifacts stay untracked unless a later issue names them as
fixtures. Public wording may say that Ouroforge can propose narrative/theme
material for human curation on a local evidence path. It must not say that the
engine supplies soul, proves fun, guarantees good writing, replaces another
engine, validates market demand, or ships without human gates.
