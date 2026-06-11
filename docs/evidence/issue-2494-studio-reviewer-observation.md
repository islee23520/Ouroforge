# Issue #2494 local Studio reviewer observation evidence

Closure classification: product-observed complete

This note records a bounded local Studio/reviewer observation for the existing
Signal Gate Relay workflow. The observation is read-only: Studio renders and
records Rust-owned evidence/read-model references, while write-affecting actions
remain outside Studio and must route through existing Rust/local gates.

## Local Studio entry point

The Studio shell is reproducible from a tracked script without dependency installation by loading the
existing local Studio modules directly:

```bash
elixir scripts/render_studio_observation.exs
```

The tracked script regenerates the ignored transcript:

```text
runs/issue-2494/studio-observation/studio-transcript.txt
```

Diagnostics:

```text
runs/issue-2494/studio-observation/studio-stderr.txt  # 0 bytes
```

## Browser presentation capture

The generated read-only Studio observation page was served locally and captured
in a real browser:

```bash
python3 -m http.server 8884 --bind 127.0.0.1
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
  --headless=new --disable-gpu --no-first-run --no-default-browser-check \
  --window-size=1280,900 \
  --screenshot=runs/issue-2494/studio-observation/screenshots/studio-reviewer-observation.png \
  'http://127.0.0.1:8884/runs/issue-2494/studio-observation/studio-observation.html'
```

Screenshot evidence:

```text
runs/issue-2494/studio-observation/screenshots/studio-reviewer-observation.png
PNG image data, 1280 x 900, 8-bit/color RGB, non-interlaced
```

## Workspace and decision transcript

Project identity:

```text
examples/playable-demo-v2/signal-gate-dogfood/ouroforge.project.json
examples/playable-demo-v2/signal-gate-dogfood/scenes/signal-gate-relay.scene.json
```

Reviewer observation decision:

```text
Reviewer observed Signal Gate Relay #2493 evidence/read-model as read-only and
accepted the bounded screenshot gap closure; no write-affecting Studio action was
taken.
```

Read-only/trusted-write boundary recorded by the Studio shell:

```text
readOnlyRendering=true
rustDataPlaneOwnsTruth=true
trustedWriteAuthority=false
directArtifactWrite=false
commandBridge=false
```

The transcript also records the evidence refs inspected by Studio, including the
Signal Gate Relay project, scene, #2493 stable evidence summary, and generated
live-observability manifest path.

## Manual steps and follow-up visibility

The local reviewer observation entry point is tracked and reproducible; generated transcript/page/screenshot outputs remain ignored local evidence.
The remaining human-owned manual judgment is intentionally not hidden:

| Manual item | Owner | Follow-up |
| --- | --- | --- |
| Human playtest fun/feel judgment | user/human playtester | #2496 environment/template handoff; fun/feel judgment is not automated here |

## Generated-state audit

Generated Studio transcript, browser page, screenshot, and summary stayed under
ignored `runs/issue-2494/studio-observation/`. Tracked source contains this
stable summary, the JSON summary beside it, and `scripts/render_studio_observation.exs`.

#1 and #23 remain open governance anchors; #1 body was not edited.
