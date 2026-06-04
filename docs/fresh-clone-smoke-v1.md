# Fresh Clone Smoke v1

Status: **PA1.2.2 fresh-clone smoke script/docs** for issue #368.

This document defines the non-destructive fresh-clone-style onboarding smoke for
Ouroforge public-alpha preparation. It adds a local script and reproducible
command sequence only; it does not install dependencies, publish artifacts,
change repository visibility, apply source patches, run browser trusted writes,
or track generated run/dashboard output.

PA1.2.1 owns the onboarding command audit in
[`fresh-clone-onboarding-command-audit-v1.md`](fresh-clone-onboarding-command-audit-v1.md).
Expanded troubleshooting, prerequisite checks, and cleanup guidance are recorded
in [`fresh-clone-troubleshooting-cleanup-v1.md`](fresh-clone-troubleshooting-cleanup-v1.md).

## Prerequisites

Install these before running the smoke:

- Rust + Cargo;
- Node.js;
- Python 3;
- Chrome/Chromium available from a standard platform path, or
  `OUROFORGE_CHROME=/path/to/chrome`.

The smoke intentionally does not install tools or manage system packages.

## Smoke wrapper

Run from the repository root:

```bash
scripts/fresh-clone-smoke.sh --keep
```

The wrapper creates an isolated temporary work directory, copies tracked
repository files into a clone-like directory, and runs the onboarding checks from
that isolated copy. By default it removes the work directory on success or
failure. With `--keep`, it prints the directory for inspection.

Use an explicit empty work directory when you want stable evidence paths:

```bash
OUROFORGE_FRESH_CLONE_WORKERS=2 \
  scripts/fresh-clone-smoke.sh \
  --work-dir /tmp/ouroforge-fresh-clone-smoke --keep
```

`OUROFORGE_FRESH_CLONE_WORKERS` must be at least `2` so browser/scenario evidence
is generated. `CARGO_TARGET_DIR` may be set by the caller for an external build
cache, but it must stay outside the repository; otherwise the script uses
`<work-dir>/target` so build output stays inside the smoke work directory. The
wrapper snapshots the maintainer worktree generated-state roots before and after
the smoke and fails if their ignored/tracked status changes.

## Commands exercised

The wrapper runs this fresh-clone-oriented sequence inside the isolated copy:

```bash
cargo fmt --check
cargo run -p ouroforge-cli -- seed validate seeds/platformer.yaml
cargo run -p ouroforge-cli -- project validate examples/project-workspace-fixtures/valid
cargo run -p ouroforge-cli -- project init <work-dir>/generated/project-scaffold-smoke --template minimal-2d
cargo run -p ouroforge-cli -- run <work-dir>/generated/project-scaffold-smoke/seeds/platformer.yaml \
  --project <work-dir>/generated/project-scaffold-smoke --scenario-pack smoke --workers 2
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 2
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output <work-dir>/generated/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

It then audits the maintainer worktree generated-state roots to confirm the
smoke did not change their ignored/tracked status.

## Expected generated output

All smoke output stays under the isolated work directory:

- `<work-dir>/repo/` — clone-like copy of tracked repository files;
- `<work-dir>/target/` — Cargo build output, unless `CARGO_TARGET_DIR` is set;
- `<work-dir>/repo/runs/run-*` — generated run evidence from the isolated copy;
- `<work-dir>/generated/project-scaffold-smoke/` — generated minimal project;
- `<work-dir>/generated/dashboard-data.json` — generated dashboard data;
- `<work-dir>/generated/*.log` and `*.status` — command evidence;
- `<work-dir>/generated/fresh-clone-smoke-summary.txt` — smoke summary.

These outputs are local/generated evidence. They should remain untracked unless a
future issue explicitly scopes a tiny deterministic fixture.

## Guardrails

The fresh-clone smoke stays inside Public Alpha Readiness v1 / Open-Source
Preparation:

- no repository visibility change;
- no package, binary, crates.io, npm, release, or launch automation;
- no dependency installation workflow;
- no hosted/cloud/server/auth behavior;
- no native export, plugin runtime, marketplace, source patch apply, browser
  trusted write, command bridge, auto-merge, or auto-apply;
- no claim that Ouroforge is production-ready, compatibility-stable, a Godot
  replacement, or a secure sandbox.
