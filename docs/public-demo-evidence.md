# Public demo media and smoke evidence

Date: 2026-06-02
Branch: `issue-49-demo-fresh-clone-evidence`; refreshed on `al2-8-1-public-readiness-evidence`
Status: pre-release MVP evidence for public-readiness review; this is not a launch announcement.

## Demo media

The screenshots below were refreshed on 2026-06-02 from the static local MVP surfaces with Chrome headless and a local `python3 -m http.server` process. They are committed as public demo references, not generated run evidence. The refreshed dashboard and cockpit captures use the AL2.8.1 generated dashboard export, including Platformer and Engine Expansion v1 smoke runs.

| Surface | Screenshot | Scope boundary |
| --- | --- | --- |
| Minimal 2D runtime | [`docs/assets/demo/runtime-demo.png`](assets/demo/runtime-demo.png) | Canvas/runtime demo only; no engine compatibility promise. |
| Evidence dashboard | [`docs/assets/demo/evidence-dashboard.png`](assets/demo/evidence-dashboard.png) | Read-only inspection of exported run data. |
| Authoring cockpit | [`docs/assets/demo/authoring-cockpit.png`](assets/demo/authoring-cockpit.png) | Static Studio v2 cockpit prototype; browser does not write files directly or execute commands. |

Capture commands used locally:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- run seeds/engine-expansion-v1-demo.yaml --workers 4
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

AL2.8.2 refresh wrote current image files for all three surfaces. Chrome headless emitted local updater/crashpad noise during capture; the tracked PNG files were still written and visually checked.

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

## Fresh-clone / clean-worktree smoke evidence

Original fresh-clone verification for issue #49 was run from `/tmp/ouroforge-fresh-49` after pushing that branch. Authoring Loop v2 public-readiness refresh was re-run from a clean latest-`main` worktree on 2026-06-02. This refresh adds the Engine Expansion v1 demo smoke path required after the current milestone.

```text
gh issue view 217 --repo shaun0927/Ouroforge
# passed: #217 OPEN, Public Readiness Refresh after Engine Expansion v1
gh issue view 1 --repo shaun0927/Ouroforge
# passed: #1 OPEN, Define Ouroforge final goal and evidence-native implementation roadmap
cargo fmt --check
# passed
cargo test
# passed: ouroforge-cli integration tests 5/5; ouroforge-core tests 143/143; doc-tests 0/0
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
# passed: status "passed"; run directory runs/run-1780406739942-16401; browser_smoke succeeded 4/4; scenarios passed 2/2
cargo run -p ouroforge-cli -- run seeds/engine-expansion-v1-demo.yaml --workers 4
# passed: status "passed"; run directory runs/run-1780406747216-16614; browser_smoke succeeded 4/4; scenarios passed 2/2
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
# passed: Loaded 1102 security advisories; scanned Cargo.lock for vulnerabilities (66 crate dependencies); exit code 0; no vulnerabilities reported.
```

`cargo audit` was available locally; this did not add a repository dependency.

## Known limitations

- Demo media are screenshots, not a polished launch trailer.
- Public visibility remains a separate manual decision.
- The dashboard screenshot depends on locally exported `dashboard-data.json`; that generated file is intentionally not committed.
- The cockpit is a static prototype that displays Rust-validated commands, transaction provenance, semantic comparison summaries, and scene-only mutation lifecycle state; it does not directly write files or execute commands from the browser.
