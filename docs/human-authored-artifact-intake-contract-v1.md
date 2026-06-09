# Human-Authored Artifact Intake Contract v1

Issue: #2057 — Era M, Milestone 76

Status: **scope and contract gate**. This document fixes the contract for
human-authored artifact intake before implementation. It is documentation only:
it adds no Studio endpoint, no Rust schema, no evaluator rule, no data store, no
artifact write path, and no new authority.

## Decision summary

Ouroforge may accept a human-authored artifact only as an opt-in candidate for
existing gates. The input is **intervention-as-evidence**: a validated,
recorded proposal/constraint/directive with provenance and base refs, never a
trusted artifact write by itself.

The Studio posture for this capability is **read + gated-write**. A local
Phoenix LiveView surface may display context and capture an intake request, but
it only routes the request to the Rust-owned gated path. Studio, Elixir, and the
browser do not write artifacts, certify semantics, edit ledgers, or bypass
review.

The default loop remains agent-first. If no human-authored artifact is supplied,
the autonomous CLI/Rust loop must still complete with zero human input.

## Two-plane invariant

This milestone inherits and narrows the Era M **two-plane** invariant:

- **Rust = data plane**: owns artifact truth, schema validation, deterministic
  normalization, evaluator decisions, review/apply decisions,
  scene/source-apply preflight/apply, evidence/provenance records, trusted
  writes, and the local-first CLI fallback.
- **Elixir/OTP + Phoenix LiveView = control + presentation**: may render current
  state, capture an intake request, show warnings and gate outcomes, and route to
  Rust/CLI commands. It never owns artifact semantics, performs validation as
  truth, writes trusted artifacts, or stores canonical artifact state.

A captured intake request is untrusted until the Rust data plane validates,
records, and accepts it through the same gates used by agent-authored proposals.

## Intake points and candidate shapes

Human-authored artifacts may enter only at defined intervention points:

| Intake point | Candidate shape | Existing gated path reused | Required evidence |
| --- | --- | --- | --- |
| Scene or level artifact | Proposed scene/level JSON, tilemap, placement, or bounded scene delta | Scene/source-apply preflight, review/apply, evaluator, evidence/provenance | Original human file/text, normalized candidate, target refs, validation report, provenance, review decision |
| Source-like artifact | Proposed source patch or bounded source-adjacent file candidate | Source-apply preview/sandbox/stale-target guards, review/apply, evaluator, evidence/provenance | Patch/diff, target hashes, sandbox/preflight report, reviewer decision, rollback refs |
| Design/content artifact | Human-authored brief, constraint, tuning table, locale/narrative/content proposal | Review/apply plus artifact-specific schema/evaluator gates | Original artifact, normalized proposal, validation output, author/source metadata, decision ref |
| Gate or constraint input | Proposed evaluator constraint/directive | Evaluator validation plus review/apply and provenance | Source text, compiled/normalized rule ref, test/evaluator result, active/inactive decision |

Every row is proposal-only until accepted by the existing gate. Missing, stale,
malformed, unsupported, or overbroad candidates fail closed and remain visible as
blocked evidence.

## Gated write path reused

Human-authored artifact intake creates no parallel path. Every candidate must
reuse the existing gates relevant to the target:

1. **Capture as intervention-as-evidence** with author/source metadata, target
   refs, base hashes, and raw candidate bytes/text preserved as untrusted input.
2. **Normalize in Rust** into the existing proposal, scene/source-apply,
   source-preview, evaluator, or artifact-specific validation shape.
3. **Validate and preflight** with schema checks, stale-ref checks,
   generated-state/source-class policies, evaluator checks, and any
   scene/source-apply preview required by the target.
4. **Review/apply decision** records accepted, rejected, deferred, blocked,
   stale, or needs-fix with evidence refs. Reviewer independence and high-risk
   rules continue to apply where the existing gate requires them.
5. **Trusted apply** happens only through the existing Rust-owned
   scene/source-apply or review/apply mechanism. The original human artifact is
   never copied directly into trusted state.
6. **Ledger/evidence/provenance** links the original candidate, normalized
   candidate, validation reports, decision, and final outcome.

A Studio submit button may call the same local CLI/Rust entrypoint that a CLI
user would call. It may not directly edit scene files, source files, ledgers,
evaluator verdicts, generated artifacts, or governance anchors.

## Read + gated-write Studio contract

A future local Studio intake panel may include:

- current target context and base refs;
- drag/drop or text capture for bounded human-authored candidates;
- stale, unsupported, generated-state, source-class, and scope warnings;
- a submit-to-gates action that creates untrusted intervention evidence;
- status displays for pending, accepted, rejected, deferred, blocked, stale, and
  malformed candidates;
- links to validation, evaluator, review/apply, scene/source-apply, and
  provenance evidence.

The panel must display boundary copy equivalent to:

> Human-authored artifacts are intervention-as-evidence. Studio captures and
> routes; Rust validates, records, and applies only through existing gates.

The panel must not include raw artifact editors, browser file writes, command
bridges, hidden execution, direct ledger edits, direct evaluator decisions,
auto-approve, auto-merge, source/scene apply bypasses, hosted accounts,
collaboration, or remote real-time authority.

## Local-first CLI fallback

The **local-first** CLI fallback is mandatory and sufficient. A fresh checkout
must be able to run the full loop without Studio, Phoenix, Elixir, a database, a
hosted service, a remote worker, or human input.

When implemented, the CLI path must be able to:

1. inspect target state and base refs;
2. record a human-authored candidate as untrusted evidence;
3. normalize it through Rust data-plane validation;
4. run the existing review/apply, scene/source-apply, evaluator, evidence, and
   provenance gates required by the target;
5. record the decision and blockers;
6. continue the autonomous loop deterministically when no human candidate exists.

Studio remains an optional local control/presentation surface over this path,
not a prerequisite for authoring, verification, or completion.

## Failure and hold states

Human-authored artifact intake fails closed when:

- target refs, base hashes, evidence refs, or provenance refs are missing or
  stale;
- the candidate is empty, unbounded, malformed, unsupported, or outside the
  declared target class;
- the candidate attempts a raw write, source/scene apply bypass, direct ledger
  edit, evaluator-verdict edit, generated-state overwrite, or governance anchor
  edit outside explicit governance scope;
- required review/apply, scene/source-apply, evaluator, evidence, provenance, or
  rollback evidence is missing;
- Studio/Elixir/Phoenix attempts to own artifact semantics or trusted writes;
- hosted/multi-user/collaborative authority is required;
- the path makes human input mandatory for autonomous loop completion.

Rejected, blocked, stale, and malformed candidates remain auditable. They may
not be silently converted into accepted artifacts, hidden from the ledger, or
used as prompt-only policy that changes behavior without evidence.

## Non-goals

- Raw human writes to trusted artifacts.
- Mandatory human input or mandatory manual approval for autonomous completion.
- New artifact store, database, hosted service, account model, remote
  collaboration, or real-time multi-user Studio.
- Elixir/Phoenix validation authority, canonical artifact storage, or direct
  artifact mutation.
- New evaluator semantics beyond reusing existing gates.
- Automated fun/taste verdicts or release go/no-go.
- Closing or modifying #1 or #23 outside explicit governance scope.

## Downstream implementation references

Downstream Milestone 76 issues must treat this document as their source of
truth:

- #2058 should implement Rust intake validation and normalized proposal evidence
  without adding a parallel write path.
- #2059 should expose only local Studio control/presentation capture over the
  Rust path.
- #2060 should demo both no-human autonomous fallback and gated human-authored
  intake while preserving this contract.

## Acceptance checklist for future code

- Agent-first default preserved; zero-human-input CLI loop remains valid.
- Human-authored candidate is intervention-as-evidence with provenance.
- Existing review/apply, scene/source-apply, evaluator, evidence/provenance gates
  are reused.
- Rust owns validation, decisions, trusted writes, and CLI fallback.
- Studio is read + gated-write only; Elixir renders/captures/routes.
- Missing/stale/unsupported evidence fails closed.
- No raw bypass, no new data store, and no hosted/multi-user Studio.
- Fun/taste and release go/no-go remain human Ring-2 decisions.
- #1 and #23 remain open.
