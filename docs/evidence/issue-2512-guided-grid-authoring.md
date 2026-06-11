# Issue #2512 guided grid-puzzle authoring evidence

Closure classification: product-observed complete for the bounded local guided CLI authoring path only.

This evidence closes the #2497 raw JSON/YAML authoring gap by adding and using a
tracked local CLI path that generates the Dogfood Game 2 grid-puzzle authoring
artifacts without manual raw file editing as the primary workflow.

## Guided product surface

The product surface is the Rust/local CLI gate:

```bash
CARGO_TARGET_DIR=/private/tmp/ouroforge-target-2512 \
  cargo run -p ouroforge-cli -- \
  dogfood guided-grid-puzzle \
  examples/playable-demo-v2/crate-garden-guided-2512 \
  --project-id crate-garden-guided-2512 \
  --title 'Crate Garden Guided 2512' \
  --issue 2512
```

The command generated all scoped authoring artifacts:

- `examples/playable-demo-v2/crate-garden-guided-2512/seeds/crate-garden-guided-2512.yaml`
- `examples/playable-demo-v2/crate-garden-guided-2512/ouroforge.project.json`
- `examples/playable-demo-v2/crate-garden-guided-2512/scenes/before-review.scene.json`
- `examples/playable-demo-v2/crate-garden-guided-2512/scenes/crate-garden-guided-2512.scene.json`
- `examples/playable-demo-v2/crate-garden-guided-2512/scenarios/crate-garden-guided-2512-core.json`
- `examples/playable-demo-v2/crate-garden-guided-2512/review/review-apply-decision.json`
- `examples/playable-demo-v2/crate-garden-guided-2512/playtest/playtest-backlog.json`

The review/apply decision records `rawJsonManualSteps: []`,
`rawFileEditingRequired: false`, `guidedCliGenerated: true`, and preserves the
trusted-write boundary (`browserTrustedWrite=false`, `commandBridge=false`,
`autoApply=false`, `autoMerge=false`). The playtest backlog has no residual raw
JSON authoring finding and does not claim human fun/feel judgment.

## Validator evidence

```bash
CARGO_TARGET_DIR=/private/tmp/ouroforge-target-2512 \
  cargo run -p ouroforge-cli -- project validate \
  examples/playable-demo-v2/crate-garden-guided-2512/ouroforge.project.json

CARGO_TARGET_DIR=/private/tmp/ouroforge-target-2512 \
  cargo run -p ouroforge-cli -- seed validate \
  examples/playable-demo-v2/crate-garden-guided-2512/seeds/crate-garden-guided-2512.yaml
```

Observed results:

```text
Project manifest valid: crate-garden-guided-2512
Seed valid: crate-garden-guided-2512
```

The CLI integration test also locks this generated artifact contract:

```bash
CARGO_TARGET_DIR=/private/tmp/ouroforge-target-2512 \
  cargo test -p ouroforge-cli --test artifact_commands \
  dogfood_guided_grid_puzzle_generates_authoring_artifacts_without_raw_editing
```

## Live browser before/after evidence

A local static server served the repository and the live observability runner
drove the generated scenes through the browser runtime using `--replay grid-puzzle`.
Generated bundles are ignored under `runs/issue-2512/guided-authoring/`.

Before-review scene:

```text
runs/issue-2512/guided-authoring/issue-2512-guided-before
final goal flags: {"grid_status":"playing","grid_won":false,"move_count":1}
Mechanical contract: contract-fail
Product observation: product-observed FAIL
```

After-review scene:

```text
runs/issue-2512/guided-authoring/issue-2512-guided-after
final goal flags: {"grid_status":"won","grid_won":true,"move_count":4}
Mechanical contract: contract-pass
Product observation: product-observed complete
```

Concrete generated artifact refs:

- `runs/issue-2512/guided-authoring/issue-2512-guided-before/input-replay.json`
- `runs/issue-2512/guided-authoring/issue-2512-guided-before/verdict.md`
- `runs/issue-2512/guided-authoring/issue-2512-guided-before/screenshots/start.png`
- `runs/issue-2512/guided-authoring/issue-2512-guided-before/screenshots/final.png`
- `runs/issue-2512/guided-authoring/issue-2512-guided-after/input-replay.json`
- `runs/issue-2512/guided-authoring/issue-2512-guided-after/verdict.md`
- `runs/issue-2512/guided-authoring/issue-2512-guided-after/screenshots/start.png`
- `runs/issue-2512/guided-authoring/issue-2512-guided-after/screenshots/final.png`

## Scope and guardrails

No browser trusted write, command bridge, self-approval, auto-apply, auto-merge,
hosted collaboration, signing/upload/store/export expansion, or Godot parity
claim was added. The trusted write remains the explicit Rust/local CLI command.
Generated screenshots, browser profiles, and observability bundles stay under
ignored `runs/` roots.

#1 and #23 remain open. #2496 remains open and untouched for human fun/feel
playtest notes.
