# Trusted Artifact Write Policy v1

Trusted Artifact Write Policy v1 defines local write categories for Ouroforge's
Rust-owned outputs. The goal is to keep source-like project files, generated
evidence, transaction artifacts, dashboard exports, comparison outputs, local
runtime state, and deterministic fixtures from drifting into ambiguous overwrite
or clobber behavior.

This policy is local-first only. It does not introduce a storage backend,
server, database, cloud persistence, browser trusted writes, or a packaged editor
storage model.

## Write Categories

| Category | Examples | Default overwrite rule | Trust boundary |
| --- | --- | --- | --- |
| Tracked source-like project files | project manifests, scenes, scenario packs, seeds, docs | only commands explicitly scoped to edit that source-like file may write it; generated outputs must not target it | Rust CLI validation owns writes |
| Generated run evidence | `runs/<run-id>/run.json`, `evidence/index.json`, screenshots, probe outputs, verdicts, journals | command-owned generated paths may overwrite within the run directory when rebuilding that artifact is documented | Rust runtime owns writes; browser reads exports only |
| Transaction artifacts | scene edit transaction JSON and scene-only mutation transaction output | valid distinct generated paths are allowed; paths that equal or alias the target scene are forbidden | Rust CLI/core owns writes |
| Dashboard exports | `examples/evidence-dashboard/dashboard-data.json`, `.omx/tmp/.../dashboard-data.json`, project `dashboard-data/` | generated export paths may be overwritten intentionally; source-like targets are rejected | Rust CLI exports, browser reads |
| Comparison outputs | generated comparison JSON under run or requested output directories | generated comparison paths may be overwritten inside generated output roots; source-like output directories are rejected | Rust core owns writes |
| Local runtime/tool state | `.omx/`, `.openchrome/`, `.omc/`, `.claude/`, `target/` | local and ignored; never source evidence by default | local tools/runtime only |
| Deterministic fixtures | small checked-in seeds, scenes, docs, explicit test fixtures | tracked only when an issue explicitly scopes fixture changes | review-owned source |

## Writer Inventory and Current Behavior

| Writer / command surface | Current destination | Current behavior | Policy status for v1 |
| --- | --- | --- | --- |
| `create_run` and runtime evidence writers | `runs/<run-id>/...` | create/write generated run files, evidence index, scenario artifacts, screenshots, metrics, and verdicts | intentionally generated; preserve compatibility |
| Ledger/evidence append helpers | generated run evidence tree | append or rewrite generated run indexes/journals | intentionally generated; preserve compatibility |
| `scene edit` without `--transaction-output` | target scene path | edits the trusted scene file by explicit command contract | allowed source-like write |
| `scene edit --transaction-output` | operator-provided transaction artifact path | rejects exact/canonical/symlink/hard-link scene aliases before writing; valid generated paths work | protected by #286; preserve |
| `mutation apply-scene --transaction-output` | operator-provided transaction artifact path | rejects exact/canonical/same-file target scene aliases before writing; then applies validated scene edit | protected by #286; preserve |
| `dashboard export --output` | operator-provided dashboard JSON path, default ignored dashboard data path | creates parent directories and overwrites dashboard export JSON only after generated-output/source-like guard passes | protected from source-like redirection by EF1.3.2 |
| `compare ... --output-dir` / comparison writer | requested output directory, often generated run/mutation directory | writes generated comparison artifact only after generated-output/source-like guard passes | protected from source-like output directories by EF1.3.2 |
| patch draft / sandbox writers | generated mutation/sandbox paths | write inert preview/sandbox artifacts without modifying trusted main worktree | generated preview; preserve blocked source-apply boundary |
| project scaffold writer | requested new scaffold destination | creates tracked-style project files plus scaffold `.gitignore` | allowed because command contract creates source-like project tree |
| project/manifest validation | read-only | no writes | no change |

EF1.3.1 was the audit/control PR. EF1.3.2 added focused generated-output
source-like collision checks for dashboard exports and comparison output
directories. EF1.3.3 reconciles `.gitignore`, scaffold `.gitignore`, README,
and generated-state policy drift without adding generated artifacts as source.

## Protected Source-like Collision Rule

Generated outputs must not silently clobber source-like project or repository
files when a command contract says the output is generated evidence. A protected
check should be focused and command-local unless the same rule is needed by
multiple writers. Prefer existing path validation, `write_json_atomic`, and the
transaction-output alias helper before adding new machinery.

Protected checks should reject:

- exact source path destinations;
- canonical/symlink aliases where paths resolve;
- same-file aliases where filesystem identity is available;
- known generated-output options redirected to manifests, scenes, scenario
  packs, seeds, docs, or other tracked source-like project files when the command
  is not explicitly scoped to edit those files.

## Intentional Generated Overwrites

Some generated artifacts are intentionally rewritten. That remains compatible
when the destination is a generated/local root and the command contract documents
it, for example:

- dashboard export refreshing an ignored `dashboard-data.json`;
- verdict or evidence index regeneration inside a run directory;
- comparison artifacts under generated run/mutation output directories;
- local `.omx/tmp/...` evidence used for PR or issue comments.

These overwrites must not be used to justify writing generated evidence into
tracked source-like files.

## Generated-state Policy

Generated/local roots remain untracked by default:

- `runs/`;
- `target/`;
- `.openchrome/`;
- `.omc/`;
- `.omx/`;
- `.claude/`;
- dashboard export data such as `examples/evidence-dashboard/dashboard-data.json`
  and project-local `dashboard-data/`.

The scaffolded project `.gitignore` must stay aligned with the repository policy
for these roots. If policy drift is found, EF1.3.3 must reconcile the docs and
ignore files without adding generated artifacts as source.

## Browser and Manual Review Boundary

Browser/Studio surfaces may read exported JSON, display writer category warnings,
and show copyable commands. They must not write trusted files, execute commands,
start command bridges, auto-run checks, auto-apply mutations, or auto-merge
source changes.

Manual review remains required whenever generated evidence is used to support a
mutation, comparison, or roadmap decision.

## Non-goals

This policy does not authorize:

- broad virtual filesystem abstraction;
- storage subsystem or persistence backend;
- cloud/server/database/auth infrastructure;
- browser trusted writes or command bridge;
- native export or packaged editor storage;
- plugin storage API;
- arbitrary source-code mutation;
- public release automation.
