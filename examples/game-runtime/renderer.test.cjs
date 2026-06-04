const assert = require('node:assert/strict');
const { normalizeRenderer, renderOrder, renderQueue, renderBreakdown, compareBreakdowns, debugState, drawRuntime, worldToScreen, cameraOffsetForLayer } = require('./renderer.js');
const tilemap = require('./tilemap.js');

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
    strokeStyle: null,
    strokeRect(...args) { calls.push(['strokeRect', this.strokeStyle, ...args]); },
  };
  return context;
}

const renderer = normalizeRenderer({
  version: '1',
  camera: { x: 8, y: 4 },
  viewport: { width: 160, height: 90 },
  background: '#101827',
  layers: [
    { id: 'background', order: -10, parallaxFactor: 50 },
    { id: 'actors', order: 0 },
    { id: 'debug', order: 10, visible: false, cameraParticipation: false },
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
    { id: 'background', order: -10, visible: true, parallaxFactor: 50, cameraParticipation: true },
    { id: 'actors', order: 0, visible: true, parallaxFactor: 100, cameraParticipation: true },
    { id: 'debug', order: 10, visible: false, parallaxFactor: 100, cameraParticipation: false },
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
  ['fillRect', '#0f172a', -4, -2, 320, 180],
  ['fillRect', '#5eead4', 16, 16, 16, 16],
  ['fillRect', '#facc15', 32, 16, 8, 8],
]);
assert.deepEqual(cameraOffsetForLayer(renderer, 'background'), { x: 4, y: 2 });
assert.deepEqual(cameraOffsetForLayer(renderer, 'debug'), { x: 0, y: 0 });
assert.deepEqual(worldToScreen({ x: 24, y: 20 }, renderer, 'actors'), { x: 16, y: 16, layer: 'actors', cameraOffset: { x: 8, y: 4 } });
assert.deepEqual(worldToScreen({ x: 0, y: 0 }, renderer, 'background'), { x: -4, y: -2, layer: 'background', cameraOffset: { x: 4, y: 2 } });
assert.ok(context.calls.some((call) => call[0] === 'fillText' && call[2] === 'player'));
assert.ok(context.calls.some((call) => call[0] === 'fillText' && call[2] === 'scene=renderer-test tick=3'));

const queueRenderer = normalizeRenderer({
  version: '1',
  camera: { x: 0, y: 0 },
  viewport: { width: 160, height: 90 },
  background: '#101827',
  layers: [
    { id: 'background', order: -10 },
    { id: 'actors', order: 0 },
  ],
  debug: { showBounds: true, showCamera: true, showEntityIds: true },
}, { width: 320, height: 180 });
const queueTilemaps = tilemap.normalizeTilemaps([{
  id: 'level',
  tileSize: { width: 16, height: 16 },
  grid: { width: 1, height: 1 },
  tiles: [{ id: 'grass', color: '#22c55e' }],
  layers: [{ id: 'ground', order: -5, data: ['grass'] }],
}]);
const queue = renderQueue({
  world: { sceneId: 'queue-test', tick: 6, bounds: { width: 320, height: 180 }, entities, tilemaps: queueTilemaps },
  renderer: queueRenderer,
  tilemap,
  frameId: 'frame-queue',
});
assert.equal(queue.schemaVersion, 'ouroforge.scene-render-queue.v1');
assert.equal(queue.validation.status, 'ready');
assert.deepEqual(queue.tilemapStats, { layerCount: 1, cellCount: 1, drawnTileCount: 1, missingTileRefCount: 0, assetTileCount: 0 });
assert.deepEqual(queue.renderables.find((renderable) => renderable.id === 'tilemap-level-ground').tileCount, 1);
assert.deepEqual(queue.renderables.map((renderable) => [renderable.id, renderable.drawOrder, renderable.primitiveKind, renderable.visible]), [
  ['entity-sky', 0, 'rect', true],
  ['tilemap-level-ground', 1, 'tilemap', true],
  ['entity-sprite-hidden', 2, 'rect', false],
  ['entity-player', 3, 'rect', true],
  ['entity-zebra', 4, 'rect', true],
  ['entity-debug-hidden', 5, 'rect', true],
  ['debug-camera', 6, 'debug_camera', true],
  ['debug-bounds-debug-hidden', 7, 'debug_bounds', true],
  ['debug-bounds-player', 8, 'debug_bounds', true],
  ['debug-bounds-sky', 9, 'debug_bounds', true],
  ['debug-bounds-sprite-hidden', 10, 'debug_bounds', true],
  ['debug-bounds-zebra', 11, 'debug_bounds', true],
  ['debug-label-debug-hidden', 12, 'debug_label', true],
  ['debug-label-player', 13, 'debug_label', true],
  ['debug-label-sky', 14, 'debug_label', true],
  ['debug-label-sprite-hidden', 15, 'debug_label', true],
  ['debug-label-zebra', 16, 'debug_label', true],
]);
assert.deepEqual(queue.readOnlyInspection.disallowedActions, ['trusted writes', 'command bridge', 'live mutation']);

const missingTileQueue = renderQueue({
  world: {
    sceneId: 'missing-tile-queue-test',
    tick: 1,
    bounds: { width: 64, height: 64 },
    entities: [],
    tilemaps: tilemap.normalizeTilemaps([{ id: 'broken', tileSize: { width: 16, height: 16 }, grid: { width: 2, height: 1 }, tiles: [{ id: 'known', asset: 'known-tile' }], layers: [{ id: 'ground', order: 0, data: ['known', 'missing'] }] }]),
  },
  renderer: normalizeRenderer({ layers: [{ id: 'ground', order: 0 }] }),
  tilemap,
  frameId: 'frame-missing-tile',
});
assert.equal(missingTileQueue.validation.status, 'warning');
assert.deepEqual(missingTileQueue.tilemapStats, { layerCount: 1, cellCount: 2, drawnTileCount: 1, missingTileRefCount: 1, assetTileCount: 1 });
assert.match(missingTileQueue.validation.warnings[0], /references missing tile missing/);
assert.equal(missingTileQueue.renderables.find((renderable) => renderable.id === 'tilemap-broken-ground').missingTileRefCount, 1);


const queueContext = createContext();
drawRuntime({
  canvas: { width: 320, height: 180 },
  context: queueContext,
  renderer: queueRenderer,
  world: { sceneId: 'queue-test', tick: 6, bounds: { width: 320, height: 180 }, entities, tilemaps: queueTilemaps },
  assets: { imageFor: () => null },
  animation: { activeSpriteFrame: () => null },
  tilemap,
});
assert.deepEqual(queueContext.calls.filter((call) => call[0] === 'fillRect').slice(0, 5), [
  ['fillRect', '#101827', 0, 0, 320, 180],
  ['fillRect', '#0f172a', 0, 0, 320, 180],
  ['fillRect', '#22c55e', 0, 0, 16, 16],
  ['fillRect', '#5eead4', 24, 20, 16, 16],
  ['fillRect', '#facc15', 40, 20, 8, 8],
]);
assert.ok(queueContext.calls.some((call) => call[0] === 'fillText' && call[2] === 'camera=0,0'));
assert.ok(queueContext.calls.some((call) => call[0] === 'strokeRect'));

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

// Regression (#892, 못쟁이 blocking): drawRuntime resolved entity renderables
// through a Map keyed on entity id, which silently collapsed duplicate or missing
// ids to a single entity — the last match was drawn repeatedly and the earlier
// entities were never drawn. Each entity must draw exactly once at its own
// position/color regardless of id collisions.
{
  const idCollisionRenderer = normalizeRenderer({
    version: '1',
    background: '#000000',
    layers: [{ id: 'actors', order: 0 }],
  }, { width: 100, height: 100 });
  const idCollisionContext = createContext();
  const idCollisionEntities = [
    { id: 'dup', sprite: { color: '#aa0000', layer: 'actors' }, components: { transform: { x: 10, y: 10 }, size: { width: 4, height: 4 } } },
    { id: 'dup', sprite: { color: '#00bb00', layer: 'actors' }, components: { transform: { x: 20, y: 20 }, size: { width: 4, height: 4 } } },
    { id: '', sprite: { color: '#0000cc', layer: 'actors' }, components: { transform: { x: 30, y: 30 }, size: { width: 4, height: 4 } } },
    { id: '', sprite: { color: '#cccc00', layer: 'actors' }, components: { transform: { x: 40, y: 40 }, size: { width: 4, height: 4 } } },
  ];
  drawRuntime({
    canvas: { width: 100, height: 100 },
    context: idCollisionContext,
    renderer: idCollisionRenderer,
    world: { sceneId: 'id-collision-test', tick: 0, bounds: { width: 100, height: 100 }, entities: idCollisionEntities },
    assets: { imageFor: () => null },
    animation: { activeSpriteFrame: () => null },
  });
  const drawnColors = idCollisionContext.calls.filter((call) => call[0] === 'fillRect').map((call) => call[1]);
  for (const color of ['#aa0000', '#00bb00', '#0000cc', '#cccc00']) {
    assert.ok(drawnColors.includes(color), `entity ${color} must draw despite duplicate/missing id (no id-keyed collapse)`);
  }
}

{
  const atlasContext = createContext();
  const atlasImage = { id: 'player-sheet-image' };
  drawRuntime({
    canvas: { width: 64, height: 64 },
    context: atlasContext,
    renderer: normalizeRenderer({ layers: [{ id: 'actors', order: 0 }] }),
    world: {
      sceneId: 'atlas-render-test',
      tick: 1,
      bounds: { width: 64, height: 64 },
      entities: [{ id: 'atlas-player', sprite: { asset: 'player-atlas', frameId: 'idle_0', layer: 'actors' }, components: { transform: { x: 8, y: 12 }, size: { width: 16, height: 16 } } }],
    },
    assets: { spriteFor: () => ({ image: atlasImage, frame: { x: 16, y: 0, width: 16, height: 16 } }), imageFor: () => null },
    animation: { activeSpriteFrame: () => null },
  });
  assert.deepEqual(atlasContext.calls.find((call) => call[0] === 'drawImage'), ['drawImage', atlasImage, 16, 0, 16, 16, 8, 12, 16, 16]);
  const atlasBreakdown = renderBreakdown({
    world: { sceneId: 'atlas-render-test', entities: [{ id: 'atlas-player', sprite: { asset: 'player-atlas', frameId: 'idle_0', layer: 'actors' }, components: { transform: { x: 8, y: 12 }, size: { width: 16, height: 16 } } }] },
    renderer: normalizeRenderer({ layers: [{ id: 'actors', order: 0 }] }),
  });
  assert.equal(atlasBreakdown.elements[0].assetRef, 'player-atlas');
  assert.equal(atlasBreakdown.elements[0].assetFrameRef, 'idle_0');
}

// Regression: an active animation frame asset must override the base sprite even
// when that base sprite resolves through the sprite-atlas path. Previously the
// atlas image for the base sprite was drawn and the animation frame asset was
// silently dropped.
{
  const overrideContext = createContext();
  const baseAtlasImage = { id: 'base-atlas-sheet' };
  const frameImage = { id: 'hurt-frame-image' };
  drawRuntime({
    canvas: { width: 64, height: 64 },
    context: overrideContext,
    renderer: normalizeRenderer({ layers: [{ id: 'actors', order: 0 }] }),
    world: {
      sceneId: 'frame-override-test',
      tick: 1,
      bounds: { width: 64, height: 64 },
      entities: [{ id: 'hero', sprite: { asset: 'player-atlas', frameId: 'idle_0', layer: 'actors' }, components: { transform: { x: 8, y: 12 }, size: { width: 16, height: 16 }, animation: {} } }],
    },
    assets: {
      spriteFor: () => ({ image: baseAtlasImage, frame: { x: 16, y: 0, width: 16, height: 16 } }),
      imageFor: (asset) => (asset === 'hurt-frame' ? frameImage : null),
    },
    animation: { activeSpriteFrame: () => ({ asset: 'hurt-frame' }) },
  });
  const overrideDraw = overrideContext.calls.find((call) => call[0] === 'drawImage');
  assert.deepEqual(
    overrideDraw,
    ['drawImage', frameImage, 8, 12, 16, 16],
    'animation frame asset must override the base sprite-atlas image',
  );
}
