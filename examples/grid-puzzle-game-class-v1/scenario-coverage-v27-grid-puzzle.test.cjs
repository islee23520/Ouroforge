'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const root = path.resolve(__dirname, '../..');
const runtimeDir = path.join(root, 'examples/game-runtime');
const matrix = JSON.parse(fs.readFileSync(path.join(__dirname, 'scenario-coverage-v27/matrix.fixture.json'), 'utf8'));
assert.equal(matrix.issue, 1577);
for (const key of ['valid','malformed','unsupported']) {
  assert.ok(fs.existsSync(path.join(root, matrix.dslFixtures[key])));
}

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in scenario coverage v27')),
    addEventListener: () => {},
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of [
    'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
    'renderer.js', 'tilemap.js', 'grid-puzzle.js', 'runtime.js',
  ]) {
    vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  }
  return context.__OUROFORGE__;
}

function runGrid(scene, moves) {
  const api = createRuntime();
  api.loadScene(scene);
  for (const move of moves) {
    api.setInput({
      left: move === 'left',
      right: move === 'right',
      up: move === 'up',
      down: move === 'down',
    });
    api.step(1);
  }
  return api;
}

const gridScene = JSON.parse(fs.readFileSync(path.join(root, matrix.gridScenes.solvable), 'utf8'));
const solved = runGrid(gridScene, matrix.gridScenes.solution);
assert.equal(solved.getWorldState().gridPuzzle.status, 'won');
assert.ok(solved.getEvents().some((event) => event.type === 'runtime.grid_puzzle.status_changed'
  && event.payload.status === 'won'));

const nonWinning = runGrid(gridScene, matrix.gridScenes.nonWinningReplay).getWorldState().gridPuzzle;
assert.equal(nonWinning.status, 'playing');
assert.equal(nonWinning.lastMove.result, 'blocked');

const collect = JSON.parse(fs.readFileSync(path.join(root, matrix.backwardCompat.collectAndExit), 'utf8'));
assert.equal(collect.id, 'collect-and-exit-scene');
assert.equal(collect.metadata.scenarioId, 'collect-and-exit-source-smoke');
assert.ok(collect.gameplayRules.flags.some((flag) => flag.id === 'exit_reached'));

const doc = fs.readFileSync(path.join(root, matrix.wordingAudit.doc), 'utf8');
for (const a of matrix.wordingAudit.anchors) assert.ok(doc.includes(a));
console.log('scenario-coverage-v27 fixture smoke passed');
