#!/usr/bin/env node
'use strict';

// Godot-Plus demo UI/HUD/feedback smoke (#785).
//
// Validates the declarative HUD model against the live runtime: every HUD row
// binds an objective flag and reflects gameplay state (objective, key, health);
// win/lose game-state is derivable from the same flags; the existing visual
// key-pickup feedback fires; and the existing scene-load spawn audio intent is
// modeled without implying pickup audio.
// Also confirms the read-only dashboard renders the HUD evidence. Pure read-only
// harness: temp dir only, removed before exit.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const dashboard = require(path.join(repoRoot, 'examples', 'evidence-dashboard', 'dashboard.js'));
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

function hudRow(world, entityId) {
  return world.componentModel.hudValues.find((row) => row.entityId === entityId);
}

// Derive game-state and HUD display text from objective flags per the HUD model.
function deriveGameState(model, flags) {
  if (flags.player_alive === false) return 'lost';
  if (flags.exit_reached === true) return 'won';
  return 'in-progress';
}

function rowDisplay(model, rowId, flags) {
  const row = model.rows.find((candidate) => candidate.id === rowId);
  const flagValue = flags[row.bindFlag] === true;
  return row.states[String(flagValue)];
}

function feedbackRows(model, kind) {
  return model.feedback.filter((row) => row.kind === kind);
}

(async () => {
  const model = readJson(path.join(fixtureDir, 'hud-model.json'));
  const scene = readJson(path.join(fixtureDir, 'scenes', 'collect-and-exit.scene.json'));

  assert.equal(model.schemaVersion, 'demo-hud-model-v1', 'hud model schema');
  assert.deepEqual(model.rows.map((r) => r.id).sort(), ['goal', 'health', 'key'], 'hud model rows');
  const audioFeedback = feedbackRows(model, 'audio');
  assert.deepEqual(
    audioFeedback.map((row) => ({ on: row.on, asset: row.asset })),
    [{ on: 'scene_loaded', asset: 'collect_sound' }],
    'model: audio feedback is scene-load spawn intent'
  );
  assert.equal(
    audioFeedback.some((row) => row.on === 'key_collected'),
    false,
    'model: no pickup audio feedback is declared'
  );

  const api = createRuntime(scene);
  await api.whenReady();

  // Initial HUD reflects the start state; every model row maps to a live HUD row.
  const initial = api.getWorldState();
  for (const row of model.rows) {
    const live = hudRow(initial, row.entityId);
    assert.ok(live, `HUD row present: ${row.entityId}`);
    assert.equal(live.bindFlag, row.bindFlag, `HUD row binds ${row.bindFlag}`);
  }
  assert.equal(hudRow(initial, 'hud_goal').flagValue, false, 'initial: objective not complete');
  assert.equal(hudRow(initial, 'hud_key').flagValue, false, 'initial: key not collected');
  assert.equal(hudRow(initial, 'hud_health').flagValue, true, 'initial: player alive');
  const initialFlags = initial.componentModel.goalFlags;
  assert.equal(deriveGameState(model, initialFlags), 'in-progress', 'initial: in-progress');
  assert.equal(rowDisplay(model, 'key', initialFlags), '0/1', 'initial key display');

  // Win path: HUD reflects key collection, objective complete, player alive.
  api.setInput({ right: true });
  const won = api.step(120);
  api.setInput({ right: false });
  assert.equal(hudRow(won, 'hud_key').flagValue, true, 'win: key HUD updates');
  assert.equal(hudRow(won, 'hud_goal').flagValue, true, 'win: objective HUD complete');
  assert.equal(hudRow(won, 'hud_health').flagValue, true, 'win: health HUD alive');
  const wonFlags = won.componentModel.goalFlags;
  assert.equal(deriveGameState(model, wonFlags), 'won', 'win: game-state won');
  assert.equal(rowDisplay(model, 'key', wonFlags), '1/1', 'win key display');
  assert.equal(rowDisplay(model, 'goal', wonFlags), 'Exit reached', 'win goal display');

  // Visual feedback: the key entity is hidden on pickup.
  const keyEntity = won.entities.find((entity) => entity.id === 'key');
  assert.equal(keyEntity.sprite.visible, false, 'visual feedback: key hidden on pickup');

  // Audio feedback: the existing collect_sound intent is scene-load spawn audio.
  const spawnAudioEvent = won.audioEvents.find((event) => event.asset === 'collect_sound');
  assert.deepEqual(
    {
      name: spawnAudioEvent && spawnAudioEvent.name,
      trigger: spawnAudioEvent && spawnAudioEvent.trigger,
      asset: spawnAudioEvent && spawnAudioEvent.asset,
      playback: spawnAudioEvent && spawnAudioEvent.playback,
    },
    {
      name: 'player_spawn',
      trigger: 'scene_loaded',
      asset: 'collect_sound',
      playback: 'intent',
    },
    'audio feedback: collect_sound is player_spawn scene-load intent'
  );

  // Lose-state HUD derivation (model-level; no new assets needed).
  const lostFlags = { player_alive: false, key_collected: true, exit_reached: false };
  assert.equal(deriveGameState(model, lostFlags), 'lost', 'lose: game-state lost');
  assert.equal(rowDisplay(model, 'health', lostFlags), '0/3', 'lose health display');

  // Read-only dashboard renders the HUD evidence.
  const rendered = dashboard.renderGameplaySummary({
    present: true,
    gameplay: {
      present: true,
      hudValues: won.componentModel.hudValues,
      hudValueEntityCount: won.componentModel.hudValues.length,
    },
  });
  assert.match(rendered, /HUD values/, 'dashboard renders HUD values');
  assert.match(rendered, /Key: 0\/1/, 'dashboard shows the key HUD row');

  // Evidence shape -> temp dir, removed before exit.
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-collect-exit-hud-'));
  const tempEvidence = path.join(tempDir, 'hud.json');
  fs.writeFileSync(tempEvidence, JSON.stringify({ won: wonFlags, lost: lostFlags }, null, 2));
  assert.equal(readJson(tempEvidence).won.exit_reached, true);
  fs.rmSync(tempDir, { recursive: true, force: true });

  for (const name of ['runs', 'target', 'dashboard-data', 'dist']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay untracked`);
  }

  console.log('collect-and-exit HUD smoke passed');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
