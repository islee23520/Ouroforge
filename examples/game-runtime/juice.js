(function attachJuice(root) {
  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  const KINDS = new Set(['tween', 'shake', 'hit-stop', 'sfx']);
  const EASINGS = new Set(['linear', 'ease-out', 'ease-in', 'ease-in-out']);

  function finite(value, fallback = 0) {
    return Number.isFinite(value) ? value : fallback;
  }

  function boundedPositiveInt(value, fallback, min = 1, max = 240) {
    if (!Number.isFinite(value)) return fallback;
    return Math.max(min, Math.min(max, Math.floor(value)));
  }

  function normalizePrimitive(primitive, index = 0) {
    if (!primitive || typeof primitive !== 'object') return null;
    const id = typeof primitive.id === 'string' && primitive.id ? primitive.id : `juice-${index}`;
    const kind = KINDS.has(primitive.kind) ? primitive.kind : 'tween';
    const trigger = typeof primitive.trigger === 'string' && primitive.trigger ? primitive.trigger : 'tick';
    return {
      id,
      kind,
      trigger,
      disabled: primitive.disabled === true,
      targetEntityId: typeof primitive.targetEntityId === 'string' && primitive.targetEntityId ? primitive.targetEntityId : null,
      durationFrames: boundedPositiveInt(primitive.durationFrames, kind === 'hit-stop' ? 2 : 8),
      easing: EASINGS.has(primitive.easing) ? primitive.easing : 'linear',
      intensity: Math.max(0, Math.min(32, finite(primitive.intensity, kind === 'shake' ? 4 : 1))),
      property: typeof primitive.property === 'string' && primitive.property ? primitive.property : (kind === 'tween' ? 'scale' : null),
      from: finite(primitive.from, 0),
      to: finite(primitive.to, 1),
      axis: ['x', 'y', 'both'].includes(primitive.axis) ? primitive.axis : 'both',
      sfx: primitive.sfx && typeof primitive.sfx === 'object' ? {
        name: typeof primitive.sfx.name === 'string' && primitive.sfx.name ? primitive.sfx.name : `${id}-sfx`,
        action: primitive.sfx.action === 'stop' ? 'stop' : 'play',
        kind: ['sound', 'music', 'ambient', 'ui'].includes(primitive.sfx.kind) ? primitive.sfx.kind : 'sound',
        bus: typeof primitive.sfx.bus === 'string' && primitive.sfx.bus ? primitive.sfx.bus : null,
        asset: typeof primitive.sfx.asset === 'string' && primitive.sfx.asset ? primitive.sfx.asset : null,
      } : null,
    };
  }

  function normalizeJuiceConfig(config = {}) {
    const source = config && typeof config === 'object' && !Array.isArray(config) ? config : {};
    const primitives = Array.isArray(source.primitives) ? source.primitives : [];
    return {
      schemaVersion: 'ouroforge.runtime-juice-config.v1',
      enabled: source.enabled !== false,
      primitives: primitives.map(normalizePrimitive).filter(Boolean).slice(0, 32),
      readOnlyInspection: {
        trustedEmitter: 'browser-runtime-juice-probe',
        browserStudioMode: 'read-only mechanical game-feel feedback inspection',
        disallowedActions: ['trusted writes', 'command bridge', 'live mutation', 'auto-fun verdict'],
      },
      boundary: 'Deterministic juice feedback events only; feel/fun judgment remains human and trusted state stays Rust/local.',
    };
  }

  function createJuiceState(config = {}, sceneId = 'unknown') {
    const normalized = normalizeJuiceConfig(config);
    return {
      schemaVersion: 'ouroforge.runtime-juice-state.v1',
      sceneId: String(sceneId || 'unknown'),
      config: normalized,
      sequence: 0,
      active: [],
      emitted: [],
      lastTrigger: null,
    };
  }

  function easingValue(name, t) {
    const clamped = Math.max(0, Math.min(1, finite(t, 0)));
    if (name === 'ease-in') return clamped * clamped;
    if (name === 'ease-out') return 1 - ((1 - clamped) * (1 - clamped));
    if (name === 'ease-in-out') {
      return clamped < 0.5 ? 2 * clamped * clamped : 1 - Math.pow(-2 * clamped + 2, 2) / 2;
    }
    return clamped;
  }

  function samplePrimitive(primitive, elapsedFrames) {
    const duration = Math.max(1, primitive.durationFrames);
    const progress = Math.max(0, Math.min(1, elapsedFrames / duration));
    const easedProgress = easingValue(primitive.easing, progress);
    const sample = { progress, easedProgress };
    if (primitive.kind === 'tween') {
      sample.property = primitive.property;
      sample.value = primitive.from + ((primitive.to - primitive.from) * easedProgress);
    } else if (primitive.kind === 'shake') {
      const direction = (elapsedFrames % 2 === 0) ? 1 : -1;
      const amplitude = primitive.intensity * (1 - easedProgress);
      sample.offset = {
        x: primitive.axis === 'y' ? 0 : Number((direction * amplitude).toFixed(4)),
        y: primitive.axis === 'x' ? 0 : Number((-direction * amplitude).toFixed(4)),
      };
    } else if (primitive.kind === 'hit-stop') {
      sample.holdFramesRemaining = Math.max(0, primitive.durationFrames - elapsedFrames);
      sample.runtimePaused = false;
      sample.explanation = 'mechanical hit-stop feedback event only; runtime stepping remains deterministic';
    } else if (primitive.kind === 'sfx') {
      sample.audioIntent = primitive.sfx ? clone(primitive.sfx) : {
        name: `${primitive.id}-sfx`, action: 'play', kind: 'sound', bus: null, asset: null,
      };
    }
    return sample;
  }

  function feedbackEvent(primitive, state, context = {}) {
    const tick = Number.isFinite(context.tick) ? context.tick : 0;
    state.sequence += 1;
    const base = {
      schemaVersion: 'ouroforge.runtime-juice-feedback.v1',
      feedbackId: `juice-${tick}-${state.sequence}`,
      sceneId: String(context.sceneId || state.sceneId || 'unknown'),
      primitiveId: primitive.id,
      kind: primitive.kind,
      trigger: primitive.trigger,
      tick,
      durationFrames: primitive.durationFrames,
      elapsedFrames: 0,
      expiresAtTick: tick + primitive.durationFrames,
      easing: primitive.easing,
      intensity: primitive.intensity,
      targetEntityId: primitive.targetEntityId,
      readOnlyEvidence: true,
      boundary: 'mechanical deterministic feedback evidence; not a fun/quality verdict and not a trusted write',
      sourceEventType: context.sourceEvent && context.sourceEvent.type ? String(context.sourceEvent.type) : null,
    };
    return { ...base, sample: samplePrimitive(primitive, 0) };
  }

  function emitFeedback(state, trigger, context = {}) {
    if (!state || !state.config || state.config.enabled === false) return { state, events: [] };
    const emitted = [];
    for (const primitive of state.config.primitives) {
      if (primitive.disabled || primitive.trigger !== trigger) continue;
      const event = feedbackEvent(primitive, state, context);
      emitted.push(event);
      state.active.push(clone(event));
      state.emitted.push(clone(event));
    }
    if (state.active.length > 64) state.active = state.active.slice(-64);
    if (state.emitted.length > 64) state.emitted = state.emitted.slice(-64);
    state.lastTrigger = emitted.length ? String(trigger) : state.lastTrigger;
    return { state, events: emitted.map(clone) };
  }

  function advanceFeedback(state, tick = 0) {
    if (!state || !Array.isArray(state.active)) return { state, events: [] };
    const updates = [];
    const primitivesById = new Map((state.config && state.config.primitives || []).map((primitive) => [primitive.id, primitive]));
    const nextActive = [];
    for (const active of state.active) {
      const primitive = primitivesById.get(active.primitiveId);
      if (!primitive) continue;
      const elapsed = Math.max(0, Math.min(primitive.durationFrames, (Number.isFinite(tick) ? tick : active.tick) - active.tick));
      const updated = { ...clone(active), elapsedFrames: elapsed, tick: active.tick, sample: samplePrimitive(primitive, elapsed) };
      if (elapsed < primitive.durationFrames) nextActive.push(updated);
      updates.push({ ...clone(updated), updateTick: Number.isFinite(tick) ? tick : active.tick, type: 'runtime.juice.feedback_update' });
    }
    state.active = nextActive;
    return { state, events: updates };
  }

  function worldStateView(state) {
    const source = state && typeof state === 'object' ? state : createJuiceState({}, 'unknown');
    return {
      schemaVersion: 'ouroforge.runtime-juice-probe.v1',
      sceneId: source.sceneId,
      primitiveCount: source.config && Array.isArray(source.config.primitives) ? source.config.primitives.length : 0,
      activeCount: Array.isArray(source.active) ? source.active.length : 0,
      emittedCount: Array.isArray(source.emitted) ? source.emitted.length : 0,
      lastTrigger: source.lastTrigger || null,
      active: clone(source.active || []),
      emitted: clone(source.emitted || []),
      readOnlyInspection: {
        trustedEmitter: 'browser-runtime-juice-probe',
        browserStudioMode: 'read-only juice feedback inspection',
        disallowedActions: ['trusted writes', 'command bridge', 'live mutation', 'auto-fun verdict'],
      },
      boundary: 'Mechanical feedback event evidence only; feel/fun judgment remains human (Era J).',
    };
  }

  const api = Object.freeze({
    normalizeJuiceConfig,
    createJuiceState,
    emitFeedback,
    advanceFeedback,
    worldStateView,
    easingValue,
  });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeJuice = api;
})(typeof window !== 'undefined' ? window : globalThis);
