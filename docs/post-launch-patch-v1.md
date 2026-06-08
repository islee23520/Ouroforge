# Post-Launch Patch, Re-Verify, and Save-Migration Loop v1 Scope and Contract

Issue: **#1844** (#1 Era I Milestone 55)

Status: **Scope contract complete — bounded GO for a local post-launch patch evidence loop; DEFER human/Ring-3 release decisions and market obligations.**

This is a scope/contract document. It adds **no executable patching behavior**, no new pipeline, no release automation, no browser or Studio trusted-write authority, no Steam upload, no save-file mutation, and no new runtime. It defines the contracts, boundaries, and follow-up dependency order for iterating on a launched local desktop/web package while protecting player saves.

#1 remains the roadmap/vision anchor and #23 remains the repo memory/design anchor. This contract preserves both issues as open anchors.

## ADR question

Can Ouroforge define a conservative post-launch patch loop that reuses the existing gates, packaging, provenance, compare, and save contracts so a patch is always re-verified before re-packaging and save compatibility is explicit before release handoff?

## Decision summary

| Area | Decision | Contract |
| --- | --- | --- |
| Patch intake | **GO, bounded** | Treat post-launch changes as source/apply or proposal changes through the existing review/apply/trust-gradient path. No direct browser/Studio/generated trusted writes. |
| Re-verify before re-package | **GO, required** | Every patch candidate re-runs the full declared gate set before packaging: tests, evaluator gates, scenario coverage, comparison/provenance checks, export/package checks, and issue-specific verification. |
| Re-package | **GO, reused** | Reuse the existing local packaging/export contracts, including web/desktop package descriptors, checksums, provenance, and generated-state policy. No new package pipeline. |
| Save migration and compatibility | **GO, bounded** | Define versioned save schema compatibility, forward-only migrations, fixture-scoped migration evidence, rollback/diagnostic handling, and fail-closed incompatibility states. |
| Human/Ring-3 release actions | **DEFER** | Steam account work, code signing, content survey, Release button, support commitments, market demand, and release go/no-go remain human-owned and outside engine scope. |
| Layer-3 cloud/mobile | **DEFER** | Post-launch patches remain local package updates; hosted/cloud/mobile remains outside this milestone. |

The bounded GO authorizes contract definition and the follow-up implementation issues in the dependency sequence. It does not authorize autonomous shipping, auto-merge, or release.

## Goals

- Define the canonical update -> re-verify -> re-package loop for post-launch patches.
- Define save-migration/version-compatibility requirements before a patch can be packaged for release handoff.
- Reuse the existing runtime/evaluator/evolve/compare/provenance/source-apply/export/package/dashboard/cockpit/CLI surfaces; do not define a parallel pipeline.
- Preserve generated runs, packages, migrations, and evidence as untracked unless explicitly fixture-scoped.
- Define dependency order and closure gates: **#1844 scope -> #1845 -> #1846 -> #1847 -> #1848 -> #1849**.
- Preserve #1 and #23 as open governance anchors.

## Non-goals and human/Ring-3 split

Human/Ring-3, out of scope:

- Steam account creation, partner portal work, code signing, content survey, store mutation, upload, publish, and the Steam Release button.
- Release go/no-go, launch timing, support policy, pricing, wishlists, user acquisition, discoverability, community management, and market demand.
- Fun/feel/quality/taste verdicts. The human Era J fun/feel/release judgment remains outside this milestone.

Engine/repo non-goals:

- No executable behavior in this scope issue beyond documentation and regression tests.
- No new runtime, parallel engine, or Godot replacement/parity claim.
- No new patch pipeline; reuse existing gates, packaging, compare, provenance, and review/apply/trust-gradient contracts.
- No direct trusted write from generation, browser, Studio, dashboard, cockpit, Electron, Steamworks, or any JavaScript surface.
- No autonomous apply, auto-merge, self-approval, reviewer bypass, hidden trusted writes, or release bot.
- No automated fun score, quality score, production-ready claim, commercial-readiness claim, or shippability claim.
- No hosted/cloud/mobile Layer-3 capability; post-launch patch packaging remains local package update work.
- No generated runs/assets/builds/migration outputs committed unless explicitly fixture-scoped.

## Patch loop contract

A post-launch patch candidate is a proposed update against an already packaged baseline. The canonical loop is:

1. **Baseline selection.** Identify the current released/local-package baseline by package descriptor, source revision, export fingerprint, provenance bundle, scenario coverage version, and save schema version.
2. **Patch intake.** Accept a patch only through existing source-apply, review/apply/trust-gradient, proposal, or human-authored source paths. Browser, Studio, dashboard, cockpit, Electron, Steamworks, and generated surfaces remain read-only for trusted state.
3. **Targeted change record.** Record patch intent, affected contracts, changed fixtures, save-schema impact, and expected gate coverage.
4. **Full re-verification.** Re-run the full declared gate set before re-packaging. At minimum this includes the issue verification block, Rust tests, clippy, JavaScript dashboard/cockpit checks where relevant, evaluator gates, scenario coverage rows touched by the patch, compare/provenance checks, package/export checks, and save-compatibility checks when saves can be affected.
5. **Regression comparison.** Compare patched outputs against the baseline and classify improvements, unchanged behavior, regressions, inconclusive results, and unsupported checks without hiding failures.
6. **Save compatibility decision.** Evaluate the save-migration/version-compatibility model before package handoff. A patch that changes save shape without a verified migration is blocked.
7. **Re-package.** Reuse the existing local package/export contracts to produce fixture-scoped package descriptors, manifests, checksums, and provenance. Re-package only after re-verification and compatibility are pass/accepted.
8. **Release handoff.** Produce a read-only evidence bundle for human/Ring-3 release decision. The engine does not press Release, upload, sign, or assert market/fun readiness.

A patch **must re-verify before re-packaging**. Re-package evidence without preceding gate evidence is invalid.

## Full gate set reuse

The post-launch loop does not invent a second QA pipeline. It composes existing contracts:

- Rust/local tests, validation, persistence, source-apply, review/apply/trust-gradient, package descriptors, export fingerprints, provenance, compare, and evidence writing.
- Existing evaluator gates, including declared visual/semantic/design-integrity/content/performance/production QA gates where applicable to the changed surface.
- Existing scenario coverage suites, continuing the numbering with **Scenario Coverage v50** for post-launch patch regressions.
- Existing dashboard/cockpit read-only summaries for evidence display and inspection.
- Existing local web/desktop packaging contracts, including generated-state and untracked-artifact policy.
- Existing save/profile/runtime state contracts, extended only by explicit save-migration/version-compatibility follow-up work.

If a future patch requires a new gate, that gate must be introduced as a separate scoped contract before it can be required by the patch loop.

## Save-migration and version-compatibility model

Save compatibility is explicit and fail-closed.

| Concept | Contract |
| --- | --- |
| Save schema version | Every persisted save/profile format has an explicit schema/version identity and compatibility range. |
| Baseline compatibility | A patch records the oldest supported baseline save version and the current target save version. |
| Forward migration | Migrations are forward-only, deterministic, and Rust/local-owned. They validate input, produce a new versioned save, preserve required player progress fields, and emit fixture-scoped evidence. |
| No silent downgrade | Downgrades are not implied. If rollback needs an older save, the rollback path must use explicit backup/diagnostic evidence rather than silent schema rewriting. |
| Incompatibility | Unsupported, malformed, stale, missing, or ambiguous saves fail closed with a user-visible diagnostic and do not produce trusted migrated state. |
| Replay/provenance | Migration evidence links old save hash, new save hash, migration id, target version, validation result, and any compatibility warnings. |
| Conflict policy | Steam/cloud/local conflicts are transport concerns only; Rust/local owns validation and conflict classification before trusted state is accepted. |

A patch that touches save/profile/runtime persistence cannot pass closure unless migration evidence demonstrates compatibility or the patch explicitly proves no save-shape impact.

## Ownership and language boundary

- **Rust/local owns** trusted validation, persistence, source-apply/review/apply/trust-gradient, save migration, compatibility checks, package/export descriptor derivation, provenance, compare, evidence writing, run/project binding, and CLI behavior.
- **TypeScript/JavaScript owns** deterministic runtime behavior, `window.__OUROFORGE__` probe data, in-game UI, browser-local read-only inspection, dashboard/cockpit display, and Electron/Steamworks read-only or bridge surfaces where already scoped.
- **Python may be used** only for temporary local tooling or smoke helpers and must not own core Era I/J contracts.
- No new language/runtime is introduced without explicit issue-level authorization; distributed/Elixir remains NO-GO per ADR #92 (`docs/distributed-elixir-design.md`).

## Dependency order and closure gates

Follow-up sequence:

1. **#1844** — scope and contract (this document).
2. **#1845** — Update Re-Verify and Re-Package Loop v1.
3. **#1846** — Save-Migration and Version-Compatibility v1.
4. **#1847** — Post-Launch Patch Demo v1.
5. **#1848** — Scenario Coverage v50: Post-Launch Patch Regression Suite.
6. **#1849** — Roadmap and #1 Governance Refresh after Post-Launch Patch v1.

Closure gates:

- #1845 must prove patch candidates re-verify through the full declared gate set before re-packaging and must reuse existing package/export evidence.
- #1846 must define deterministic Rust/local save migrations, compatibility ranges, failure diagnostics, and migration evidence without silent trusted writes.
- #1847 must be a deterministic fixture-scoped demo with no live release/upload/signing behavior.
- #1848 must continue Scenario Coverage as **v50** and cover success plus blocked cases: missing re-verification, regression, stale provenance, unsupported save version, malformed migration, generated-artifact drift, and release-authority drift.
- #1849 must refresh roadmap/#1 governance while keeping #1 and #23 open.

## Conservative wording and governance

- This contract defines a bounded post-launch patch loop; it does not claim autonomous shipping or production readiness.
- Patch release, Steam upload, code signing, content survey, Release button, support obligations, and market demand remain human/Ring-3.
- A patch re-verifies through the full gate set before re-packaging.
- Saves migrate forward only with verified compatibility evidence.
- Browser, Studio, dashboard, cockpit, Electron, Steamworks, and generated surfaces remain read-only for trusted state.
- High-risk and source-affecting changes never auto-apply.
- Public wording must avoid production-ready, commercial-readiness, quality/fun, Godot replacement/parity, autonomous shipping, auto-merge, self-approval, reviewer bypass, and hidden trusted write claims.
- Generated runs/artifacts/builds/migration outputs remain untracked unless explicitly fixture-scoped.
- #1 remains open.
- #23 remains open.
