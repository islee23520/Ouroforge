(() => {
  const fixedDeltaMs = 16;
  const input = { left: false, right: false, up: false, down: false };
  const events = [];
  const collision = window.OuroforgeCollision || { detectAabbCollisions: () => [] };
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
  }).createAssetTracker({ onChange: () => renderCanvas() });
  const animation = window.OuroforgeAnimation || {
    normalizeAnimation: () => null,
    advanceAnimations: () => {},
    activeSpriteFrame: () => null,
  };
  const audio = window.OuroforgeAudio || {
    emitIntentEvents: () => [],
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
  };
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
    bounds: clone(defaultScene.bounds),
    entities: clone(defaultScene.entities),
    metadata: clone(defaultScene.metadata),
    collisions: [],
    collisionEvents: [],
    audioEvents: [],
    reloads: [],
    tilemaps: [],
    assetManifest: null,
    goalFlags: {},
    physics: {
      gravity: 1,
      maxFallSpeed: 8,
      grounded: {},
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

  function stringList(value = []) {
    return Array.isArray(value) ? value.map(String) : [];
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
    if (components.audio && Array.isArray(components.audio.events)) {
      normalized.components.audio = {
        events: components.audio.events
          .filter((event) => event && event.trigger === 'scene_loaded' && typeof event.name === 'string')
          .map((event) => ({ name: event.name, trigger: event.trigger, action: event.action === 'stop' ? 'stop' : 'play', asset: event.asset })),
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
    const sourceEntities = Array.isArray(scene.entities) && scene.entities.length > 0
      ? scene.entities
      : defaultScene.entities;
    const bounds = size(scene.bounds, defaultScene.bounds);
    const componentDefaults = normalizeComponentDefaults(scene.componentDefaults);
    return {
      schemaVersion: String(scene.schemaVersion || defaultScene.schemaVersion),
      id: String(scene.id || 'unnamed-scene'),
      bounds,
      renderer: renderer.normalizeRenderer(scene.renderer, bounds),
      tilemaps: tilemap.normalizeTilemaps(scene.tilemaps),
      assetManifest: scene.assetManifest && typeof scene.assetManifest === 'object' ? objectValue(scene.assetManifest) : null,
      metadata: objectValue(scene.metadata),
      componentDefaults,
      entities: resolveComposition(sourceEntities.map((entity, index) => normalizeEntity(entity, index, componentDefaults))),
    };
  }


  function emitAudioEvents(trigger) {
    const emittedEvents = audio.emitIntentEvents({ entities: world.entities, trigger, tick: world.tick, muted: true });
    for (const emitted of emittedEvents) {
      world.audioEvents.push(emitted);
      if (world.audioEvents.length > 64) world.audioEvents.shift();
      record('runtime.audio.emitted', emitted);
    }
  }

  function applyInput() {
    const entity = player();
    const velocity = entity.components.velocity;
    const inputComponent = entity.components.input || {};
    const allowedActions = Array.isArray(inputComponent.allowedActions) ? inputComponent.allowedActions : [];
    const canMove = allowedActions.length === 0 || allowedActions.includes('move');
    const canJump = (allowedActions.length === 0 || allowedActions.includes('jump')) && Number.isFinite(inputComponent.jumpImpulse);
    const speed = canMove ? (Number.isFinite(inputComponent.moveSpeed) ? inputComponent.moveSpeed : 2) : 0;
    velocity.x = ((input.right ? 1 : 0) - (input.left ? 1 : 0)) * speed;
    if (canJump) {
      if (input.up && world.physics.grounded[entity.id]) {
        velocity.y = -inputComponent.jumpImpulse;
        world.physics.grounded[entity.id] = false;
        record('runtime.physics.jump', { entityId: entity.id, impulse: inputComponent.jumpImpulse });
      }
    } else {
      velocity.y = ((input.down ? 1 : 0) - (input.up ? 1 : 0)) * speed;
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
    for (const entity of world.entities) {
      if (isDynamicPhysicsEntity(entity)) grounded[entity.id] = false;
    }
    for (const event of collisionEvents) {
      if (event.type !== 'runtime.collision.contact') continue;
      if (event.movingEntityId && event.normal && event.normal.y === -1) {
        grounded[event.movingEntityId] = true;
        const entity = entityById(event.movingEntityId);
        if (entity && entity.components && entity.components.velocity) {
          entity.components.velocity.y = Math.min(0, entity.components.velocity.y || 0);
        }
      }
    }
    world.physics.grounded = grounded;
  }

  function entityById(entityId) {
    return world.entities.find((entity) => entity.id === entityId) || null;
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
          pairId: event.pairId,
          targetFlag: trigger.targetFlag,
        });
      }
    }
  }

  function stepOne() {
    applyInput();
    applyGravity();
    animation.advanceAnimations(world.entities, 1);
    world.tick += 1;
    if (typeof collision.stepAabbPhysics === 'function') {
      world.collisions = collision.stepAabbPhysics(world.entities, world.bounds, world.tick).events;
    } else {
      for (const entity of world.entities) {
        const transform = entity.components.transform;
        const velocity = entity.components.velocity;
        const size = entity.components.size;
        transform.x = Math.max(0, Math.min(world.bounds.width - size.width, transform.x + velocity.x));
        transform.y = Math.max(0, Math.min(world.bounds.height - size.height, transform.y + velocity.y));
      }
      world.collisions = collision.detectAabbCollisions(world.entities, world.tick);
    }
    for (const event of world.collisions) {
      world.collisionEvents.push(event);
      if (world.collisionEvents.length > 64) world.collisionEvents.shift();
      record(event.type, event);
    }
    refreshGroundedState(world.collisions);
    processTriggerEvents(world.collisions);
  }

  function renderCanvas() {
    const canvas = document.getElementById('game');
    if (!canvas) return;
    const context = canvas.getContext('2d');
    renderer.drawRuntime({ canvas, context, world, renderer: rendererState, assets, animation, tilemap });
  }

  function renderDebug() {
    renderCanvas();
    const debug = document.getElementById('debug');
    if (debug) debug.textContent = JSON.stringify(api.getWorldState(), null, 2);
  }

  function loadScene(scene) {
    const normalized = normalizeScene(scene);
    snapshots = snapshotFactory.createSnapshotRegistry();
    world.schemaVersion = normalized.schemaVersion;
    world.sceneId = normalized.id;
    world.bounds = clone(normalized.bounds);
    world.entities = clone(normalized.entities);
    world.componentDefaults = clone(normalized.componentDefaults);
    world.tilemaps = clone(normalized.tilemaps);
    world.assetManifest = normalized.assetManifest ? clone(normalized.assetManifest) : null;
    world.goalFlags = {};
    world.physics = { gravity: 1, maxFallSpeed: 8, grounded: {} };
    for (const entity of world.entities) {
      if (isDynamicPhysicsEntity(entity)) world.physics.grounded[entity.id] = false;
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
    world.metadata = clone(normalized.metadata);
    world.collisions = [];
    world.collisionEvents = [];
    world.audioEvents = [];
    world.tick = 0;
    const assetMetadata = assets.load(world, world.assetManifest);
    record('runtime.scene.loaded', {
      schemaVersion: world.schemaVersion,
      sceneId: world.sceneId,
      entityCount: world.entities.length,
      assetCount: assetMetadata.length,
      assetManifestId: assets.manifestSummary ? assets.manifestSummary().id : null,
    });
    emitAudioEvents('scene_loaded');
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
    record('runtime.input.changed', { input });
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
    const snapshotId = snapshots.capture({ world, input, events, rendererState }, world.tick);
    record('runtime.snapshot.captured', { snapshotId, tick: world.tick });
    renderDebug();
    return { snapshotId, metadata: snapshots.metadata(snapshotId) };
  }

  function restore(snapshotId) {
    const snapshotValue = snapshots.restore(snapshotId);
    Object.assign(world, clone(snapshotValue.world || world));
    Object.assign(input, clone(snapshotValue.input || input));
    events.splice(0, events.length, ...clone(snapshotValue.events || []));
    if (snapshotValue.rendererState) rendererState = clone(snapshotValue.rendererState);
    record('runtime.snapshot.restored', { snapshotId, tick: world.tick });
    renderDebug();
    return api.getWorldState();
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
    const names = ['status', 'input', 'trigger', 'goalFlag', 'cameraTarget', 'uiText'];
    const counts = Object.fromEntries(names.map((name) => [name, 0]));
    const entityComponents = entities.map((entity) => {
      const components = {};
      for (const name of names) {
        if (entity.components && entity.components[name]) {
          counts[name] += 1;
          components[name] = clone(entity.components[name]);
        }
      }
      return { entityId: entity.id, components };
    });
    return {
      version: '2',
      counts,
      entities: entityComponents,
      goalFlags: clone(world.goalFlags),
    };
  }

  const api = Object.freeze({
    getWorldState() {
      const state = clone(world);
      state.input = clone(input);
      state.renderer = renderer.debugState(rendererState, world.entities);
      state.tilemaps = tilemap.debugState(world.tilemaps);
      state.composition = compositionDebugState(world.entities);
      state.componentModel = componentModelDebugState(world.entities);
      state.assetManifest = assets.manifestSummary ? assets.manifestSummary() : null;
      state.assets = assets.metadata();
      state.snapshots = snapshots.list();
      const currentPlayer = player();
      state.object = {
        id: currentPlayer.id,
        ...clone(currentPlayer.components.transform),
        ...clone(currentPlayer.components.size),
      };
      return state;
    },
    getFrameStats() {
      return clone({ tick: world.tick, fixedDeltaMs, entityCount: world.entities.length, eventCount: events.length });
    },
    getEvents() {
      return clone(events);
    },
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
    snapshot,
    restore,
    loadScene,
    reload,
  });

  window.addEventListener('keydown', (event) => {
    if (event.key === 'ArrowLeft') setInput({ left: true });
    if (event.key === 'ArrowRight') setInput({ right: true });
    if (event.key === 'ArrowUp') setInput({ up: true });
    if (event.key === 'ArrowDown') setInput({ down: true });
  });
  window.addEventListener('keyup', (event) => {
    if (event.key === 'ArrowLeft') setInput({ left: false });
    if (event.key === 'ArrowRight') setInput({ right: false });
    if (event.key === 'ArrowUp') setInput({ up: false });
    if (event.key === 'ArrowDown') setInput({ down: false });
  });

  record('runtime.loaded', { api: Object.keys(api) });
  renderDebug();
  const sceneReady = fetch('scene.json')
    .then((response) => response.json())
    .then((scene) => loadScene(scene))
    .catch((error) => record('runtime.scene.load_failed', { error: String(error) }));
  // Expose a readiness accessor so harnesses can await the fetched scene before
  // reading world state (otherwise they observe the synchronous fallback scene
  // and a late loadScene would reset the steps they executed in the interim).
  api.whenReady = () => sceneReady;
  window.__OUROFORGE__ = api;
})();
