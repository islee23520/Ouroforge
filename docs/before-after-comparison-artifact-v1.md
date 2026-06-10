# Before/After Comparison Artifact v1

Before/After Comparison Artifact v1 is the #2380/M126 report contract for
reviewed gameplay changes. It compares two live evidence bundle summaries and
emits both JSON and Markdown reports for M127 journal consumption.

## Verdict enum

The artifact verdict is one of:

- `improvement`
- `regression`
- `no-change`
- `inconclusive`

The output is deterministic for the same `comparisonId`, before bundle, and
after bundle.

## Compared dimensions

The model compares:

- flags;
- events;
- screenshot refs;
- console diagnostics;
- frame stats;
- replay result;
- known gaps.

Screenshot differences are linked through before/after artifact refs even when
stable pixel diff is unavailable. Regression dimensions dominate the summary
verdict; otherwise improvements dominate; otherwise unresolved inconclusive
dimensions produce `inconclusive`; otherwise `no-change`.

## Report outputs

The JSON artifact records the full structured comparison. The Markdown renderer
includes:

- verdict, before run, after run, and determinism key;
- per-dimension verdict table;
- before/after artifact refs;
- known gaps;
- forbidden actions.

All refs must be local run/path/digest references. The renderer rejects escaping
paths and authority-expanding text.

## Boundary

This report is data-only. It does not run a browser, execute commands, compute
unstable pixel diffs, mutate source, auto-apply, auto-merge, publish, deploy,
install dependencies, or write trusted files.
