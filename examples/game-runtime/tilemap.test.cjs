const assert = require('node:assert/strict');
const { normalizeTilemaps, orderedLayers, drawTilemaps, debugState } = require('./tilemap.js');

function createContext() {
  const calls = [];
  return {
    calls,
    fillStyle: null,
    fillRect(...args) { calls.push(['fillRect', this.fillStyle, ...args]); },
    drawImage(...args) { calls.push(['drawImage', ...args]); },
  };
}

const [tilemap] = normalizeTilemaps([
  {
    id: 'level',
    tileSize: { width: 16, height: 16 },
    grid: { width: 3, height: 2 },
    tiles: [
      { id: 'grass', color: '#22c55e' },
      { id: 'stone', color: '#64748b', solid: true },
    ],
    layers: [
      { id: 'foreground', order: 5, data: [null, 'stone', null, null, null, 'stone'], collisionLayer: 'collision' },
      { id: 'background', order: -5, data: ['grass', 'grass', 'grass', null, null, null] },
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
    tileCount: 2,
    layers: [
      { id: 'foreground', order: 5, visible: true, cellCount: 6, nonEmptyCells: 2, collisionLayer: 'collision' },
      { id: 'background', order: -5, visible: true, cellCount: 6, nonEmptyCells: 3, collisionLayer: null },
      { id: 'collision', order: 0, visible: false, cellCount: 6, nonEmptyCells: 2, collisionLayer: null },
    ],
  }],
  layerOrder: [
    { tilemapId: 'level', layerId: 'background', order: -5 },
    { tilemapId: 'level', layerId: 'foreground', order: 5 },
  ],
});

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
  { tilemapId: 'level', layerId: 'foreground', tileId: 'stone', x: 8, y: -4 },
  { tilemapId: 'level', layerId: 'foreground', tileId: 'stone', x: 24, y: 12 },
]);
assert.deepEqual(context.calls.slice(0, 3), [
  ['fillRect', '#22c55e', -8, -4, 16, 16],
  ['fillRect', '#22c55e', 8, -4, 16, 16],
  ['fillRect', '#22c55e', 24, -4, 16, 16],
]);
