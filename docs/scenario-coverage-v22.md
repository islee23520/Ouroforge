# Scenario Coverage v22

Scenario Coverage v22 is the Milestone 21 regression suite for second game class loop generalization.

## Required Coverage

- Validate the second game project manifest, Seed, scenario pack, and scene refs.
- Validate second-class four-gate verdict evidence.
- Validate loop coverage refs for Seed, Build, Observe, Verify, Journal, and Evolve.
- Validate comparable evidence shape between collect-and-exit and signal-gate-platformer.
- Validate explicit structured gap findings for unsupported or incomplete generalization work.
- Preserve collect-and-exit compatibility by checking the existing source fixture still parses.
- Reject missing comparison inputs, incomparable evidence shapes, and stale evidence refs.

## Runner

Run:

```sh
node examples/signal-gate-platformer/scenario-coverage-v22-loop-generalization.test.cjs
```

The runner is offline and fixture-scoped. It does not read generated runs, open a browser, or rely on network state.

## Boundary

Coverage v22 is a regression contract over evidence shape and local parsing. It is not a quality score, production-readiness gate, or broad genre-support claim.
