const assert = require('node:assert/strict');
const { collectSpriteAssets, createAssetTracker, normalizeManifest } = require('./assets.js');

class FakeImage {
  constructor() {
    this.naturalWidth = 16;
    this.naturalHeight = 16;
  }

  set src(value) {
    this._src = value;
    FakeImage.loaded.push(value);
    if (this.onload) this.onload();
  }

  get src() {
    return this._src;
  }
}
FakeImage.loaded = [];

const manifest = {
  schemaVersion: '1',
  id: 'runtime-v1-assets',
  assets: [
    { id: 'player-sprite', kind: 'sprite', path: 'assets/sprites/player.svg' },
    { id: 'goal-sprite', kind: 'sprite', path: 'assets/sprites/goal.svg' },
    { id: 'spawn-sound', kind: 'audio', path: 'assets/audio/spawn.ogg' },
  ],
};

const scene = {
  assetManifest: manifest,
  entities: [
    { id: 'player', sprite: { asset: 'player-sprite' } },
    { id: 'goal', sprite: { asset: 'goal-sprite' } },
    { id: 'clone', sprite: { asset: 'player-sprite' } },
    { id: 'fallback', sprite: {} },
  ],
};

assert.deepEqual(collectSpriteAssets(scene), [
  'goal-sprite',
  'player-sprite',
]);

const normalized = normalizeManifest(manifest);
assert.deepEqual(normalized.errors, []);
assert.deepEqual(normalized.entries.map((entry) => entry.id), ['goal-sprite', 'player-sprite', 'spawn-sound']);

let tick = 1000;
const tracker = createAssetTracker({ ImageCtor: FakeImage, now: () => { tick += 5; return tick; } });
tracker.load(scene);
assert.deepEqual(FakeImage.loaded.sort(), ['assets/sprites/goal.svg', 'assets/sprites/player.svg']);

FakeImage.loaded = [];
let sceneRelativeTick = 5000;
const sceneRelativeTracker = createAssetTracker({
  ImageCtor: FakeImage,
  now: () => { sceneRelativeTick += 5; return sceneRelativeTick; },
});
sceneRelativeTracker.load(scene, scene.assetManifest, {
  resolvePath: (assetPath) => `/examples/playable-demo-v2/collect-and-exit/${assetPath}`,
});
assert.deepEqual(FakeImage.loaded.sort(), [
  '/examples/playable-demo-v2/collect-and-exit/assets/sprites/goal.svg',
  '/examples/playable-demo-v2/collect-and-exit/assets/sprites/player.svg',
]);
assert.deepEqual(sceneRelativeTracker.metadata().map((asset) => ({ id: asset.id, path: asset.path, resolvedPath: asset.resolvedPath })), [
  { id: 'goal-sprite', path: 'assets/sprites/goal.svg', resolvedPath: '/examples/playable-demo-v2/collect-and-exit/assets/sprites/goal.svg' },
  { id: 'player-sprite', path: 'assets/sprites/player.svg', resolvedPath: '/examples/playable-demo-v2/collect-and-exit/assets/sprites/player.svg' },
]);

assert.deepEqual(tracker.manifestSummary(), {
  id: 'runtime-v1-assets',
  enabled: true,
  assetCount: 3,
  errors: [],
  assets: [
    { id: 'goal-sprite', kind: 'sprite', path: 'assets/sprites/goal.svg' },
    { id: 'player-sprite', kind: 'sprite', path: 'assets/sprites/player.svg' },
    { id: 'spawn-sound', kind: 'audio', path: 'assets/audio/spawn.ogg' },
  ],
});
assert.deepEqual(tracker.metadata(), [
  { attemptId: 'load-goal-sprite', id: 'goal-sprite', path: 'assets/sprites/goal.svg', resolvedPath: 'assets/sprites/goal.svg', kind: 'image', status: 'loaded', startedAtUnixMs: 1005, endedAtUnixMs: 1010, loadDurationMs: 5, failureReason: null, width: 16, height: 16 },
  { attemptId: 'load-player-sprite', id: 'player-sprite', path: 'assets/sprites/player.svg', resolvedPath: 'assets/sprites/player.svg', kind: 'image', status: 'loaded', startedAtUnixMs: 1015, endedAtUnixMs: 1020, loadDurationMs: 5, failureReason: null, width: 16, height: 16 },
]);
assert.ok(tracker.imageFor('player-sprite'));
assert.equal(tracker.imageFor('assets/sprites/player.svg'), null, 'manifest mode rejects direct browser path lookup');
assert.deepEqual(tracker.metadata().at(-1), { attemptId: 'reject-assets-sprites-player-svg', id: 'assets/sprites/player.svg', path: null, kind: 'image', status: 'rejected', startedAtUnixMs: 1025, endedAtUnixMs: 1030, loadDurationMs: 1, failureReason: 'Asset reference unresolved', width: null, height: null });

const rejected = normalizeManifest({
  schemaVersion: '1',
  id: 'bad-assets',
  assets: [
    { id: 'remote', kind: 'sprite', path: 'https://example.com/player.svg' },
    { id: 'escape', kind: 'sprite', path: 'assets/../outside.svg' },
    { id: 'text', kind: 'sprite', path: 'assets/sprites/player.txt' },
  ],
});
assert.ok(rejected.errors.some((error) => error.includes('safe local assets/ path')));
assert.ok(rejected.errors.some((error) => error.includes('unsupported sprite path')));


class FailingImage {
  set src(value) {
    this._src = value;
    if (this.onerror) this.onerror(new Error(`failed ${value}`));
  }

  get src() {
    return this._src;
  }
}
let failTick = 2000;
const loadEvents = [];
const failingTracker = createAssetTracker({
  ImageCtor: FailingImage,
  now: () => { failTick += 7; return failTick; },
  onEvent: (event) => loadEvents.push(event),
});
failingTracker.load(scene);
assert.deepEqual(loadEvents.map((event) => [event.id, event.status, event.failureReason]), [
  ['goal-sprite', 'failed', 'Image load failed'],
  ['player-sprite', 'failed', 'Image load failed'],
]);
assert.deepEqual(failingTracker.metadata().map((entry) => ({ id: entry.id, status: entry.status, duration: entry.loadDurationMs })), [
  { id: 'goal-sprite', status: 'failed', duration: 7 },
  { id: 'player-sprite', status: 'failed', duration: 7 },
]);

const rejectedTracker = createAssetTracker({ ImageCtor: null, now: () => 3000 });
rejectedTracker.load(scene);
assert.equal(rejectedTracker.metadata()[0].status, 'rejected');
assert.equal(rejectedTracker.metadata()[0].failureReason, 'Image constructor unavailable');

FakeImage.loaded = [];
let atlasTick = 4000;
const atlasManifest = {
  schemaVersion: 'asset-manifest-v1',
  id: 'runtime-atlas-assets',
  assets: [
    { id: 'player-sheet-image', type: 'image', path: 'assets/sprites/player-sheet.png' },
    {
      id: 'player-atlas',
      type: 'sprite_atlas',
      path: 'assets/atlases/player.atlas.json',
      atlas: { imageAssetId: 'player-sheet-image', frames: [{ id: 'idle_0', rect: { x: 16, y: 0, width: 16, height: 16 } }] },
    },
  ],
};
const atlasScene = { assetManifest: atlasManifest, entities: [{ id: 'player', sprite: { asset: 'player-atlas', frameId: 'idle_0' } }] };
assert.deepEqual(collectSpriteAssets(atlasScene), ['player-sheet-image']);
const normalizedAtlas = normalizeManifest(atlasManifest);
assert.deepEqual(normalizedAtlas.errors, []);
assert.equal(normalizedAtlas.byId.get('player-atlas').kind, 'sprite_atlas');
assert.equal(normalizedAtlas.byId.get('player-atlas').atlas.frames[0].id, 'idle_0');
const atlasTracker = createAssetTracker({ ImageCtor: FakeImage, now: () => { atlasTick += 5; return atlasTick; } });
atlasTracker.load(atlasScene);
assert.deepEqual(FakeImage.loaded, ['assets/sprites/player-sheet.png']);
const spriteRef = atlasTracker.spriteFor('player-atlas', 'idle_0');
assert.ok(spriteRef.image);
assert.deepEqual(spriteRef.frame, { x: 16, y: 0, width: 16, height: 16 });
assert.equal(spriteRef.imageAssetId, 'player-sheet-image');
assert.equal(atlasTracker.spriteFor('player-atlas', 'missing'), null);
assert.equal(atlasTracker.metadata().at(-1).failureReason, 'Sprite atlas frame unresolved: missing');
