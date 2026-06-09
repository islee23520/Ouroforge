# Era N Adoption-UX Assessment

Issue #2099 records the Era N adoption assessment after Milestones 82-86 merged. The assessment is evidence-derived from the checked-in contracts, demos, and scenario coverage; it is not a public launch claim, usability study, not a no-code product claim, hosted Studio claim, or not a release-readiness claim.

## Evidence Chain

| Milestone | Evidence | What it proves |
| --- | --- | --- |
| M82 Non-Developer Front Door | #2078-#2081; Scenario Coverage v72 | A newcomer/non-developer can enter a guided brief, preview a proposal, and route generation through Rust-owned gates without a trusted write. |
| M83 Onboarding/Templates/Docs | #2082-#2085; Scenario Coverage v73 | A fresh user has template gallery, first-run steps, sample seeds, and in-product docs that preserve the gated path. |
| M84 Accessibility/i18n/Themes/Keyboard | #2086-#2089; Scenario Coverage v74 | Keyboard navigation, screen-reader labels, local labels, themes, and preference routing improve access without bypassing determinism or gates. |
| M85 Human-Grade Studio | #2090-#2094; Scenario Coverage v75 | One local Phoenix LiveView surface composes observation, intervention, and authoring while every write remains read + gated-write. |
| M86 Local Packaging/Delivery | #2095-#2098; Scenario Coverage v76 | The local Studio + Rust kernel install/run path, generated smoke evidence, and no-human CLI fallback are locked. |

## Newcomer Time-to-First-Verified-Game Assessment

The Era N newcomer path is now a bounded local path rather than a hand-assembled expert path:

1. Start from the guided front door or a template/sample seed.
2. Follow first-run docs to run the Rust build and the local Studio/Mix app when desired.
3. Generate or select a proposal/template.
4. Validate through the Rust data plane and existing review/apply/evaluator/evidence gates.
5. Inspect the result in Studio or continue entirely through the CLI.

Evidence-backed expectation for a fresh local checkout with toolchains installed: a newcomer can reach a first verified local game/proposal path in a short first-run session by following the M83/M86 commands and templates. The assessment deliberately avoids a minute-level marketing promise because hardware, dependency cache state, and user familiarity vary. The regression requirement is instead structural: the path must remain copyable, local, deterministic, and gate-backed, and the autonomous CLI fallback must complete with zero human input.

## No-Bypass and Accessibility Assessment

Accessibility and onboarding lowered friction without weakening the core:

- Accessibility/i18n/theme/keyboard preferences are routed as local control/presentation state and do not become artifact truth.
- Human actions remain proposal/constraint/directive/correction/amendment/takeover/handback/review evidence envelopes.
- Every write-affecting action routes through existing Rust-owned review/apply, scene/source-apply, evaluator, evidence/provenance, and related gates.
- Generated smoke output under `runs/` is evidence only, not trusted source or release output.
- No raw bypass, command bridge, direct Elixir artifact write, new data store, hosted collaboration, signing/release/deploy/publish path, or mandatory human dependency was introduced.

## Reaffirmed Boundaries

- Agent-first default remains preserved: humans may intervene, but the loop completes without them.
- Rust remains the data plane for truth, validation, determinism, evidence/provenance, and trusted writes.
- Elixir/OTP + Phoenix LiveView remains the local single-user control/presentation plane.
- Studio is read + gated-write, not raw-write, not a command bridge, and not a hosted collaboration product.
- Hosted/multi-user/collaborative Studio remains Layer-3 DEFER.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- #1 and #23 remain open governance anchors.
