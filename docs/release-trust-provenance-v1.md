# Scaled Trust Gradient, Release Provenance and Compliance v1

Issue: **#1689** (Era H Milestone 44 scope and contract)

Scaled Trust Gradient, Release Provenance and Compliance v1 scales safety and
audit from **per-change** to **per-release** by composition over existing
surfaces. It defines three contracts: a **broadened-but-bounded auto-apply tier**
backed by stronger verification and game-scale rollback/kill-switch (extending
Milestone 22), a **per-release provenance bundle** spanning intent -> content ->
assets -> QA -> release (extending Milestone 25 by composition), and a
**compliance reviewer gate** for content policy, age-rating signals, and asset
license/provenance completeness.

This is a scope and contract document. It **adds no executable behavior**. It
introduces no parallel engine, runtime, writer, or provenance system; every
follow-up reuses existing runtime/probe, evaluator (`declared-gate-and`), visual
gate, `compare`, evolve/campaign, provenance-bundle, asset-manifest,
`source_apply_*`/`trust_gradient_*`, QA-swarm, dashboard, cockpit, and CLI
surfaces. High-risk and source-affecting changes **never auto-apply**, and humans
retain release **go/no-go**.

#1 remains the roadmap/vision anchor and #23 remains the repo memory/design
anchor. This contract preserves both issues as open anchors.

## Contract 1 — Broadened bounded auto-apply tier (extends Milestone 22)

Milestone 22 (Safe Source Mutation Apply, `docs/safe-source-mutation-apply-v1.md`
and `docs/trust-gradient-*`) established a per-change trust gradient where only
low-risk, fully-evidenced, non-source changes may auto-apply through the existing
review/apply path. This contract **broadens the eligible tier** to additional
low-risk change classes **only** when stronger verification and game-scale
rollback/kill-switch controls are present. It does not relax any existing
boundary.

- The broadened tier is a **composition** over `source_apply_*` and
  `trust_gradient_*`; it adds no new apply engine and no new trusted write path.
- Eligibility **fails closed**: a change is auto-apply-eligible only when its
  risk tier is explicitly low, every required verification artifact is present,
  fresh, and mutually consistent, and a game-scale rollback snapshot plus an
  armed kill-switch exist before apply readiness.
- **High-risk and source-affecting changes never auto-apply.** Dependency,
  lockfile, CI/workflow, build-script, install-script, credential/auth/network/
  cloud, and release/export/publish changes remain blocked from auto-apply.
- **Stronger verification** means the broadened tier requires more than the
  per-change minimum: post-apply rerun/comparison evidence and the
  function-specific QA gate (`declared-gate-and`, visual gate, QA swarm as
  applicable) must pass before any success claim.
- **Game-scale rollback/kill-switch** extends the existing rollback snapshot and
  emergency-hold surfaces to release-scale change sets: a single armed control
  can halt the broadened tier and revert the affected set without manual
  per-change recovery.
- Generation, role agents, the producer, and any browser/Studio surface remain
  **proposal-only** through the existing review/apply/trust-gradient path. No
  autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden trusted
  write is introduced.

## Contract 2 — Per-release provenance bundle (extends Milestone 25)

Milestone 25 (End-to-End Provenance, `docs/end-to-end-provenance-v1.md`) defined
a **per-change** provenance bundle composed by reference over intent, artifact,
trusted validation, runtime observation, evaluator verdict, regression
comparison, and review/promotion records. This contract defines a **per-release**
provenance bundle as **composition by reference over per-change bundles** plus
release-level references. It introduces **no new provenance engine**.

A release SHOULD be describable as an ordered, additive bundle spanning:

1. **Intent** — release goal / design brief / campaign refs.
2. **Content** — the set of per-change provenance bundles included in the
   release, by reference.
3. **Assets** — asset-manifest refs with license/provenance completeness for
   every generated asset/audio/content promoted into the release.
4. **QA** — function-specific QA gate verdicts (`declared-gate-and`, visual gate,
   QA-swarm, regression `compare`) covering the release set.
5. **Release** — compliance gate verdict, human go/no-go decision, and
   rollback/kill-switch arming refs.

Per-release bundles are **additive**: releases or workflows without a bundle
remain valid. Rust/local tooling owns bundle composition and validation;
dashboard/Studio render exported bundles **read-only** and execute no apply,
merge, or shell command.

## Contract 3 — Compliance reviewer gate

The compliance reviewer gate is a **fail-closed verification gate** modeled on the
existing reviewer/evaluator gates; it is **not** a new engine and not an
automated taste/quality judgement. A release set passes the gate only when all of
the following are present, fresh, and mutually consistent:

- **Content policy** — declared content-policy signals for the release set are
  present and within the declared bounds; missing or out-of-bounds signals fail
  closed.
- **Age-rating signals** — declared age-rating signals are present and
  consistent with the content set; the gate records signals only and makes **no
  certification claim**.
- **Asset license/provenance completeness** — every generated or promoted
  asset/audio/content carries license and provenance metadata and the
  function-specific QA gate verdict; **no unlicensed, uncredited, or
  unverified-style asset is ever promoted**.

The gate produces a verdict (for example **pass**, **blocked**, with reasons)
that a human consumes; it **does not** auto-promote, auto-merge, or replace the
human release go/no-go. Art/audio/UX/narrative direction and "looks good / sounds
good / is fun" remain human decisions.

## Dependency order and closure gates

Follow-up issues are ordered so later behavior is never implemented before its
controls exist:

| Order | Issue | Purpose |
| --- | --- | --- |
| 1 | #1689 | Scope and contract (this issue). |
| 2 | #1690 | Broadened bounded auto-apply and game-scale rollback v1. |
| 3 | #1691 | Per-release provenance bundle v1. |
| 4 | #1693 | Compliance reviewer gate v1. |
| 5 | #1694 | Scaled trust, release provenance and compliance demo v1. |
| 6 | #1695 | Scenario coverage v41: scaled trust and release provenance regression suite. |
| 7 | #1696 | Roadmap and #1 governance refresh after scaled trust and release provenance v1. |

```text
#1689 scope -> #1690 -> #1691 -> #1693 -> #1694 -> #1695 -> #1696
```

A Milestone 44 issue cannot close until its own scope has targeted regression
tests for the artifact/control it introduces (including blocked/negative and
fail-closed cases), broad repository checks when Rust contracts, dashboard
readers, Studio views, or public wording change, evidence that generated
runs/assets/content/release artifacts stayed untracked except fixture-scoped
roots, and evidence that **#1 and #23 remain open**. #1696 is the final
governance refresh that records Scaled Trust, Release Provenance and Compliance
v1 as complete only because implementation evidence from #1690 through #1695
exists.

## Reuse and compatibility

- **Compose by reference** over existing runtime/evaluator/evolve/`compare`/
  provenance/asset/`source_apply_*`/`trust_gradient_*`/QA-swarm surfaces; no
  parallel engine, runtime, writer, or provenance system.
- **Owned by Rust/local** trusted logic; the JS runtime owns the deterministic
  runtime and `window.__OUROFORGE__` probe; browser/Studio/dashboard/cockpit
  surfaces are **read-only**. No new language/runtime; distributed/Elixir remains
  NO-GO per ADR #92 (`docs/distributed-elixir-design.md`).
- **Additive and backward-compatible**: workflows without the new bundles, tiers,
  or gates remain valid; no breaking changes to existing artifact shapes in this
  issue.
- Generated runs/assets/content/release artifacts remain **untracked** unless
  explicitly fixture-scoped.

## Boundaries

Conservative wording: this milestone defines per-release safety, audit, and
compliance **workflow contracts**, not production readiness, quality/fun/taste
guarantees, auto-merge, autonomous shipping, or Godot replacement/parity. No
shipping (native/store export), hosted/cloud, real-player telemetry, or live-ops
is authorized absent an explicit Layer-3 GO (DEFER per Milestone 26 / #1508).
Growth stays demand-driven (Milestone 24). High-risk and source-affecting
changes never auto-apply; no release proceeds without compliance
(license/policy/age-rating) and human go/no-go.

**#1 and #23 remain open.**
