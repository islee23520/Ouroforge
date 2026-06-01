# Contributing to Ouroforge

Ouroforge is pre-release. Contributions should preserve the evidence-native loop and avoid expanding scope beyond the active issue.

## Development workflow

1. Start from `main` or the current stacked dependency branch.
2. Keep changes small, reversible, and tied to an issue.
3. Do not commit generated local state such as `runs/`, `.openchrome/`, `.omc/`, `target/`, or `examples/evidence-dashboard/dashboard-data.json`.
4. Include verification evidence in every PR.
5. Use decision-style commit messages that explain why the change exists and what was tested.

## Required verification

Run the relevant targeted tests plus the standard checks:

```bash
cargo fmt --check
cargo test
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
cargo clippy --all-targets --all-features -- -D warnings
```

When changing MVP run behavior, also run:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
```

Record the generated run id and relevant artifact inspection commands in the PR.

## Scope rules

Do not add these unless an issue explicitly authorizes them:

- Playwright;
- cloud services;
- auth/accounts;
- database/server infrastructure;
- plugin systems;
- native packaging;
- marketplace concepts;
- claims that Ouroforge replaces Godot or any mature engine.

## PR checklist

- [ ] Issue scope is named.
- [ ] Verification commands and outputs are included.
- [ ] Guardrails/non-goals are acknowledged.
- [ ] Generated local state is not committed.
- [ ] Documentation is updated when command behavior changes.
