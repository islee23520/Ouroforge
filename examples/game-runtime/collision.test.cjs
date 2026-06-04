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

const sensorBodyScene = [
  baseEntity('sensor-zone', 20, 'sensor', { collisionGroup: 'sensors' }),
  baseEntity('player', 0, 'dynamic', { collisionGroup: 'actors', collisionMask: ['sensors'] }),
];
sensorBodyScene[1].components.velocity.x = 24;
const sensorStep = stepAabbPhysics(sensorBodyScene, { width: 64, height: 32 }, 13);
assert.equal(sensorBodyScene[1].components.transform.x, 24, 'sensor bodies do not block movement');
assert.deepEqual(sensorStep.events, [{
  tick: 13,
  type: 'runtime.collision.trigger',
  pairId: 'player:sensor-zone',
  dynamicEntityId: 'player',
  otherEntityId: 'sensor-zone',
  movingEntityId: 'player',
  staticEntityId: null,
  movingBody: 'dynamic',
  otherBody: 'sensor',
  dynamicBody: 'dynamic',
  sensor: true,
  trigger: true,
  normal: { x: 0, y: 0 },
}]);

const triggerBodyScene = [
  baseEntity('checkpoint', 20, 'trigger', { collisionGroup: 'triggers' }),
  baseEntity('player', 0, 'dynamic', { collisionGroup: 'actors', collisionMask: ['triggers'] }),
];
triggerBodyScene[1].components.velocity.x = 24;
const triggerBodyStep = stepAabbPhysics(triggerBodyScene, { width: 64, height: 32 }, 14);
assert.equal(triggerBodyScene[1].components.transform.x, 24, 'trigger bodies do not block movement');
assert.deepEqual(triggerBodyStep.events.map((event) => [event.type, event.otherBody, event.trigger]), [
  ['runtime.collision.trigger', 'trigger', true],
]);

const disabledWallScene = [
  baseEntity('wall', 20, 'static', { disabled: true, collisionGroup: 'world' }),
  baseEntity('player', 0, 'dynamic', { collisionGroup: 'actors', collisionMask: ['world'] }),
];
disabledWallScene[1].components.velocity.x = 24;
const disabledStep = stepAabbPhysics(disabledWallScene, { width: 64, height: 32 }, 15);
assert.equal(disabledWallScene[1].components.transform.x, 24, 'disabled colliders are ignored by resolution');
assert.deepEqual(disabledStep.events, []);

function deterministicBlockedStep() {
  const scene = [
    baseEntity('wall', 20, 'static', { collisionGroup: 'world' }),
    baseEntity('player', 0, 'dynamic', { collisionGroup: 'actors', collisionMask: ['world'] }),
  ];
  scene[1].components.velocity.x = 48;
  const result = stepAabbPhysics(scene, { width: 96, height: 32 }, 16);
  return { x: scene[1].components.transform.x, events: result.events };
}
const replayA = deterministicBlockedStep();
const replayB = deterministicBlockedStep();
assert.deepEqual(replayA, replayB, 'bounded AABB resolution is deterministic for repeated high-delta steps');
assert.equal(replayA.x, 4, 'high-delta movement stops at the first blocking AABB within scoped limits');
assert.equal(replayA.events[0].normal.x, -1);
