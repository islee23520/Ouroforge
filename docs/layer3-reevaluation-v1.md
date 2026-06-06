# Layer-3 Re-evaluation Design Gate v1

Status: **ADR complete — DEFER all four Layer-3 capabilities; distributed / Elixir
NO-GO reaffirmed (ADR #92).**

Decision date: 2026-06-07

This is a design-gate ADR under #1 Era E Milestone 26, paired with the Era E
governance refresh (#1507). It follows the project idiom for risky scope
(Distributed / Elixir Design Review ADR #92 in `docs/distributed-elixir-design.md`,
and the Native Export Design Gate #168). **No Layer-3 implementation is added by
this issue; it only produces a per-capability GO/DEFER decision.**

## ADR question

On evidence from Era E (Milestones 20-25), is any Layer-3 capability now
warranted: real native export, a plugin runtime, hosted/cloud, or distributed
orchestration / Elixir?

## Decision summary

| Layer-3 capability | Decision | Basis |
| --- | --- | --- |
| Real native export | **DEFER** | Local web export/package evidence is sufficient for the demonstrated scope; no shipped-game/native-distribution demand. |
| Plugin runtime (executable plugins) | **DEFER** | Plugin / Extension System v1 is a declarative, descriptor/evidence foundation that fails closed on executable/credentialed/network capabilities; no demand for executing third-party code. |
| Hosted / cloud | **DEFER** | Everything in Era E is local-first and read-only at the browser boundary; no multi-tenant/hosted requirement is evidenced. |
| Distributed orchestration / Elixir | **NO-GO / DEFER** | ADR #92 NO-GO stands; the Rust local worker pool remains sufficient for the demonstrated loop, and Era E adds no evidence of insufficiency. |

**DEFER is the default and remains in force:** absent a GO, Rust-first /
local-first is preserved and no Layer-3 capability is introduced.

## Era E evidence considered

The decision is grounded in what Era E actually demonstrated (descriptive, not a
maturity claim; see `docs/roadmap.md` Era E closing section and #1507):

- **Loop coverage x game classes (M20/M21):** the loop-coverage metric is
  evidenced across **two** game classes — the collect-and-exit baseline and the
  Signal Gate Platformer. Two local game classes do not require distributed
  scheduling, hosted execution, native packaging, or a plugin runtime.
- **Game complexity (M24):** **one** Game Complexity Ladder rung is satisfied
  (`game-complexity-ladder-v1.collect-and-exit`); engine growth stays
  demand-driven and rung-justified. A single climbed rung does not demand any
  Layer-3 capability.
- **Trust (M22/M23/M25):** the Trust Gradient is GO only for bounded, reversible,
  audited, default-off auto-apply in a narrow scope; evolve campaigns are bounded
  and audited; end-to-end provenance is a read-only composition by reference. The
  trust posture is deliberately local and bounded — it does not call for hosted
  execution or distributed authority.

In short: the loop's demonstrated generalization, complexity, and trust posture
do **not** demand any Layer-3 capability at this point.

## Per-capability rationale, blockers, and revisit criteria

### Real native export — DEFER
- **Why defer:** local web export/package verification covers the demonstrated
  workflow; there is no evidence of demand for native/native-store distribution,
  and native export carries toolchain, signing, and platform-maintenance cost.
- **Blockers to a future GO:** a concrete distribution requirement; a bounded
  target-platform list; signing/verification and artifact-provenance preservation
  equivalent to the current export evidence.
- **Revisit when:** a milestone requires distributing a built game off the local
  web-export path, with the above in scope.

### Plugin runtime — DEFER
- **Why defer:** Plugin / Extension System v1 is intentionally declarative and
  fails closed on executable/credentialed/network capabilities; executing
  third-party plugin code is a large trust-boundary expansion with no current
  demand.
- **Blockers to a future GO:** a sandbox/threat model that fails closed under
  adversarial plugins; a capability-permission model; provenance over plugin
  execution; evidence of demand the declarative descriptors cannot meet.
- **Revisit when:** a milestone requires plugin-owned execution that the
  declarative descriptor/evidence foundation cannot express.

### Hosted / cloud — DEFER
- **Why defer:** Era E is local-first with read-only browser/Studio/dashboard
  surfaces; no multi-tenant, remote-execution, or hosted-state requirement is
  evidenced, and hosting introduces operational, security, and data-ownership
  surface inconsistent with the current local-first guarantees.
- **Blockers to a future GO:** a concrete hosted use case; an authn/authz and
  data-isolation model; preservation of Rust/local trusted ownership and
  read-only browser boundary.
- **Revisit when:** a milestone requires execution or state outside the local
  machine.

### Distributed orchestration / Elixir — NO-GO / DEFER (ADR #92 re-evaluation)
- **ADR #92 re-evaluation:** ADR #92 (`docs/distributed-elixir-design.md`,
  2026-06-02) decided **NO-GO for implementation now** because Rust local
  orchestration is not currently insufficient. Era E does not change that: the
  demonstrated loop ran on the existing Rust local worker pool across two game
  classes and one complexity rung without hitting a scheduling, supervision, or
  fault-tolerance limit that the Rust pool cannot meet. None of ADR #92's revisit
  criteria are met by Era E evidence.
- **Decision:** the ADR #92 NO-GO **stands**; distributed/Elixir remains DEFER.
- **Blockers to a future GO:** the ADR #92 revisit criteria — a measured Rust
  worker-pool limitation (supervision/fault-tolerance/scale) that Rust cannot
  reasonably meet, plus a Rust-artifact-contract and local-first compatibility
  plan.
- **Revisit when:** ADR #92's revisit criteria are met by concrete evidence.

## Rust-first / local-first preserved

Absent a GO, the project remains Rust-first and local-first: Rust/local retains
trusted ownership; browser/Studio/dashboard surfaces remain read-only; contracts
stay backward-compatible; generated artifacts remain ignored unless
fixture-scoped. No native export, plugin runtime, hosted/cloud, or
distributed/Elixir capability is introduced by this gate.

## Conservative wording

This gate makes no production-readiness, quality, fun, commercial-readiness, broad
genre/engine-breadth, or current Godot replacement claim. The Era E evidence it
cites is descriptive, not a maturity claim.

## #1 / #23 governance audit

- #1 (north-star) and #23 (anchor) remain **open** and are neither modified nor
  closed by this gate.
- This gate adds a single ADR document and authorizes no implementation; it
  records DEFER for all four Layer-3 capabilities, keeping the deferral that the
  Era E milestone records and ADR #92 already state.

## Revisit / stop criteria

- Re-run this gate at the next milestone that proposes any Layer-3 capability, or
  when a capability's documented blockers above are concretely met by evidence.
- Until then, DEFER stands and no Layer-3 implementation may be opened on the
  basis of this gate.
