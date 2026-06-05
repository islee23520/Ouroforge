# Behavior Draft v1

Behavior Draft v1 lets agents propose structured gameplay behavior changes as
untrusted data. A draft can be validated and previewed, but validation is not
apply authority and does not apply trusted files. Any later trusted write must be
a separate Rust/local review-gated flow with fresh evidence.

## Artifact contract

A behavior draft records:

- `draftId` for the local draft identity.
- `target.projectId`, `target.scenePath`, and `target.sceneHash` to bind the
  draft to a project scene and detect stale target drift.
- `proposedBehavior`, using the existing structured Behavior Artifact v1 model.
- `rationale`, `linkedEvidence`, and `expectedScenarioImpact` so reviewers can
  inspect why the draft exists and what scenario evidence it expects to affect.
- `author`, `validationStatus`, and `blockedReasons` so agent/human source and
  blocked/stale states are explicit.
- `untrustedBoundary`, which must state the draft is untrusted, does not apply
  trusted files, and has no arbitrary script authority.

## Read-only validate/preview boundary

`ouroforge behavior draft validate` and `ouroforge behavior draft preview` are
read-only inspection commands. They may parse the draft and read the target scene
hash when `--project-root` is provided. They do not write source files, project
files, scene files, generated evidence, review decisions, browser state, or local
server state.

No arbitrary script execution, `eval`, dynamic import, plugin loader/runtime,
command bridge, browser trusted writes, local server bridge, hosted/cloud
behavior, auto-apply, auto-merge, or self-approval is introduced by Behavior
Draft v1.

## Studio/browser read model

Studio may display exported behavior draft status under `behavior_drafts` or
`behaviorDrafts` as escaped read-only diagnostics: draft id, validation status,
target hash status, evidence/scenario counts, diagnostics, and copyable CLI
preview command text. The browser surface is inert. It cannot run commands,
write files, persist trusted draft state, apply drafts, approve reviews, or treat
preview metadata as a review decision.

## Generated-state policy

Generated behavior drafts remain untracked unless a later issue explicitly scopes
a deterministic fixture. Fixture-scoped examples live under
`examples/behavior-draft-v1/` and are intentionally small source-like contracts
for validator, CLI, and Studio read-model tests. Local generated drafts, preview
outputs, dashboard exports, runs, screenshots, and tool state remain ignored.

## Reviewer checklist

Before accepting a behavior draft flow, verify:

1. The draft is target-hash-aware and stale targets block visibly.
2. Unsupported behavior, missing evidence, unsafe targets, duplicate ids, and
   malformed operations fail validation or remain visibly blocked.
3. Existing structured behavior/events/state-machine contracts remain separate
   from arbitrary executable scripting.
4. Studio/browser surfaces remain read-only or draft-only and inert.
5. #1 and #23 remain open unless a separate governance decision says otherwise.
