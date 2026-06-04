const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const dashboard = require('../evidence-dashboard/dashboard.js');
const cockpit = require('../authoring-cockpit/cockpit.js');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..');
const matrixPath = path.join(fixtureDir, 'coverage-matrix.json');

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function assertNoGeneratedFixtureState() {
  for (const name of ['runs', 'target', 'dashboard-data']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} remains generated/untracked`);
  }
}

function assertCoverageMatrix() {
  const matrix = readJson(matrixPath);
  assert.equal(matrix.issue, 592);
  assert.match(matrix.scope, /Source-like regression coverage/);
  assert.ok(matrix.guardrails.some((guardrail) => /#1 and #23 remain open/.test(guardrail)));
  const requiredFeatures = [
    'renderer-ordering',
    'camera-follow-clamp-parallax',
    'collision-physics',
    'input-action-mapping-and-replay',
    'save-load-snapshot-replay-divergence',
    'animation-vfx',
    'audio-intent-events',
    'frame-budget-profiler',
    'dashboard-read-model-compatibility',
    'studio-read-model-compatibility',
  ];
  const byId = new Map(matrix.features.map((feature) => [feature.id, feature]));
  for (const featureId of requiredFeatures) {
    const feature = byId.get(featureId);
    assert.ok(feature, `${featureId} is present in coverage matrix`);
    assert.equal(feature.status, 'covered', `${featureId} is covered or explicitly documented elsewhere`);
    assert.ok(Array.isArray(feature.scenarioIds) && feature.scenarioIds.length > 0, `${featureId} links scenario ids`);
    assert.ok(Array.isArray(feature.evidenceRefs) && feature.evidenceRefs.length > 0, `${featureId} links evidence refs`);
    for (const ref of feature.evidenceRefs) {
      assert.equal(fs.existsSync(path.join(repoRoot, ref)), true, `${featureId} evidence ref exists: ${ref}`);
    }
  }
  assert.ok(matrix.knownGaps.some((gap) => gap.id === '3d' && gap.status === 'out-of-scope'));
  assert.ok(matrix.guardrails.some((guardrail) => /no Godot replacement/.test(guardrail)));
}

function production2dSummary() {
  const runtimeFrameBudget = {
    frameId: 'prod-2d-frame-0001',
    sceneId: 'prod-2d-scene',
    scenarioId: 'dashboard-present-missing-malformed-fields',
    timings: { updateMs: 3, renderMs: 18.5, evidenceMs: 1, totalMs: 24.25 },
    budget: { updateMs: 8, renderMs: 16, evidenceMs: 4, totalMs: 20 },
    counts: { entityCount: 4, drawCallCount: 4, layerCount: 3, collisionPairCount: 1, activeAnimationCount: 1, activeVfxCount: 2, audioEventCount: 2 },
    status: 'violated',
    slowFrame: true,
    violations: [{ field: 'renderMs', actualMs: 18.5, budgetMs: 16 }],
    readOnlyInspection: { disallowedActions: ['trusted writes', 'command bridge', 'live mutation'] },
    authority: 'browser_runtime_evidence_input_not_profiler_truth',
  };
  const events = {
    present: true,
    animationEntityCount: 1,
    audioEventCount: 2,
    audioWarningCount: 1,
    vfxEventCount: 2,
    collisionEventCount: 1,
    animationEntities: [{ entityId: 'player', mode: 'sprite_frame', activeState: 'run', currentClip: 'run', frameIndex: 1 }],
    audioEvents: [{ name: 'player_spawn', intentKind: 'sound', busId: 'sfx', volume: 80 }],
    audioWarnings: [{ warning: 'audible_output_not_verified', requestId: 'audio-0-1' }],
    vfxEvents: [{ emitterId: 'run-dust', kind: 'trail', particleCount: 4 }],
  };
  return {
    present: true,
    source_world_state: 'evidence/runtime-state/prod-2d-world-state.json',
    scene: { sceneId: 'prod-2d-scene', entityCount: 4, tick: 1 },
    animation: { animatedEntityCount: 1, activeStateCount: 1 },
    vfx: { present: true, vfxEntityCount: 1, vfxEmitterCount: 2, vfxEventCount: 2 },
    audio: { audioEntityCount: 1, audioEventCount: 2, audioWarningCount: 1, browserAudioAuthority: 'intent_evidence_only', audioEvents: events.audioEvents, audioWarnings: events.audioWarnings },
    events,
    runtime_frame_budget: runtimeFrameBudget,
    collision: { present: true, collisionEventCount: 1, events: [{ type: 'runtime.trigger.entered', payload: { triggerId: 'collect_key' } }] },
    transition: { present: true, declaredTransitionCount: 0, declaredTransitions: [], transitionEventCount: 0, transitions: [], lastReloadStatus: null },
    physics: { colliderEntityCount: 3, collisionEventCount: 1 },
    components: { present: true, entityCount: 4, componentCounts: { animation: 1, audio: 1, vfx: 1 }, entities: [{ entityId: 'player', components: ['animation', 'audio', 'vfx'] }] },
    triggers: { present: true, triggerCount: 1, triggerCollisionEventCount: 1, triggers: [{ entityId: 'key', id: 'collect_key', kind: 'overlap', targetFlag: 'key_collected', requiredFlags: [], onEnterCount: 3 }] },
    hud: { present: true, hudValueEntityCount: 1, values: [{ entityId: 'hud_goal', label: 'Goal', value: 'Collect key', bindFlag: 'exit_reached', flagValue: false }] },
  };
}

function assertReadModelCompatibility() {
  const engine_summaries = production2dSummary();
  const run = {
    summary: { id: 'production-2d-compatibility-smoke', run_status: 'passed', verdict_status: 'passed', scenario_status: 'passed' },
    engine_summaries,
    evidence: [{ id: 'world-state', path: 'evidence/runtime-state/prod-2d-world-state.json', exists: true }],
  };

  assert.match(dashboard.renderAnimationVfxSummary(engine_summaries), /Animated entities/);
  assert.match(dashboard.renderAnimationVfxSummary(engine_summaries), /VFX events/);
  assert.match(dashboard.renderAudioEvidenceSummary(engine_summaries), /Audio intent events/);
  assert.match(dashboard.renderAudioEvidenceSummary(engine_summaries), /Read-only audio intent evidence/);
  assert.match(dashboard.renderRuntimeProfilerSummary(engine_summaries), /Budget violations/);
  assert.match(dashboard.renderRuntimeProfilerSummary(engine_summaries), /browser observations are evidence inputs, not trusted authority/i);

  assert.match(dashboard.renderAnimationVfxSummary({}), /No animation\/VFX read model/);
  assert.match(dashboard.renderAudioEvidenceSummary({}), /No audio intent evidence/);
  assert.match(dashboard.renderRuntimeProfilerSummary({ present: true, runtime_frame_budget: '<bad>' }), /No runtime profiler\/frame-budget read model/);

  assert.match(cockpit.renderStudioNavigation(run), /Runtime profiler inspection/);
  assert.match(cockpit.renderStudioNavigation(run), /Collision\/transition\/event inspection/);
  assert.match(cockpit.renderEngineExpansionSurface(run), /Runtime profiler/);
  assert.match(cockpit.renderRuntimeProfilerInspectionSurface(run), /Budget violations/);
  assert.match(cockpit.renderRuntimeProfilerInspectionSurface(run), /Disallowed actions/);
  assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /Animation entities/);
  assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /Audio events/);
  assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /VFX events/);

  assert.match(cockpit.renderRuntimeProfilerInspectionSurface({ engine_summaries: { present: true, runtime_frame_budget: '<bad>' } }), /missing or malformed/);
  assert.match(cockpit.renderRuntimeEventInspectionSurface({ engine_summaries: { present: true, collision: '<bad>', transition: null, events: [] } }), /collision summary missing or malformed/);
  const xssProfiler = cockpit.renderRuntimeProfilerInspectionSurface({ engine_summaries: { present: true, runtime_frame_budget: { frameId: '<script>frame</script>', status: '<img>', violations: [{ field: '<svg>', actualMs: '<b>', budgetMs: '<i>' }] } } });
  assert.match(xssProfiler, /&lt;script&gt;frame&lt;\/script&gt;/);
  assert.doesNotMatch(xssProfiler, /<script>frame<\/script>|<svg>|<img>/);
  assert.doesNotMatch(cockpit.renderIntegration(run), /trusted browser write|command bridge enabled|<script>/);
}

assertCoverageMatrix();
assertReadModelCompatibility();
assertNoGeneratedFixtureState();
console.log('production 2d coverage/read-model compatibility passed');
