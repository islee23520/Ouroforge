#!/usr/bin/env node
'use strict';

// Godot-Plus demo enemy/system behavior smoke (#784).
//
// Exercises the deterministic Signal Gate hazard-drone archetype over the base
// collect-and-exit scene using the existing runtime trigger contract (sensor +
// requiredFlags + onEnter clearFlag). It proves:
//   - the deterministic dormant -> armed state transition (collecting the key
//     arms the drone);
//   - collision/interaction affects gameplay: contact with an armed drone ends
//     the run (player_alive -> false) before the exit (lose);
//   - a dormant drone passed before the key is harmless, so the player still
//     wins (win-with-hazard);
//   - determinism: the lose outcome and death tick are identical across runs;
//   - the runtime probe exposes the hazard behavior/entity state.
// Pure read-only harness: temp dir only, removed before exit.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
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

function buildSceneWithHazard(baseScene, archetype, placement) {
  const scene = JSON.parse(JSON.stringify(baseScene));
  const hazard = JSON.parse(JSON.stringify(archetype.entityTemplate));
  hazard.components.transform = { x: placement.x, y: placement.y };
  scene.entities.push(hazard);
  return scene;
}

// Drive the player right and stop as soon as the player dies (player_alive=false),
// up to a frame cap. Returns { world, deathFrame } (deathFrame=null if survived).
function playUntilDeath(api, cap) {
  api.setInput({ right: true });
  let world = api.getWorldState();
  let deathFrame = null;
  for (let frame = 1; frame <= cap; frame += 1) {
    world = api.step(1);
    if (world.componentModel.goalFlags.player_alive === false) {
      deathFrame = frame;
      break;
    }
  }
  api.setInput({ right: false });
  return { world, deathFrame };
}

(async () => {
  const archetype = readJson(path.join(fixtureDir, 'behaviors', 'hazard-drone.json'));
  const baseScene = readJson(path.join(fixtureDir, 'scenes', 'collect-and-exit.scene.json'));

  assert.equal(archetype.schemaVersion, 'demo-behavior-archetype-v1', 'archetype schema');
  assert.ok(archetype.states.length >= 2, 'archetype declares states');
  assert.ok(
    archetype.transitions.some((t) => t.from === 'dormant' && t.to === 'armed'),
    'archetype declares dormant -> armed transition'
  );

  const failScenario = archetype.scenarios.find((s) => s.id === 'hazard-contact-fail');
  const passScenario = archetype.scenarios.find((s) => s.id === 'hazard-dormant-pass');
  assert.ok(failScenario && passScenario, 'archetype declares fail and pass scenarios');

  // --- Lose path: armed drone past the key ends the run before the exit ---
  const failApi = createRuntime(buildSceneWithHazard(baseScene, archetype, failScenario.placement));
  await failApi.whenReady();

  // Probe exposes the hazard entity + its declared dormant state.
  const failInitial = failApi.getWorldState();
  const droneEntity = failInitial.entities.find((e) => e.id === 'hazard_drone');
  assert.ok(droneEntity, 'probe exposes the hazard_drone entity');
  assert.ok(
    droneEntity.components.status.states.includes('dormant'),
    'probe exposes the dormant behavior state'
  );
  const probe = failApi.runtimeState('behavior');
  assert.ok(probe.entities.some((e) => e.entityId === 'hazard_drone'), 'runtime probe lists the hazard');

  const fail = playUntilDeath(failApi, 80);
  assert.equal(fail.world.componentModel.goalFlags.key_collected, true, 'fail: key collected (arms drone)');
  assert.equal(fail.world.componentModel.goalFlags.player_alive, false, 'fail: armed drone ends the run');
  assert.notEqual(fail.world.componentModel.goalFlags.exit_reached, true, 'fail: not exited (lose)');
  assert.ok(Number.isInteger(fail.deathFrame), 'fail: death is observed at a deterministic frame');
  assert.deepEqual(fail.world.componentModel.goalFlags.player_alive, failScenario.expect.player_alive);

  // --- Determinism: same lose outcome and death frame on a fresh run ---
  const failApi2 = createRuntime(buildSceneWithHazard(baseScene, archetype, failScenario.placement));
  await failApi2.whenReady();
  const fail2 = playUntilDeath(failApi2, 80);
  assert.equal(fail2.deathFrame, fail.deathFrame, 'death frame is deterministic across runs');
  assert.equal(fail2.world.componentModel.goalFlags.player_alive, false, 'deterministic lose');

  // --- Win-with-hazard: dormant drone passed before the key is harmless ---
  const passApi = createRuntime(buildSceneWithHazard(baseScene, archetype, passScenario.placement));
  await passApi.whenReady();
  passApi.setInput({ right: true });
  const passWorld = passApi.step(200);
  passApi.setInput({ right: false });
  assert.equal(passWorld.componentModel.goalFlags.key_collected, true, 'pass: key collected');
  assert.equal(passWorld.componentModel.goalFlags.player_alive, true, 'pass: dormant drone harmless');
  assert.equal(passWorld.componentModel.goalFlags.exit_reached, true, 'pass: exited alive (win)');
  assert.equal(passWorld.componentModel.goalFlags.player_alive, passScenario.expect.player_alive);

  // Evidence shape -> temp dir outside repo, removed before exit.
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-collect-exit-behavior-'));
  const tempEvidence = path.join(tempDir, 'behavior.json');
  fs.writeFileSync(tempEvidence, JSON.stringify({
    archetype: archetype.id,
    fail: { deathFrame: fail.deathFrame, flags: fail.world.componentModel.goalFlags },
    pass: { flags: passWorld.componentModel.goalFlags },
  }, null, 2));
  assert.equal(readJson(tempEvidence).fail.flags.player_alive, false);
  fs.rmSync(tempDir, { recursive: true, force: true });

  for (const name of ['runs', 'target', 'dashboard-data', 'dist']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay untracked`);
  }

  console.log(`collect-and-exit behavior smoke passed; hazard lose at frame ${fail.deathFrame}, dormant pass wins`);
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
