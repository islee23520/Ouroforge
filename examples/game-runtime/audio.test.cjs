const assert = require('node:assert/strict');
const { normalizeAudioEvents, emitIntentEvents } = require('./audio.js');

const entities = [
  {
    id: 'player',
    components: {
      audio: {
        events: [
          { name: 'player_spawn', trigger: 'scene_loaded', action: 'play', asset: 'player-spawn-audio' },
          { name: 'player_stop', trigger: 'scene_loaded', action: 'stop', asset: 'player-spawn-audio' },
          { name: 'ignored', trigger: 'collision', action: 'play', asset: 'player-spawn-audio' },
        ],
      },
    },
  },
];

assert.deepEqual(normalizeAudioEvents(entities[0]), [
  { name: 'player_spawn', trigger: 'scene_loaded', action: 'play', asset: 'player-spawn-audio' },
  { name: 'player_stop', trigger: 'scene_loaded', action: 'stop', asset: 'player-spawn-audio' },
  { name: 'ignored', trigger: 'collision', action: 'play', asset: 'player-spawn-audio' },
]);

assert.deepEqual(emitIntentEvents({ entities, trigger: 'scene_loaded', tick: 7 }), [
  { kind: 'audio_request', requestId: 'audio-7-1', tick: 7, name: 'player_spawn', trigger: 'scene_loaded', action: 'play', entityId: 'player', asset: 'player-spawn-audio', muted: true, playback: 'intent' },
  { kind: 'audio_request', requestId: 'audio-7-2', tick: 7, name: 'player_stop', trigger: 'scene_loaded', action: 'stop', entityId: 'player', asset: 'player-spawn-audio', muted: true, playback: 'intent' },
]);

assert.deepEqual(emitIntentEvents({ entities, trigger: 'collision', tick: 8, muted: false }), [
  { kind: 'audio_request', requestId: 'audio-8-1', tick: 8, name: 'ignored', trigger: 'collision', action: 'play', entityId: 'player', asset: 'player-spawn-audio', muted: false, playback: 'intent' },
]);
