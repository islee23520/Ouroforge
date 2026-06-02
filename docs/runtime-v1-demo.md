# Runtime v1 playable demo evidence

Issue #67 integrates the completed Runtime v1 feature set into one small local
browser demo. It composes existing capabilities only; it does not add new engine,
editor, server, plugin, native export, or production-readiness scope.

## Command

```bash
cargo run -p ouroforge-cli -- seed validate seeds/runtime-v1-demo.yaml
cargo run -p ouroforge-cli -- run seeds/runtime-v1-demo.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

`examples/evidence-dashboard/dashboard-data.json` is generated local state and
must not be committed.

## Feature coverage

`seeds/runtime-v1-demo.yaml` proves the completed Runtime v1 capabilities:

- scene schema v1 through the `foundation-scene` runtime data;
- deterministic input replay via `demo-move-to-goal`;
- collision through the `goal:player` contact assertion;
- snapshot/restore through the `goal-contact` branch;
- local static assets through loaded `assets/sprites/...` metadata;
- fixed-timestep sprite-frame animation through animation state assertions;
- evidence-first audio through the `player_spawn` event log.

No Runtime v1 feature is intentionally omitted: #60 through #66 were completed
before this integration issue. The demo intentionally avoids production maturity,
polished content, native export, editor subsystem, plugin, marketplace, server,
database, cloud, or Elixir/distributed claims.

## Fresh evidence

Fresh #67.1 run evidence:

- Run: `runs/run-1780362082701-91050`
- Browser workers: 4/4 passed
- Scenarios: 1/1 passed
- Verdict: `passed`
- Scenario artifacts:
  - `evidence/scenarios/runtime-v1-composed-demo/input-replay-1780362088708.json`
  - `evidence/scenarios/runtime-v1-composed-demo/snapshot-goal-contact-1780362089011.json`
  - `evidence/scenarios/runtime-v1-composed-demo/restore-goal-contact-1780362089023.json`
  - `evidence/scenarios/runtime-v1-composed-demo/world-state-1780362088708.json`
  - `evidence/scenarios/runtime-v1-composed-demo/frame-stats-1780362089054.json`
  - `evidence/scenarios/runtime-v1-composed-demo/scenario-result-1780362089057.json`

Post-merge #67.1 verification also passed on main with run
`runs/run-1780362145064-20895`.

## Dashboard and cockpit compatibility

The evidence dashboard remains read-only: export the generated dashboard data and
open `examples/evidence-dashboard/` through a local static server to inspect the
Runtime v1 demo run. The dashboard consumes generated `dashboard-data.json` only.

The authoring cockpit remains compatible with the Runtime v1 scene data because
it loads `examples/game-runtime/scene.json`, renders the scene tree/inspector, and
continues to route persistence through Rust-validated `ouroforge scene edit`
commands. The cockpit intentionally supports only its current editable scalar
fields; animation/audio/asset management UI is not introduced by #67.
