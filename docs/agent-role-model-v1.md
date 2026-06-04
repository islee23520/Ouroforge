# Agent Role Model v1

Agent Role Model v1 defines bounded responsibilities for evidence-gated local collaboration. It is accountability metadata only: it does not spawn agents, execute commands, apply patches, merge branches, publish, deploy, or grant trusted write authority.

Fixture: `examples/multi-agent-pipeline-v1/agent-roles.fixture.json`.

## Roles

The v1 fixture defines designer, gameplay engineer, level designer, asset/import planner, QA agent, performance/regression agent, reviewer, critic, and build/release candidate agent. Each role declares:

- allowed outputs;
- required input artifacts;
- required evidence;
- handoff targets;
- forbidden actions.

Outputs remain untrusted until validated by Rust/local contracts and accepted through review-gated apply or promotion. Browser, dashboard, and Studio surfaces may display this model as read-only/draft-only state only.

## Separation requirements

- No self-review: the proposer/generating role cannot be the sole reviewer or critic for trusted apply/promotion.
- Build/release candidate role is design-gate only; it cannot publish, sign, deploy, alter CI/CD, or change visibility.

## Generated-state policy

Generated task boards, handoffs, work packages, snapshots, evidence bundles, runs, dashboard exports, and local tool state stay under ignored local roots unless explicitly fixture-scoped.
