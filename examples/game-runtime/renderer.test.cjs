const assert = require('node:assert/strict');
const { normalizeRenderer, renderOrder, renderBreakdown, compareBreakdowns, debugState, drawRuntime } = require('./renderer.js');

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

assert.deepEqual(debugState(renderer, entities), {
  version: '1',
  camera: { x: 8, y: 4 },
  viewport: { width: 160, height: 90 },
  background: '#101827',
  layers: [
    { id: 'background', order: -10, visible: true },
    { id: 'actors', order: 0, visible: true },
    { id: 'debug', order: 10, visible: false },
  ],
  debug: { showBounds: false, showCamera: false, showEntityIds: true },
  renderedEntities: ordered,
});

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

const uiContext = createContext();
drawRuntime({
  canvas: { width: 320, height: 180 },
  context: uiContext,
  renderer,
  world: {
    sceneId: 'renderer-ui-text-test',
    tick: 4,
    bounds: { width: 320, height: 180 },
    entities: [{
      id: 'hud_coin',
      sprite: { color: '#ffffff', layer: 'actors', order: 6 },
      components: {
        transform: { x: 8, y: 8 },
        size: { width: 64, height: 12 },
        uiText: { text: 'Coin: 0/1', role: 'hud', bindFlag: 'coin_collected' },
      },
    }],
  },
  assets: { imageFor: () => null },
  animation: { activeSpriteFrame: () => null },
});
assert.ok(uiContext.calls.some((call) => call[0] === 'fillText' && call[2] === 'Coin: 0/1'));

const hudValueContext = createContext();
drawRuntime({
  canvas: { width: 320, height: 180 },
  context: hudValueContext,
  renderer,
  world: {
    sceneId: 'renderer-hud-value-test',
    tick: 5,
    bounds: { width: 320, height: 180 },
    entities: [{
      id: 'hud_health',
      sprite: { color: '#ffffff', layer: 'actors', order: 6 },
      components: {
        transform: { x: 8, y: 24 },
        size: { width: 64, height: 12 },
        hudValue: { kind: 'health', label: 'HP', value: '3/3', bindFlag: 'player_alive' },
      },
    }],
  },
  assets: { imageFor: () => null },
  animation: { activeSpriteFrame: () => null },
});
assert.ok(hudValueContext.calls.some((call) => call[0] === 'fillText' && call[2] === 'HP: 3/3'));


const breakdown = renderBreakdown({ world: { sceneId: 'renderer-test', tick: 3, bounds: { width: 320, height: 180 }, entities }, renderer, frameId: 'frame-0003' });
assert.equal(breakdown.schemaVersion, 'ouroforge.scene-render-breakdown.v1');
assert.deepEqual(breakdown.elements.map((element) => [element.entityId, element.drawOrder, element.primitiveCategory]), [['sky', 0, 'rect'], ['player', 1, 'rect'], ['zebra', 2, 'rect']]);
assert.deepEqual(breakdown.absenceDiagnostics.map((diag) => [diag.entityId, diag.reason]), [['debug-hidden', 'layer_hidden'], ['sprite-hidden', 'hidden']]);
assert.deepEqual(renderBreakdown({ world: { sceneId: 'renderer-test', entities }, renderer, filter: { layer: 'actors' } }).elements.map((element) => element.entityId), ['player', 'zebra']);
assert.deepEqual(breakdown.readOnlyInspection.disallowedActions, ['trusted writes', 'command bridge', 'live mutation']);
const malformedBreakdown = renderBreakdown({ world: { sceneId: 'malformed-renderer-test', entities: [{ sprite: { color: '#fff', layer: 'actors' }, components: {} }] }, renderer });
assert.ok(malformedBreakdown.absenceDiagnostics.some((diag) => diag.reason === 'malformed'));
assert.ok(malformedBreakdown.absenceDiagnostics.some((diag) => diag.reason === 'fallback'));
const changedBreakdown = renderBreakdown({ world: { sceneId: 'renderer-test', tick: 4, bounds: { width: 320, height: 180 }, entities: entities.map((entity) => entity.id === 'zebra' ? { ...entity, sprite: { ...entity.sprite, order: 0 } } : entity) }, renderer, frameId: 'frame-0004' });
assert.ok(compareBreakdowns(breakdown, changedBreakdown).changes.some((change) => change.renderableId === 'renderer-test/entity:zebra' && change.field === 'drawOrder'));
