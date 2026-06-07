# Evidence-Native Marketplace v1

Issue: **#1612** (Era F Milestone 33 scope and design gate)

Evidence-Native Marketplace v1 defines how accumulated evidence compounds into
an ecosystem moat via **verifiable assets**: each asset ships its acceptance
suite, a deterministic replay proof, and a provenance lineage so a consumer can
re-verify it locally instead of trusting a publisher's claim. This is a
**scope/design-gate** document. It defines contracts and boundaries only and
adds **no executable behavior** and **no hosted or paid capability**.

The scope is the **local verifiable-asset registry** that runs inside the free
OSS core. Any hosted, cloud, networked, or paid marketplace — including a
transaction layer or take-rate collection — is **Layer-3-adjacent** and stays
**DEFER until a #1508 Layer-3 GO**. Absent that GO, only the local registry is
in scope.

## OSS-core-vs-paid take-rate boundary

- **Engine is free.** Generation, the deterministic verification loop, the local
  registry, replay proofs, and provenance bundles are part of the free OSS core.
  Publishing and consuming verifiable assets locally costs nothing and requires
  no account, network, or credential.
- **Transaction layer is paid and Layer-3-gated.** Any hosted distribution,
  payment, settlement, or take-rate (revenue-share) collection belongs to a
  future hosted marketplace. It is **not** implemented here and stays DEFER
  until a #1508 Layer-3 GO. This document records the boundary; it does not
  create the paid layer or any hook toward it.
- The take-rate boundary is a **policy line**, not a code path: nothing in the
  local registry computes, records, or reserves a fee.

## Local verifiable-asset registry contract

The registry is a **local index of verifiable assets** owned by Rust/local
tooling. It is additive and backward-compatible; workflows that never touch the
registry remain valid.

- **Asset** — a publishable unit (for example a scene/seed/source bundle) paired
  with the evidence required to re-verify it: its acceptance suite, a
  deterministic replay proof, and a provenance lineage (see below).
- **Publish (local)** — record an asset entry by reference: the artifact refs +
  hashes, the acceptance-suite ref, the replay-proof ref, and the provenance
  bundle ref. Publishing is an index write to local storage only; it performs no
  network, upload, sign, deploy, or credentialed operation.
- **Consume (local)** — resolve an asset entry, then **re-verify locally** by
  replaying its proof and running its acceptance suite against the referenced
  artifacts. Consumption trusts the **replay**, not the publisher.
- **Permissions** — the registry is proposal-and-record only. It never performs
  a trusted write into a project; adopting a consumed asset flows through the
  existing **review / apply / trust-gradient** path
  (`source_apply_*` / `trust_gradient_*`), never a direct trusted write and
  never from a browser/Studio surface.
- **Read-only surfaces** — dashboard/Studio surfaces may render exported registry
  entries read-only. They do not publish, consume-apply, merge, or execute
  shell commands.

## Asset replay-proof + provenance binding contract

Each registry asset **binds** to a Milestone 25 provenance bundle; the registry
adds no parallel provenance engine.

- **Acceptance suite** — the scenario/evaluator suite that defines the asset's
  declared behavior, carried by reference.
- **Deterministic replay proof** — references sufficient to reconstruct and
  re-run the asset locally and compare against the recorded result. Replay
  outcomes are enumerated (for example **reproduced**, **diverged**,
  **not-replayable**); the enumeration and any checking logic are defined in
  follow-up issues, not here.
- **Provenance lineage** — the asset binds to a **Provenance Bundle Model v1**
  (`provenance_bundle.rs`, #1500) entry: intent → artifact → trusted validation
  → runtime observation → evaluator verdict → regression comparison → review →
  promotion, composed **by reference**. The registry stores the bundle ref; it
  does not duplicate or re-author bundle contents.
- A consumer treats an asset as verifiable only when its provenance bundle
  resolves, its replay reproduces, and its acceptance suite passes locally.

## Layer-3 gating

- The **local registry** (publish/consume/re-verify, all local) is in scope.
- Any **hosted/paid marketplace** — networked distribution, transaction layer,
  take-rate collection, remote asset hosting, or distributed/Elixir
  orchestration — stays **DEFER until a #1508 Layer-3 GO** (Layer-3 distributed
  orchestration / Elixir is NO-GO per ADR #92 absent that gate).
- No code in this milestone may add a network, upload, payment, or credentialed
  path, or a hook reserved for one.

## Follow-up dependency order and closure gates

Follow-ups reuse existing surfaces (`provenance_bundle.rs`, `source_apply_*` /
`trust_gradient_*`, runtime/probe, evaluator, dashboard/cockpit) and build **no
new engine, runtime, or writer**. Each follow-up closes only after its own
verification passes and **#1 and #23 remain open**.

1. **#1612 — Scope and Design Gate** (this doc). Defines the registry/marketplace
   boundary, the take-rate boundary, the replay-proof/provenance binding, and
   the Layer-3 gating. **Closure gate:** doc merged; no behavior added.
2. **#1613 — Local Verifiable-Asset Registry v1.** Implements the local
   registry contract (publish/consume by reference, local re-verify) in
   Rust/local. **Prerequisite:** #1612 merged.
3. **#1614 — Asset Replay-Proof and Provenance Binding v1.** Binds each asset
   to a deterministic replay proof (re-run on consume) and a Milestone 25
   provenance lineage; reuses the existing replay digest and
   `provenance_bundle.rs`, no new provenance engine. **Prerequisite:** #1613
   merged.
4. **#1615 — Evidence-Native Marketplace Demo v1.** A read-only,
   fixture-scoped demonstration of publish → consume → local re-verify.
   **Prerequisite:** #1614 merged.
5. **#1616 — Scenario Coverage v33.** Regression suite locking the registry,
   replay binding, and boundaries. **Prerequisite:** #1615 merged.
6. **#1617 — Roadmap and #1 Governance Refresh.** Records Milestone 33 as
   complete and reaffirms the Layer-3 gate. **Prerequisite:** #1616 merged.

## Reuse and compatibility

- **Compose by reference** to existing provenance / evidence / review / apply /
  trust-gradient contracts; no parallel provenance engine and no new runtime.
- **Additive and backward-compatible**; no breaking changes to existing artifact
  shapes in this issue.
- Generated registry/replay/evidence outputs remain **untracked** unless
  explicitly scoped as source-like fixtures.

## Boundaries

Conservative wording: this defines a **local, evidence-native asset registry**
and its verification workflow — not a hosted/paid marketplace, not network
distribution, not a production-ready engine, not Godot replacement or parity,
and no claim that generated games are good, fun, or shippable. Generation stays
proposal-only through review/apply/trust-gradient; browser/Studio surfaces stay
read-only.

**#1 and #23 remain open.**
