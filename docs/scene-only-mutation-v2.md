# Scene-Only Safe Mutation Application

Scene-only mutation application is the safe Evolve Loop v2 bridge from mutation proposals to controlled scene data edits.

## Contract

- Applies scene data only; arbitrary source-code patch application remains unsupported.
- Requires an existing mutation proposal id.
- Requires an allowed scene edit path from `SUPPORTED_SCENE_EDIT_PATHS`.
- Requires `expectedBeforeSceneHash` to match the current target scene hash.
- Requires Rust scene validation before a transaction can be successful.
- Writes a scene edit transaction artifact for successful application and validation-failed candidate edits.
- Records successful applications in `mutation/scene-applications.json` and the ledger as `mutation.scene_applied`.
- Does not auto-accept, auto-merge, or write mutation review decisions.
- Browser/dashboard surfaces remain read-only.
- Project-scoped application is documented in `docs/project-mutation-loop-v1.md` and requires manifest authorization when `--project` is used.

## Operation shape

```json
{
  "schemaVersion": "scene-only-mutation-v1",
  "proposalId": "mutation-...",
  "targetScenePath": "path/to/scene.json",
  "edit": {
    "entity_id": "player",
    "path": "components.transform.x",
    "value": 48
  },
  "expectedBeforeSceneHash": {
    "algorithm": "fnv1a64-canonical-json-v1",
    "value": "..."
  },
  "validationRequired": true
}
```

## CLI flow

```bash
cargo run -p ouroforge-cli -- mutation apply-scene <run-dir> \
  --operation <operation.json> \
  --transaction-output <transaction.json>

# Project-scoped variant:
cargo run -p ouroforge-cli -- mutation apply-scene <run-dir> \
  --project <project-root-or-ouroforge.project.json> \
  --operation <operation.json> \
  --transaction-output <transaction.json>
```

The command prints the transaction id, transaction path, before/after scene hashes, and a manual next QA command. The caller remains responsible for running QA and reviewing results.

## AL2.6.4 integration evidence

Commands run:

```bash
cargo fmt --check
cargo test --workspace
cargo run -p ouroforge-cli -- seed validate seeds/evolve-v1-demo.yaml
cargo run -p ouroforge-cli -- run seeds/evolve-v1-demo.yaml --workers 4
cargo run -p ouroforge-cli -- mutation apply-scene runs/run-1780406033875-78128 \
  --operation .omx/tmp/scene-mutation-v2/operation.json \
  --transaction-output .omx/tmp/scene-mutation-v2/transaction.json
cargo run -p ouroforge-cli -- run seeds/evolve-v1-demo.yaml --workers 4 \
  --transaction .omx/tmp/scene-mutation-v2/transaction.json
cargo run -p ouroforge-cli -- compare runs/run-1780406033875-78128 runs/run-1780406043645-78491 \
  --output-dir runs/run-1780406043645-78491/comparisons
cargo run -p ouroforge-cli -- dashboard export --runs-root runs \
  --output .omx/tmp/scene-mutation-v2/dashboard-data.json
cargo run -p ouroforge-cli -- scene validate examples/game-runtime/scene.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
cargo clippy --all-targets --all-features -- -D warnings
```

Evidence refs:

- Baseline run: `runs/run-1780406033875-78128`
- Applied transaction: `.omx/tmp/scene-mutation-v2/transaction.json`
- Transaction id: `scene-edit-16990308561785380071`
- Before scene hash: `4021413abac59b75`
- After scene hash: `9cd7b8934e1c4bfe`
- Transaction-bound rerun: `runs/run-1780406043645-78491`
- Comparison artifact: `runs/run-1780406043645-78491/comparisons/run-comparison-run-1780406033875-78128--run-1780406043645-78491.json`

Top semantic compare reasons:

- `[changed] evidence_artifacts: 31 evidence artifacts added, 32 removed`
- `[changed] transaction_provenance: scene edit transaction provenance changed`

Generated artifacts were left under `runs/` and `.omx/tmp/scene-mutation-v2/` and remain untracked.
