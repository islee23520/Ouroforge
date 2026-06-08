# Game-Feel and Juice Demo v1

Issue: #1822 — Game-Feel and Juice Demo v1  
Anchor: #1 Era I Milestone 51 (Game feel and juice)

This demo is a deterministic, fixture-scoped walkthrough of the Game-Feel and
Juice Toolkit v1. It shows that existing runtime juice primitives can make a
score cascade visible, that Rust/local score-cascade evidence preserves the
authoritative substrate order, and that responsiveness evidence records both a
within-budget response and an over-budget failure. It is mechanical evidence
only: it is not a fun/quality verdict, not a production-readiness claim, not a
new engine, and not browser trusted authority.

## Demo fixture

The tracked demo fixture is:

- `examples/game-feel-juice-v1/demo/game-feel-juice-demo-v1.json`

The fixture reuses the existing Game-Feel and Juice surfaces instead of creating
parallel systems:

- `examples/game-runtime/juice-scene-v1.json` for deterministic runtime juice
  primitives and read-only probe feedback;
- `examples/game-runtime/score-cascade-feedback-v1.json` for ordered Rust/local
  score-cascade payoff feedback;
- `examples/game-runtime/responsiveness-v1.json` for pass/fail sub-100ms
  responsiveness evidence;
- `docs/game-feel-juice-v1.md` for the milestone scope and boundary contract.

Generated demo runs, cascade traces, responsiveness reports, runtime snapshots,
dashboard exports, and browser-local inspection artifacts remain untracked unless
an issue explicitly scopes a small deterministic fixture as source-like data.

## Expected deterministic behavior

The smoke test recomputes the score-cascade trace from the fixture config and
asserts the ordered feedback sequence:

```text
base -> modifier -> modifier -> card-total -> base -> modifier -> card-total -> cascade-complete
```

The final score is `24`, and every feedback event uses the `score_cascade` juice
trigger. The feedback is read-only evidence over the existing card-roguelite
substrate. browser/Studio surfaces read-only: Browser and Studio surfaces may display it but must not recompute
score, write trusted state, execute commands, or promote generated output.

The responsiveness portion verifies two deterministic fixed-step cases:

| Scenario | Latency | Verdict |
| --- | ---: | --- |
| `responsiveness-within-budget` | 80ms | pass |
| `responsiveness-over-budget` | 112ms | fail |

The budget is 100ms. The failing case is intentionally included so the demo shows
that late feedback remains visible and fails closed instead of being treated as a
silent pass.

## Governance and wording audit

- #1 and #23 remain open; this demo does not close, narrow, or replace either
  anchor.
- Rust/local owns trusted validation, persistence, provenance, evidence writing,
  run/project binding, and the review/apply/trust-gradient path.
- TypeScript/JavaScript owns deterministic runtime feedback and read-only probe
  inspection only.
- The deckbuilder demo remains configuration over the card-roguelite substrate;
  this work does not introduce a parallel engine.
- Feel/fun judgment remains a human Era J gate. This demo verifies mechanical
  ordering and timing only.
- Public wording must avoid auto-merge, autonomous apply, subjective quality,
  fun, shippable, production-ready, Godot replacement/parity, hosted/cloud, or
  market-demand claims.

## Reproducibility

From a fresh clone, the issue-specific smoke test can be run locally without
network or a live browser:

```bash
cargo test -p ouroforge-core --test game_feel_juice_demo_contract -- --nocapture
```

The full issue gate also runs formatting, workspace tests, clippy, dashboard and
cockpit Node checks, `git diff --check`, and a final generated-state status audit.
