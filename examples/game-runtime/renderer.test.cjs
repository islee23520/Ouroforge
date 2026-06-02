const assert = require('node:assert/strict');
const { normalizeRenderer, renderOrder, drawRuntime } = require('./renderer.js');

function createContext() {
  const calls = [];
  const context = {
    calls,
    fillStyle: null,
    font: null,
    clearRect(...args) { calls.push(['clearRect', ...args]); },
    fillRect(...args) { calls.push(['fillRect', this.fillStyle, ...args]); },
    drawImage(...args) { calls.push(['drawImage', ...args]); },
    fillText(...args) { calls.push(['fillText', this.fillStyle, ...args]); },
  };
  return context;
}

const renderer = normalizeRenderer({
  version: '1',
  camera: { x: 8, y: 4 },
  viewport: { width: 160, height: 90 },
  background: '#101827',
  layers: [
    { id: 'background', order: -10 },
    { id: 'actors', order: 0 },
    { id: 'debug', order: 10, visible: false },
  ],
  debug: { showEntityIds: true },
}, { width: 320, height: 180 });

const entities = [
  {
    id: 'zebra',
    sprite: { color: '#facc15', layer: 'actors', order: 5 },
    components: { transform: { x: 40, y: 20 }, size: { width: 8, height: 8 } },
  },
  {
    id: 'player',
    sprite: { color: '#5eead4', layer: 'actors', order: 5 },
    components: { transform: { x: 24, y: 20 }, size: { width: 16, height: 16 } },
  },
  {
    id: 'sky',
    sprite: { color: '#0f172a', layer: 'background', order: 0 },
    components: { transform: { x: 0, y: 0 }, size: { width: 320, height: 180 } },
  },
  {
    id: 'debug-hidden',
    sprite: { color: '#ffffff', layer: 'debug', order: 0 },
    components: { transform: { x: 0, y: 0 }, size: { width: 8, height: 8 } },
  },
  {
    id: 'sprite-hidden',
    sprite: { color: '#ffffff', layer: 'actors', order: 1, visible: false },
    components: { transform: { x: 0, y: 0 }, size: { width: 8, height: 8 } },
  },
];

const ordered = renderOrder(entities, renderer).map(({ entityId, layer, layerOrder, spriteOrder }) => ({ entityId, layer, layerOrder, spriteOrder }));
assert.deepEqual(ordered, [
  { entityId: 'sky', layer: 'background', layerOrder: -10, spriteOrder: 0 },
  { entityId: 'player', layer: 'actors', layerOrder: 0, spriteOrder: 5 },
  { entityId: 'zebra', layer: 'actors', layerOrder: 0, spriteOrder: 5 },
]);

const context = createContext();
const drawOrder = drawRuntime({
  canvas: { width: 320, height: 180 },
  context,
  renderer,
  world: { sceneId: 'renderer-test', tick: 3, bounds: { width: 320, height: 180 }, entities },
  assets: { imageFor: () => null },
  animation: { activeSpriteFrame: () => null },
});

assert.deepEqual(drawOrder, ordered);
assert.deepEqual(context.calls.filter((call) => call[0] === 'fillRect').slice(0, 4), [
  ['fillRect', '#101827', 0, 0, 320, 180],
  ['fillRect', '#0f172a', -8, -4, 320, 180],
  ['fillRect', '#5eead4', 16, 16, 16, 16],
  ['fillRect', '#facc15', 32, 16, 8, 8],
]);
assert.ok(context.calls.some((call) => call[0] === 'fillText' && call[2] === 'player'));
assert.ok(context.calls.some((call) => call[0] === 'fillText' && call[2] === 'scene=renderer-test tick=3'));
