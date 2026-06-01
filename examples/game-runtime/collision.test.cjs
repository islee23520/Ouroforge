const assert = require('node:assert/strict');
const { detectAabbCollisions } = require('./collision.js');

const baseEntity = (id, x, body = 'static') => ({
  id,
  components: {
    transform: { x, y: 0 },
    size: { width: 16, height: 16 },
    velocity: { x: 0, y: 0 },
    controllable: body === 'dynamic',
    collider: {
      shape: 'aabb',
      body,
      offset: { x: 0, y: 0 },
      size: { width: 16, height: 16 },
      sensor: false,
    },
  },
});

const overlap = detectAabbCollisions([
  baseEntity('goal', 10, 'static'),
  baseEntity('player', 0, 'dynamic'),
], 7);
assert.deepEqual(overlap, [{
  tick: 7,
  type: 'runtime.collision.detected',
  pairId: 'goal:player',
  dynamicEntityId: 'player',
  otherEntityId: 'goal',
  dynamicBody: 'dynamic',
  otherBody: 'static',
  sensor: false,
}]);

const separated = detectAabbCollisions([
  baseEntity('goal', 32, 'static'),
  baseEntity('player', 0, 'dynamic'),
], 8);
assert.deepEqual(separated, []);

const staticOnly = detectAabbCollisions([
  baseEntity('crate-a', 0, 'static'),
  baseEntity('crate-b', 10, 'static'),
], 9);
assert.deepEqual(staticOnly, []);
