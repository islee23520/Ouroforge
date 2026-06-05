// Fixture (#725): an exported runtime bootstrap that drops a required probe
// hook (getEvents). The probe compatibility check must fail closed on it.
'use strict';
(function () {
  let tick = 0;
  const clone = (value) => JSON.parse(JSON.stringify(value));
  const probe = Object.freeze({
    getWorldState() { return clone({ tick }); },
    getFrameStats() { return clone({ tick }); },
    // getEvents is intentionally missing.
    snapshot() { return clone({ tick }); },
    step() { tick += 1; return clone({ tick }); },
    pause() { return clone({ tick }); },
    resume() { return clone({ tick }); },
    setInput() { return clone({ tick }); },
    restore() { return clone({ tick }); },
  });
  const globalScope = typeof window !== 'undefined' ? window : globalThis;
  globalScope.__OUROFORGE__ = probe;
})();
