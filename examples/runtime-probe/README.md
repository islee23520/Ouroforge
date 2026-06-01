# Ouroforge Runtime Probe

This directory contains the minimal browser probe page for issue #7.1.
It is a static page that exposes `window.__OUROFORGE__` and a deterministic moving-square state.
It is intentionally not a game engine, renderer abstraction, ECS, asset pipeline, or editor.

## Serve locally

```bash
python3 -m http.server 8767 --bind 127.0.0.1 --directory examples/runtime-probe
```

Then open <http://127.0.0.1:8767/> or point the Ouroforge browser smoke command at that URL.

## Probe API

`window.__OUROFORGE__` exposes:

- `getWorldState()`
- `getFrameStats()`
- `getEvents()`
- `step(count = 1)`
- `pause()`
- `resume()`
- `setInput({ left, right, up, down })`
- `snapshot()`
- `restore(snapshot)`

The page keeps a fixed `fixedDeltaMs` value and a deterministic tick counter. The only object is a single moving square used to prove browser runtime observability.
