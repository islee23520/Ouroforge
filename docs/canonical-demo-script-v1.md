# Canonical Demo Script v1

Status: **demo flow contract and smoke script contract** for issue #369 PA1.3.1/PA1.3.2.

This document defines the canonical non-destructive Ouroforge demo command
sequence. PA1.3.2 implements the local smoke wrapper at
`scripts/canonical-demo-smoke.sh`; PA1.3.3 owns final evidence docs. It does not add automation that applies source
patches, writes trusted browser state, publishes packages, merges branches,
changes repository visibility, or claims production readiness.

## Demo intent

The demo should show the local evidence-native loop from project validation to
read-only inspection:

1. validate a project manifest;
2. run local browser/scenario evidence;
3. export dashboard data;
4. compare two generated runs;
5. record an explicit mutation review decision without applying a mutation;
6. preview visual edit drafts without writes;
7. validate an inert source patch preview without applying it;
8. open static dashboard/Studio pages for read-only inspection;
9. clean generated local state.

## Generated output boundary

All generated outputs must stay in ignored/local paths unless a later PR
explicitly scopes a tiny deterministic fixture:

- `runs/`
- `runs/comparisons/` or another generated comparison output directory
- `examples/evidence-dashboard/dashboard-data.json`
- `/tmp/ouroforge-canonical-demo-*`
- browser profile folders under `/tmp/ouroforge-demo-browser-*`
- local server logs, smoke logs, screenshots, or command transcripts
- `target/`

Tracked docs may reference generated run ids and paths, but generated run,
dashboard, comparison, browser, screenshot, or local tool outputs are not source
files.

## Canonical command sequence

Run from the repository root on a fresh clone with a local Chrome/Chromium
available through the platform default path or `OUROFORGE_CHROME`.

### 0. Preflight and live governance checks

```bash
gh issue view 369 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
```

Expected output:

- #369 is open before implementation/closure.
- #1 and #23 are open.
- Formatting is clean.

### 1. Project validation

```bash
cargo run -p ouroforge-cli -- project validate \
  examples/playable-demo-v2/collect-and-exit/ouroforge.project.json
```

Expected output:

- the manifest id is reported;
- source refs, asset roots, scenario packs, `runs`, and generated roots are
  summarized;
- no project files are written.

### 2. Local run/evidence generation

The PA1.3.2 smoke wrapper runs this step with explicit logging and failure
handling. The contract uses two runs so the comparison command has a before/after
pair:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- run seeds/engine-expansion-v1-demo.yaml --workers 4
```

Expected output:

- each command creates a generated `runs/run-*` directory;
- browser smoke outcomes are recorded under run evidence;
- scenario verdicts are recorded honestly as passed or failed;
- a failed verdict may create a placeholder mutation proposal, but it does not
  apply changes.

Do not rewrite failed verdicts as passing evidence. If current run assertions
fail, the demo script must report that status and continue only through read-only
inspection steps.

### 3. Dashboard export

```bash
cargo run -p ouroforge-cli -- dashboard export \
  --runs-root runs \
  --output examples/evidence-dashboard/dashboard-data.json
```

Expected output:

- `examples/evidence-dashboard/dashboard-data.json` is generated locally;
- dashboard data remains ignored/untracked;
- browser/dashboard surfaces are read-only inspection surfaces.

### 4. Run comparison

Use the two generated run directories from step 2:

```bash
cargo run -p ouroforge-cli -- compare \
  runs/<before-run-id> \
  runs/<after-run-id> \
  --output-dir runs/comparisons
```

Expected output:

- comparison evidence is written under an ignored generated output directory;
- source files are not modified;
- comparison output is evidence, not an automatic promotion or release decision.

### 5. Mutation review decision without apply

If a generated run contains a mutation proposal, record an explicit review
decision against the generated run or draft path. The canonical public demo uses
`--defer` or `--reject`; it must not call `mutation apply-scene`.

```bash
cargo run -p ouroforge-cli -- mutation review \
  --defer \
  --reason "canonical demo records review decision only; no apply" \
  --evidence mutation/patch-drafts.json \
  runs/<run-id>
```

Expected output:

- a review decision is recorded in generated run state;
- no scene/source files are changed;
- no auto-merge, auto-apply, or branch mutation occurs.

### 6. Visual draft preview without writes

Use existing valid demo fixtures. These commands print read-only previews and do
not write transaction output unless PA1.3.2 explicitly scopes a generated
`/tmp/...` output and audits it.

```bash
cargo run -p ouroforge-cli -- edit draft-preview \
  --project examples/playable-demo-v2/collect-and-exit/ouroforge.project.json \
  examples/visual-edit-draft-v1/valid/collect-and-exit-scene-demo.visual-edit-draft.json

cargo run -p ouroforge-cli -- edit draft-preview \
  --project examples/playable-demo-v2/collect-and-exit/ouroforge.project.json \
  examples/visual-edit-draft-v1/valid/collect-and-exit-asset-frame-demo.visual-edit-draft.json
```

Expected output:

- preview JSON includes guardrail text stating preview-only/no writes;
- browser trusted writes and apply behavior remain absent;
- tilemap or mismatched fixtures are not part of the canonical demo unless a
  later issue explicitly enables them.

### 7. Source patch preview validation without apply

```bash
cargo run -p ouroforge-cli -- patch-preview validate \
  examples/source-mutation-preview-demo-v1/patch-preview-demo.sample.json
```

Expected output:

- validation status is reported;
- file-class and diff-integrity guardrails are shown;
- required tests remain inert metadata;
- source patch apply, merge, command execution, and trusted writes remain out of
  scope.

### 8. Static read-only Studio/dashboard inspection

After dashboard export, serve the repository locally for browser inspection:

```bash
python3 -m http.server 8765 --bind 127.0.0.1 --directory .
```

Open these local pages manually or through a bounded screenshot smoke in PA1.3.2:

- `http://127.0.0.1:8765/examples/evidence-dashboard/index.html`
- `http://127.0.0.1:8765/examples/authoring-cockpit/index.html`
- `http://127.0.0.1:8765/examples/game-runtime/index.html`

Expected output:

- pages read local static files and generated dashboard data;
- pages do not write trusted files, execute shell commands, upload content,
  bridge to a local command server, apply patches, or merge branches.

### 9. Cleanup

```bash
rm -rf runs
rm -f examples/evidence-dashboard/dashboard-data.json
rm -rf /tmp/ouroforge-canonical-demo-* /tmp/ouroforge-demo-browser-*
```

Optional local build cleanup:

```bash
rm -rf target
```


## PA1.3.2 smoke wrapper

Run the non-destructive local smoke wrapper from the repository root:

```bash
scripts/canonical-demo-smoke.sh --keep
```

The wrapper calls the Rust CLI through this repository's `Cargo.toml`. Run
evidence is created from the repository root so repo-relative browser targets
resolve correctly, then newly generated `runs/run-*` directories are moved into
an isolated generated work directory such as `/tmp/ouroforge-canonical-demo-*`.
Dashboard data, comparison outputs, visual previews, source patch preview
validation output, logs, and the summary file also stay under that generated
work directory. Without `--keep`, the work directory is removed on exit; with
`--keep`, the script prints the path for manual inspection.

The wrapper records failed run verdicts honestly and still proceeds through
read-only dashboard, compare, deferred mutation review, visual preview, source
patch preview validation, and static Node surface checks. It never calls
`mutation apply-scene`, `edit draft-apply`, source patch apply, merge, publish,
release, visibility, browser trusted-write, or command-bridge operations.

Tunable local-only options:

```bash
OUROFORGE_DEMO_WORKERS=4 scripts/canonical-demo-smoke.sh --keep
scripts/canonical-demo-smoke.sh --work-dir /tmp/ouroforge-canonical-demo-manual --keep
```

## Command audit

Allowed command categories for the canonical demo:

| Category | Commands | Write boundary |
| --- | --- | --- |
| Governance checks | `gh issue view 369/1/23` | Read-only GitHub state checks. |
| Static verification | `cargo fmt --check`, Node syntax/smoke checks | No source writes. |
| Project validation | `project validate` | Read-only manifest validation. |
| Run/evidence | `run ... --workers 4` | Generated `runs/` only. |
| Dashboard export | `dashboard export` | Generated ignored dashboard JSON only. |
| Compare | `compare ... --output-dir runs/comparisons` | Generated comparison output only. |
| Review decision | `mutation review --defer` or `--reject` | Generated run review record only; no apply. |
| Visual preview | `edit draft-preview` | stdout preview or explicitly generated temp output only. |
| Source preview | `patch-preview validate` | read-only validation output. |
| Static inspection | `python3 -m http.server` and browser viewing | local read-only static serving. |
| Cleanup | `rm -rf runs`, `rm -f ...dashboard-data.json` | generated paths only. |

Forbidden for the canonical demo:

- repository visibility changes;
- release, package, binary, signing, upload, deployment, or public announcement
  automation;
- `mutation apply-scene`, `edit draft-apply`, source patch apply, source merge,
  auto-merge, auto-apply, branch mutation, or trusted source writes;
- browser trusted file writes, local command bridge, hidden command execution,
  hosted/cloud/server/auth behavior, plugin runtime, marketplace, or native
  export;
- dependency installation workflows or credential/network publishing commands.

## PA1.3.1/PA1.3.2 verification commands

```bash
gh issue view 369 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo run -p ouroforge-cli -- project validate examples/playable-demo-v2/collect-and-exit/ouroforge.project.json
cargo run -p ouroforge-cli -- edit draft-preview --project examples/playable-demo-v2/collect-and-exit/ouroforge.project.json examples/visual-edit-draft-v1/valid/collect-and-exit-scene-demo.visual-edit-draft.json
cargo run -p ouroforge-cli -- edit draft-preview --project examples/playable-demo-v2/collect-and-exit/ouroforge.project.json examples/visual-edit-draft-v1/valid/collect-and-exit-asset-frame-demo.visual-edit-draft.json
cargo run -p ouroforge-cli -- patch-preview validate examples/source-mutation-preview-demo-v1/patch-preview-demo.sample.json
bash -n scripts/canonical-demo-smoke.sh
OUROFORGE_DEMO_WORKERS=1 CARGO_TARGET_DIR=/tmp/ouroforge-canonical-demo-target scripts/canonical-demo-smoke.sh --keep
cargo fmt --check
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```
