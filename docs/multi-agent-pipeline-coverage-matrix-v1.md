# Scenario Coverage v12 / MAP13.16.3 coverage matrix

This matrix records the final MAP13.16.3 compatibility contract for the multi-agent pipeline regression suite. It is a regression and read-model coverage artifact only: dashboard and Studio/cockpit surfaces may display advisory evidence, but they do not gain worker-spawn, command execution, trusted writes, auto-apply, auto-merge, or self-approval authority.

## Coverage matrix

| Area | Covered behavior | Compatibility surface | Evidence |
| --- | --- | --- | --- |
| Role validation | Agent role definitions keep reviewer, critic, QA, performance, and build/release duties separated and reject forbidden output authority. | Core validator, dashboard display, Studio/cockpit display. | `scenario_coverage_v12_review_critic_qa_and_build_lanes_stay_separated`; role fixture display tests. |
| Task board/status transitions | Authoring-loop task board transitions are state-machine metadata and cannot jump directly to completed or silently retry from running to pending. | Core authoring-loop plan validator. | `scenario_coverage_v12_task_board_status_transitions_are_state_machine_only`. |
| Ownership conflicts | Duplicate step ids and duplicate expected artifact ids are blocked before work-package evidence is accepted. | Core validator and evidence bundle generation. | `scenario_coverage_v12_work_packages_and_ownership_block_conflicts`. |
| Work packages | Each work package records expected artifacts, required decisions, rollback refs for trusted mutations, and producer/consumer prerequisites. | Core plan schema and generated evidence bundle. | Valid authoring-loop fixtures plus MAP13.16.1 regression tests. |
| Handoff v2 / handoff contract | Handoffs expose current step, next safe action, blockers, required decisions, drift guardrails, inert allowed commands, forbidden actions, and evidence refs. | Dashboard handoff cards, Studio/cockpit handoff surface, Studio loop cockpit read model. | `agent_handoff_generation_writes_advisory_contract_and_ledger_summary`; JS handoff rendering tests. |
| State snapshot/staleness | Stale project context and missing snapshot refs are reported as read-only status evidence, not repaired or overwritten by the UI. | Core status read model, dashboard evidence bundle, Studio/cockpit display. | `scenario_coverage_v12_snapshots_and_staleness_are_read_only_status_evidence`. |
| Review/critic independence | Reviewer decisions and critic findings are separate append-only event kinds; self-review and self-approval stay forbidden. | Core role model, decision ledger, dashboard display, Studio/cockpit display. | `scenario_coverage_v12_decision_ledger_appends_review_and_critic_events`. |
| QA queue / QA worker assignment | QA candidate, fuzzing, and worker assignment evidence is display-only and must not spawn workers from dashboard or Studio. | Dashboard QA cards, Studio/cockpit QA surface. | Dashboard and cockpit tests assert read-only worker assignment language. |
| Performance/regression lane | Performance regression agents may summarize metrics and regression candidates without production-safety claims or automatic promotion. | Core role model and evidence bundle categories. | `scenario_coverage_v12_gate_lane_bundle_categorizes_review_regression_and_matrix`. |
| Build/release design gate | Build/release candidate lane is design-gate metadata only and does not publish, deploy, merge, or mutate CI. | Core role model and display docs. | Role fixture validation and JS display tests. |
| Decision ledger append-only | Review and critic events append to the ledger in order and retain advisory boundaries. | Core ledger helpers. | `scenario_coverage_v12_decision_ledger_appends_review_and_critic_events`. |
| Production/authoring loop evidence bundle | Bundles categorize runs, comparisons, proposals, review decisions, transactions, regression promotions, matrix snapshots, journal summaries, and missing refs. | Core bundle read model, dashboard, Studio/cockpit. | `scenario_coverage_v12_gate_lane_bundle_categorizes_review_regression_and_matrix`. |
| Dashboard display | Dashboard renders role models, handoffs, loop evidence bundles, missing/malformed/stale states, and generated-state boundaries as inert escaped HTML. | `examples/evidence-dashboard/dashboard.js`. | `node examples/evidence-dashboard/dashboard.test.cjs`. |
| Studio/cockpit display | Studio/cockpit renders role models, handoffs, loop evidence bundles, cockpit rows, missing refs, allowed inert commands, and forbidden actions without execution controls. | `examples/authoring-cockpit/cockpit.js`. | `node examples/authoring-cockpit/cockpit.test.cjs`. |
| Malformed/missing/stale/unresolved conflict evidence | Malformed input, missing refs, stale snapshots, unresolved ownership conflicts, and blocked prerequisites remain visible as evidence/status rows instead of being auto-fixed. | Core validators plus dashboard and Studio read models. | MAP13.16.1, MAP13.16.2, and MAP13.16.3 regression commands below. |

## Generated-state and compatibility audit

Generated runs/screenshots/traces/dashboard exports/local tool state remain generated and ignored unless a fixture explicitly scopes them for regression coverage. The multi-agent pipeline read models may point at generated roots such as `runs/` and fixture-owned example data, but dashboard display and Studio/cockpit display are read-only consumers. They must not create hidden background agents, execute browser-side commands, promote regressions, mutate source, or write trusted files.

## Known gaps and out-of-scope behavior

Scenario Coverage v12 is not a production-ready multi-agent runtime, hosted service, Godot replacement, or public launch milestone. The following remain explicitly out of scope:

- no hidden background agents;
- no unbounded spawning;
- no auto-apply, auto-merge, or self-approval;
- no browser trusted writes or command bridge;
- no remote worker pool, hosted worker pool, cloud worker pool, or remote/cloud swarm;
- no dependency mutation, CI mutation, or workflow mutation;
- no production-ready claim and no Godot replacement claim.

## Verification commands

Run these commands for MAP13.16.3 and future compatibility checks:

```sh
gh issue view 679 --repo shaun0927/Ouroforge --json number,state,title
gh issue view 1 --repo shaun0927/Ouroforge --json number,state,title
gh issue view 23 --repo shaun0927/Ouroforge --json number,state,title
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ouroforge-core scenario_coverage_v12_
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
```
