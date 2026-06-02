# Reproducible Run Command Context v1

Issue #291 records normalized command context as run evidence so maintainers can
understand how a generated run was produced and manually reproduce it. The
context is metadata only. It is not an auto-rerun executor, browser command
bridge, shell script generator, CI launcher, or trusted automation surface.

## Run metadata contract

New runs include `run_command_context` in `run.json`:

- `schemaVersion`: `run-command-context-v1`;
- `command`: display-only command string for manual copying;
- `argv`: normalized argument vector;
- `seedPath`: seed path used for the run command;
- `workers`: requested worker count;
- `runsRoot`: generated run root;
- `projectRoot`: project root when `--project` is present;
- `manifestPath`: validated project manifest path when present;
- `scenarioPackId`: scenario pack id when present;
- `transactionPath`: scene edit transaction path when present;
- `runtimeTarget`: local runtime assumption;
- `browserBoundary`: `openchrome_cdp`;
- `cdpTransport`: `chrome_devtools_protocol`;
- `environmentHints`: non-secret textual hints only.

Legacy exports without this field remain readable. Malformed command-context data
is ignored by dashboard read models instead of inferred.

## Read-only surfaces

The command context appears in:

- `journal.md` as a “Reproducible Command Context” section;
- `dashboard export` read models as `command_context` on both run detail and
  summary;
- the static evidence dashboard as escaped display-only text;
- the authoring cockpit project run panel as escaped display-only text.

These surfaces must not execute commands, start a bridge, write files, rerun QA,
auto-apply mutations, or infer missing context.

## Secret and machine-detail policy

The model records only the explicit run inputs and bounded local execution
assumptions. It must not capture arbitrary environment variables, credentials,
tokens, cookies, SSH keys, browser profiles, or machine/user secrets. Absolute or
local paths may appear only when they were explicit command inputs or validated
project metadata needed to reproduce a local run.

## Smoke commands

Legacy run context smoke:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 1
```

Project-bound run context smoke:

```bash
cargo run -p ouroforge-cli -- project init .omx/tmp/repro-run-smoke --template minimal-2d
cargo run -p ouroforge-cli -- run .omx/tmp/repro-run-smoke/seeds/platformer.yaml \
  --project .omx/tmp/repro-run-smoke \
  --scenario-pack smoke \
  --workers 1
```

Dashboard/cockpit compatibility smoke:

```bash
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
rm -f examples/evidence-dashboard/dashboard-data.json
rm -rf .omx/tmp/repro-run-smoke
```

`runs/`, `.omx/tmp/repro-run-smoke`, and the dashboard export are generated local
state. Do not commit them.

## Fresh EF1.7.3 smoke evidence

Fresh local smoke evidence for #291 EF1.7.3:

- Legacy run: `runs/run-1780423410317-40423`
  - schema: `run-command-context-v1`
  - workers: `1`
  - seed path: `seeds/platformer.yaml`
  - project root: none
  - scenario pack: none
  - command: `cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 1`
- Project run: `runs/run-1780423411233-40680`
  - schema: `run-command-context-v1`
  - workers: `1`
  - seed path: `.omx/tmp/repro-run-smoke/seeds/platformer.yaml`
  - project root: `.omx/tmp/repro-run-smoke`
  - scenario pack: `smoke`
  - command: `cargo run -p ouroforge-cli -- run .omx/tmp/repro-run-smoke/seeds/platformer.yaml --project .omx/tmp/repro-run-smoke --workers 1 --scenario-pack smoke`
- Dashboard export succeeded and was removed before commit.
- Evidence dashboard Node syntax/smoke passed.
- Authoring cockpit Node syntax/smoke passed.

## Non-goals

Reproducible Run Command Context v1 does not add:

- auto-rerun;
- browser/local command bridge;
- browser-side trusted writes;
- shell script generation as trusted automation;
- CI launch automation;
- public release workflow;
- server/cloud/auth infrastructure;
- secret or credential capture.
