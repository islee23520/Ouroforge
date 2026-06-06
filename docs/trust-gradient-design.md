# Trust Gradient Design Gate v1

Status: **ADR complete — GO for bounded, reversible, audited auto-apply (narrow scope only)**

Issue: #1476 — Trust Gradient v1 Scope and Design Gate
Anchor: #1 Era E Milestone 22 (Trust Gradient)

This document is the canonical Trust Gradient v1 design artifact. It is a
control milestone, not an implementation milestone. It records the design-gate
decision on whether and how a narrow, reversible, audited exception to the
pervasive "no auto-apply" boundary may ever be allowed, and — if GO — it bounds
that exception and orders the follow-up implementation issues.

This gate follows the project idiom for risky capabilities: the ADR #92
Distributed/Elixir gate, the Native Export Design Gate (#168), and the Source
Mutation Design Gate (#331). No auto-apply behavior is implemented in #1476; it
only decides and bounds. Implementation is authorized only through the gated
follow-up issues listed below.

## ADR decision

Decision: **GO for bounded, reversible, audited auto-apply — narrow scope only.**

The default across the codebase remains **"no auto-apply"**. This gate does not
change that default for any existing path. It authorizes a single, narrow,
opt-in exception, owned by Rust/local, in which a mutation may be applied
without a per-apply human decision **only** when every mandatory safety property
below holds. Every other path — and the default when the exception is off —
remains review-gated exactly as today.

Rationale:

1. Ouroforge already has the safety substrate required to bound such an
   exception: review-gated scene-only mutation apply, the four-gate evaluation
   pipeline, rollback-backed source-mutation checkpoints/audit, evidence
   contracts, and read-only Studio inspection. A Trust Gradient is the
   composition of controls that already exist, not a new trust assumption.
2. The exception is valuable only where it is cheap to reverse and impossible to
   escalate: low-risk **data/scene** mutations that the four gates already
   verify on rerun. Confining auto-apply to that tier yields throughput on the
   safest edits while keeping every higher-trust path human-gated.
3. The risk of a GO is over-broad scope creep. This gate mitigates that by
   making the never-auto-apply set explicit and by requiring that auto-apply be
   off by default, opt-in, kill-switchable, append-only audited, and revertible
   by one command.

Selected alternative: **bounded auto-apply confined to the low-risk data/scene
tier, gated behind mandatory safety properties, off by default.** The rejected
alternatives are (a) NO-GO / keep everything review-gated forever — rejected
because the safety substrate already exists and the narrow tier is provably
reversible; and (b) broad auto-apply across source/high-risk changes — rejected
outright and placed permanently in the never-auto-apply set.

## What a "Trust Gradient" is — and is not

A Trust Gradient is a graduated scale of how much autonomy a mutation may earn,
determined by risk tier and by how completely the safety properties are
satisfied. Higher trust is **earned per mutation** by passing controls, never
granted to an actor, a session, or a file.

Bounded, reversible, audited autonomy **is not**:

- auto-merge of pull requests;
- self-approval or reviewer bypass;
- unsupervised or unrestricted self-modification;
- a correctness or quality guarantee;
- a production-ready claim or a current Godot replacement claim.

It **is** a narrow exception in which the system may apply an already-verified,
cheaply-reversible, low-risk data/scene mutation without a per-apply human
click, while recording an append-only audit trail and remaining instantly
reversible and killable.

## Risk-tier model

Every candidate mutation is classified into exactly one tier. Classification is
conservative: anything ambiguous or unclassifiable is treated as the highest
applicable risk and is **not** eligible for auto-apply.

| Tier | Examples | Auto-apply eligibility |
| --- | --- | --- |
| **T0 — low-risk data/scene** | Scene-only data edits within the existing scene-only mutation contract: entity/component/property value changes, layout/data tweaks that the four gates verify and that the existing rollback path can revert. | **Eligible** for bounded auto-apply only when every mandatory safety property holds. |
| **T1 — review-required data** | Data/scene changes that touch manifests, scene transitions, scenario packs, promotion/regression matrices, or anything whose blast radius the four gates do not fully verify. | **Never** auto-applies; remains human review-gated. |
| **T2 — source-affecting / high-risk / ambiguous** | Any source code, build scripts, tests, CI/workflow, dependencies, repo history, credentials/network/install behavior, or any mutation that cannot be conservatively proven to be T0. | **Never** auto-applies under any configuration. Hard boundary. |

The classifier (follow-up #1477) is the authority for tier assignment. Its
default for any uncertainty is T2 (never auto-apply). Source-affecting,
high-risk, and ambiguous changes never auto-apply regardless of configuration,
confidence, or audit completeness.

## Mandatory safety properties for any auto-apply (all required)

Auto-apply may occur **only if every one** of these holds for that specific
mutation. If any property is missing, the mutation falls back to review-gated
apply (the default), never to silent application.

1. **Tier is T0.** The classifier assigns low-risk data/scene; never T1/T2.
2. **All four gates pass on rerun.** The mutation re-passes the full four-gate
   evaluation pipeline against the post-apply state, not a cached verdict.
3. **High confidence.** The decision carries an explicit, recorded high-confidence
   signal; low/ambiguous confidence disqualifies auto-apply.
4. **Explicit risk budget.** Auto-apply consumes from a bounded, configured risk
   budget (e.g. count/scope ceiling per run); exhausting the budget forces
   review-gated apply.
5. **Append-only audit log.** Every auto-apply (and every skip/fallback decision)
   writes an append-only audit record with provenance, tier, confidence, gate
   results, and the rollback handle. Audit is a precondition, not a side effect.
6. **One-command rollback.** The applied mutation is backed by a checkpoint that
   a single documented command reverts, with rollback evidence.
7. **Emergency kill switch.** A single switch disables all auto-apply immediately
   and returns the system to the review-gated default; when engaged, no mutation
   auto-applies regardless of tier or confidence.

Auto-apply is **off by default** and **opt-in**. Absence of explicit opt-in is
equivalent to the kill switch being engaged.

## What remains manual-review-gated regardless of outcome

- All T1 and T2 mutations, always.
- Any source-affecting change, build/test/CI/workflow change, dependency change,
  repo-history change, or anything in the existing source-mutation blocked set.
- Any mutation where confidence, gate results, risk budget, audit, rollback, or
  kill-switch readiness is not fully satisfied.
- Pull request merges, review decisions, and reviewer assignment — autonomy never
  extends to merge or approval.
- Browser/Studio surfaces: they remain **read-only** inspection of trust-gradient
  state and never apply, write trusted files, or execute commands.

The default stays "no auto-apply" unless GO **and** every mandatory safety
property holds for that specific T0 mutation.

## Backward-compatibility and ownership constraints

- Existing review-gated apply, rollback, evidence, and four-gate contracts remain
  backward-compatible. Any future change is additive and reversible; no existing
  contract is narrowed or made mandatory where it was optional.
- All auto-apply logic, risk classification, audit, rollback, and kill-switch
  ownership is **Rust/local**. Browser/Studio is read-only.
- Generated runs and audit logs remain untracked unless explicitly fixture-scoped.
- Public wording remains conservative throughout: bounded, reversible, audited
  autonomy is not auto-merge, self-approval, or a quality guarantee; no
  production-ready or current-Godot-replacement claim is made or implied.

## Dependency order and closure gates (authorized by this GO)

Follow-up implementation issues are authorized in this order. Each is bounded to
the T0 tier and the mandatory safety properties; none may broaden the
never-auto-apply set.

1. **#1476 Scope and Design Gate** (this issue) — GO/NO-GO, risk-tier model,
   mandatory safety properties, never-auto-apply set, #1/#23 audit.
2. **#1477 Mutation Risk-Tier Classifier v1** — classify every candidate into
   T0/T1/T2; conservative default to T2; data-only, Rust/local.
3. **#1478 Rollback-Backed Bounded Auto-Apply v1** — apply T0 mutations only when
   all mandatory safety properties hold; one-command rollback; off by default.
4. **#1479 Auto-Apply Audit Log and Kill Switch v1** — append-only audit records
   and an emergency kill switch returning to the review-gated default.
5. **#1480 Studio Trust Gradient Inspection Surface v1** — read-only Studio
   inspection of tiers, decisions, audit, and kill-switch state.
6. **#1481 Trust Gradient Demo v1** — end-to-end demonstration over fixtures.
7. **#1483 Scenario Coverage v23: Trust Gradient Regression Suite** — regression
   coverage for tiering, bounded apply, audit, rollback, and kill switch.
8. **#1484 Roadmap and #1 Governance Refresh** — governance/roadmap update after
   the implementation surfaces land, preserving #1 and #23 as open anchors.

```text
#1476 design-gate (GO) -> #1477 risk-tier -> #1478 bounded auto-apply -> #1479 audit+killswitch
       -> { #1480 studio, #1481 demo, #1483 coverage v23 } -> #1484 governance
```

Each follow-up issue must define the exact control/behavior surface, the files
expected to change, the verification commands, and the generated-state audit.
A follow-up that cannot keep auto-apply confined to T0 with every mandatory
safety property satisfied must stop and escalate rather than broaden scope.

## #1 / #23 governance audit

| Audit area | Result |
| --- | --- |
| #1 state | Remains **open**; this gate does not close or narrow it. |
| #23 state | Remains **open** as repo-memory/design context anchor. |
| Default apply behavior | Remains review-gated "no auto-apply" unless the narrow T0 exception is opt-in and every safety property holds. |
| Source / high-risk / ambiguous mutation | Never auto-applies (T2); hard boundary. |
| Auto-merge / self-approval / reviewer bypass | Remain blocked and out of scope. |
| Browser/Studio trusted writes or command execution | Remain blocked; Studio stays read-only. |
| Existing review-gated apply / rollback / evidence contracts | Remain backward-compatible; changes additive and reversible. |
| Generated runs / audit logs | Remain untracked unless explicitly fixture-scoped. |
| Production-ready / Godot-replacement claims | None made or implied. |

## Revisit / stop criteria

Auto-apply scope may **not** be broadened beyond T0 by any follow-up issue.
Broadening (e.g. admitting any source-affecting change, relaxing the four-gate
rerun requirement, making auto-apply on-by-default, or removing the kill switch)
requires a **separate explicit governance decision** under #1, not an
implementation issue. If an implementation issue discovers it cannot meet a
mandatory safety property, it stops and escalates; it does not weaken the
property.
