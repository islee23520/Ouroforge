# Asset Pipeline v1 Regression Fixture

This fixture is the AP1.10.1 source-only coverage matrix for issue #341,
Scenario Coverage v4. It records deterministic local asset inputs for manifest
validation, hash mismatch, missing asset, atlas frame bounds, tile collision and
trigger extraction, runtime asset load observability, and future dashboard/Studio
read-model compatibility smokes.

The fixture does not implement new asset behavior. It is source-like test data
plus scenario/Seed contracts only. Generated `runs/`, `dashboard-data/`, asset
previews, screenshots, and local tool state must remain untracked.

Suggested focused validation:

```bash
cargo run -p ouroforge-cli -- project validate examples/asset-pipeline-v1-regression/ouroforge.project.json
cargo run -p ouroforge-cli -- seed validate examples/asset-pipeline-v1-regression/seeds/asset-pipeline-regression.yaml
cargo run -p ouroforge-cli -- asset validate examples/asset-pipeline-v1-regression/asset-manifest.json
cargo test asset_pipeline_v1_regression --lib
```


AP1.10.2 adds a smoke that executes the fixture in-process, evaluates every
scenario-pack assertion against bounded runtime evidence, writes temporary
evidence/verdict-shaped output outside the repository, and deletes it before
exit:

```bash
node examples/asset-pipeline-v1-regression/evidence-smoke.test.cjs
```

Invalid manifests under `invalid/` are deliberate regression fixtures and should
continue to fail with explicit Rust validation errors.
