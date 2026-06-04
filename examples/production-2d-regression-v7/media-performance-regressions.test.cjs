const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function createRuntime(scene) {
  const context = {
    console,
    Image: function Image() {},
    URLSearchParams,
    location: { search: '' },
    document: { getElementById: () => null },
    fetch: () => Promise.resolve({ json: () => Promise.resolve(scene) }),
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

function assertNoGeneratedFixtureState() {
  for (const name of ['runs', 'target', 'dashboard-data']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} remains generated/untracked`);
  }
}

function mediaPerformanceScene() {
  const scene = clone(readJson('examples/game-runtime/scene.json'));
  scene.metadata = {
    ...(scene.metadata || {}),
    scenarioId: 'production-2d-media-performance-regression',
    runtimeDebug: {
      frameTimings: { updateMs: 3, renderMs: 18.5, evidenceMs: 1, totalMs: 24.25 },
      frameBudget: { updateMs: 8, renderMs: 16, evidenceMs: 4, totalMs: 20 },
    },
  };
  const player = scene.entities.find((entity) => entity.id === 'player');
  assert.ok(player, 'fixture scene has player entity');
  player.components.animation.stateClips = { idle: 'idle', run: 'run' };
  player.components.animation.clips.push({
    id: 'run',
    frameDuration: 1,
    loop: true,
    frames: [
      { color: '#38bdf8', asset: 'player-sprite' },
      { color: '#0284c7', asset: 'player-sprite' },
    ],
  });
  player.components.vfx = {
    emitters: [
      { id: 'run-dust', kind: 'trail', trigger: 'tick', particleCount: 4, lifetimeFrames: 6, color: '#94a3b8', asset: 'player-sprite', layer: 'actors' },
      { id: 'clamped-burst', kind: 'burst', trigger: 'tick', particleCount: 999, lifetimeFrames: 999, color: '#ffffff' },
    ],
  };
  player.components.audio = {
    buses: [
      { id: 'sfx', kind: 'sound', volume: 80, muted: false },
      { id: 'music', kind: 'music', volume: 45, muted: true },
    ],
    events: [
      { name: 'player_spawn', trigger: 'scene_loaded', action: 'play', kind: 'sound', bus: 'sfx', asset: 'player-spawn-audio' },
      { name: 'music_hold', trigger: 'scene_loaded', action: 'stop', kind: 'music', bus: 'music', asset: 'player-spawn-audio' },
    ],
  };
  return scene;
}

async function animationVfxAudioFrameBudgetRegression() {
  const scene = mediaPerformanceScene();
  const api = createRuntime(scene);
  let state = api.loadScene(scene);
  assert.equal(state.runtimeFrameBudget.schemaVersion, 'ouroforge.runtime-frame-budget.v1');
  assert.equal(state.runtimeFrameBudget.status, 'violated');
  assert.deepEqual(Array.from(state.runtimeFrameBudget.violations, (violation) => violation.field), ['renderMs', 'totalMs']);
  assert.equal(state.runtimeFrameBudget.authority, 'browser_runtime_evidence_input_not_profiler_truth');
  assert.deepEqual(Array.from(state.runtimeFrameBudget.readOnlyInspection.disallowedActions), ['trusted writes', 'command bridge', 'live mutation']);

  const loadEvents = api.getEvents();
  const spawnAudio = loadEvents.find((event) => event.type === 'runtime.audio.emitted' && event.payload.name === 'player_spawn');
  assert.ok(spawnAudio, 'scene load emits audio intent evidence');
  assert.equal(spawnAudio.payload.playback, 'intent');
  assert.ok(spawnAudio.payload.limitationWarnings.includes('audible_output_not_verified'));

  api.setInput({ right: true });
  state = api.step(1);
  const player = state.entities.find((entity) => entity.id === 'player');
  assert.equal(player.components.animation.state.activeState, 'run');
  assert.equal(player.components.animation.state.currentClip, 'run');
  assert.equal(state.vfxEvents.length, 2);
  const clamped = state.vfxEvents.find((event) => event.emitterId === 'clamped-burst');
  assert.equal(clamped.particleCount, 64);
  assert.equal(clamped.lifetimeFrames, 120);
  assert.equal(state.runtimeFrameBudget.counts.activeAnimationCount, 1);
  assert.equal(state.runtimeFrameBudget.counts.activeVfxCount, 2);
  assert.ok(state.runtimeFrameBudget.counts.audioEventCount >= 2);

  const tickEvents = api.getEvents();
  assert.ok(tickEvents.some((event) => event.type === 'runtime.animation.state' && event.payload.activeState === 'run'));
  assert.ok(tickEvents.some((event) => event.type === 'runtime.vfx.emitted' && event.payload.emitterId === 'run-dust'));

  const frameStats = api.getFrameStats();
  assert.equal(frameStats.runtimeFrameBudgetStatus, 'violated');
  assert.equal(frameStats.runtimeFrameBudgetViolationCount, 2);
  assert.equal(frameStats.runtimeFrameBudgetCounts.activeAnimationCount, 1);
  assert.equal(frameStats.runtimeFrameBudgetCounts.activeVfxCount, 2);
}

(async () => {
  await animationVfxAudioFrameBudgetRegression();
  assertNoGeneratedFixtureState();
  console.log('production 2d media/performance regressions passed');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
