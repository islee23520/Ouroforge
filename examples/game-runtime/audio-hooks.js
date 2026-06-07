// Adaptive-Audio Runtime Hooks v1 (#1644).
//
// Deterministic runtime mirror of the trusted Rust evaluator
// (crates/ouroforge-core/src/audio_hooks.rs). Given a set of declarative
// adaptive-audio hooks and a bounded snapshot of world-state signals, it returns
// the ordered audio intents to emit. It reuses the existing audio-intent shape
// ({ behaviorId, actionId, targetEntityId, intent }); it adds adaptive-audio
// hooks, not a new audio engine, and emits intent metadata only — no synthesis,
// mixing, decoding, or playback. Evaluation is a pure function of (hooks,
// signals): identical signals always yield identical intents, so a restored
// world-state snapshot reproduces exactly the same audio intents.
(function attachAudioHooks(root) {
  const AUDIO_HOOKS_SCHEMA_VERSION = 'audio-hooks-v1';

  function requireText(field, value) {
    if (typeof value !== 'string' || value.trim() === '') {
      throw new Error(`${field} is required`);
    }
  }

  function conditionMatches(when, signals) {
    const flags = (signals && signals.flags) || {};
    const numbers = (signals && signals.numbers) || {};
    const events = (signals && Array.isArray(signals.events)) ? signals.events : [];
    switch (when && when.kind) {
      case 'always':
        return true;
      case 'flagEquals':
        return Object.prototype.hasOwnProperty.call(flags, when.flag)
          && flags[when.flag] === when.value;
      case 'numberAtLeast': {
        const value = numbers[when.signal];
        return Number.isFinite(value) && value >= when.threshold;
      }
      case 'numberBelow': {
        const value = numbers[when.signal];
        return Number.isFinite(value) && value < when.threshold;
      }
      case 'eventPresent':
        return events.includes(when.event);
      default:
        throw new Error(`unknown audio hook condition kind: ${when && when.kind}`);
    }
  }

  // Validate a condition fail-closed, mirroring the Rust
  // AudioHookCondition::validate so the runtime stays in lockstep.
  function validateCondition(when, hookId) {
    switch (when && when.kind) {
      case 'always':
        break;
      case 'flagEquals':
        requireText(`audio hook ${hookId} condition flag`, when.flag);
        requireText(`audio hook ${hookId} condition value`, when.value);
        break;
      case 'numberAtLeast':
      case 'numberBelow':
        requireText(`audio hook ${hookId} condition signal`, when.signal);
        if (!Number.isFinite(when.threshold)) {
          throw new Error(`audio hook ${hookId} condition threshold must be finite`);
        }
        break;
      case 'eventPresent':
        requireText(`audio hook ${hookId} condition event`, when.event);
        break;
      default:
        throw new Error(`unknown audio hook condition kind: ${when && when.kind}`);
    }
  }

  function validateAudioHookSet(hookSet) {
    if (!hookSet || hookSet.schemaVersion !== AUDIO_HOOKS_SCHEMA_VERSION) {
      throw new Error(`audio hook set schemaVersion must be "${AUDIO_HOOKS_SCHEMA_VERSION}"`);
    }
    if (!Array.isArray(hookSet.hooks) || hookSet.hooks.length === 0) {
      throw new Error('audio hook set must declare at least one hook');
    }
    if (hookSet.hooks.length > 64) {
      throw new Error('audio hook set is overbroad for v1 (max 64 hooks)');
    }
    const seen = new Set();
    for (const hook of hookSet.hooks) {
      requireText('audio hook hookId', hook && hook.hookId);
      if (seen.has(hook.hookId)) {
        throw new Error(`audio hook hookId "${hook.hookId}" is declared more than once`);
      }
      seen.add(hook.hookId);
      // `priority` is optional (defaults to 0) but, when present, must be an
      // integer to match the Rust i64 contract (which rejects non-integer or
      // non-numeric values on deserialize).
      if (hook.priority !== undefined && !Number.isInteger(hook.priority)) {
        throw new Error(`audio hook ${hook.hookId} priority must be an integer`);
      }
      validateCondition(hook.when, hook.hookId);
      const emit = hook.emit || {};
      requireText(`audio hook ${hook.hookId} emit behaviorId`, emit.behaviorId);
      requireText(`audio hook ${hook.hookId} emit actionId`, emit.actionId);
      requireText(`audio hook ${hook.hookId} emit targetEntityId`, emit.targetEntityId);
      requireText(`audio hook ${hook.hookId} emit intent`, emit.intent);
    }
    return true;
  }

  // Evaluate hooks against a signal snapshot; return ordered audio intents
  // (priority descending, then hookId ascending) to match the Rust evaluator.
  function evaluateAudioHooks(hookSet, signals) {
    validateAudioHookSet(hookSet);
    const matched = hookSet.hooks.filter((hook) => conditionMatches(hook.when, signals));
    matched.sort((a, b) => {
      const pa = Number.isFinite(a.priority) ? a.priority : 0;
      const pb = Number.isFinite(b.priority) ? b.priority : 0;
      if (pb !== pa) return pb - pa;
      return a.hookId < b.hookId ? -1 : a.hookId > b.hookId ? 1 : 0;
    });
    return matched.map((hook) => ({
      behaviorId: hook.emit.behaviorId,
      actionId: hook.emit.actionId,
      targetEntityId: hook.emit.targetEntityId,
      intent: hook.emit.intent,
    }));
  }

  const api = Object.freeze({
    AUDIO_HOOKS_SCHEMA_VERSION,
    validateAudioHookSet,
    evaluateAudioHooks,
  });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeAudioHooks = api;
})(typeof window !== 'undefined' ? window : globalThis);
