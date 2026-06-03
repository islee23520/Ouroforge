# Studio Review Cockpit v1

Studio Review Cockpit v1 extends the static authoring cockpit with read-only
review governance panels for Agentic Review & Regression Promotion v1.

The cockpit consumes `dashboard-data.json` exported by the Rust CLI. It does not
own trusted state and must not write files, execute local commands, apply
mutations, promote regressions, rerun scenarios, merge changes, or act as a
browser command bridge.

## Exported data

The cockpit reads these existing/exported fields:

- `run.review_cockpit` — normalized lifecycle summary for proposal, review,
  application, comparison, promotion, and matrix stages;
- `run.mutation_lifecycle` — detailed proposal/review/application records;
- `run.regression_promotions` — promotion records written by the Rust CLI;
- top-level `regression_matrix`, copied onto the selected run by the static
  loader for display;
- `run.journal_view` — generated journal snippets and evidence refs.

Missing or malformed fields render as empty/warning states. The browser must
escape all exported text before rendering.

## Surfaces

The cockpit displays:

1. proposal rationale cards with evidence ids and expected effect;
2. review decision cards with reviewer, state, reason, and evidence refs;
3. review-gated scene application cards with transaction/project/rollback
   provenance;
4. regression promotion status cards and display-only dry-run command text;
5. regression matrix cards with current status, last pass/fail, and linked
   mutation/review/promotion context counts;
6. inert copyable CLI commands as escaped text.

## Boundaries

Allowed:

- display exported lifecycle state;
- render evidence links through the existing static artifact path policy;
- show copyable command strings as inert text;
- show missing/malformed data explicitly.

Not allowed:

- browser file writes;
- `fetch`/WebSocket/local shell command bridges for trusted writes;
- auto-rerun, auto-apply, auto-promote, or auto-merge;
- hosted audit storage or production editor claims;
- hiding missing evidence behind optimistic status.

## Verification evidence

AR1.8.4 smoke evidence on latest main should run:

```bash
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Generated dashboard exports and run directories remain untracked. For issue
closure, also run `cargo fmt --check`, `cargo test`, and
`cargo clippy --all-targets --all-features -- -D warnings` on latest `main`.
