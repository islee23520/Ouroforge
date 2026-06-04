# 3D Demo Scene v1 capability gate

This fixture demonstrates a tiny, deterministic local 3D capability gate for
Ouroforge. It validates a source-like 3D scene graph, loads it through the
existing browser runtime harness, advances a short transform animation, emits
read-only probe/render/collision/camera/animation evidence, and evaluates the
scenario-pack assertions in a local smoke test.

Boundary: this demo is a bounded local 3D evidence fixture. It is not a
production 3D engine, not broad 3D compatibility evidence, and not a Godot
replacement. It does not add native export, advanced lighting/PBR/material
graphs, broad import pipelines, hosted/cloud behavior, plugin runtime, browser
trusted writes, command bridges, auto-apply, or auto-merge behavior.

## Source-like fixture files

- `ouroforge.project.json` binds the scene, Seed, scenario pack, asset root, and
  generated-state roots for this example.
- `scenes/bounded-3d-demo.scene.json` defines the active camera, primitive cube
  meshes/materials, transform hierarchy, dynamic player collider, trigger
  collider, and one bounded translation animation clip.
- `seeds/3d-demo-scene-v1.yaml` records the scenario acceptance criteria and
  3D evidence assertions.
- `scenarios/3d-demo-scene-v1.json` mirrors the Seed assertions as a scenario
  pack for local evidence checks.
- `evidence-smoke.test.cjs` is the deterministic runtime/evaluator-style smoke
  test. It writes only temporary OS-local evidence and removes it before exit.

## Fresh-clone validation commands

Run from the repository root:

```bash
cargo run -p ouroforge-cli -- seed validate examples/3d-demo-scene-v1/seeds/3d-demo-scene-v1.yaml
cargo run -p ouroforge-cli -- project validate examples/3d-demo-scene-v1
node --check examples/3d-demo-scene-v1/evidence-smoke.test.cjs
node examples/3d-demo-scene-v1/evidence-smoke.test.cjs
```

The smoke should print:

```text
3d demo scene v1 evidence smoke passed
```

Optional generated-run inspection, also from the repository root:

```bash
cargo run -p ouroforge-cli -- run \
  examples/3d-demo-scene-v1/seeds/3d-demo-scene-v1.yaml \
  --project examples/3d-demo-scene-v1 \
  --scenario-pack 3d-demo-scene-v1
```

`run` creates a local `/runs/` artifact tree in the current worktree. Keep it
untracked and remove it after inspection if you do not need the generated
artifacts:

```bash
rm -rf runs dashboard-data target
```

## Expected smoke evidence

The Node smoke loads `bounded-3d-demo.scene.json`, confirms the animation starts
at frame 0, advances the scenario wait step by 4 frames, and checks:

- `scene3d_probe.status == present` and `nodeCount > 0`;
- transform evidence includes the source-like 3D nodes;
- render smoke sees both primitive demo meshes;
- collision evidence records the animated player cube entering the trigger;
- camera evidence reports `demo-camera` as active;
- animation evidence reports `player-slide-to-trigger` at the final frame;
- evidence references in the generated temporary verdict are relative
  `evidence/...` paths.

All emitted browser-runtime evidence is read-only inspection input. Rust/local
validation remains the trusted source-like fixture validation and CLI contract
owner.

## Generated-state policy

Tracked source-like fixture files live under `examples/3d-demo-scene-v1/`.
Generated output must stay untracked:

- `runs/`
- `dashboard-data/`
- `target/`
- `tmp/`
- screenshots or browser-local probe captures

Before sending evidence or closing the issue, run:

```bash
git status --short --ignored
```

A clean audit may show ignored build output such as `!! target/`, but it must not
show generated 3D runs, screenshots, dashboard data, temp projects, or local tool
state as tracked changes.
