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
  }).createAssetTracker({
    onChange: () => renderCanvas(),
    onEvent: (asset) => record(`runtime.asset.${asset.status}`, {
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
    }),
  });
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
    collisionRules: { defaultLayer: 'default' },
    gameplayRules: { version: '1', flags: [] },
    sceneTransitions: [],
    transitionEvents: [],
    audioEvents: [],
    reloads: [],
    tilemaps: [],
    cameras: [],
    activeCameraId: null,
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
    const sourceEntities = Array.isArray(scene.entities) && scene.entities.length > 0
      ? scene.entities
      : defaultScene.entities;
    const bounds = size(scene.bounds, defaultScene.bounds);
    const componentDefaults = normalizeComponentDefaults(scene.componentDefaults);
    const normalizedRenderer = renderer.normalizeRenderer(scene.renderer, bounds);
    return {
      schemaVersion: String(scene.schemaVersion || defaultScene.schemaVersion),
      id: String(scene.id || 'unnamed-scene'),
      bounds,
      renderer: normalizedRenderer,
      activeCameraId: typeof scene.activeCameraId === 'string' ? scene.activeCameraId : null,
      cameras: normalizeCameras(scene.cameras, scene.activeCameraId, bounds, normalizedRenderer),
      tilemaps: tilemap.normalizeTilemaps(scene.tilemaps),
      collisionRules: normalizeCollisionRules(scene.collisionRules),
      gameplayRules: normalizeGameplayRules(scene.gameplayRules),
      sceneTransitions: normalizeSceneTransitions(scene.sceneTransitions),
      assetManifest: scene.assetManifest && typeof scene.assetManifest === 'object' ? objectValue(scene.assetManifest) : null,
      metadata: objectValue(scene.metadata),
      componentDefaults,
      entities: resolveComposition(sourceEntities.map((entity, index) => normalizeEntity(entity, index, componentDefaults))),
    };
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

  function animationState(entity) {
    const animationComponent = entity && entity.components && entity.components.animation;
    const state = animationComponent && animationComponent.state;
    if (!animationComponent || !state) return null;
    return {
      mode: animationComponent.mode,
      currentClip: state.currentClip || animationComponent.currentClip || null,
      frameIndex: Number.isInteger(state.frameIndex) ? state.frameIndex : 0,
      elapsedFrames: Number.isInteger(state.elapsedFrames) ? state.elapsedFrames : 0,
    };
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
        frameIndex: after.frameIndex,
        elapsedFrames: after.elapsedFrames,
      });
    }
  }

  function stepOne() {
    applyInput();
    applyGravity();
    const beforeAnimationStates = animationStatesByEntity(world.entities);
    animation.advanceAnimations(world.entities, 1);
    world.tick += 1;
    recordAnimationTransitions(beforeAnimationStates);
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
    for (const event of world.collisions) {
      world.collisionEvents.push(event);
      if (world.collisionEvents.length > 64) world.collisionEvents.shift();
      record(event.type, event);
    }
    refreshGroundedState(world.collisions);
    processTriggerEvents(world.collisions);
    updateCameraState();
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
    updateCameraState();
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
      sceneTransitionCount: world.sceneTransitions.length,
      assetCount: assetMetadata.length,
      gameplayFlagCount: world.gameplayRules.flags.length,
      assetManifestId: assets.manifestSummary ? assets.manifestSummary().id : null,
    });
    emitAudioEvents('scene_loaded');
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
      loadScene(scene);
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
    const names = ['status', 'input', 'trigger', 'goalFlag', 'cameraTarget', 'uiText', 'hudValue'];
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

  let sceneReady = Promise.resolve();
  const api = Object.freeze({
    getWorldState() {
      const state = clone(world);
      state.input = clone(input);
      const frameId = `tick-${world.tick}`;
      state.renderer = renderer.debugState(rendererState, world.entities);
      state.camera = cameraEvidence();
      state.renderBreakdown = renderer.renderBreakdown({ world: state, renderer: rendererState, frameId });
      state.renderQueue = typeof renderer.renderQueue === 'function'
        ? renderer.renderQueue({ world: state, renderer: rendererState, tilemap, frameId })
        : null;
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
      const frameId = `tick-${world.tick}`;
      const renderQueue = typeof renderer.renderQueue === 'function'
        ? renderer.renderQueue({ world, renderer: rendererState, tilemap, frameId })
        : { layers: [], renderables: [], validation: { status: 'unreported', blockedReasons: [], warnings: [] } };
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
        tilemapRenderLayerCount: renderQueue.tilemapStats ? renderQueue.tilemapStats.layerCount : 0,
        tilemapRenderCellCount: renderQueue.tilemapStats ? renderQueue.tilemapStats.cellCount : 0,
        tilemapRenderDrawnTileCount: renderQueue.tilemapStats ? renderQueue.tilemapStats.drawnTileCount : 0,
        tilemapRenderMissingTileRefCount: renderQueue.tilemapStats ? renderQueue.tilemapStats.missingTileRefCount : 0,
      });
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
    transition,
    whenReady() {
      return sceneReady;
    },
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
    .then((scene) => loadScene(scene))
    .catch((error) => record('runtime.scene.load_failed', { sceneSource, error: String(error) }));
  // Expose a readiness accessor so harnesses can await the fetched scene before
  // reading world state (otherwise they observe the synchronous fallback scene
  // and a late loadScene would reset the steps they executed in the interim).
  window.__OUROFORGE__ = api;
})();
