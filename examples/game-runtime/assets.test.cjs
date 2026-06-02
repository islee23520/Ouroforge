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

const tracker = createAssetTracker({ ImageCtor: FakeImage });
tracker.load(scene);
assert.deepEqual(FakeImage.loaded.sort(), ['assets/sprites/goal.svg', 'assets/sprites/player.svg']);
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
  { id: 'goal-sprite', path: 'assets/sprites/goal.svg', kind: 'image', status: 'loaded', width: 16, height: 16 },
  { id: 'player-sprite', path: 'assets/sprites/player.svg', kind: 'image', status: 'loaded', width: 16, height: 16 },
]);
assert.ok(tracker.imageFor('player-sprite'));
assert.equal(tracker.imageFor('assets/sprites/player.svg'), null, 'manifest mode rejects direct browser path lookup');
assert.deepEqual(tracker.metadata().at(-1), { id: 'assets/sprites/player.svg', path: null, kind: 'unknown', status: 'unresolved', width: null, height: null });

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
