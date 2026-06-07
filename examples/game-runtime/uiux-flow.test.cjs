// Runtime contract test for UI/UX Flow, Onboarding and Accessibility v1 (#1660).
// Mirror of the Rust contract test crates/ouroforge-core/tests/uiux_flow_contract.rs.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'uiux-flow.js', 'runtime.js',
];

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in uiux flow test')),
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

function flowContract() {
  return JSON.parse(fs.readFileSync(path.join(runtimeDir, 'uiux-flow-v1.json'), 'utf8'));
}

function norm(value) {
  return JSON.parse(JSON.stringify(value));
}

function sceneWithFlow(flow) {
  return { schemaVersion: '1', id: 'uiux-scene', uiux: flow };
}

// Navigation flow is deterministic and probe-observable.
(() => {
  const api = createRuntime();
  api.loadScene(sceneWithFlow(flowContract()));
  let uiux = api.getWorldState().uiux;
  assert.equal(uiux.currentScreen, 'title');
  assert.equal(uiux.currentKind, 'menu');
  assert.equal(uiux.screenCount, 4);
  assert.equal(uiux.accessibilityOptionCount, 3);
  assert.equal(uiux.onboardingComplete, false);

  api.uiuxNavigate('start');
  api.uiuxNavigate('next');
  uiux = api.getWorldState().uiux;
  assert.equal(uiux.currentScreen, 'hud');
  assert.equal(uiux.onboardingComplete, true);
  assert.deepEqual(norm(uiux.visitedScreens), ['title', 'onboarding', 'hud']);

  // An undeclared navigation action is a deterministic no-op.
  const view = api.uiuxNavigate('does-not-exist');
  assert.equal(view.currentScreen, 'hud');
  assert.equal(view.lastNavigation.accepted, false);
  console.log('uiux navigation flow test passed');
})();

// Accessibility options take effect and are validated fail-closed.
(() => {
  const api = createRuntime();
  api.loadScene(sceneWithFlow(flowContract()));
  let uiux = api.getWorldState().uiux;
  assert.equal(uiux.accessibility.highContrast, false);
  assert.equal(uiux.accessibility.textScale, 'medium');

  api.uiuxSetAccessibility('highContrast', true);
  api.uiuxSetAccessibility('textScale', 'large');
  uiux = api.getWorldState().uiux;
  assert.equal(uiux.accessibility.highContrast, true);
  assert.equal(uiux.accessibility.textScale, 'large');

  assert.throws(() => api.uiuxSetAccessibility('textScale', 'enormous'), /not allowed/);
  assert.throws(() => api.uiuxSetAccessibility('noSuchOption', true), /unknown accessibility option/);
  assert.throws(() => api.uiuxSetAccessibility('highContrast', 'yes'), /requires a boolean/);
  console.log('uiux accessibility effect test passed');
})();

// The flow is exposed read-only through the runtime probe with its boundary.
(() => {
  const api = createRuntime();
  api.loadScene(sceneWithFlow(flowContract()));
  const state = api.getWorldState();
  assert.ok(state.uiux, 'probe exposes uiux flow');
  assert.equal(state.uiux.schema, 'ouroforge.uiux-flow-state.v1');
  assert.match(state.uiux.boundary, /read-only/);
  assert.equal(state.uiux.screenCount, 4);
  assert.equal(state.uiux.accessibilityOptionCount, 3);
  console.log('uiux probe observability test passed');
})();

// Malformed flow specs fail closed on scene load.
(() => {
  const api = createRuntime();
  const dupScreen = flowContract();
  dupScreen.screens.push({ id: 'title', kind: 'menu' });
  assert.throws(() => api.loadScene(sceneWithFlow(dupScreen)), /duplicate screen id/);

  const nonDeterministic = flowContract();
  nonDeterministic.transitions.push({ from: 'title', action: 'start', to: 'hud' });
  assert.throws(() => api.loadScene(sceneWithFlow(nonDeterministic)), /non-deterministic transition/);

  const unreachable = flowContract();
  unreachable.screens.push({ id: 'credits', kind: 'menu' });
  assert.throws(() => api.loadScene(sceneWithFlow(unreachable)), /unreachable/);

  const noAccessibility = flowContract();
  noAccessibility.accessibilityOptions = [];
  assert.throws(() => api.loadScene(sceneWithFlow(noAccessibility)), /at least one option/);

  const wrongBoundary = flowContract();
  wrongBoundary.boundary = 'browser-can-write';
  assert.throws(() => api.loadScene(sceneWithFlow(wrongBoundary)), /canonical read-only/);
  console.log('uiux fail-closed validation test passed');
})();

// Determinism: normalizing the same spec twice yields the same flow state.
(() => {
  const mod = require(path.join(runtimeDir, 'uiux-flow.js'));
  const a = mod.createState(flowContract());
  const b = mod.createState(flowContract());
  assert.deepEqual(norm(a), norm(b));
  console.log('uiux determinism test passed');
})();

console.log('all uiux flow tests passed');
