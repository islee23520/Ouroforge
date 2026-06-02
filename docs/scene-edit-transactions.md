# Scene Edit Transactions

Scene edit transactions record small Rust-validated scene changes as evidence-native artifacts. They are part of Authoring Loop v2 and keep the trusted write boundary in the CLI/Rust layer.

## Contract

A transaction artifact has schema version `ouroforge.scene-edit-transaction.v1` and records:

- `id` — deterministic transaction id for the scene path, edit, and before/after hashes;
- `scenePath` — scene file targeted by the edit;
- `edit` — entity id, supported edit path, and JSON value;
- `beforeSceneHash` — canonical scene hash before validation/application;
- `afterSceneHash` — canonical scene hash after the edit, present only for successful validation;
- `validationResult` — `passed` or `failed` plus bounded error messages;
- `rollback` — restore metadata pointing back to the `beforeSceneHash`.

Hashes use `fnv1a64-canonical-json-v1`: a deterministic local hash over canonical JSON. It is an evidence/provenance hash, not a security or public compatibility claim.

## CLI usage

Successful edit with artifact output:

```sh
cargo run -p ouroforge-cli -- scene edit examples/game-runtime/scene.json \
  --entity player \
  --path components.transform.x \
  --value '48' \
  --transaction-output runs/manual/transactions/player-x-48.json
```

Failed edits also write a transaction artifact, but leave the trusted scene file unchanged:

```sh
cargo run -p ouroforge-cli -- scene edit examples/game-runtime/scene.json \
  --entity player \
  --path components.size.width \
  --value '0' \
  --transaction-output runs/manual/transactions/invalid-size.json
```

## Transaction-output safety

`--transaction-output` is a generated evidence destination, not a trusted scene
source destination. Rust rejects transaction artifact paths that would overwrite
or alias the target scene before any artifact write occurs. The shared guard
rejects:

- exact path equality with the scene path;
- canonical/symlink aliases that resolve to the scene path;
- hard-link or same-file aliases where filesystem identity is available.

This rule applies to both `scene edit --transaction-output` and scene-only
mutation application transaction outputs. Rejection must leave the original scene
bytes unchanged and parseable, and valid distinct transaction-output paths must
continue to work.

Manual review boundary: a transaction artifact can prove what the Rust command
validated or rejected, but it does not authorize browser-side writes, command
bridges, source patch application, or automatic mutation acceptance.

## Generated-state policy

Transaction artifacts are generated evidence. Keep them in ignored/local paths such as `runs/...` or `.omx/tmp/...` unless a future issue explicitly asks for a small tracked fixture. Do not commit generated transaction artifacts from local authoring sessions.

## Browser boundary

The authoring cockpit may display copyable transaction CLI commands, but it does not execute commands, write files, store trusted state, or become a local command bridge. Browser edits remain preview-only until the Rust CLI command is run by a human/operator.

## Non-goals

- No full undo/redo editor stack.
- No collaborative editing.
- No browser-side trusted persistence.
- No arbitrary source-code patch transaction model.
- No public schema stability guarantee.
