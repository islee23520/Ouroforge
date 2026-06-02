# Studio v2 Cockpit Integration Evidence

Studio v2 is the local, evidence-backed authoring cockpit for Authoring Loop v2.
It composes existing Rust-authored evidence and generated dashboard data into one
static browser surface; it does not execute commands, write trusted files, or
apply/review mutations from JavaScript.

## Integrated surfaces

The cockpit at `examples/authoring-cockpit/` now shows:

- scene editing command generation for Rust-validated scalar edits;
- authoring provenance from scene edit transaction metadata;
- QA run and dashboard export commands;
- run/evidence and journal read models;
- Engine Expansion state summaries;
- semantic run comparison summaries;
- scene-only mutation proposal/application lifecycle state;
- manual scene mutation apply, validation, dashboard export, and review command
  strings;
- replay and live preview probe controls that remain ephemeral browser state.

## Manual authoring loop

A safe local authoring loop remains explicitly manual:

1. Use the cockpit only to inspect scene state and copy Rust CLI commands.
2. Persist scene edits through `ouroforge-cli scene edit --transaction-output`.
3. Bind QA runs with `ouroforge-cli run ... --transaction <transaction.json>`.
4. Export dashboard data with `ouroforge-cli dashboard export`.
5. Compare runs with `ouroforge-cli compare`.
6. For scene-only mutations, apply only validated scene operations through
   `ouroforge-cli mutation apply-scene` and review results manually.

## Guardrails

Studio v2 remains inside the Authoring Loop v2 boundary:

- no browser-side command execution;
- no direct browser writes to trusted files;
- no server, cloud, database, auth, plugin runtime, or native export path;
- no production editor or Godot-replacement claim;
- no browser-side mutation acceptance/rejection/application;
- generated `runs/` and `examples/evidence-dashboard/dashboard-data.json` stay
  untracked local artifacts.

## Verification evidence

Last refreshed for Authoring Loop v2 AL2.7.4 on 2026-06-02.

Commands run from the repository root:

```bash
cargo fmt --check
cargo test
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
cargo clippy --all-targets --all-features -- -D warnings
```

Observed evidence:

- QA run: `runs/run-1780406582240-7959`
- Browser smoke: 4 workers succeeded, 0 failed.
- Scenario suite: 2 passed, 0 failed.
- Verdict: `passed`.
- Dashboard data export succeeded to
  `examples/evidence-dashboard/dashboard-data.json`.
- Dashboard smoke test passed.
- Authoring cockpit smoke test passed.
- Clippy completed with `-D warnings`.

The QA run directory and dashboard-data export are generated evidence and are
not tracked by git.

## Known gaps

The cockpit still intentionally does not provide:

- a production editor UX;
- hosted collaboration or remote execution;
- native shell or command bridge;
- browser-owned persistence;
- browser-side semantic diff computation;
- automatic scene mutation application, PR merge, or mutation review;
- source-code mutation.
