const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const repoRoot = path.resolve(runtimeDir, '..', '..');
const nestedScene = '/examples/playable-demo-v2/collect-and-exit/scenes/session-k-product-observed/after-reviewed-fix.scene.json';
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

class LoadingImage {
  constructor() {
    this.naturalWidth = 64;
    this.naturalHeight = 32;
  }

  set src(value) {
    this._src = value;
    if (this.onload) this.onload();
  }

  get src() {
    return this._src;
  }
}

function createRuntime() {
  const context = {
    console,
    Image: LoadingImage,
    URLSearchParams,
    location: { search: `?scene=${nestedScene}` },
    document: { getElementById: () => null },
    fetch: (scenePath) => {
      assert.equal(scenePath, nestedScene);
      const absolute = path.join(repoRoot, scenePath.replace(/^\//, ''));
      return Promise.resolve({ json: () => Promise.resolve(JSON.parse(fs.readFileSync(absolute, 'utf8'))) });
    },
    addEventListener: () => {},
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of scripts) {
    vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  }
  return context.__OUROFORGE__;
}

(async () => {
  const api = createRuntime();
  const state = await api.whenReady();
  const sheetAsset = state.assets.find((asset) => asset.id === 'collect_and_exit_sheet');
  assert.ok(sheetAsset, 'collect_and_exit_sheet must be present in runtime assets');
  assert.equal(
    sheetAsset.resolvedPath,
    '/examples/playable-demo-v2/collect-and-exit/assets/sprites/collect-and-exit-sheet.png',
    'nested session-k scene paths must resolve assets from the demo root, not the nested scenes/ folder',
  );
  assert.equal(sheetAsset.status, 'loaded');
  assert.equal(state.runtimeDiagnostics.filter((diag) => diag.code === 'missing_asset').length, 0);
  console.log('nested scene asset resolve smoke passed');
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});