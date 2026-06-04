# Runtime Frame Budget v1

Runtime Frame Budget v1 is the P2D8.10.1 contract slice for #590. It defines a
Rust-trusted evidence shape for local frame timing budgets and debug counts
before runtime emission, evaluator compatibility, or dashboard/Studio panels are
implemented.

The contract records:

- `frameId`, `sceneId`, and optional `scenarioId`;
- `timings.updateMs`, `timings.renderMs`, `timings.evidenceMs`, and
  `timings.totalMs`;
- matching positive budget thresholds under `budget`;
- runtime debug counts for entities, draw calls, layers, collision pairs, active
  animations, active VFX, and audio events;
- `readOnlyInspection` guardrails for future browser/Studio display surfaces.

`runtime-debug frame-budget validate` reports the computed status and debug
counts. `runtime-debug frame-budget show` prints the validated JSON.

```bash
cargo run -p ouroforge-cli -- runtime-debug frame-budget validate examples/runtime-frame-budget-v1/valid/frame-budget.sample.json
cargo run -p ouroforge-cli -- runtime-debug frame-budget show examples/runtime-frame-budget-v1/violation/frame-budget.slow.json
```

This slice does not emit runtime evidence, add profiler accuracy claims, modify
browser runtime code, add dashboard/Studio panels, or authorize trusted browser
writes. Browser metrics remain bounded evidence inputs, not trusted authority.

Verification:

```bash
cargo test -p ouroforge-core --test runtime_frame_budget_contract
cargo test -p ouroforge-cli --test artifact_commands runtime_frame_budget_cli
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
```
