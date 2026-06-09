# Tool Accessibility, Internationalization, Themes, and Keyboard Scope & Contract v1

Issue: #2086 — Era N, Milestone 84

Status: **scope and contract accepted for implementation planning**. This documentation-only milestone defines how Ouroforge may add tool accessibility, internationalization, themes, and keyboard-first interaction to the local Phoenix LiveView Studio without changing the trusted write model.

## Goal

Make the Studio easier to use across accessibility needs, locales, visual themes, and keyboard-only workflows while preserving Ouroforge's agent-first default. Human preferences captured by these surfaces are intervention-as-evidence when they can affect generated or trusted outputs; they are never a raw bypass, a new data plane, or a mandatory human dependency.

## Final Implementation Scope

- Define the gated path reused by every write-affecting accessibility, i18n, theme, or keyboard intervention.
- Record the intervention-as-evidence invariant for preference changes, locale/template copy choices, keyboard remapping proposals, and accessibility constraints.
- Preserve the local-first CLI fallback for the autonomous loop and every affected capability.
- Keep Studio read + gated-write: local control/presentation only, with Rust retaining data-plane validation, evidence, provenance, evaluator, review/apply, scene/source-apply, and artifact semantics.

## Reused gated paths

| Capability area | Studio capture | Required reused path | Trusted effect only after |
| --- | --- | --- | --- |
| Accessibility preferences | Reduced motion, high contrast, text scale, screen-reader labels, focus-order constraints | Evaluator/accessibility constraint gate, evidence/provenance, review/apply when artifacts change | Rust validates the constraint and records evidence; artifact changes still require review/apply or scene/source-apply |
| Internationalization | Locale selection, copy variant, translation proposal, missing-string report | Existing localization proposal/validation, evaluator, evidence/provenance, review/apply | Rust validates placeholder integrity, completeness, and provenance |
| Themes | Theme token choice, contrast adjustment, color-scheme proposal | Accessibility/visual evaluator gates, evidence/provenance, review/apply | Contrast and visual checks pass and current refs are fresh |
| Keyboard | Shortcut preference, keybinding proposal, focus traversal issue, command palette intent | UI/UX flow validation, evaluator, evidence/provenance, review/apply when config/artifacts change | Rust validates deterministic action maps and blocks conflicts/stale refs |
| In-product help for these areas | Copyable command/help text and read-only status panels | Read-only rendering of Rust-owned docs/evidence; gated-write only for submitted proposals | No trusted write occurs from help rendering itself |

No row authorizes a raw Studio write. If a captured value can affect trusted artifacts, sources, scenes, evaluator verdicts, release readiness, or generated evidence, it is a proposal/constraint/directive until the Rust-owned gates record it.

## Intervention-as-evidence invariant

Every human intervention in this milestone is optional and evidence-shaped. A captured preference, locale fix, theme adjustment, keyboard shortcut, or accessibility constraint must include author/source, target ref, base hash or run/task id when applicable, captured value, validation status, and evidence/provenance links.

Missing, stale, malformed, unsupported, or unverifiable interventions fail closed. They may remain pending, rejected, or blocked, but they do not broaden Studio authority and they do not block the autonomous loop when no human participates.

## Read + gated-write Studio posture

Phoenix LiveView Studio may render accessible controls, locale selectors, theme previews, keyboard navigation, copyable CLI commands, validation messages, and evidence/status panels. It may capture a bounded proposal/constraint/directive and route it to Rust gates. It may not write trusted artifacts, ledgers, evidence, scenes, sources, release decisions, merge decisions, evaluator verdicts, or source-apply records directly.

The Studio surface must not introduce a browser command bridge. Commands shown for accessibility/i18n/theme/keyboard workflows are copyable references or Rust-owned CLI paths, not hidden execution authority.

## Two-plane invariant

| Plane | Owns | M84 responsibility | Forbidden leakage |
| --- | --- | --- | --- |
| Rust data plane | Artifact truth, schemas, deterministic validation, evaluator decisions, review/apply, scene/source-apply, evidence, provenance, CLI fallback | Validate and record any write-affecting accessibility, i18n, theme, or keyboard proposal | None; Rust remains the trusted writer and validator |
| Elixir/OTP + Phoenix LiveView control/presentation plane | Local single-user rendering, UI state, capture, routing, supervision, status presentation | Make the tools perceivable, operable, understandable, themeable, localizable, and keyboard accessible | Artifact semantics, direct mutation, evaluator truth, ledger/evidence writes, hidden command bridges |

Elixir can make the local Studio more usable. It cannot make a locale complete, a theme valid, an action map trusted, a proposal accepted, an artifact applied, a release ready, or a fun/taste verdict true.

## Local-first CLI fallback

The CLI fallback is mandatory and sufficient. A fresh checkout must be able to run the full autonomous loop without Phoenix, a browser, a database, a hosted service, a remote worker, or human input. Studio accessibility/i18n/theme/keyboard affordances improve local operation but are not prerequisites for completion.

For every write-affecting Studio capture path, downstream implementation must document the equivalent Rust-owned validation or inspection path and prove that zero-human autonomous execution still completes when no preference/proposal is submitted.

## Guardrails

- Agent-first default preserved: accessibility, locale, theme, and keyboard intervention is opt-in and never required for the loop to progress.
- Every human intervention is a validated, recorded proposal, constraint, or directive routed through review/apply, scene/source-apply, evaluator, evidence, and provenance gates.
- No raw write bypasses gates, determinism, or audit.
- Studio surfaces are read + gated-write only.
- Two-plane: Rust is the data plane for artifact truth, validation, determinism, and semantics; Elixir/OTP plus Phoenix LiveView are the local control and presentation plane.
- Local-first: a fresh checkout can run the full loop through the CLI without Studio.
- Hosted, multi-user, collaborative, or real-time remote Studio remains Layer-3 DEFER.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- Governance anchors #1 and #23 remain open.

## Explicit Non-Goals

- Raw human writes to artifacts, ledgers, evidence, scenes, sources, release, merge, deploy, evaluator, or source-apply state.
- Making accessibility/i18n/theme/keyboard input mandatory for autonomous progress.
- Hosted, multi-user, collaborative, real-time remote Studio, account systems, or a new data store.
- Elixir/Phoenix ownership of artifact semantics, localization validation, accessibility scoring, action-map validation, evaluator truth, or provenance ledgers.
- Automated fun/taste verdicts, release go/no-go, auto-apply, auto-merge, reviewer bypass, signing, publishing, or deployment.

## PR Decomposition

- PR 1: this contract document.
- Downstream PRs: implementation, demo, scenario coverage, and governance handoff must reference this contract and prove no new write path.

## Over-Engineering Checklist

- [x] No capability beyond M84 accessibility/i18n/themes/keyboard scope.
- [x] No speculative cluster, hosted, collaborative, or multi-user behavior.
- [x] Existing gates are reused; no parallel write path is introduced.
- [x] Phoenix LiveView scope remains minimal, local, and read + gated-write.
- [x] No new data store; Rust kernel ledger/evidence remains source of truth.

## Drift-Prevention Checklist

- [x] Agent-first default preserved; interventions are opt-in and never required.
- [x] Every intervention routes through existing gates; never raw bypass.
- [x] Rust = data plane; Elixir/Phoenix LiveView = local control + presentation.
- [x] Hosted/multi-user collaborative Studio deferred; CLI fallback intact.
- [x] Fun/taste and release go/no-go remain human; #1 and #23 remain open.

## Language Boundary

Documentation only for this issue. The contract records decisions for later Rust and Elixir implementation: Rust owns validation, evidence, provenance, determinism, and artifact semantics; Elixir/Phoenix may render/capture/route only.

## Critical Risk Review

- Raw-bypass risk: mitigated by requiring all write-affecting preferences, translations, theme changes, and keybindings to become validated and recorded proposals, constraints, or directives before trusted effects.
- Autonomy regression: mitigated by requiring a zero-human CLI fallback and forbidding mandatory waits on human accessibility/i18n/theme/keyboard input.
- Presentation-plane leakage: mitigated by forbidding Elixir/Phoenix artifact semantics, validation, evaluator truth, or trusted writes.
- Scope creep: mitigated by keeping hosted and collaborative Studio deferred and limiting this contract to local single-user control/presentation.

## Definition of Done

- This contract exists in docs and is indexed.
- Downstream M84 issues can reference the gated path, intervention-as-evidence invariant, two-plane boundary, and local-first CLI fallback.
- #1 and #23 remain open.
- Verification passes.
