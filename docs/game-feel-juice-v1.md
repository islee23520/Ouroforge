# Game-Feel and Juice Toolkit v1 Scope and Contract

Status: **Design gate — scope and contracts only; no executable behavior**

Issue: #1818 — Game-Feel and Juice Toolkit v1 Scope and Contract
Anchor: #1 Era I Milestone 51 (Game feel and juice)

This document is the canonical Game-Feel and Juice Toolkit v1 design artifact. It
defines the runtime feedback layer that makes score cascades legible and
responsive: juice primitives, score-cascade payoff feedback, a sub-100ms
responsiveness check, dependency order, and closure gates. It adds no executable
behavior, schemas, runtime features, fixtures, browser authority, or engine
capability.

The toolkit is a mechanical feedback contract, not an automated taste judgment.
The runtime may prove that tweens, shake, hit-stop, SFX hooks, score-cascade
feedback, and response timing obey deterministic budgets. Whether the resulting
feel is satisfying remains a human Era J gate.

## Existing surfaces this milestone reuses

Game-Feel and Juice Toolkit v1 is defined on top of existing Ouroforge surfaces;
follow-up issues must extend these before adding anything parallel.

| Concern | Reused surface |
| --- | --- |
| Deterministic play loop | The existing JavaScript runtime in `examples/game-runtime/runtime.js` and its snapshot/replay/probe contracts. |
| Runtime observation | The read-only `window.__OUROFORGE__` probe and browser-local inspection surfaces. |
| Scenario verification | Existing scenario/evaluator gates, replay digests, compare output, and generated evidence conventions. |
| Source/trust boundary | Existing Rust/local review/apply/trust-gradient path for trusted writes and proposal promotion. |
| Provenance and generated state | Existing provenance-bundle, compare, dashboard, cockpit, and ignored generated-output policies. |

No follow-up may introduce a new game loop, renderer, browser write path, or
parallel engine to support juice. A deckbuilder variant remains configuration and
deterministic state over the card-roguelite substrate.

## Scope

The contract applies to:

- runtime juice primitives: easing/tween, shake, hit-stop, and SFX hooks;
- score-cascade payoff feedback for chained scoring events;
- the sub-100ms responsiveness verification budget;
- read-only browser/Studio display of feedback evidence;
- dependency order and closure gates for the follow-up issues.

The contract governs claims, ownership, evidence shape, and trust boundaries.
Concrete code changes must be scoped by the follow-up issues and verified with
mechanical tests.

## Non-goals

Game-Feel and Juice Toolkit v1 does not authorize:

- executable behavior in this scope issue;
- a new runtime, renderer, engine, mixer, framework, or browser command bridge;
- direct trusted writes from generation or any browser/Studio surface;
- autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden trusted
  writes;
- an automated fun, quality, taste, or shippability score;
- production-ready, Godot replacement/parity, or market-success claims;
- hosted/cloud/mobile Layer-3 capability, Steam account/signing/release actions,
  or market-demand work;
- committed generated runs, exports, audio, dashboard state, or local artifacts
  unless explicitly fixture-scoped by a follow-up issue;
- closing, modifying, replacing, or narrowing #1 or #23.

## Juice primitive contract

Juice primitives are deterministic feedback intents emitted by the existing
runtime and exposed read-only for verification. They enrich player feedback but
must not become a second source of game truth.

### Easing and tween

- Easing/tween effects describe visual interpolation for existing runtime state
  or UI elements. They do not alter authoritative score, card, run, or encounter
  state.
- Tween definitions must be deterministic from declared inputs: target, start
  tick or frame, duration, easing curve id, and value range.
- Scenario verification checks that tween lifecycle, timing, and final values are
  mechanically correct. It does not assert that an easing curve “feels good.”

### Shake

- Shake is a bounded visual feedback intent attached to a declared event or UI
  target.
- Shake must carry deterministic amplitude, duration, decay, and target metadata.
  It must not affect collision, score, card order, or other authoritative state.
- The probe exposes active shake intents read-only so tests can assert start,
  decay, and completion without granting browser authority.

### Hit-stop

- Hit-stop is a bounded pause/slowdown feedback intent for impact emphasis.
- Hit-stop must preserve deterministic simulation semantics. Follow-up work must
  state whether the pause applies only to presentation or to a bounded runtime
  tick gate, and replay digest behavior must remain stable.
- Hit-stop duration contributes to responsiveness budgeting; it may emphasize a
  payoff but may not hide delayed input handling beyond the sub-100ms budget.

### SFX hooks

- SFX hooks are runtime audio-intent metadata connected to existing audio-intent
  surfaces. They do not add a new audio engine or trusted browser write path.
- SFX hooks identify event, cue id, timing, and optional intensity. Actual audio
  quality and direction remain human-controlled and, where generated, subject to
  the existing proposal/provenance/QA path.
- Missing or invalid SFX hook metadata fails closed in mechanical verification;
  it must not silently promote an unproven asset.

## Score-cascade payoff feedback contract

Score-cascade payoff feedback makes chained scoring understandable and
rewarding while preserving the underlying deterministic score model.

- A score cascade is a bounded sequence of score events whose cause, increment,
  multiplier or combo metadata, cumulative total, and completion state are
  exposed read-only.
- Payoff feedback may layer tweens, shake, hit-stop, and SFX hooks over each
  cascade step, but the feedback layer never recomputes authoritative score.
- Cascade feedback must be reconstructible from the same declared runtime state
  that produced the score events. Replay and snapshot/restore must produce the
  same cascade feedback sequence for identical inputs.
- Negative and blocked cases must be visible: invalid cascade ids, impossible
  totals, missing cause metadata, and out-of-order completion fail mechanical
  verification instead of falling back to untracked UI behavior.
- Browser/Studio surfaces may inspect cascade feedback and evidence read-only;
  trusted fixes route through Rust/local validation and the review/apply/trust-
  gradient path.

## Sub-100ms responsiveness verification contract

Responsiveness is a mechanical budget. Follow-up issues must prove the runtime
responds to input-to-feedback within less than 100ms for scoped interactions.

- The measured interval is from accepted player input or runtime event to the
  first observable feedback intent/probe state that acknowledges the action.
- The budget is **<100ms**. Evidence must record the measured value, event id,
  feedback id, runtime version, and pass/fail status.
- The check must be deterministic and local. It may use simulated ticks or a
  controlled clock; it must not depend on network timing, wall-clock flakiness,
  browser paint timing alone, or a human stopwatch.
- Hit-stop and cascade staging cannot mask late acknowledgement. A later payoff
  animation may continue after the acknowledgement, but the first response must
  satisfy the budget.
- The check verifies responsiveness only. It never claims that the interaction is
  fun, polished, or shippable.

## Generated state policy

Juice evidence, cascade traces, responsiveness reports, dashboard exports,
runtime snapshots, and temporary feedback artifacts are generated/local state.
They remain under ignored generated roots unless a follow-up issue explicitly
scopes a tiny deterministic fixture as tracked source-like data. Fixture-scoped
artifacts must be deterministic, minimal, license-clear, and justified by the
issue.

## Rust-trusted / browser-read-only boundary

Rust/local owns trusted validation, persistence, evidence writing, provenance,
run/project binding, source apply, and the review/apply/trust-gradient path.
TypeScript/JavaScript owns deterministic runtime feedback, the
`window.__OUROFORGE__` probe, and browser-local read-only inspection. Browser and
Studio surfaces may display feedback state, cascade traces, and responsiveness
evidence only as escaped/read-only data. They must not write trusted files, apply
source changes, execute commands, promote generated assets, or bypass review.

## Dependency order and closure gates

Follow-up Game-Feel and Juice Toolkit v1 issues are implemented in this order:

1. **Scope and Contract** (this issue, #1818) — define the contracts,
   boundaries, reuse statement, and dependency order; no executable behavior.
2. **Juice Primitives v1** (#1819) — implement mechanically verifiable easing,
   tween, shake, hit-stop, and SFX hook feedback on the existing runtime.
3. **Score-Cascade Payoff Feedback v1** (#1820) — implement cascade feedback and
   probe-visible payoff traces without recomputing authoritative score.
4. **Sub-100ms Responsiveness Verification v1** (#1821) — add the deterministic
   responsiveness budget check and evidence.
5. **Game-Feel and Juice Demo v1** (#1822) — demonstrate the toolkit with local,
   generated evidence and conservative claims.
6. **Scenario Coverage v46: Game-Feel and Juice Regression Suite** (#1823) — add
   regression coverage for primitives, cascades, responsiveness, and negative
   cases.
7. **Roadmap and #1 Governance Refresh after Game-Feel and Juice v1** (#1824) —
   record the milestone outcome while keeping #1 and #23 open.

```text
#1818 scope -> #1819 -> #1820 -> #1821 -> #1822 -> #1823 -> #1824
```

Closure gates for each follow-up issue:

- the prior issue in the dependency order is merged or explicitly superseded by
  a maintainer-approved governance decision;
- independently verifiable behavior is not combined across primitives, cascade,
  responsiveness, demo, coverage, and governance steps;
- the implementation reuses the existing runtime/probe/evaluator/evolve/compare/
  provenance/dashboard/cockpit/CLI surfaces and does not add a parallel runtime;
- juice is verified mechanically; the feel/fun judgment remains a human Era J
  decision;
- the sub-100ms responsiveness budget is enforced where scoped;
- generated-state and trust-boundary audits pass;
- public wording stays conservative: no auto-merge, quality, fun, production, or
  Godot-replacement claim;
- #1 and #23 are reconfirmed open in final evidence and are not closed or
  narrowed by this milestone.

## Governance audit

As of this contract, #1 remains the open roadmap anchor and #23 remains the open
governance/constraint anchor. Game-Feel and Juice Toolkit v1 adds a bounded Era I
mechanical feedback layer while preserving those anchors and the existing trust
model.
