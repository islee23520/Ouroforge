# Shipping and LiveOps Layer-3 Re-evaluation Design Gate v1

Issue: **#1697** (#1 Era H Milestone 45, paired with #1508 Layer-3 gate)

Status: **ADR complete — DEFER native/store export, real-player telemetry,
live balancing, and update/patch pipelines.** A future GO for any one capability
must be a separate #1508 Layer-3 GO with bounded follow-up scope and a human
release-governance plan.

This is a design-gate ADR. It adds **no shipping/liveops implementation code**,
no release automation, no hosted service, no telemetry collection, no live
balancing loop, no update mechanism, and no new engine/runtime/writer. It records
whether Era F-H evidence now demands actual shipping and live-operations
surfaces. **DEFER is the default.** Absent an explicit GO, Rust-first/local-first
is preserved and autonomy ends at a local web release candidate with synthetic
and fixture-scoped evidence.

#1 remains the roadmap/vision anchor and #23 remains the repo memory/design
anchor. This gate preserves both issues as open anchors.

## ADR question

On evidence from Era F through Era H, does the demonstrated production coverage,
autonomy, provenance, compliance, and governance now require any of these
Layer-3 capabilities?

1. Native/store export.
2. Real-player telemetry.
3. Live balancing.
4. Update/patch pipeline.

## Decision summary

| Capability | Decision | #1508 Layer-3 tie | Basis |
| --- | --- | --- | --- |
| Native/store export | **DEFER** | Requires a future #1508 Layer-3 shipping GO. | Current evidence reaches local web export/package and a web release candidate only; there is no demonstrated platform, signing, store-submission, credential, support, or distribution requirement. |
| Real-player telemetry | **DEFER** | Requires a future #1508 Layer-3 telemetry/data GO. | Era F-H evidence uses deterministic probes, synthetic players, QA/playtest evidence, and read-only dashboards; it does not require collecting, storing, or processing real-player data. |
| Live balancing | **DEFER** | Requires a future #1508 Layer-3 live-ops GO. | Balance and game-feel work is local, descriptive, fixture-scoped, and human-gated; no deployed population, live economy, remote config, or automated fun/quality verdict exists. |
| Update/patch pipeline | **DEFER** | Requires a future #1508 Layer-3 update/patch GO. | Source apply, rollback, provenance, compliance, and release-candidate evidence are local review/apply surfaces; they do not authorize release automation, remote patch delivery, auto-merge, or hidden trusted writes. |

**DEFER stands for all four capabilities.** Any later GO must be scoped as a
separate follow-up after #1508 explicitly authorizes the capability, and must
preserve human release governance.

## Era F-H evidence considered

This gate considers only merged evidence and describes capability demand; it is
not a maturity, production, quality, fun, or market claim.

- **Era F — game-class and evidence expansion:** grid-puzzle, puzzle solver,
  design-regression, generative front door, deck-roguelike, synthetic balance,
  and evidence-marketplace work broadened local evidence and bounded game-class
  coverage. The evidence is still fixture-scoped/local and does not demand
  native/store distribution, real-player data, live balancing, or remote update
  delivery.
- **Era G — generation and content systems:** asset/audio/content/long-form
  systems, QA matrix, and scenario coverage added specialized local gates,
  provenance, and read-only inspection. Generated assets/content remain
  proposal-only until license/provenance and function-specific QA pass; no
  unlicensed or unverified-style generated content is promoted, and no hosted
  content marketplace or live content pipeline is required.
- **Era H Milestone 42 — Multi-Agent Production Pipeline v1:** role agents own
  proposal artifacts, hand off with deterministic conflict handling, and pass
  reviewer/critic gates. They never perform direct trusted writes, auto-apply,
  auto-merge, self-approve, bypass reviewers, or publish.
- **Era H Milestone 43 — Autonomous Producer and Whole-Game Orchestration v1:**
  the producer coordinates deterministic plans, budgets, stop conditions, and
  human approval gates. It can produce a release-candidate evidence trail, but it
  is not a release authority and cannot ship or operate a live game.
- **Era H Milestone 44 — Scaled Trust Gradient, Release Provenance and
  Compliance v1:** release trust, per-release provenance, compliance review, and
  Scenario Coverage v41 improve local release-candidate auditability. High-risk
  and source-affecting changes never auto-apply; release still requires
  compliance plus a human go/no-go.

The evidence supports safer local release-candidate preparation and audit. It
does **not** prove a need for Layer-3 shipping/liveops surfaces.

## Per-capability rationale, blockers, and revisit criteria

### Native/store export — DEFER

- **Why defer:** current export evidence is local web/package evidence; native,
  mobile, console, and store export would add signing, platform-specific
  packaging, credential handling, store policy, support, and distribution
  obligations not demanded by Era F-H evidence.
- **Blockers to GO:** a concrete distribution target and platform list; signing
  and credential boundaries; artifact provenance/checksum preservation; store
  policy/compliance mapping; rollback/update strategy; and a human go/no-go that
  cannot be bypassed by automation.
- **Bounded follow-up criteria if GO:** a separate issue must name exactly one
  target family, keep generated release artifacts untracked unless
  fixture-scoped, reuse the existing export/profile/asset-manifest/provenance/
  compliance surfaces, and include blocked negative tests for credentials,
  publishing, signing, and unsupported targets.
- **Revisit when:** a loop-produced game has a documented human-approved need to
  distribute outside the local web release-candidate path.

### Real-player telemetry — DEFER

- **Why defer:** current evidence uses deterministic runtime probes, synthetic
  players, QA/playtest records, and read-only dashboards. It has no real-player
  data collection requirement and no user-consent, privacy, retention, deletion,
  abuse, or data-isolation model.
- **Blockers to GO:** explicit product need; consent/privacy model; data schema
  minimization; retention/deletion plan; local-first opt-out; security review;
  provenance tying telemetry to release/build IDs without exposing private data;
  and human review of any decision informed by the data.
- **Bounded follow-up criteria if GO:** a separate issue must start with a
  fixture-only/local mock telemetry reader, not a hosted collector, and must keep
  browser/Studio/dashboard/cockpit surfaces read-only for trusted state.
- **Revisit when:** synthetic and fixture-scoped evidence is insufficient for a
  human-approved question that requires real-player observations.

### Live balancing — DEFER

- **Why defer:** balance, difficulty, game-feel, and fun/feel gates are
  descriptive and local. Human fun/feel and release verdicts remain mandatory;
  there is no deployed population, remote config channel, live economy, or
  authority to change a shipped title.
- **Blockers to GO:** a human-approved live-ops need; real-player telemetry GO;
  safe remote-config/change proposal model; rollback; rate limiting; audit;
  compliance review; and explicit proof that automated metrics cannot replace the
  human fun/quality/taste verdict.
- **Bounded follow-up criteria if GO:** a separate issue must model proposed
  balance changes as review-gated local proposals first, using existing balance,
  compare, provenance, compliance, source-apply, and trust-gradient surfaces;
  high-risk/source-affecting changes remain manual-review.
- **Revisit when:** a distributed title exists under a prior shipping GO and a
  human operator needs audited, review-gated balance proposals.

### Update/patch pipeline — DEFER

- **Why defer:** the repo already has local source-apply, review, rollback,
  provenance, compliance, and release-candidate audit surfaces. A live update or
  patch pipeline would add credentialed publishing, remote delivery, rollback at
  population scale, support obligations, and security risks not demanded by the
  current evidence.
- **Blockers to GO:** prior native/store or hosted distribution GO; signed update
  format; credential isolation; artifact provenance; staged rollout/rollback;
  emergency hold; compatibility policy; compliance review; and human go/no-go.
- **Bounded follow-up criteria if GO:** a separate issue must begin with an inert
  local package/update manifest and blocked negative cases for credentialed
  publish, CI/workflow mutation, auto-merge, self-approval, reviewer bypass, and
  hidden trusted writes.
- **Revisit when:** an approved shipped/distributed surface requires audited
  patch delivery that local release-candidate evidence cannot represent.

## Reuse statement

Every future follow-up must reuse existing surfaces before adding anything new:
runtime/probe, evaluator gates, visual gate, evolve/campaign, compare,
provenance-bundle and per-release provenance, asset-manifest, QA-swarm,
production role/handoff/review gates, producer budget/human gates, source-apply,
trust-gradient rollback/kill-switch, compliance gate, dashboard, cockpit, and CLI
contracts. This ADR adds no parallel engine, runtime, writer, scheduler, hosted
service, telemetry service, release bot, or update agent.

Trusted validation, persistence, provenance/compliance logic, evidence writing,
run/project binding, source-apply, and CLI behavior remain **Rust/local**.
TypeScript/JavaScript owns deterministic runtime behavior, `window.__OUROFORGE__`
probe data, browser-local read-only inspection, and static dashboard/cockpit
rendering where already scoped. Browser, Studio, dashboard, and cockpit surfaces
remain **read-only** for trusted state.

## Boundaries and conservative wording

- No shipping/liveops implementation is added by this gate.
- No native/store export, hosted/cloud operation, real-player telemetry, live
  balancing, remote update, patch delivery, publish/deploy/sign/upload, or
  credentialed release flow is authorized absent a later #1508 Layer-3 GO.
- Generation, role agents, the producer, and browser/Studio surfaces remain
  proposal-only or read-only; they perform no direct trusted writes.
- High-risk and source-affecting changes never auto-apply.
- No autonomous apply, auto-merge, self-approval, reviewer bypass, hidden trusted
  writes, release bot, or release authority is introduced.
- No release proceeds without compliance plus human go/no-go.
- No automated quality/fun/taste claim is made; art/audio/UX/narrative direction
  and the fun/feel verdict remain human decisions.
- No production-ready engine, commercial-readiness, Godot replacement/parity, or
  autonomous shipping claim is introduced.
- Distributed/Elixir remains NO-GO per ADR #92 (`docs/distributed-elixir-design.md`).
- Generated runs/assets/content/release artifacts remain untracked unless
  explicitly fixture-scoped.

## Governance audit

- #1 remains open.
- #23 remains open.
- This gate modifies no issue bodies and closes no governance anchors.
- The stop condition is explicit: until a separate #1508 Layer-3 GO is merged for
  a named capability, all four shipping/liveops capabilities remain DEFER and
  autonomy ends at a local web release candidate with synthetic and fixture-scoped
  evidence.
