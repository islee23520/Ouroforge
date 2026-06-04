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
    fetch: () => Promise.reject(new Error('fetch disabled in scene3d probe test')),
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
api.loadScene({
  schemaVersion: '1',
  id: 'runtime-scene3d-probe-smoke',
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
      { id: 'root-node', localTransform: { translation: { x: 10, y: 1, z: 0 }, rotation: { x: 0, y: 5, z: 0 }, scale: { x: 2, y: 1, z: 1 } } },
      { id: 'cube-node', parentId: 'root-node', meshRef: 'cube-mesh', colliderRef: 'cube-box', localTransform: { translation: { x: 2, y: 3, z: 4 } } },
      { id: 'goal-node', colliderRef: 'goal-box', localTransform: { translation: { x: 2, y: 0, z: 0 } } },
    ],
    animationClips: [{
      id: 'cube-slide',
      targetNodeId: 'cube-node',
      channel: 'translation',
      durationFrames: 2,
      looped: false,
      keyframes: [
        { frame: 0, value: { x: 2, y: 3, z: 4 } },
        { frame: 2, value: { x: 4, y: 3, z: 4 } },
      ],
    }],
    animationStates: [
      { clipId: 'cube-slide', targetNodeId: 'cube-node', channel: 'translation', currentFrame: 0, currentTimeMs: 0, playing: true, looped: false },
    ],
  },
});

const state = api.getWorldState();
assert.equal(state.scene3dProbe.schemaVersion, 'ouroforge.scene3d-runtime-probe.v1');
assert.equal(state.scene3dProbe.present, true);
assert.equal(state.scene3dProbe.status, 'present');
assert.equal(state.scene3dProbe.nodeCount, 3);
assert.equal(state.scene3dProbe.cameraCount, 1);
assert.equal(state.scene3dProbe.colliderCount, 2);
assert.equal(state.scene3dProbe.animationClipCount, 1);
assert.equal(state.scene3dProbe.animationStateCount, 1);
assert.match(state.scene3dProbe.boundary, /not trusted persistence/);
assert.deepEqual(state.scene3dProbe.readOnlyInspection.disallowedActions.includes('command bridge'), true);

assert.equal(state.scene3dCamera.schemaVersion, 'ouroforge.scene3d-camera-state.v1');
assert.equal(state.scene3dCamera.activeCameraId, 'main-camera');
assert.equal(state.camera.scene3dCamera.activeCameraId, 'main-camera');
assert.equal(state.camera.camera3d.activeCameraId, 'main-camera');

const cubeTransform = state.scene3dTransforms.transforms.find((entry) => entry.nodeId === 'cube-node');
assert.equal(state.scene3dTransforms.schemaVersion, 'ouroforge.scene3d-transform-probe.v1');
assert.equal(state.scene3dTransforms.transformCount, 3);
assert.equal(cubeTransform.parentId, 'root-node');
assert.equal(cubeTransform.depth, 1);
assert.equal(JSON.stringify(cubeTransform.localTransform.translation), JSON.stringify({ x: 2, y: 3, z: 4 }));
assert.equal(JSON.stringify(cubeTransform.worldTransform.translation), JSON.stringify({ x: 12, y: 4, z: 4 }));
assert.equal(JSON.stringify(cubeTransform.worldTransform.rotation), JSON.stringify({ x: 0, y: 5, z: 0 }));
assert.equal(JSON.stringify(cubeTransform.worldTransform.scale), JSON.stringify({ x: 2, y: 1, z: 1 }));

const stats = api.getFrameStats();
assert.equal(stats.scene3dCameraCount, 1);
assert.equal(stats.scene3dTransformNodeCount, 3);
assert.equal(stats.scene3dTransformCount, 3);
assert.equal(stats.scene3dAnimationStateCount, 1);
assert.equal(stats.scene3dCollisionEventCount, 0);

const legacy = createRuntime();
legacy.loadScene({ schemaVersion: '1', id: 'runtime-2d-probe-compat', bounds: { width: 64, height: 64 }, entities: [] });
const legacyState = legacy.getWorldState();
assert.equal(legacyState.scene3dProbe.present, false);
assert.equal(legacyState.scene3dProbe.status, 'unavailable');
assert.equal(legacyState.scene3dTransforms.present, false);
assert.equal(legacyState.scene3dCamera.present, false);
assert.equal(legacyState.input.left, false, 'existing 2D input probe shape remains available');
assert.equal(typeof legacy.getFrameStats().tick, 'number', 'existing 2D frame stats remain available');

console.log('scene3d runtime probe contract smoke test passed');
