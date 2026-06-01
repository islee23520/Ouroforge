# Public Open-Source Readiness Audit — Issue #18.1

Date: 2026-06-01
Branch: `issue-18-1-release-readiness-audit`
Decision: **NO-GO for public open-source release today**

## Decision summary

Ouroforge has a working local evidence-native MVP path, but it is **not ready for public open-source release** because public-facing governance, security, licensing, contribution, roadmap, and demo documentation are incomplete. The correct next step is private remediation, not repo visibility change or launch messaging.

## Evidence reviewed

- Repository tracked files via `git ls-files`.
- `README.md`.
- Workspace/package manifests and lockfile.
- Runtime and UI examples under `examples/`.
- Existing Seeds under `seeds/`.
- Cargo verification and MVP run commands.
- Local generated runtime/tool state status.

## Readiness checklist

| Area | Status | Evidence | Required remediation |
| --- | --- | --- | --- |
| README | Remediated by #47 | README now documents MVP status, prerequisites, quickstart, verification, repository map, generated state, and maturity boundaries. | Keep screenshots/demo references current after #49. |
| License | Remediated by #46 | `LICENSE` adds MIT terms. | Confirm license choice before public visibility changes. |
| Architecture docs | Remediated by #47 | `docs/architecture.md` documents Seed → Run → Ledger/Evidence → Evaluator → Journal → Mutation → UI boundaries. | Keep architecture docs aligned with future feature issues. |
| Contribution guide | Remediated by #47 | `CONTRIBUTING.md` documents workflow, verification, generated-state rules, and scope boundaries. | Keep PR checklist current with future commands. |
| Security posture | Remediated by #46 | `SECURITY.md` documents reporting expectations and local Chrome/browser execution boundaries. | Replace temporary private reporting guidance with a dedicated public security contact before launch. |
| Dependency posture | Remediated by #49 | `docs/public-demo-evidence.md` records `cargo audit` output against `Cargo.lock`. | Re-run audit before any later public release decision. |
| Demo quality | Remediated by #49 | `docs/assets/demo/` contains runtime, evidence dashboard, and authoring cockpit screenshots; `docs/public-demo-evidence.md` documents capture commands and limitations. | Keep media current when UI behavior changes. |
| Issue templates | Remediated by #48 | `.github/ISSUE_TEMPLATE` defines bug, scoped feature, and public-readiness templates with evidence and guardrail fields. | Keep templates aligned with support policy and SECURITY.md. |
| Roadmap clarity | Remediated by #47 | `docs/roadmap.md` documents current status, public-readiness work, direction, and non-goals. | Keep roadmap conservative until public launch decision. |
| Example reproducibility | Remediated by #49 | `docs/public-demo-evidence.md` records Chrome/`OUROFORGE_CHROME`, generated artifacts, and fresh-clone smoke commands. | Re-run fresh-clone smoke before public visibility changes. |
| Secrets/private paths | Pass with caveat | No tracked `.openchrome/`, `.omc/`, or generated `runs/`; verification found only untracked local tool state. | Keep `.openchrome/`, `.omc/`, `runs/`, and generated dashboard data out of commits. |

## Fresh-clone MVP smoke expectation

A fresh clone of the stacked branch should be able to run:

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
```

Known environment requirement: local Chrome must be available at a standard path or through `OUROFORGE_CHROME`.

## No-go blockers

1. **License and security policy missing.** Public users cannot know usage rights or report vulnerabilities safely.
2. **README and demo instructions are insufficient.** A fresh public user cannot discover prerequisites, run the MVP, or inspect artifacts from the README alone.
3. **Contribution and issue intake are undefined.** Public issue/PR flow would create ambiguity and scope drift.
4. **Architecture/roadmap maturity boundaries are undocumented.** Without explicit positioning, the project risks being overstated as a mature engine or Godot replacement.
5. **Demo media and screenshots are missing.** Public release would be difficult to evaluate without visual proof and limitations.

## Launch checklist if blockers are resolved later

- [x] Add `LICENSE` with chosen OSS license.
- [x] Add `SECURITY.md` with vulnerability reporting and local execution caveats.
- [x] Add `CONTRIBUTING.md` with verification commands and scope rules.
- [x] Expand `README.md` with accurate MVP positioning, prerequisites, quickstart, demo, and artifact inspection.
- [x] Add architecture docs for artifact contracts and UI boundaries.
- [x] Add roadmap/non-goals that explicitly avoid Godot-replacement claims.
- [x] Add `.github/ISSUE_TEMPLATE` files.
- [x] Add screenshots or demo recording references for runtime, dashboard, and cockpit.
- [x] Run a fresh-clone smoke test and record exact output.
- [x] Run dependency/security audit and record exact output.
- [x] Confirm no generated local state or private paths are tracked.
- [ ] Make a separate manual visibility decision; do not automate publication.

## Guardrail results

- No automatic publication performed.
- No compatibility promise added.
- No maturity/marketing claim added.
- No Godot-replacement claim added.
- No new product feature added during this audit.

## Follow-up blocker issues

Created private blockers:

- #46 — Add OSS license and security policy before public release.
- #47 — Harden README, architecture, contribution, and roadmap docs for public onboarding.
- #48 — Add public issue templates and support/scope boundaries.
- #49 — Add public demo media and reproduce fresh-clone smoke evidence.

## Final recommendation

**NO-GO.** Keep Ouroforge private until the blockers above are resolved and a fresh clone can follow public docs from zero context to a verified MVP run.
