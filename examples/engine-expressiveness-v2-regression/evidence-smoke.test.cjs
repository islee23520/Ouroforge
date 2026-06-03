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
    fetch: () => Promise.reject(new Error('fetch disabled in expressiveness v2 evidence smoke')),
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
  if (Object.prototype.hasOwnProperty.call(contract, 'countGreaterThan')) {
    const count = Array.isArray(actual) ? actual.length : actual && typeof actual === 'object' ? Object.keys(actual).length : null;
    return typeof count === 'number' && count > contract.countGreaterThan;
  }
  if (Object.prototype.hasOwnProperty.call(contract, 'contains')) {
    return String(actual).includes(String(contract.contains));
  }
  throw new Error(`unsupported smoke operator in ${JSON.stringify(assertion)}`);
}

const scene = JSON.parse(fs.readFileSync(path.join(fixtureDir, 'scenes', 'expressiveness-v2.scene.json'), 'utf8'));
const pack = JSON.parse(fs.readFileSync(path.join(fixtureDir, 'scenarios', 'expressiveness-v2-regression.json'), 'utf8'));
const scenarios = pack.scenarioGroups.flatMap((group) => group.scenarios.map((scenario) => ({ groupId: group.id, ...scenario })));

const api = createRuntime();
let state = api.loadScene(scene);
api.setInput({ right: true });
state = api.step(90);
api.setInput({ right: false });

const evidence = {
  world_state: state,
  frame_stats: { fixedDeltaMs: state.fixedDeltaMs, tick: state.tick },
  runtime_events: { events: api.getEvents() },
  collision_evidence: { contacts: state.collisions, events: state.collisionEvents },
  animation_evidence: state.entities
    .filter((entity) => entity.components && entity.components.animation)
    .map((entity) => ({ entityId: entity.id, components: { animation: entity.components.animation } })),
  audio_evidence: state.audioEvents,
  comparison: {
    schemaVersion: 'scenario-coverage-v3-smoke',
    scenarioPackId: pack.id,
    scenarios: scenarios.map((scenario) => scenario.id),
  },
};

assert.equal(state.sceneId, 'engine-expressiveness-v2-regression-scene');
assert.equal(state.componentModel.goalFlags.key_collected, true);
assert.equal(state.componentModel.goalFlags.door_open, true);
assert.equal(state.componentModel.goalFlags.exit_reached, true);
assert.ok(evidence.runtime_events.events.some((event) => event.type === 'runtime.trigger.entered'));
assert.ok(evidence.runtime_events.events.some((event) => event.type === 'runtime.animation.state'));

const verdicts = scenarios.map((scenario) => ({
  scenarioId: scenario.id,
  groupId: scenario.groupId,
  evidence: {
    world_state: 'evidence/world-state.json',
    frame_stats: 'evidence/frame-stats.json',
    runtime_events: 'evidence/runtime-events.json',
    collision_evidence: 'evidence/collision-evidence.json',
    animation_evidence: 'evidence/animation-evidence.json',
    audio_evidence: 'evidence/audio-evidence.json',
    comparison: 'evidence/comparison.json',
  },
  assertions: scenario.assertions.map((assertion) => ({ assertion, passed: evaluateAssertion(evidence, assertion) })),
}));

for (const verdict of verdicts) {
  assert.ok(Object.values(verdict.evidence).every((ref) => ref.startsWith('evidence/')), `${verdict.scenarioId} evidence refs are relative`);
  for (const assertion of verdict.assertions) {
    assert.equal(assertion.passed, true, `${verdict.scenarioId} failed ${JSON.stringify(assertion.assertion)}`);
  }
}

const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-expressiveness-v2-'));
const tempEvidence = path.join(tempDir, 'verdicts.json');
fs.writeFileSync(tempEvidence, JSON.stringify({ schemaVersion: 'scenario-coverage-v3-smoke', verdicts }, null, 2));
assert.equal(JSON.parse(fs.readFileSync(tempEvidence, 'utf8')).verdicts.length, 8);
fs.rmSync(tempDir, { recursive: true, force: true });

for (const generatedRoot of ['runs', 'dashboard-data', 'target']) {
  assert.equal(fs.existsSync(path.join(fixtureDir, generatedRoot)), false, `${generatedRoot} must remain untracked`);
}

console.log('engine expressiveness v2 evidence smoke passed');
