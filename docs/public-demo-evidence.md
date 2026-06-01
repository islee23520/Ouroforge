# Public demo media and smoke evidence

Date: 2026-06-02
Branch: `issue-49-demo-fresh-clone-evidence`
Status: pre-release MVP evidence for public-readiness review; this is not a launch announcement.

## Demo media

The screenshots below were captured from the static local MVP surfaces with Chrome headless and a local `python3 -m http.server` process. They are committed as public demo references, not generated run evidence.

| Surface | Screenshot | Scope boundary |
| --- | --- | --- |
| Minimal 2D runtime | [`docs/assets/demo/runtime-demo.png`](assets/demo/runtime-demo.png) | Canvas/runtime demo only; no engine compatibility promise. |
| Evidence dashboard | [`docs/assets/demo/evidence-dashboard.png`](assets/demo/evidence-dashboard.png) | Read-only inspection of exported run data. |
| Authoring cockpit | [`docs/assets/demo/authoring-cockpit.png`](assets/demo/authoring-cockpit.png) | Static cockpit prototype; browser does not write files directly. |

Capture commands used locally:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
python3 -m http.server 8765 --bind 127.0.0.1 --directory .

CHROME=${OUROFORGE_CHROME:-"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"}
"$CHROME" --headless=new --disable-gpu --user-data-dir=/tmp/ouroforge-shot-runtime \
  --screenshot=docs/assets/demo/runtime-demo.png --window-size=1280,720 \
  http://127.0.0.1:8765/examples/game-runtime/index.html
"$CHROME" --headless=new --disable-gpu --user-data-dir=/tmp/ouroforge-shot-dashboard \
  --screenshot=docs/assets/demo/evidence-dashboard.png --window-size=1440,1000 \
  http://127.0.0.1:8765/examples/evidence-dashboard/index.html
"$CHROME" --headless=new --disable-gpu --user-data-dir=/tmp/ouroforge-shot-cockpit \
  --screenshot=docs/assets/demo/authoring-cockpit.png --window-size=1440,1000 \
  http://127.0.0.1:8765/examples/authoring-cockpit/index.html
```

## Chrome requirement

The MVP browser capture path requires a local Chrome or Chromium executable. Use the platform default path when available, or set:

```bash
export OUROFORGE_CHROME=/path/to/chrome-or-chromium
```

No Playwright, browser cloud, hosted service, database, account system, or external runtime is required.

## Expected generated artifacts

The MVP smoke path generates local state that must stay out of git:

- `runs/run-*/run.json`
- `runs/run-*/ledger.jsonl`
- `runs/run-*/evidence/**`
- `runs/run-*/verdict.json`
- `runs/run-*/journal.jsonl`
- `runs/run-*/mutations/*.json`
- `examples/evidence-dashboard/dashboard-data.json`
- local runtime/tool folders such as `.openchrome/` and `.omc/`

These artifacts are evidence for a local run, not source files.

## Fresh-clone smoke evidence

Fresh-clone verification for this branch was run from `/tmp/ouroforge-fresh-49` after pushing the branch.

```text
cargo fmt --check
# passed
cargo test
# passed: ouroforge-cli test 1/1; ouroforge-core tests 49/49; doc-tests 0/0
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
# passed: status "passed"; run directory runs/run-1780329066179-52042; browser_smoke succeeded 4/4; scenarios passed 1/1
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
# passed: Dashboard data exported
node --check examples/evidence-dashboard/dashboard.js
# passed
node examples/evidence-dashboard/dashboard.test.cjs
# dashboard smoke test passed
node --check examples/authoring-cockpit/cockpit.js
# passed
node examples/authoring-cockpit/cockpit.test.cjs
# authoring cockpit smoke test passed
cargo clippy --all-targets --all-features -- -D warnings
# passed
```

## Dependency and security audit evidence

```text
cargo audit
# passed in fresh clone: Loaded 1100 security advisories; scanned Cargo.lock for vulnerabilities (66 crate dependencies); exit code 0; no vulnerabilities reported.
```

`cargo audit` was installed locally with `cargo install cargo-audit --locked`; this did not add a repository dependency.

## Known limitations

- Demo media are screenshots, not a polished launch trailer.
- Public visibility remains a separate manual decision.
- The dashboard screenshot depends on locally exported `dashboard-data.json`; that generated file is intentionally not committed.
- The cockpit is a static prototype that displays Rust-validated commands; it does not directly write files from the browser.
