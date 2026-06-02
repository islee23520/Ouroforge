(() => {
  const fixedDeltaMs = 16;
  const input = { left: false, right: false, up: false, down: false };
  const events = [];
  const collision = window.OuroforgeCollision || { detectAabbCollisions: () => [] };
  const snapshots = (window.OuroforgeSnapshots || {
    createSnapshotRegistry: () => ({
      capture: () => 'snapshot-unavailable',
      restore: () => { throw new Error('snapshot registry unavailable'); },
      metadata: () => null,
      list: () => [],
    }),
  }).createSnapshotRegistry();
  const assets = (window.OuroforgeAssets || {
    createAssetTracker: () => ({
      load: () => [],
      metadata: () => [],
      imageFor: () => null,
    }),
  }).createAssetTracker();
  const animation = window.OuroforgeAnimation || {
    normalizeAnimation: () => null,
    advanceAnimations: () => {},
    activeSpriteFrame: () => null,
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
    audioEvents: [],
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
    return {
      x: Number.isFinite(value.x) ? value.x : fallback.x,
      y: Number.isFinite(value.y) ? value.y : fallback.y,
    };
  }

  function size(value = {}, fallback = { width: 16, height: 16 }) {
    return {
      width: Number.isFinite(value.width) && value.width > 0 ? value.width : fallback.width,
      height: Number.isFinite(value.height) && value.height > 0 ? value.height : fallback.height,
    };
  }

  function objectValue(value) {
    return value && typeof value === 'object' && !Array.isArray(value) ? clone(value) : {};
  }

  function normalizeEntity(entity = {}, index = 0) {
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
        transform: point(components.transform),
        velocity: point(components.velocity),
        size: size(components.size),
        controllable: Boolean(components.controllable),
      },
      tags: Array.isArray(entity.tags) ? entity.tags.map(String) : [],
      metadata: objectValue(entity.metadata),
    };
    if (typeof sprite.asset === 'string') normalized.sprite.asset = sprite.asset;
    if (components.audio && Array.isArray(components.audio.events)) {
      normalized.components.audio = {
        events: components.audio.events
          .filter((event) => event && event.trigger === 'scene_loaded' && typeof event.name === 'string')
          .map((event) => ({ name: event.name, trigger: event.trigger, asset: event.asset })),
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
      };
    }
    return normalized;
  }

  function normalizeScene(scene = {}) {
    const sourceEntities = Array.isArray(scene.entities) && scene.entities.length > 0
      ? scene.entities
      : defaultScene.entities;
    const bounds = size(scene.bounds, defaultScene.bounds);
    return {
      schemaVersion: String(scene.schemaVersion || defaultScene.schemaVersion),
      id: String(scene.id || 'unnamed-scene'),
      bounds,
      renderer: renderer.normalizeRenderer(scene.renderer, bounds),
      metadata: objectValue(scene.metadata),
      entities: sourceEntities.map((entity, index) => normalizeEntity(entity, index)),
    };
  }


  function emitAudioEvents(trigger) {
    for (const entity of world.entities) {
      const audio = entity.components.audio;
      const audioEvents = audio && Array.isArray(audio.events) ? audio.events : [];
      for (const event of audioEvents) {
        if (event.trigger !== trigger) continue;
        const emitted = {
          tick: world.tick,
          name: event.name,
          trigger,
          entityId: entity.id,
          asset: event.asset || null,
        };
        world.audioEvents.push(emitted);
        record('runtime.audio.emitted', emitted);
      }
    }
  }

  function applyInput() {
    const entity = player();
    const velocity = entity.components.velocity;
    const speed = 2;
    velocity.x = ((input.right ? 1 : 0) - (input.left ? 1 : 0)) * speed;
    velocity.y = ((input.down ? 1 : 0) - (input.up ? 1 : 0)) * speed;
  }

  function stepOne() {
    applyInput();
    animation.advanceAnimations(world.entities, 1);
    for (const entity of world.entities) {
      const transform = entity.components.transform;
      const velocity = entity.components.velocity;
      const size = entity.components.size;
      transform.x = Math.max(0, Math.min(world.bounds.width - size.width, transform.x + velocity.x));
      transform.y = Math.max(0, Math.min(world.bounds.height - size.height, transform.y + velocity.y));
    }
    world.tick += 1;
    world.collisions = collision.detectAabbCollisions(world.entities, world.tick);
    for (const event of world.collisions) record(event.type, event);
  }

  function renderCanvas() {
    const canvas = document.getElementById('game');
    if (!canvas) return;
    const context = canvas.getContext('2d');
    renderer.drawRuntime({ canvas, context, world, renderer: rendererState, assets, animation });
  }

  function renderDebug() {
    renderCanvas();
    const debug = document.getElementById('debug');
    if (debug) debug.textContent = JSON.stringify(api.getWorldState(), null, 2);
  }

  function loadScene(scene) {
    const normalized = normalizeScene(scene);
    world.schemaVersion = normalized.schemaVersion;
    world.sceneId = normalized.id;
    world.bounds = clone(normalized.bounds);
    world.entities = clone(normalized.entities);
    rendererState = clone(normalized.renderer);
    world.metadata = clone(normalized.metadata);
    world.collisions = [];
    world.audioEvents = [];
    world.tick = 0;
    const assetMetadata = assets.load(world.entities);
    record('runtime.scene.loaded', {
      schemaVersion: world.schemaVersion,
      sceneId: world.sceneId,
      entityCount: world.entities.length,
      assetCount: assetMetadata.length,
    });
    emitAudioEvents('scene_loaded');
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
    const snapshotId = snapshots.capture({ world, input, events }, world.tick);
    record('runtime.snapshot.captured', { snapshotId, tick: world.tick });
    renderDebug();
    return { snapshotId, metadata: snapshots.metadata(snapshotId) };
  }

  function restore(snapshotId) {
    const snapshotValue = snapshots.restore(snapshotId);
    Object.assign(world, clone(snapshotValue.world || world));
    Object.assign(input, clone(snapshotValue.input || input));
    events.splice(0, events.length, ...clone(snapshotValue.events || []));
    record('runtime.snapshot.restored', { snapshotId, tick: world.tick });
    renderDebug();
    return api.getWorldState();
  }

  const api = Object.freeze({
    getWorldState() {
      const state = clone(world);
      state.input = clone(input);
      state.renderer = renderer.debugState(rendererState, world.entities);
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

  window.__OUROFORGE__ = api;
  record('runtime.loaded', { api: Object.keys(api) });
  renderDebug();
  fetch('scene.json')
    .then((response) => response.json())
    .then((scene) => loadScene(scene))
    .catch((error) => record('runtime.scene.load_failed', { error: String(error) }));
})();
