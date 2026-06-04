# Public demo CI/manual smoke evidence policy v1

Status: **evidence policy only**. This document describes how maintainers record
manual or existing CI smoke evidence for the public alpha demo. It does not add a
workflow, require hosted infrastructure, publish artifacts, change repository
visibility, or automate launch/release decisions.

## Policy boundary

The public demo stability gate may be satisfied by either:

1. a manual maintainer smoke run recorded in an issue or PR comment; or
2. an already-present repository check that runs an equivalent read-only/local
   smoke subset.

This policy does not require GitHub Actions, hosted browsers, external
monitoring, cloud services, package publication, release automation, source
apply, browser trusted writes, local server command bridges, credentials, or
repository setting changes.

## Acceptable evidence sources

| Source | Acceptable when | Required record |
| --- | --- | --- |
| Manual local smoke | A maintainer runs commands from a clean latest-`main` worktree or fresh clone | Date, commit, command summary, run ids, pass/fail result, Chrome caveats, generated-state cleanup, #1/#23 open state |
| Existing repository check | A check already runs a documented local/static subset without launch automation | Check name, commit, URL/status, command subset, known gaps compared with the manual checklist |
| Documentation-only audit | The PR changes only governance/docs/templates | Wording scan, generated-state audit, #1/#23 open state, and a rationale for skipping demo execution |

A hosted check is optional. If no hosted check exists, the manual smoke record is
the canonical evidence source.

## Required manual evidence fields

Every public-demo stability refresh comment should include:

```text
Public demo stability refresh
Date:
Commit or branch:
Issue / PR:
Commands run:
Run ids generated:
Dashboard smoke:
Authoring cockpit smoke:
Chrome / OUROFORGE_CHROME notes:
Generated-state cleanup:
Known flaky conditions:
Conservative wording audit:
#1 state:
#23 state:
Launch boundary: no visibility, release, publication, or settings change was automated.
```

Keep raw generated run data local unless a fixture-scoped issue explicitly
authorizes committing it.

## CI/check expectations if present

An existing check can support the public demo stability gate when it follows
these rules:

- it is read-only with respect to trusted repository settings and publication;
- it does not publish packages, screenshots, release artifacts, binaries, or
  deployment outputs;
- it does not mutate repository visibility, secrets, environments, branch
  protection, issue state, or release state;
- it does not run browser trusted file writes, local command bridges, source
  apply, auto-merge, or auto-approve flows;
- it reports the commit and pass/fail status clearly;
- it keeps generated artifacts ephemeral unless a fixture-scoped PR explicitly
  tracks them;
- it documents any difference from the manual smoke checklist.

If a check only runs a subset, maintainers must record the missing manual
coverage before using it for a visibility or public-communication decision.

## Generated-state cleanup policy

Before closing a stability-monitoring issue or PR, record:

```bash
git status --short --ignored
git ls-files runs target .openchrome .omc .omx .claude examples/evidence-dashboard/dashboard-data.json 2>/dev/null || true
```

Expected result:

- no generated run directories are tracked;
- no generated dashboard export is tracked unless fixture-scoped;
- no local tool state is tracked;
- ignored build/runtime outputs may exist locally but are not source evidence;
- any retained logs are summarized in issue/PR comments rather than committed as
  generated run state.

## Chrome and browser caveat policy

Chrome/Chromium is a local prerequisite for screenshot or browser smoke refresh.
When the default executable path fails, use `OUROFORGE_CHROME` and record only
reproducible environment facts. Avoid committing machine-local paths,
profile directories, crash reports, or browser caches.

If Chrome emits updater/crashpad/profile noise but produces the expected
screenshots and tests pass, record it as an environment caveat. If screenshots or
browser smoke fail, treat the refresh as held until rerun or documented as a
known environment-specific blocker.

## Hold and rollback criteria

Hold a public-demo stability refresh when any of the following is true:

- canonical smoke commands fail without an environment-specific explanation;
- dashboard or cockpit tests fail;
- generated demo/run/dashboard/tool state becomes tracked accidentally;
- documentation introduces production-ready, compatibility-stable, secure
  sandbox, Godot replacement, native export, plugin runtime, source apply,
  hosted/cloud, or support SLA claims;
- #1 or #23 is closed or modified without a separate explicit governance issue;
- a check or script attempts release, publication, repository setting mutation,
  auto-merge, auto-apply, or browser trusted writes.

Rollback for this policy means reverting the documentation or template change
that introduced the drift and leaving visibility/release decisions manual.
