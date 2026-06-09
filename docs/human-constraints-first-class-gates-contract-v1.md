# Human Constraints as First-Class Gates Contract v1

Issue: #2065 — Era M, Milestone 78

Status: **scope and contract gate**. This document fixes the contract for human
constraints as first-class gates before implementation. It is documentation
only: it adds no evaluator rule, no Rust schema, no Studio endpoint, no data
store, no artifact writer, and no new authority.

## Decision summary

Ouroforge may accept a human constraint only as an opt-in, evidence-backed gate
input. A constraint is not a prompt-only preference and not a raw override. It is
**intervention-as-evidence**: a validated, recorded proposal/constraint/directive
with provenance, scope, target refs, and decision evidence.

The write posture is **read + gated-write**. A local Studio/Phoenix surface may
show the current run, capture a proposed constraint, and route it to the same
Rust/CLI gate path, but it never owns artifact semantics, evaluator truth, or
trusted writes.

The default remains agent-first. If no human constraint is supplied, the
autonomous loop must still complete with zero human input. Human constraints may
narrow or block a candidate only after being validated and recorded through the
existing gates.

## Two-plane invariant

Milestone 78 inherits the Era M **two-plane** invariant:

- **Rust = data plane**: owns constraint validation, deterministic normalization,
  evaluator gate materialization, review/apply decisions, scene/source-apply
  preflight/apply linkage, evidence/provenance, trusted writes, and the
  local-first CLI fallback.
- **Elixir/OTP + Phoenix LiveView = control + presentation**: may render current
  constraints, capture proposed constraints/directives, display gate outcomes,
  and call the local Rust/CLI route. It never validates as truth, writes
  canonical artifacts, edits ledgers, modifies evaluator verdicts, or bypasses
  Rust-owned gates.

## Constraint intervention points

Human constraints may enter only at defined intervention points:

| Intervention point | Constraint shape | Existing gated path reused | Required evidence |
| --- | --- | --- | --- |
| Candidate review | Accept/reject/needs-fix constraint over a proposal, patch, scene, or artifact candidate | Review/apply, evaluator, evidence/provenance | Constraint text, author provenance, target candidate ref, base hash, decision ref |
| Scene/source change | Bounded constraint over allowed scene/source apply targets | Scene/source-apply preflight, review/apply, evaluator, evidence/provenance | Target refs, stale-target report, source/scene class report, preflight result, review decision |
| Evaluator policy | Proposed evaluator constraint/directive that narrows acceptance criteria | Evaluator validation, review/apply, evidence/provenance | Original directive, normalized evaluator rule ref, dry-run/test result, active/inactive decision |
| Run steering | Local run directive that constrains next autonomous action without applying artifacts | Review/apply or evaluator gate as appropriate, evidence/provenance | Run id, scope, expiry, provenance, accepted/blocked status, audit refs |

Every row is inert until Rust validates it and the relevant existing gate accepts
it. Missing, stale, malformed, unsupported, or overbroad constraints fail closed
and remain visible as blocked evidence.

## Gated path every write reuses

Human constraints create no parallel write path. Every constraint affecting a
write must reuse the existing gates:

1. **Capture as intervention-as-evidence** with author/source metadata, scope,
   target refs, base hashes, expiry if applicable, and raw text preserved as
   untrusted input.
2. **Normalize in Rust** into a bounded proposal, evaluator constraint,
   review/apply decision input, scene/source-apply preflight input, or other
   existing artifact-specific gate shape.
3. **Validate and preflight** with schema checks, stale-ref checks,
   generated-state/source-class policies, evaluator dry-runs, and
   scene/source-apply preview where the target requires it.
4. **Review/apply decision** records accepted, rejected, deferred, blocked,
   stale, or needs-fix with evidence refs. Existing independence and high-risk
   rules continue to apply.
5. **Trusted apply or block** happens only through the existing Rust-owned
   review/apply and scene/source-apply mechanisms. A human constraint may never
   directly mutate trusted scene, source, ledger, evaluator, or generated
   artifacts.
6. **Evidence/provenance ledger** links original constraint text, normalized
   constraint, validation reports, decision, target refs, and final outcome.

A Studio control may submit the same local CLI/Rust request that a CLI user would
submit. It may not directly edit artifacts, ledgers, evaluator verdicts, source
files, scene files, generated state, or governance anchors.

## Read + gated-write Studio contract

A future local Studio constraint panel may include:

- current candidate/run context and base refs;
- capture fields for bounded human constraints/directives;
- stale, unsupported, generated-state, source-class, and scope warnings;
- submit-to-gates actions that create untrusted intervention evidence;
- status displays for active, inactive, accepted, rejected, deferred, blocked,
  stale, malformed, and expired constraints;
- links to evaluator, review/apply, scene/source-apply, and provenance evidence.

The panel must state that human constraints are intervention-as-evidence: Studio
captures and routes; Rust validates, records, and applies or blocks only through
existing gates.

The panel must not include browser trusted writes, raw artifact editors, command
bridges, hidden execution, direct ledger edits, direct evaluator decisions,
auto-approve, auto-merge, source/scene apply bypasses, hosted accounts,
collaboration, or remote real-time authority.

## Local-first CLI fallback

The **local-first** CLI fallback is mandatory and sufficient. A fresh checkout
must be able to run the full loop without Studio, Phoenix, Elixir, a database, a
hosted service, a remote worker, or human input.

When implemented, the CLI path must be able to:

1. inspect current target/run state and base refs;
2. record an optional human constraint as untrusted intervention evidence;
3. normalize it through Rust data-plane validation;
4. run the existing review/apply, scene/source-apply, evaluator, evidence, and
   provenance gates required by the target;
5. record active/inactive/blocked/rejected/stale outcomes and blockers;
6. continue deterministically when no human constraint exists.

Studio remains an optional local control/presentation surface over this path,
not a prerequisite for authoring, verification, or completion.

## Failure and hold states

Human constraints fail closed when:

- target refs, base hashes, evidence refs, provenance refs, or expiry/scope
  fields are missing or stale;
- the constraint is empty, unbounded, malformed, unsupported, contradictory, or
  outside the declared target class;
- the constraint attempts raw artifact writes, direct source/scene apply, direct
  ledger edits, evaluator-verdict edits, generated-state overwrites, or
  governance anchor edits outside explicit governance scope;
- required review/apply, scene/source-apply, evaluator, evidence, provenance, or
  rollback evidence is missing;
- Studio/Elixir/Phoenix attempts to own artifact semantics, evaluator truth, or
  trusted writes;
- hosted/multi-user/collaborative authority is required;
- the path makes human input mandatory for autonomous loop completion.

Rejected, blocked, stale, malformed, and expired constraints remain auditable.
They may not be silently treated as accepted constraints or used as hidden prompt
policy.

## Non-goals

- Raw human writes to trusted artifacts.
- Mandatory human input or mandatory manual approval for autonomous completion.
- New artifact store, database, hosted service, account model, remote
  collaboration, or real-time multi-user Studio.
- Elixir/Phoenix validation authority, canonical artifact storage, or direct
  artifact mutation.
- A parallel evaluator or gate system outside existing review/apply,
  scene/source-apply, evaluator, and evidence/provenance gates.
- Automated fun/taste verdicts or release go/no-go.
- Closing or modifying #1 or #23 outside explicit governance scope.

## Downstream implementation references

Downstream Milestone 78 issues must treat this document as their source of truth:

- #2066 should implement the Rust constraint gate model without adding a
  parallel write path.
- #2067 should expose only local Studio control/presentation capture over the
  Rust path.
- #2068 should lock regression coverage for no raw bypass and zero-human loop
  completion.

## Acceptance checklist for future code

- Agent-first default preserved; zero-human-input CLI loop remains valid.
- Human constraint is intervention-as-evidence with provenance, scope, and target
  refs.
- Existing review/apply, scene/source-apply, evaluator, evidence/provenance gates
  are reused.
- Rust owns validation, evaluator materialization, decisions, trusted writes, and
  CLI fallback.
- Studio is read + gated-write only; Elixir renders/captures/routes.
- Missing/stale/unsupported/contradictory evidence fails closed.
- No raw bypass, no new data store, no hidden prompt-only policy, and no
  hosted/multi-user Studio.
- Fun/taste and release go/no-go remain human Ring-2 decisions.
- #1 and #23 remain open.
