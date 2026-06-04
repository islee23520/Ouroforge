# Source patch evidence bundle v1 fixture

This fixture is source-like sample data for the Source Mutation Preview v1 / Studio Source Patch Review v1 read-only surface.

`source-patch-evidence-bundle.sample.json` demonstrates the fields exported by Rust and rendered by the dashboard/authoring cockpit:

- patch summary and expected behavior change;
- file-class counts and highest risk;
- risk ids and blocked reasons;
- linked evidence refs;
- sandbox dry-run status and allowlist policy;
- required test command text;
- review status and decision refs;
- forbidden action notices.

The fixture is intentionally inert. It does not apply source patches, merge branches, execute commands, write trusted files, install dependencies, fetch network resources, mutate CI/workflows, or close/modify #1 or #23. Required test commands are display/copy text only and must not become browser-executed actions.

## Focused smoke checks

```bash
cargo test -p ouroforge-core source_patch_evidence_bundle -- --nocapture
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Generated bundle, sandbox, report, dashboard, and run artifacts remain untracked unless a separate issue explicitly scopes a deterministic fixture.
