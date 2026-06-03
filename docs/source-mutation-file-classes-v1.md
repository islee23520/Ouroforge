# Allowed Source Mutation File Classes v1

Allowed Source Mutation File Classes v1 is a Source Mutation Design Gate v1
artifact. It classifies which repository file classes could be considered by a
future source patch proposal workflow and which classes remain disallowed unless
a separate explicit design approval changes the boundary.

This document is design/control only. It does not implement source mutation,
source patch generation, source patch application, file-class enforcement,
preview schemas, sandbox behavior, browser trusted writes, or command execution.

## Classification statuses

| Status | Meaning |
| --- | --- |
| Potentially allowed later | A future proposal may include this class only after the relevant later design issue defines preview, review, rollback, sandbox, evidence, and stale-target controls. |
| Restricted / separate approval | The class is not eligible by default; a separate design approval must justify why it belongs in a future source mutation milestone. |
| Blocked by design | The class is out of scope for this design gate and must not be accepted by a future source mutation flow without a separate governance decision. |

## File-class matrix

| File class | Examples | Status | Rationale |
| --- | --- | --- | --- |
| Deterministic scene fixtures | `seeds/*.yaml`, curated example scene JSON under `examples/` | Potentially allowed later | Existing scene-only mutation and validation concepts already distinguish bounded source-like data from ignored run state. Future source patch work still needs explicit preview/review/rollback controls. |
| Scenario packs and deterministic regression fixtures | `examples/**/scenario*.json`, tracked regression pack data | Potentially allowed later | These are evidence-facing source-like data and can be reviewed semantically, but test/evidence spoofing risk requires elevated review. |
| Runtime demo configuration | Static demo config, manifest-declared local demo references | Potentially allowed later | Bounded demo config may be reviewable as data when it cannot execute commands, fetch network code, or cross trusted boundaries. |
| Deterministic source-like generated templates | Tiny tracked templates explicitly promoted by an issue | Potentially allowed later | Generated-origin content may be allowed only when promoted as deterministic source-like fixture with provenance and no ignored-state ambiguity. |
| Bounded game behavior modules | Small local behavior modules in a future explicitly approved surface | Restricted / separate approval | Behavior code can affect runtime semantics and tests; it requires a separate design decision, risk labels, review separation, and rollback evidence. |
| Documentation and governance docs | `README.md`, `docs/*.md`, issue/roadmap handoff text | Restricted / separate approval | Docs can alter public claims and governance anchors. They are safer than executable code but can still weaken guardrails or overclaim readiness. |
| Rust trust-boundary code | `crates/**`, CLI/core validators, artifact writers, mutation/review code | Restricted / separate approval | Rust code owns trusted persistence and validation. It must not be mutated by a source patch flow without a dedicated design and review gate. |
| Tests and evidence readers | Rust tests, JS smoke tests, dashboard/read-model tests | Restricted / separate approval | Test/evidence changes can hide failures or spoof confidence. Future proposals require elevated independent review. |
| Browser/Studio display code | `examples/evidence-dashboard/**`, `examples/authoring-cockpit/**` | Restricted / separate approval | Display-only edits may be possible later, but any trusted write or command bridge remains blocked. |
| Build scripts and host-executed tooling | `build.rs`, package scripts, installer scripts, shell scripts | Blocked by design | These can execute host commands and create arbitrary code execution paths. |
| CI workflows, release automation, and secrets config | `.github/workflows/**`, deployment/release config, secret references | Blocked by design | These can expose credentials, alter automation authority, or perform public/release actions. |
| Dependency manifests and lockfiles | `Cargo.toml`, `Cargo.lock`, package manifests/locks | Blocked by design | Dependency changes require supply-chain review and must not be introduced by a generic source mutation flow. |
| Auth, network, hosted/cloud/server code | Server/auth/cloud integrations, network service code | Blocked by design | Hosted/server/auth scope is outside the current local-first roadmap boundary. |
| Plugin loaders or dynamic extension code | Plugin runtime, marketplace, dynamic loading hooks | Blocked by design | Plugin runtime scope is explicitly out of scope and would expand execution authority. |
| Native export or packaged editor code | Native export/build packaging/editor distribution code | Blocked by design | Native export and production editor claims are out of scope for this roadmap segment. |
| Ignored local/generated state | `runs/`, `target/`, `.omx/`, `.omc/`, `.claude/`, `.openchrome/` | Blocked by design | Generated/local artifacts are not source and must not be committed or treated as reviewed source changes. |
| Binary or opaque generated assets | Binary blobs, minified/generated output without deterministic source | Blocked by design | Opaque content prevents reviewer-visible diff, rollback, and rationale checks. |

## Matrix boundary

A `Potentially allowed later` status is not permission to apply patches. It only
means the class may be discussed in a future source mutation proposal after the
required preview artifact, review gate, rollback/audit, sandbox, evidence, and
stale-target controls exist. Until then, source mutation apply remains blocked.

#1 remains the broad roadmap/vision anchor and #23 remains the repo-memory/design
context anchor. This document does not close, replace, or narrow either issue.

## Review levels

| Review level | Applies to | Required reviewer evidence |
| --- | --- | --- |
| Standard design review | Documentation/governance docs that do not alter public readiness claims or trusted boundaries. | Issue/PR drift lock, no-implementation audit, conservative wording scan, and #1/#23 open-state check. |
| Elevated source-like data review | Deterministic scene fixtures, scenario packs, runtime demo config, and promoted deterministic templates. | File-class label, source/generated boundary check, semantic rationale, stale-target check, rollback/audit expectation, and focused verification list. |
| Elevated behavior review | Bounded game behavior modules, tests, evidence readers, browser/Studio display code, or Rust trust-boundary-adjacent files if separately approved. | Independent reviewer separation, risk IDs from the threat model, affected-surface summary, multi-file invariant checklist, rollback plan, and explicit proof that no command bridge/trusted write path is introduced. |
| Separate governance approval | Rust trust-boundary code, dependency manifests/lockfiles, CI/workflow/secrets, build scripts, plugin/runtime/hosted/native export/public launch scope. | A new design/governance issue must authorize the class before any future patch preview can include it. Default decision is reject/hold. |
| Reject by default | Ignored generated state, opaque binaries, stale previews, symlink/hard-link/path-ambiguous targets, unclassified files, or any file outside canonical repo root. | Reviewer-visible rejection reason and no source write. |

## File-class drift detection expectations

Future preview/review artifacts should detect and report drift before any reviewer
decision:

1. **Path drift**: canonical path must stay inside the repository root and must
   not traverse through symlinks, hard links, hidden generated roots, or ignored
   local state.
2. **Classification drift**: every touched file must have exactly one current
   file-class label; unknown or newly introduced classes default to reject/hold.
3. **Boundary drift**: a patch that changes browser/Studio code, tests, evidence
   readers, review gates, rollback contracts, sandbox contracts, or trust-boundary
   Rust code must be elevated even if another file in the same patch is lower
   risk.
4. **Generated-state drift**: ignored local/generated files must remain ignored
   and untracked; deterministic generated-origin fixtures need explicit promotion
   rationale.
5. **Stale-target drift**: previewed target hashes and latest-main context must
   match at review time; mismatches require regenerating the preview.
6. **Governance drift**: #1 and #23 must remain open, conservative public wording
   must be preserved, and no future patch may claim production/source-mutation
   readiness from a design-only artifact.
7. **Scope drift**: adding source mutation apply, arbitrary patch apply,
   auto-merge/auto-accept, hidden command execution, browser trusted writes,
   dependency changes, CI mutation, plugin runtime, hosted/server/auth, native
   export, public launch automation, or Godot replacement scope is a hard reject.

## Closure expectations for later file-class decisions

Any later issue that changes this matrix must record:

- the previous class and proposed class;
- why the change is necessary for the current milestone;
- affected threat-model risk IDs;
- required review level before and after the change;
- verification commands and evidence needed to prove no boundary weakening;
- generated-state audit result; and
- #1/#23 open-state confirmation.
