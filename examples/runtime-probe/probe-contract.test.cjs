const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');

const html = fs.readFileSync(new URL('./index.html', `file://${__dirname}/`), 'utf8');
const script = html.match(/<script>\s*([\s\S]*?)\s*<\/script>/)?.[1];
assert.ok(script, 'runtime probe script is present');

const canvas = {
  width: 160,
  height: 120,
  getContext() {
    return {
      clearRect() {},
      fillRect() {},
      fillText() {},
      set fillStyle(_value) {},
      set font(_value) {},
    };
  },
};
const stateElement = { textContent: '' };
const window = {
  requestAnimationFrame() {},
};
const document = {
  getElementById(id) {
    if (id === 'world') return canvas;
    if (id === 'state') return stateElement;
    throw new Error(`unexpected element id: ${id}`);
  },
};
window.window = window;

vm.runInNewContext(script, { window, document, JSON, Number, Object, Math, Error }, { filename: 'runtime-probe/index.html' });

const probe = window.__OUROFORGE__;
assert.equal(typeof probe, 'object');
const requiredMethods = [
  'getWorldState',
  'getFrameStats',
  'getEvents',
  'step',
  'pause',
  'resume',
  'setInput',
  'snapshot',
  'restore',
];
for (const method of requiredMethods) {
  assert.equal(typeof probe[method], 'function', `${method} is exposed`);
}

const world = probe.getWorldState();
assert.equal(typeof world.tick, 'number');
assert.equal(typeof world.fixedDeltaMs, 'number');
assert.equal(typeof world.paused, 'boolean');
assert.equal(typeof world.object.id, 'string');
assert.equal(typeof world.input.right, 'boolean');

const stats = probe.getFrameStats();
assert.equal(typeof stats.tick, 'number');
assert.equal(typeof stats.totalSteps, 'number');
assert.equal(typeof stats.lastStepCount, 'number');
assert.equal(typeof stats.eventCount, 'number');

assert.ok(Array.isArray(probe.getEvents()));
const stepped = probe.step(2);
assert.equal(stepped.tick, world.tick + 2);
assert.equal(probe.getFrameStats().lastStepCount, 2);
assert.equal(probe.pause().tick, stepped.tick);
assert.equal(probe.getWorldState().paused, true);
assert.equal(probe.resume().tick, stepped.tick);
assert.equal(probe.getWorldState().paused, false);
assert.equal(probe.setInput({ right: false, left: true }).input.left, true);
const snapshot = probe.snapshot();
probe.step(3);
const restored = probe.restore(snapshot);
assert.equal(restored.tick, snapshot.tick);
assert.throws(() => probe.restore(null), /snapshot object is required/);
