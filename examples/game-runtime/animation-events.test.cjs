const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in animation event test')),
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

const scene = JSON.parse(fs.readFileSync(path.join(runtimeDir, 'scene.json'), 'utf8'));
const api = createRuntime();
api.loadScene(scene);
const loadedEvents = api.getEvents();
assert.equal(loadedEvents.some((event) => event.type === 'runtime.animation.state'), false);
api.step(1);
api.step(1);
const animationEvents = api.getEvents().filter((event) => event.type === 'runtime.animation.state');
assert.ok(animationEvents.length >= 2);
const frameTransition = animationEvents.find((event) => event.payload.entityId === 'player' && event.payload.to.frameIndex === 1);
assert.ok(frameTransition);
assert.equal(frameTransition.payload.sceneId, scene.id);
assert.equal(frameTransition.payload.mode, 'sprite_frame');
assert.equal(frameTransition.payload.currentClip, 'idle');
assert.deepEqual(JSON.parse(JSON.stringify(frameTransition.payload.from)), {
  mode: 'sprite_frame',
  currentClip: 'idle',
  frameIndex: 0,
  elapsedFrames: 1,
});
assert.deepEqual(JSON.parse(JSON.stringify(frameTransition.payload.to)), {
  mode: 'sprite_frame',
  currentClip: 'idle',
  frameIndex: 1,
  elapsedFrames: 2,
});


const stateScene = JSON.parse(JSON.stringify(scene));
const playerAnimation = stateScene.entities.find((entity) => entity.id === 'player').components.animation;
playerAnimation.stateClips = { idle: 'idle', run: 'run' };
playerAnimation.clips.push({
  id: 'run',
  frameDuration: 1,
  loop: true,
  frames: [
    { color: '#38bdf8', asset: 'player-sprite' },
    { color: '#0284c7', asset: 'player-sprite' },
  ],
});
const statefulApi = createRuntime();
statefulApi.loadScene(stateScene);
statefulApi.setInput({ right: true });
const statefulWorld = statefulApi.step(1);
const statefulPlayer = statefulWorld.entities.find((entity) => entity.id === 'player');
assert.equal(statefulPlayer.components.animation.state.activeState, 'run');
assert.equal(statefulPlayer.components.animation.state.currentClip, 'run');
const stateTransition = statefulApi.getEvents().find((event) => event.type === 'runtime.animation.state' && event.payload.activeState === 'run');
assert.ok(stateTransition, 'runtime emits active animation state transition evidence');
assert.equal(stateTransition.payload.from.activeState, 'idle');
assert.equal(stateTransition.payload.to.activeState, 'run');
