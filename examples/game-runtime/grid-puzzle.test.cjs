// Runtime contract test for the Grid-Puzzle Game Class v1 (#1574).
// Mirror of the Rust contract test crates/ouroforge-core/tests/grid_puzzle_game_class_contract.rs.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'grid-puzzle.js', 'runtime.js',
];

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in grid puzzle test')),
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

function readScene(file) {
  return JSON.parse(fs.readFileSync(path.join(runtimeDir, file), 'utf8'));
}

// Re-realm values produced inside the vm context so deepStrictEqual compares by
// value rather than by the vm context's Object.prototype identity.
function norm(value) {
  return JSON.parse(JSON.stringify(value));
}

function directionInput(direction) {
  return {
    left: direction === 'left',
    right: direction === 'right',
    up: direction === 'up',
    down: direction === 'down',
  };
}

function runScene(scene, moves) {
  const api = createRuntime();
  api.loadScene(scene);
  for (const move of moves) {
    api.setInput(directionInput(move));
    api.step(1);
  }
  return api;
}

const scene = readScene('grid-puzzle-scene-v1.json');
const solution = scene.gridPuzzle.intendedSolution;

// 1. The grid puzzle is fully observable via the probe before any input.
const initial = runScene(scene, []).getWorldState().gridPuzzle;
assert.equal(initial.schemaVersion, 'ouroforge.grid-puzzle-state.v1');
assert.equal(initial.status, 'playing');
assert.equal(initial.width, 6);
assert.equal(initial.height, 5);
assert.deepEqual(norm(initial.player), { x: 3, y: 2 });
assert.equal(initial.win.type, 'all-targets-covered');
assert.equal(initial.win.satisfied, false);
assert.deepEqual(norm(initial.targets), [{ x: 1, y: 1 }]);
assert.equal(initial.readOnlyInspection.browserStudioMode.includes('read-only'), true);
assert.deepEqual(
  norm(initial.readOnlyInspection.disallowedActions),
  ['trusted writes', 'command bridge', 'live mutation'],
);

// 2. The declared intended solution deterministically reaches the win state.
const solvedApi = runScene(scene, solution);
const solved = solvedApi.getWorldState().gridPuzzle;
assert.equal(solved.status, 'won');
assert.equal(solved.win.satisfied, true);
assert.deepEqual(norm(solved.player), { x: 1, y: 2 });
assert.ok(solved.cells[1][1].includes('crate'), 'target cell must be covered by a crate');
assert.ok(
  solvedApi.getEvents().some((event) => event.type === 'runtime.grid_puzzle.status_changed'
    && event.payload.status === 'won'),
  'runtime must record the win status change',
);

// 3. A blocked push (into a wall) is reported and does not falsely win.
const blocked = runScene(scene, ['left', 'left']).getWorldState().gridPuzzle;
assert.equal(blocked.status, 'playing');
assert.equal(blocked.lastMove.result, 'blocked');
assert.deepEqual(norm(blocked.player), { x: 2, y: 2 });

// 4. Malformed grid fails closed with a clear diagnostic.
assert.throws(
  () => runScene(readScene('grid-puzzle-invalid-malformed-grid.json'), []),
  /grid puzzle spec invalid: row 2 must be a string of length 6/,
);

// 5. Missing win condition fails closed with a clear diagnostic.
assert.throws(
  () => runScene(readScene('grid-puzzle-invalid-missing-win.json'), []),
  /grid puzzle spec invalid: a win condition with a string type is required/,
);

// 6. Digest stability: two identical runs produce the same replay digest; a
//    divergent input produces a different digest.
const runA = runScene(scene, solution);
const digestA = runA.replayStateDigest('frame-grid-solve');
assert.match(digestA.digest.value, /^[0-9a-f]{16}$/);
assert.equal(digestA.policy.browserWriteAccess, 'none');

const runB = runScene(scene, solution);
const matched = runB.compareReplayDigest(digestA, 'frame-grid-solve');
assert.equal(matched.status, 'matched');
assert.equal(matched.firstDivergence, null);
assert.equal(matched.actual.value, digestA.digest.value);

const runC = runScene(scene, ['up']);
const diverged = runC.compareReplayDigest(digestA.digest, 'frame-grid-up');
assert.equal(diverged.status, 'diverged');
assert.notEqual(diverged.actual.value, digestA.digest.value);

// 7. Non-grid scenes are unaffected: the probe exposes gridPuzzle as null and
//    the digest still computes.
const plainApi = createRuntime();
plainApi.loadScene({ schemaVersion: '1', id: 'plain-scene' });
const plain = plainApi.getWorldState();
assert.equal(plain.gridPuzzle, null);
assert.match(plainApi.replayStateDigest('frame-plain').digest.value, /^[0-9a-f]{16}$/);

console.log('grid puzzle game class runtime test passed');
