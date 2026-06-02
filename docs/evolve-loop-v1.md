# Evolve Loop v1 scope and contract

Evolve Loop v1 upgrades Ouroforge's proposal-only mutation path into an
evidence-linked patch draft iteration loop. It remains local-first, artifact-first,
sandboxed, and manual-review gated.

This document is the canonical control artifact for issues #76 through #82. It
adds no product behavior by itself; each follow-up GitHub issue remains the
implementation contract for its own PRs.

Related control documents:

- `docs/architecture.md` for the Seed -> Run -> Ledger + Evidence -> Scenario
  Results -> Verdict -> Journal -> Mutation Proposal loop;
- `docs/scenario-evaluator-v1.md` for deterministic scenario, verdict, suite,
  and before/after comparison boundaries;
- `docs/scenario-evaluator-v1-demo.md` for the completed Scenario/Evaluator v1
  evidence demo that Evolve Loop v1 may consume.

## 1. Purpose and relation to #1 final goal

Issue #1 defines Ouroforge as an evidence-native game engine built around local,
inspectable Ouroboros loops. Evolve Loop v1 supports that final goal by turning
observed evidence and journal findings into reviewable patch drafts and rerun
comparisons.

Evolve Loop v1 must answer bounded questions:

- Which journal/verdict evidence suggests a change?
- How was that suggestion classified?
- What patch draft was produced from the suggestion?
- Was the draft safely applied in an isolated sandbox?
- Did rerun comparison show improvement, regression, or no change?
- Did a human reviewer accept or reject the draft?

Evolve Loop v1 must not answer unbounded questions such as whether an AI should
merge a change, whether a patch is production-ready, or whether a semantic game
quality improvement exists without artifact evidence.

## 2. Current v0 baseline

The current v0 loop can:

- evaluate a run and write `verdict.json`;
- render `journal.md` from the Seed, evidence, ledger, verdict, and mutation
  proposals;
- create deterministic mutation proposal records for failed verdicts;
- list mutation proposals without applying them;
- compare before/after runs using the Scenario/Evaluator v1 comparison artifact.

The v0 baseline is proposal-only. Mutation proposals are records under the run
artifact tree; they do not edit source files, apply patches, commit, merge, or
publish changes.

## 3. Evolve Loop v1 lifecycle states

Allowed lifecycle states are deliberately small and ordered:

1. `proposed` — a mutation proposal exists and links to run evidence.
2. `classified` — the proposal has a deterministic classification derived from
   journal/verdict/evidence signals.
3. `drafted` — a patch draft artifact exists, but it has not been applied to any
   source tree.
4. `sandbox_applied` — the patch draft was applied only inside an isolated local
   sandbox/worktree, not to `main`.
5. `rerun_compared` — the sandbox rerun and before/after comparison artifacts
   exist.
6. `pending_review` — evidence is ready for manual review.
7. `accepted` — a reviewer accepted the mutation workflow outcome for a bounded
   next action.
8. `rejected` — a reviewer rejected the mutation workflow outcome with a recorded
   reason.

State transitions may move forward only when the required evidence for that
transition is present. `accepted` and `rejected` are terminal for the specific
mutation workflow record. A rejected mutation may inspire a new proposal, but the
old record must remain auditable.

## 4. Evidence requirements per transition

Every lifecycle transition must append or update an artifact-backed record and
must cite concrete evidence paths. A transition without evidence references is
invalid.

| Transition | Required evidence references |
| --- | --- |
| `proposed` -> `classified` | source proposal id, `journal.md`, `verdict.json`, relevant evidence ids/paths, classification reason |
| `classified` -> `drafted` | classification record, draft id, target files/paths as text metadata, patch draft artifact, rationale, source evidence refs |
| `drafted` -> `sandbox_applied` | draft id, sandbox/worktree path or id, apply log, changed-file list, failure log when apply fails |
| `sandbox_applied` -> `rerun_compared` | before run id, sandbox after run id, rerun command, comparison artifact path, verdict/scenario refs |
| `rerun_compared` -> `pending_review` | comparison classification, supported deltas, known unsupported claims, reviewer checklist input |
| `pending_review` -> `accepted` | explicit manual reviewer decision, reviewer identity/source, accepted scope, evidence refs, follow-up action boundary |
| `pending_review` -> `rejected` | explicit manual reviewer decision, rejection reason, evidence refs, whether a new proposal is needed |

Evidence records must remain local artifacts. They may reference generated run
paths, comparison artifacts, sandbox logs, and patch draft files, but generated
run/sandbox state must not be committed unless a follow-up issue explicitly
changes the repository contract.

## 5. Issue order and dependency graph

Implement Evolve Loop v1 in this order:

1. #76 Evolve Loop v1 Scope and Contract — documentation/control only.
2. #77 Journal-to-Mutation Classification — depends on current journal, verdict,
   and mutation proposal artifacts.
3. #78 Mutation Proposal to Patch Draft — depends on #77 classification records.
4. #79 Patch Application Sandbox — depends on #78 patch draft artifacts.
5. #80 Rerun Comparison Integration — depends on #79 sandbox application and
   Scenario/Evaluator v1 before/after comparison from #74.
6. #81 Accepted/Rejected Mutation Workflow — depends on #80 rerun comparison and
   preserves manual review boundaries.
7. #82 Evolve Loop v1 Integration Demo — composes #77-#81 only after they are
   merged and verified.

No follow-up issue may implement later lifecycle states early. If a dependency is
not merged, the next issue must stop or follow the approved PR/merge discipline;
it must not hide stacked scope in another issue.

## 6. Scenario/Evaluator v1 comparison boundary

Evolve Loop v1 may consume Scenario/Evaluator v1 comparison artifacts to decide
whether a sandbox rerun is better, worse, or unchanged according to supported
artifact deltas.

Allowed comparison inputs:

- before run id and run directory;
- sandbox after run id and run directory;
- `run.json`, `verdict.json`, `evidence/index.json`, scenario results, suite
  summaries, assertion failures, performance/evidence counts, mutation proposal
  refs when present;
- the deterministic comparison artifact produced by `ouroforge-cli compare`.

Comparison must not be treated as an AI semantic judge. Unsupported claims, such
as subjective gameplay quality or production readiness, remain unsupported unless
future issues add explicit deterministic evidence.

## 7. Sandbox and manual approval boundaries

Sandbox boundaries:

- Patch drafts may be applied only to an isolated local sandbox or worktree.
- Applying a patch draft must not mutate `main` or the active clean repository
  branch.
- Apply logs and changed-file lists must be recorded.
- Failed patch application is evidence, not permission to retry destructively.
- Remote execution sandboxes, cloud workers, databases, servers, and distributed
  orchestration are outside v1.

Manual approval boundaries:

- Patch drafting is not acceptance.
- Sandbox application is not acceptance.
- Rerun comparison is not acceptance.
- `accepted` and `rejected` require an explicit manual review decision.
- No AI-only acceptance decision is authorized.
- No automatic commit, merge, GitHub auto-merge, or publish action is authorized.

## 8. Language boundary

This #76 issue is documentation only.

Follow-up implementation should default to Rust for mutation classification,
patch draft artifacts, sandbox orchestration, CLI, lifecycle records, and
comparison linkage. JavaScript changes are not expected unless a later demo or
runtime evidence issue explicitly requires a browser-facing surface. No Elixir or
distributed orchestration is authorized for Evolve Loop v1.

## 9. Non-goals and drift risks

Non-goals:

- no autonomous production patching;
- no automatic commit or merge to `main`;
- no GitHub auto-merge or release automation;
- no remote execution sandbox;
- no multi-agent orchestration system;
- no plugin system;
- no unbounded agent loop;
- no server/database/cloud infrastructure;
- no AI-only acceptance decision;
- no source-file mutation in #76.

Primary drift risks and countermeasures:

- Drift into autonomous patching: require manual review before `accepted`.
- Drift into destructive writes: require isolated sandbox/worktree application.
- Drift into unbounded agent loops: keep lifecycle states finite and evidence
  gated.
- Drift into semantic claims: require Scenario/Evaluator comparison evidence and
  record unsupported claims explicitly.
- Drift into infrastructure: keep v1 local artifact-based; defer distributed or
  server designs to explicit later design issues.

## 10. PR decomposition summary for follow-up issues

- #77 should classify existing journal/verdict/proposal signals into bounded
  categories with evidence refs; it must not draft or apply patches.
- #78 should create patch draft artifacts from classified proposals; it must not
  apply drafts to source trees or accept mutations.
- #79 should apply patch drafts only in an isolated local sandbox/worktree and
  record apply logs; it must not mutate `main`.
- #80 should rerun the relevant Seed/scenario in the sandbox and compare before
  and after runs; it must not decide acceptance.
- #81 should add accepted/rejected workflow records with explicit manual review
  evidence; it must not automate merge/commit/publish.
- #82 should add an integration demo that composes #77-#81 through the existing
  evidence-native loop; it must not add missing upstream behavior.

Every follow-up PR must include verification output, lifecycle evidence, guardrail
checks, over-engineering checks, and drift-prevention checks in the PR body.
Generated runs, sandbox directories, target artifacts, and local tool state remain
untracked unless a future issue explicitly authorizes a new committed artifact.
