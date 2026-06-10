#!/usr/bin/env node
'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const scenePath = path.join(fixtureDir, 'scenes', 'signal-gate-relay.scene.json');
const scenarioPath = path.join(fixtureDir, 'scenarios', 'signal-gate-relay-core.json');
const scripts = ['collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js', 'renderer.js', 'tilemap.js', 'runtime.js'];

function readJson(filePath) { return JSON.parse(fs.readFileSync(filePath, 'utf8')); }
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
  for (const script of scripts) vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  return context.__OUROFORGE__;
}
function summarize(state) {
  const flags = state.componentModel.goalFlags;
  return {
    relay: flags.relay_1_active === true,
    key: flags.key_collected === true,
    gate: flags.gate_open === true,
    exit: flags.exit_reached === true,
    alive: flags.player_alive === true,
  };
}

(async () => {
  const scene = readJson(scenePath);
  const scenarioPack = readJson(scenarioPath);
  assert.equal(scenarioPack.schemaVersion, 'scenario-pack-v1');
  assert.equal(scenarioPack.scenarios.length, 2);

  const api = createRuntime(scene);
  await api.whenReady();
  const start = api.getWorldState();
  const checkpoint = api.createSave('signal-yard-start');
  api.setInput({ right: true });
  const relay = api.step(28);
  const keyGate = api.step(30);
  const win = api.step(65);
  api.setInput({ right: false });
  const probe = api.runtimeState('signal-gate-first-playable-loop');
  const restored = api.loadSave(checkpoint);

  const failApi = createRuntime(scene);
  await failApi.whenReady();
  failApi.setInput({ right: true, up: true });
  const fail = failApi.step(78);

  const tempRunRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-signal-gate-runs-'));
  const bundleDir = path.join(tempRunRoot, 'runs', 'signal-gate-relay', 'first-playable-loop');
  fs.mkdirSync(path.join(bundleDir, 'screenshots'), { recursive: true });
  const bundle = {
    schemaVersion: 'signal-gate-live-evidence-bundle-v1',
    issue: 2385,
    scenarioPack: 'scenarios/signal-gate-relay-core.json',
    classification: 'product-observed-local-runtime-smoke',
    generatedRoot: 'runs/signal-gate-relay/first-playable-loop',
    states: {
      start: summarize(start),
      relay: summarize(relay),
      keyGate: summarize(keyGate),
      win: summarize(win),
      fail: summarize(fail),
      restarted: summarize(restored)
    },
    digest: probe.digest.value,
    events: api.getEvents().map((event) => event.type),
    screenshots: [
      'screenshots/state-start.png',
      'screenshots/state-key-collected.png',
      'screenshots/state-gate-open.png',
      'screenshots/state-fail-blocked.png',
      'screenshots/state-restarted.png',
      'screenshots/state-win-exit.png'
    ],
    humanFunFeelVerdict: 'not-automated; record in M129 journal/playtest backlog'
  };
  fs.writeFileSync(path.join(bundleDir, 'bundle.json'), JSON.stringify(bundle, null, 2));
  const observed = readJson(path.join(bundleDir, 'bundle.json'));

  assert.equal(observed.states.start.alive, true);
  assert.equal(observed.states.relay.relay, true);
  assert.equal(observed.states.keyGate.key, true);
  assert.equal(observed.states.keyGate.gate, true);
  assert.equal(observed.states.win.exit, true);
  assert.equal(observed.states.fail.alive, false);
  assert.equal(observed.states.restarted.exit, false);
  assert.ok(observed.digest, 'bundle records runtime digest');
  assert.ok(observed.events.includes('runtime.trigger.entered'), 'bundle records runtime trigger evidence');
  assert.ok(observed.screenshots.every((name) => name.startsWith('screenshots/state-')));
  assert.equal(observed.humanFunFeelVerdict.startsWith('not-automated'), true);

  fs.rmSync(tempRunRoot, { recursive: true, force: true });
  for (const name of ['runs', 'target', 'dashboard-data', 'screenshots', 'browser-profiles', 'dist']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay untracked`);
  }

  console.log('signal-gate live evidence smoke passed');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
