# Fresh Clone Troubleshooting and Cleanup v1

Status: **PA1.2.3 troubleshooting and cleanup docs** for issue #368.

This guide helps a fresh-clone reader diagnose local onboarding failures and
clean generated state after running the quickstart or fresh-clone smoke. It is
documentation-only. It does not install dependencies, change repository
visibility, publish artifacts, add release automation, apply source patches, run
browser trusted writes, or create a command bridge.

Use it with:

- [`fresh-clone-onboarding-command-audit-v1.md`](fresh-clone-onboarding-command-audit-v1.md)
- [`fresh-clone-smoke-v1.md`](fresh-clone-smoke-v1.md)
- [`artifact-write-policy-v1.md`](artifact-write-policy-v1.md)

## Prerequisite checks

Run these checks before the quickstart when a fresh clone behaves differently
from the documented path:

```bash
rustc --version
cargo --version
node --version
python3 --version
```

Expected result: each command prints a version and exits successfully. Ouroforge
does not provide dependency installation automation; use your operating system or
language-toolchain documentation to install or update missing tools.

Chrome/Chromium is required for browser/scenario evidence. If the runtime cannot
find Chrome at a standard platform path, point Ouroforge at an executable:

```bash
OUROFORGE_CHROME=/path/to/chrome scripts/fresh-clone-smoke.sh --keep
```

Use the same environment variable with direct `cargo run -p ouroforge-cli -- run
...` commands. Keep the path local to your machine; do not commit private local
paths into docs, issues, fixtures, or generated evidence comments.

## Common failures

| Symptom | Likely cause | Safe next check |
| --- | --- | --- |
| `cargo` or `rustc` is not found | Rust toolchain is not installed or not on `PATH`. | Run `rustc --version` and `cargo --version` in the same shell used for the smoke. |
| `node` is not found | Node.js is missing or not on `PATH`. | Run `node --version`; Node is needed for dashboard/cockpit syntax and smoke checks. |
| `python3` is not found | Python 3 is missing or named differently on the host. | Run `python3 --version`; the smoke wrapper uses Python for path normalization. |
| Browser/scenario evidence is missing or the run fails before browser evidence | Chrome/Chromium is missing, blocked, or not discoverable. | Set `OUROFORGE_CHROME=/path/to/chrome` and rerun the smoke. |
| `OUROFORGE_FRESH_CLONE_WORKERS` or `OUROFORGE_DEMO_WORKERS` is rejected | Worker count is not a positive integer or is below the evidence-generating minimum. | Use `2` or higher for smoke wrappers that require browser/scenario evidence. |
| `--work-dir` is rejected | The directory is inside the repository, is not a directory, or is already populated. | Use an absent or empty path under `/tmp` or another disposable local directory. |
| Scenario verdicts are `failed` in generated evidence | The evaluator records actual scenario outcomes; failed verdicts can be honest evidence. | Inspect the generated run directory and do not rewrite failures as public success claims. |
| Dashboard data is missing | No run evidence exists at the selected `--runs-root`, or export wrote somewhere else. | Check the smoke summary for `dashboard_data=` and the run count. |
| `git status --short --ignored` shows local roots | A run, smoke, Cargo build, or local tool created ignored generated state. | Confirm paths are ignored/local and use the cleanup guidance below if disposable. |

## Generated-state policy

Fresh-clone onboarding commands may create local evidence and build output. These
paths are generated/local by default and should remain untracked unless a future
issue explicitly scopes a tiny deterministic fixture:

- `runs/`
- `target/`
- `.omx/`
- `.omc/`
- `.openchrome/`
- `.claude/`
- `examples/evidence-dashboard/dashboard-data.json`
- `/tmp/ouroforge-fresh-clone-smoke-*`
- `/tmp/ouroforge-canonical-demo-*`
- `/tmp/ouroforge-demo-browser-*`
- source patch preview or sandbox worktrees under documented generated roots

Use this audit after a smoke or quickstart run:

```bash
git status --short --ignored
```

Expected generated/local entries may appear with `!!`. The fresh-clone smoke
snapshots these generated-state roots before and after execution and fails if
their ignored/tracked status changes. Tracked source changes or unignored
generated files should be investigated before opening a PR.

## Cleanup commands

Only remove generated state that you know is disposable. Do not run cleanup
against maintainer evidence, issue evidence, or any path you still need for a PR,
review, or issue comment.

From the repository root, after confirming the paths are disposable:

```bash
rm -rf runs target
rm -f examples/evidence-dashboard/dashboard-data.json
rm -rf .omx/tmp/project-scaffold-smoke
```

For smoke wrappers that used the default temporary directories:

```bash
rm -rf /tmp/ouroforge-fresh-clone-smoke-* \
  /tmp/ouroforge-canonical-demo-* \
  /tmp/ouroforge-demo-browser-*
```

For an explicit `--work-dir`, remove only the exact path you supplied after
reviewing it:

```bash
rm -rf /tmp/ouroforge-fresh-clone-smoke
```

The fresh-clone smoke refuses to own or clean a pre-existing populated
`--work-dir` so it does not recursively delete unrelated local files.

## Quick re-run sequence

After cleanup, a bounded re-run for onboarding evidence is:

```bash
OUROFORGE_FRESH_CLONE_WORKERS=2 scripts/fresh-clone-smoke.sh --keep
```

For a stable evidence path:

```bash
rm -rf /tmp/ouroforge-fresh-clone-smoke
OUROFORGE_FRESH_CLONE_WORKERS=2 \
  scripts/fresh-clone-smoke.sh \
  --work-dir /tmp/ouroforge-fresh-clone-smoke --keep
```

If Chrome is not discoverable:

```bash
OUROFORGE_CHROME=/path/to/chrome \
  OUROFORGE_FRESH_CLONE_WORKERS=2 \
  scripts/fresh-clone-smoke.sh --keep
```

## Guardrails

Troubleshooting and cleanup remain inside Public Alpha Readiness v1 /
Open-Source Preparation:

- no repository visibility change;
- no package, binary, crates.io, npm, release, or launch automation;
- no dependency installation workflow;
- no hosted/cloud/server/auth behavior;
- no native export, plugin runtime, marketplace, source patch apply, browser
  trusted write, command bridge, auto-merge, or auto-apply;
- no claim that Ouroforge is production-ready, compatibility-stable, a Godot
  replacement, or a secure sandbox.

## Verification checklist for docs changes

For PRs touching this guide, record:

```bash
gh issue view 368 --repo shaun0927/Ouroforge --json number,state,title
gh issue view 1 --repo shaun0927/Ouroforge --json number,state,title
gh issue view 23 --repo shaun0927/Ouroforge --json number,state,title
git diff --check
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo audit
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git status --short --ignored
```

Wording-scan matches are acceptable only when they are conservative boundary
statements, explicit negations, non-goals, or wording-audit examples.
