# Studio Multi-Agent Pipeline Inspection Surface v1

Studio Multi-Agent Pipeline Inspection Surface v1 is the #677 read-only UI contract for inspecting Multi-Agent Production Pipeline v1 evidence. It lets Studio/dashboard/cockpit display pipeline readiness, blockers, malformed data, and generated-state boundaries without becoming an agent runner, trusted writer, command bridge, hosted orchestrator, release system, or production automation surface.

## Inputs

The surface consumes trusted Rust/local read models and fixture-scoped examples, including:

- `studio-multi-agent-pipeline-inspection-read-model-v1` for normalized section status, item counts, blockers, and malformed reasons;
- production task boards;
- role model and ownership policy evidence;
- work package and handoff summaries;
- review/critic, QA queue, performance/regression, decision-ledger, and production evidence bundle sections.

Missing or malformed inputs stay visible as read-only warnings. The browser must not repair, overwrite, persist, promote, or hide malformed pipeline data.

## Boundary

The Studio, dashboard, and cockpit surfaces are inspect-only. They must not:

- execute commands or turn copyable command text into a command bridge;
- spawn agents, hidden background agents, unbounded workers, or cloud/hosted orchestration;
- write trusted browser state, source files, generated evidence, dashboards, runs, ledgers, task boards, work packages, handoffs, reviews, QA queues, regression data, decisions, or evidence bundles;
- auto-apply, auto-merge, self-approve, bypass reviewers, promote outputs, publish, sign, release, deploy, mutate dependencies, or edit CI/workflow/build scripts;
- claim autonomous arbitrary game completion, production readiness, shipped-game readiness, commercial readiness, or current Godot replacement capability.

## Rendering requirements

- Escape all rendered data, including section labels, blocker text, malformed reasons, and boundary text.
- Render status, item counts, blockers, malformed reasons, generated-state warnings, and empty states as inert text.
- Do not render buttons, forms, command execution controls, local server bridges, browser command bridge controls, auto-apply controls, auto-merge controls, or self-approval controls.
- Preserve existing dashboard/cockpit panels for task boards, ownership, role models, work packages, handoffs, evidence bundles, mutation review, QA, and regression state.
- Keep Generated task boards, handoffs, work packages, snapshots, evidence bundles, runs, traces, screenshots, dashboard exports, and local tool state ignored unless explicitly fixture-scoped.

## Audit evidence

MAP13.14.3 audits this contract with local tests that scan this document plus the dashboard/cockpit source for the required no-agent-runner, no-write, no-command, no-cloud, no-auto-apply, no-auto-merge, no-self-approval, and escaped read-only rendering language.

