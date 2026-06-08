# Human Playtest Harness and Fun-Feel Gate v1 Scope and Design Gate

Human Playtest Harness and Fun-Feel Gate v1 is a scope/design-gate milestone
under #1 Era J Milestone 58. It gives Ouroforge an honest, first-class answer to
"is it fun" by making the fun/feel verdict permanently human-in-the-loop. Fun is
emergent and cannot be reduced to an automated score; the engine may collect
structured playtest evidence and enforce a gate, but it does not judge fun.

This issue adds the design decision and contract only. It does not add executable
harness behavior, a new evaluator implementation, a browser-owned writer, a
release button, or any automated fun metric. Issues #1 and #23 remain open
governance anchors.

## Gate outcome: GO (bounded)

The decision for Human Playtest Harness and Fun-Feel Gate v1 is **GO**, bounded
to the contract and follow-up sequence below. Everything else remains **DEFER**
by default.

GO is justified only because the harness and gate reuse existing Ouroforge
surfaces:

- the card-roguelite substrate for deckbuilder/card/tuning titles as
  configuration over one deterministic substrate, not a parallel engine;
- the deterministic runtime and `window.__OUROFORGE__` probe for local playtest
  sessions and read-only browser evidence;
- evaluator gate aggregation for fail-closed release-readiness preconditions;
- evolve/campaign, compare, provenance-bundle, asset, dashboard, cockpit, and CLI
  surfaces for evidence capture, comparison, provenance, and inspection; and
- the existing review/apply/trust-gradient path for any later trusted write.

The gate is **human-owned**: Rust/local may validate that a human verdict exists,
that its refs are fresh, and that release-readiness is blocked without it. Rust
must not compute an automated fun score. Browser/Studio surfaces may display
playtest evidence and verdict state read-only; they must not make trusted release
or source mutations.

## Permanent human fun verdict

The fun/feel verdict has one stable rule: **a title cannot be release-ready
without a human verdict**.

The verdict may be `approved`, `rejected`, `deferred`, or `needs-rework`, with a
human rationale and evidence refs. A missing, malformed, stale, or non-human
verdict blocks release-readiness. A metric may inform the human, but it cannot
stand in for the human. The wording "fun-feel gate passed" means only that a
human recorded an approving verdict against the scoped evidence; it does not mean
Ouroforge proved fun objectively.

Automated signals are explicitly advisory:

- first-session completion or abandonment;
- "one more run?" intent;
- replay count or rerun request;
- retention proxy within a local test window;
- frustration/confusion markers;
- qualitative feedback tags; and
- mechanical/balance/evaluator status from existing gates.

These signals may be summarized for a reviewer. They must not be combined into a
hidden score that claims to decide fun, quality, market demand, production
readiness, or shippability.

## Structured human-playtest capture contract

A structured playtest capture records what happened during a bounded local human
session so the human verdict can be audited:

1. **Session identity.** Local project/run id, title/config id, candidate/version
   refs, playtest build or scenario refs, timestamp, and actor metadata.
2. **First-session signals.** Start/end reason, session duration bucket, number
   of runs or attempts, "one more run?" response, local retention proxy, and any
   replay/continue decision.
3. **Feedback.** Human notes, friction/confusion tags, liked/disliked moments,
   severity, and optional suggested follow-up.
4. **Evidence refs.** Runtime/probe/evaluator/provenance refs, balance evidence,
   screenshots or logs where fixture-scoped, and links to any candidate set or
   review state being judged.
5. **Boundary.** The capture is evidence only. It does not apply code, promote
   content, publish a build, or grant release authority by itself.

The capture schema must fail closed for missing actor/session identity, unsafe
refs, stale project/run binding, unsupported verdict status, and any field that
claims automated fun judgment.

## Fun-feel evaluation gate contract

The fun-feel gate is a release-readiness precondition layered on top of existing
mechanical and evidence gates:

- **Human sign-off required.** Release-readiness is blocked until a human verdict
  explicitly approves the scoped title/candidate/version.
- **Evidence-bound.** The verdict must cite the playtest capture and any relevant
  runtime/evaluator/provenance refs it judges.
- **Freshness-bound.** A verdict for an older candidate, build, substrate config,
  or playtest evidence cannot silently approve a newer one.
- **Fail-closed.** Missing, malformed, stale, rejected, deferred, or
  needs-rework verdicts block release-readiness.
- **No substitution.** Automated metrics, retention proxies, balance scores,
  solver results, or playtest summaries may support the human decision, but they
  never replace it.

The gate may output a machine-readable readiness state such as `blocked`,
`approved-by-human`, or `needs-human-review`. That state is evidence about the
presence and validity of a human verdict; it is not an automated fun score.

## Browser/Studio/read-only boundary

Browser, dashboard, and Studio surfaces may inspect playtest captures, display
human verdict status, show advisory first-session signals, and link to local
evidence. They must remain read-only unless a later issue explicitly scopes a
Rust/local command that records proposal evidence. They must not execute commands,
write trusted source/project state, approve release-readiness, apply candidates,
publish builds, or hide the human gate.

## Language boundary

- **Rust** owns trusted validation, persistence-facing schemas, playtest capture
  validation, fun-feel gate evaluation, substrate/scoring/balance/export/
  provenance logic, evidence writing, run/project binding, review/apply/trust-
  gradient behavior, and CLI behavior.
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

The follow-up issues must stay in this order and must reuse existing substrate,
runtime, evaluator, provenance, dashboard/cockpit, CLI, and review/apply
surfaces:

1. **#1857 Scope and Design Gate** — this document.
2. **#1858 Structured Human-Playtest Capture v1** — define and validate bounded
   local human session capture with advisory first-session signals.
3. **#1859 Fun-Feel Evaluation Gate v1** — enforce human verdict presence and
   freshness as a release-readiness blocker.
4. **#1860 Playtest and Fun-Feel Gate Demo v1** — fixture-scoped deterministic
   demo of playtest capture and gate behavior without trusted browser writes.
5. **#1861 Scenario Coverage v52: Playtest and Fun-Feel Gate Regression Suite**
   — lock human-only verdict, freshness, read-only, generated-state, and
   release-readiness blocking regressions.
6. **#1862 Roadmap and #1 Governance Refresh after Playtest and Fun-Feel Gate
   v1** — update roadmap/#1 context only after the above are complete, preserving
   #1 and #23 as open anchors.

```text
#1857 scope -> #1858 -> #1859 -> #1860 -> #1861 -> #1862
```

Closure for every follow-up requires latest `origin/main`, issue-specific
verification, no automated fun score, no parallel engine, Rust/local trusted
ownership, browser/Studio read-only behavior, fixture-scoped generated artifacts
only, conservative public wording, final evidence for the implemented surface,
and confirmation that #1 and #23 remain open.

## Explicit non-goals

This design gate does not authorize:

- an automated fun score or automated fun verdict;
- release-readiness without human fun/feel sign-off;
- direct trusted writes from generation or browser/Studio;
- autonomous apply, self-approval, reviewer bypass, release automation, or hidden
  trusted mutation;
- a new engine, resolver, runtime, language, or hosted/cloud/mobile Layer-3
  surface;
- production-readiness, market-demand, quality, fun, shippability, or Godot-parity
  claims;
- Steam account creation, code signing, release-button behavior, wishlists, user
  acquisition, discoverability, or market validation;
- committing generated runs/assets/builds unless fixture-scoped by a later issue;
  or
- modifying or closing #1 or #23.

## Generated-state and wording audit

Playtest captures, screenshots, logs, local builds, generated candidates, and
other run artifacts stay untracked unless a later issue names them as fixtures.
Public wording may say that Ouroforge records human playtest evidence and blocks
release-readiness without human fun/feel sign-off. It must not say that the
engine proves a title is fun, automatically makes good games, replaces another
engine, validates market demand, or ships without human gates.
