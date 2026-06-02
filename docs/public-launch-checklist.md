# Public launch checklist

Status: **conditional go for a manual public-visibility decision after #217 refresh PRs merge**.

This checklist is the handoff artifact for public-readiness review. It does not publish the repository, change visibility, announce a release, or promise compatibility.

## Merge prerequisites

Historical readiness remediation has already landed:

1. #52 — license and security policy.
2. #53 — README, architecture, contribution, and roadmap hardening.
3. #54 — scoped public issue templates.
4. #55 — public demo media, fresh-clone smoke evidence, and dependency/security audit output.
5. #18.2 PR — final documentation hardening and launch checklist.

Authoring Loop v2 refresh prerequisites for issue #217:

1. #241 / AL2.8.1 — public-readiness smoke and dependency audit evidence.
2. #242 / AL2.8.2 — demo media and documentation drift refresh.
3. AL2.8.3 — this launch-checklist reconciliation PR.

## Evidence gate

Before changing visibility, re-run and record:

```bash
gh issue view 217 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- run seeds/engine-expansion-v1-demo.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
cargo clippy --all-targets --all-features -- -D warnings
cargo audit
```

A passing gate means:

- #217, #1, and #23 remain open until their own close criteria are met;
- the fresh clone or clean latest-main worktree can run the MVP and Engine Expansion v1 demos;
- browser smoke succeeds for all configured workers;
- Scenario DSL results pass;
- dashboard and cockpit smoke tests pass;
- the Rust lockfile has no reported RustSec vulnerabilities;
- generated artifacts remain untracked.

## Manual decision checklist

- [ ] All historical public-readiness PRs and #217 refresh PRs above are merged into `main`.
- [ ] `LICENSE` is present and license choice is confirmed.
- [ ] `SECURITY.md` contains an acceptable vulnerability-reporting path for the visibility date.
- [ ] README quickstart succeeds from a fresh clone.
- [ ] `docs/public-demo-evidence.md` screenshots still match the current UI.
- [ ] `.github/ISSUE_TEMPLATE` renders correctly in GitHub.
- [ ] No generated `runs/`, `target/`, `.openchrome/`, `.omc/`, `.omx/`, `.claude/`, or `dashboard-data.json` files are tracked.
- [ ] No secrets, private screenshots, machine-local paths, or private issue links are tracked.
- [ ] Maintainers explicitly approve the repository visibility change outside the code PR.

## Current #217 reconciliation snapshot

Recorded during AL2.8.3 on 2026-06-02:

- #217 remained OPEN before final issue audit.
- #1 remained OPEN and untouched.
- #23 remained OPEN and untouched.
- `git ls-files` found no tracked `runs/`, `target/`, `.openchrome/`, `.omc/`, `.omx/`, `.claude/`, or `examples/evidence-dashboard/dashboard-data.json` paths.
- `.github/ISSUE_TEMPLATE` contained `bug_report.yml`, `feature_request.yml`, `public_readiness.yml`, and `config.yml`.
- Focused dashboard/cockpit syntax and smoke checks passed after screenshot/label refresh.
- Public wording scan found conservative boundary language only: no Godot replacement, production readiness, hosted/cloud guarantee, compatibility stability, or automatic public-release claim was added.
- Repository visibility was not changed.

The final public visibility decision remains a manual maintainer action outside #217.

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
