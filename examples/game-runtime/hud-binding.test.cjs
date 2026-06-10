const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const repoRoot = path.resolve(runtimeDir, '..', '..');
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function element(id, extra = {}) {
  return {
    id, textContent: '', className: '', value: '', open: false, style: {}, addEventListener() {},
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
  const ids = ['game','debug','debug-panel','scene-name','run-state','status-message','hud-key','hud-gate','hud-exit','hud-player','hud-tick','hud-event','fps'];
  const elements = new Map(ids.map((id) => [id, element(id)]));
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
    addEventListener: () => {},
    requestAnimationFrame: () => 0,
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of scripts) vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  return { api: context.__OUROFORGE__, elements };
}

function hud(elements) {
  return {
    runState: elements.get('run-state').textContent,
    key: elements.get('hud-key').textContent,
    gate: elements.get('hud-gate').textContent,
    exit: elements.get('hud-exit').textContent,
    player: elements.get('hud-player').textContent,
    tick: elements.get('hud-tick').textContent,
    event: elements.get('hud-event').textContent,
    status: elements.get('status-message').textContent,
  };
}

function sample(label, state, elements) {
  return {
    label,
    tick: state.tick,
    flags: {
      key_collected: state.componentModel.goalFlags.key_collected === true,
      door_open: state.componentModel.goalFlags.door_open === true,
      exit_reached: state.componentModel.goalFlags.exit_reached === true,
      player_alive: state.componentModel.goalFlags.player_alive !== false,
    },
    hud: hud(elements),
  };
}

(async () => {
  const { api, elements } = createRuntime();
  const inventory = api.apiInventory();
  for (const key of ['getWorldState', 'getEvents', 'setInput', 'step', 'loadScene', 'whenReady']) {
    assert.ok(inventory.stable.includes(key), `${key} must be stable per M119.1`);
  }

  let state = await api.whenReady();
  const samples = [sample('start', state, elements)];
  assert.equal(samples[0].hud.key, 'Missing');
  assert.equal(samples[0].hud.gate, 'Closed');
  assert.equal(samples[0].hud.exit, 'Locked');
  assert.equal(samples[0].hud.player, 'Alive');
  assert.equal(elements.get('debug-panel').open, false);

  api.setInput({ right: true });
  state = api.step(40);
  samples.push(sample('key-collected', state, elements));
  assert.equal(samples[1].flags.key_collected, true);
  assert.equal(samples[1].flags.door_open, true);
  assert.equal(samples[1].flags.exit_reached, false);
  assert.equal(samples[1].hud.key, 'Collected');
  assert.equal(samples[1].hud.gate, 'Open');
  assert.equal(samples[1].hud.exit, 'Ready');
  assert.match(samples[1].hud.event, /Objective updated|trigger/i);

  samples.push(sample('gate-open', state, elements));
  assert.equal(samples[2].flags.door_open, true);
  assert.equal(samples[2].hud.gate, 'Open');

  state = api.step(45);
  samples.push(sample('win', state, elements));
  assert.equal(samples[3].flags.exit_reached, true);
  assert.equal(samples[3].hud.runState, 'Win');
  assert.equal(samples[3].hud.exit, 'Reached');
  assert.match(samples[3].hud.status, /^Win:/);

  const jsonl = samples.map((entry) => JSON.stringify(entry)).join('\n');
  for (const line of jsonl.split('\n')) assert.doesNotThrow(() => JSON.parse(line));
  console.log(`hud binding checkpoint world-samples verified\n${jsonl}`);
})().catch((error) => { console.error(error); process.exitCode = 1; });
