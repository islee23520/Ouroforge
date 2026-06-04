(function attachAudio(root) {
  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function normalizeAudioBuses(audio = {}) {
    const buses = audio && Array.isArray(audio.buses) ? audio.buses : [];
    return buses
      .filter((bus) => bus && typeof bus.id === 'string' && typeof bus.kind === 'string')
      .map((bus) => ({
        id: bus.id,
        kind: ['sound', 'music', 'ambient', 'ui'].includes(bus.kind) ? bus.kind : 'sound',
        volume: Number.isFinite(bus.volume) ? Math.max(0, Math.min(100, bus.volume)) : 100,
        muted: bus.muted === true,
      }));
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
        kind: ['sound', 'music', 'ambient', 'ui'].includes(event.kind) ? event.kind : 'sound',
        bus: typeof event.bus === 'string' && event.bus.length > 0 ? event.bus : null,
        asset: typeof event.asset === 'string' && event.asset.length > 0 ? event.asset : null,
      }));
  }

  function emitIntentEvents({ entities = [], trigger, tick = 0, muted = true } = {}) {
    const emitted = [];
    for (const entity of entities) {
      for (const event of normalizeAudioEvents(entity)) {
        if (event.trigger !== trigger) continue;
        const requestIndex = emitted.length + 1;
        const audio = entity.components && entity.components.audio ? entity.components.audio : {};
        const buses = normalizeAudioBuses(audio);
        const bus = buses.find((entry) => entry.id === event.bus)
          || buses.find((entry) => entry.kind === event.kind)
          || null;
        const busMuted = bus ? bus.muted === true : false;
        const outputMuted = muted !== false || busMuted;
        emitted.push({
          kind: 'audio_request',
          requestId: `audio-${tick}-${requestIndex}`,
          tick,
          name: event.name,
          trigger,
          action: event.action,
          intentKind: event.kind,
          busId: bus ? bus.id : event.bus,
          busKind: bus ? bus.kind : event.kind,
          volume: bus ? bus.volume : 100,
          busMuted,
          entityId: entity.id,
          asset: event.asset,
          muted: outputMuted,
          playback: 'intent',
          limitationWarnings: [
            'browser_audio_intent_only',
            'audible_output_not_verified',
            ...(outputMuted ? ['muted_or_unavailable_context'] : []),
          ],
        });
      }
    }
    return emitted.map(clone);
  }

  const api = Object.freeze({ normalizeAudioBuses, normalizeAudioEvents, emitIntentEvents });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeAudio = api;
})(typeof window !== 'undefined' ? window : globalThis);
