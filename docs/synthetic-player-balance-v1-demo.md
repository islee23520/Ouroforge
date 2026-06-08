# Synthetic Player Balance Demo v1

A deterministic, fixture-scoped demo of the Synthetic Player Balance v1 slice
(#1605) under #1 Era F Milestone 32. It composes the existing surfaces — it adds
no new engine, runtime, or writer — and reproduces from a fresh clone with **no
network and no live browser**.

The demo shows the milestone's loop end to end:

1. **Synthetic personas play a deck-roguelike.** Human-like persona agents
   (#1606) drive the existing deck-roguelike probe (#1601) over a seeded run
   distribution. They are not win-maximizers: skill governs misplay, style
   governs attack-vs-block.
2. **Balance telemetry flags a degenerate combo.** Aggregating the run records
   (#1607) produces a descriptive balance report. The `smite`+`hex` pair
   dominates usage and is co-played in every winning run, so it is flagged as a
   **degenerate combo** with a **replayable seed**; `brick` (cost 4, unaffordable
   with 3 energy) is flagged as a **dead item**.
3. **The read-only cockpit re-runs a proposed nerf and diffs the win-rate.** A
   proposed `smite` nerf (18 → 6 damage) is applied to a *copy* of the deck spec
   (the trusted spec is never mutated) and the identical seed distribution is
   re-run (#1608). The win-rate drops from **5/5 to 3/5**, and the nerf resolves
   the `smite` degeneracy. The cockpit is read-only and human-in-the-loop: the
   nerf is a proposal, never auto-applied and never a trusted write.

## Reproduce

The demo is reproduced and asserted by two smoke tests that re-derive every
number above (the flag, the replayable seed, and the re-run diff) and check the
committed evidence reproduces byte-for-byte:

```bash
# Runtime (JavaScript) smoke
node examples/synthetic-player-balance-v1/demo/demo.test.cjs

# Trusted (Rust) mirror
cargo test -p ouroforge-core --test synthetic_player_balance_demo_contract
```

## Fixtures

All inputs and the expected evidence are fixture-scoped under
`examples/synthetic-player-balance-v1/demo/`:

| File | Role |
| --- | --- |
| `deck-roguelike-demo-scene.json` | the seeded deck-roguelike encounter (seed 31) |
| `personas.json` | the persona roster (#1606) |
| `balance-change.json` | the proposed `smite` nerf (proposal only) |
| `demo-evidence.json` | the expected report digest, flagged combo + replay seed, and re-run diff |

## Evidence summary

- **Degenerate combo:** `hex` + `smite`, in 5/5 winning runs, replay seed `31` /
  persona `cautious-novice`.
- **Dead item:** `brick`.
- **Baseline win-rate:** `5/5`.
- **Re-run after the proposed `smite` 18 → 6 nerf:** win-rate `3/5` (Δ −2); the
  `smite` degeneracy is resolved.

## Scope and non-goals

This is a descriptive demo of seeded balance telemetry, not a balance or quality
guarantee. It does not claim a production-ready engine, Godot replacement or
parity, or that generated games are good, fun, or shippable. It performs no
trusted write, no auto-applied nerf, and no network or live-browser action; the
browser/Studio surface stays read-only. `#1` and `#23` remain open.
