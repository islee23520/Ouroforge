# Source Apply Threat Model Refresh v1

Source Apply Threat Model Refresh v1 updates the earlier source mutation design
controls for the higher-trust moment where a reviewed patch may be promoted from
preview/sandbox evidence toward trusted worktree writes. It is a design gate for
#700 only. It does not implement trusted apply, rollback, verification runners,
command execution, browser writes, source repair, auto-merge, or dependency/CI
mutation.

The prior design controls remain active inputs:

- `docs/source-mutation-design-gate-v1.md`
- `docs/source-mutation-threat-model-v1.md`
- `docs/source-mutation-file-classes-v1.md`
- `docs/source-mutation-preview-v1.md`
- `docs/source-mutation-sandbox-boundary-v1.md`
- `docs/patch-preview-artifact-v1.md`
- `docs/patch-diff-integrity-v1.md`
- `docs/source-patch-review-gate-v1.md`
- `docs/source-mutation-rollback-audit-v1.md`
- `docs/source-patch-sandbox-dry-run-evaluator-v1.md`
- `docs/studio-source-patch-review-surface-v1.md`
- `docs/safe-source-mutation-apply-v1.md`

## Threat assumptions

Trusted source apply is riskier than preview because a successful path can change
tracked repository files. The apply path must therefore assume:

- preview evidence can be stale, incomplete, or misleading;
- sandbox evidence can diverge from the trusted worktree;
- reviewers can accidentally accept mismatched or partial evidence;
- generated/local state can contaminate source state;
- dependency/CI/build-script changes can hide supply-chain or execution risk;
- browser/Studio surfaces can accidentally become command bridges if affordances
  are not kept inert; and
- public wording can overclaim safety or production readiness before evidence
  exists.

## Risk/control matrix

| Risk id | Trusted-apply risk | Failure mode | Required control | Follow-up issue(s) |
| --- | --- | --- | --- | --- |
| SA-R01 | Source corruption | Patch writes the wrong file, malformed content, or an unsupported source-like class. | Transaction target coverage, allowed file classes, rollback snapshot, post-apply verification. | #701, #702, #706, #707 |
| SA-R02 | Hidden dependency change | Patch changes manifests, lockfiles, package configs, or install/build inputs. | Dependency/CI/build-script blocker before preview promotion and apply readiness. | #709 |
| SA-R03 | CI/workflow mutation | Patch changes `.github/workflows`, CI configs, release/publish/export files, or automation gates. | Forbidden file-class blocker and separate-governance requirement. | #709 |
| SA-R04 | Build-script or shell mutation | Patch introduces build scripts, shell/install scripts, or command execution vectors. | File-class blocker plus allowlisted verification only. | #707, #709 |
| SA-R05 | Rollback failure | Apply succeeds but no usable before-state or recovery guidance exists. | Rollback snapshot must exist and validate before apply readiness. | #706 |
| SA-R06 | Stale patch apply | Target content, branch/head, preview, sandbox report, file-class report, diff integrity, review, or transaction changed. | Stale target/hash guard and freshness checks across all refs. | #703 |
| SA-R07 | Reviewer bypass | Apply proceeds without accepted independent review or with self-review/partial/mismatched decision. | Exact review decision enforcement and independence validation. | #704 |
| SA-R08 | Sandbox/trusted mismatch | Sandbox applied a different diff, ran different commands, missed cleanup, or used a divergent worktree. | Sandbox-to-trusted promotion readiness with exact preview/transaction/hash matching. | #705 |
| SA-R09 | Dirty worktree contamination | Local modified/untracked files, ignored generated roots, symlinks, or path traversal affect apply context. | Trusted worktree boundary and context evidence; fail on dirty target/collision. | #701, #703 |
| SA-R10 | Concurrent apply | Two apply attempts race, share state, or overwrite each other. | Worktree context lock/attempt identity and append-only audit. | #701, #710, #715 |
| SA-R11 | Evidence spoofing | Missing/stale/malformed artifacts make a failed or unsafe apply appear successful. | Evidence bundle validation, audit ledger, status consistency checks. | #710, #711 |
| SA-R12 | Verification spoofing | Commands are arbitrary, shell-expanded, networked, dependency-mutating, too broad, or logs are truncated. | Allowlisted argv runner with forbidden-command rejection and bounded generated logs. | #707 |
| SA-R13 | Outcome overclaim | Apply is called improved/ready without rerun or comparison evidence. | Post-apply rerun/comparison classification with inconclusive and regressed states. | #708 |
| SA-R14 | Browser/Studio trusted action | Dashboard or Studio adds apply, command execution, merge, or trusted-write controls. | Read-only source apply review surface and forbidden action notices. | #712 |
| SA-R15 | Emergency risk persists | A discovered risk cannot stop apply readiness globally or by scope. | Hold/kill-switch state that fails closed and records audit evidence. | #715 |
| SA-R16 | Demo hides gaps | A demo applies only the happy path and masks missing blockers. | Demo after controls plus regression coverage matrix. | #713, #714 |
| SA-R17 | Governance drift | Roadmap/#1 or public docs claim completion before evidence exists. | Final governance refresh and conservative wording audit. | #716 |

## Fail-closed apply preconditions

A trusted apply readiness check must return blocked before any trusted write when
any of these preconditions is missing, malformed, stale, mismatched, or failed:

1. clean and eligible trusted worktree context for every target path;
2. allowed source-like file class report;
3. diff integrity validation;
4. patch preview artifact with target hashes and required tests;
5. source apply transaction linking exact preview, sandbox, review, target hashes,
   rollback, verification, and expected after hashes;
6. current trusted target before-hash match for every target;
7. successful sandbox dry-run evidence for the exact preview/diff/transaction;
8. accepted independent review decision for the exact transaction and target set;
9. rollback snapshot and recovery guidance;
10. dependency/CI/build-script blocker check with no blocked classes;
11. allowlisted post-apply verification plan;
12. audit ledger attempt identity;
13. evidence bundle refs for final status;
14. no active emergency hold for the relevant scope; and
15. explicit proof that browser/Studio surfaces are display-only.

A blocked precondition is evidence, not permission to retry through a shell,
weaken the allowlist, bypass review, or broaden file classes.

## Design-gate non-goals

This document intentionally does not implement:

- trusted source apply;
- rollback writing/restoration;
- verification command execution;
- rerun/comparison orchestration;
- audit ledger persistence;
- source apply evidence bundle generation;
- Studio apply controls;
- dependency/CI/build mutation;
- browser command bridges; or
- auto-merge, auto-apply, autonomous repair, secure-sandbox, production-ready, or
  Godot-replacement claims.

#1 and #23 remain open anchors for roadmap and memory/governance context.
