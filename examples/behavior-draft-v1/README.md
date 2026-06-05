# Behavior Draft v1 fixtures

This directory contains fixture-scoped Behavior Draft v1 examples for #619.
They are tracked because they are deterministic validator/read-model contracts;
local generated behavior drafts remain untracked.

- `valid/behavior-draft.valid.json` — drafted behavior with linked evidence.
- `valid/behavior-draft.stale.json` — stale target state that must remain
  visibly blocked before review.
- `valid/behavior-draft.missing-evidence-blocked.json` — missing evidence state
  represented as an explicit blocked draft.
- `valid/behavior-draft.unsupported-blocked.json` — unsupported structured
  behavior represented as an explicit blocked draft.
- `invalid/behavior-draft.unsafe-target.json` — unsafe target path.
- `invalid/behavior-draft.malformed-operation.json` — malformed proposed
  behavior operation.

Use the read-only validate/preview commands for local inspection:

```bash
cargo run -p ouroforge-cli -- behavior draft validate examples/behavior-draft-v1/valid/behavior-draft.valid.json
cargo run -p ouroforge-cli -- behavior draft preview examples/behavior-draft-v1/valid/behavior-draft.valid.json
```

The commands and cockpit read model do not apply trusted files, execute scripts,
open command bridges, persist browser state, or approve behavior changes.
