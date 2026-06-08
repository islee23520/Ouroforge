# Scenario Coverage v48: Localization Regression Suite

Issue: #1835
Anchor: #1 Era I Milestone 53 (Localization Pipeline v1)

Scenario Coverage v48 locks Localization Pipeline v1 with local deterministic
state/shape checks only. It covers string externalization, translated locale
proposal validation, missing translation rejection, placeholder mismatch
rejection, the deterministic Localization Demo v1, and a default-locale
backward-compatibility golden.

The suite does not run a live browser, use the network, assert wall-clock timing,
mutate trusted sources, auto-apply fixes, auto-merge, self-approve, automate
creative/tone judgment, or claim fun, quality, production readiness,
shippability, release authority, or Godot replacement/parity. Browser/Studio
surfaces remain read-only or draft-only. Generated runs/artifacts remain
untracked unless fixture-scoped. Issues #1 and #23 remain open.

## Matrix

`examples/localization-v1/scenario-coverage-v48/matrix.fixture.json` enumerates
these rows:

| Row | Surface | Expected state/shape |
| --- | --- | --- |
| `V48.externalization.catalog` | #1833 externalization | stable ids, source refs, context, and placeholder declarations match source text. |
| `V48.locale.generation.valid` | #1833 generated locale | complete `es-ES` locale proposal validates pass with `proposalOnly: true`. |
| `V48.locale.missing.reject` | #1833 missing translation | incomplete locale fails closed with a visible missing-key diagnostic. |
| `V48.locale.placeholder.reject` | #1833 placeholder drift | missing or renamed placeholders fail closed per string id. |
| `V48.demo.smoke` | #1834 demo | localized title and rejected incomplete locale reproduce without network/live browser. |
| `V48.default_locale.backcompat` | default-locale back-compat | source `en-US` catalog text remains valid without a generated locale proposal. |

## Reproducibility

Run:

```bash
cargo test -p ouroforge-core --test scenario_coverage_v48_localization
```

The runner reads tracked fixtures and validates Rust/local state only. It avoids
flaky timing assertions and subjective translation quality or feel/fun judgments.
