const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { normalizeTilemaps, orderedLayers, drawTilemaps, debugState, extractAuthoringCells, collisionEntities } = require('./tilemap.js');

function createContext() {
  const calls = [];
  return {
    calls,
    fillStyle: null,
    fillRect(...args) { calls.push(['fillRect', this.fillStyle, ...args]); },
    drawImage(...args) { calls.push(['drawImage', ...args]); },
  };
}

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in tilemap test')),
    addEventListener: () => {},
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of ['collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js', 'renderer.js', 'tilemap.js', 'runtime.js']) {
    vm.runInContext(fs.readFileSync(path.join(__dirname, script), 'utf8'), context, { filename: script });
  }
  return context.__OUROFORGE__;
}

const [tilemap] = normalizeTilemaps([
  {
    id: 'level',
    tileSize: { width: 16, height: 16 },
    grid: { width: 3, height: 2 },
    tiles: [
      { id: 'grass', color: '#22c55e' },
      { id: 'stone', color: '#64748b', solid: true },
      { id: 'coin', color: '#facc15', trigger: 'coin_collected', goal: true },
      { id: 'spike', color: '#ef4444', hazard: true },
    ],
    layers: [
      { id: 'foreground', order: 5, data: [null, 'stone', 'coin', null, null, 'stone'], collisionLayer: 'collision' },
      { id: 'background', order: -5, data: ['grass', 'grass', 'grass', null, null, 'spike'] },
      { id: 'collision', order: 0, visible: false, data: [null, 'stone', null, null, null, 'stone'] },
    ],
  },
]);

assert.deepEqual(orderedLayers([tilemap]).map(({ tilemapId, layerId, order }) => ({ tilemapId, layerId, order })), [
  { tilemapId: 'level', layerId: 'background', order: -5 },
  { tilemapId: 'level', layerId: 'foreground', order: 5 },
]);
assert.deepEqual(debugState([tilemap]), {
  version: '1',
  tilemaps: [{
    id: 'level',
    tileSize: { width: 16, height: 16 },
    grid: { width: 3, height: 2 },
    tileCount: 4,
    authoring: {
      version: '1',
      collisionCells: [
        { tilemapId: 'level', layerId: 'foreground', tileId: 'stone', index: 1, x: 1, y: 0, worldX: 16, worldY: 0, width: 16, height: 16, trigger: null },
        { tilemapId: 'level', layerId: 'foreground', tileId: 'stone', index: 5, x: 2, y: 1, worldX: 32, worldY: 16, width: 16, height: 16, trigger: null },
        { tilemapId: 'level', layerId: 'collision', tileId: 'stone', index: 1, x: 1, y: 0, worldX: 16, worldY: 0, width: 16, height: 16, trigger: null },
        { tilemapId: 'level', layerId: 'collision', tileId: 'stone', index: 5, x: 2, y: 1, worldX: 32, worldY: 16, width: 16, height: 16, trigger: null },
      ],
      triggerCells: [
        { tilemapId: 'level', layerId: 'foreground', tileId: 'coin', index: 2, x: 2, y: 0, worldX: 32, worldY: 0, width: 16, height: 16, trigger: 'coin_collected' },
      ],
      hazardCells: [
        { tilemapId: 'level', layerId: 'background', tileId: 'spike', index: 5, x: 2, y: 1, worldX: 32, worldY: 16, width: 16, height: 16, trigger: null },
      ],
      goalCells: [
        { tilemapId: 'level', layerId: 'foreground', tileId: 'coin', index: 2, x: 2, y: 0, worldX: 32, worldY: 0, width: 16, height: 16, trigger: 'coin_collected' },
      ],
    },
    layers: [
      { id: 'foreground', order: 5, visible: true, cellCount: 6, nonEmptyCells: 3, collisionLayer: 'collision' },
      { id: 'background', order: -5, visible: true, cellCount: 6, nonEmptyCells: 4, collisionLayer: null },
      { id: 'collision', order: 0, visible: false, cellCount: 6, nonEmptyCells: 2, collisionLayer: null },
    ],
  }],
  layerOrder: [
    { tilemapId: 'level', layerId: 'background', order: -5 },
    { tilemapId: 'level', layerId: 'foreground', order: 5 },
  ],
});

const extracted = extractAuthoringCells([tilemap]);
assert.equal(extracted.collisionCells.length, 4);
assert.equal(extracted.triggerCells[0].trigger, 'coin_collected');
assert.equal(extracted.hazardCells[0].tileId, 'spike');
assert.equal(extracted.goalCells[0].tileId, 'coin');
assert.equal(collisionEntities([tilemap]).filter((entity) => entity.tags.includes('collision')).length, 2, 'duplicate visible/hidden solid cells are deduped for physics');
assert.ok(collisionEntities([tilemap]).some((entity) => entity.components.trigger && entity.components.trigger.targetFlag === 'coin_collected'));

const context = createContext();
const drawn = drawTilemaps({
  context,
  renderer: { camera: { x: 8, y: 4 } },
  tilemaps: [tilemap],
  assets: { imageFor: () => null },
});
assert.deepEqual(drawn.map(({ tilemapId, layerId, tileId, x, y }) => ({ tilemapId, layerId, tileId, x, y })), [
  { tilemapId: 'level', layerId: 'background', tileId: 'grass', x: -8, y: -4 },
  { tilemapId: 'level', layerId: 'background', tileId: 'grass', x: 8, y: -4 },
  { tilemapId: 'level', layerId: 'background', tileId: 'grass', x: 24, y: -4 },
  { tilemapId: 'level', layerId: 'background', tileId: 'spike', x: 24, y: 12 },
  { tilemapId: 'level', layerId: 'foreground', tileId: 'stone', x: 8, y: -4 },
  { tilemapId: 'level', layerId: 'foreground', tileId: 'coin', x: 24, y: -4 },
  { tilemapId: 'level', layerId: 'foreground', tileId: 'stone', x: 24, y: 12 },
]);
assert.deepEqual(context.calls.slice(0, 3), [
  ['fillRect', '#22c55e', -8, -4, 16, 16],
  ['fillRect', '#22c55e', 8, -4, 16, 16],
  ['fillRect', '#22c55e', 24, -4, 16, 16],
]);

const api = createRuntime();
const tileRuntimeScene = {
  schemaVersion: '1',
  id: 'tilemap-authoring-runtime-evidence',
  bounds: { width: 96, height: 32 },
  entities: [{
    id: 'player',
    sprite: { color: '#5eead4' },
    components: {
      transform: { x: 0, y: 0 },
      velocity: { x: 0, y: 0 },
      size: { width: 16, height: 16 },
      controllable: false,
      input: { scheme: 'keyboard', moveSpeed: 24, allowedActions: ['move'] },
      collider: { shape: 'aabb', body: 'dynamic', offset: { x: 0, y: 0 }, size: { width: 16, height: 16 } },
    },
  }],
  tilemaps: [{
    id: 'authoring-map',
    tileSize: { width: 16, height: 16 },
    grid: { width: 4, height: 1 },
    tiles: [
      { id: 'empty', color: '#000000' },
      { id: 'coin', color: '#facc15', trigger: 'coin_collected', goal: true },
      { id: 'wall', color: '#64748b', solid: true },
    ],
    layers: [
      { id: 'terrain', order: 0, data: [null, 'coin', 'wall', null] },
    ],
  }],
};
let state = api.loadScene(tileRuntimeScene);
assert.equal(state.goalFlags.coin_collected, false, 'tile trigger flags initialize as scenario-readable false');
assert.equal(state.tilemaps.tilemaps[0].authoring.triggerCells[0].trigger, 'coin_collected');
api.setInput({ right: true });
state = api.step(1);
assert.equal(state.goalFlags.coin_collected, true, 'overlapping a trigger tile sets its declared flag');
assert.ok(state.collisionEvents.some((event) => event.type === 'runtime.collision.trigger' && event.pairId.includes('tilemap.trigger.authoring-map.terrain.1.0.coin')));
state = api.step(1);
const player = state.entities.find((entity) => entity.id === 'player');
assert.equal(player.components.transform.x, 16, 'solid tilemap cell blocks dynamic entity movement');
assert.ok(state.collisions.some((event) => event.type === 'runtime.collision.contact' && event.staticEntityId.includes('tilemap.collision.authoring-map.terrain.2.0.wall')));
