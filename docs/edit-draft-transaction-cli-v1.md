# Edit Draft to Transaction CLI v1

Status: **implemented for Visual Authoring v1 / issues #348 and #350**.

The edit-draft CLI is the Rust-trusted boundary for turning visual edit draft
JSON into reviewable transaction previews and, for accepted scene-only changes,
review-gated apply artifacts. It is intentionally local-first and conservative:
Studio/browser surfaces may show inert draft state or copy commands, but they do
not write trusted files, execute commands, apply changes, or make review
decisions.

## Commands

### Preview a draft

```bash
cargo run -p ouroforge-cli -- edit draft-preview <draft.json> --project <project-root>
```

`draft-preview` parses the draft, loads the project manifest, validates target
hashes and references, and prints deterministic preview JSON. Scene drafts return
scene edit transaction previews. Asset-reference drafts return bounded reference
preview summaries. Unsupported draft target classes remain rejected before any
write.

To preflight a scene transaction output path without writing it:

```bash
cargo run -p ouroforge-cli -- edit draft-preview <draft.json> \
  --project <project-root> \
  --transaction-output runs/draft-previews/scene-edit.json
```

The `--transaction-output` form validates that the destination is generated-state
safe, rejects source/project collisions, and requires exactly one scene
transaction preview. The preview response records the checked output path, but
`draft-preview` does not create the file.

### Apply an accepted scene draft

```bash
cargo run -p ouroforge-cli -- edit draft-apply <draft.json> \
  --project <project-root> \
  --run-dir <run-dir> \
  --proposal <proposal-id> \
  --decision <decision-id> \
  --transaction-output runs/draft-apply/accepted.json
```

`draft-apply` is review-gated and scene-only. It requires an explicit review
decision id, verifies the draft `reviewGate` proposal, patch draft, accepted
decision, and review hashes through the existing mutation review ledger, requires
exactly one scene operation, validates the target scene/hash, and writes the
transaction artifact only through the trusted Rust CLI boundary. Accepted apply
output preserves rollback/application metadata from the existing scene-only
mutation path and appends `mutation/visual-edit-applications.json` with draft,
proposal, patch draft, decision, transaction, before/after hash, rollback, and
display-only command-context evidence.

The stored command context is a reproducibility/rerun aid only. Dashboards and
Studio may render it as escaped text for manual copy, but they must not execute
it, auto-rerun scenarios, auto-apply drafts, or infer trusted state from it.

## Safety boundaries

- The browser/Studio boundary remains read-only or copy-only; it never writes
  project files, runs shell commands, or applies draft previews.
- `draft-preview` never writes transaction output; it only validates and reports
  whether the requested output path is safe.
- `draft-apply` supports scene drafts only in this milestone and requires a
  review decision. Tilemap and asset-reference apply paths are not authorized by
  #348.
- `mutation/visual-edit-applications.json` is an audit/read-model artifact. It
  may feed journals, regression review, and loop/status inspection, but it does
  not grant any browser or Studio authority to replay commands.
- Generated draft, transaction, preview, evidence, and run output belongs under
  ignored generated roots such as `runs/`, `.omx/`, or other explicitly ignored
  local state. Do not commit generated smoke output.
- This milestone does not authorize source mutation, browser apply, auto-apply,
  auto-accept, auto-merge, dependency/CI mutation, production editor claims, or
  narrowing/closing #1 or #23.

## VA1.8.3 rerun, regression, and loop compatibility

The review-gated visual edit apply lifecycle is compatible with existing review, rerun, regression, and loop surfaces by recording local evidence for later operators to inspect without adding automation:

- `reviewGate.proposalId`, `reviewGate.patchDraftId`, and `reviewGate.reviewDecisionId` bind the draft to review evidence before apply.
- The apply response and `mutation/visual-edit-applications.json` record draft/proposal/patch-draft/decision ids, transaction id, target scene path, before/after scene hashes, rollback metadata, and reproducible `commandContext`.
- Rerun context remains explicit command text/evidence only. A human or loop operator may use the recorded command context and transaction artifact as inputs to existing rerun/compare workflows, but Ouroforge does not auto-rerun scenarios from `draft-apply`, dashboard, Studio, or loop read-model surfaces.
- Regression promotion remains the existing review-gated Rust CLI flow. Visual edit apply evidence may be linked as provenance, but it does not promote scenarios, write scenario packs, or mark matrix status by itself.
- Authoring loop status/read models may display the visual edit application, rollback refs, command context, rerun/compare refs, and blockers as escaped read-only state. Browser surfaces must not resume loops, execute commands, apply mutations, promote regressions, repair refs, or hide failed/stale evidence.

This compatibility layer is documentation and evidence linkage only. It does not add auto-rerun, browser apply, browser command execution, hidden retries, source mutation, or any new trusted write path.

## Smoke evidence procedure

Use temporary or ignored generated roots when collecting smoke evidence for this
command family. A typical smoke run should include:

```bash
gh issue view 348 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo test -p ouroforge-cli edit_draft --test artifact_commands
cargo test -p ouroforge-cli --test artifact_commands
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

For issue #348 PR evidence, smoke logs are stored under `.omx/context/` and
remain untracked local audit artifacts. The generated-state audit must show no
tracked draft/transaction/run output, with only ignored local/generated roots
reported by `git status --short --ignored`.

## Relationship to Visual Authoring v1

This CLI is PR unit `VA1.6.4`'s documentation and smoke-evidence surface for the
implemented #348 command family. It follows the Visual Authoring v1 ordering in
`docs/visual-authoring-v1.md`: draft models and preview validation come before
review-gated apply, and Studio authoring remains a later read-only/copy-only UI
surface rather than a trusted writer.
