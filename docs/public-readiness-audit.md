# Public Open-Source Readiness Audit — Issue #18.1

Date: 2026-06-01; updated 2026-06-02
Audit branch: `issue-18-1-release-readiness-audit`; hardening branch: `issue-18-2-public-readiness-hardening`; Authoring Loop v2 refresh branch: `al2-8-1-public-readiness-evidence`
Decision: **CONDITIONAL GO for manual public-visibility review after remediation PRs merge**

## Decision summary

Ouroforge now has the documentation, governance, demo evidence, fresh-clone smoke path, and dependency audit records needed for a manual public-visibility review once the stacked remediation PRs merge. The correct next step is still a separate maintainer visibility decision, not automated publication or launch messaging.

## Evidence reviewed

- Repository tracked files via `git ls-files`.
- `README.md`.
- Workspace/package manifests and lockfile.
- Runtime and UI examples under `examples/`.
- Existing Seeds under `seeds/`, including `seeds/platformer.yaml` and `seeds/engine-expansion-v1-demo.yaml`.
- Cargo verification, MVP run, and Engine Expansion v1 demo run commands.
- Local generated runtime/tool state status.

## Readiness checklist

| Area | Status | Evidence | Required remediation |
| --- | --- | --- | --- |
| README | Remediated by #47 | README now documents MVP status, prerequisites, quickstart, verification, repository map, generated state, and maturity boundaries. | Keep screenshots/demo references current after #49. |
| License | Remediated by #46 | `LICENSE` adds MIT terms. | Confirm license choice before public visibility changes. |
| Architecture docs | Remediated by #47 | `docs/architecture.md` documents Seed → Run → Ledger/Evidence → Evaluator → Journal → Mutation → UI boundaries. | Keep architecture docs aligned with future feature issues. |
| Contribution guide | Remediated by #47 | `CONTRIBUTING.md` documents workflow, verification, generated-state rules, and scope boundaries. | Keep PR checklist current with future commands. |
| Security posture | Remediated by #46 | `SECURITY.md` documents reporting expectations and local Chrome/browser execution boundaries. | Replace temporary private reporting guidance with a dedicated public security contact before launch. |
| Dependency posture | Refreshed by #217 AL2.8.1 | `docs/public-demo-evidence.md` records current `cargo audit` output against `Cargo.lock`: 1102 advisories loaded, 66 crate dependencies scanned, no vulnerabilities reported. | Re-run audit before any later public release decision. |
| Demo quality | Refreshed by #217 AL2.8.1 | `docs/assets/demo/` contains runtime, evidence dashboard, and authoring cockpit screenshots; `docs/public-demo-evidence.md` now records Platformer and Engine Expansion v1 smoke run ids and limitations. | Review media drift in AL2.8.2 before changing screenshots. |
| Issue templates | Remediated by #48 | `.github/ISSUE_TEMPLATE` defines bug, scoped feature, and public-readiness templates with evidence and guardrail fields. | Keep templates aligned with support policy and SECURITY.md. |
| Roadmap clarity | Refreshed after Asset Pipeline v1 (#342) | `docs/roadmap.md` documents completed Asset Pipeline v1, next milestone candidates, public-readiness work, direction, and non-goals. | Keep roadmap conservative until public launch decision. |
| Example reproducibility | Remediated by #49 | `docs/public-demo-evidence.md` records Chrome/`OUROFORGE_CHROME`, generated artifacts, and fresh-clone smoke commands. | Re-run fresh-clone smoke before public visibility changes. |
| Secrets/private paths | Pass with caveat | No tracked `.openchrome/`, `.omc/`, or generated `runs/`; verification found only untracked local tool state. | Keep `.openchrome/`, `.omc/`, `runs/`, and generated dashboard data out of commits. |

## Fresh-clone MVP smoke expectation

A fresh clone of the stacked branch should be able to run:

```bash
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
```

Known environment requirement: local Chrome must be available at a standard path or through `OUROFORGE_CHROME`. Current clean-worktree refresh passed with Platformer run `runs/run-1780406739942-16401`, Engine Expansion run `runs/run-1780406747216-16614`, dashboard/cockpit Node checks, clippy, and `cargo audit`.

## Resolved blockers

1. **License and security policy** are covered by #46.
2. **README and demo instructions** are covered by #47 and #49.
3. **Contribution and issue intake** are covered by #47 and #48.
4. **Architecture/roadmap maturity boundaries** are covered by #47.
5. **Demo media, fresh-clone smoke, and dependency audit evidence** are covered by #49.

## Launch checklist if blockers are resolved later

- [x] Add `LICENSE` with chosen OSS license.
- [x] Add `SECURITY.md` with vulnerability reporting and local execution caveats.
- [x] Add `CONTRIBUTING.md` with verification commands and scope rules.
- [x] Expand `README.md` with accurate MVP positioning, prerequisites, quickstart, demo, and artifact inspection.
- [x] Add architecture docs for artifact contracts and UI boundaries.
- [x] Add roadmap/non-goals that explicitly avoid Godot-replacement claims.
- [x] Add `.github/ISSUE_TEMPLATE` files.
- [x] Add screenshots or demo recording references for runtime, dashboard, and cockpit.
- [x] Run a fresh-clone or clean-worktree smoke test and record exact output. #217 AL2.8.1 records Platformer and Engine Expansion v1 run ids.
- [x] Run dependency/security audit and record exact output. #217 AL2.8.1 records current `cargo audit` output.
- [x] Confirm no generated local state or private paths are tracked.
- [ ] Make a separate manual visibility decision; do not automate publication. See `docs/public-launch-checklist.md`.

## Guardrail results

- No automatic publication performed.
- No compatibility promise added.
- No maturity/marketing claim added.
- No Godot-replacement claim added.
- No new product feature added during this audit.

## Follow-up blocker issues

Remediation blockers created by #18.1 and addressed by stacked follow-up PRs:

- #46 — Add OSS license and security policy before public release.
- #47 — Harden README, architecture, contribution, and roadmap docs for public onboarding.
- #48 — Add public issue templates and support/scope boundaries.
- #49 — Add public demo media and reproduce fresh-clone smoke evidence.

## Post-Asset Pipeline v1 note

Asset Pipeline v1 completion adds local asset manifest, loading, preview, Studio
inspection, demo refresh, and regression evidence. It does not change this audit
into a launch approval: public visibility remains a separate manual maintainer
decision after the launch checklist is re-run on the intended visibility date.

## Final recommendation

**CONDITIONAL GO for manual public-visibility review after #52, #53, #54, #55, and the #18.2 PR merge.** The repository should not be published automatically. Maintainers should re-run the evidence gate in `docs/public-launch-checklist.md`, confirm no generated or private state is tracked, then make a separate manual visibility decision.
