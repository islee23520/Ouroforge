# Evidence-Backed Journal v2

Evidence-Backed Journal v2 extends `journal.md` with a deterministic authoring
governance lifecycle section. The journal remains a local artifact generated from
trusted run files; it is not a freeform AI narrative, hosted audit service, or
browser-owned mutation surface.

## Lifecycle section

`update_journal` appends an `Authoring Governance Lifecycle` section with schema
marker:

```text
journal-authoring-governance-v2
```

The section summarizes these local records when available:

1. proposal quality and rationale from `mutation/proposals.json`;
2. review decisions from `mutation/review-decisions.json`;
3. review-gated scene applications from `mutation/scene-applications.json`;
4. rerun/compare artifacts from `mutation/run-comparison-*.json` or
   `comparisons/run-comparison-*.json`;
5. regression promotion records from `regression-promotions/*.json`.

Each subsection uses ids and evidence paths already present in run artifacts. It
does not invent missing context or infer a pass/fail decision from absent files.

## Missing, partial, and malformed data

Journal v2 is intentionally explicit about gaps:

- no proposals -> `No mutation proposals recorded`;
- proposal without rationale -> `Rationale: missing`;
- no review decisions -> accepted/rejected/deferred status is unavailable;
- no scene applications -> no review-gated application is recorded;
- malformed scene application index -> the parse error is shown as a lifecycle
  evidence quality gap;
- missing comparison artifacts -> the comparison empty state is shown;
- malformed comparison artifacts -> read errors are shown;
- no regression promotions -> no promotion status is recorded.

Older runs remain compatible because every new lifecycle subsection has a
missing/empty state. Existing `journal show` and dashboard journal read models
continue to work with legacy `journal.md` files.

## Boundaries

Journal v2 must remain deterministic and evidence-backed.

It must not add:

- LLM/network summarization;
- hidden mutation application;
- browser writes;
- command bridges;
- hosted audit storage;
- source-code mutation;
- auto-merge or auto-promotion.

Browser surfaces may display exported journal headings, snippets, and evidence
links as escaped read-only content only.

## Generated-state policy

Journals are generated run artifacts under `runs/<run-id>/journal.md`. Do not
commit generated run directories or generated dashboard exports unless a later
fixture-scoped issue explicitly authorizes that fixture.

Keep these local/generated paths untracked:

- `runs/`
- `target/`
- `examples/evidence-dashboard/dashboard-data.json`
- project-local `dashboard-data/`

## Verification

Focused checks:

```bash
cargo test journal
cargo test mutation
cargo test regression
cargo run -p ouroforge-cli -- journal show <run-dir>
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
```

Issue-level closure should additionally run `cargo fmt --check`, full
`cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings` on
latest `main`.
