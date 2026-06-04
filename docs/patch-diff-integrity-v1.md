# Patch Diff Integrity v1

Patch Diff Integrity v1 is the implemented read-only preflight layer for issue
#357 in Source Mutation Preview v1. It parses unified diff text into a bounded
integrity report, links every diff target to Source File Class Validator v1, and
returns a preview validation read-model before any sandbox dry-run or trusted
source write can occur.

This layer does **not** implement source patch apply, sandbox evaluation,
merge/rebase automation, branch mutation, dependency/CI/build-script mutation,
browser trusted writes, command bridges, install/network/arbitrary shell
execution, plugin/native export behavior, hosted/cloud/server/auth behavior, or
production security sandbox claims.

## Pipeline position

```text
source file class validation
  -> patch diff integrity validation
  -> future inert source patch preview artifact
  -> future allowlisted sandbox dry-run evidence
  -> future review/evidence/Studio read-only surfaces
```

Patch Diff Integrity v1 accepts diff text as data and emits data. A `passed`
validation means the diff is well-formed enough for later preview checks. It is
not permission to apply, merge, execute commands, or trust browser/UI state.

## Parser report shape

The parser report serializes as `patch-diff-integrity-v1`:

```json
{
  "schemaVersion": "patch-diff-integrity-v1",
  "fileCount": 1,
  "hunkCount": 1,
  "hasWarnings": false,
  "counts": { "added": 1, "removed": 1, "context": 1 },
  "files": [
    {
      "oldPath": "docs/example.md",
      "newPath": "docs/example.md",
      "status": "modified",
      "isBinary": false,
      "hunks": [
        {
          "oldStart": 1,
          "oldLines": 3,
          "newStart": 1,
          "newLines": 3,
          "actualOldLines": 3,
          "actualNewLines": 3,
          "counts": { "added": 1, "removed": 1, "context": 1 }
        }
      ],
      "counts": { "added": 1, "removed": 1, "context": 1 }
    }
  ],
  "warnings": []
}
```

Malformed but parseable diffs remain reportable data. Parser warnings are
included in `warnings[]` with `kind`, `lineNumber`, optional `path`, and
`message`; validation decides whether a warning blocks later preview.

## Preview validation read-model

The preview validation read-model serializes as
`patch-diff-integrity-validation-v1`:

```json
{
  "schemaVersion": "patch-diff-integrity-validation-v1",
  "status": "blocked",
  "report": { "schemaVersion": "patch-diff-integrity-v1" },
  "fileClassValidation": {
    "schemaVersion": "source-patch-target-class-validation-v1",
    "status": "blocked"
  },
  "limits": { "maxFiles": 32, "maxChangedLines": 5000 },
  "blockedReasons": [
    "file class blocked: runs/run-1/run.json (generated-local-state): generated/local/tool state"
  ],
  "guardrails": [
    "diff integrity validation only; no source patch apply",
    "blocked diffs stop before sandbox dry-run or trusted source writes",
    "browser/dashboard/Studio surfaces remain read-only and command-inert",
    "generated preview/sandbox/report artifacts remain untracked unless fixture-scoped"
  ]
}
```

The abbreviated nested objects above keep the document readable; real serialized
output includes the full parser report and full source file class validation
report.

Review surfaces that need compact display data may use
`patch-diff-integrity-read-model-v1`:

```json
{
  "schemaVersion": "patch-diff-integrity-read-model-v1",
  "status": "blocked",
  "fileCount": 1,
  "hunkCount": 1,
  "changedLines": 2,
  "warningCount": 0,
  "blockedReasons": [
    "file class blocked: runs/run-1/run.json (generated-local-state): generated or local root runs"
  ],
  "targetSummaries": [
    "runs/run-1/run.json: generated-local-state/blocked"
  ],
  "guardrails": [
    "diff integrity validation only; no source patch apply"
  ]
}
```

The compact read model is display-only. It must not execute commands, write
files, apply patches, merge branches, alter reviewer decisions, or infer that a
blocked diff is safe.

## Blocking rules

Validation returns `status: "blocked"` when any of these are present:

- source target class validation reports `blocked` for generated/local state,
  dependency manifests, CI/workflows, build scripts, auth/network/cloud/server
  paths, plugin/native export paths, hidden paths, traversal/absolute paths,
  binary/opaque paths, or unknown classes;
- touched file count exceeds `limits.maxFiles`;
- added plus removed lines exceed `limits.maxChangedLines`;
- binary patch markers or binary diff targets are present;
- file mode metadata or mode changes are present;
- rename/copy metadata is present;
- critical file deletion targets are present, currently including `Cargo.toml`,
  `Cargo.lock`, `README.md`, `AGENTS.md`, and `.github/workflows/**`.

Blocked validation errors are fail-closed before any preview artifact, sandbox
dry-run, trusted source write, merge, browser action, or command execution.

## Fixture groups

Fixture diffs live under `examples/patch-diff-integrity-v1/`:

- `valid/` — ordinary parser/validation-positive unified diffs.
- `invalid/` — malformed parser-warning fixtures that remain inert report data.
- `unsafe/` — generated-target, traversal, binary, mode-change, and critical
  deletion fixtures that validation blocks before preview.

These fixtures are tracked source-like test data. Generated preview, sandbox,
report, dashboard, run, cache, and local tool outputs remain ignored/untracked
unless a later issue explicitly scopes tiny deterministic fixtures.

## Verification

Focused checks:

```bash
cargo test -p ouroforge-core --test patch_diff_integrity -- --nocapture
cargo test -p ouroforge-core patch_diff_integrity_v1 -- --nocapture
```

Broad gates for issue #357 remain:

```bash
gh issue view 357 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

#1 and #23 remain open governance/context anchors.
