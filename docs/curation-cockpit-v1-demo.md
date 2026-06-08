# Curation Cockpit Demo v1

This fixture-scoped demo shows the Era J curation loop without network access, a live browser, or generated state outside `examples/curation-cockpit-v1/demo/`.

## Inputs

- `examples/curation-cockpit-v1/demo/candidate-generation-brief-demo-v1.json` requests **3** deterministic deck-roguelike candidates: a card candidate, a tuning candidate, and a flavor candidate.
- `examples/curation-cockpit-v1/demo/curation-selection-demo-v1.json` records the human selection of `demo-card-rivet-strike-v1` as read-only provenance.

## Deterministic path

1. Rust parses the candidate brief and calls `generate_candidates` with a fixed timestamp.
2. Each candidate goes through the existing generative front door and remains `status = proposed`, `verdictStatus = pending`, and `confidence = unverified`.
3. Rust records a human selection with `record_human_selection` and replays it with `replay_selection`, checking the candidate set id, proposal id, and selected payload digest.
4. `build_curation_read_model` exposes only `inspect-candidates` and `record-selection-provenance` for read-only cockpit/dashboard consumers.

The demo does not grant trusted write authority, does not auto-apply, does not auto-merge, and does not claim an automated fun, quality, release, or market verdict. It demonstrates mechanical candidate generation plus provenance recording only; the human fun/feel and release decisions remain outside the engine.

## Smoke test

From a fresh clone, run:

```bash
cargo test -p ouroforge-core --test curation_cockpit_demo_contract --jobs 2
```

The smoke test asserts the documented candidate count and ids, verifies the fixture selection against the computed payload digest, replays the selection, and confirms the read model stays read-only.
