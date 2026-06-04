const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in 3d core regression smoke')),
    addEventListener: () => {},
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of scripts) {
    vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  }
  return context.__OUROFORGE__;
}

function getPath(value, dottedPath) {
  return String(dottedPath).split('.').reduce((cursor, segment) => {
    if (cursor === undefined || cursor === null) return undefined;
    if (Array.isArray(cursor) && /^\d+$/.test(segment)) return cursor[Number(segment)];
    return cursor[segment];
  }, value);
}

function evaluateAssertion(evidence, assertion) {
  const entries = Object.entries(assertion);
  assert.equal(entries.length, 1, 'assertion must have one target');
  const [target, contract] = entries[0];
  const actual = getPath(evidence[target], contract.path);
  if (Object.prototype.hasOwnProperty.call(contract, 'exists')) {
    return contract.exists ? actual !== undefined : actual === undefined;
  }
  if (Object.prototype.hasOwnProperty.call(contract, 'equals')) {
    return actual === contract.equals;
  }
  throw new Error(`unsupported core 3D smoke operator in ${JSON.stringify(assertion)}`);
}

const scene = JSON.parse(fs.readFileSync(path.join(fixtureDir, 'scenes', 'core-3d-regression.scene.json'), 'utf8'));
const pack = JSON.parse(fs.readFileSync(path.join(fixtureDir, 'scenarios', '3d-core-regression-v8.json'), 'utf8'));
const scenarios = pack.scenarioGroups.flatMap((group) => group.scenarios.map((scenario) => ({ groupId: group.id, ...scenario })));
const coreScenario = scenarios.find((scenario) => scenario.id === 'core-3d-transform-render-collision');
assert.ok(coreScenario, 'core 3D regression scenario remains present');
assert.ok(scenarios.some((scenario) => scenario.id === 'probe-animation-evaluator-regression'), 'probe animation evaluator scenario is packed alongside core coverage');

const api = createRuntime();
let state = api.loadScene(scene);
for (const step of coreScenario.steps) {
  if (step.wait && Number.isFinite(step.wait.frames)) {
    state = api.step(step.wait.frames);
  } else {
    throw new Error(`unsupported core 3D regression step ${JSON.stringify(step)}`);
  }
}

const evidence = {
  world_state: state,
  scene3d_transform: state.scene3dTransforms,
  scene3d_camera: state.scene3dCamera,
  scene3d_render: state.scene3dRender,
  scene3d_collision: state.scene3dCollision,
  runtime_events: { events: api.getEvents() },
  frame_stats: api.getFrameStats(),
};

const actorTransform = evidence.scene3d_transform.transforms.find((entry) => entry.nodeId === 'actor-cube');
const triggerTransform = evidence.scene3d_transform.transforms.find((entry) => entry.nodeId === 'trigger-cube');
assert.equal(state.sceneId, 'core-3d-regression');
assert.equal(state.sceneKind, '3d');
assert.equal(actorTransform.parentId, 'regression-root');
assert.equal(JSON.stringify(actorTransform.worldTransform.translation), JSON.stringify({ x: 7, y: 1, z: 0 }));
// The trigger must share the actor's world position so the trigger collision is
// coherent with the parent-applied transform hierarchy (not just local space).
assert.equal(triggerTransform.parentId, 'regression-root');
assert.equal(JSON.stringify(triggerTransform.worldTransform.translation), JSON.stringify({ x: 7, y: 1, z: 0 }), 'trigger world position must overlap the actor for a coherent trigger collision');
assert.equal(evidence.scene3d_camera.activeCameraId, 'regression-camera');
assert.equal(evidence.scene3d_camera.cameras[0].projection.kind, 'perspective');
assert.equal(evidence.scene3d_render.visibleObjectCount, 2);
assert.equal(
  JSON.stringify(evidence.scene3d_render.renderables.map((renderable) => [renderable.nodeId, renderable.meshRef, renderable.materialRef]).sort()),
  JSON.stringify([
    ['actor-cube', 'actor-cube-mesh', 'actor-green'],
    ['trigger-cube', 'trigger-cube-mesh', 'trigger-purple'],
  ]),
);
assert.equal(evidence.scene3d_collision.triggerCount, 1);
assert.equal(evidence.scene3d_collision.events[0].type, 'runtime.scene3d.collision.trigger');
assert.ok(evidence.runtime_events.events.some((event) => event.type === 'runtime.scene3d.collision.trigger'));
assert.match(evidence.scene3d_render.boundary, /production renderer claim/);
assert.match(evidence.scene3d_collision.boundary, /no full 3D physics engine/);

const legacy = createRuntime();
const legacyState = legacy.loadScene({ schemaVersion: '1', id: 'core-3d-regression-2d-compat', bounds: { width: 64, height: 64 }, entities: [] });
assert.equal(legacyState.sceneKind, '2d');
assert.equal(legacyState.scene3dRender.present, false);
assert.equal(legacyState.scene3dCollision.present, false);
assert.equal(typeof legacy.getFrameStats().tick, 'number', '2D frame stats remain compatible');

const verdicts = [coreScenario].map((scenario) => ({
  scenarioId: scenario.id,
  groupId: scenario.groupId,
  status: 'passed',
  evidence: {
    scene3d_transform: 'evidence/scene3d-transform.json',
    scene3d_camera: 'evidence/scene3d-camera.json',
    scene3d_render: 'evidence/scene3d-render.json',
    scene3d_collision: 'evidence/scene3d-collision.json',
    runtime_events: 'evidence/runtime-events.json',
    frame_stats: 'evidence/frame-stats.json',
  },
  assertions: scenario.assertions.map((assertion) => ({ assertion, passed: evaluateAssertion(evidence, assertion) })),
}));
for (const verdict of verdicts) {
  assert.ok(Object.values(verdict.evidence).every((ref) => ref.startsWith('evidence/')), `${verdict.scenarioId} evidence refs are relative`);
  for (const assertion of verdict.assertions) {
    assert.equal(assertion.passed, true, `${verdict.scenarioId} failed ${JSON.stringify(assertion.assertion)}`);
  }
}

const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-3d-core-regression-v8-'));
const tempResult = path.join(tempDir, 'scenario-result.json');
fs.writeFileSync(tempResult, JSON.stringify({
  schemaVersion: 'scenario-coverage-v8-core-smoke',
  scenarioPackId: pack.id,
  boundary: 'Temporary smoke artifact only; not trusted persistence, production 3D readiness, or Godot replacement evidence.',
  verdicts,
}, null, 2));
assert.equal(JSON.parse(fs.readFileSync(tempResult, 'utf8')).verdicts[0].assertions.length, scenarios[0].assertions.length);
fs.rmSync(tempDir, { recursive: true, force: true });

for (const generatedRoot of ['runs', 'dashboard-data', 'target', 'tmp']) {
  assert.equal(fs.existsSync(path.join(fixtureDir, generatedRoot)), false, `${generatedRoot} must remain untracked in the regression fixture`);
}

console.log('scenario coverage v8 core 3d smoke passed');
