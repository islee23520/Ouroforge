const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function element(id, extra = {}) {
  return {
    id,
    textContent: '',
    className: '',
    value: '',
    open: false,
    style: {},
    addEventListener() {},
    getContext() {
      return {
        clearRect() {}, fillRect() {}, strokeRect() {}, fillText() {}, strokeText() {},
        beginPath() {}, moveTo() {}, lineTo() {}, arc() {}, fill() {}, stroke() {}, save() {}, restore() {},
        translate() {}, scale() {}, drawImage() {}, measureText(text) { return { width: String(text).length * 6 }; },
      };
    },
    ...extra,
  };
}

function createRuntime() {
  const ids = [
    'game', 'debug', 'debug-panel', 'scene-name', 'run-state', 'status-message',
    'hud-key', 'hud-gate', 'hud-exit', 'hud-player', 'hud-tick', 'hud-event', 'fps',
  ];
  const elements = new Map(ids.map((id) => [id, element(id)]));
  const consoleErrors = [];
  const context = {
    console: { ...console, error: (...args) => consoleErrors.push(args.join(' ')) },
    Image: function Image() {},
    document: { getElementById: (id) => elements.get(id) || null },
    fetch: () => Promise.reject(new Error('fetch disabled in game UI shell test')),
    addEventListener: () => {},
    requestAnimationFrame: () => 0,
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of scripts) {
    vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  }
  return { api: context.__OUROFORGE__, elements, consoleErrors };
}

const html = fs.readFileSync(path.join(runtimeDir, 'index.html'), 'utf8');
assert.match(html, /<main class="runtime-shell"/);
assert.match(html, /<details class="debug-card" id="debug-panel">/);
const detailsStart = html.indexOf('<details class="debug-card"');
const debugPre = html.indexOf('<pre id="debug"');
const detailsEnd = html.indexOf('</details>', detailsStart);
assert.ok(debugPre > detailsStart && debugPre < detailsEnd, 'raw JSON debug pre stays inside collapsed details panel');

const scene = JSON.parse(fs.readFileSync(path.join(runtimeDir, 'scene-components-v2.json'), 'utf8'));
const { api, elements, consoleErrors } = createRuntime();
api.loadScene(scene);
assert.equal(elements.get('debug-panel').open, false, 'debug panel remains collapsed by default');
assert.equal(elements.get('run-state').textContent, 'Start');
assert.equal(elements.get('hud-key').textContent, 'Missing');
assert.equal(elements.get('hud-gate').textContent, 'Closed');
assert.equal(elements.get('hud-player').textContent, 'Alive');
assert.match(elements.get('status-message').textContent, /^Start:/);
assert.match(elements.get('debug').textContent, /"sceneId"/);
assert.deepEqual(consoleErrors, [], 'live shell render emits no fatal console errors in DOM smoke');

console.log('game runtime shell DOM smoke passed');
