const assert = require('node:assert/strict');
const { detectAabbCollisions, stepAabbPhysics } = require('./collision.js');

const baseEntity = (id, x, body = 'static', collider = {}) => ({
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
      trigger: false,
      collisionMask: [],
      ...collider,
    },
  },
});

const overlap = detectAabbCollisions([
  baseEntity('goal', 10, 'static'),
  baseEntity('player', 0, 'dynamic'),
], 7);
assert.deepEqual(overlap, [{
  tick: 7,
  type: 'runtime.collision.contact',
  pairId: 'goal:player',
  dynamicEntityId: 'player',
  otherEntityId: 'goal',
  movingEntityId: 'player',
  staticEntityId: 'goal',
  movingBody: 'dynamic',
  otherBody: 'static',
  dynamicBody: 'dynamic',
  sensor: false,
  trigger: false,
  normal: { x: -1, y: 0 },
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

const blocked = [
  baseEntity('wall', 20, 'static'),
  baseEntity('player', 0, 'kinematic'),
];
blocked[1].components.velocity.x = 24;
const contactStep = stepAabbPhysics(blocked, { width: 64, height: 32 }, 10);
assert.equal(blocked[1].components.transform.x, 4);
assert.deepEqual(contactStep.events, [{
  tick: 10,
  type: 'runtime.collision.contact',
  pairId: 'player:wall',
  dynamicEntityId: null,
  otherEntityId: 'player',
  movingEntityId: 'player',
  staticEntityId: 'wall',
  movingBody: 'kinematic',
  otherBody: 'static',
  dynamicBody: null,
  sensor: false,
  trigger: false,
  normal: { x: -1, y: 0 },
}]);

const triggerScene = [
  baseEntity('goal', 20, 'static', { trigger: true, collisionGroup: 'triggers' }),
  baseEntity('player', 0, 'dynamic', { collisionGroup: 'actors', collisionMask: ['triggers'] }),
];
triggerScene[1].components.velocity.x = 24;
const triggerStep = stepAabbPhysics(triggerScene, { width: 64, height: 32 }, 11);
assert.equal(triggerScene[1].components.transform.x, 24);
assert.deepEqual(triggerStep.events, [{
  tick: 11,
  type: 'runtime.collision.trigger',
  pairId: 'goal:player',
  dynamicEntityId: 'player',
  otherEntityId: 'goal',
  movingEntityId: 'player',
  staticEntityId: 'goal',
  movingBody: 'dynamic',
  otherBody: 'static',
  dynamicBody: 'dynamic',
  sensor: false,
  trigger: true,
  normal: { x: 0, y: 0 },
}]);

const maskedOut = [
  baseEntity('wall', 20, 'static', { collisionGroup: 'world' }),
  baseEntity('player', 0, 'dynamic', { collisionGroup: 'actors', collisionMask: ['triggers'] }),
];
maskedOut[1].components.velocity.x = 24;
const maskedStep = stepAabbPhysics(maskedOut, { width: 64, height: 32 }, 12);
assert.equal(maskedOut[1].components.transform.x, 24);
assert.deepEqual(maskedStep.events, []);
