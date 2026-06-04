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
    fetch: () => Promise.reject(new Error('fetch disabled in 3d probe animation evaluator smoke')),
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
  const [[target, contract]] = Object.entries(assertion);
  const actual = getPath(evidence[target], contract.path);
  if (Object.prototype.hasOwnProperty.call(contract, 'equals')) return actual === contract.equals;
  if (Object.prototype.hasOwnProperty.call(contract, 'exists')) return contract.exists ? actual !== undefined : actual === undefined;
  throw new Error(`unsupported probe/animation operator in ${JSON.stringify(assertion)}`);
}

const scene = JSON.parse(fs.readFileSync(path.join(fixtureDir, 'scenes', 'probe-animation-regression.scene.json'), 'utf8'));
const pack = JSON.parse(fs.readFileSync(path.join(fixtureDir, 'scenarios', '3d-core-regression-v8.json'), 'utf8'));
const scenario = pack.scenarioGroups
  .flatMap((group) => group.scenarios.map((entry) => ({ groupId: group.id, ...entry })))
  .find((entry) => entry.id === 'probe-animation-evaluator-regression');
assert.ok(scenario, 'probe/animation/evaluator scenario exists');

const api = createRuntime();
let state = api.loadScene(scene);
assert.equal(state.scene3dProbe.status, 'present');
assert.equal(state.scene3dAnimation.states[0].currentFrame, 0);
assert.equal(state.scene3dCollision.triggerCount, 0);
for (const step of scenario.steps) {
  if (step.wait && Number.isFinite(step.wait.frames)) state = api.step(step.wait.frames);
  else throw new Error(`unsupported scenario step ${JSON.stringify(step)}`);
}

const evidence = {
  scene3d_probe: state.scene3dProbe,
  scene3d_animation: state.scene3dAnimation,
  scene3d_collision: state.scene3dCollision,
  runtime_events: { events: api.getEvents() },
  frame_stats: api.getFrameStats(),
};
assert.equal(evidence.scene3d_probe.status, 'present');
assert.equal(evidence.scene3d_probe.animationStateCount, 1);
assert.equal(evidence.scene3d_animation.states[0].clipId, 'animated-cube-to-trigger');
assert.equal(evidence.scene3d_animation.states[0].currentFrame, 3);
assert.equal(evidence.scene3d_animation.states[0].playing, false);
assert.equal(evidence.scene3d_collision.triggerCount, 1);
assert.ok(evidence.runtime_events.events.some((event) => event.type === 'runtime.scene3d.animation.state'));
assert.ok(evidence.runtime_events.events.some((event) => event.type === 'runtime.scene3d.collision.trigger'));
assert.match(evidence.scene3d_probe.boundary, /not trusted persistence/);
assert.match(evidence.scene3d_animation.boundary, /no skeletal authoring/);

const passedAssertions = scenario.assertions.map((assertion) => ({ assertion, passed: evaluateAssertion(evidence, assertion) }));
const negativeAssertions = [{ scene3d_probe: { path: 'status', equals: 'unavailable' } }];
const failedAssertions = negativeAssertions.map((assertion) => ({ assertion, passed: evaluateAssertion(evidence, assertion) }));
assert.ok(passedAssertions.every((entry) => entry.passed), 'all positive assertions pass');
assert.ok(failedAssertions.every((entry) => !entry.passed), 'deliberate negative assertions fail');

const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-3d-probe-animation-v8-'));
const tempResult = path.join(tempDir, 'scenario-result.json');
fs.writeFileSync(tempResult, JSON.stringify({
  schemaVersion: 'scenario-coverage-v8-probe-animation-smoke',
  scenarioId: scenario.id,
  status: passedAssertions.every((entry) => entry.passed) && failedAssertions.every((entry) => !entry.passed) ? 'passed' : 'failed',
  boundary: 'Temporary smoke artifact only; not trusted persistence, production 3D readiness, or Godot replacement evidence.',
  assertions: passedAssertions,
  negativeAssertions: failedAssertions,
}, null, 2));
const parsed = JSON.parse(fs.readFileSync(tempResult, 'utf8'));
assert.equal(parsed.status, 'passed');
assert.equal(parsed.assertions.length, scenario.assertions.length);
assert.equal(parsed.negativeAssertions.length, negativeAssertions.length);
fs.rmSync(tempDir, { recursive: true, force: true });

for (const generatedRoot of ['runs', 'dashboard-data', 'target', 'tmp']) {
  assert.equal(fs.existsSync(path.join(fixtureDir, generatedRoot)), false, `${generatedRoot} must remain untracked in the regression fixture`);
}

console.log('scenario coverage v8 probe animation evaluator smoke passed');
