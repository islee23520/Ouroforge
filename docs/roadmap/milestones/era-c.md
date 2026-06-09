### Era C: Agentic Game Builder

#### Milestone 12: Game Design Document to Playable Prototype

Goal: transform a game design brief into a playable prototype with traceable evidence.

Target deliverables:

- GDD / design brief schema;
- mechanics, asset/style, scene, scenario, and acceptance-criteria extraction;
- project scaffold generation;
- implementation task graph;
- first playable run;
- evidence bundle, journal, and next mutation proposals.

Example target command:

```bash
ouroforge prototype docs/game-designs/tiny-rpg.md
```

Success criteria:

- The command produces a project scaffold, scenes, gameplay logic, placeholder or referenced assets, scenarios, a first playable run, evidence, journal, and mutation backlog.
- A human can inspect why the generated prototype satisfies or fails the design brief.

#### Milestone 13: Multi-Agent Production Pipeline

Goal: make role-specialized agent collaboration part of the engine workflow.

Target deliverables:

- designer, gameplay engineer, level designer, asset/import, QA, performance, build/release, reviewer, and critic roles;
- file/artifact ownership model;
- handoff artifacts and conflict resolution;
- shared state, approvals, guardrails, and observability;
- evaluator-gated promotion and regression blocking.

Success criteria:

- Multiple agents can work on one game project without hidden state or unreviewed trusted writes.
- Every handoff and decision is recorded as evidence or journal context.
- Reviewer/critic agents can block promotion before trusted apply, export, or release.

#### Milestone 14: Autonomous QA and Playtest Swarm

Goal: make adversarial playtesting a first-class engine primitive.

Target deliverables:

- scenario generator and fuzz input runner;
- regression matrix across scenarios, seeds, projects, browsers, and platforms where supported;
- visual comparison, performance budgets, crash/console monitoring, and accessibility checks;
- gameplay objective solver or heuristic playtester;
- flaky evidence detection;
- failure-to-mutation backlog generation.

Success criteria:

- QA workers can actively search for broken gameplay, unreachable goals, regressions, crashes, and performance violations.
- Failed evidence can be classified and converted into proposed fixes or design questions.
- Humans can understand failures through dashboard, journal, and replay artifacts.

#### Milestone 15: Safe Source Mutation Apply

Goal: allow source-changing agent work only after explicit sandbox, review, and rollback controls exist.

Target deliverables:

- Source Mutation Apply design gate;
- isolated sandbox/worktree apply;
- allowlisted commands only;
- stale-target detection;
- dependency/CI/build-script mutation blocked unless separately authorized;
- rollback snapshots and audit records;
- review decision ledger;
- post-apply rerun and before/after evidence comparison.

Success criteria:

- Agents can propose source patches, run them in sandbox, and request review.
- Trusted worktree apply only happens after explicit accepted review/policy approval.
- Apply is rollbackable and linked to evidence, tests, journal, and regression comparison.
