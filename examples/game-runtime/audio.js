(function attachAudio(root) {
  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function normalizeAudioEvents(entity = {}) {
    const audio = entity.components && entity.components.audio;
    const events = audio && Array.isArray(audio.events) ? audio.events : [];
    return events
      .filter((event) => event && typeof event.name === 'string' && typeof event.trigger === 'string')
      .map((event) => ({
        name: event.name,
        trigger: event.trigger,
        action: event.action === 'stop' ? 'stop' : 'play',
        asset: typeof event.asset === 'string' && event.asset.length > 0 ? event.asset : null,
      }));
  }

  function emitIntentEvents({ entities = [], trigger, tick = 0, muted = true } = {}) {
    const emitted = [];
    for (const entity of entities) {
      for (const event of normalizeAudioEvents(entity)) {
        if (event.trigger !== trigger) continue;
        emitted.push({
          tick,
          name: event.name,
          trigger,
          action: event.action,
          entityId: entity.id,
          asset: event.asset,
          muted: muted !== false,
          playback: 'intent',
        });
      }
    }
    return emitted.map(clone);
  }

  const api = Object.freeze({ normalizeAudioEvents, emitIntentEvents });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeAudio = api;
})(typeof window !== 'undefined' ? window : globalThis);
