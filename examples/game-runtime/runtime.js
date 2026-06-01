(() => {
  const fixedDeltaMs = 16;
  const input = { left: false, right: false, up: false, down: false };
  const events = [];
  const world = {
    tick: 0,
    fixedDeltaMs,
    paused: false,
    bounds: { width: 320, height: 180 },
    entities: [
      {
        id: 'player',
        components: {
          transform: { x: 32, y: 72 },
          velocity: { x: 0, y: 0 },
          size: { width: 16, height: 16 },
          controllable: true,
        },
      },
    ],
  };

  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function record(type, payload = {}) {
    events.push({ tick: world.tick, type, payload: clone(payload) });
    if (events.length > 64) events.shift();
  }

  function player() {
    return world.entities.find((entity) => entity.id === 'player');
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
    for (const entity of world.entities) {
      const transform = entity.components.transform;
      const velocity = entity.components.velocity;
      const size = entity.components.size;
      transform.x = Math.max(0, Math.min(world.bounds.width - size.width, transform.x + velocity.x));
      transform.y = Math.max(0, Math.min(world.bounds.height - size.height, transform.y + velocity.y));
    }
    world.tick += 1;
  }

  function renderDebug() {
    const debug = document.getElementById('debug');
    if (debug) debug.textContent = JSON.stringify(api.getWorldState(), null, 2);
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
    return clone({ world, input, events });
  }

  function restore(snapshotValue) {
    if (!snapshotValue || typeof snapshotValue !== 'object') throw new Error('snapshot object is required');
    Object.assign(world, clone(snapshotValue.world || world));
    Object.assign(input, clone(snapshotValue.input || input));
    events.splice(0, events.length, ...clone(snapshotValue.events || []));
    record('runtime.restored');
    renderDebug();
    return api.getWorldState();
  }

  const api = Object.freeze({
    getWorldState() {
      const state = clone(world);
      state.input = clone(input);
      state.object = {
        id: player().id,
        ...clone(player().components.transform),
        ...clone(player().components.size),
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
})();
