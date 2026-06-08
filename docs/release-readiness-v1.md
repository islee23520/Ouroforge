# Balance Tuning Co-Pilot and Release Readiness Go/No-Go Surface v1 Scope and Contract

Balance Tuning Co-Pilot and Release Readiness Go/No-Go Surface v1 is a
scope/contract milestone under #1 Era J Milestone 60. It bridges automated
findings to human decisions and consolidates release-readiness evidence into one
read-only sign-off surface. Automated systems may surface findings and assemble
bundles; the human go/no-go decision is required and recorded.

This issue adds no executable behavior. It defines contracts that extend the
Milestone 25 provenance bundle, Milestone 44 release trust/rollback/autonomy
surfaces, and Milestone 50 engine-builder balance findings. It does not add a
parallel engine, release automation, browser/Studio trusted writes, or an
automated production/fun/quality verdict. Issues #1 and #23 remain open
governance anchors.

## Gate outcome: GO (bounded)

The decision for Release Readiness v1 is **GO**, bounded to the contract and
follow-up sequence below. Any capability not explicitly listed here remains
**DEFER** by default.

GO is justified only because this milestone reuses existing Ouroforge surfaces:

- Milestone 25 provenance-bundle and replay surfaces for auditable evidence
  lineage;
- Milestone 44 release trust/provenance, rollback, kill-switch, and release
  auto-apply guardrails as evidence about controlled local changes, not release
  authority;
- Milestone 50 engine-builder balance findings and balance cockpit/rerun surfaces
  for descriptive balance issues;
- four-gate evaluator evidence, solver/over-solution, synthetic-player balance,
  curation, fun-feel, compliance, export, asset, dashboard, cockpit, compare, and
  CLI surfaces for local readiness evidence;
- the card-roguelite substrate for deckbuilder variants as configuration over one
  deterministic substrate rather than a parallel engine; and
- the existing review/apply/trust-gradient path for every trusted write.

This GO authorizes only a human-in-the-loop co-pilot contract and read-only
release-readiness sign-off contract. It does not authorize unattended release,
Steam account actions, signing, upload, public deployment, hosted/cloud/mobile
Layer-3 behavior, market-demand validation, or any claim that a title is good,
fun, shippable, production-ready, or a Godot replacement.

## Balance tuning co-pilot contract

The balance co-pilot turns Milestone 50 findings into human-reviewed
recommendations:

1. **Input findings.** Findings come from existing balance telemetry, dominant
   combo detection, dead-item detection, rerun/diff evidence, and engine-builder
   balance reports. Findings are descriptive evidence, not edit authority.
2. **Recommendation shape.** A recommendation records finding refs, affected
   cards/tuning/config fields, proposed knob changes, expected impact, risk,
   required re-verification, and rollback/revert evidence expectations.
3. **Human approval.** A human may approve, tweak, reject, defer, or request more
   evidence. No recommendation is trusted or applied without human approval and
   the existing review/apply/trust-gradient path.
4. **Re-verify.** After any approved change is applied through the trusted path,
   the same balance/evaluator/compare surfaces re-run and record before/after
   evidence. A stale or missing re-verification blocks readiness.
5. **No auto-nerf.** The co-pilot never auto-applies, auto-merges, self-approves,
   bypasses review, or claims the resulting game is balanced in a subjective or
   production sense.

The co-pilot is a bridge from evidence to human decision. It does not become the
decision-maker.

## Consolidated release-readiness bundle contract

A release-readiness bundle is a local, auditable evidence bundle that gathers the
latest scoped verdicts for a candidate title/config/build:

- four-gate evaluator verdicts and required scenario coverage;
- solver, over-solution, design integrity, and content curation evidence where
  applicable;
- Milestone 50 balance findings, human-approved/tweaked recommendation status,
  and post-change re-verification;
- fun-feel human verdict from the playtest gate;
- narrative/theme human-selection status where narrative assist is in scope;
- asset/license/provenance/compliance evidence;
- export/build package readiness for local desktop/Steam-export consideration,
  without Steam account/signing/upload authority;
- release trust/provenance, rollback, kill-switch, and audit evidence from
  Milestone 44 where source changes are involved; and
- explicit generated-state, stale-ref, and governance audits.

The bundle may classify readiness as `go`, `no-go`, `needs-human-review`, or
`blocked`, but `go` requires a human go/no-go sign-off record in addition to all
machine-checkable evidence. A missing, stale, malformed, rejected, deferred, or
non-human sign-off fails closed.

## Read-only go/no-go sign-off surface

The go/no-go surface is a read-only dashboard/cockpit view over the release-
readiness bundle plus a human sign-off record:

- It may display current bundle status, failing dimensions, stale refs, balance
  recommendations, fun-feel verdict, compliance/export status, provenance links,
  and copyable local CLI commands.
- It may show the human go/no-go decision and rationale after Rust/local evidence
  records it through a scoped follow-up issue.
- It must not execute commands, apply fixes, publish builds, upload to Steam,
  sign binaries, create accounts, mutate source/project state, approve its own
  output, or hide missing human gates.
- Browser/Studio/dashboard surfaces stay read-only. Trusted validation,
  persistence, evidence writing, and sign-off validation remain Rust/local owned.

Human sign-off is not a production guarantee. It records that a scoped human
reviewer made a go/no-go decision against the visible local evidence bundle.

## Language boundary

- **Rust** owns trusted validation, persistence-facing schemas, release-readiness
  bundle validation, balance recommendation validation, human sign-off validation,
  substrate/scoring/balance/export/provenance logic, evidence writing,
  run/project binding, review/apply/trust-gradient behavior, and CLI behavior.
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

The follow-up issues must stay in this order and must reuse existing Milestone
25/44/50, substrate, runtime, evaluator, provenance, dashboard/cockpit, CLI, and
review/apply surfaces:

1. **#1869 Scope and Contract** — this document.
2. **#1870 Balance Tuning Co-Pilot v1** — turn Milestone 50 findings into
   human-approved/tweaked recommendations and require re-verification.
3. **#1871 Release-Readiness Bundle and Go/No-Go Surface v1** — consolidate the
   readiness bundle and read-only human sign-off surface.
4. **#1872 Release Readiness Demo v1** — fixture-scoped deterministic demo of the
   co-pilot-to-readiness path without release automation or trusted browser
   writes.
5. **#1873 Scenario Coverage v54: Release Readiness Regression Suite** — lock
   human go/no-go, stale-ref, balance re-verification, read-only display,
   generated-state, and release-authority-blocking regressions.
6. **#1874 Roadmap and #1 Governance Refresh after Release Readiness v1** —
   update roadmap/#1 context only after the above are complete, preserving #1 and
   #23 as open anchors.

```text
#1869 scope -> #1870 -> #1871 -> #1872 -> #1873 -> #1874
```

Closure for every follow-up requires latest `origin/main`, issue-specific
verification, no auto-applied balance changes, no release automation, no parallel
engine, Rust/local trusted ownership, browser/Studio read-only behavior,
fixture-scoped generated artifacts only, conservative public wording, final
evidence for the implemented surface, and confirmation that #1 and #23 remain
open.

## Explicit non-goals

This contract does not authorize:

- balance recommendation auto-apply, auto-merge, self-approval, reviewer bypass,
  or hidden trusted mutation;
- release go/no-go without a human sign-off record;
- direct trusted writes from generation or browser/Studio;
- a new engine, resolver, runtime, language, or hosted/cloud/mobile Layer-3
  surface;
- Steam account creation, code signing, upload, public release, release-button
  behavior, wishlists, user acquisition, discoverability, market validation, or
  any other Ring-3 market/release operation;
- automated fun, quality, production-readiness, market-demand, shippability,
  balance-guarantee, or Godot-parity claims;
- committing generated runs/assets/builds outside fixture-scoped evidence; or
- modifying or closing #1 or #23.

## Generated-state and wording audit

Release-readiness bundles, balance recommendation artifacts, generated runs,
exports, screenshots, logs, local builds, and candidate outputs stay untracked
unless a later issue names them as fixtures. Public wording may say that
Ouroforge can assemble local evidence for human release go/no-go review. It must
not say that the engine ships autonomously, proves production readiness, proves
fun, validates market demand, replaces another engine, or bypasses human gates.
