#!/usr/bin/env node
'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const runtimeDir = path.resolve(__dirname, '..', '..', 'game-runtime');
const scripts = ['collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js', 'renderer.js', 'tilemap.js', 'grid-puzzle.js', 'runtime.js'];
function runtime() {
  const context = { console, Image: function Image(){}, document: { getElementById: () => null }, fetch: () => Promise.reject(new Error('fetch disabled')), addEventListener: () => {} };
  context.window = context; context.globalThis = context; vm.createContext(context);
  for (const script of scripts) vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  return context.__OUROFORGE__;
}
function run(sceneFile) {
  const scene = JSON.parse(fs.readFileSync(path.join(__dirname, sceneFile), 'utf8'));
  const api = runtime(); api.loadScene(scene);
  for (const move of scene.gridPuzzle.intendedSolution) {
    api.setInput({ [move]: true, keys: { [move]: true } }); api.step(1); api.setInput({ [move]: false, keys: { [move]: false } });
  }
  return api.getWorldState().gridPuzzle;
}
assert.equal(run('scenes/before-review.scene.json').status, 'playing');
const solved = run('scenes/crate-garden-puzzle.scene.json');
assert.equal(solved.status, 'won');
assert.equal(solved.win.satisfied, true);
assert.deepEqual(JSON.parse(JSON.stringify(solved.intendedSolution)), ['left', 'down', 'left', 'up']);
const manifest = JSON.parse(fs.readFileSync(path.join(__dirname, 'ouroforge.project.json'), 'utf8'));
assert.match(manifest.genreSelectionRationale, /Different from Signal Gate Relay/);
const review = JSON.parse(fs.readFileSync(path.join(__dirname, 'review/review-apply-decision.json'), 'utf8'));
assert.equal(review.trustedWriteBoundary.browserTrustedWrite, false);
assert.equal(review.trustedWriteBoundary.commandBridge, false);
assert.ok(review.rawJsonManualSteps.length >= 1);
console.log('crate garden dogfood game 2 smoke passed');
