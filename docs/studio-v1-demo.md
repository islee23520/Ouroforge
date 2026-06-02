# Studio / Authoring UI v1 Demo Evidence

Issue: #91 — Studio v1 Integration Demo

This document records the local static Studio v1 demo composition. It is a demo
evidence note, not a production-editor claim.

## Demo boundary

The Studio v1 demo composes completed local surfaces into the static authoring
cockpit:

- run/evidence browser;
- journal viewer;
- mutation review state and manual command hints;
- replay evidence surface;
- live preview controls through the existing `window.__OUROFORGE__` probe;
- scene edit command generation for Rust-validated scene edits;
- run comparison artifact surface;
- known-gap summary.

The browser remains inspect-only. It does not write files directly, persist scene
edits, accept or reject mutations, apply patches, compute run comparisons, start a
server, add a database, or claim full editor maturity. Scene persistence remains
through explicit Rust CLI commands such as `cargo run -p ouroforge-cli -- scene
edit ...`.

## Demo commands

From the repository root:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

Then open:

- <http://127.0.0.1:8000/examples/authoring-cockpit/>
- <http://127.0.0.1:8000/examples/evidence-dashboard/>

## Verification evidence

ST9.2 verification on `91-2-studio-demo-evidence-docs` started from main commit
`6160842330e35375c986c3b94023bf76707b95b4` and produced this run:

- Run ID: `run-1780383540793-20338`
- Run path: `runs/run-1780383540793-20338`
- Result: `status: passed`
- Workers: `4` succeeded
- Scenarios: `1` passed

Commands and observed results:

```text
$ cargo fmt --check
passed

$ cargo test
113 core tests passed; CLI integration test passed; doc tests passed

$ cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
Run created: runs/run-1780383540793-20338
status: passed; workers: 4 succeeded; scenarios: 1 passed

$ cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
Dashboard data exported: examples/evidence-dashboard/dashboard-data.json

$ node --check examples/evidence-dashboard/dashboard.js
passed

$ node examples/evidence-dashboard/dashboard.test.cjs
dashboard smoke test passed

$ node --check examples/authoring-cockpit/cockpit.js
passed

$ node examples/authoring-cockpit/cockpit.test.cjs
authoring cockpit smoke test passed

$ cargo clippy --all-targets --all-features -- -D warnings
passed

$ git check-ignore examples/evidence-dashboard/dashboard-data.json
examples/evidence-dashboard/dashboard-data.json

$ git status --short --untracked-files=no
(no tracked changes)
```

## Surface checklist

- [x] Static cockpit can load generated dashboard data.
- [x] Run/evidence browser is reachable.
- [x] Journal viewer is reachable when journal data exists.
- [x] Mutation review state and manual command hints are visible when lifecycle
      artifacts exist.
- [x] Replay evidence surface is visible when replay artifacts exist.
- [x] Live preview controls are available through the runtime probe only.
- [x] Scene edit command generation remains Rust-validation oriented.
- [x] Run comparison surface displays existing comparison artifacts when present.
- [x] Known gaps are visible in the demo.

## Known gaps and omitted features

These omissions are intentional for Studio v1:

- no production editor;
- no native app shell;
- no hosted studio;
- no collaborative editing;
- no plugin or marketplace UI;
- no visual scripting;
- no direct browser file writes;
- no browser-side comparison algorithms;
- no mutation acceptance/rejection or patch application from browser UI;
- no broad engine compatibility or Godot-replacement claim.

## Generated data policy

Generated local state remains uncommitted:

- `runs/`
- `target/`
- `examples/evidence-dashboard/dashboard-data.json`
- `.openchrome/`
- `.omc/`
- `.omx/`

Use `git check-ignore examples/evidence-dashboard/dashboard-data.json` and
`git status --short --untracked-files=no` before opening PRs to prove generated
data is ignored and tracked files are clean.
