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
    fetch: () => Promise.reject(new Error('fetch disabled in frame budget test')),
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
scene.metadata = {
  ...(scene.metadata || {}),
  scenarioId: 'frame-budget-smoke',
  runtimeDebug: {
    frameTimings: { updateMs: 3, renderMs: 18.5, evidenceMs: 1, totalMs: 24.25 },
    frameBudget: { updateMs: 8, renderMs: 16, evidenceMs: 4, totalMs: 20 },
  },
};

const api = createRuntime();
const state = api.loadScene(scene);
const budget = state.runtimeFrameBudget;

assert.equal(budget.schemaVersion, 'ouroforge.runtime-frame-budget.v1');
assert.equal(budget.frameId, 'tick-0');
assert.equal(budget.sceneId, state.sceneId);
assert.equal(budget.scenarioId, 'frame-budget-smoke');
assert.equal(budget.status, 'violated');
assert.equal(budget.slowFrame, true);
assert.equal(Array.from(budget.violations).map((violation) => violation.field).join(','), 'renderMs,totalMs');
assert.equal(budget.counts.entityCount, state.entities.length);
assert.equal(budget.counts.drawCallCount, state.renderQueue.renderables.filter((renderable) => renderable.visible !== false).length);
assert.equal(Array.from(budget.readOnlyInspection.disallowedActions).join(','), 'trusted writes,command bridge,live mutation');
assert.equal(budget.authority, 'browser_runtime_evidence_input_not_profiler_truth');

const frameStats = api.getFrameStats();
assert.equal(frameStats.runtimeFrameBudgetStatus, 'violated');
assert.equal(frameStats.runtimeFrameBudgetViolationCount, 2);
assert.equal(frameStats.runtimeFrameBudgetCounts.entityCount, state.entities.length);

const scene3dState = api.loadScene({
  schemaVersion: '1',
  id: 'runtime-scene3d-render-smoke',
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
    nodes: [{ id: 'cube-node', meshRef: 'cube-mesh', localTransform: { translation: { x: 0, y: 0, z: 0 } } }],
  },
});
assert.equal(scene3dState.sceneKind, '3d');
assert.equal(scene3dState.entities.length, 0);
assert.equal(scene3dState.scene3dRender.present, true);
assert.equal(scene3dState.scene3dRender.cameraId, 'main-camera');
assert.equal(scene3dState.scene3dRender.attemptedObjectCount, 1);
assert.equal(scene3dState.scene3dRender.visibleObjectCount, 1);
assert.equal(scene3dState.scene3dRender.skippedObjectCount, 0);
const scene3dFrameStats = api.getFrameStats();
assert.equal(scene3dFrameStats.scene3dRenderFrameId, 'tick-0');
assert.equal(scene3dFrameStats.scene3dRenderAttemptedObjectCount, 1);
assert.equal(scene3dFrameStats.scene3dRenderVisibleObjectCount, 1);
assert.equal(scene3dFrameStats.scene3dRenderSkippedObjectCount, 0);

// A 3D scene with OMITTED entities must not inherit the default 2D demo entities.
const scene3dNoEntities = api.loadScene({
  schemaVersion: '1',
  id: 'runtime-scene3d-no-entities',
  sceneKind: '3d',
  bounds: { width: 320, height: 180 },
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
    nodes: [{ id: 'cube-node', meshRef: 'cube-mesh', localTransform: { translation: { x: 0, y: 0, z: 0 } } }],
  },
});
assert.equal(scene3dNoEntities.sceneKind, '3d');
assert.equal(scene3dNoEntities.entities.length, 0, '3D scene with omitted entities must not inject default 2D entities');

console.log('runtime frame budget smoke test passed');
