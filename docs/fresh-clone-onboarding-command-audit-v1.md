# Fresh Clone Onboarding Command Audit v1

Status: **PA1.2.1 onboarding command audit** for issue #368.

This audit checks that the current public quickstart commands remain runnable and
conservative for a fresh-clone reader. It does not add product behavior, release
automation, repository visibility changes, dependency installation automation,
source apply authority, browser trusted writes, or generated artifact tracking.
PA1.2.2 owns the fresh-clone smoke script/docs unit, and PA1.2.3 owns expanded
troubleshooting and cleanup docs.

## Scope audited

| Surface | Result | Notes |
| --- | --- | --- |
| Top-level README quickstart | Current commands are still runnable locally. | Validation, project scaffold, project-bound run, and generated-state cleanup commands were exercised. |
| Canonical demo smoke wrapper | Current wrapper reaches `canonical-demo-smoke=passed` with `OUROFORGE_DEMO_WORKERS=2`. | Scenario verdicts may be `failed`; the wrapper records them honestly rather than claiming scenario success. |
| Read-only dashboard/cockpit checks | Node syntax and smoke tests pass through the wrapper and remain read-only. | Browser surfaces render exported/local evidence and do not write files or run commands. |
| Generated state | Local artifacts remain ignored/untracked. | `runs/` and `target/` may appear in `git status --short --ignored`; that is expected generated state, not source. |
| Protected governance anchors | #1 and #23 remained open during PA1.2.1. | These anchors are not modified by this issue. |

## Verified command sequence

Live preflight before editing:

```bash
gh issue view 368 --repo shaun0927/Ouroforge --json number,state,title
gh issue view 1 --repo shaun0927/Ouroforge --json number,state,title
gh issue view 23 --repo shaun0927/Ouroforge --json number,state,title
```

Focused README quickstart audit:

```bash
cargo run -p ouroforge-cli -- seed validate seeds/platformer.yaml
cargo run -p ouroforge-cli -- project validate examples/project-workspace-fixtures/valid
cargo run -p ouroforge-cli -- project init .omx/tmp/project-scaffold-smoke --template minimal-2d
cargo run -p ouroforge-cli -- run .omx/tmp/project-scaffold-smoke/seeds/platformer.yaml \
  --project .omx/tmp/project-scaffold-smoke --scenario-pack smoke --workers 1
rm -rf .omx/tmp/project-scaffold-smoke
```

Canonical local demo smoke audit, using an external work directory and target dir:

```bash
OUROFORGE_DEMO_WORKERS=2 \
  CARGO_TARGET_DIR=/tmp/ouroforge-368-pa121-smoke-target \
  scripts/canonical-demo-smoke.sh \
  --work-dir /tmp/ouroforge-368-pa121-canonical-smoke --keep
```

Observed smoke summary included:

```text
canonical-demo-smoke=passed
work_dir=/tmp/ouroforge-368-pa121-canonical-smoke
workers=2
non_goals=not applied, not published, not merged, no visibility change, no browser trusted writes
```

## Quickstart interpretation for fresh-clone readers

- Install Rust + Cargo, Node.js, Python 3, and Chrome/Chromium, or set
  `OUROFORGE_CHROME=/path/to/chrome` when Chrome is not on a standard path.
- The local run and smoke commands create generated evidence. Keep it local and
  untracked unless a future issue explicitly scopes a deterministic fixture.
- `runs/`, `target/`, `.omx/`, and `examples/evidence-dashboard/dashboard-data.json`
  are generated/local paths. Seeing them as ignored entries after a run is
  expected.
- The canonical demo smoke requires `OUROFORGE_DEMO_WORKERS=2` or higher so it
  records browser/scenario evidence instead of only pending shell state.
- Failed scenario verdicts in generated evidence are allowed audit evidence; do
  not rewrite them as public success claims.

## Cleanup after local audit

For a disposable audit worktree, cleanup may remove generated state with:

```bash
rm -rf runs target .omx/tmp/project-scaffold-smoke \
  examples/evidence-dashboard/dashboard-data.json
```

Do not run cleanup commands against unrelated maintainer evidence directories
unless they are known to be disposable. PA1.2.3 will expand the troubleshooting
and cleanup guidance for contributors.

## Wording and guardrail audit

The audited quickstart remains inside Public Alpha Readiness v1 / Open-Source
Preparation:

- no repository visibility change;
- no package, binary, crates.io, npm, release, or launch automation;
- no dependency installation workflow;
- no native export, plugin runtime, marketplace, hosted/cloud/server/auth, source
  patch apply, browser trusted write, command bridge, auto-merge, or auto-apply;
- no claim that Ouroforge is production-ready, compatibility-stable, a Godot
  replacement, or a secure sandbox.

## PA1.2.1 verification evidence

Focused checks run for this audit:

```text
gh issue view 368/1/23 PASS (#368 OPEN, #1 OPEN, #23 OPEN)
README quickstart validation/project-scaffold/project-bound run PASS
canonical demo smoke with OUROFORGE_DEMO_WORKERS=2 PASS
node dashboard/cockpit checks PASS via canonical smoke wrapper
git status --short --ignored observed only expected generated roots after local runs
```

Broad verification is recorded in the PR body and issue closure gate rather than
claimed by this audit note alone.
