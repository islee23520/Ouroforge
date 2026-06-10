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
function stateName(flags) {
  if (flags.player_alive === false) return 'lost';
  if (flags.exit_reached === true) return 'won';
  if (flags.gate_open === true) return 'gate-open';
  if (flags.key_collected === true) return 'key-collected';
  if (flags.relay_1_active === true) return 'relay-1';
  return 'start';
}

(async () => {
  const scene = readJson(scenePath);
  const api = createRuntime(scene);
  await api.whenReady();

  const start = api.getWorldState();
  assert.equal(stateName(start.componentModel.goalFlags), 'start');
  assert.equal(start.componentModel.goalFlags.player_alive, true);
  const checkpoint = api.createSave('signal-yard-start');

  api.setInput({ right: true });
  const relay = api.step(28);
  assert.equal(relay.componentModel.goalFlags.relay_1_active, true, 'relay activates');
  assert.equal(stateName(relay.componentModel.goalFlags), 'relay-1');

  const key = api.step(30);
  assert.equal(key.componentModel.goalFlags.key_collected, true, 'signal key collected');
  assert.equal(key.componentModel.goalFlags.gate_open, true, 'gate opens with key');
  assert.equal(stateName(key.componentModel.goalFlags), 'gate-open');

  const win = api.step(65);
  api.setInput({ right: false });
  assert.equal(win.componentModel.goalFlags.exit_reached, true, 'exit reached');
  assert.equal(stateName(win.componentModel.goalFlags), 'won');
  assert.equal(api.getFrameStats().runtimeFrameBudgetStatus, 'within-budget');

  const probe = api.runtimeState('signal-gate-win');
  assert.equal(probe.schemaVersion, 'runtime-state-v1');
  assert.ok(probe.digest && probe.digest.value, 'runtime probe digest exists');
  assert.ok(probe.entities.some((entity) => entity.entityId === 'player'), 'probe exposes player');

  const restored = api.loadSave(checkpoint);
  assert.equal(stateName(restored.componentModel.goalFlags), 'start', 'restart restores initial flags');
  assert.ok(api.getEvents().some((event) => event.type === 'runtime.save.loaded'));

  const failApi = createRuntime(scene);
  await failApi.whenReady();
  failApi.setInput({ right: true, up: true });
  const failed = failApi.step(78);
  assert.equal(failed.componentModel.goalFlags.player_alive, false, 'hazard creates visible fail state');
  assert.equal(stateName(failed.componentModel.goalFlags), 'lost');

  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-signal-gate-core-'));
  const evidence = {
    schemaVersion: 'signal-gate-live-evidence-v1',
    issue: 2385,
    states: {
      start: start.componentModel.goalFlags,
      relay: relay.componentModel.goalFlags,
      key: key.componentModel.goalFlags,
      win: win.componentModel.goalFlags,
      fail: failed.componentModel.goalFlags,
      restart: restored.componentModel.goalFlags,
    },
    digest: probe.digest.value,
    classification: 'product-observed-local-runtime-smoke',
  };
  const evidencePath = path.join(tempDir, 'first-playable-loop.json');
  fs.writeFileSync(evidencePath, JSON.stringify(evidence, null, 2));
  assert.equal(readJson(evidencePath).states.win.exit_reached, true);
  fs.rmSync(tempDir, { recursive: true, force: true });

  for (const name of ['runs', 'target', 'dashboard-data', 'screenshots', 'browser-profiles', 'dist']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay untracked`);
  }

  console.log('signal-gate core loop smoke passed');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
