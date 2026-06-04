# Canonical Demo Readiness Evidence v1

Status: **PA1.3.3 final demo docs, cleanup, and readiness evidence** for
issue #369.

This note records the conservative readiness boundary for the canonical local
demo after the PA1.3.1 flow contract and PA1.3.2 smoke wrapper. It is not a
launch announcement, release checklist approval, production-support promise, or
source-apply authorization.

## Merged PR unit evidence

| Unit | Evidence | Boundary preserved |
| --- | --- | --- |
| PA1.3.1 — Demo flow contract and command sequence | PR #951 merged with `docs/canonical-demo-script-v1.md` and the docs index link. | Documented the canonical command sequence only; no destructive automation. |
| PA1.3.2 — Non-destructive demo script/smoke | PR #953 merged with `scripts/canonical-demo-smoke.sh` and updated demo docs. | The wrapper runs local checks, moves generated run output into a local work directory, and does not apply, publish, merge, change visibility, or write trusted browser state. |
| PA1.3.3 — Demo docs and cleanup/readiness evidence | This document and docs index updates. | Records outputs, cleanup, failure modes, and final verification without committing generated state. |

## Canonical local demo command

Run from a fresh clone or clean worktree:

```bash
OUROFORGE_DEMO_WORKERS=2 \
  CARGO_TARGET_DIR=/tmp/ouroforge-canonical-demo-target \
  scripts/canonical-demo-smoke.sh --keep
```

Expected successful summary fields:

- `canonical-demo-smoke=passed`
- `work_dir=/tmp/ouroforge-canonical-demo-*` or the platform temporary root
- `before_run=<work_dir>/runs/run-*`
- `after_run=<work_dir>/runs/run-*`
- `dashboard_data=<work_dir>/dashboard-data.json`
- `comparison_output=<work_dir>/runs/comparisons`
- `non_goals=not applied, not published, not merged, no visibility change, no browser trusted writes`

The wrapper may report pending or failed scenario verdicts honestly. Readiness is
based on the command path preserving evidence and trust boundaries, not on
rewriting demo verdicts into passing evidence.

## Generated output and cleanup policy

Generated demo output remains local and untracked:

- `runs/` and `runs/comparisons/` when manual commands are run directly;
- the smoke wrapper's `/tmp/ouroforge-canonical-demo-*` work directory;
- `examples/evidence-dashboard/dashboard-data.json` when manual dashboard export
  writes to the repository demo path;
- local browser profiles under `/tmp/ouroforge-demo-browser-*`;
- local logs, screenshots, command transcripts, and `target/`.

Cleanup for manual command runs:

```bash
rm -rf runs
rm -f examples/evidence-dashboard/dashboard-data.json
rm -rf /tmp/ouroforge-canonical-demo-* /tmp/ouroforge-demo-browser-*
```

If the smoke wrapper is run without `--keep`, it removes its generated work
directory automatically. If `--keep` is used, remove the printed work directory
manually after inspection.

## Failure modes and operator response

| Failure mode | Expected response |
| --- | --- |
| Missing Chrome/Chromium or browser smoke failure | Keep the generated work directory, inspect run logs and evidence, and report the failed verdict as current evidence. Do not claim browser coverage passed. |
| Pending or failed run verdict | Continue only through read-only dashboard, compare, review-defer, preview, and validation steps; do not rewrite verdicts. |
| Missing mutation patch draft | Record the skipped deferred review as a non-failure; no apply step is introduced. |
| Generated-state audit failure | Stop and remove only generated local paths; do not commit generated runs, dashboard data, browser profiles, or screenshots. |
| Wording scan finds an overclaim | Rewrite public wording to a conservative boundary statement before issue closure. |

## Final issue verification checklist

Before issue #369 is closed, run and record current evidence for:

```bash
gh issue view 369 --repo shaun0927/Ouroforge --json state,title
gh issue view 1 --repo shaun0927/Ouroforge --json state,title
gh issue view 23 --repo shaun0927/Ouroforge --json state,title
bash -n scripts/canonical-demo-smoke.sh
OUROFORGE_DEMO_WORKERS=2 CARGO_TARGET_DIR=/tmp/ouroforge-canonical-demo-target scripts/canonical-demo-smoke.sh --keep
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo audit
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
grep -RInE "Godot replacement|Godot parity|production-ready|production ready|commercial-release ready|ship-ready|compatibility-stable|stable public engine API|secure sandbox|sandbox guarantee|source apply ready|auto-apply|auto-merge|autonomous repair|browser trusted write|command bridge|local server bridge|native export ready|desktop/mobile export|installer|app-store ready|plugin runtime ready|extension marketplace|third-party code loading|hosted service|cloud runtime|multi-user auth|autonomous launch|public release automation|go-live automation|support SLA|guaranteed support|security guarantee" README.md CONTRIBUTING.md SECURITY.md docs examples .github || true
git diff --check
git status --short --ignored
```

Allowed wording-scan matches are conservative denials, explicit non-goals,
audit examples, or trust-boundary statements. Generated state must remain ignored
or absent from tracked changes.

## Closure boundary

Closing issue #369 only means the canonical local demo command sequence, smoke
wrapper, cleanup notes, and readiness evidence are documented and merged. It does
not mean Ouroforge is publicly launched, production-ready, compatibility-stable,
a secure sandbox, a Godot replacement, a hosted service, or authorized to apply
source patches without separate review.
