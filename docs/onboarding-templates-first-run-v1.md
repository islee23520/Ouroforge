# Onboarding, Templates, In-Product Docs, and First-Run Scope and Contract v1

Issue: #2082 — Era N, Milestone 83

Status: **scope and contract accepted for implementation planning**. This
milestone is documentation only. It defines how Ouroforge may guide a new local
user through first-run onboarding, templates, and in-product documentation while
preserving the existing agent-first posture, intervention-as-evidence invariant,
read + gated-write Studio boundary, Rust-owned data plane, and local-first CLI
fallback.

## Goal

Onboarding should help a non-developer understand what Ouroforge can do, choose a
bounded starting template, and inspect the next CLI or Studio step without
becoming a trusted writer. First-run guidance is a control/presentation layer over
existing contracts. It can explain, capture choices, and route proposed intent;
it cannot write trusted artifacts, create a new project truth store, validate
artifacts, apply changes, or certify readiness.

If no human uses the onboarding surface, the autonomous agent loop remains able
to run through the CLI with zero human input.

## Reused gated paths

Every write-affecting onboarding action must reuse an existing gate. Studio may
render, capture, and route only; Rust validates and records trusted outcomes.

| Onboarding action | Captured input shape | Required reused path | Trusted effect only after |
| --- | --- | --- | --- |
| Select a starter template | Template selection proposal with user/source metadata | Generative front-door intake, review/apply, evidence/provenance | Rust validates the selected template and review/apply accepts it |
| Customize first-run answers | Constraint or directive proposal against the chosen template | Evaluator/constraint gate, review/apply, evidence/provenance | Constraint compiles or validates through Rust gates |
| Import or paste human-authored starter content | Human-authored artifact candidate | Human artifact intake, scene/source-apply, evaluator, evidence/provenance | Candidate schema/evaluator checks pass and review accepts it |
| Amend generated first-run proposal | Amendment proposal against a base proposal ref | Proposal amendment re-verify, review/apply, evidence/provenance | Re-verification succeeds against current base refs |
| Follow in-product docs step | Copyable CLI command or read-only explanation | CLI fallback and existing Rust command surface | User or agent explicitly runs the CLI; docs never execute commands |
| Record first-run blocker or correction | Correction/directive proposal with target refs | Diagnosis correction, steering directive, review/apply, evidence/provenance | Rust records accepted correction/directive evidence |

No onboarding path creates a raw write. Template and docs choices are proposals,
constraints, or directives until accepted by existing Rust-owned gates.

## Intervention-as-evidence invariant

Human choices in onboarding are **intervention-as-evidence**. They must carry
provenance: actor/source, selected template or doc step, target/base refs when
available, captured text or parameters, validation status, gate verdict, and
accept/reject/block evidence. Missing, stale, malformed, or unverifiable
onboarding input fails closed and leaves the autonomous CLI path intact.

Onboarding must never become mandatory. A user who skips first-run UI, templates,
or in-product docs does not block agent-first execution.

## Read + gated-write Studio posture

Phoenix LiveView Studio may provide local first-run screens, template cards,
copyable CLI commands, docs panels, pending validation states, and evidence
status displays. This is **read + gated-write** only:

- **Read**: display Rust-exported status, known templates, docs, command examples,
  proposal provenance, review state, and evidence locations.
- **Gated-write capture**: capture a proposed template selection, constraint,
  directive, artifact candidate, amendment, or correction and route it to the
  appropriate Rust gate.
- **No raw write**: never write trusted artifacts, ledgers, evidence, scenes,
  sources, docs-generated state, release decisions, merge decisions, or evaluator
  verdicts directly.
- **Fail closed**: stale refs, missing provenance, unsupported templates, or
  unavailable Rust validation keep the item pending/rejected rather than
  broadening Studio authority.

## Two-plane invariant

| Plane | Owns | Onboarding responsibility | Forbidden leakage |
| --- | --- | --- | --- |
| Rust data plane | Artifact truth, template validation, deterministic checks, evaluator decisions, review/apply, scene/source-apply, evidence, provenance, CLI fallback | Validate and record any trusted effect from onboarding choices | None; Rust remains the only trusted artifact/evidence writer |
| Elixir/OTP + Phoenix LiveView control/presentation plane | Local first-run rendering, UI state, capture, routing, docs display, supervision | Make the first-run path understandable and route captured inputs to Rust gates | Artifact semantics, direct artifact mutation, evaluator truth, ledger/evidence writes, hidden command bridges |

Elixir can make the system approachable; it cannot make a template valid,
applied, verified, releasable, merged, or fun.

## Local-first CLI fallback

The CLI fallback is mandatory and sufficient. A fresh checkout must be able to
run the onboarding-adjacent loop without Phoenix, a browser, a hosted service, a
remote worker, a database, or human input. For every Studio first-run affordance,
there must be an equivalent or narrower local CLI path or static documentation
path that preserves the same gates.

Studio may display copyable commands and docs links, but it does not execute
commands as a browser bridge.

## Conservative wording

Onboarding and templates must say "starter proposal", "template candidate",
"pending validation", "copyable command", and "review/apply required". They must
not imply no-code authority, instant shipping, production readiness, automated
fun/taste judgment, auto-apply, auto-merge, reviewer bypass, hosted accounts, or
Godot replacement/parity.

Fun/taste verdict and release go/no-go remain human-owned and evidence-recorded;
they are not automated by first-run guidance.

## Deferred scope

The following are not authorized by this contract:

- direct Studio/browser/Elixir trusted writes;
- a new data store, account system, hosted onboarding service, collaborative
  Studio, remote template registry, or marketplace;
- Elixir-owned template schemas, artifact semantics, validators, evaluator truth,
  or provenance ledgers;
- prompt-only acceptance rules that silently change outputs without a compiled
  Rust/evaluator gate;
- browser command bridges, shell execution, dependency installation, CI/workflow
  mutation, credentialed operations, signing, publishing, deployment, Steam
  account actions, or public launch;
- mandatory human onboarding, fun/taste automation, release go/no-go automation,
  auto-apply, auto-merge, self-approval, or reviewer bypass.

Hosted/multi-user/collaborative Studio remains Layer-3 DEFER.

## Guardrail checklist for downstream M83 issues

Downstream implementation issues must prove all of the following before merge:

- Agent-first default preserved; zero-human-input CLI loop remains valid.
- Onboarding input is intervention-as-evidence with provenance.
- Every write-affecting action routes through review/apply, scene/source-apply,
  source-apply, evaluator, evidence, or provenance gates; no parallel write path.
- Rust owns trusted validation and persistence; Elixir renders/captures/routes
  only.
- Studio posture is read + gated-write and fails closed on missing/stale evidence.
- CLI fallback remains documented, tested, and sufficient.
- No new data store, hosted/multi-user surface, command bridge, release path, or
  fun/taste automation is introduced.
- #1 and #23 remain open governance anchors.

## Verification and evidence expectations

A downstream issue referencing this contract should include evidence for:

1. the first-run/template/docs input shape and provenance fields;
2. the exact reused Rust gate or CLI command;
3. failure behavior for stale/missing evidence;
4. proof that Studio does not write trusted artifacts directly;
5. proof that the CLI path works without Studio or human input; and
6. proof that generated/local onboarding state remains ignored unless explicitly
   fixture-scoped.

For this documentation-only milestone, verification is limited to the repository
build plus text evidence that the contract records read + gated-write,
intervention-as-evidence, two-plane, and local-first boundaries.

## ADR outcome

Decision: **GO for scoped local onboarding, starter-template, in-product docs,
and first-run planning over existing gates, with Phoenix LiveView Studio as read
+ gated-write control and presentation only.**

Rejected alternatives:

- Build a template wizard that writes project artifacts directly: rejected
  because it bypasses Rust validation, evidence, provenance, and review/apply.
- Require first-run human onboarding: rejected because it violates the
  agent-first default and CLI fallback.
- Add hosted/collaborative onboarding or a new template store: rejected as
  Layer-3 scope creep and unnecessary for local-first M83.
- Let Elixir own template semantics or validation: rejected because Rust remains
  the data plane.

## #1 / #23 governance preservation

- #1 remains open as the broad roadmap and vision anchor.
- #23 remains open as the repository memory/design context anchor.
- This contract does not close, replace, narrow, or modify either anchor. Any
  change to those anchors requires a separate explicit governance decision.
