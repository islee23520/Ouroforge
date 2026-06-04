# Public demo stability monitoring v1

Status: **manual monitoring playbook for maintainers**. This document does not
change repository visibility, publish a release, add hosted monitoring, create a
support SLA, or automate launch/go-live decisions.

## Scope boundary

This playbook keeps the public alpha demo stable after a maintainer chooses to
make it visible. It is governance and evidence retention only:

- define a repeatable local smoke checklist;
- define a manual refresh cadence;
- name evidence that maintainers should retain in issues or PRs;
- keep generated demo/run/dashboard/tool state out of git;
- document caveats without promising production readiness or compatibility.

Out of scope: hosted uptime checks, package publication, repository visibility
changes, release automation, browser trusted writes, source apply, command
bridges, cloud services, native export, plugin runtime, or a production support
process.

## Core demo stability checklist

Run this checklist from a fresh clone or clean latest-`main` worktree before a
manual visibility decision and during scheduled demo refreshes.

1. **Issue and governance state**
   - Confirm the monitoring issue is still the current scope when running a
     refresh.
   - Confirm #1 remains open.
   - Confirm #23 remains open.
   - Confirm any launch, release, or visibility decision remains a separate
     maintainer action outside this checklist.
2. **Clean generated state**
   - Start with no tracked `runs/`, `target/`, `.openchrome/`, `.omc/`, `.omx/`,
     `.claude/`, or `examples/evidence-dashboard/dashboard-data.json` files.
   - Remove local generated outputs only after preserving the evidence needed for
     the refresh record.
3. **Canonical demo script**
   - Run the Platformer smoke path.
   - Run the Engine Expansion v1 demo smoke path.
   - Export dashboard data from local run evidence.
   - Open or smoke-check the static runtime, evidence dashboard, and authoring
     cockpit views.
4. **Dashboard and Studio surfaces**
   - Verify dashboard rendering remains read-only evidence inspection.
   - Verify the authoring cockpit remains a static/browser-local inspection
     surface and does not write trusted files or execute commands.
   - Record any visual drift as a documentation refresh task rather than a launch
     blocker unless the demo script itself fails.
5. **Chrome caveats**
   - Use a local Chrome or Chromium executable.
   - Set `OUROFORGE_CHROME` when the platform default is unavailable.
   - Treat updater, crashpad, or profile-directory noise as environment caveats
     when screenshots and smoke assertions still succeed.
6. **Generated-state cleanup**
   - Keep run directories, dashboard exports, screenshots generated during the
     refresh, browser profiles, and local tool state untracked unless a separate
     fixture-scoped PR explicitly authorizes committing them.
   - End with `git status --short --ignored` evidence in the issue or PR record.

## Canonical manual smoke commands

```bash
gh issue view <monitoring-issue-number> --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- run seeds/engine-expansion-v1-demo.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
cargo clippy --all-targets --all-features -- -D warnings
git status --short --ignored
```

`cargo audit` remains a recommended dependency/security check before public
visibility or release decisions, but this playbook does not add a hosted security
service or repository dependency.

## Manual refresh cadence

| Cadence | Trigger | Required evidence | Hold condition |
| --- | --- | --- | --- |
| Before visibility decision | Maintainer considers making the repository public | Full canonical smoke command output, generated-state audit, #1/#23 open state | Any required command fails, generated state is tracked, or wording drifts into production/compatibility claims |
| Weekly while the public alpha demo is being actively shared | Demo is linked in docs, issues, or discussions | Issue or PR comment with command summary, run ids, dashboard/cockpit smoke result, Chrome caveats | Repeated smoke failure or stale screenshots that misrepresent current UI |
| After demo-affecting milestone | Runtime, dashboard, cockpit, Seed, scenario, or evidence schema changes | Fresh command summary and notes about changed screenshots/docs | New feature changes demo behavior without updated evidence |
| Before public communication refresh | README, screenshots, roadmap, or public announcement draft changes | Wording scan and generated-state audit | Claims of production readiness, Godot replacement, secure sandbox, native export, plugin runtime, source apply, or support SLA |

## Evidence retention

For each scheduled refresh, keep a concise issue or PR comment with:

- date, branch or commit, and local environment notes;
- commands run and pass/fail summary;
- run ids for local demo runs when generated;
- dashboard/cockpit smoke result;
- Chrome or `OUROFORGE_CHROME` caveats;
- generated-state cleanup result;
- confirmation that #1 and #23 remain open;
- explicit statement that no visibility, launch, release, or publication action
  was automated.

Do not commit raw local `runs/`, generated dashboard exports, browser profiles,
local tool folders, or temporary screenshots unless a separate fixture-scoped
issue authorizes that artifact.

## Known flaky conditions and acceptable workarounds

| Condition | Acceptable workaround | Evidence to record |
| --- | --- | --- |
| Chrome path differs by machine | Set `OUROFORGE_CHROME=/path/to/chrome-or-chromium` | Path family only; do not record private machine details beyond what is needed to reproduce |
| Chrome updater/crashpad/profile noise appears during screenshot capture | Re-run with a fresh temporary `--user-data-dir` and verify output files/tests | Note that the noise was environment-specific and whether screenshots/tests passed |
| Dashboard export depends on local run order | Re-run the two canonical demo commands before export | Record run ids and export command |
| Local generated state remains after smoke | Preserve needed logs in the issue/PR comment, then clean generated outputs | `git status --short --ignored` summary |
| Screenshot drift after UI changes | Refresh docs/media in a separate scoped PR | Link the follow-up issue or PR; do not treat drift as launch approval |

## Conservative wording boundary

Allowed wording: "public alpha demo", "local smoke evidence", "manual monitoring
cadence", "read-only dashboard inspection", and "static authoring cockpit
prototype".

Avoid claims that Ouroforge is production-ready, compatibility-stable, a secure
sandbox, a Godot replacement, a hosted/cloud service, a native export pipeline, a
plugin runtime, an autonomous source-apply system, or covered by a support SLA.
