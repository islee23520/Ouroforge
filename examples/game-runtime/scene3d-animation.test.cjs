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
    fetch: () => Promise.reject(new Error('fetch disabled in scene3d animation test')),
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

const api = createRuntime();
const state = api.loadScene({
  schemaVersion: '1',
  id: 'runtime-scene3d-animation-smoke',
  sceneKind: '3d',
  bounds: { width: 320, height: 180 },
  entities: [],
  scene3d: {
    version: '1',
    activeCameraId: 'main-camera',
    cameras: [{
      id: 'main-camera',
      active: true,
      transform: { translation: { x: 0, y: 0, z: -8 } },
      projection: { kind: 'perspective', fovDegrees: 60 },
      viewport: { width: 320, height: 180 },
    }],
    meshes: [{ id: 'cube-mesh', kind: 'primitive', primitive: 'cube', materialRef: 'cube-mat' }],
    materials: [{ id: 'cube-mat', kind: 'unlit', baseColor: '#44ccff' }],
    colliders: [
      { id: 'cube-box', shape: 'box', body: 'dynamic', size: { x: 2, y: 2, z: 2 }, collisionGroup: 'actors', collisionMask: ['triggers'] },
      { id: 'goal-box', shape: 'box', body: 'trigger', trigger: true, size: { x: 2, y: 2, z: 2 }, collisionGroup: 'triggers' },
    ],
    nodes: [
      { id: 'cube-node', meshRef: 'cube-mesh', colliderRef: 'cube-box', localTransform: { translation: { x: 0, y: 0, z: 0 } } },
      { id: 'goal-node', colliderRef: 'goal-box', localTransform: { translation: { x: 2, y: 0, z: 0 } } },
      { id: 'warning-node', localTransform: { translation: { x: 6, y: 0, z: 0 } } },
    ],
    animationClips: [{
      id: 'cube-slide',
      targetNodeId: 'cube-node',
      channel: 'translation',
      durationFrames: 4,
      looped: false,
      keyframes: [
        { frame: 0, value: { x: 0, y: 0, z: 0 } },
        { frame: 2, value: { x: 2, y: 0, z: 0 } },
        { frame: 4, value: { x: 4, y: 0, z: 0 } },
      ],
    }],
    animationStates: [
      { clipId: 'cube-slide', targetNodeId: 'cube-node', channel: 'translation', currentFrame: 0, currentTimeMs: 0, playing: true, looped: false },
      { clipId: 'missing-preview', targetNodeId: 'warning-node', channel: 'rotation', currentFrame: 0, currentTimeMs: 0, playing: false, looped: false, missingClipWarning: 'missing clip preserved as warning evidence' },
    ],
  },
});

assert.equal(state.scene3dAnimation.present, true);
assert.equal(state.scene3dAnimation.stateCount, 2);
assert.equal(state.scene3dAnimation.warningCount, 1);
assert.equal(state.scene3d.nodes[0].localTransform.translation.x, 0);
assert.equal(state.scene3dRender.visibleObjectCount, 1);

const firstStep = api.step();
assert.equal(firstStep.scene3dAnimation.states[0].clipId, 'cube-slide');
assert.equal(firstStep.scene3dAnimation.states[0].currentFrame, 1);
assert.equal(JSON.stringify(firstStep.scene3dAnimation.states[0].value), JSON.stringify({ x: 1, y: 0, z: 0 }));
assert.equal(firstStep.scene3d.nodes[0].localTransform.translation.x, 1);
assert.equal(firstStep.scene3dAnimation.states[1].status, 'missing_clip');
assert.equal(firstStep.scene3dAnimation.warningCount, 1);
assert.ok(firstStep.scene3dAnimationEvents.some((event) => event.type === 'runtime.scene3d.animation.state' && event.clipId === 'cube-slide'));
assert.ok(api.getEvents().some((event) => event.type === 'runtime.scene3d.animation.state' && event.payload.clipId === 'cube-slide'));

const secondStep = api.step();
assert.equal(secondStep.scene3dAnimation.states[0].currentFrame, 2);
assert.equal(JSON.stringify(secondStep.scene3dAnimation.states[0].value), JSON.stringify({ x: 2, y: 0, z: 0 }));
assert.equal(secondStep.scene3dCollision.triggerCount, 1, 'animated cube reaches trigger volume deterministically');
assert.ok(secondStep.collisions.some((event) => event.type === 'runtime.scene3d.collision.trigger'));

const frameStats = api.getFrameStats();
assert.equal(frameStats.scene3dAnimationStateCount, 2);
assert.equal(frameStats.scene3dAnimationActiveStateCount, 1);
assert.equal(frameStats.scene3dAnimationWarningCount, 1);

api.step(3);
const finalState = api.getWorldState();
assert.equal(finalState.scene3dAnimation.states[0].currentFrame, 4);
assert.equal(finalState.scene3dAnimation.states[0].playing, false, 'non-looping clip stops at duration');
assert.equal(finalState.scene3d.nodes[0].localTransform.translation.x, 4);

console.log('scene3d animation runtime smoke test passed');
