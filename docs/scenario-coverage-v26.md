# Scenario Coverage v26: End-to-End Provenance Regression Suite

Issue: #1505

Scenario Coverage v26 locks deterministic, fixture-scoped regression coverage
for End-to-End Provenance v1. It enumerates provenance bundle states,
provenance replay result states, and an additive compatibility golden for an
older change/read model that has no provenance bundle.

## Fixture Layout

- `examples/end-to-end-provenance-v1/scenario-coverage-v26/matrix.fixture.json`
  enumerates the regression cases and wording audits.
- `bundles/complete.fixture.json` covers a complete eight-link provenance
  bundle with replay inputs.
- `bundles/incomplete.fixture.json` covers an incomplete bundle with visible
  reasons and present refs.
- `bundles/dangling.fixture.json` covers a dangling reference state.
- `bundles/diverged.fixture.json` and `bundles/not-replayable.fixture.json`
  provide replay-specific bundle inputs.
- `replay-results/*.fixture.json` enumerates reproduced, diverged, and
  not-replayable result shapes.
- `compatibility/no-bundle-change.golden.json` proves that older change/read
  models without `provenanceBundle` remain valid.

## Coverage

| Case | Fixture | Expected state |
| --- | --- | --- |
| `V26.bundle.complete` | `bundles/complete.fixture.json` | `complete` with all required link states present. |
| `V26.bundle.incomplete` | `bundles/incomplete.fixture.json` | `incomplete` with explicit reasons and present refs. |
| `V26.bundle.dangling` | `bundles/dangling.fixture.json` | `dangling` with a validation-result ref reported as dangling. |
| `V26.replay.reproduced` | `replay-results/reproduced.fixture.json` | `reproduced` with empty diff and issues. |
| `V26.replay.diverged` | `replay-results/diverged.fixture.json` | `diverged` with a deterministic `$.status` diff shape. |
| `V26.replay.not-replayable` | `replay-results/not-replayable.fixture.json` | `not-replayable` with visible issues and no actual verdict. |
| `V26.compat.no-bundle-additive` | `compatibility/no-bundle-change.golden.json` | `valid` without a `provenanceBundle` field. |

## Runner

Run:

```bash
node examples/end-to-end-provenance-v1/scenario-coverage-v26-provenance.test.cjs
```

The runner asserts states and JSON shapes only. It does not use timing
assertions, browser automation, network access, command execution, trusted
writes, promotion, approval, merge, or source mutation behavior.

Rust/local persistence and serialization remain owned by the provenance bundle
and replay contract tests. The v26 Rust regression test reuses those public
model APIs against the v26 fixture directory so schema or replay-state breaks
fail under `cargo test`.

## Audits

- Generated state remains untracked unless explicitly fixture-scoped.
- Generated replay workspaces, dashboard exports, local temp files, browser
  profiles, command logs, and tool state remain ignored unless fixture-scoped.
- Wording stays conservative: this is local audit/replay regression evidence,
  not a broad readiness claim or engine-replacement claim.
- Changes without a provenance bundle remain valid because provenance bundles are additive.
- #1 remains open.
- #23 remains open.
- #1506 governance remains out of scope.
