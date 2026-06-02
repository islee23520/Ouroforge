const assert = require('node:assert/strict');
const { collectSpriteAssets, createAssetTracker } = require('./assets.js');

class FakeImage {
  constructor() {
    this.naturalWidth = 16;
    this.naturalHeight = 16;
  }

  set src(value) {
    this._src = value;
    if (this.onload) this.onload();
  }

  get src() {
    return this._src;
  }
}

const entities = [
  { id: 'player', sprite: { asset: 'assets/sprites/player.svg' } },
  { id: 'goal', sprite: { asset: 'assets/sprites/goal.svg' } },
  { id: 'clone', sprite: { asset: 'assets/sprites/player.svg' } },
  { id: 'fallback', sprite: {} },
];

assert.deepEqual(collectSpriteAssets(entities), [
  'assets/sprites/goal.svg',
  'assets/sprites/player.svg',
]);

const tracker = createAssetTracker({ ImageCtor: FakeImage });
tracker.load(entities);
assert.deepEqual(tracker.metadata(), [
  { path: 'assets/sprites/goal.svg', kind: 'image', status: 'loaded', width: 16, height: 16 },
  { path: 'assets/sprites/player.svg', kind: 'image', status: 'loaded', width: 16, height: 16 },
]);
assert.ok(tracker.imageFor('assets/sprites/player.svg'));
assert.equal(tracker.imageFor('missing.svg'), null);
