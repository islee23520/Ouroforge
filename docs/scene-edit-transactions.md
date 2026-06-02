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
