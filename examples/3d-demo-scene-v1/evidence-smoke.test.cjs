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
    fetch: () => Promise.reject(new Error('fetch disabled in 3d demo evidence smoke')),
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
  if (Object.prototype.hasOwnProperty.call(contract, 'greaterThan')) {
    return typeof actual === 'number' && actual > contract.greaterThan;
  }
  throw new Error(`unsupported smoke operator in ${JSON.stringify(assertion)}`);
}

const scene = JSON.parse(fs.readFileSync(path.join(fixtureDir, 'scenes', 'bounded-3d-demo.scene.json'), 'utf8'));
const pack = JSON.parse(fs.readFileSync(path.join(fixtureDir, 'scenarios', '3d-demo-scene-v1.json'), 'utf8'));
const scenarios = pack.scenarioGroups.flatMap((group) => group.scenarios.map((scenario) => ({ groupId: group.id, ...scenario })));
assert.equal(scenarios.length, 1, '3D demo smoke stays a tiny single-scenario gate');

const api = createRuntime();
let state = api.loadScene(scene);
assert.equal(state.sceneId, 'bounded-3d-demo');
assert.equal(state.sceneKind, '3d');
assert.equal(state.scene3dAnimation.states[0].currentFrame, 0, 'demo starts before bounded transform animation runs');
assert.equal(state.scene3dCollision.triggerCount, 0, 'demo does not start inside the trigger volume');

for (const step of scenarios[0].steps) {
  if (step.wait && Number.isFinite(step.wait.frames)) {
    state = api.step(step.wait.frames);
  } else {
    throw new Error(`unsupported 3D demo smoke step ${JSON.stringify(step)}`);
  }
}

const evidence = {
  world_state: state,
  scene3d_probe: state.scene3dProbe,
  scene3d_transform: state.scene3dTransforms,
  scene3d_render: state.scene3dRender,
  scene3d_collision: state.scene3dCollision,
  scene3d_camera: state.scene3dCamera,
  scene3d_animation: state.scene3dAnimation,
  runtime_events: { events: api.getEvents() },
  frame_stats: api.getFrameStats(),
};

assert.equal(evidence.scene3d_probe.status, 'present');
assert.equal(evidence.scene3d_probe.nodeCount, 4);
assert.equal(evidence.scene3d_render.visibleObjectCount, 2, 'both primitive demo meshes are render-smoke visible');
assert.equal(evidence.scene3d_collision.triggerCount, 1, 'animated player cube reaches goal trigger');
assert.equal(evidence.scene3d_camera.activeCameraId, 'demo-camera');
assert.equal(evidence.scene3d_animation.states[0].clipId, 'player-slide-to-trigger');
assert.equal(evidence.scene3d_animation.states[0].currentFrame, 4);
assert.equal(evidence.scene3d_animation.states[0].playing, false, 'bounded non-looping demo clip stops at the target frame');
assert.ok(evidence.runtime_events.events.some((event) => event.type === 'runtime.scene3d.animation.state' && event.payload.clipId === 'player-slide-to-trigger'));
assert.ok(evidence.runtime_events.events.some((event) => event.type === 'runtime.scene3d.collision.trigger'));
assert.match(evidence.scene3d_render.boundary, /no WebGPU, GLTF import, PBR, remote fetch, or production renderer claim/);
assert.match(evidence.scene3d_collision.boundary, /no full 3D physics engine/);
assert.match(evidence.scene3d_animation.boundary, /no skeletal authoring/);

const verdicts = scenarios.map((scenario) => ({
  scenarioId: scenario.id,
  groupId: scenario.groupId,
  status: 'passed',
  evidence: {
    world_state: 'evidence/world-state.json',
    scene3d_probe: 'evidence/scene3d-probe.json',
    scene3d_transform: 'evidence/scene3d-transform.json',
    scene3d_render: 'evidence/scene3d-render.json',
    scene3d_collision: 'evidence/scene3d-collision.json',
    scene3d_camera: 'evidence/scene3d-camera.json',
    scene3d_animation: 'evidence/scene3d-animation.json',
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
  assert.equal(verdict.status, 'passed');
}

const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-3d-demo-scene-v1-'));
const tempResult = path.join(tempDir, 'scenario-result.json');
fs.writeFileSync(tempResult, JSON.stringify({
  schemaVersion: 'scenario-coverage-v3-smoke',
  scenarioPackId: pack.id,
  trustedWriter: 'local-node-smoke-temp-only',
  boundary: 'Temporary smoke artifact only; not trusted persistence, browser command bridge, production 3D readiness, or Godot replacement evidence.',
  verdicts,
}, null, 2));
const parsedResult = JSON.parse(fs.readFileSync(tempResult, 'utf8'));
assert.equal(parsedResult.verdicts[0].status, 'passed');
assert.equal(parsedResult.verdicts[0].assertions.length, scenarios[0].assertions.length);
fs.rmSync(tempDir, { recursive: true, force: true });

for (const generatedRoot of ['runs', 'dashboard-data', 'target', 'tmp']) {
  assert.equal(fs.existsSync(path.join(fixtureDir, generatedRoot)), false, `${generatedRoot} must remain untracked in the demo fixture`);
}

console.log('3d demo scene v1 evidence smoke passed');
