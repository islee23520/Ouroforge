const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const repoRoot = path.resolve(runtimeDir, '..', '..');
const scripts = ['collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js', 'renderer.js', 'tilemap.js', 'runtime.js'];

function element(id, extra = {}) {
  const listeners = {};
  return {
    id, textContent: '', className: '', value: '', open: false, style: {}, listeners,
    addEventListener(type, handler) { listeners[type] = handler; },
    click() { if (listeners.click) listeners.click({ preventDefault() {} }); },
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
  const ids = ['game','debug','debug-panel','scene-name','run-state','status-message','hud-key','hud-gate','hud-exit','hud-player','hud-tick','hud-event','fps','pause-toggle','restart-button'];
  const elements = new Map(ids.map((id) => [id, element(id)]));
  const windowListeners = {};
  const context = {
    console,
    Image: function Image() {},
    URLSearchParams,
    location: { search: '?scene=/examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json' },
    document: { getElementById: (id) => elements.get(id) || null },
    fetch: (scenePath) => {
      const absolute = path.join(repoRoot, String(scenePath).replace(/^\//, ''));
      return Promise.resolve({ json: () => Promise.resolve(JSON.parse(fs.readFileSync(absolute, 'utf8'))) });
    },
    addEventListener: (type, handler) => { windowListeners[type] = handler; },
    requestAnimationFrame: () => 0,
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of scripts) vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  return { api: context.__OUROFORGE__, elements, windowListeners };
}

(async () => {
  const { api, elements, windowListeners } = createRuntime();
  let state = await api.whenReady();
  const initialFlags = { ...state.componentModel.goalFlags };
  assert.equal(elements.get('run-state').textContent, 'Start');
  assert.equal(elements.get('pause-toggle').textContent, 'Pause');

  elements.get('pause-toggle').click();
  state = api.getWorldState();
  assert.equal(state.paused, true);
  assert.equal(elements.get('run-state').textContent, 'Paused');
  assert.equal(elements.get('pause-toggle').textContent, 'Resume');

  elements.get('pause-toggle').click();
  state = api.getWorldState();
  assert.equal(state.paused, false);
  assert.equal(elements.get('run-state').textContent, 'Start');

  api.setInput({ right: true });
  state = api.step(85);
  assert.equal(state.componentModel.goalFlags.key_collected, true);
  assert.equal(state.componentModel.goalFlags.door_open, true);
  assert.equal(state.componentModel.goalFlags.exit_reached, true);
  assert.equal(elements.get('run-state').textContent, 'Win');
  assert.match(elements.get('status-message').textContent, /^Win:/);

  elements.get('restart-button').click();
  state = api.getWorldState();
  assert.equal(state.tick, 0);
  assert.equal(state.paused, false);
  assert.equal(state.componentModel.goalFlags.key_collected, initialFlags.key_collected);
  assert.equal(state.componentModel.goalFlags.door_open, initialFlags.door_open);
  assert.equal(state.componentModel.goalFlags.exit_reached, initialFlags.exit_reached);
  assert.equal(state.componentModel.goalFlags.player_alive, initialFlags.player_alive);
  assert.equal(elements.get('run-state').textContent, 'Restarted');
  assert.match(elements.get('status-message').textContent, /^Restarted:/);

  windowListeners.keydown({ key: 'p', preventDefault() {} });
  assert.equal(api.getWorldState().paused, true);
  windowListeners.keydown({ key: 'r', preventDefault() {} });
  assert.equal(api.getWorldState().paused, false);
  assert.equal(elements.get('run-state').textContent, 'Restarted');

  const failScene = JSON.parse(fs.readFileSync(path.join(repoRoot, 'examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json'), 'utf8'));
  failScene.gameplayRules.flags = failScene.gameplayRules.flags.map((flag) => flag.id === 'player_alive' ? { ...flag, initial: false } : flag);
  failScene.entities = failScene.entities.map((entity) => entity.id === 'player'
    ? { ...entity, components: { ...entity.components, status: { ...entity.components.status, flags: [] } } }
    : entity);
  failScene.entities.push({
    id: 'fail_state_probe',
    sprite: { color: '#000000', visible: false },
    components: {
      transform: { x: 0, y: 0 }, velocity: { x: 0, y: 0 }, size: { width: 1, height: 1 },
      goalFlag: { flag: 'player_alive', label: 'Player alive', value: false },
    },
  });
  state = api.loadScene(failScene);
  assert.equal(state.componentModel.goalFlags.player_alive, false);
  assert.equal(elements.get('run-state').textContent, 'Fail');
  assert.match(elements.get('status-message').textContent, /^Fail:/);

  console.log('pause/restart/win/fail shell controls test passed');
})().catch((error) => { console.error(error); process.exitCode = 1; });
