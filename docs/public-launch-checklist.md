# Public launch checklist

Status: **conditional go for a manual public-visibility decision after stacked readiness PRs merge**.

This checklist is the handoff artifact for issue #18.2. It does not publish the repository, change visibility, announce a release, or promise compatibility.

## Merge prerequisites

Merge these stacked remediation PRs in order before using this checklist for a public visibility decision:

1. #52 — license and security policy.
2. #53 — README, architecture, contribution, and roadmap hardening.
3. #54 — scoped public issue templates.
4. #55 — public demo media, fresh-clone smoke evidence, and dependency/security audit output.
5. #18.2 PR — this final documentation hardening and launch checklist.

## Evidence gate

Before changing visibility, re-run and record:

```bash
cargo fmt --check
cargo test
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
cargo clippy --all-targets --all-features -- -D warnings
cargo audit
```

A passing gate means:

- the fresh clone can run the MVP demo;
- browser smoke succeeds for all configured workers;
- the Scenario DSL result passes;
- dashboard and cockpit smoke tests pass;
- the Rust lockfile has no reported RustSec vulnerabilities;
- generated artifacts remain untracked.

## Manual decision checklist

- [ ] All stacked public-readiness PRs above are merged into `main`.
- [ ] `LICENSE` is present and license choice is confirmed.
- [ ] `SECURITY.md` contains an acceptable vulnerability-reporting path for the visibility date.
- [ ] README quickstart succeeds from a fresh clone.
- [ ] `docs/public-demo-evidence.md` screenshots still match the current UI.
- [ ] `.github/ISSUE_TEMPLATE` renders correctly in GitHub.
- [ ] No generated `runs/`, `target/`, `.openchrome/`, `.omc/`, or `dashboard-data.json` files are tracked.
- [ ] No secrets, private screenshots, machine-local paths, or private issue links are tracked.
- [ ] Maintainers explicitly approve the repository visibility change outside the code PR.

## Launch wording boundaries

Allowed wording:

- “pre-release local MVP”
- “evidence-native game engine experiment”
- “fresh-clone smoke path is documented”
- “public-readiness evidence is recorded”

Avoid wording that claims:

- Godot replacement status;
- compatibility stability;
- production readiness;
- hosted/cloud security guarantees;
- sandboxing arbitrary untrusted content;
- automatic public release or launch approval.

## Rollback / deferral

If any evidence gate fails, keep the repository private and file a blocker issue with:

- failing command;
- exact output;
- affected artifact path;
- whether the failure is environment-specific or repository-specific;
- proposed remediation owner.
