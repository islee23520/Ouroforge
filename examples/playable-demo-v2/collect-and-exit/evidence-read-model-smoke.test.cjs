const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vm = require('node:vm');

const fixtureDir = __dirname;
const repoRoot = path.resolve(fixtureDir, '..', '..', '..');
const runtimeDir = path.join(repoRoot, 'examples', 'game-runtime');
const dashboard = require(path.join(repoRoot, 'examples', 'evidence-dashboard', 'dashboard.js'));
const cockpit = require(path.join(repoRoot, 'examples', 'authoring-cockpit', 'cockpit.js'));
const scripts = [
  'collision.js', 'snapshot.js', 'assets.js', 'animation.js', 'audio.js',
  'renderer.js', 'tilemap.js', 'runtime.js',
];

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function createRuntime(scene) {
  const context = {
    console,
    Image: function Image() {},
    URLSearchParams,
    location: { search: '' },
    document: { getElementById: () => null },
    fetch: () => Promise.resolve({ json: () => Promise.resolve(scene) }),
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

function generatedStateAudit() {
  for (const name of ['runs', 'target', 'dashboard-data']) {
    assert.equal(fs.existsSync(path.join(fixtureDir, name)), false, `${name} must stay generated/untracked`);
  }
}

(async () => {
  const scene = readJson(path.join(fixtureDir, 'scenes', 'collect-and-exit.scene.json'));
  const api = createRuntime(scene);
  await api.whenReady();
  const startSave = api.createSave('demo-start');
  api.setInput({ right: true });
  api.step(40);
  const worldState = api.step(45);
  const frameStats = api.getFrameStats();
  const runtimeEvents = api.getEvents();

  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ouroforge-demo-read-model-'));
  const worldRef = path.join(tempDir, 'world-state.json');
  const dashboardRef = path.join(tempDir, 'dashboard-data.json');
  fs.writeFileSync(worldRef, JSON.stringify(worldState, null, 2));

  const engine_summaries = {
    present: true,
    source_world_state: worldRef,
    scene: { sceneId: worldState.sceneId, entityCount: worldState.entities.length, tick: worldState.tick },
    renderer: worldState.renderer,
    camera: worldState.camera,
    render_breakdown: { present: true, ...worldState.renderBreakdown },
    render_queue: { present: true, ...worldState.renderQueue },
    runtime_frame_budget: { present: true, ...worldState.runtimeFrameBudget },
    tilemaps: worldState.tilemaps,
    input: {
      present: true,
      mappedActionCount: Object.keys(worldState.actionState || {}).length,
      activeActionCount: Object.values(worldState.actionState || {}).filter(Boolean).length,
      activeActions: Object.entries(worldState.actionState || {}).filter(([, active]) => active).map(([action]) => action),
      warningCount: worldState.inputDiagnostics?.warningCount || 0,
      rawInput: worldState.rawInput,
      diagnostics: worldState.inputDiagnostics,
    },
    gameplay: {
      present: true,
      worldFlagCount: Object.keys(worldState.componentModel.goalFlags || {}).length,
      trueFlagCount: Object.values(worldState.componentModel.goalFlags || {}).filter(Boolean).length,
      falseFlagCount: Object.values(worldState.componentModel.goalFlags || {}).filter((value) => value === false).length,
      triggerCollisionEventCount: runtimeEvents.filter((event) => event.type === 'runtime.trigger.entered').length,
      hudValueEntityCount: worldState.componentModel.counts.hudValue,
      hudValues: worldState.componentModel.hudValues,
      trueFlags: Object.entries(worldState.componentModel.goalFlags || {}).filter(([, value]) => value === true).map(([flag]) => flag),
    },
    animation: { animatedEntityCount: worldState.runtimeFrameBudget.counts.activeAnimationCount, activeStateCount: worldState.runtimeFrameBudget.counts.activeAnimationCount },
    vfx: { present: true, vfxEntityCount: 1, vfxEmitterCount: 1, vfxEventCount: worldState.vfxEvents.length },
    audio: {
      audioEntityCount: worldState.entities.filter((entity) => entity.components && entity.components.audio).length,
      audioEventCount: worldState.audioEvents.length,
      audioWarningCount: worldState.audioWarnings.length,
      browserAudioAuthority: 'intent_evidence_only',
      audioEvents: worldState.audioEvents,
      audioWarnings: worldState.audioWarnings,
    },
    events: {
      present: true,
      animationEntityCount: worldState.runtimeFrameBudget.counts.activeAnimationCount,
      audioEventCount: worldState.audioEvents.length,
      audioWarningCount: worldState.audioWarnings.length,
      vfxEventCount: worldState.vfxEvents.length,
      collisionEventCount: runtimeEvents.filter((event) => event.type === 'runtime.collision.trigger').length,
      animationEntities: worldState.entities.filter((entity) => entity.components && entity.components.animation).map((entity) => ({ entityId: entity.id, ...entity.components.animation.state, activeState: entity.components.animation.state.currentClip })),
      audioEvents: worldState.audioEvents,
      audioWarnings: worldState.audioWarnings,
      vfxEvents: worldState.vfxEvents,
      collisionEvents: runtimeEvents.filter((event) => event.type.startsWith('runtime.collision') || event.type.startsWith('runtime.trigger')),
    },
    collision: { present: true, collisionEventCount: runtimeEvents.filter((event) => event.type === 'runtime.collision.trigger').length, events: runtimeEvents.filter((event) => event.type.startsWith('runtime.collision')) },
    components: { present: true, entityCount: worldState.entities.length, componentCounts: worldState.componentModel.counts, entities: worldState.componentModel.entities.map((entity) => ({ entityId: entity.entityId, components: Object.keys(entity.components || {}) })) },
    triggers: { present: true, triggerCount: worldState.componentModel.counts.trigger, triggerCollisionEventCount: runtimeEvents.filter((event) => event.type === 'runtime.trigger.entered').length, triggers: worldState.componentModel.entities.filter((entity) => entity.components && entity.components.trigger).map((entity) => ({ entityId: entity.entityId, id: entity.components.trigger.id, kind: entity.components.trigger.kind, targetFlag: entity.components.trigger.targetFlag, requiredFlags: entity.components.trigger.requiredFlags || [], onEnterCount: (entity.components.trigger.onEnter || []).length })) },
    hud: { present: true, hudValueEntityCount: worldState.componentModel.counts.hudValue, values: worldState.componentModel.hudValues },
    physics: worldState.physics,
    reload: { reloadCount: worldState.reloads.length, lastStatus: null },
    composition: worldState.composition,
  };

  const run = {
    summary: {
      id: 'temp-collect-and-exit-read-model-smoke',
      run_dir: tempDir,
      seed_id: 'playable-demo.collect-and-exit',
      run_status: 'passed',
      verdict_status: 'passed',
      scenario_status: 'passed',
      evidence_count: 2,
      mutation_count: 0,
      worker_count: 1,
      evidence_categories: [{ id: 'world_states', label: 'World-state snapshots', count: 1, missing_count: 0, malformed_count: 0 }],
    },
    project: { id: 'collect_and_exit_demo', name: 'Collect and Exit Demo', manifestPath: 'ouroforge.project.json' },
    evidence: [{ id: 'world-state', kind: 'application/json', path: 'world-state.json', exists: true, value: worldState }],
    engine_summaries,
    runtime_save: startSave,
  };
  fs.writeFileSync(dashboardRef, JSON.stringify({ schema: 'ouroforge-dashboard-v1', runs: [run] }, null, 2));

  assert.match(dashboard.renderRunDetail(run), /Runtime profiler evidence/);
  assert.match(dashboard.renderRunDetail(run), /Collect and Exit Demo/);
  assert.match(dashboard.renderRuntimeProfilerSummary(engine_summaries), /within-budget/);
  assert.match(dashboard.renderRenderBreakdownSummary(engine_summaries), /Renderable elements/);
  assert.match(dashboard.renderAudioEvidenceSummary(engine_summaries), /Audio intent events/);
  assert.match(cockpit.renderStudioNavigation(run), /Runtime profiler inspection/);
  assert.match(cockpit.renderIntegration(run), /Render breakdown inspection/);
  assert.match(cockpit.renderIntegration(run), /Runtime profiler inspection/);
  assert.match(cockpit.renderRuntimeEventInspectionSurface(run), /Collision\/transition\/event inspection/);
  assert.doesNotMatch(cockpit.renderIntegration(run), /<script>|trusted browser writes? (?:enabled|available|active)|browser trusted writes? (?:enabled|available|active)|command bridge enabled/);

  fs.rmSync(tempDir, { recursive: true, force: true });
  generatedStateAudit();
  console.log(`collect-and-exit read-model smoke passed; generated temp dashboard removed: ${path.basename(tempDir)}`);
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
