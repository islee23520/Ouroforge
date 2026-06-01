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
| README | Blocked | README is only a title and one-line tagline. | Add accurate private-MVP positioning, prerequisites, setup, demo, artifact inspection, and maturity boundaries. |
| License | Remediated by #46 | `LICENSE` adds MIT terms. | Confirm license choice before public visibility changes. |
| Architecture docs | Blocked | No durable architecture doc is tracked. | Document Seed → Run → Ledger/Evidence → Evaluator → Journal → Mutation → UI boundaries. |
| Contribution guide | Blocked | No `CONTRIBUTING.md` is tracked. | Add contribution workflow, test expectations, stacked PR guidance, and scope rules. |
| Security posture | Remediated by #46 | `SECURITY.md` documents reporting expectations and local Chrome/browser execution boundaries. | Replace temporary private reporting guidance with a dedicated public security contact before launch. |
| Dependency posture | Needs hardening | Dependencies are small and pinned by `Cargo.lock`, but no public dependency policy/audit notes are documented. | Document dependency review and run a formal vulnerability audit before launch. |
| Demo quality | Partial | MVP command can run locally and produce artifacts; UI examples are static and still require manual command/export steps. | Add demo walkthrough, screenshots/recordings, and known limitations. |
| Issue templates | Blocked | No `.github/ISSUE_TEMPLATE` files are tracked. | Add bug/feature/security-adjacent templates with scope boundaries. |
| Roadmap clarity | Blocked | No public roadmap is tracked. | Add realistic roadmap and non-goals; avoid Godot-replacement claims. |
| Example reproducibility | Partial | `cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4` works locally with Chrome. | Document Chrome/`OUROFORGE_CHROME`, local server behavior, and expected artifacts. |
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
- [ ] Add `CONTRIBUTING.md` with verification commands and scope rules.
- [ ] Expand `README.md` with accurate MVP positioning, prerequisites, quickstart, demo, and artifact inspection.
- [ ] Add architecture docs for artifact contracts and UI boundaries.
- [ ] Add roadmap/non-goals that explicitly avoid Godot-replacement claims.
- [ ] Add `.github/ISSUE_TEMPLATE` files.
- [ ] Add screenshots or demo recording references for runtime, dashboard, and cockpit.
- [ ] Run a fresh-clone smoke test and record exact output.
- [ ] Run dependency/security audit and record exact output.
- [ ] Confirm no generated local state or private paths are tracked.
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
