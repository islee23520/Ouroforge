#!/usr/bin/env node
'use strict';

// Godot-Plus demo core gameplay loop smoke (#782).
//
// Drives the collect-and-exit demo through the full deterministic loop using the
// existing browser runtime probe contract: spawn -> move/control -> primary
// interaction (collect key) -> objective tracking (gate opens) -> win
// (exit reached) -> restart/reset (checkpoint restore). It also proves the
// deterministic non-win (objective-blocked) path and asserts the runtime probe
// (runtime-state-v1) exposes objective/player/game-state data.
//
// Loop-level lose is the objective-blocked (not-won) state. Hazard-contact death
// (player_alive -> false) is owned by issue #784. Pure read-only harness: writes
// only to a temp dir outside the repository and removes it before exit.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const scenePath = path.join(fixtureDir, 'scenes', 'collect-and-exit.scene.json');
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
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

// Derive the loop-level game state from objective flags.
function gameState(flags) {
  if (flags.player_alive === false) return 'lost';
  if (flags.exit_reached === true) return 'won';
  return 'in-progress';
}

(async () => {
  const scene = readJson(scenePath);

  // --- Win path: spawn -> move -> interact -> objective -> win -> restart ---
  const api = createRuntime(scene);
  await api.whenReady();

  // Spawn + runtime probe contract.
  const spawn = api.getWorldState();
  assert.equal(spawn.componentModel.goalFlags.key_collected, false, 'spawn: key not collected');
  assert.notEqual(spawn.componentModel.goalFlags.door_open, true, 'spawn: gate closed');
  assert.equal(spawn.componentModel.goalFlags.exit_reached, false, 'spawn: not won');
  assert.equal(spawn.componentModel.goalFlags.player_alive, true, 'spawn: player alive');
  assert.ok(spawn.object && spawn.object.id === 'player', 'spawn: player object exposed by probe');
  assert.equal(gameState(spawn.componentModel.goalFlags), 'in-progress', 'spawn: loop in-progress');

  // Runtime probe state contract exposes objective + player + digest.
  const probe = api.runtimeState('gameplay-loop');
  assert.equal(probe.schemaVersion, 'runtime-state-v1', 'probe schema');
  assert.equal(probe.flags.exit_reached, false, 'probe exposes objective flags');
  const probePlayer = probe.entities.find((entity) => entity.entityId === 'player');
  assert.ok(probePlayer, 'probe exposes player entity');
  assert.ok(probePlayer.status && probePlayer.status.hitPoints > 0, 'probe exposes player status');
  assert.ok(probe.digest && probe.digest.value, 'probe exposes a deterministic digest');

  // Restart checkpoint captured at the start state.
  const startSave = api.createSave('demo-start');
  assert.equal(startSave.state.flags.key_collected, false, 'checkpoint captures start state');

  // Move/control + primary interaction (collect key) + objective (gate opens).
  api.setInput({ right: true });
  const afterKey = api.step(40);
  assert.equal(afterKey.componentModel.goalFlags.key_collected, true, 'interaction: key collected');
  assert.equal(afterKey.componentModel.goalFlags.door_open, true, 'objective: gate opened');
  assert.equal(gameState(afterKey.componentModel.goalFlags), 'in-progress', 'still playing after key');

  // Win condition: reach the exit.
  const afterExit = api.step(45);
  api.setInput({ right: false });
  assert.equal(afterExit.componentModel.goalFlags.exit_reached, true, 'win: exit reached');
  assert.equal(gameState(afterExit.componentModel.goalFlags), 'won', 'loop reports won');

  // Restart/reset via checkpoint restore.
  const restored = api.loadSave(startSave);
  assert.equal(restored.componentModel.goalFlags.key_collected, false, 'restart resets key');
  assert.equal(restored.componentModel.goalFlags.exit_reached, false, 'restart resets win');
  assert.equal(gameState(restored.componentModel.goalFlags), 'in-progress', 'restart returns to play');
  assert.ok(
    api.getEvents().some((event) => event.type === 'runtime.save.loaded'),
    'restart records a save.loaded event'
  );

  // --- Non-win (objective-blocked) path: idle, never collect the key ---
  const blockedApi = createRuntime(scene);
  await blockedApi.whenReady();
  const blocked = blockedApi.step(80); // no input: player never reaches the key
  assert.equal(blocked.componentModel.goalFlags.key_collected, false, 'blocked: key uncollected');
  assert.notEqual(blocked.componentModel.goalFlags.door_open, true, 'blocked: gate stays closed');
  assert.equal(blocked.componentModel.goalFlags.exit_reached, false, 'blocked: not won');
  assert.equal(blocked.componentModel.goalFlags.player_alive, true, 'blocked: player still alive');
  assert.equal(gameState(blocked.componentModel.goalFlags), 'in-progress', 'blocked: win is gated');

  // Frame budget stays within bounds across the loop.
  assert.equal(api.getFrameStats().runtimeFrameBudgetStatus, 'within-budget', 'frame budget held');

  // Evidence shape can be written to a temp dir outside the repo, then removed.
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-collect-exit-loop-'));
  const tempEvidence = path.join(tempDir, 'gameplay-loop.json');
  fs.writeFileSync(tempEvidence, JSON.stringify({
    win: { flags: afterExit.componentModel.goalFlags, state: 'won' },
    blocked: { flags: blocked.componentModel.goalFlags, state: 'in-progress' },
    restart: { flags: restored.componentModel.goalFlags },
    probeDigest: probe.digest.value,
  }, null, 2));
  assert.equal(readJson(tempEvidence).win.state, 'won');
  fs.rmSync(tempDir, { recursive: true, force: true });

  // Generated-state audit: no generated roots committed inside the fixture.
  for (const name of ['runs', 'target', 'dashboard-data', 'dist']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay untracked`);
  }

  console.log('collect-and-exit gameplay loop smoke passed');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
