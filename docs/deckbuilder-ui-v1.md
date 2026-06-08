# Deckbuilder UI Kit v1 Scope and Contract

Status: **Design gate — scope and contracts only; no executable behavior**

Issue: #1825 — Deckbuilder UI Kit v1 Scope and Contract
Anchor: #1 Era I Milestone 52 (Deckbuilder UI surface)

This document is the canonical Deckbuilder UI Kit v1 design artifact. It defines
the genre-specific UI surface for a deckbuilder variant: card/hand/pipeline UI,
shop and run-map UI, number-cascade/score display, tooltips, dependency order,
and closure gates. It adds no executable behavior, UI framework, runtime feature,
fixture, browser authority, or engine capability.

Deckbuilder UI is presentation and inspection over the existing deterministic
runtime, probe, and card-roguelite substrate. The UI may display state and draft
intent, but trusted writes continue to route through the existing Rust/local
review/apply/trust-gradient path. Browser and Studio surfaces remain read-only or
draft-only.

## Existing surfaces this milestone reuses

| Concern | Reused surface |
| --- | --- |
| Runtime state | Existing JavaScript runtime, card-roguelite substrate, seeded replay, and snapshot/probe state. |
| Read-only inspection | `window.__OUROFORGE__`, the in-game JS runtime UI, static dashboard, and authoring cockpit display patterns. |
| Draft/trust boundary | Existing source-apply, mutation review, trust-gradient, provenance, and generated-evidence surfaces. |
| Verification | Existing scenario/evaluator gates, replay digest checks, compare output, and JS smoke tests. |
| Generated state | Existing ignored generated roots for run evidence, dashboard exports, and temporary UI artifacts. |

No follow-up may add a new UI framework, browser command bridge, trusted browser
write path, or parallel engine. The deckbuilder variant remains configuration and
deterministic state over the existing card-roguelite substrate.

## Scope

The contract applies to:

- card, hand, discard, draw, and play-pipeline UI over runtime/probe state;
- shop and run-map UI as read-only or draft-only surfaces;
- number-cascade and score display with tooltips;
- ownership, generated-state, and trust-boundary rules for deckbuilder UI;
- dependency order and closure gates for the follow-up issues.

The contract governs claims and boundaries. Follow-up issues own concrete
implementation and tests.

## Non-goals

Deckbuilder UI Kit v1 does not authorize:

- executable behavior in this scope issue;
- a new UI framework, renderer, engine, runtime, or command bridge;
- direct trusted writes from generation or any browser/Studio surface;
- autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden trusted
  writes;
- browser/Studio promotion of generated content or source changes;
- automated fun, quality, taste, production, market, or shippability claims;
- Godot replacement/parity claims;
- hosted/cloud/mobile Layer-3 capability, Steam account/signing/release actions,
  or market-demand work;
- committed generated runs, dashboard exports, UI captures, or local artifacts
  unless explicitly fixture-scoped by a follow-up issue;
- closing, modifying, replacing, or narrowing #1 or #23.

## Card, hand, and pipeline UI contract

Card/hand/pipeline UI exposes the current deckbuilder run state without becoming
a second source of truth.

- Cards display id, name/key, cost, rules text key or localized display text,
  status, and deterministic runtime metadata supplied by the existing substrate.
- Hand, draw pile, discard pile, exhaust/removed piles where scoped, and selected
  card state are rendered from probe-visible runtime state. UI order must be
  deterministic and replay-stable for identical inputs.
- The play pipeline displays declared phases such as selectable, queued,
  resolved, blocked, and discarded. Blocked or illegal actions must show bounded
  diagnostics rather than silently disappearing.
- UI interactions are either read-only inspection or draft-only intent capture.
  They do not mutate trusted state directly and do not bypass the existing apply
  path.
- Scenario coverage must distinguish valid card play, blocked card play, stale
  probe state, missing card metadata, and pipeline completion.

## Shop and run-map UI contract

Shop and run-map UI are read-only/draft-only navigation and choice surfaces over
existing run state.

- Shop UI displays offered cards/relics/items, prices or requirements, selected
  draft choices, unavailable choices, and deterministic refresh/seed metadata
  where scoped.
- Run-map UI displays nodes, edges, current position, known upcoming choices,
  completed/blocked nodes, and terminal state. It must not fabricate unexplored
  state beyond what the runtime declares.
- Draft-only actions may record proposed selections or planned paths for review,
  but trusted acceptance/persistence remains Rust/local and review-gated.
- Browser/Studio surfaces must escape displayed data, remain local/read-only, and
  never execute commands, fetch remote content, install dependencies, or write
  trusted files.
- Negative cases include invalid node ids, stale offers, undeclared prices,
  impossible paths, and malformed run-map state.

## Number-cascade, score display, and tooltip contract

Number-cascade UI explains payoff math while preserving authoritative scoring in
the runtime.

- Score displays show current total, per-event increments, combo/multiplier
  metadata, source/cause, and final cascade total from runtime-owned state.
- Number cascades are presentation of score events, not score recomputation. Any
  displayed total must reconcile with authoritative runtime state or fail
  mechanical verification.
- Tooltips are deterministic explanations for card effects, status, scoring
  causes, shop/run-map choices, and blocked actions. They must reference bounded
  state and avoid hidden trusted mutation.
- Tooltip text may use externalized string keys when localization follow-up work
  exists; this issue does not authorize a localization engine.
- Scenario coverage must include valid cascades, mismatched totals, missing cause
  metadata, tooltip escaping, and stale score display.


## Card/Hand/Pipeline UI implementation compatibility (#1826)

Issue: #1826 is the follow-up implementation issue for Card/Hand/Pipeline UI v1.
The scope contract above remains the #1825 authority, while #1826 may add the
mechanically verified card/hand/pipeline read model over the same runtime/probe
surface. For that compatibility contract: generated runs/artifacts remain untracked unless explicitly fixture-scoped; browser/Studio surfaces remain read-only and draft-only; issues #1 and #23 remain open; no production-ready,
Godot-replacement/parity, quality, fun, public-launch, or market-demand claim is
authorized.


## Shop and Run-Map UI implementation compatibility (#1827)

Issue: #1827 is the follow-up implementation issue for read-only/draft-only shop and run-map UI. It extends the #1826 runtime UI module and `window.__OUROFORGE__` probe model with fixture-scoped shop offers, unavailable choices, run-map nodes, edges, current position, and draft path planning. Shop offer selection and run-map path planning are local UI proposals only: they set `trustedWrite: false`, remain draft-only, and route any trusted acceptance through the existing Rust/local review/apply/trust-gradient path.

The implementation reuses the existing JavaScript runtime UI surface and adds no new UI framework, renderer, command bridge, browser trusted-write path, or parallel engine. Negative cases remain visible and fail closed: invalid node ids, stale offers, undeclared/unavailable prices, impossible paths, blocked edges, and unknown run-map state must be rendered or reported as bounded UI state rather than silently promoted. Generated runs/artifacts remain untracked unless explicitly fixture-scoped; browser/Studio surfaces remain read-only; issues #1 and #23 remain open; no production-ready, Godot-replacement/parity, quality, fun, public-launch, or market-demand claim is authorized.

## Generated state policy

UI screenshots, probe exports, dashboard/cockpit exports, run-map captures,
number-cascade traces, tooltip audits, and temporary browser state are generated
local artifacts. They remain untracked unless a follow-up issue explicitly scopes
a tiny deterministic fixture as source-like regression data.

## Rust-trusted / browser-read-only boundary

Rust/local owns trusted validation, persistence, evidence writing, provenance,
run/project binding, source apply, and the review/apply/trust-gradient path.
TypeScript/JavaScript owns deterministic runtime UI, deckbuilder in-game display,
the `window.__OUROFORGE__` probe, and browser-local read-only inspection. Browser
and Studio surfaces may display deckbuilder UI state, draft intent, and evidence
only as escaped/read-only data; they must not apply changes, write trusted files,
execute commands, promote generated content, or bypass review.

## Dependency order and closure gates

Follow-up Deckbuilder UI Kit v1 issues are implemented in this order:

1. **Scope and Contract** (this issue, #1825) — define the contracts,
   boundaries, reuse statement, and dependency order; no executable behavior.
2. **Card/Hand/Pipeline UI v1** (#1826) — implement card, hand, pile, and play
   pipeline display over the existing runtime/probe.
3. **Shop and Run-Map UI v1** (#1827) — implement read-only/draft-only shop and
   run-map display over declared run state.
4. **Number-Cascade and Score Display v1** (#1828) — implement score display,
   number-cascade explanation, and tooltip behavior.
5. **Deckbuilder UI Demo v1** (#1829) — demonstrate the UI with local generated
   evidence and conservative claims.
6. **Scenario Coverage v47: Deckbuilder UI Regression Suite** (#1830) — add
   regression coverage for card UI, shop/run-map UI, score display, tooltips, and
   negative cases.
7. **Roadmap and #1 Governance Refresh after Deckbuilder UI v1** (#1831) — record
   the milestone outcome while keeping #1 and #23 open.

```text
#1825 scope -> #1826 -> #1827 -> #1828 -> #1829 -> #1830 -> #1831
```

Closure gates for each follow-up issue:

- the prior issue in the dependency order is merged or explicitly superseded by
  a maintainer-approved governance decision;
- independently verifiable behavior is not combined across card UI, shop/run-map
  UI, score display, demo, coverage, and governance steps;
- the implementation reuses existing runtime/probe/UI/evaluator/evolve/compare/
  provenance/dashboard/cockpit/CLI surfaces and does not add a parallel runtime
  or UI framework;
- browser/Studio surfaces stay read-only or draft-only, with trusted writes only
  through the existing Rust/local review/apply/trust-gradient path;
- generated-state and trust-boundary audits pass;
- public wording stays conservative: no auto-merge, quality, fun, production, or
  Godot-replacement claim;
- #1 and #23 are reconfirmed open in final evidence and are not closed or
  narrowed by this milestone.

## Governance audit

As of this contract, #1 remains the open roadmap anchor and #23 remains the open
governance/constraint anchor. Deckbuilder UI Kit v1 adds a bounded Era I
presentation layer over the existing substrate while preserving those anchors,
read-only browser/Studio constraints, and the existing trust model.
