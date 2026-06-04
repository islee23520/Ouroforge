# Source File Class Validator v1

Source File Class Validator v1 is the implemented preflight surface for issue
#356 in Source Mutation Preview v1. It classifies source patch preview target
paths before any diff integrity check, patch preview artifact, sandbox dry-run,
review decision, or source write can occur.

This validator does **not** implement source patch apply, arbitrary patch apply,
merge/rebase automation, sandbox execution, browser trusted writes, command
bridges, dependency/CI/build-script mutation, public launch behavior, native
export, plugin runtime, hosted/cloud/server/auth code, or Godot replacement
claims.

## Report shape

A single target classification serializes as `source-file-class-v1`:

```json
{
  "schemaVersion": "source-file-class-v1",
  "path": "seeds/platformer.yaml",
  "class": "deterministic-scene-fixture",
  "decision": "allowed",
  "reviewLevel": "Elevated source-like data review",
  "rationale": "deterministic seed fixtures may be previewed later with stale-target and rollback controls",
  "blockedReasons": [],
  "guardrails": [
    "preview classification only; no source patch apply",
    "no merge, auto-merge, auto-apply, or hidden command execution",
    "generated/local state remains untracked unless explicitly fixture-scoped",
    "#1 and #23 remain open"
  ]
}
```

A multi-target preflight serializes as
`source-patch-target-class-validation-v1`:

```json
{
  "schemaVersion": "source-patch-target-class-validation-v1",
  "status": "blocked",
  "targets": [
    { "schemaVersion": "source-file-class-v1", "path": "Cargo.lock", "class": "dependency-manifest", "decision": "blocked" }
  ],
  "blockedReasons": ["Cargo.lock (dependency-manifest): dependency manifest or lockfile"],
  "guardrails": [
    "target class validation only; no diff evaluation or patch preview artifact",
    "blocked classes stop before sandbox dry-run or trusted source writes",
    "source patch apply and merge automation remain unimplemented",
    "browser/dashboard/Studio surfaces remain read-only and command-inert"
  ]
}
```

The example above is abbreviated; real `targets[]` entries include `reviewLevel`,
`rationale`, `blockedReasons`, and `guardrails`.

## Decisions

| Decision | Meaning | Later handling |
| --- | --- | --- |
| `allowed` | Deterministic source-like data that may be previewed later after diff integrity, stale-target, review, rollback, and generated-state controls exist. | May continue to later preview checks. No apply permission is implied. |
| `needs-approval` | Restricted class such as docs/governance, Rust trust-boundary code, tests/evidence readers, or browser/Studio display code. | May continue only as review-held metadata for later explicit approval. No default apply or sandbox authority. |
| `blocked` | Unsafe, generated/local, dependency, CI/workflow, build-script, auth/network/cloud, plugin, native export, opaque/binary, hidden, traversal, or unknown class. | Stops before diff evaluation, patch preview artifact creation, sandbox dry-run, or trusted source writes. |

## Hard-blocked classes

The validator hard-blocks these by default:

- dependency manifests and lockfiles (`Cargo.toml`, `Cargo.lock`, package
  manifests/locks);
- CI/workflows, GitHub Actions, secrets-adjacent config (`.github/**`);
- build scripts, installer/tool scripts, shell scripts, and host-executed tooling;
- auth, network, hosted/cloud/server/deploy paths;
- plugin/extension/marketplace runtime paths;
- native export, release, packaging, distribution paths;
- ignored local/generated roots (`runs/`, `target/`, `dashboard-data/`, `.omx/`,
  `.omc/`, `.claude/`, `.openchrome/`) and generated dashboard output;
- hidden root or nested hidden components other than the explicitly classified
  `.github` workflow root;
- absolute, rooted, prefixed, traversal, empty, or otherwise unsafe paths;
- binary/opaque assets and unknown classes.

## Review-held classes

The validator marks these as `needs-approval` rather than `allowed`:

- documentation and governance docs, including `README.md` and `docs/*.md`;
- Rust trust-boundary code under `crates/**`;
- tests and evidence readers;
- browser/dashboard/Studio display code under `examples/evidence-dashboard/**`
  and `examples/authoring-cockpit/**`.

## Generated-state policy

Classifier fixtures under `examples/source-file-class-v1/` are tracked
source-like test fixtures. Generated preview, sandbox, report, dashboard, run,
cache, and local tool outputs remain ignored/untracked unless a later issue
explicitly scopes a tiny deterministic source-like fixture.

## Verification

Focused checks:

```bash
cargo test -p ouroforge-core source_file_class_v1 -- --nocapture
cargo test -p ouroforge-core source_patch_target_class_validation -- --nocapture
```

Broad gates for #356 remain:

```bash
gh issue view 356 --repo shaun0927/Ouroforge
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
