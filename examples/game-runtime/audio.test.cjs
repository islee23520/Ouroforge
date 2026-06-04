const assert = require('node:assert/strict');
const { normalizeAudioBuses, normalizeAudioEvents, emitIntentEvents } = require('./audio.js');

const entities = [
  {
    id: 'player',
    components: {
      audio: {
        buses: [
          { id: 'sfx', kind: 'sound', volume: 80, muted: false },
          { id: 'music', kind: 'music', volume: 50, muted: true },
        ],
        events: [
          { name: 'player_spawn', trigger: 'scene_loaded', action: 'play', kind: 'sound', bus: 'sfx', asset: 'player-spawn-audio' },
          { name: 'player_stop', trigger: 'scene_loaded', action: 'stop', kind: 'music', bus: 'music', asset: 'player-spawn-audio' },
          { name: 'ignored', trigger: 'collision', action: 'play', asset: 'player-spawn-audio' },
        ],
      },
    },
  },
];

assert.deepEqual(normalizeAudioBuses(entities[0].components.audio), [
  { id: 'sfx', kind: 'sound', volume: 80, muted: false },
  { id: 'music', kind: 'music', volume: 50, muted: true },
]);

assert.deepEqual(normalizeAudioEvents(entities[0]), [
  { name: 'player_spawn', trigger: 'scene_loaded', action: 'play', kind: 'sound', bus: 'sfx', asset: 'player-spawn-audio' },
  { name: 'player_stop', trigger: 'scene_loaded', action: 'stop', kind: 'music', bus: 'music', asset: 'player-spawn-audio' },
  { name: 'ignored', trigger: 'collision', action: 'play', kind: 'sound', bus: null, asset: 'player-spawn-audio' },
]);

assert.deepEqual(emitIntentEvents({ entities, trigger: 'scene_loaded', tick: 7 }), [
  { kind: 'audio_request', requestId: 'audio-7-1', tick: 7, name: 'player_spawn', trigger: 'scene_loaded', action: 'play', intentKind: 'sound', busId: 'sfx', busKind: 'sound', volume: 80, busMuted: false, entityId: 'player', asset: 'player-spawn-audio', muted: true, playback: 'intent', limitationWarnings: ['browser_audio_intent_only', 'audible_output_not_verified', 'muted_or_unavailable_context'] },
  { kind: 'audio_request', requestId: 'audio-7-2', tick: 7, name: 'player_stop', trigger: 'scene_loaded', action: 'stop', intentKind: 'music', busId: 'music', busKind: 'music', volume: 50, busMuted: true, entityId: 'player', asset: 'player-spawn-audio', muted: true, playback: 'intent', limitationWarnings: ['browser_audio_intent_only', 'audible_output_not_verified', 'muted_or_unavailable_context'] },
]);

assert.deepEqual(emitIntentEvents({ entities, trigger: 'collision', tick: 8, muted: false }), [
  { kind: 'audio_request', requestId: 'audio-8-1', tick: 8, name: 'ignored', trigger: 'collision', action: 'play', intentKind: 'sound', busId: 'sfx', busKind: 'sound', volume: 80, busMuted: false, entityId: 'player', asset: 'player-spawn-audio', muted: false, playback: 'intent', limitationWarnings: ['browser_audio_intent_only', 'audible_output_not_verified'] },
]);
