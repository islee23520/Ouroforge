(() => {
  const fixedDeltaMs = 16;

  const runtimeApiVersion = '119.1.0';
  const runtimeApiCompatibility = Object.freeze({
    schemaVersion: 'ouroforge.runtime-api-compatibility.v1',
    apiVersion: runtimeApiVersion,
    stabilityPolicy: 'Exported keys are snapshot-governed; additions/removals require inventory and test updates.',
    closureAuthority: 'contract-complete unless paired with live product-observed evidence',
    consumers: Object.freeze(['runtime-shell', 'studio', 'observability-harness']),
  });

  const runtimeDiagnosticTypes = Object.freeze([
    { code: 'scene_load_failed', severity: 'error', class: 'scene-load', description: 'Scene normalization, fetch, or transition loading failed.' },
    { code: 'invalid_query', severity: 'warning', class: 'invalid-query', description: 'A runtime query was malformed or targeted a missing entity.' },
    { code: 'missing_asset', severity: 'warning', class: 'missing-asset', description: 'An asset reference failed to resolve or load from the bounded manifest.' },
    { code: 'replay_sequence_failed', severity: 'error', class: 'replay', description: 'Replay sequence execution failed before final digest evidence.' },
    { code: 'replay_run_failed', severity: 'error', class: 'replay', description: 'One run in a replay determinism check failed.' },
    { code: 'replay_determinism_diverged', severity: 'error', class: 'replay', description: 'Same scene and seed produced divergent final state digests.' },
    { code: 'sampler_failed', severity: 'error', class: 'sampler', description: 'Runtime state sampler could not produce bounded evidence.' },
    { code: 'render_failed', severity: 'error', class: 'render', description: 'Runtime render evidence generation failed.' },
    { code: 'collision_trigger_anomaly', severity: 'warning', class: 'collision-trigger', description: 'Collision or trigger evidence was anomalous.' },
  ]);
  const runtimeDiagnosticTypeByCode = new Map(runtimeDiagnosticTypes.map((diagnostic) => [diagnostic.code, diagnostic]));

  const runtimeApiInventoryEntries = Object.freeze([
    ['apiCompatibility', 'stable', 'Compatibility policy for runtime shell, Studio, and observability harness consumers.'],
    ['apiInventory', 'stable', 'Machine-readable inventory generated from the actual frozen runtime API object.'],
    ['apiVersion', 'stable', 'Semver-like runtime API inventory version.'],
    ['compareReplayDigest', 'stable', 'Compare an expected replay digest against the current final state digest.'],
    ['createSave', 'stable', 'Create a generated runtime-state save artifact; browser trusted writes remain disabled.'],
    ['getEvents', 'stable', 'Read recent runtime events for observability harnesses.'],
    ['sampleRuntimeState', 'stable', 'Run a bounded runtime-state sampler and emit typed sampler diagnostics on failure.'],
    ['scenarioCoverageV100', 'test-only', 'Runs the M119 Scenario Coverage v100 planted diagnostic suite in bounded runtime scope.'],
    ['queryEntity', 'stable', 'Query an entity by id and emit typed invalid-query diagnostics.'],
    ['getDiagnostics', 'stable', 'Read structured runtime diagnostics emitted by the browser runtime.'],
    ['diagnosticTypes', 'stable', 'Machine-readable single-source runtime diagnostic enum consumed by classifiers.'],
    ['getFrameStats', 'stable', 'Read frame/render/collision counters for runtime shell diagnostics.'],
    ['getWorldState', 'stable', 'Read the browser runtime world-state evidence model.'],
    ['loadSave', 'stable', 'Restore a generated runtime-state save artifact.'],
    ['loadScene', 'stable', 'Load a bounded scene object into the browser runtime.'],
    ['nextRandom', 'stable', 'Advance the deterministic seeded RNG stream.'],
    ['pause', 'stable', 'Pause runtime stepping.'],
    ['replayStateDigest', 'stable', 'Capture a deterministic final state digest evidence record.'],
    ['replayDeterminismCheck', 'stable', 'Run the same scene+seed replay repeatedly and emit diagnostics on digest divergence.'],
    ['runReplay', 'stable', 'Run a deterministic key/action replay sequence and return final state digest evidence.'],
    ['restore', 'stable', 'Restore an in-memory snapshot by id.'],
    ['resume', 'stable', 'Resume runtime stepping.'],
    ['rngState', 'stable', 'Read seeded RNG state used by replay digesting.'],
    ['runtimeState', 'stable', 'Capture the canonical runtime state evidence payload used for digesting.'],
    ['seedRng', 'stable', 'Set the deterministic runtime RNG seed.'],
    ['setInput', 'stable', 'Set direction/key/action input state.'],
    ['snapshot', 'stable', 'Capture an in-memory runtime snapshot.'],
    ['step', 'stable', 'Advance the fixed-step runtime loop.'],
    ['transition', 'stable', 'Load a declared bounded scene transition.'],
    ['whenReady', 'stable', 'Promise for the initial fetched scene load.'],
    ['reload', 'experimental', 'Reload a scene from a bounded browser-side payload.'],
    ['deckRoguelikeEndTurn', 'experimental', 'Advance deck-roguelike runtime demo state.'],
    ['deckRoguelikePlayCard', 'experimental', 'Play a card in deck-roguelike runtime demo state.'],
    ['deckbuilderUiPlanRunMapNode', 'experimental', 'Draft-only deckbuilder run-map planning interaction.'],
    ['deckbuilderUiQueueSelected', 'experimental', 'Draft-only deckbuilder queue interaction.'],
    ['deckbuilderUiSelectCard', 'experimental', 'Deckbuilder hand selection interaction.'],
    ['deckbuilderUiSelectShopOffer', 'experimental', 'Draft-only deckbuilder shop selection interaction.'],
    ['uiuxNavigate', 'experimental', 'Navigate in-game UI/UX flow demo state.'],
    ['uiuxSetAccessibility', 'experimental', 'Update in-game UI/UX accessibility option demo state.'],
  ]);

  function runtimeApiInventory(apiObject) {
    const exportedKeys = Object.keys(apiObject).sort();
    const entryByKey = new Map(runtimeApiInventoryEntries.map(([key, stability, description]) => [key, {
      key,
      stability,
      description,
    }]));
    return {
      schemaVersion: 'ouroforge.runtime-api-inventory.v1',
      apiVersion: runtimeApiVersion,
      exportedKeys,
      stable: exportedKeys.filter((key) => entryByKey.get(key) && entryByKey.get(key).stability === 'stable'),
      experimental: exportedKeys.filter((key) => entryByKey.get(key) && entryByKey.get(key).stability === 'experimental'),
      testOnly: exportedKeys.filter((key) => entryByKey.get(key) && entryByKey.get(key).stability === 'test-only'),
      undocumented: exportedKeys.filter((key) => !entryByKey.has(key)),
      staleInventory: Array.from(entryByKey.keys()).filter((key) => !exportedKeys.includes(key)),
      entries: exportedKeys.map((key) => entryByKey.get(key) || {
        key,
        stability: 'undocumented',
        description: 'Missing runtime API inventory entry; update the inventory when changing exports.',
      }),
      compatibility: clone(runtimeApiCompatibility),
    };
  }
  const input = { left: false, right: false, up: false, down: false };
  const rawKeys = {};
  const actionInput = {};
  const events = [];
  const runtimeDiagnostics = [];
  const saveSlots = new Map();
  const collision = window.OuroforgeCollision || { detectAabbCollisions: () => [], scene3dCollisionSummary: () => ({ present: false, events: [] }) };
  const snapshotFactory = (window.OuroforgeSnapshots || {
    createSnapshotRegistry: () => ({
      capture: () => 'snapshot-unavailable',
      restore: () => { throw new Error('snapshot registry unavailable'); },
      metadata: () => null,
      list: () => [],
    }),
  });
  let snapshots = snapshotFactory.createSnapshotRegistry();
  const assets = (window.OuroforgeAssets || {
    createAssetTracker: () => ({
      load: () => [],
      metadata: () => [],
      imageFor: () => null,
    }),
  }).createAssetTracker({
    onChange: () => renderCanvas(),
    onEvent: (asset) => {
      record(`runtime.asset.${asset.status}`, {
        attemptId: asset.attemptId,
        assetId: asset.id,
        assetType: asset.kind,
        path: asset.path,
        status: asset.status,
        startedAtUnixMs: asset.startedAtUnixMs,
        endedAtUnixMs: asset.endedAtUnixMs,
        loadDurationMs: asset.loadDurationMs,
        width: asset.width,
        height: asset.height,
        failureReason: asset.failureReason,
      });
      if (asset.status === 'failed' || asset.status === 'rejected') {
        recordRuntimeDiagnostic('missing_asset', `Asset ${asset.id || asset.path || 'unknown'} did not load.`, {
          assetId: asset.id,
          assetType: asset.kind,
          path: asset.path,
          status: asset.status,
          failureReason: asset.failureReason,
          evidenceRefs: [asset.attemptId].filter(Boolean),
        });
      }
    },
  });
  const animation = window.OuroforgeAnimation || {
    normalizeAnimation: () => null,
    advanceAnimations: () => {},
    activeSpriteFrame: () => null,
  };
  const audio = window.OuroforgeAudio || {
    emitIntentEvents: () => [],
  };
  const juice = window.OuroforgeJuice || {
    normalizeJuiceConfig: () => ({ schemaVersion: 'ouroforge.runtime-juice-config.v1', enabled: false, primitives: [] }),
    createJuiceState: (_config, sceneId) => ({ schemaVersion: 'ouroforge.runtime-juice-state.v1', sceneId, config: { enabled: false, primitives: [] }, active: [], emitted: [] }),
    emitFeedback: (state) => ({ state, events: [] }),
    advanceFeedback: (state) => ({ state, events: [] }),
    worldStateView: (state) => ({ schemaVersion: 'ouroforge.runtime-juice-probe.v1', sceneId: state && state.sceneId, active: [], emitted: [], activeCount: 0, emittedCount: 0 }),
  };
  const tilemap = window.OuroforgeTilemap || {
    normalizeTilemaps: () => [],
    debugState: () => ({ version: '1', tilemaps: [], layerOrder: [] }),
    drawTilemaps: () => [],
  };
  const renderer = window.OuroforgeRenderer || {
    normalizeRenderer: (_renderer, bounds) => ({
      version: '1',
      camera: { x: 0, y: 0 },
      viewport: bounds || { width: 320, height: 180 },
      background: '#172532',
      layers: [{ id: 'default', order: 0, visible: true }],
      debug: { showBounds: false, showCamera: false, showEntityIds: false },
    }),
    debugState: (state) => ({
      version: (state && state.version) || '1',
      camera: (state && state.camera) || { x: 0, y: 0 },
      viewport: (state && state.viewport) || { width: 320, height: 180 },
      background: (state && state.background) || '#172532',
      layers: (state && Array.isArray(state.layers)) ? state.layers : [],
      debug: (state && state.debug) || { showBounds: false, showCamera: false, showEntityIds: false },
      renderedEntities: [],
    }),
    drawRuntime: () => [],
    renderBreakdown: ({ world = {}, renderer: state = {}, frameId = `tick-${world.tick ?? 0}` } = {}) => ({
      schemaVersion: 'ouroforge.scene-render-breakdown.v1',
      frameId: String(frameId),
      sceneId: String(world.sceneId || 'unknown'),
      camera: clone((state && state.camera) || { x: 0, y: 0 }),
      viewport: clone((state && state.viewport) || world.bounds || { width: 320, height: 180 }),
      elements: [],
      absenceDiagnostics: [],
      readOnlyInspection: { trustedEmitter: 'browser-runtime-evidence-helper', browserStudioMode: 'read-only evidence inspection', disallowedActions: ['trusted writes', 'command bridge', 'live mutation'] },
    }),
    scene3dRenderSummary: ({ world = {}, frameId = `tick-${world.tick ?? 0}` } = {}) => ({
      schemaVersion: 'ouroforge.scene3d-render-smoke.v1',
      present: false,
      frameId: String(frameId),
      sceneId: String(world.sceneId || 'unknown'),
      cameraId: null,
      meshCount: 0,
      materialCount: 0,
      attemptedObjectCount: 0,
      visibleObjectCount: 0,
      skippedObjectCount: 0,
      failedObjectCount: 0,
      screenshotArtifact: null,
      renderables: [],
      fallbackReasons: ['scene3d renderer unavailable'],
      boundary: 'Read-only bounded 3D render smoke evidence; no WebGPU, GLTF import, PBR, remote fetch, or production renderer claim.',
    }),
  };
  const gridPuzzleModule = (typeof window !== 'undefined' && window.OuroforgeGridPuzzle) || null;
  const uiuxFlowModule = (typeof window !== 'undefined' && window.OuroforgeUiuxFlow) || null;
  const deckRoguelikeModule = (typeof window !== 'undefined' && window.OuroforgeDeckRoguelike) || null;
  const deckbuilderUiModule = (typeof window !== 'undefined' && window.OuroforgeDeckbuilderUi) || null;
  const defaultScene = {
    schemaVersion: '1',
    id: 'fallback-scene',
    bounds: { width: 320, height: 180 },
    metadata: {},
    entities: [
      {
        id: 'player',
        sprite: { color: '#5eead4' },
        components: {
          transform: { x: 32, y: 72 },
          velocity: { x: 0, y: 0 },
          size: { width: 16, height: 16 },
          controllable: true,
        },
        tags: ['player'],
        metadata: {},
      },
    ],
  };
  const world = {
    schemaVersion: defaultScene.schemaVersion,
    sceneId: defaultScene.id,
    tick: 0,
    fixedDeltaMs,
    paused: false,
    sceneKind: '2d',
    bounds: clone(defaultScene.bounds),
    entities: clone(defaultScene.entities),
    scene3d: null,
    metadata: clone(defaultScene.metadata),
    collisions: [],
    collisionEvents: [],
    scene3dCollision: null,
    scene3dCollisions: [],
    scene3dAnimation: null,
    scene3dAnimationEvents: [],
    collisionRules: { defaultLayer: 'default' },
    gameplayRules: { version: '1', flags: [] },
    sceneTransitions: [],
    transitionEvents: [],
    audioEvents: [],
    audioWarnings: [],
    vfxEvents: [],
    juice: null,
    juiceEvents: [],
    reloads: [],
    tilemaps: [],
    cameras: [],
    activeCameraId: null,
    assetManifest: null,
    goalFlags: {},
    rng: { schemaVersion: 'runtime-seeded-rng-v1', algorithm: 'mulberry32', seed: 0, state: 0, drawCount: 0 },
    gridPuzzle: null,
    uiux: null,
    deckRoguelike: null,
    deckbuilderUi: null,
    physics: {
      gravity: 1,
      maxFallSpeed: 8,
      grounded: {},
      contacts: {},
      contactPairs: [],
      blockedMovement: {},
    },
  };
  let rendererState = renderer.normalizeRenderer(defaultScene.renderer, defaultScene.bounds);

  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function record(type, payload = {}) {
    events.push({ tick: world.tick, type, payload: clone(payload) });
    if (events.length > 64) events.shift();
  }

  function recordRuntimeDiagnostic(code, message, details = {}) {
    const type = runtimeDiagnosticTypeByCode.get(code) || { code, severity: 'warning', class: 'unknown' };
    const diagnostic = {
      schemaVersion: 'ouroforge.runtime-diagnostic.v1',
      code: type.code,
      class: type.class,
      severity: type.severity,
      message: String(message || type.description || code),
      sceneId: world.sceneId,
      tick: world.tick,
      evidenceRefs: Array.isArray(details.evidenceRefs) ? clone(details.evidenceRefs) : [],
      details: clone({ ...details, evidenceRefs: undefined }),
    };
    runtimeDiagnostics.push(diagnostic);
    if (runtimeDiagnostics.length > 64) runtimeDiagnostics.shift();
    record('runtime.diagnostic', diagnostic);
    return diagnostic;
  }

  function diagnosticTypes() {
    return {
      schemaVersion: 'ouroforge.runtime-diagnostic-types.v1',
      source: 'window.__OUROFORGE__.diagnosticTypes',
      types: clone(runtimeDiagnosticTypes),
    };
  }


  // Seeded stochastic determinism (Era F Milestone 31, #1600). All runtime
  // randomness derives from an explicit seed via a mulberry32 stream whose state
  // lives on `world.rng`, so it is captured by snapshot/restore and contributes
  // to the replay-state digest. No wall-clock, host entropy, or Math.random.
  const RNG_INCREMENT = 0x6d2b79f5;

  function normalizeSeed(value) {
    const numeric = Number(value);
    if (!Number.isFinite(numeric)) return 0;
    return Math.floor(numeric) >>> 0;
  }

  function seedRng(seed = 0) {
    const normalized = normalizeSeed(seed);
    world.rng = {
      schemaVersion: 'runtime-seeded-rng-v1',
      algorithm: 'mulberry32',
      seed: normalized,
      state: normalized,
      drawCount: 0,
    };
    record('runtime.rng.seeded', { seed: normalized, algorithm: 'mulberry32' });
    return clone(world.rng);
  }

  function nextRandom() {
    if (!world.rng || world.rng.algorithm !== 'mulberry32') seedRng(0);
    const rng = world.rng;
    rng.state = (rng.state + RNG_INCREMENT) >>> 0;
    let t = rng.state;
    t = Math.imul(t ^ (t >>> 15), 1 | t);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    const raw = (t ^ (t >>> 14)) >>> 0;
    rng.drawCount += 1;
    const draw = { drawIndex: rng.drawCount, seed: rng.seed, state: rng.state, raw, unit: raw / 4294967296 };
    record('runtime.rng.draw', { drawIndex: draw.drawIndex, state: draw.state, raw: draw.raw });
    return draw;
  }

  function rngState() {
    return clone(world.rng);
  }

  function player() {
    return world.entities.find((entity) => entity.id === 'player') || world.entities[0];
  }

  function point(value = {}, fallback = { x: 0, y: 0 }) {
    const source = value && typeof value === 'object' ? value : {};
    return {
      x: Number.isFinite(source.x) ? source.x : fallback.x,
      y: Number.isFinite(source.y) ? source.y : fallback.y,
    };
  }

  function vector3(value = {}, fallback = { x: 0, y: 0, z: 0 }) {
    const source = value && typeof value === 'object' ? value : {};
    return {
      x: Number.isFinite(source.x) ? source.x : fallback.x,
      y: Number.isFinite(source.y) ? source.y : fallback.y,
      z: Number.isFinite(source.z) ? source.z : fallback.z,
    };
  }

  function transform3(value = {}) {
    const source = value && typeof value === 'object' ? value : {};
    return {
      translation: vector3(source.translation),
      rotation: vector3(source.rotation),
      scale: vector3(source.scale, { x: 1, y: 1, z: 1 }),
    };
  }

  function size(value = {}, fallback = { width: 16, height: 16 }) {
    const source = value && typeof value === 'object' ? value : {};
    return {
      width: Number.isFinite(source.width) && source.width > 0 ? source.width : fallback.width,
      height: Number.isFinite(source.height) && source.height > 0 ? source.height : fallback.height,
    };
  }

  function objectValue(value) {
    return value && typeof value === 'object' && !Array.isArray(value) ? clone(value) : {};
  }

  function stableJson(value) {
    if (Array.isArray(value)) return `[${value.map(stableJson).join(',')}]`;
    if (value && typeof value === 'object') {
      return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${stableJson(value[key])}`).join(',')}}`;
    }
    return JSON.stringify(value);
  }

  function fnv1a64(value) {
    let hash = 0xcbf29ce484222325n;
    const prime = 0x100000001b3n;
    const inputText = stableJson(value);
    for (let index = 0; index < inputText.length; index += 1) {
      hash ^= BigInt(inputText.charCodeAt(index));
      hash = BigInt.asUintN(64, hash * prime);
    }
    return hash.toString(16).padStart(16, '0');
  }

  function stringList(value = []) {
    return Array.isArray(value) ? value.map(String) : [];
  }

  function normalizeActionMap(value = null) {
    const source = value && typeof value === 'object' && !Array.isArray(value) ? value : null;
    const actions = Array.isArray(source && source.actions) ? source.actions : [];
    return {
      actions: actions
        .filter((action) => action && typeof action === 'object' && typeof action.id === 'string')
        .map((action) => ({
          id: action.id,
          keyboard: stringList(action.keyboard).map((key) => key.toLowerCase()),
          gamepad: action.gamepad && typeof action.gamepad === 'object'
            ? {
              buttons: stringList(action.gamepad.buttons).map((button) => button.toLowerCase()),
              axes: Array.isArray(action.gamepad.axes)
                ? action.gamepad.axes
                  .filter((axis) => axis && typeof axis.axis === 'string')
                  .map((axis) => ({ axis: axis.axis.toLowerCase(), direction: String(axis.direction || '') }))
                : [],
            }
            : null,
        })),
    };
  }

  function statusComponent(value = {}, fallback = null) {
    const source = value && typeof value === 'object' ? value : {};
    const base = fallback && typeof fallback === 'object' ? fallback : {};
    const maxHitPoints = Number.isFinite(source.maxHitPoints)
      ? source.maxHitPoints
      : (Number.isFinite(base.maxHitPoints) ? base.maxHitPoints : null);
    const hitPoints = Number.isFinite(source.hitPoints)
      ? source.hitPoints
      : (Number.isFinite(base.hitPoints) ? base.hitPoints : maxHitPoints);
    return {
      hitPoints,
      maxHitPoints,
      flags: stringList(Object.prototype.hasOwnProperty.call(source, 'flags') ? source.flags : base.flags),
      states: stringList(Object.prototype.hasOwnProperty.call(source, 'states') ? source.states : base.states),
    };
  }

  function inputComponent(value = {}, fallback = null) {
    const source = value && typeof value === 'object' ? value : {};
    const base = fallback && typeof fallback === 'object' ? fallback : {};
    return {
      scheme: typeof source.scheme === 'string' ? source.scheme : (typeof base.scheme === 'string' ? base.scheme : 'keyboard'),
      moveSpeed: Number.isFinite(source.moveSpeed) ? source.moveSpeed : (Number.isFinite(base.moveSpeed) ? base.moveSpeed : 2),
      jumpImpulse: Number.isFinite(source.jumpImpulse) ? source.jumpImpulse : (Number.isFinite(base.jumpImpulse) ? base.jumpImpulse : null),
      allowedActions: stringList(Object.prototype.hasOwnProperty.call(source, 'allowedActions') ? source.allowedActions : base.allowedActions),
      actionMap: Object.prototype.hasOwnProperty.call(source, 'actionMap')
        ? normalizeActionMap(source.actionMap)
        : (base.actionMap ? clone(base.actionMap) : null),
    };
  }

  function triggerComponent(value = {}) {
    const source = value && typeof value === 'object' ? value : {};
    return {
      id: String(source.id || 'trigger'),
      kind: typeof source.kind === 'string' ? source.kind : 'overlap',
      targetFlag: typeof source.targetFlag === 'string' ? source.targetFlag : null,
      requiredFlags: stringList(source.requiredFlags),
      onEnter: Array.isArray(source.onEnter)
        ? source.onEnter
          .filter((action) => action && typeof action === 'object')
          .map((action) => ({
            kind: typeof action.kind === 'string' ? action.kind : 'setFlag',
            flag: typeof action.flag === 'string' ? action.flag : null,
            value: action.value !== false,
            entityId: typeof action.entityId === 'string' ? action.entityId : null,
          }))
        : [],
    };
  }

  function goalFlagComponent(value = {}) {
    const source = value && typeof value === 'object' ? value : {};
    const component = {
      flag: String(source.flag || ''),
      label: typeof source.label === 'string' ? source.label : null,
    };
    if (Object.prototype.hasOwnProperty.call(source, 'value')) component.value = Boolean(source.value);
    return component;
  }

  function cameraTargetComponent(value = {}) {
    const source = value && typeof value === 'object' ? value : {};
    const component = { weight: Number.isFinite(source.weight) ? source.weight : 1 };
    if (source.deadZone && typeof source.deadZone === 'object') component.deadZone = size(source.deadZone);
    return component;
  }

  function uiTextComponent(value = {}) {
    const source = value && typeof value === 'object' ? value : {};
    return {
      text: typeof source.text === 'string' ? source.text : '',
      role: typeof source.role === 'string' ? source.role : null,
      bindFlag: typeof source.bindFlag === 'string' ? source.bindFlag : null,
    };
  }

  function hudValueComponent(value = {}) {
    const source = value && typeof value === 'object' ? value : {};
    const allowedKinds = new Set(['score', 'health', 'inventory', 'key_count', 'goal', 'flag', 'text']);
    return {
      kind: typeof source.kind === 'string' && allowedKinds.has(source.kind) ? source.kind : 'text',
      label: typeof source.label === 'string' ? source.label : '',
      value: typeof source.value === 'string' ? source.value : '',
      bindFlag: typeof source.bindFlag === 'string' ? source.bindFlag : null,
    };
  }

  function vfxComponent(value = {}) {
    const source = value && typeof value === 'object' ? value : {};
    const emitters = Array.isArray(source.emitters) ? source.emitters : [];
    return {
      emitters: emitters.slice(0, 16).map((emitter, index) => ({
        id: typeof emitter.id === 'string' && emitter.id ? emitter.id : `emitter-${index}`,
        kind: ['burst', 'trail', 'spark'].includes(emitter.kind) ? emitter.kind : 'burst',
        trigger: ['tick', 'animation_state', 'manual'].includes(emitter.trigger) ? emitter.trigger : 'tick',
        disabled: emitter.disabled === true,
        particleCount: Number.isFinite(emitter.particleCount) ? Math.max(1, Math.min(64, Math.floor(emitter.particleCount))) : 1,
        lifetimeFrames: Number.isFinite(emitter.lifetimeFrames) ? Math.max(1, Math.min(120, Math.floor(emitter.lifetimeFrames))) : 1,
        color: typeof emitter.color === 'string' ? emitter.color : '#f8fafc',
        asset: typeof emitter.asset === 'string' ? emitter.asset : null,
        layer: typeof emitter.layer === 'string' ? emitter.layer : null,
      })),
    };
  }

  function normalizeEntity(entity = {}, index = 0, componentDefaults = {}) {
    const components = entity.components || {};
    const sprite = entity.sprite || {};
    const normalized = {
      id: String(entity.id || `entity-${index}`),
      sprite: {
        color: typeof sprite.color === 'string' ? sprite.color : '#f2f6f8',
        layer: typeof sprite.layer === 'string' ? sprite.layer : 'default',
        order: Number.isFinite(sprite.order) ? sprite.order : 0,
        visible: sprite.visible !== false,
      },
      components: {
        transform: point(components.transform, componentDefaults.transform || { x: 0, y: 0 }),
        velocity: point(components.velocity, componentDefaults.velocity || { x: 0, y: 0 }),
        size: size(components.size, componentDefaults.size || { width: 16, height: 16 }),
        controllable: Object.prototype.hasOwnProperty.call(components, 'controllable') ? Boolean(components.controllable) : Boolean(componentDefaults.controllable),
      },
      parent: typeof entity.parent === 'string' ? entity.parent : null,
      tags: Array.isArray(entity.tags) ? entity.tags.map(String) : [],
      metadata: objectValue(entity.metadata),
    };
    if (typeof sprite.asset === 'string') normalized.sprite.asset = sprite.asset;
    if (typeof sprite.frameId === 'string') normalized.sprite.frameId = sprite.frameId;
    if (typeof sprite.frame === 'string') normalized.sprite.frameId = sprite.frame;
    if (components.status || componentDefaults.status) {
      normalized.components.status = statusComponent(components.status, componentDefaults.status);
    }
    if (components.input || componentDefaults.input) {
      normalized.components.input = inputComponent(components.input, componentDefaults.input);
    }
    if (components.trigger) normalized.components.trigger = triggerComponent(components.trigger);
    if (components.goalFlag) normalized.components.goalFlag = goalFlagComponent(components.goalFlag);
    if (components.cameraTarget) normalized.components.cameraTarget = cameraTargetComponent(components.cameraTarget);
    if (components.uiText) normalized.components.uiText = uiTextComponent(components.uiText);
    if (components.hudValue) normalized.components.hudValue = hudValueComponent(components.hudValue);
    if (components.vfx && Array.isArray(components.vfx.emitters)) normalized.components.vfx = vfxComponent(components.vfx);
    if (components.audio && Array.isArray(components.audio.events)) {
      normalized.components.audio = {
        buses: Array.isArray(components.audio.buses)
          ? components.audio.buses
            .filter((bus) => bus && typeof bus.id === 'string' && typeof bus.kind === 'string')
            .map((bus) => ({
              id: bus.id,
              kind: bus.kind,
              volume: Number.isFinite(bus.volume) ? bus.volume : 100,
              muted: bus.muted === true,
            }))
          : [],
        events: components.audio.events
          .filter((event) => event && typeof event.trigger === 'string' && typeof event.name === 'string')
          .map((event) => ({
            name: event.name,
            trigger: event.trigger,
            action: event.action === 'stop' ? 'stop' : 'play',
            kind: typeof event.kind === 'string' ? event.kind : 'sound',
            bus: typeof event.bus === 'string' ? event.bus : null,
            asset: event.asset,
          })),
      };
    }
    const normalizedAnimation = animation.normalizeAnimation(components.animation);
    if (normalizedAnimation) normalized.components.animation = normalizedAnimation;
    if (components.collider) {
      normalized.components.collider = {
        shape: components.collider.shape || 'aabb',
        body: components.collider.body || 'static',
        offset: point(components.collider.offset),
        size: size(components.collider.size, normalized.components.size),
        sensor: Boolean(components.collider.sensor),
        trigger: Boolean(components.collider.trigger),
        disabled: Boolean(components.collider.disabled),
        collisionGroup: typeof components.collider.collisionGroup === 'string' ? components.collider.collisionGroup : null,
        collisionMask: Array.isArray(components.collider.collisionMask) ? components.collider.collisionMask.map(String) : [],
      };
    }
    return normalized;
  }


  function normalizeComponentDefaults(defaults = {}) {
    const source = defaults && typeof defaults === 'object' ? defaults : {};
    return {
      transform: point(source.transform),
      velocity: point(source.velocity),
      size: size(source.size),
      controllable: Boolean(source.controllable),
      status: source.status ? statusComponent(source.status) : null,
      input: source.input ? inputComponent(source.input) : null,
    };
  }

  function normalizeCollisionRules(collisionRules = {}) {
    const source = collisionRules && typeof collisionRules === 'object' ? collisionRules : {};
    return {
      defaultLayer: typeof source.defaultLayer === 'string' && source.defaultLayer ? source.defaultLayer : 'default',
    };
  }

  function normalizeGameplayRules(gameplayRules = null) {
    if (!gameplayRules || typeof gameplayRules !== 'object' || Array.isArray(gameplayRules)) return null;
    const source = objectValue(gameplayRules);
    const seen = new Set();
    return {
      ...source,
      version: typeof source.version === 'string' ? source.version : '1',
      flags: Array.isArray(source.flags)
        ? source.flags
          .filter((flag) => flag && typeof flag === 'object' && typeof flag.id === 'string' && flag.id)
          .filter((flag) => {
            if (seen.has(flag.id)) return false;
            seen.add(flag.id);
            return true;
          })
          .map((flag) => ({
            ...objectValue(flag),
            id: flag.id,
            initial: flag.initial === true,
          }))
        : [],
    };
  }

  function normalizeSceneTransitions(sceneTransitions = []) {
    const seen = new Set();
    return Array.isArray(sceneTransitions)
      ? sceneTransitions
        .filter((transition) => transition && typeof transition === 'object')
        .filter((transition) => typeof transition.id === 'string' && transition.id && typeof transition.toScene === 'string' && transition.toScene)
        .filter((transition) => {
          if (seen.has(transition.id)) return false;
          seen.add(transition.id);
          return true;
        })
        .map((transition) => ({
          id: transition.id,
          toScene: transition.toScene,
          label: typeof transition.label === 'string' ? transition.label : null,
        }))
      : [];
  }

  function resolveComposition(entities) {
    const byId = new Map(entities.map((entity) => [entity.id, entity]));
    const resolving = new Set();
    const resolved = new Set();

    function resolveEntity(entity) {
      if (resolved.has(entity.id)) return entity.components.transform;
      if (resolving.has(entity.id)) return entity.components.transform;
      resolving.add(entity.id);
      const localTransform = clone(entity.components.transform);
      let worldTransform = clone(localTransform);
      if (entity.parent && byId.has(entity.parent)) {
        const parent = byId.get(entity.parent);
        const parentTransform = resolveEntity(parent);
        worldTransform = {
          x: parentTransform.x + localTransform.x,
          y: parentTransform.y + localTransform.y,
        };
      }
      entity.components.localTransform = localTransform;
      entity.components.transform = worldTransform;
      entity.composition = {
        parent: entity.parent,
        localTransform,
        worldTransform: clone(worldTransform),
      };
      resolving.delete(entity.id);
      resolved.add(entity.id);
      return entity.components.transform;
    }

    for (const entity of entities.slice().sort((left, right) => (left.id < right.id ? -1 : left.id > right.id ? 1 : 0))) {
      resolveEntity(entity);
    }
    return entities;
  }

  function normalizeScene(scene = {}) {
    const sceneKind = scene.sceneKind === '3d' ? '3d' : '2d';
    // A 3D scene carries its content through the scene3d graph, so an omitted (or
    // empty) entities array must stay empty rather than inheriting the default 2D
    // demo entities.
    const defaultEntities = sceneKind === '3d' ? [] : defaultScene.entities;
    const sourceEntities = Array.isArray(scene.entities) && (scene.entities.length > 0 || sceneKind === '3d')
      ? scene.entities
      : defaultEntities;
    const bounds = size(scene.bounds, defaultScene.bounds);
    const componentDefaults = normalizeComponentDefaults(scene.componentDefaults);
    const normalizedRenderer = renderer.normalizeRenderer(scene.renderer, bounds);
    const deckRoguelike = normalizeDeckRoguelikeSpec(scene.deckRoguelike);
    const deckRoguelikeView = deckRoguelike && deckRoguelikeModule
      ? deckRoguelikeModule.worldStateView(deckRoguelike)
      : null;
    return {
      schemaVersion: String(scene.schemaVersion || defaultScene.schemaVersion),
      id: String(scene.id || 'unnamed-scene'),
      sceneKind,
      bounds,
      renderer: normalizedRenderer,
      activeCameraId: typeof scene.activeCameraId === 'string' ? scene.activeCameraId : null,
      cameras: normalizeCameras(scene.cameras, scene.activeCameraId, bounds, normalizedRenderer),
      tilemaps: tilemap.normalizeTilemaps(scene.tilemaps),
      collisionRules: normalizeCollisionRules(scene.collisionRules),
      gameplayRules: normalizeGameplayRules(scene.gameplayRules),
      sceneTransitions: normalizeSceneTransitions(scene.sceneTransitions),
      juice: juice.normalizeJuiceConfig(scene.juice),
      assetManifest: scene.assetManifest && typeof scene.assetManifest === 'object' ? objectValue(scene.assetManifest) : null,
      scene3d: sceneKind === '3d' && scene.scene3d && typeof scene.scene3d === 'object' && !Array.isArray(scene.scene3d)
        ? objectValue(scene.scene3d)
        : null,
      metadata: objectValue(scene.metadata),
      componentDefaults,
      entities: resolveComposition(sourceEntities.map((entity, index) => normalizeEntity(entity, index, componentDefaults))),
      gridPuzzle: normalizeGridPuzzleSpec(scene.gridPuzzle),
      uiux: normalizeUiuxFlowSpec(scene.uiux),
      deckRoguelike,
      deckbuilderUi: normalizeDeckbuilderUiSpec(scene.deckbuilderUi, deckRoguelikeView),
    };
  }

  // Build the initial grid-puzzle state for a scene, or null when the scene
  // declares no grid puzzle. Malformed grid-puzzle specs fail closed: the
  // module throws a clear diagnostic that propagates out of loadScene.
  function normalizeGridPuzzleSpec(spec) {
    if (spec === undefined || spec === null) return null;
    if (!gridPuzzleModule) {
      throw new Error('grid puzzle scene requires the OuroforgeGridPuzzle module to be loaded');
    }
    return gridPuzzleModule.createState(spec);
  }

  // Build the initial UI/UX flow state for a scene, or null when the scene
  // declares no flow. Malformed flow specs fail closed: the module throws a
  // clear diagnostic that propagates out of loadScene.
  function normalizeUiuxFlowSpec(spec) {
    if (spec === undefined || spec === null) return null;
    if (!uiuxFlowModule) {
      throw new Error('uiux flow scene requires the OuroforgeUiuxFlow module to be loaded');
    }
    return uiuxFlowModule.createState(spec);
  }

  // Build the initial deck-roguelike state for a scene, or null when the scene
  // declares no deck run. Malformed deck specs fail closed: the module throws a
  // clear diagnostic that propagates out of loadScene.
  function normalizeDeckRoguelikeSpec(spec) {
    if (spec === undefined || spec === null) return null;
    if (!deckRoguelikeModule) {
      throw new Error('deck roguelike scene requires the OuroforgeDeckRoguelike module to be loaded');
    }
    return deckRoguelikeModule.createState(spec);
  }


  // Build the initial deckbuilder UI state for a scene, or null when the scene
  // declares no deckbuilder UI. It consumes the current deck run as read-only
  // observation; malformed UI specs fail closed with a clear diagnostic.
  function normalizeDeckbuilderUiSpec(spec, deckRoguelikeView) {
    if (spec === undefined || spec === null) return null;
    if (!deckbuilderUiModule) {
      throw new Error('deckbuilder ui scene requires the OuroforgeDeckbuilderUi module to be loaded');
    }
    return deckbuilderUiModule.createState(spec, deckRoguelikeView);
  }

  function normalizeCamera(camera = {}, index = 0, activeCameraId = null, bounds = defaultScene.bounds, fallbackRenderer = rendererState) {
    const viewport = size(camera.viewport, fallbackRenderer.viewport || bounds);
    const id = String(camera.id || `camera-${index}`);
    const clamp = camera.clampBounds && typeof camera.clampBounds === 'object'
      ? {
        x: Number.isFinite(camera.clampBounds.x) ? camera.clampBounds.x : 0,
        y: Number.isFinite(camera.clampBounds.y) ? camera.clampBounds.y : 0,
        width: Number.isFinite(camera.clampBounds.width) && camera.clampBounds.width > 0 ? camera.clampBounds.width : bounds.width,
        height: Number.isFinite(camera.clampBounds.height) && camera.clampBounds.height > 0 ? camera.clampBounds.height : bounds.height,
      }
      : null;
    return {
      id,
      active: activeCameraId ? id === activeCameraId : index === 0,
      position: point(camera.position, fallbackRenderer.camera || { x: 0, y: 0 }),
      viewport,
      followTarget: typeof camera.followTarget === 'string' ? camera.followTarget : null,
      clampBounds: clamp,
      deadZone: camera.deadZone && typeof camera.deadZone === 'object' ? size(camera.deadZone, { width: 1, height: 1 }) : null,
      zoom: Number.isFinite(camera.zoom) && camera.zoom > 0 ? camera.zoom : 100,
    };
  }

  function normalizeCameras(cameras, activeCameraId, bounds, fallbackRenderer) {
    const list = Array.isArray(cameras) && cameras.length > 0
      ? cameras.map((camera, index) => normalizeCamera(camera, index, activeCameraId, bounds, fallbackRenderer))
      : [normalizeCamera({ id: 'default', position: fallbackRenderer.camera, viewport: fallbackRenderer.viewport }, 0, 'default', bounds, fallbackRenderer)];
    if (!list.some((camera) => camera.active)) list[0].active = true;
    return list;
  }


  function emitAudioEvents(trigger) {
    const emittedEvents = audio.emitIntentEvents({ entities: world.entities, trigger, tick: world.tick, muted: true });
    for (const emitted of emittedEvents) {
      emitted.sceneId = world.sceneId;
      world.audioEvents.push(emitted);
      if (world.audioEvents.length > 64) world.audioEvents.shift();
      for (const warning of emitted.limitationWarnings || []) {
        const record = { tick: world.tick, sceneId: world.sceneId, requestId: emitted.requestId, warning };
        world.audioWarnings.push(record);
        if (world.audioWarnings.length > 64) world.audioWarnings.shift();
      }
      record('runtime.audio.emitted', emitted);
    }
  }

  function recordJuiceAudioIntent(event) {
    const intent = event && event.sample && event.sample.audioIntent;
    if (!intent) return;
    const request = {
      kind: 'audio_request',
      requestId: `${event.feedbackId}-audio`,
      tick: world.tick,
      sceneId: world.sceneId,
      name: intent.name,
      trigger: `juice:${event.trigger}`,
      action: intent.action || 'play',
      intentKind: intent.kind || 'sound',
      busId: intent.bus || null,
      busKind: intent.kind || 'sound',
      volume: 100,
      busMuted: true,
      entityId: event.targetEntityId || null,
      asset: intent.asset || null,
      muted: true,
      playback: 'intent',
      limitationWarnings: ['browser_audio_intent_only', 'audible_output_not_verified', 'juice_sfx_hook'],
      sourceFeedbackId: event.feedbackId,
    };
    world.audioEvents.push(request);
    if (world.audioEvents.length > 64) world.audioEvents.shift();
    for (const warning of request.limitationWarnings) {
      const warningRecord = { tick: world.tick, sceneId: world.sceneId, requestId: request.requestId, warning };
      world.audioWarnings.push(warningRecord);
      if (world.audioWarnings.length > 64) world.audioWarnings.shift();
    }
    record('runtime.audio.emitted', request);
  }

  function storeJuiceEvents(emitted = []) {
    for (const event of emitted) {
      world.juiceEvents.push(event);
      if (world.juiceEvents.length > 64) world.juiceEvents.shift();
      record('runtime.juice.feedback', event);
      recordJuiceAudioIntent(event);
    }
  }

  function emitJuiceEvents(trigger, sourceEvent = null) {
    if (!world.juice) return [];
    const result = juice.emitFeedback(world.juice, trigger, { tick: world.tick, sceneId: world.sceneId, sourceEvent });
    world.juice = result.state || world.juice;
    const emitted = result.events || [];
    storeJuiceEvents(emitted);
    return emitted;
  }

  function advanceJuiceEvents() {
    if (!world.juice) return [];
    const result = juice.advanceFeedback(world.juice, world.tick);
    world.juice = result.state || world.juice;
    for (const event of result.events || []) {
      record('runtime.juice.feedback_update', event);
    }
    return result.events || [];
  }

  function keyAlias(key) {
    const normalized = String(key || '').toLowerCase();
    if (normalized === 'arrowleft') return 'left';
    if (normalized === 'arrowright') return 'right';
    if (normalized === 'arrowup') return 'up';
    if (normalized === 'arrowdown') return 'down';
    if (normalized === ' ') return 'space';
    return normalized;
  }

  function rawKeyPressed(key) {
    const normalized = keyAlias(key);
    if (Object.prototype.hasOwnProperty.call(input, normalized)) return Boolean(input[normalized]);
    return Boolean(rawKeys[normalized]);
  }

  function legacyActionState() {
    return {
      move: Boolean(input.left || input.right || input.up || input.down),
      move_left: Boolean(input.left),
      move_right: Boolean(input.right),
      move_up: Boolean(input.up),
      move_down: Boolean(input.down),
      jump: Boolean(input.up),
      interact: false,
    };
  }

  function actionMapState(inputComponent = {}) {
    const map = inputComponent.actionMap && Array.isArray(inputComponent.actionMap.actions)
      ? inputComponent.actionMap
      : null;
    if (!map) return legacyActionState();
    const resolved = {};
    for (const action of map.actions) {
      const keys = Array.isArray(action.keyboard) ? action.keyboard : [];
      resolved[action.id] = keys.some((key) => rawKeyPressed(key));
    }
    return { ...legacyActionState(), ...resolved };
  }

  function resolvedActionState(inputComponent = null) {
    const entity = player();
    const component = inputComponent || (entity && entity.components && entity.components.input) || {};
    return { ...actionMapState(component), ...clone(actionInput) };
  }

  function inputActionDiagnostics(inputComponent = null) {
    const entity = player();
    const component = inputComponent || (entity && entity.components && entity.components.input) || {};
    const allowedActions = Array.isArray(component.allowedActions) ? component.allowedActions : [];
    const mapActions = component.actionMap && Array.isArray(component.actionMap.actions)
      ? component.actionMap.actions
      : [];
    const actionIds = new Set();
    const keyboardBindings = new Map();
    const duplicateActions = [];
    const conflictingBindings = [];
    const unmappedActions = [];
    for (const action of mapActions) {
      if (!action || typeof action.id !== 'string') continue;
      if (actionIds.has(action.id)) duplicateActions.push(action.id);
      actionIds.add(action.id);
      const keys = Array.isArray(action.keyboard) ? action.keyboard : [];
      if (keys.length === 0) unmappedActions.push(action.id);
      for (const key of keys) {
        const normalized = keyAlias(key);
        const existing = keyboardBindings.get(normalized);
        if (existing && existing !== action.id) {
          conflictingBindings.push({ key: normalized, actions: [existing, action.id] });
        } else {
          keyboardBindings.set(normalized, action.id);
        }
      }
    }
    const missingActions = mapActions.length > 0
      ? allowedActions.filter((action) => !actionIds.has(action))
      : [];
    const explicitActions = Object.keys(actionInput);
    const unresolvedOverrides = explicitActions.filter((action) => mapActions.length > 0 && !actionIds.has(action));
    return {
      present: mapActions.length > 0 || explicitActions.length > 0,
      missingActions,
      conflictingBindings,
      duplicateActions,
      unmappedActions,
      unresolvedOverrides,
      warningCount: missingActions.length + conflictingBindings.length + duplicateActions.length + unmappedActions.length + unresolvedOverrides.length,
      readOnlyInspection: { trustedEmitter: 'browser-runtime-evidence-helper', browserStudioMode: 'read-only evidence inspection', disallowedActions: ['trusted writes', 'command bridge', 'live mutation'] },
    };
  }

  function actionAllowed(allowedActions, actionId, legacyGroup = null) {
    if (!Array.isArray(allowedActions) || allowedActions.length === 0) return true;
    return allowedActions.includes(actionId) || (legacyGroup ? allowedActions.includes(legacyGroup) : false);
  }

  function applyInput() {
    const entity = player();
    if (!entity || !entity.components) return;
    const velocity = entity.components.velocity;
    const inputComponent = entity.components.input || {};
    const allowedActions = Array.isArray(inputComponent.allowedActions) ? inputComponent.allowedActions : [];
    const actions = resolvedActionState(inputComponent);
    const canMoveLeft = actionAllowed(allowedActions, 'move_left', 'move');
    const canMoveRight = actionAllowed(allowedActions, 'move_right', 'move');
    const canMoveUp = actionAllowed(allowedActions, 'move_up', 'move');
    const canMoveDown = actionAllowed(allowedActions, 'move_down', 'move');
    const canJump = actionAllowed(allowedActions, 'jump') && Number.isFinite(inputComponent.jumpImpulse);
    const speed = Number.isFinite(inputComponent.moveSpeed) ? inputComponent.moveSpeed : 2;
    velocity.x = (((actions.move_right && canMoveRight) ? 1 : 0) - ((actions.move_left && canMoveLeft) ? 1 : 0)) * speed;
    if (canJump) {
      if (actions.jump && world.physics.grounded[entity.id]) {
        velocity.y = -inputComponent.jumpImpulse;
        world.physics.grounded[entity.id] = false;
        record('runtime.physics.jump', { entityId: entity.id, actionId: 'jump', impulse: inputComponent.jumpImpulse });
      }
    } else {
      velocity.y = (((actions.move_down && canMoveDown) ? 1 : 0) - ((actions.move_up && canMoveUp) ? 1 : 0)) * speed;
    }
  }

  function isDynamicPhysicsEntity(entity) {
    return Boolean(entity
      && entity.components
      && entity.components.collider
      && entity.components.collider.body === 'dynamic');
  }

  function applyGravity() {
    for (const entity of world.entities) {
      if (!isDynamicPhysicsEntity(entity)) continue;
      const velocity = entity.components.velocity;
      velocity.y = Math.min(world.physics.maxFallSpeed, (velocity.y || 0) + world.physics.gravity);
    }
  }

  function refreshGroundedState(collisionEvents) {
    const grounded = {};
    const contacts = {};
    const contactPairs = [];
    const blockedMovement = {};
    for (const entity of world.entities) {
      if (isDynamicPhysicsEntity(entity)) {
        grounded[entity.id] = false;
        contacts[entity.id] = [];
        blockedMovement[entity.id] = { x: false, y: false };
      }
    }
    for (const event of collisionEvents) {
      if (event.type !== 'runtime.collision.contact') continue;
      contactPairs.push({ pairId: event.pairId, normal: event.normal || { x: 0, y: 0 } });
      if (event.movingEntityId && contacts[event.movingEntityId]) {
        contacts[event.movingEntityId].push({
          pairId: event.pairId,
          otherEntityId: event.otherEntityId,
          normal: event.normal || { x: 0, y: 0 },
        });
      }
      if (event.movingEntityId && blockedMovement[event.movingEntityId] && event.normal) {
        if (event.normal.x !== 0) blockedMovement[event.movingEntityId].x = true;
        if (event.normal.y !== 0) blockedMovement[event.movingEntityId].y = true;
      }
      if (event.movingEntityId && event.normal && event.normal.y === -1) {
        grounded[event.movingEntityId] = true;
        const entity = entityById(event.movingEntityId);
        if (entity && entity.components && entity.components.velocity) {
          entity.components.velocity.y = Math.min(0, entity.components.velocity.y || 0);
        }
      }
    }
    world.physics.grounded = grounded;
    world.physics.contacts = contacts;
    world.physics.contactPairs = contactPairs;
    world.physics.blockedMovement = blockedMovement;
  }

  function entityById(entityId) {
    return world.entities.find((entity) => entity.id === entityId)
      || (typeof tilemap.entityById === 'function' ? tilemap.entityById(world.tilemaps, entityId) : null)
      || null;
  }

  function activeCamera() {
    return world.cameras.find((camera) => camera.active) || world.cameras[0] || null;
  }

  function clampCameraPosition(position, camera) {
    const viewport = camera && camera.viewport ? camera.viewport : rendererState.viewport;
    const clamp = camera && camera.clampBounds;
    if (!clamp) {
      return {
        x: Math.max(0, Math.min(world.bounds.width - viewport.width, position.x)),
        y: Math.max(0, Math.min(world.bounds.height - viewport.height, position.y)),
      };
    }
    return {
      x: Math.max(clamp.x, Math.min(clamp.x + clamp.width - viewport.width, position.x)),
      y: Math.max(clamp.y, Math.min(clamp.y + clamp.height - viewport.height, position.y)),
    };
  }

  function updateCameraState() {
    const camera = activeCamera();
    if (!camera) return;
    let next = point(camera.position, rendererState.camera);
    const target = camera.followTarget ? entityById(camera.followTarget) : null;
    if (target && target.components) {
      const transform = point(target.components.transform);
      const targetSize = size(target.components.size);
      next = {
        x: transform.x + (targetSize.width / 2) - (camera.viewport.width / 2),
        y: transform.y + (targetSize.height / 2) - (camera.viewport.height / 2),
      };
    }
    next = clampCameraPosition(next, camera);
    camera.position = clone(next);
    rendererState.camera = clone(next);
    rendererState.viewport = clone(camera.viewport);
  }

  function scene3dAnimationChannel(channel) {
    return ['translation', 'rotation', 'scale'].includes(channel) ? channel : null;
  }

  function scene3dNodeById(scene3d) {
    return new Map((Array.isArray(scene3d && scene3d.nodes) ? scene3d.nodes : [])
      .filter((node) => node && typeof node.id === 'string')
      .map((node) => [node.id, node]));
  }

  function scene3dClipById(scene3d) {
    return new Map((Array.isArray(scene3d && scene3d.animationClips) ? scene3d.animationClips : [])
      .filter((clip) => clip && typeof clip.id === 'string')
      .map((clip) => [clip.id, clip]));
  }

  function combineScene3dTransforms(parent, local) {
    const parentTransform = transform3(parent);
    const localTransform = transform3(local);
    return {
      translation: {
        x: parentTransform.translation.x + localTransform.translation.x,
        y: parentTransform.translation.y + localTransform.translation.y,
        z: parentTransform.translation.z + localTransform.translation.z,
      },
      rotation: {
        x: parentTransform.rotation.x + localTransform.rotation.x,
        y: parentTransform.rotation.y + localTransform.rotation.y,
        z: parentTransform.rotation.z + localTransform.rotation.z,
      },
      scale: {
        x: parentTransform.scale.x * localTransform.scale.x,
        y: parentTransform.scale.y * localTransform.scale.y,
        z: parentTransform.scale.z * localTransform.scale.z,
      },
    };
  }

  function scene3dTransformSummary({ scene3d = world.scene3d, frameId = `tick-${world.tick}` } = {}) {
    if (!scene3d || typeof scene3d !== 'object' || Array.isArray(scene3d)) {
      return {
        schemaVersion: 'ouroforge.scene3d-transform-probe.v1',
        present: false,
        frameId: String(frameId),
        sceneId: world.sceneId,
        nodeCount: 0,
        transformCount: 0,
        transforms: [],
        warnings: ['scene3d graph unavailable'],
        readOnlyInspection: {
          trustedEmitter: 'browser-runtime-3d-probe',
          browserStudioMode: 'read-only 3D transform hierarchy inspection',
          disallowedActions: ['trusted writes', 'command bridge', 'scene mutation', 'viewport persistence'],
        },
      };
    }
    const nodes = Array.isArray(scene3d.nodes) ? scene3d.nodes : [];
    const nodesById = scene3dNodeById(scene3d);
    const resolved = new Map();
    const warnings = [];
    function resolveNode(node, stack = []) {
      if (!node || typeof node.id !== 'string') return null;
      if (resolved.has(node.id)) return resolved.get(node.id);
      const localTransform = transform3(node.localTransform);
      let worldTransform = node.worldTransform && typeof node.worldTransform === 'object'
        ? transform3(node.worldTransform)
        : localTransform;
      let depth = 0;
      if (typeof node.parentId === 'string' || typeof node.parent === 'string') {
        const parentId = String(node.parentId || node.parent);
        const parent = nodesById.get(parentId);
        if (!parent) {
          warnings.push({ nodeId: node.id, warning: `missing parent ${parentId}` });
        } else if (stack.includes(node.id) || stack.includes(parentId)) {
          warnings.push({ nodeId: node.id, warning: `cycle detected near parent ${parentId}` });
        } else {
          const parentRow = resolveNode(parent, [...stack, node.id]);
          if (parentRow) {
            worldTransform = combineScene3dTransforms(parentRow.worldTransform, localTransform);
            depth = parentRow.depth + 1;
          }
        }
      }
      const row = {
        nodeId: node.id,
        parentId: typeof node.parentId === 'string' ? node.parentId : (typeof node.parent === 'string' ? node.parent : null),
        depth,
        localTransform,
        worldTransform,
        meshRef: typeof node.meshRef === 'string' ? node.meshRef : null,
        materialRef: typeof node.materialRef === 'string' ? node.materialRef : null,
        colliderRef: typeof node.colliderRef === 'string' ? node.colliderRef : null,
      };
      resolved.set(node.id, row);
      return row;
    }
    for (const node of nodes) resolveNode(node);
    const transforms = nodes
      .filter((node) => node && typeof node.id === 'string')
      .map((node) => resolved.get(node.id))
      .filter(Boolean);
    return {
      schemaVersion: 'ouroforge.scene3d-transform-probe.v1',
      present: true,
      frameId: String(frameId),
      sceneId: world.sceneId,
      nodeCount: nodes.length,
      transformCount: transforms.length,
      transforms,
      warnings,
      readOnlyInspection: {
        trustedEmitter: 'browser-runtime-3d-probe',
        browserStudioMode: 'read-only 3D transform hierarchy inspection',
        disallowedActions: ['trusted writes', 'command bridge', 'scene mutation', 'viewport persistence'],
      },
    };
  }

  function scene3dProbeSummary({ state, frameId = `tick-${world.tick}` }) {
    const scene3d = state && state.scene3d && typeof state.scene3d === 'object' && !Array.isArray(state.scene3d)
      ? state.scene3d
      : null;
    const transforms = scene3dTransformSummary({ scene3d, frameId });
    return {
      schemaVersion: 'ouroforge.scene3d-runtime-probe.v1',
      present: Boolean(scene3d),
      status: scene3d ? 'present' : 'unavailable',
      frameId: String(frameId),
      sceneId: world.sceneId,
      sceneKind: state && state.sceneKind ? state.sceneKind : '2d',
      nodeCount: scene3d && Array.isArray(scene3d.nodes) ? scene3d.nodes.length : 0,
      transformCount: transforms.transformCount,
      cameraCount: scene3d && Array.isArray(scene3d.cameras) ? scene3d.cameras.length : 0,
      colliderCount: scene3d && Array.isArray(scene3d.colliders) ? scene3d.colliders.length : 0,
      animationClipCount: scene3d && Array.isArray(scene3d.animationClips) ? scene3d.animationClips.length : 0,
      animationStateCount: state && state.scene3dAnimation ? state.scene3dAnimation.stateCount || 0 : 0,
      renderVisibleObjectCount: state && state.scene3dRender ? state.scene3dRender.visibleObjectCount || 0 : 0,
      collisionEventCount: state && Array.isArray(state.scene3dCollisions) ? state.scene3dCollisions.length : 0,
      transforms,
      activeCamera: state && state.scene3dCamera ? state.scene3dCamera : null,
      render: state && state.scene3dRender ? state.scene3dRender : null,
      collision: state && state.scene3dCollision ? state.scene3dCollision : null,
      animation: state && state.scene3dAnimation ? state.scene3dAnimation : null,
      readOnlyInspection: {
        trustedEmitter: 'browser-runtime-3d-probe',
        browserStudioMode: 'read-only 3D runtime probe inspection',
        disallowedActions: ['trusted writes', 'command bridge', 'scene mutation', 'viewport persistence', 'trusted persistence'],
      },
      boundary: 'Browser-local read-only 3D probe evidence; not trusted persistence, command execution, production 3D parity, or a Godot replacement claim.',
    };
  }

  function scene3dCameraProbeSummary({ scene3d = world.scene3d, frameId = `tick-${world.tick}` } = {}) {
    if (!scene3d || typeof scene3d !== 'object' || Array.isArray(scene3d)) {
      return {
        schemaVersion: 'ouroforge.scene3d-camera-state.v1',
        present: false,
        frameId: String(frameId),
        sceneId: world.sceneId,
        activeCameraId: null,
        activeCamera: null,
        cameraCount: 0,
        cameras: [],
        emptyState: 'No scene3d camera state is available in the runtime probe.',
        readOnlyInspection: {
          trustedEmitter: 'browser-runtime-3d-probe',
          browserStudioMode: 'read-only 3D camera evidence inspection',
          disallowedActions: ['trusted writes', 'command bridge', 'scene mutation', 'viewport persistence', 'camera editor tooling'],
        },
      };
    }
    const cameras = Array.isArray(scene3d.cameras) ? scene3d.cameras : [];
    const activeCameraId = typeof scene3d.activeCameraId === 'string'
      ? scene3d.activeCameraId
      : (cameras.find((camera) => camera && camera.active === true) || cameras[0] || {}).id || null;
    const rows = cameras
      .filter((camera) => camera && typeof camera.id === 'string')
      .map((camera) => ({
        id: camera.id,
        active: camera.id === activeCameraId || camera.active === true,
        nodeId: typeof camera.nodeId === 'string' ? camera.nodeId : null,
        transform: transform3(camera.transform),
        projection: clone(camera.projection || {}),
        viewport: clone(camera.viewport || {}),
      }));
    const activeCamera = rows.find((camera) => camera.id === activeCameraId) || rows.find((camera) => camera.active) || null;
    return {
      schemaVersion: 'ouroforge.scene3d-camera-state.v1',
      present: rows.length > 0,
      frameId: String(frameId),
      sceneId: world.sceneId,
      activeCameraId,
      activeCamera,
      cameraCount: rows.length,
      cameras: rows,
      readOnlyInspection: {
        trustedEmitter: 'browser-runtime-3d-probe',
        browserStudioMode: 'read-only 3D camera evidence inspection',
        disallowedActions: ['trusted writes', 'command bridge', 'scene mutation', 'viewport persistence', 'camera editor tooling'],
      },
    };
  }

  function boundedScene3dKeyframes(clip) {
    const keyframes = Array.isArray(clip && clip.keyframes) ? clip.keyframes : [];
    return keyframes
      .filter((keyframe) => keyframe && Number.isFinite(keyframe.frame) && keyframe.value && typeof keyframe.value === 'object')
      .map((keyframe) => ({ frame: Math.trunc(keyframe.frame), value: vector3(keyframe.value) }))
      .sort((left, right) => left.frame - right.frame);
  }

  function interpolateScene3dVector(keyframes, frame) {
    if (keyframes.length === 0) return null;
    if (frame <= keyframes[0].frame) return clone(keyframes[0].value);
    const last = keyframes[keyframes.length - 1];
    if (frame >= last.frame) return clone(last.value);
    for (let index = 1; index < keyframes.length; index += 1) {
      const right = keyframes[index];
      const left = keyframes[index - 1];
      if (frame > right.frame) continue;
      const span = Math.max(1, right.frame - left.frame);
      const ratio = (frame - left.frame) / span;
      return {
        x: Math.round(left.value.x + ((right.value.x - left.value.x) * ratio)),
        y: Math.round(left.value.y + ((right.value.y - left.value.y) * ratio)),
        z: Math.round(left.value.z + ((right.value.z - left.value.z) * ratio)),
      };
    }
    return clone(last.value);
  }

  function applyScene3dAnimationValue(node, channel, value) {
    if (!node || !channel || !value) return;
    node.localTransform = transform3(node.localTransform);
    node.localTransform[channel] = clone(value);
    if (node.worldTransform && typeof node.worldTransform === 'object') {
      node.worldTransform = transform3(node.worldTransform);
      node.worldTransform[channel] = clone(value);
    }
  }

  function scene3dAnimationSummary({ advanceFrames = 0, frameId = `tick-${world.tick}` } = {}) {
    const scene3d = world.scene3d && typeof world.scene3d === 'object' && !Array.isArray(world.scene3d)
      ? world.scene3d
      : null;
    if (!scene3d) {
      return {
        schemaVersion: 'ouroforge.scene3d-animation-evidence.v1',
        present: false,
        frameId: String(frameId),
        sceneId: world.sceneId,
        clipCount: 0,
        stateCount: 0,
        activeStateCount: 0,
        warningCount: 0,
        states: [],
        warnings: ['scene3d graph unavailable'],
        boundary: 'Read-only bounded 3D transform animation evidence; no skeletal authoring, retargeting, IK, graph editor, or production animation tooling claim.',
      };
    }
    const states = Array.isArray(scene3d.animationStates) ? scene3d.animationStates : [];
    const clips = Array.isArray(scene3d.animationClips) ? scene3d.animationClips : [];
    const nodesById = scene3dNodeById(scene3d);
    const clipsById = scene3dClipById(scene3d);
    const warnings = [];
    const rows = [];
    for (const state of states) {
      if (!state || typeof state !== 'object') continue;
      const clipId = String(state.clipId || '');
      const clip = clipsById.get(clipId);
      const targetNodeId = String(state.targetNodeId || (clip && clip.targetNodeId) || '');
      const channel = scene3dAnimationChannel(state.channel || (clip && clip.channel));
      const node = nodesById.get(targetNodeId);
      const row = {
        clipId,
        targetNodeId,
        channel: channel || String(state.channel || ''),
        currentFrame: Number.isFinite(state.currentFrame) ? Math.trunc(state.currentFrame) : 0,
        currentTimeMs: Number.isFinite(state.currentTimeMs) ? Math.trunc(state.currentTimeMs) : 0,
        playing: state.playing !== false,
        looped: Boolean(state.looped || (clip && clip.looped)),
        status: 'ready',
        value: null,
        warnings: [],
      };
      if (!clip) {
        row.status = 'missing_clip';
        row.playing = false;
        row.warnings.push(state.missingClipWarning || `missing animation clip ${clipId}`);
      } else if (!node) {
        row.status = 'missing_target';
        row.playing = false;
        row.warnings.push(`missing target node ${targetNodeId}`);
      } else if (!channel) {
        row.status = 'unsupported_channel';
        row.playing = false;
        row.warnings.push(`unsupported animation channel ${state.channel || clip.channel || 'unknown'}`);
      } else {
        const durationFrames = Number.isFinite(clip.durationFrames) && clip.durationFrames > 0
          ? Math.trunc(clip.durationFrames)
          : 0;
        const keyframes = boundedScene3dKeyframes(clip);
        if (durationFrames <= 0 || keyframes.length === 0) {
          row.status = 'malformed_clip';
          row.playing = false;
          row.warnings.push(`malformed animation clip ${clipId}`);
        } else {
          let nextFrame = row.currentFrame;
          if (row.playing && advanceFrames > 0) {
            nextFrame += advanceFrames;
            if (row.looped) {
              nextFrame %= (durationFrames + 1);
            } else if (nextFrame >= durationFrames) {
              nextFrame = durationFrames;
              row.playing = false;
            }
          }
          row.currentFrame = Math.max(0, Math.min(durationFrames, nextFrame));
          row.currentTimeMs += Math.max(0, advanceFrames) * fixedDeltaMs;
          row.value = interpolateScene3dVector(keyframes, row.currentFrame);
          applyScene3dAnimationValue(node, channel, row.value);
          state.currentFrame = row.currentFrame;
          state.currentTimeMs = row.currentTimeMs;
          state.playing = row.playing;
          state.looped = row.looped;
          state.targetNodeId = targetNodeId;
          state.channel = channel;
        }
      }
      for (const warning of (Array.isArray(state.malformedClipWarnings) ? state.malformedClipWarnings : [])) {
        row.warnings.push(String(warning));
      }
      for (const warning of row.warnings) warnings.push({ clipId, targetNodeId, warning });
      rows.push(row);
    }
    const summary = {
      schemaVersion: 'ouroforge.scene3d-animation-evidence.v1',
      present: true,
      frameId: String(frameId),
      sceneId: world.sceneId,
      clipCount: clips.length,
      stateCount: rows.length,
      activeStateCount: rows.filter((row) => row.playing && row.status === 'ready').length,
      warningCount: warnings.length,
      states: rows,
      warnings,
      boundary: 'Read-only bounded 3D transform animation evidence; no skeletal authoring, retargeting, IK, graph editor, or production animation tooling claim.',
    };
    world.scene3dAnimation = clone(summary);
    world.scene3dAnimationEvents = rows.map((row) => ({
      tick: world.tick,
      frameId: String(frameId),
      type: 'runtime.scene3d.animation.state',
      sceneId: world.sceneId,
      clipId: row.clipId,
      targetNodeId: row.targetNodeId,
      channel: row.channel,
      currentFrame: row.currentFrame,
      currentTimeMs: row.currentTimeMs,
      playing: row.playing,
      looped: row.looped,
      status: row.status,
      value: row.value,
      warningCount: row.warnings.length,
    }));
    return summary;
  }

  function cameraEvidence() {
    const active = activeCamera();
    const worldToScreen = {};
    for (const entity of world.entities) {
      const layer = entity.sprite && entity.sprite.layer ? entity.sprite.layer : 'default';
      worldToScreen[entity.id || `entity-${Object.keys(worldToScreen).length}`] = renderer.worldToScreen(
        entity.components && entity.components.transform ? entity.components.transform : { x: 0, y: 0 },
        rendererState,
        layer,
      );
    }
    return {
      activeCameraId: active ? active.id : null,
      cameras: clone(world.cameras),
      rendererCamera: clone(rendererState.camera),
      viewport: clone(rendererState.viewport),
      worldToScreen,
      scene3dCamera: scene3dCameraProbeSummary(),
      camera3d: scene3dCameraProbeSummary(),
    };
  }

  function setGoalFlag(flag, value) {
    if (!flag) return;
    world.goalFlags[flag] = Boolean(value);
  }

  function triggerReady(trigger) {
    return !Array.isArray(trigger.requiredFlags)
      || trigger.requiredFlags.every((flag) => world.goalFlags[flag] === true);
  }

  function applyTriggerAction(triggerEntity, action) {
    if (action.kind === 'setFlag' && action.flag) {
      setGoalFlag(action.flag, action.value !== false);
      record('runtime.trigger.action', { triggerId: triggerEntity.id, action: action.kind, flag: action.flag, value: world.goalFlags[action.flag] });
    } else if (action.kind === 'clearFlag' && action.flag) {
      setGoalFlag(action.flag, false);
      record('runtime.trigger.action', { triggerId: triggerEntity.id, action: action.kind, flag: action.flag, value: false });
    } else if (action.kind === 'hideEntity' && action.entityId) {
      const target = entityById(action.entityId);
      if (target) target.sprite.visible = false;
      record('runtime.trigger.action', { triggerId: triggerEntity.id, action: action.kind, entityId: action.entityId, hidden: Boolean(target) });
    }
  }

  function processTriggerEvents(eventsToProcess) {
    for (const event of eventsToProcess) {
      if (event.type !== 'runtime.collision.trigger') continue;
      const candidates = [entityById(event.otherEntityId), entityById(event.movingEntityId)].filter(Boolean);
      for (const triggerEntity of candidates) {
        const trigger = triggerEntity.components && triggerEntity.components.trigger;
        if (!trigger || !triggerReady(trigger)) continue;
        if (trigger.targetFlag) setGoalFlag(trigger.targetFlag, true);
        for (const action of trigger.onEnter || []) applyTriggerAction(triggerEntity, action);
        record('runtime.trigger.entered', {
          triggerId: trigger.id,
          entityId: triggerEntity.id,
          movingEntityId: event.movingEntityId,
          otherEntityId: event.otherEntityId,
          pairId: event.pairId,
          targetFlag: trigger.targetFlag,
          targetValue: trigger.targetFlag ? world.goalFlags[trigger.targetFlag] === true : null,
          flags: clone(world.goalFlags),
        });
      }
    }
  }


  function emitVfxEvents(trigger = 'tick') {
    for (const entity of world.entities) {
      const vfx = entity.components && entity.components.vfx;
      if (!vfx || !Array.isArray(vfx.emitters)) continue;
      for (const emitter of vfx.emitters) {
        if (emitter.disabled || emitter.trigger !== trigger) continue;
        const transform = entity.components && entity.components.transform ? entity.components.transform : { x: 0, y: 0 };
        const event = {
          schemaVersion: 'runtime-vfx-event-v1',
          sceneId: world.sceneId,
          entityId: entity.id,
          emitterId: emitter.id,
          kind: emitter.kind,
          trigger: emitter.trigger,
          tick: world.tick,
          particleCount: emitter.particleCount,
          lifetimeFrames: emitter.lifetimeFrames,
          expiresAtTick: world.tick + emitter.lifetimeFrames,
          color: emitter.color,
          asset: emitter.asset,
          layer: emitter.layer || (entity.sprite && entity.sprite.layer) || 'default',
          origin: { x: transform.x, y: transform.y },
          readOnlyEvidence: true,
        };
        world.vfxEvents.push(event);
        if (world.vfxEvents.length > 64) world.vfxEvents.shift();
        record('runtime.vfx.emitted', event);
      }
    }
  }

  function animationState(entity) {
    const animationComponent = entity && entity.components && entity.components.animation;
    const state = animationComponent && animationComponent.state;
    if (!animationComponent || !state) return null;
    const result = {
      mode: animationComponent.mode,
      currentClip: state.currentClip || animationComponent.currentClip || null,
      frameIndex: Number.isInteger(state.frameIndex) ? state.frameIndex : 0,
      elapsedFrames: Number.isInteger(state.elapsedFrames) ? state.elapsedFrames : 0,
    };
    if (state.activeState) result.activeState = state.activeState;
    return result;
  }

  function animationStatesByEntity(entities) {
    const states = new Map();
    for (const entity of entities) {
      const state = animationState(entity);
      if (state) states.set(entity.id, state);
    }
    return states;
  }

  function recordAnimationTransitions(beforeStates) {
    for (const entity of world.entities) {
      const after = animationState(entity);
      if (!after) continue;
      const before = beforeStates.get(entity.id) || null;
      const changed = !before
        || before.currentClip !== after.currentClip
        || before.frameIndex !== after.frameIndex
        || before.elapsedFrames !== after.elapsedFrames;
      if (!changed) continue;
      record('runtime.animation.state', {
        sceneId: world.sceneId,
        entityId: entity.id,
        mode: after.mode,
        from: before,
        to: after,
        currentClip: after.currentClip,
        ...(after.activeState ? { activeState: after.activeState } : {}),
        frameIndex: after.frameIndex,
        elapsedFrames: after.elapsedFrames,
      });
    }
  }

  function mergeCollisionEvents(primary = [], extra = []) {
    const merged = [];
    const seen = new Set();
    for (const event of primary.concat(extra)) {
      if (!event || typeof event !== 'object') continue;
      const key = `${event.type || 'collision'}:${event.pairId || event.nodeId || JSON.stringify(event)}`;
      if (seen.has(key)) continue;
      seen.add(key);
      merged.push(event);
    }
    return merged;
  }

  function isScene3dCollisionEvent(event) {
    return Boolean(event && typeof event.type === 'string' && event.type.startsWith('runtime.scene3d.collision.'));
  }

  function collisionEventsFor2dPhysics(eventsToProcess = []) {
    return eventsToProcess.filter((event) => !isScene3dCollisionEvent(event));
  }

  function stepOne() {
    applyInput();
    applyGravity();
    const beforeAnimationStates = animationStatesByEntity(world.entities);
    animation.advanceAnimations(world.entities, 1);
    world.tick += 1;
    recordAnimationTransitions(beforeAnimationStates);
    advanceJuiceEvents();
    emitJuiceEvents('tick');
    const scene3dAnimation = scene3dAnimationSummary({ advanceFrames: 1, frameId: `tick-${world.tick}` });
    for (const event of world.scene3dAnimationEvents) record(event.type, event);
    const physicsEntities = world.entities.concat(typeof tilemap.collisionEntities === 'function' ? tilemap.collisionEntities(world.tilemaps) : []);
    if (typeof collision.stepAabbPhysics === 'function') {
      world.collisions = collision.stepAabbPhysics(physicsEntities, world.bounds, world.tick, world.collisionRules).events;
    } else {
      for (const entity of world.entities) {
        const transform = entity.components.transform;
        const velocity = entity.components.velocity;
        const size = entity.components.size;
        transform.x = Math.max(0, Math.min(world.bounds.width - size.width, transform.x + velocity.x));
        transform.y = Math.max(0, Math.min(world.bounds.height - size.height, transform.y + velocity.y));
      }
      world.collisions = collision.detectAabbCollisions(physicsEntities, world.tick, world.collisionRules);
    }
    const scene3dCollision = typeof collision.scene3dCollisionSummary === 'function'
      ? collision.scene3dCollisionSummary({ world, frameId: `tick-${world.tick}` })
      : { present: false, events: [] };
    world.scene3dCollision = clone(scene3dCollision);
    world.scene3dCollisions = clone(Array.isArray(scene3dCollision.events) ? scene3dCollision.events : []);
    const currentCollisionEvents = mergeCollisionEvents(world.collisions, world.scene3dCollisions);
    for (const event of currentCollisionEvents) {
      world.collisionEvents.push(event);
      if (world.collisionEvents.length > 64) world.collisionEvents.shift();
      record(event.type, event);
      emitJuiceEvents('collision', event);
    }
    const physics2dCollisions = collisionEventsFor2dPhysics(currentCollisionEvents);
    refreshGroundedState(physics2dCollisions);
    processTriggerEvents(physics2dCollisions);
    emitVfxEvents('tick');
    updateCameraState();
    if (world.gridPuzzle && gridPuzzleModule) {
      const previousStatus = world.gridPuzzle.status;
      world.gridPuzzle = gridPuzzleModule.advance(world.gridPuzzle, input);
      if (world.gridPuzzle.status !== previousStatus) {
        record('runtime.grid_puzzle.status_changed', {
          status: world.gridPuzzle.status,
          tick: world.gridPuzzle.tick,
          moveCount: world.gridPuzzle.moveCount,
        });
      }
    }
  }

  function renderCanvas() {
    const canvas = document.getElementById('game');
    if (!canvas) return;
    const context = canvas.getContext('2d');
    renderer.drawRuntime({ canvas, context, world, renderer: rendererState, assets, animation, tilemap });
  }

  function setShellText(id, value) {
    const node = document.getElementById(id);
    if (node) node.textContent = value;
  }

  function setHudValue(id, value, tone = '') {
    const node = document.getElementById(id);
    if (!node) return;
    node.textContent = value;
    const base = 'hud-value';
    node.className = tone ? `${base} ${tone}` : base;
  }

  function latestObjectiveEvent(state) {
    const runtimeEvents = Array.isArray(state.runtimeEvents) ? state.runtimeEvents : [];
    const latest = runtimeEvents.slice().reverse().find((event) => {
      const type = event && event.type ? String(event.type) : '';
      return type.includes('trigger') || type.includes('scene.loaded') || type.includes('paused') || type.includes('resumed');
    });
    if (!latest) return 'Scene loaded';
    const type = String(latest.type || 'event').replace(/^runtime\./, '');
    if (type === 'scene.loaded') return 'Scene loaded';
    if (type === 'trigger.entered') return 'Objective updated';
    if (type === 'paused') return 'Paused';
    if (type === 'resumed') return 'Resumed';
    return type.replace(/[._-]+/g, ' ');
  }

  function shellFlag(state, names, fallback = false) {
    const flags = state && state.componentModel && state.componentModel.goalFlags
      ? state.componentModel.goalFlags
      : {};
    for (const name of names) {
      if (Object.prototype.hasOwnProperty.call(flags, name)) return flags[name] === true;
    }
    return fallback;
  }

  function playerAlive(state) {
    const flags = state && state.componentModel && state.componentModel.goalFlags
      ? state.componentModel.goalFlags
      : {};
    if (Object.prototype.hasOwnProperty.call(flags, 'player_alive')) return flags.player_alive === true;
    if (Object.prototype.hasOwnProperty.call(flags, 'alive')) return flags.alive === true;
    return true;
  }

  function renderShell(state) {
    if (!state || typeof document === 'undefined' || !document.getElementById) return;
    const sceneLabel = activeSceneSource || state.sceneId || 'scene.json';
    const hasKey = shellFlag(state, ['key_collected', 'coin_collected', 'has_key']);
    const gateOpen = shellFlag(state, ['gate_open', 'door_open', 'exit_open']);
    const exitReached = shellFlag(state, ['exit_reached', 'level_complete', 'won']);
    const alive = playerAlive(state);
    const paused = state.paused === true;
    const runState = paused ? 'Paused' : exitReached ? 'Win' : !alive ? 'Fail' : gateOpen ? 'Gate open' : hasKey ? 'Key collected' : 'Start';
    const status = paused
      ? 'Paused: resume when ready or restart the scene.'
      : exitReached
        ? 'Win: objective complete. Restart to play again.'
        : !alive
          ? 'Fail: the player is blocked or down. Restart to retry.'
          : gateOpen
            ? 'Gate open: reach the exit to win.'
            : hasKey
              ? 'Key collected: the gate is ready to open.'
              : 'Start: collect the key, open the gate, and reach the exit.';
    setShellText('scene-name', sceneLabel);
    setShellText('run-state', runState);
    setShellText('status-message', status);
    setHudValue('hud-key', hasKey ? 'Collected' : 'Missing', hasKey ? 'good' : 'warn');
    setHudValue('hud-gate', gateOpen ? 'Open' : 'Closed', gateOpen ? 'good' : 'warn');
    setHudValue('hud-exit', exitReached ? 'Reached' : gateOpen ? 'Ready' : 'Locked', exitReached ? 'good' : gateOpen ? 'good' : 'warn');
    setHudValue('hud-player', alive ? 'Alive' : 'Down', alive ? 'good' : 'bad');
    setHudValue('hud-tick', String(state.tick || 0));
    setHudValue('hud-event', latestObjectiveEvent(state));
  }

  function renderDebug() {
    renderCanvas();
    const state = api.getWorldState();
    renderShell(state);
    const debug = document.getElementById('debug');
    if (debug) debug.textContent = JSON.stringify(state, null, 2);
  }

  function loadScene(scene, options = {}) {
    let normalized;
    try {
      normalized = normalizeScene(scene);
    } catch (error) {
      recordRuntimeDiagnostic('scene_load_failed', 'Scene failed to load.', {
        error: String(error && error.message ? error.message : error),
        sceneId: scene && scene.id,
      });
      throw error;
    }
    runtimeDiagnostics.splice(0, runtimeDiagnostics.length);
    snapshots = snapshotFactory.createSnapshotRegistry();
    for (const key of Object.keys(input)) input[key] = false;
    for (const key of Object.keys(rawKeys)) delete rawKeys[key];
    for (const actionId of Object.keys(actionInput)) delete actionInput[actionId];
    world.schemaVersion = normalized.schemaVersion;
    world.sceneId = normalized.id;
    world.sceneKind = normalized.sceneKind;
    world.bounds = clone(normalized.bounds);
    world.entities = clone(normalized.entities);
    world.scene3d = normalized.scene3d ? clone(normalized.scene3d) : null;
    world.componentDefaults = clone(normalized.componentDefaults);
    world.tilemaps = clone(normalized.tilemaps);
    world.cameras = clone(normalized.cameras);
    world.activeCameraId = normalized.activeCameraId || (world.cameras[0] && world.cameras[0].id) || null;
    world.collisionRules = clone(normalized.collisionRules);
    world.gameplayRules = normalized.gameplayRules ? clone(normalized.gameplayRules) : { version: '1', flags: [] };
    world.sceneTransitions = clone(normalized.sceneTransitions);
    world.assetManifest = normalized.assetManifest ? clone(normalized.assetManifest) : null;
    world.goalFlags = {};
    for (const flag of world.gameplayRules.flags) {
      if (flag.initial === true) world.goalFlags[flag.id] = true;
    }
    if (typeof tilemap.extractAuthoringCells === 'function') {
      for (const cell of tilemap.extractAuthoringCells(world.tilemaps).triggerCells) {
        if (cell.trigger && world.goalFlags[cell.trigger] !== true) world.goalFlags[cell.trigger] = false;
      }
    }
    world.physics = { gravity: 1, maxFallSpeed: 8, grounded: {}, contacts: {}, contactPairs: [], blockedMovement: {} };
    for (const entity of world.entities) {
      if (isDynamicPhysicsEntity(entity)) {
        world.physics.grounded[entity.id] = false;
        world.physics.contacts[entity.id] = [];
        world.physics.blockedMovement[entity.id] = { x: false, y: false };
      }
    }
    for (const entity of world.entities) {
      const goalFlag = entity.components && entity.components.goalFlag;
      if (goalFlag && goalFlag.flag) world.goalFlags[goalFlag.flag] = Boolean(goalFlag.value);
      const status = entity.components && entity.components.status;
      if (status && Array.isArray(status.flags)) {
        for (const flag of status.flags) world.goalFlags[flag] = true;
      }
    }
    rendererState = clone(normalized.renderer);
    updateCameraState();
    world.metadata = clone(normalized.metadata);
    world.collisions = [];
    world.collisionEvents = [];
    world.scene3dCollision = null;
    world.scene3dCollisions = [];
    world.scene3dAnimation = null;
    world.scene3dAnimationEvents = [];
    world.audioEvents = [];
    world.audioWarnings = [];
    world.vfxEvents = [];
    world.juice = juice.createJuiceState(normalized.juice, normalized.id);
    world.juiceEvents = [];
    world.gridPuzzle = normalized.gridPuzzle ? clone(normalized.gridPuzzle) : null;
    world.uiux = normalized.uiux ? clone(normalized.uiux) : null;
    world.deckRoguelike = normalized.deckRoguelike ? clone(normalized.deckRoguelike) : null;
    world.deckbuilderUi = normalized.deckbuilderUi ? clone(normalized.deckbuilderUi) : null;
    world.tick = 0;
    seedRng(scene && scene.seed !== undefined ? scene.seed : 0);
    scene3dAnimationSummary({ advanceFrames: 0, frameId: 'tick-0' });
    if (options.sceneSource) activeSceneSource = options.sceneSource;
    const assetMetadata = assets.load(world, world.assetManifest, { resolvePath: sceneRelativeAssetPath });
    const assetDiagnostics = assetMetadata.concat(typeof assets.metadata === 'function' ? assets.metadata() : []);
    const reportedAssets = new Set();
    for (const asset of assetDiagnostics) {
      const key = asset && (asset.attemptId || asset.id || asset.path);
      if (!asset || reportedAssets.has(key)) continue;
      reportedAssets.add(key);
      if (asset.status === 'failed' || asset.status === 'rejected') {
        recordRuntimeDiagnostic('missing_asset', `Asset ${asset.id || asset.path || 'unknown'} did not load.`, {
          assetId: asset.id,
          assetType: asset.kind,
          path: asset.path,
          status: asset.status,
          failureReason: asset.failureReason,
          evidenceRefs: [asset.attemptId].filter(Boolean),
        });
      }
    }
    const manifestSummary = assets.manifestSummary ? assets.manifestSummary() : null;
    for (const error of (manifestSummary && Array.isArray(manifestSummary.errors) ? manifestSummary.errors : [])) {
      recordRuntimeDiagnostic('missing_asset', 'Asset manifest is invalid.', {
        assetManifestId: manifestSummary.id,
        failureReason: error,
      });
    }
    if (manifestSummary && manifestSummary.enabled) {
      const declaredAssetIds = new Set((manifestSummary.assets || []).map((asset) => asset.id));
      for (const entity of world.entities) {
        const refs = [];
        if (entity.sprite && typeof entity.sprite.asset === 'string') refs.push({ ref: entity.sprite.asset, entityId: entity.id, source: 'sprite.asset' });
        const vfx = entity.components && entity.components.vfx;
        for (const emitter of (vfx && Array.isArray(vfx.emitters)) ? vfx.emitters : []) {
          if (typeof emitter.asset === 'string') refs.push({ ref: emitter.asset, entityId: entity.id, source: 'vfx.emitter.asset' });
        }
        const animationComponent = entity.components && entity.components.animation;
        const animationFrames = []
          .concat(animationComponent && Array.isArray(animationComponent.frames) ? animationComponent.frames : [])
          .concat(...((animationComponent && Array.isArray(animationComponent.clips) ? animationComponent.clips : []).map((clip) => Array.isArray(clip.frames) ? clip.frames : [])));
        for (const frame of animationFrames) {
          if (frame && typeof frame.asset === 'string') refs.push({ ref: frame.asset, entityId: entity.id, source: 'animation.frame.asset' });
        }
        for (const { ref, entityId, source } of refs) {
          if (!declaredAssetIds.has(ref)) {
            recordRuntimeDiagnostic('missing_asset', `Asset reference ${ref} is not declared in the active manifest.`, {
              assetId: ref,
              entityId,
              source,
              assetManifestId: manifestSummary.id,
            });
          }
        }
      }
    }
    record('runtime.scene.loaded', {
      schemaVersion: world.schemaVersion,
      sceneId: world.sceneId,
      entityCount: world.entities.length,
      sceneTransitionCount: world.sceneTransitions.length,
      assetCount: assetMetadata.length,
      gameplayFlagCount: world.gameplayRules.flags.length,
      assetManifestId: manifestSummary ? manifestSummary.id : null,
    });
    emitAudioEvents('scene_loaded');
    emitJuiceEvents('scene_loaded');
    renderDebug();
    return api.getWorldState();
  }

  function safeTransitionPath(path) {
    // Apply the strict character allowlist to every target, including absolute
    // `/examples/` paths. The allowlist excludes `%`, so percent-encoded dot
    // segments (e.g. `/examples/%2e%2e/secret.scene.json`) are rejected before
    // they can be normalized back into a parent-directory escape by fetch/URL
    // handling. Legitimate relative and `/examples/` scene paths already match
    // the allowlist, so no bounded target loses access.
    return typeof path === 'string'
      && path.endsWith('.json')
      && !path.includes('..')
      && !path.includes('\\')
      && !/^[a-z][a-z0-9+.-]*:/i.test(path)
      && /^[A-Za-z0-9_./-]+$/.test(path);
  }

  function recordTransitionOutcome(outcome) {
    const entry = {
      tick: world.tick,
      ...clone(outcome),
    };
    world.transitionEvents.push(entry);
    if (world.transitionEvents.length > 16) world.transitionEvents.shift();
    record(`runtime.scene.transition.${entry.status}`, entry);
    return entry;
  }

  async function transition(transitionId) {
    if (typeof transitionId !== 'string' || !transitionId) {
      const reason = 'transition id is required';
      recordTransitionOutcome({ status: 'failed', reason, fromSceneId: world.sceneId });
      throw new Error(reason);
    }
    const declared = world.sceneTransitions.find((candidate) => candidate.id === transitionId);
    if (!declared) {
      const reason = `transition ${transitionId} is not declared by current scene`;
      recordTransitionOutcome({ status: 'failed', id: transitionId, reason, fromSceneId: world.sceneId });
      throw new Error(reason);
    }
    if (!safeTransitionPath(declared.toScene)) {
      const reason = `transition ${transitionId} target is not a bounded scene path`;
      recordTransitionOutcome({ status: 'failed', id: transitionId, toScene: declared.toScene, reason, fromSceneId: world.sceneId });
      throw new Error(reason);
    }
    const fromSceneId = world.sceneId;
    try {
      const response = await fetch(declared.toScene);
      const scene = await response.json();
      loadScene(scene, { sceneSource: declared.toScene });
      recordTransitionOutcome({
        status: 'succeeded',
        id: declared.id,
        label: declared.label,
        fromSceneId,
        toScene: declared.toScene,
        toSceneId: world.sceneId,
      });
    } catch (error) {
      recordTransitionOutcome({
        status: 'failed',
        id: declared.id,
        label: declared.label,
        fromSceneId,
        toScene: declared.toScene,
        reason: String(error && error.message ? error.message : error),
      });
      throw error;
    }
    renderDebug();
    return api.getWorldState();
  }



  function recordReloadOutcome(outcome) {
    const entry = {
      tick: world.tick,
      sceneId: world.sceneId,
      ...clone(outcome),
    };
    world.reloads.push(entry);
    if (world.reloads.length > 16) world.reloads.shift();
    record(`runtime.reload.${entry.status}`, entry);
    return entry;
  }

  function failReload(reason) {
    recordReloadOutcome({ status: 'failed', reason });
    throw new Error(reason);
  }

  function reload(payload = {}) {
    if (!payload || typeof payload !== 'object' || payload.schemaVersion !== 'ouroforge.scene-reload.v0') {
      failReload('reload payload schemaVersion must be ouroforge.scene-reload.v0');
    }
    if (!payload.scene || typeof payload.scene !== 'object' || Array.isArray(payload.scene)) {
      failReload('reload payload scene is required');
    }
    const scene = clone(payload.scene);
    if (payload.assetManifest && typeof payload.assetManifest === 'object' && !Array.isArray(payload.assetManifest)) {
      scene.assetManifest = clone(payload.assetManifest);
    }
    try {
      loadScene(scene);
    } catch (error) {
      recordReloadOutcome({ status: 'failed', reason: String(error.message || error) });
      throw error;
    }
    recordReloadOutcome({
      status: 'succeeded',
      schemaVersion: payload.schemaVersion,
      sceneId: world.sceneId,
      entityCount: world.entities.length,
      assetManifestId: assets.manifestSummary ? assets.manifestSummary().id : null,
    });
    renderDebug();
    return api.getWorldState();
  }

  function setInput(nextInput = {}) {
    for (const key of Object.keys(input)) {
      if (Object.prototype.hasOwnProperty.call(nextInput, key)) {
        input[key] = Boolean(nextInput[key]);
      }
    }
    if (nextInput.keys && typeof nextInput.keys === 'object' && !Array.isArray(nextInput.keys)) {
      for (const [key, value] of Object.entries(nextInput.keys)) {
        rawKeys[keyAlias(key)] = Boolean(value);
      }
    }
    if (nextInput.actions && typeof nextInput.actions === 'object' && !Array.isArray(nextInput.actions)) {
      for (const [actionId, value] of Object.entries(nextInput.actions)) {
        actionInput[String(actionId)] = Boolean(value);
      }
    }
    record('runtime.input.changed', {
      input: clone(input),
      rawInput: { directions: clone(input), keys: clone(rawKeys) },
      actionState: resolvedActionState(),
      actionDiagnostics: inputActionDiagnostics(),
    });
    renderDebug();
    return api.getWorldState();
  }

  function step(count = 1) {
    const steps = Number.isFinite(count) ? Math.max(0, Math.floor(count)) : 0;
    for (let index = 0; index < steps; index += 1) stepOne();
    renderDebug();
    return api.getWorldState();
  }

  function snapshot() {
    const snapshotId = snapshots.capture({ world, input, rawKeys, actionInput, events, rendererState }, world.tick);
    record('runtime.snapshot.captured', { snapshotId, tick: world.tick });
    renderDebug();
    return { snapshotId, metadata: snapshots.metadata(snapshotId) };
  }

  function restore(snapshotId) {
    const snapshotValue = snapshots.restore(snapshotId);
    Object.assign(world, clone(snapshotValue.world || world));
    for (const key of Object.keys(input)) input[key] = false;
    Object.assign(input, clone(snapshotValue.input || input));
    for (const key of Object.keys(rawKeys)) delete rawKeys[key];
    Object.assign(rawKeys, clone(snapshotValue.rawKeys || {}));
    for (const actionId of Object.keys(actionInput)) delete actionInput[actionId];
    Object.assign(actionInput, clone(snapshotValue.actionInput || {}));
    events.splice(0, events.length, ...clone(snapshotValue.events || []));
    if (snapshotValue.rendererState) rendererState = clone(snapshotValue.rendererState);
    record('runtime.snapshot.restored', { snapshotId, tick: world.tick });
    renderDebug();
    return api.getWorldState();
  }

  function runtimeState(slotId = 'slot-1') {
    const stateLabel = safeEvidenceStem(slotId, 'runtime state id');
    const scopedEntities = world.entities.map((entity) => ({
      entityId: entity.id,
      transform: clone(entity.components && entity.components.transform ? entity.components.transform : {}),
      velocity: clone(entity.components && entity.components.velocity ? entity.components.velocity : {}),
      status: clone(entity.components && entity.components.status ? entity.components.status : {}),
    }));
    const state = {
      schemaVersion: 'runtime-state-v1',
      stateId: `${stateLabel}-tick-${world.tick}`,
      runId: world.runId || 'browser-runtime-local',
      sceneId: world.sceneId,
      tick: world.tick,
      recordedAtUnixMs: Date.now(),
      flags: clone(world.goalFlags || {}),
      inventory: [],
      progress: {},
      entities: scopedEntities,
      camera: cameraEvidence(),
      input: {
        rawInput: { directions: clone(input), keys: clone(rawKeys) },
        actionInput: clone(actionInput),
        actionState: resolvedActionState(),
      },
      rng: clone(world.rng || { schemaVersion: 'runtime-seeded-rng-v1', algorithm: 'mulberry32', seed: 0, state: 0, drawCount: 0 }),
    };
    const gridPuzzleDigest = world.gridPuzzle && gridPuzzleModule
      ? gridPuzzleModule.digestState(world.gridPuzzle)
      : null;
    if (gridPuzzleDigest) state.gridPuzzle = gridPuzzleDigest;
    const deckRoguelikeDigest = world.deckRoguelike && deckRoguelikeModule
      ? deckRoguelikeModule.digestState(world.deckRoguelike)
      : null;
    if (deckRoguelikeDigest) {
      state.deckRoguelike = deckRoguelikeDigest;
      state.cardRogueliteSubstrate = deckRoguelikeDigest;
    }
    state.digest = {
      algorithm: 'fnv1a64-canonical-json-v1',
      value: fnv1a64({
        sceneId: state.sceneId,
        tick: state.tick,
        flags: state.flags,
        entities: state.entities,
        camera: state.camera,
        input: state.input,
        rng: state.rng,
        // Only grid-puzzle scenes contribute this key, so non-grid scene
        // digests remain byte-identical to their pre-grid-puzzle values.
        ...(gridPuzzleDigest ? { gridPuzzle: gridPuzzleDigest } : {}),
        // Likewise, only deck-roguelike scenes contribute this key, so other
        // scene digests remain byte-identical to their prior values.
        ...(deckRoguelikeDigest ? { deckRoguelike: deckRoguelikeDigest } : {}),
      }),
    };
    return state;
  }

  function safeEvidenceStem(value = 'slot-1', label = 'runtime evidence id') {
    const stem = String(value || 'slot-1');
    if (!/^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/.test(stem) || stem.includes('..')) {
      throw new Error(`${label} must be a path-safe generated evidence file stem`);
    }
    return stem;
  }

  function createSave(slotId = 'slot-1') {
    const safeSlotId = safeEvidenceStem(slotId, 'runtime save slotId');
    const state = runtimeState(safeSlotId);
    const save = {
      schemaVersion: 'runtime-save-artifact-v1',
      saveId: `${safeSlotId}-tick-${world.tick}`,
      runId: state.runId,
      slotId: safeSlotId,
      createdAtUnixMs: Date.now(),
      state,
      policy: {
        artifactPath: `evidence/runtime-state/saves/${safeSlotId}.save.json`,
        rootKind: 'generated_evidence',
        trustedWriter: 'rust-local-runtime-save-v1',
        browserWriteAccess: 'none',
        retention: 'generated run evidence; untracked unless fixture-scoped',
      },
    };
    saveSlots.set(safeSlotId, clone(save));
    record('runtime.save.created', { saveId: save.saveId, slotId: safeSlotId, stateDigest: state.digest });
    return clone(save);
  }

  function applyRuntimeState(state) {
    if (!state || typeof state !== 'object' || state.schemaVersion !== 'runtime-state-v1') {
      throw new Error('runtime state schemaVersion must be runtime-state-v1');
    }
    if (state.sceneId && state.sceneId !== world.sceneId) {
      throw new Error(`runtime save sceneId ${state.sceneId} does not match current scene ${world.sceneId}`);
    }
    world.tick = Number.isFinite(state.tick) ? Math.max(0, Math.floor(state.tick)) : world.tick;
    world.goalFlags = clone(state.flags || {});
    const entitiesById = new Map(world.entities.map((entity) => [entity.id, entity]));
    for (const savedEntity of Array.isArray(state.entities) ? state.entities : []) {
      const entity = entitiesById.get(savedEntity.entityId);
      if (!entity || !entity.components) continue;
      if (savedEntity.transform) entity.components.transform = clone(savedEntity.transform);
      if (savedEntity.velocity) entity.components.velocity = clone(savedEntity.velocity);
      if (savedEntity.status && entity.components.status) entity.components.status = clone(savedEntity.status);
    }
    const savedInput = state.input || {};
    const savedRaw = savedInput.rawInput || {};
    for (const key of Object.keys(input)) input[key] = Boolean(savedRaw.directions && savedRaw.directions[key]);
    for (const key of Object.keys(rawKeys)) delete rawKeys[key];
    Object.assign(rawKeys, clone(savedRaw.keys || {}));
    for (const actionId of Object.keys(actionInput)) delete actionInput[actionId];
    Object.assign(actionInput, clone(savedInput.actionInput || {}));
    if (state.rng && state.rng.algorithm === 'mulberry32') {
      world.rng = {
        schemaVersion: 'runtime-seeded-rng-v1',
        algorithm: 'mulberry32',
        seed: normalizeSeed(state.rng.seed),
        state: normalizeSeed(state.rng.state),
        drawCount: Number.isFinite(state.rng.drawCount) ? Math.max(0, Math.floor(state.rng.drawCount)) : 0,
      };
    }
  }

  function loadSave(saveOrSlotId) {
    const save = typeof saveOrSlotId === 'string' ? saveSlots.get(safeEvidenceStem(saveOrSlotId, 'runtime save slotId')) : saveOrSlotId;
    if (!save || typeof save !== 'object' || save.schemaVersion !== 'runtime-save-artifact-v1') {
      throw new Error('runtime save artifact schemaVersion must be runtime-save-artifact-v1');
    }
    applyRuntimeState(save.state);
    record('runtime.save.loaded', { saveId: save.saveId, slotId: save.slotId, stateDigest: save.state && save.state.digest });
    renderDebug();
    return api.getWorldState();
  }


  function replayStateDigest(frameId = `tick-${world.tick}`) {
    const safeFrameId = safeEvidenceStem(frameId, 'runtime replay frameId');
    const state = runtimeState(safeFrameId);
    return {
      schemaVersion: 'runtime-replay-digest-v1',
      frameId: safeFrameId,
      sceneId: state.sceneId,
      tick: state.tick,
      stateId: state.stateId,
      digest: clone(state.digest),
      policy: {
        artifactPath: `evidence/runtime-state/replay/${safeFrameId}.digest.json`,
        rootKind: 'generated_evidence',
        trustedWriter: 'rust-local-scenario-runner-v1',
        browserWriteAccess: 'none',
        retention: 'generated replay evidence; untracked unless fixture-scoped',
      },
    };
  }

  function compareReplayDigest(expectedDigest, frameId = `tick-${world.tick}`) {
    const expected = expectedDigest && expectedDigest.digest ? expectedDigest.digest : expectedDigest;
    if (!expected || typeof expected !== 'object' || expected.algorithm !== 'fnv1a64-canonical-json-v1' || !/^[0-9a-f]{16}$/.test(String(expected.value || ''))) {
      throw new Error('runtime replay expected digest must use fnv1a64-canonical-json-v1 with a 16 character hex value');
    }
    const actual = replayStateDigest(frameId);
    const diverged = actual.digest.value !== expected.value;
    const evidence = {
      schemaVersion: 'runtime-replay-divergence-v1',
      status: diverged ? 'diverged' : 'matched',
      frameId: actual.frameId,
      sceneId: actual.sceneId,
      tick: actual.tick,
      expected: clone(expected),
      actual: clone(actual.digest),
      firstDivergence: diverged ? { frameId: actual.frameId, tick: actual.tick, reason: 'state digest mismatch' } : null,
      policy: {
        artifactPath: `evidence/runtime-state/replay/${actual.frameId}.divergence.json`,
        rootKind: 'generated_evidence',
        trustedWriter: 'rust-local-scenario-runner-v1',
        browserWriteAccess: 'none',
        retention: 'generated replay evidence; untracked unless fixture-scoped',
      },
    };
    record('runtime.replay.digest_compared', { frameId: actual.frameId, status: evidence.status, expected, actual: actual.digest });
    return evidence;
  }

  function replayDiagnostic(code, message, details = {}) {
    const diagnostic = recordRuntimeDiagnostic(code, message, details);
    record('runtime.replay.diagnostic', diagnostic);
    return diagnostic;
  }

  function replayActionInputPatch(action) {
    const patch = {};
    const source = action && typeof action === 'object' ? action : {};
    const directionSource = source.input && typeof source.input === 'object' ? source.input : source;
    for (const key of Object.keys(input)) {
      if (Object.prototype.hasOwnProperty.call(directionSource, key)) patch[key] = Boolean(directionSource[key]);
    }
    if (source.keys && typeof source.keys === 'object' && !Array.isArray(source.keys)) patch.keys = clone(source.keys);
    if (source.actions && typeof source.actions === 'object' && !Array.isArray(source.actions)) patch.actions = clone(source.actions);
    return patch;
  }

  function replayStepCount(action) {
    const source = action && typeof action === 'object' ? action : {};
    const value = Number.isFinite(source.steps) ? source.steps : (Number.isFinite(source.step) ? source.step : 0);
    return Math.max(0, Math.floor(value));
  }

  function runReplay(replay = {}) {
    if (!replay || typeof replay !== 'object' || Array.isArray(replay)) {
      const diagnostic = replayDiagnostic('replay_invalid_request', 'Replay request must be an object.', { replayType: typeof replay });
      throw new Error(diagnostic.message);
    }
    const sequence = Array.isArray(replay.sequence) ? replay.sequence : (Array.isArray(replay.actions) ? replay.actions : []);
    const label = safeEvidenceStem(replay.label || replay.finalFrameId || `replay-${world.tick}`, 'runtime replay label');
    if (replay.scene && typeof replay.scene === 'object' && !Array.isArray(replay.scene)) {
      const scene = clone(replay.scene);
      if (replay.seed !== undefined) scene.seed = replay.seed;
      loadScene(scene);
    } else if (replay.seed !== undefined) {
      seedRng(replay.seed);
    }
    const checkpoints = [];
    try {
      sequence.forEach((action, index) => {
        if (!action || typeof action !== 'object' || Array.isArray(action)) {
          throw new Error(`replay action ${index} must be an object`);
        }
        const patch = replayActionInputPatch(action);
        if (Object.keys(patch).length > 0) setInput(patch);
        const steps = replayStepCount(action);
        if (steps > 0) step(steps);
        if (action.checkpoint || action.label) {
          const checkpointLabel = safeEvidenceStem(action.checkpoint || action.label, 'runtime replay checkpoint label');
          checkpoints.push(replayStateDigest(`${label}-${checkpointLabel}`));
        }
      });
    } catch (error) {
      const diagnostic = replayDiagnostic('replay_sequence_failed', 'Replay sequence failed before final digest.', {
        label,
        error: String(error && error.message ? error.message : error),
      });
      throw new Error(`${diagnostic.message} ${diagnostic.details.error}`);
    }
    const finalDigest = replayStateDigest(label);
    const evidence = {
      schemaVersion: 'ouroforge.runtime-replay-run-v1',
      label,
      sceneId: finalDigest.sceneId,
      seed: world.rng ? world.rng.seed : null,
      actionCount: sequence.length,
      tick: finalDigest.tick,
      flags: clone(world.goalFlags || {}),
      finalStateDigest: clone(finalDigest),
      checkpoints,
      diagnostics: [],
    };
    record('runtime.replay.completed', {
      label,
      seed: evidence.seed,
      actionCount: evidence.actionCount,
      finalDigest: evidence.finalStateDigest.digest,
    });
    return evidence;
  }

  function replayDeterminismCheck(request = {}) {
    const runs = Number.isFinite(request.runs) ? Math.max(3, Math.floor(request.runs)) : 3;
    const label = safeEvidenceStem(request.label || 'determinism', 'runtime replay determinism label');
    const results = [];
    const diagnostics = [];
    for (let index = 0; index < runs; index += 1) {
      try {
        results.push(runReplay({
          scene: request.scene,
          seed: request.seed,
          sequence: request.sequence || request.actions || [],
          label: `${label}-run-${index + 1}`,
        }));
      } catch (error) {
        diagnostics.push(replayDiagnostic('replay_run_failed', 'Replay run failed during determinism check.', {
          label,
          runIndex: index + 1,
          error: String(error && error.message ? error.message : error),
        }));
      }
    }
    const digests = results.map((result) => result.finalStateDigest.digest.value);
    const expected = digests[0] || null;
    const divergenceIndex = expected ? digests.findIndex((digest) => digest !== expected) : -1;
    if (divergenceIndex >= 0) {
      diagnostics.push(replayDiagnostic('replay_determinism_diverged', 'Replay determinism check produced different final state digests.', {
        label,
        expected,
        actual: digests[divergenceIndex],
        runIndex: divergenceIndex + 1,
        digests,
      }));
    }
    const evidence = {
      schemaVersion: 'ouroforge.runtime-replay-determinism-v1',
      label,
      sceneId: results[0] ? results[0].sceneId : world.sceneId,
      seed: results[0] ? results[0].seed : (world.rng ? world.rng.seed : null),
      runs,
      status: diagnostics.length ? 'failed' : 'matched',
      finalStateDigests: results.map((result) => result.finalStateDigest.digest),
      results,
      diagnostics,
    };
    record('runtime.replay.determinism_checked', {
      label,
      status: evidence.status,
      runs,
      digests: evidence.finalStateDigests,
      diagnosticCount: diagnostics.length,
    });
    return evidence;
  }

  function queryEntity(query) {
    const id = typeof query === 'string'
      ? query
      : (query && typeof query === 'object' && typeof query.id === 'string' ? query.id : null);
    if (!id) {
      const diagnostic = recordRuntimeDiagnostic('invalid_query', 'Entity query requires a string id.', { query });
      return { schemaVersion: 'ouroforge.runtime-query-result.v1', status: 'invalid', diagnostic };
    }
    const entity = world.entities.find((candidate) => candidate.id === id) || null;
    if (!entity) {
      const diagnostic = recordRuntimeDiagnostic('invalid_query', `Entity ${id} was not found.`, { query: { id } });
      return { schemaVersion: 'ouroforge.runtime-query-result.v1', status: 'missing', entity: null, diagnostic };
    }
    return { schemaVersion: 'ouroforge.runtime-query-result.v1', status: 'found', entity: clone(entity), diagnostic: null };
  }

  function sampleRuntimeState(sample = {}) {
    try {
      if (sample && sample.forceFailure === true) throw new Error('planted sampler failure');
      const sampleId = safeEvidenceStem(sample.sampleId || sample.id || `sample-${world.tick}`, 'runtime sampler id');
      if (sample.entityId !== undefined) {
        const result = queryEntity({ id: sample.entityId });
        if (result.status !== 'found') throw new Error(`sampler entity ${sample.entityId} not found`);
      }
      const state = runtimeState(sampleId);
      return {
        schemaVersion: 'ouroforge.runtime-state-sample.v1',
        status: 'sampled',
        sampleId,
        sceneId: state.sceneId,
        tick: state.tick,
        digest: clone(state.digest),
        diagnostic: null,
      };
    } catch (error) {
      const diagnostic = recordRuntimeDiagnostic('sampler_failed', 'Runtime state sampler failed.', {
        sample,
        error: String(error && error.message ? error.message : error),
      });
      return {
        schemaVersion: 'ouroforge.runtime-state-sample.v1',
        status: 'failed',
        diagnostic,
      };
    }
  }

  function scenarioCoverageV100(options = {}) {
    const baseScene = options.baseScene && typeof options.baseScene === 'object'
      ? clone(options.baseScene)
      : {
        schemaVersion: '1',
        id: 'scenario-coverage-v100-base',
        bounds: { width: 64, height: 64 },
        entities: [{
          id: 'player',
          sprite: { color: '#5eead4' },
          components: {
            transform: { x: 0, y: 0 },
            velocity: { x: 0, y: 0 },
            size: { width: 8, height: 8 },
            controllable: true,
          },
        }],
      };
    const observed = [];
    function collect(kind) {
      for (const diagnostic of runtimeDiagnostics) observed.push({ kind, diagnostic: clone(diagnostic) });
    }
    try {
      loadScene({ id: 'scenario-coverage-v100-scene-load-failure', gridPuzzle: {} });
    } catch (_error) {
      collect('scene-load');
    }
    loadScene(baseScene);
    queryEntity({ invalid: true });
    collect('invalid-query');
    replayDeterminismCheck({ scene: baseScene, seed: 100, sequence: [null], label: 'scenario-v100-replay' });
    collect('replay');
    const missingAssetScene = clone(baseScene);
    missingAssetScene.id = 'scenario-coverage-v100-missing-asset';
    missingAssetScene.assetManifest = { id: 'scenario-v100-assets', assets: [{ id: 'declared-other', kind: 'sprite', path: 'assets/sprites/declared-other.svg' }] };
    missingAssetScene.entities = [{
      id: 'asset-user',
      sprite: { asset: 'missing-sprite' },
      components: { transform: { x: 0, y: 0 }, velocity: { x: 0, y: 0 }, size: { width: 8, height: 8 } },
    }];
    loadScene(missingAssetScene);
    collect('missing-asset');
    sampleRuntimeState({ forceFailure: true, sampleId: 'scenario-v100-sampler' });
    collect('sampler');
    const requiredCodes = ['scene_load_failed', 'invalid_query', 'missing_asset', 'replay_run_failed', 'sampler_failed'];
    const observedCodes = observed.map((entry) => entry.diagnostic.code);
    return {
      schemaVersion: 'ouroforge.scenario-coverage-v100.v1',
      status: requiredCodes.every((code) => observedCodes.includes(code)) ? 'passed' : 'failed',
      requiredCodes,
      observedCodes,
      plantedFailureCount: requiredCodes.length,
      observed,
      classification: 'contract-complete',
    };
  }


  function compositionDebugState(entities) {
    return {
      version: '1',
      entities: entities.map((entity) => ({
        entityId: entity.id,
        parent: entity.parent,
        localTransform: clone(entity.components.localTransform || entity.components.transform),
        worldTransform: clone(entity.components.transform),
      })),
    };
  }

  function componentModelDebugState(entities) {
    const names = ['status', 'input', 'trigger', 'goalFlag', 'cameraTarget', 'uiText', 'hudValue', 'vfx'];
    const counts = Object.fromEntries(names.map((name) => [name, 0]));
    const hudValues = [];
    const entityComponents = entities.map((entity) => {
      const components = {};
      for (const name of names) {
        if (entity.components && entity.components[name]) {
          counts[name] += 1;
          components[name] = clone(entity.components[name]);
        }
      }
      if (entity.components && entity.components.hudValue) {
        const hudValue = entity.components.hudValue;
        hudValues.push({
          entityId: entity.id,
          kind: hudValue.kind,
          label: hudValue.label,
          value: hudValue.value,
          bindFlag: hudValue.bindFlag,
          flagValue: hudValue.bindFlag ? Boolean(world.goalFlags[hudValue.bindFlag]) : null,
          text: hudValue.label ? `${hudValue.label}: ${hudValue.value}` : hudValue.value,
        });
      }
      return { entityId: entity.id, components };
    });
    return {
      version: '2',
      counts,
      entities: entityComponents,
      goalFlags: clone(world.goalFlags),
      hudValues,
    };
  }

  function boundedMs(value, fallback = 0) {
    return Number.isFinite(value) && value >= 0 ? value : fallback;
  }

  function frameBudgetConfig() {
    const debug = world.metadata && typeof world.metadata.runtimeDebug === 'object' ? world.metadata.runtimeDebug : {};
    const budget = debug.frameBudget && typeof debug.frameBudget === 'object' ? debug.frameBudget : {};
    return {
      updateMs: boundedMs(budget.updateMs, 8),
      renderMs: boundedMs(budget.renderMs, 16),
      evidenceMs: boundedMs(budget.evidenceMs, 4),
      totalMs: boundedMs(budget.totalMs, 20),
    };
  }

  function frameTimings() {
    const debug = world.metadata && typeof world.metadata.runtimeDebug === 'object' ? world.metadata.runtimeDebug : {};
    const timings = debug.frameTimings && typeof debug.frameTimings === 'object' ? debug.frameTimings : {};
    return {
      updateMs: boundedMs(timings.updateMs, 0),
      renderMs: boundedMs(timings.renderMs, 0),
      evidenceMs: boundedMs(timings.evidenceMs, 0),
      totalMs: boundedMs(timings.totalMs, fixedDeltaMs),
    };
  }

  function frameDebugCounts(renderQueue) {
    const renderables = Array.isArray(renderQueue && renderQueue.renderables) ? renderQueue.renderables : [];
    return {
      entityCount: world.entities.length,
      drawCallCount: renderables.filter((renderable) => renderable.visible !== false).length,
      layerCount: Array.isArray(renderQueue && renderQueue.layers) ? renderQueue.layers.length : 0,
      collisionPairCount: Array.isArray(world.physics && world.physics.contactPairs) ? world.physics.contactPairs.length : 0,
      activeAnimationCount: world.entities.filter((entity) => entity.components && entity.components.animation && entity.components.animation.state).length,
      activeVfxCount: Array.isArray(world.vfxEvents) ? world.vfxEvents.length : 0,
      activeJuiceCount: world.juice && Array.isArray(world.juice.active) ? world.juice.active.length : 0,
      juiceEventCount: Array.isArray(world.juiceEvents) ? world.juiceEvents.length : 0,
      audioEventCount: Array.isArray(world.audioEvents) ? world.audioEvents.length : 0,
    };
  }

  function runtimeFrameBudgetEvidence(frameId, renderQueue) {
    const timings = frameTimings();
    const budget = frameBudgetConfig();
    const violations = Object.keys(budget)
      .filter((field) => timings[field] > budget[field])
      .map((field) => ({ field, actualMs: timings[field], budgetMs: budget[field] }));
    return {
      schemaVersion: 'ouroforge.runtime-frame-budget.v1',
      frameId,
      sceneId: world.sceneId,
      scenarioId: typeof world.metadata.scenarioId === 'string' ? world.metadata.scenarioId : null,
      timings,
      budget,
      counts: frameDebugCounts(renderQueue),
      status: violations.length ? 'violated' : 'within-budget',
      slowFrame: violations.length > 0,
      violations,
      readOnlyInspection: {
        trustedEmitter: 'browser-runtime-debug-probe',
        browserStudioMode: 'read-only evidence inspection',
        disallowedActions: ['trusted writes', 'command bridge', 'live mutation'],
      },
      authority: 'browser_runtime_evidence_input_not_profiler_truth',
    };
  }

  let sceneReady = Promise.resolve();
  let activeSceneSource = 'scene.json';

  function assetBaseForSceneSource(sceneSource) {
    if (typeof sceneSource !== 'string' || !sceneSource) return '';
    const withoutQuery = sceneSource.split('?')[0].split('#')[0];
    const sceneDir = withoutQuery.includes('/') ? withoutQuery.slice(0, withoutQuery.lastIndexOf('/') + 1) : '';
    if (sceneDir.endsWith('/scenes/')) return sceneDir.slice(0, -'scenes/'.length);
    return sceneDir;
  }

  function sceneRelativeAssetPath(assetPath) {
    if (typeof assetPath !== 'string' || !assetPath.startsWith('assets/')) return assetPath;
    const base = assetBaseForSceneSource(activeSceneSource);
    return base ? `${base}${assetPath}` : assetPath;
  }
  const api = Object.freeze({
    apiVersion: runtimeApiVersion,
    apiCompatibility: runtimeApiCompatibility,
    apiInventory() {
      return runtimeApiInventory(api);
    },
    getWorldState() {
      const state = clone(world);
      state.gridPuzzle = world.gridPuzzle && gridPuzzleModule
        ? gridPuzzleModule.worldStateView(world.gridPuzzle)
        : null;
      state.uiux = world.uiux && uiuxFlowModule
        ? uiuxFlowModule.worldStateView(world.uiux)
        : null;
      state.deckRoguelike = world.deckRoguelike && deckRoguelikeModule
        ? deckRoguelikeModule.worldStateView(world.deckRoguelike)
        : null;
      state.deckbuilderUi = world.deckbuilderUi && deckbuilderUiModule
        ? deckbuilderUiModule.worldStateView(world.deckbuilderUi)
        : null;
      state.juice = world.juice ? juice.worldStateView(world.juice) : null;
      state.cardRogueliteSubstrate = state.deckRoguelike
        ? state.deckRoguelike.cardRogueliteSubstrate
        : null;
      state.input = clone(input);
      state.rawInput = { directions: clone(input), keys: clone(rawKeys) };
      state.actionInput = clone(actionInput);
      state.actionState = resolvedActionState();
      state.inputDiagnostics = inputActionDiagnostics();
      const frameId = `tick-${world.tick}`;
      state.renderer = renderer.debugState(rendererState, world.entities);
      state.camera = cameraEvidence();
      state.renderBreakdown = renderer.renderBreakdown({ world: state, renderer: rendererState, frameId });
      state.renderQueue = typeof renderer.renderQueue === 'function'
        ? renderer.renderQueue({ world: state, renderer: rendererState, tilemap, frameId })
        : null;
      state.scene3dCamera = scene3dCameraProbeSummary({ scene3d: state.scene3d, frameId });
      state.scene3dTransforms = scene3dTransformSummary({ scene3d: state.scene3d, frameId });
      state.scene3dRender = typeof renderer.scene3dRenderSummary === 'function'
        ? renderer.scene3dRenderSummary({ world: state, frameId })
        : null;
      state.scene3dAnimation = scene3dAnimationSummary({ advanceFrames: 0, frameId });
      state.scene3dAnimationEvents = clone(world.scene3dAnimationEvents);
      state.scene3dCollision = typeof collision.scene3dCollisionSummary === 'function'
        ? collision.scene3dCollisionSummary({ world: state, frameId })
        : { present: false, events: [] };
      state.scene3dCollisions = Array.isArray(state.scene3dCollision.events) ? clone(state.scene3dCollision.events) : [];
      state.collisions = mergeCollisionEvents(Array.isArray(state.collisions) ? state.collisions : [], state.scene3dCollisions);
      state.scene3dProbe = scene3dProbeSummary({ state, frameId });
      state.runtimeFrameBudget = runtimeFrameBudgetEvidence(frameId, state.renderQueue);
      const runtimeStateEvidence = runtimeState(frameId);
      state.runtimeState = {
        schemaVersion: 'runtime-state-read-model-v1',
        stateId: runtimeStateEvidence.stateId,
        sceneId: runtimeStateEvidence.sceneId,
        tick: runtimeStateEvidence.tick,
        digest: runtimeStateEvidence.digest,
        rng: clone(world.rng),
        authority: 'browser_runtime_evidence_input_not_trusted_persistence',
        readOnlyInspection: {
          trustedEmitter: 'browser-runtime-world-state',
          browserStudioMode: 'read-only runtime state evidence inspection',
          disallowedActions: ['trusted writes', 'command bridge', 'live mutation'],
        },
      };
      state.runtimeEvents = clone(events);
      state.runtimeDiagnostics = clone(runtimeDiagnostics);
      state.runtimeDiagnosticTypes = diagnosticTypes();
      state.runtimeApi = runtimeApiInventory(api);
      state.tilemaps = tilemap.debugState(world.tilemaps);
      state.composition = compositionDebugState(world.entities);
      state.componentModel = componentModelDebugState(world.entities);
      state.assetManifest = assets.manifestSummary ? assets.manifestSummary() : null;
      state.assets = assets.metadata();
      state.snapshots = snapshots.list();
      const currentPlayer = player();
      state.object = currentPlayer && currentPlayer.components
        ? {
          id: currentPlayer.id,
          ...clone(currentPlayer.components.transform),
          ...clone(currentPlayer.components.size),
        }
        : null;
      return state;
    },
    getFrameStats() {
      const frameId = `tick-${world.tick}`;
      const renderQueue = typeof renderer.renderQueue === 'function'
        ? renderer.renderQueue({ world, renderer: rendererState, tilemap, frameId })
        : { layers: [], renderables: [], validation: { status: 'unreported', blockedReasons: [], warnings: [] } };
      const scene3dRender = typeof renderer.scene3dRenderSummary === 'function'
        ? renderer.scene3dRenderSummary({ world, frameId })
        : { attemptedObjectCount: 0, visibleObjectCount: 0, skippedObjectCount: 0, failedObjectCount: 0 };
      const scene3dAnimation = scene3dAnimationSummary({ advanceFrames: 0, frameId });
      const scene3dTransforms = scene3dTransformSummary({ scene3d: world.scene3d, frameId });
      const scene3dCamera = scene3dCameraProbeSummary({ scene3d: world.scene3d, frameId });
      const scene3dCollision = typeof collision.scene3dCollisionSummary === 'function'
        ? collision.scene3dCollisionSummary({ world, frameId })
        : { present: false, events: [] };
      const runtimeFrameBudget = runtimeFrameBudgetEvidence(frameId, renderQueue);
      return clone({
        tick: world.tick,
        fixedDeltaMs,
        entityCount: world.entities.length,
        eventCount: events.length,
        renderBreakdownFrameId: frameId,
        renderQueueFrameId: frameId,
        renderQueueLayerCount: renderQueue.layers.length,
        renderQueueRenderableCount: renderQueue.renderables.length,
        renderQueueDrawCallCount: renderQueue.renderables.filter((renderable) => renderable.visible !== false).length,
        renderQueueSkippedCount: renderQueue.renderables.filter((renderable) => renderable.visible === false).length,
        renderQueueBlockedReasonCount: Array.isArray(renderQueue.validation.blockedReasons) ? renderQueue.validation.blockedReasons.length : 0,
        renderQueueWarningCount: Array.isArray(renderQueue.validation.warnings) ? renderQueue.validation.warnings.length : 0,
        scene3dRenderFrameId: frameId,
        scene3dRenderAttemptedObjectCount: scene3dRender.attemptedObjectCount || 0,
        scene3dRenderVisibleObjectCount: scene3dRender.visibleObjectCount || 0,
        scene3dRenderSkippedObjectCount: scene3dRender.skippedObjectCount || 0,
        scene3dRenderFailedObjectCount: scene3dRender.failedObjectCount || 0,
        scene3dCameraCount: scene3dCamera.cameraCount || 0,
        scene3dTransformNodeCount: scene3dTransforms.nodeCount || 0,
        scene3dTransformCount: scene3dTransforms.transformCount || 0,
        scene3dCollisionEventCount: Array.isArray(scene3dCollision.events) ? scene3dCollision.events.length : 0,
        scene3dCollisionTriggerCount: scene3dCollision.triggerCount || 0,
        scene3dAnimationFrameId: frameId,
        scene3dAnimationStateCount: scene3dAnimation.stateCount || 0,
        scene3dAnimationActiveStateCount: scene3dAnimation.activeStateCount || 0,
        scene3dAnimationWarningCount: scene3dAnimation.warningCount || 0,
        tilemapRenderLayerCount: renderQueue.tilemapStats ? renderQueue.tilemapStats.layerCount : 0,
        tilemapRenderCellCount: renderQueue.tilemapStats ? renderQueue.tilemapStats.cellCount : 0,
        tilemapRenderDrawnTileCount: renderQueue.tilemapStats ? renderQueue.tilemapStats.drawnTileCount : 0,
        tilemapRenderMissingTileRefCount: renderQueue.tilemapStats ? renderQueue.tilemapStats.missingTileRefCount : 0,
        runtimeFrameBudgetStatus: runtimeFrameBudget.status,
        runtimeFrameBudgetViolationCount: runtimeFrameBudget.violations.length,
        runtimeFrameBudgetFrameId: runtimeFrameBudget.frameId,
        runtimeFrameBudgetCounts: runtimeFrameBudget.counts,
        juiceEventCount: Array.isArray(world.juiceEvents) ? world.juiceEvents.length : 0,
        activeJuiceCount: world.juice && Array.isArray(world.juice.active) ? world.juice.active.length : 0,
      });
    },
    getEvents() {
      return clone(events);
    },
    diagnosticTypes,
    getDiagnostics() {
      return clone(runtimeDiagnostics);
    },
    queryEntity,
    sampleRuntimeState,
    scenarioCoverageV100,
    step,
    pause() {
      world.paused = true;
      record('runtime.paused');
      return api.getFrameStats();
    },
    resume() {
      world.paused = false;
      record('runtime.resumed');
      return api.getFrameStats();
    },
    setInput,
    seedRng,
    nextRandom,
    rngState,
    snapshot,
    restore,
    runtimeState,
    createSave,
    loadSave,
    replayStateDigest,
    compareReplayDigest,
    runReplay,
    replayDeterminismCheck,
    loadScene,
    reload,
    transition,
    // In-game UI/UX flow navigation. Read-only with respect to trusted state:
    // it advances the deterministic runtime flow and returns the probe view.
    uiuxNavigate(action) {
      if (!world.uiux || !uiuxFlowModule) return null;
      world.uiux = uiuxFlowModule.navigate(world.uiux, action);
      record('runtime.uiux.navigate', {
        currentScreen: world.uiux.currentScreen,
        accepted: world.uiux.lastNavigation ? world.uiux.lastNavigation.accepted : false,
      });
      return uiuxFlowModule.worldStateView(world.uiux);
    },
    uiuxSetAccessibility(optionId, value) {
      if (!world.uiux || !uiuxFlowModule) return null;
      world.uiux = uiuxFlowModule.setAccessibility(world.uiux, optionId, value);
      record('runtime.uiux.accessibility', { optionId });
      return uiuxFlowModule.worldStateView(world.uiux);
    },
    // Deck-roguelike actions. Read-only with respect to trusted state: each
    // advances the deterministic deck run and returns the probe view.
    deckRoguelikePlayCard(handIndex) {
      if (!world.deckRoguelike || !deckRoguelikeModule) return null;
      const previousStatus = world.deckRoguelike.status;
      world.deckRoguelike = deckRoguelikeModule.advance(world.deckRoguelike, { action: 'play-card', handIndex });
      record('runtime.deck_roguelike.play_card', {
        handIndex,
        accepted: world.deckRoguelike.lastAction ? world.deckRoguelike.lastAction.accepted : false,
        status: world.deckRoguelike.status,
      });
      if (world.deckRoguelike.status !== previousStatus) {
        record('runtime.deck_roguelike.status_changed', { status: world.deckRoguelike.status, turn: world.deckRoguelike.turn });
      }
      const deckView = deckRoguelikeModule.worldStateView(world.deckRoguelike);
      if (world.deckbuilderUi && deckbuilderUiModule) {
        world.deckbuilderUi = deckbuilderUiModule.syncWithDeck(world.deckbuilderUi, deckView);
      }
      return deckView;
    },
    deckRoguelikeEndTurn() {
      if (!world.deckRoguelike || !deckRoguelikeModule) return null;
      const previousStatus = world.deckRoguelike.status;
      world.deckRoguelike = deckRoguelikeModule.advance(world.deckRoguelike, { action: 'end-turn' });
      record('runtime.deck_roguelike.end_turn', { turn: world.deckRoguelike.turn, status: world.deckRoguelike.status });
      if (world.deckRoguelike.status !== previousStatus) {
        record('runtime.deck_roguelike.status_changed', { status: world.deckRoguelike.status, turn: world.deckRoguelike.turn });
      }
      const deckView = deckRoguelikeModule.worldStateView(world.deckRoguelike);
      if (world.deckbuilderUi && deckbuilderUiModule) {
        world.deckbuilderUi = deckbuilderUiModule.syncWithDeck(world.deckbuilderUi, deckView);
      }
      return deckView;
    },
    deckbuilderUiSelectCard(handIndex) {
      if (!world.deckbuilderUi || !deckbuilderUiModule) return null;
      world.deckbuilderUi = deckbuilderUiModule.selectCard(world.deckbuilderUi, handIndex);
      record('runtime.deckbuilder_ui.select_card', {
        handIndex,
        accepted: world.deckbuilderUi.interaction.lastAction.accepted,
      });
      return deckbuilderUiModule.worldStateView(world.deckbuilderUi);
    },
    deckbuilderUiQueueSelected(slotId) {
      if (!world.deckbuilderUi || !deckbuilderUiModule) return null;
      world.deckbuilderUi = deckbuilderUiModule.queueSelected(world.deckbuilderUi, slotId);
      record('runtime.deckbuilder_ui.queue_selected', {
        slotId,
        accepted: world.deckbuilderUi.interaction.lastAction.accepted,
        draftOnly: true,
      });
      return deckbuilderUiModule.worldStateView(world.deckbuilderUi);
    },
    deckbuilderUiSelectShopOffer(offerId) {
      if (!world.deckbuilderUi || !deckbuilderUiModule) return null;
      world.deckbuilderUi = deckbuilderUiModule.selectShopOffer(world.deckbuilderUi, offerId);
      record('runtime.deckbuilder_ui.select_shop_offer', {
        offerId,
        accepted: world.deckbuilderUi.interaction.lastAction.accepted,
        draftOnly: true,
      });
      return deckbuilderUiModule.worldStateView(world.deckbuilderUi);
    },
    deckbuilderUiPlanRunMapNode(nodeId) {
      if (!world.deckbuilderUi || !deckbuilderUiModule) return null;
      world.deckbuilderUi = deckbuilderUiModule.planRunMapNode(world.deckbuilderUi, nodeId);
      record('runtime.deckbuilder_ui.plan_run_map_node', {
        nodeId,
        accepted: world.deckbuilderUi.interaction.lastAction.accepted,
        draftOnly: true,
      });
      return deckbuilderUiModule.worldStateView(world.deckbuilderUi);
    },
    whenReady() {
      return sceneReady;
    },
  });

  window.addEventListener('keydown', (event) => {
    const key = keyAlias(event.key);
    const patch = { keys: { [key]: true } };
    if (key === 'left' || key === 'a') patch.left = true;
    if (key === 'right' || key === 'd') patch.right = true;
    if (key === 'up' || key === 'w' || key === 'space') patch.up = true;
    if (key === 'down' || key === 's') patch.down = true;
    setInput(patch);
  });
  window.addEventListener('keyup', (event) => {
    const key = keyAlias(event.key);
    const patch = { keys: { [key]: false } };
    if (key === 'left' || key === 'a') patch.left = false;
    if (key === 'right' || key === 'd') patch.right = false;
    if (key === 'up' || key === 'w' || key === 'space') patch.up = false;
    if (key === 'down' || key === 's') patch.down = false;
    setInput(patch);
  });

  function sceneSourceFromLocation(locationValue = globalThis.location) {
    const fallback = 'scene.json';
    try {
      if (!locationValue || typeof locationValue.search !== 'string') return fallback;
      const params = new URLSearchParams(locationValue.search);
      const requested = params.get('scene');
      if (!requested) return fallback;
      if (requested.includes('..') || requested.includes('\\')) return fallback;
      if (/^[a-z][a-z0-9+.-]*:/i.test(requested)) return fallback;
      if (!requested.endsWith('.json')) return fallback;
      if (requested.startsWith('/examples/')) return requested;
      if (/^[A-Za-z0-9_./-]+$/.test(requested) && !requested.startsWith('/')) return requested;
      return fallback;
    } catch (_error) {
      return fallback;
    }
  }

  record('runtime.loaded', { api: Object.keys(api) });
  renderDebug();
  const sceneSource = sceneSourceFromLocation();
  sceneReady = fetch(sceneSource)
    .then((response) => response.json())
    .then((scene) => loadScene(scene, { sceneSource }))
    .catch((error) => {
      record('runtime.scene.load_failed', { sceneSource, error: String(error) });
      recordRuntimeDiagnostic('scene_load_failed', 'Initial scene fetch or load failed.', {
        sceneSource,
        error: String(error && error.message ? error.message : error),
      });
    });
  // Expose a readiness accessor so harnesses can await the fetched scene before
  // reading world state (otherwise they observe the synchronous fallback scene
  // and a late loadScene would reset the steps they executed in the interim).
  window.__OUROFORGE__ = api;
})();
