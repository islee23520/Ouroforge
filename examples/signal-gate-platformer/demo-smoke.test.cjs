const assert = require("assert");
const fs = require("fs");
const path = require("path");

const root = path.resolve(__dirname, "../..");
const exampleRoot = path.join(root, "examples/signal-gate-platformer");

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(root, relativePath), "utf8"));
}

function assertRepoRef(relativePath) {
  assert(
    fs.existsSync(path.join(root, relativePath)),
    `expected repo ref to exist: ${relativePath}`,
  );
}

const manifest = readJson("examples/signal-gate-platformer/ouroforge.project.json");
assert.strictEqual(manifest.schemaVersion, "project-manifest-v1");
assert.strictEqual(manifest.project.id, "signal_gate_platformer_demo");
for (const group of [manifest.scenes, manifest.seeds, manifest.scenarioPacks]) {
  for (const ref of group) {
    assertRepoRef(`examples/signal-gate-platformer/${ref.path}`);
  }
}

const scene = readJson("examples/signal-gate-platformer/scenes/signal-gate-platformer.scene.json");
assert.strictEqual(scene.id, "signal-gate-platformer-scene");
assert(scene.entities.some((entity) => entity.id === "player"));
assert(scene.entities.some((entity) => entity.id === "hazard_lane"));
assert(scene.entities.some((entity) => entity.id === "signal_gate"));
assert(scene.entities.some((entity) => entity.id === "exit"));
assert.strictEqual(scene.evidenceIntent.world_state.goalFlags.signal_open, true);
assert.strictEqual(scene.evidenceIntent.world_state.goalFlags.hazard_cleared, true);
assert.strictEqual(scene.evidenceIntent.world_state.goalFlags.exit_reached, true);

const loop = readJson("examples/signal-gate-platformer/fixtures/loop/signal-gate-loop-run.fixture.json");
assert.deepStrictEqual(loop.loopShape, ["seed", "build", "observe", "verify", "journal", "evolve"]);
for (const stage of loop.stages) {
  assertRepoRef(stage.artifactRef);
}

assert(fs.existsSync(path.join(exampleRoot, "fixtures/journal/signal-gate-journal.fixture.md")));
