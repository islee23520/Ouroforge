const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js',
  'snapshot.js',
  'assets.js',
  'animation.js',
  'audio.js',
  'renderer.js',
  'tilemap.js',
  'runtime.js',
];

function scene(id, extra = {}) {
  return {
    schemaVersion: '1',
    id,
    bounds: { width: 160, height: 90 },
    entities: [{
      id: 'player',
      sprite: { color: '#5eead4' },
      components: {
        transform: { x: 8, y: 12 },
        velocity: { x: 0, y: 0 },
        size: { width: 12, height: 10 },
        controllable: true,
      },
    }],
    ...extra,
  };
}

function createRuntime(fetchScenes = {}) {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: (target) => {
      if (!Object.prototype.hasOwnProperty.call(fetchScenes, target)) {
        return Promise.reject(new Error(`missing scene fixture: ${target}`));
      }
      return Promise.resolve({ json: () => Promise.resolve(fetchScenes[target]) });
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
  const firstScene = scene('transition-start', {
    sceneTransitions: [{ id: 'to_level_2', toScene: 'scenes/level-2.scene.json', label: 'Level 2' }],
  });
  const secondScene = scene('transition-level-2');
  const api = createRuntime({ 'scenes/level-2.scene.json': secondScene });
  api.loadScene(firstScene);

  const initial = api.getWorldState();
  assert.equal(initial.sceneId, 'transition-start');
  assert.equal(initial.sceneTransitions.length, 1);
  assert.equal(initial.sceneTransitions[0].id, 'to_level_2');

  const transitioned = await api.transition('to_level_2');
  assert.equal(transitioned.sceneId, 'transition-level-2');
  assert.equal(transitioned.tick, 0);
  assert.equal(transitioned.transitionEvents.length, 1);
  assert.deepEqual(
    {
      status: transitioned.transitionEvents[0].status,
      id: transitioned.transitionEvents[0].id,
      fromSceneId: transitioned.transitionEvents[0].fromSceneId,
      toScene: transitioned.transitionEvents[0].toScene,
      toSceneId: transitioned.transitionEvents[0].toSceneId,
    },
    {
      status: 'succeeded',
      id: 'to_level_2',
      fromSceneId: 'transition-start',
      toScene: 'scenes/level-2.scene.json',
      toSceneId: 'transition-level-2',
    },
  );
  assert.ok(api.getEvents().some((event) => event.type === 'runtime.scene.transition.succeeded'));

  await assert.rejects(
    () => api.transition('missing'),
    /transition missing is not declared by current scene/,
  );
  const afterMissing = api.getWorldState();
  assert.equal(afterMissing.sceneId, 'transition-level-2');
  assert.equal(afterMissing.transitionEvents.at(-1).status, 'failed');
  assert.equal(afterMissing.transitionEvents.at(-1).id, 'missing');

  const unsafeApi = createRuntime({});
  unsafeApi.loadScene(scene('unsafe-transition-start', {
    sceneTransitions: [{ id: 'escape', toScene: '../secret.scene.json' }],
  }));
  await assert.rejects(
    () => unsafeApi.transition('escape'),
    /target is not a bounded scene path/,
  );
  assert.equal(unsafeApi.getWorldState().sceneId, 'unsafe-transition-start');
  assert.equal(unsafeApi.getWorldState().transitionEvents[0].status, 'failed');

  // Percent-encoded parent-directory traversal must be rejected too: a literal
  // `..` check alone would let `/examples/%2e%2e/secret.scene.json` through and
  // fetch/URL handling could normalize it back into a parent escape.
  const encodedApi = createRuntime({});
  encodedApi.loadScene(scene('encoded-transition-start', {
    sceneTransitions: [{ id: 'escape', toScene: '/examples/%2e%2e/secret.scene.json' }],
  }));
  await assert.rejects(
    () => encodedApi.transition('escape'),
    /target is not a bounded scene path/,
  );
  assert.equal(encodedApi.getWorldState().sceneId, 'encoded-transition-start');
  assert.equal(encodedApi.getWorldState().transitionEvents[0].status, 'failed');
})();
