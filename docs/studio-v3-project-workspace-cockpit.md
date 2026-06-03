# Studio v3 Project Workspace Cockpit Evidence

Studio v3 extends the static authoring cockpit from Authoring Loop v2 into the
Project Workspace Loop v1 surface. It composes project manifest context,
project-bound run metadata, project comparison summaries, and project-scoped
scene mutation lifecycle state from Rust-exported dashboard data.

Studio v3 remains local, static, and read-only for trusted persistence. The
browser displays exported JSON and copyable Rust CLI command strings only; it
must not execute commands, write trusted files, apply/review mutations, or claim
production editor maturity.

## Integrated project surfaces

The cockpit at `examples/authoring-cockpit/` now includes:

- **Project workspace** — project id/name/root, manifest path/hash, seed path,
  scene source paths/hashes, and scenario pack id/path/scenario ids.
- **Project run summary** — latest project-bound run id, run dir, verdict,
  scenario status, evidence count, generated-state status, and display-only
  project run/export commands.
- **Run/evidence browser** — existing evidence link summary.
- **Authoring provenance** — scene edit transaction metadata when bound.
- **Engine Expansion state** — read-only world-state-derived engine summaries.
- **Journal viewer** — generated run journal summary and refs.
- **Project-scoped scene mutation lifecycle** — proposal/application records,
  project-scoped application count, accepted review decision linkage, rollback
  metadata, and display-only project validate / scene validate / review-gated
  apply-scene / dashboard export commands.
- **Replay controls and live preview controls** — ephemeral browser state only.
- **Run comparison** — existing comparison artifacts with Project Comparison v1
  semantic fields and display-only compare command strings.

## Manual project authoring loop

A safe Studio v3 project workflow remains explicit and terminal-owned:

```bash
cargo run -p ouroforge-cli -- project validate <project>/ouroforge.project.json
cargo run -p ouroforge-cli -- run <project>/seeds/platformer.yaml \
  --project <project>/ouroforge.project.json \
  --scenario-pack <pack-id> \
  --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs \
  --output examples/evidence-dashboard/dashboard-data.json
cargo run -p ouroforge-cli -- compare <before-run> <after-run> \
  --output-dir <after-run>/comparisons
cargo run -p ouroforge-cli -- mutation apply-scene <run-dir> \
  --project <project>/ouroforge.project.json \
  --operation <operation.json> \
  --decision <accepted-review-decision-id> \
  --transaction-output <transaction.json>
```

The cockpit may show these commands as copyable text, including the accepted
review decision id when application records provide one. JavaScript does not run,
accept, apply, rollback, merge, or schedule them.

## Guardrails

Studio v3 does not implement:

- browser-side command execution;
- direct browser writes to trusted project files;
- local HTTP command bridge;
- auto-apply, auto-accept, auto-merge, or hidden CLI execution;
- source-code mutation;
- server, cloud, database, auth, native shell, plugin runtime, marketplace UI,
  collaboration, or accounts;
- production editor or Godot-replacement claims.

Generated `runs/`, dashboard exports, screenshots, transaction artifacts, and
local runtime state remain untracked unless a future issue explicitly scopes a
small deterministic fixture as tracked source-like data.

## PW1.8.4 integration evidence

Run from the repository root on 2026-06-03:

```bash
cargo run -p ouroforge-cli -- project init /tmp/ouroforge-pw18-4-studio.k3B9gJ/project --template minimal-2d
cargo run -p ouroforge-cli -- project validate /tmp/ouroforge-pw18-4-studio.k3B9gJ/project/ouroforge.project.json
cargo run -p ouroforge-cli -- run /tmp/ouroforge-pw18-4-studio.k3B9gJ/project/seeds/platformer.yaml \
  --project /tmp/ouroforge-pw18-4-studio.k3B9gJ/project/ouroforge.project.json \
  --scenario-pack smoke \
  --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs \
  --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Observed evidence:

- Project scaffold: `/tmp/ouroforge-pw18-4-studio.k3B9gJ/project`
- Project manifest validation: `minimal_2d`, 3 source refs, 1 asset root, 1
  scenario pack, generated roots `runs,target,dashboard-data`
- Project-bound run: `runs/run-1780415438349-21616`
- Browser smoke: 4 workers succeeded, 0 failed
- Scenario suite: 1 passed, 0 failed
- Verdict: `passed`
- Dashboard export: `examples/evidence-dashboard/dashboard-data.json`
- Dashboard smoke test: passed
- Authoring cockpit smoke test: passed

The temporary project directory and dashboard export were deleted after
verification. The generated run under `runs/` remains ignored local evidence and
is not tracked.

## Screenshot policy

PW1.8.1–PW1.8.3 changed cockpit layout text and panels, but PW1.8.4 did not
refresh public screenshot assets. Current verification is source/test based:
Node tests cover project present/missing/malformed/escaped states and command
strings. Refresh screenshots in a future public-readiness/media issue if the
public demo images need to represent Studio v3.

## Known gaps

- No production editor.
- No native app/shell integration.
- No hosted studio, collaboration, accounts, or cloud sync.
- No plugin marketplace or visual scripting.
- No browser-side comparison algorithm.
- No browser-side mutation application or review authority.
- No direct trusted file writes from JavaScript.
