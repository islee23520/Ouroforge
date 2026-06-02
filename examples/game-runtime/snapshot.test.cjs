const assert = require('node:assert/strict');
const { createSnapshotRegistry } = require('./snapshot.js');

const registry = createSnapshotRegistry();
const state = {
  world: {
    tick: 4,
    sceneId: 'foundation-scene',
    entities: [{ id: 'player', components: { transform: { x: 40, y: 72 } } }],
  },
  input: { right: true },
  events: [{ type: 'runtime.input.changed' }],
};

const snapshotId = registry.capture(state, state.world.tick);
assert.equal(snapshotId, 'snapshot-0001');
assert.deepEqual(registry.metadata(snapshotId), {
  snapshotId: 'snapshot-0001',
  tick: 4,
  capturedAtTick: 4,
});
assert.deepEqual(registry.list(), [registry.metadata(snapshotId)]);

state.world.entities[0].components.transform.x = 88;
state.input.right = false;

const restored = registry.restore(snapshotId);
assert.equal(restored.world.entities[0].components.transform.x, 40);
assert.equal(restored.input.right, true);
assert.throws(() => registry.restore('missing'), /snapshot not found/);
assert.throws(() => registry.restore(''), /snapshotId is required/);
