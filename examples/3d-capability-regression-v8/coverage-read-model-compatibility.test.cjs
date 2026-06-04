const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const dashboard = require('../evidence-dashboard/dashboard.js');
const cockpit = require('../authoring-cockpit/cockpit.js');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const runtimeScripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function createRuntime() {
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in 3d coverage compatibility smoke')),
    addEventListener: () => {},
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of runtimeScripts) {
    vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, { filename: script });
  }
  return context.__OUROFORGE__;
}

function assertNoGeneratedFixtureState() {
  for (const name of ['runs', 'dashboard-data', 'target', 'tmp']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} remains generated/untracked`);
  }
}

function assertCoverageMatrix() {
  const matrix = readJson(path.join(fixtureDir, 'coverage-matrix.json'));
  assert.equal(matrix.issue, 606);
  assert.match(matrix.scope, /bounded local 3D capability/);
  assert.ok(matrix.guardrails.some((guardrail) => /#1 and #23 remain open/.test(guardrail)));
  assert.ok(matrix.guardrails.some((guardrail) => /not production 3D readiness or a Godot replacement/.test(guardrail)));

  const requiredFeatures = [
    'transform-hierarchy',
    'camera-projection',
    'mesh-material-refs',
    'render-smoke',
    'collision-trigger',
    'animation-playback',
    'runtime-probe-contract',
    'evaluator-pass-fail',
    'dashboard-read-model-compatibility',
    'studio-read-model-compatibility',
    '2d-compatibility-audit',
  ];
  const byId = new Map(matrix.features.map((feature) => [feature.id, feature]));
  for (const featureId of requiredFeatures) {
    const feature = byId.get(featureId);
    assert.ok(feature, `${featureId} is present in coverage matrix`);
    assert.equal(feature.status, 'covered', `${featureId} is covered`);
    assert.ok(Array.isArray(feature.scenarioIds) && feature.scenarioIds.length > 0, `${featureId} links scenario ids`);
    assert.ok(Array.isArray(feature.evidenceRefs) && feature.evidenceRefs.length > 0, `${featureId} links evidence refs`);
    for (const ref of feature.evidenceRefs) {
      assert.equal(fs.existsSync(path.join(repoRoot, ref)), true, `${featureId} evidence ref exists: ${ref}`);
    }
  }
  assert.ok(matrix.knownGaps.some((gap) => gap.id === 'production-3d-engine' && gap.status === 'out-of-scope'));
  assert.ok(matrix.knownGaps.some((gap) => gap.id === 'trusted-browser-writes' && /read-only/.test(gap.reason)));
}

function threeDEngineSummaries() {
  return {
    present: true,
    camera: {
      present: true,
      cameraCount: 0,
      layerCount: 0,
      scene3dCamera: {
        present: true,
        activeCameraId: 'probe-camera',
        cameraCount: 1,
        cameras: [
          {
            id: 'probe-camera',
            active: true,
            projection: { kind: 'perspective', fovDegrees: 60, near: 1, far: 1000 },
            aspectRatioX1000: 1778,
            viewport: { x: 0, y: 0, width: 320, height: 180 },
          },
        ],
        boundary: 'Read-only 3D camera evidence; no camera editor or viewport persistence.',
      },
    },
    scene3d_render: {
      present: true,
      frameId: 'frame-3d-compatibility',
      sceneId: 'probe-animation-regression',
      cameraId: 'probe-camera',
      meshCount: 2,
      materialCount: 2,
      visibleObjectCount: 2,
      skippedObjectCount: 0,
      screenshotArtifact: 'not produced',
      renderables: [
        { id: 'animated-cube', nodeId: 'animated-cube', primitive: 'cube', meshRef: 'animated-cube-mesh', materialRef: 'animated-blue', cameraId: 'probe-camera', visible: true },
        { id: 'animation-goal', nodeId: 'animation-goal', primitive: 'cube', meshRef: 'animation-goal-mesh', materialRef: 'goal-orange', cameraId: 'probe-camera', visible: true },
      ],
      fallbackReasons: [],
      boundary: 'Read-only 3D render smoke evidence; not a production renderer claim.',
    },
    render_breakdown: {
      present: true,
      frameId: 'frame-3d-compatibility',
      sceneId: 'probe-animation-regression',
      elements: [],
      absence_diagnostics: [],
      queue: { layerCount: 0, renderableCount: 0, drawCallCount: 0, renderables: [] },
      queue_validation: { status: 'ok' },
    },
    scene3d_collision: {
      present: true,
      contactCount: 1,
      triggerCount: 1,
      invalidColliderCount: 0,
      events: [{ type: 'runtime.scene3d.collision.trigger', nodeId: 'animated-cube', otherNodeId: 'animation-goal' }],
      invalidColliders: [],
      boundary: 'Read-only bounded 3D collision evidence; no full 3D physics engine claim.',
    },
    runtime_frame_budget: {
      frameId: 'frame-3d-compatibility',
      sceneId: 'probe-animation-regression',
      scenarioId: 'dashboard-present-missing-malformed-3d-fields',
      timings: { updateMs: 1, renderMs: 1, evidenceMs: 1, totalMs: 3 },
      budget: { updateMs: 8, renderMs: 16, evidenceMs: 4, totalMs: 20 },
      counts: { entityCount: 0, drawCallCount: 2, layerCount: 0, collisionPairCount: 1, activeAnimationCount: 1, activeVfxCount: 0, audioEventCount: 0 },
      status: 'within_budget',
      slowFrame: false,
      violations: [],
      authority: 'browser_runtime_evidence_input_not_profiler_truth',
      readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'live mutation'] },
    },
  };
}

function assertReadModelCompatibility() {
  const engine_summaries = threeDEngineSummaries();
  const run = {
    summary: { id: 'scenario-coverage-v8-3d-read-model', run_status: 'passed', verdict_status: 'passed', scenario_status: 'passed' },
    engine_summaries,
    evidence: [{ id: 'scene3d-render', path: 'evidence/scene3d-render.json', exists: true }],
  };

  assert.match(dashboard.renderCameraLayerSummary(engine_summaries), /3D cameras/);
  assert.match(dashboard.renderCameraLayerSummary(engine_summaries), /probe-camera/);
  assert.match(dashboard.renderRenderBreakdownSummary(engine_summaries), /3D render smoke/);
  assert.match(dashboard.renderRenderBreakdownSummary(engine_summaries), /animated-cube/);
  assert.match(dashboard.renderRuntimeProfilerSummary(engine_summaries), /Invalid 3D colliders/);
  assert.match(dashboard.renderRuntimeProfilerSummary(engine_summaries), /no full 3D physics engine claim/i);

  assert.match(dashboard.renderCameraLayerSummary({ camera: { scene3dCamera: { present: false, emptyState: 'No 3D camera evidence fixture' } } }), /No camera\/layer read model|No 3D camera evidence fixture/);
  assert.match(dashboard.renderRenderBreakdownSummary({ scene3d_render: { present: false, emptyState: 'No 3D render smoke fixture' }, render_breakdown: {} }), /No scene render breakdown evidence|No 3D render smoke fixture/);
  assert.match(dashboard.renderRuntimeProfilerSummary({ present: true, runtime_frame_budget: '<bad>' }), /No runtime profiler\/frame-budget read model/);

  assert.match(cockpit.renderStudioNavigation(run), /Camera\/layer inspection/);
  assert.match(cockpit.renderStudioNavigation(run), /probe-camera/);
  assert.match(cockpit.renderCameraLayerInspectionSurface(run), /3D cameras/);
  assert.match(cockpit.renderRenderBreakdownInspectionSurface(run), /3D render smoke/);
  assert.match(cockpit.renderRuntimeProfilerInspectionSurface(run), /Collision\/animation\/VFX\/audio/);
  assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /3D collision events/);
  assert.match(cockpit.renderIntegration(run), /Studio v2 demo surfaces/);
  assert.doesNotMatch(cockpit.renderIntegration(run), /trusted browser write|command bridge enabled|auto-merge enabled|Godot replacement/);

  assert.match(cockpit.renderCameraLayerInspectionSurface({ engine_summaries: { present: true, camera: { scene3dCamera: { present: false, emptyState: 'No 3D camera evidence fixture' } } } }), /No camera\/layer read model|No 3D camera evidence fixture/);
  assert.match(cockpit.renderRuntimeProfilerInspectionSurface({ engine_summaries: { present: true, runtime_frame_budget: '<bad>' } }), /missing or malformed/);
  const xss = cockpit.renderRenderBreakdownInspectionSurface({ engine_summaries: { present: true, scene3d_render: { present: true, frameId: '<script>bad</script>', sceneId: '<img>', renderables: [{ id: '<svg>', primitive: '<b>' }] }, render_breakdown: {} } });
  assert.match(xss, /&lt;svg&gt;/);
  assert.match(xss, /&lt;b&gt;/);
  assert.doesNotMatch(xss, /<script>bad<\/script>|<svg>|<img>/);
}

function assert2DCompatibility() {
  const api = createRuntime();
  const state = api.loadScene({ schemaVersion: '1', id: 'scenario-coverage-v8-legacy-2d', bounds: { width: 64, height: 64 }, entities: [] });
  assert.equal(state.sceneKind, '2d');
  assert.equal(state.scene3dProbe.status, 'unavailable');
  assert.equal(state.scene3dRender.present, false);
  assert.equal(state.scene3dCollision.present, false);
  assert.equal(state.scene3dAnimation.present, false);
  assert.equal(typeof api.getFrameStats().tick, 'number');
}

assertCoverageMatrix();
assertReadModelCompatibility();
assert2DCompatibility();
assertNoGeneratedFixtureState();
console.log('scenario coverage v8 3d coverage/read-model compatibility passed');
