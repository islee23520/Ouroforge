# Scenario/Evaluator v1 integration demo evidence

Issue #75 closes the Scenario/Evaluator v1 milestone by composing completed
Scenario/Evaluator capabilities into one evidence-native QA demo. The demo is a
composition artifact only: it does not add new evaluator behavior, UI redesign,
server/database/cloud storage, Playwright automation, AI semantic judging, or
Elixir/distributed orchestration.

## Commands

```bash
cargo run -p ouroforge-cli -- seed validate seeds/scenario-evaluator-v1-demo.yaml
cargo run -p ouroforge-cli -- run seeds/scenario-evaluator-v1-demo.yaml --workers 4
cargo run -p ouroforge-cli -- compare runs/<before-run> runs/<after-run>
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
```

`runs/` and `examples/evidence-dashboard/dashboard-data.json` are generated local
state and must not be committed.

## Feature coverage

`seeds/scenario-evaluator-v1-demo.yaml` composes completed Scenario/Evaluator v1
capabilities:

- replay binding through `replayRef` to `seeds/replays/scenario-replay-move-right.yaml`;
- console log evidence through bounded `console-log-*.json` artifacts;
- performance evidence through bounded `performance-metrics-*.json` artifacts;
- CDP trace summary evidence through `cdp-trace-summary-*.json` artifacts;
- richer assertions over world state, frame stats, runtime events, console errors,
  performance metrics, and collision evidence;
- visual checkpoint metadata and screenshot evidence for initial and post-replay
  states;
- multi-scenario suite ordering and aggregate summary evidence;
- before/after run comparison through the read-only `compare` command;
- verdict and journal generation through the existing Seed -> Run -> Evidence ->
  Evaluation -> Journal loop.

No Scenario/Evaluator v1 feature is intentionally omitted: #69 through #74 were
completed before this integration issue. The demo intentionally avoids upstream
feature implementation, UI redesign, AI semantic evaluation, production QA
platform claims, remote workers, release automation, and public compatibility
promises.

## Fresh evidence

Fresh #75 SE8.1 post-merge run evidence:

- Run: `runs/run-1780366027435-92590`
- Browser workers: 4/4 passed
- Scenarios: 2/2 passed
- Scenario order:
  1. `qa-initial-observability`
  2. `qa-replay-goal-contact`
- Verdict: `passed`
- Verdict summary: `2 scenario result(s) passed with consistent evidence.`
- Representative scenario artifacts:
  - `evidence/scenarios/qa-initial-observability/visual-checkpoint-initial-load-1780366035105.png`
  - `evidence/scenarios/qa-initial-observability/visual-checkpoint-initial-load-1780366035105.json`
  - `evidence/scenarios/qa-initial-observability/world-state-1780366035104.json`
  - `evidence/scenarios/qa-initial-observability/frame-stats-1780366036003.json`
  - `evidence/scenarios/qa-initial-observability/console-log-1780366036007.json`
  - `evidence/scenarios/qa-initial-observability/performance-metrics-1780366036024.json`
  - `evidence/scenarios/qa-initial-observability/cdp-trace-summary-1780366036025.json`
  - `evidence/scenarios/qa-initial-observability/scenario-result-1780366036027.json`
  - `evidence/scenarios/qa-replay-goal-contact/input-replay-1780366036029.json`
  - `evidence/scenarios/qa-replay-goal-contact/visual-checkpoint-post-replay-goal-contact-1780366036060.png`
  - `evidence/scenarios/qa-replay-goal-contact/visual-checkpoint-post-replay-goal-contact-1780366036060.json`
  - `evidence/scenarios/qa-replay-goal-contact/world-state-1780366036029.json`
  - `evidence/scenarios/qa-replay-goal-contact/frame-stats-1780366036126.json`
  - `evidence/scenarios/qa-replay-goal-contact/console-log-1780366036133.json`
  - `evidence/scenarios/qa-replay-goal-contact/performance-metrics-1780366036158.json`
  - `evidence/scenarios/qa-replay-goal-contact/cdp-trace-summary-1780366036159.json`
  - `evidence/scenarios/qa-replay-goal-contact/scenario-result-1780366036161.json`
  - `evidence/suite-summary-1780366036165.json`

Fresh SE8.1 before/after comparison evidence:

- Before run: `runs/run-1780365914089-76142`
- After run: `runs/run-1780365936784-60109`
- Comparison artifact:
  `runs/comparisons/run-comparison-run-1780365914089-76142--run-1780365936784-60109.json`
- Classification: `no_change`
- Supported deltas: all `0`

## Dashboard compatibility

The evidence dashboard remains read-only. Export generated data with:

```bash
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

Then run the dashboard smoke checks:

```bash
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
```

SE8.1 post-merge verification exported dashboard data successfully and the Node
syntax/smoke checks passed. The generated dashboard data file is intentionally
ignored by git.
