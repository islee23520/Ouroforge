# File and Artifact Ownership Conflict Policy v1

File and Artifact Ownership Conflict Policy v1 is the #668 policy artifact for Multi-Agent Production Pipeline v1. It makes file path ownership, artifact ownership, shared read-only references, exclusive write targets, generated-output roots, conflict states, and escalation rules explicit before work packages are accepted.

The policy is metadata and validation input only. It does not lock files, spawn agents, execute commands, apply changes, merge PRs, write trusted browser state, publish releases, or mutate source files.

## Schema

The schema version is `file-artifact-ownership-policy-v1`. A policy contains:

- `policyId` — stable local policy identifier;
- `milestone` — governing milestone or issue reference;
- `entries[]` — ownership records;
- `generatedState` — generated roots and `trackedFixtureOnly` policy;
- `guardrails` and `forbiddenActions` — conservative no-silent-overwrite/no-hidden-agent/no-auto-apply/no-command-bridge boundaries;
- `boundary` — plain-language statement that the policy does not lock files or execute work.

Each entry contains:

- `id`;
- `ownerAgent`;
- `role`;
- `target` with `kind`, `id`, `path`, and optional `generatedRoot`;
- `mode`;
- `state`;
- `workPackageRefs[]`;
- `evidenceRefs[]`;
- `blockedReasons[]` where scoped;
- optional `escalation` with `reason`, `requiredDecision`, and `safeNextAction`.

## Target kinds

Allowed target kinds are:

- `file-path`
- `artifact`
- `generated-output-root`
- `shared-read-only-ref`

## Ownership modes

Allowed modes are:

- `shared-read-only`
- `exclusive-write`
- `generated-write`
- `escalation-hold`

Mode is policy data only. It cannot imply a trusted writer, source apply authority, browser write authority, hidden lock, or automatic conflict resolution.

## States

Allowed states are:

- `proposed`
- `active`
- `blocked`
- `deferred`
- `escalated`
- `released`

Blocked or escalated entries must include `blockedReasons`. `escalation-hold` entries must include escalation metadata.

## Fixture set

Tracked fixtures live under `examples/multi-agent-pipeline-v1/`:

- `ownership-policy.no-conflict.fixture.json` — distinct exclusive-write and generated-write targets.
- `ownership-policy.shared-read.fixture.json` — shared read-only references to the same source document.
- `ownership-policy.exclusive-write.fixture.json` — exclusive write ownership over source-like and generated artifact targets.
- `ownership-policy.conflict.fixture.json` — unresolved conflict state that remains fixture evidence only.
- `ownership-policy.escalation.fixture.json` — escalated ownership hold with safe next action and decision requirement.
- `ownership-policy.malformed.fixture.json` — intentionally malformed policy missing required evidence.

Generated ownership policies and conflict reports remain untracked under generated roots such as `runs/multi-agent-pipeline/` unless a future issue explicitly scopes a deterministic fixture.

## Trust boundary

Rust/local validation owns schema acceptance and future conflict detection. Browser, dashboard, and Studio surfaces may display ownership/conflict evidence only when separately scoped as read-only/draft-only consumers. They must not add browser-side trusted writes, command bridges, local server command bridges, credentialed commands, network/install commands, dependency mutation, CI/workflow mutation, release automation, auto-apply, auto-merge, self-approval, silent overwrite, hidden locks, or hidden worker orchestration.
