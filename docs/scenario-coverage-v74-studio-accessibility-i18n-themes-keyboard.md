# Scenario Coverage v74 — Tool Accessibility, Internationalization, Themes, and Keyboard

Coverage v74 locks Era N M84 Studio accessibility, internationalization, themes, and keyboard behavior with regression assertions for keyboard navigation, screen-reader labels, gettext-style local labels, contrast themes, gated preference routing, no raw bypass, and autonomous-first fallback.

## Boundary

- Accessibility, locale, theme, and keyboard preferences are opt-in and never required for the autonomous loop.
- Studio surfaces are read + gated-write; they render ARIA labels, focus order, localized labels, theme tokens, keyboard shortcuts, copyable CLI references, and gate status only.
- Every write-affecting preference is intervention-as-evidence and routes through existing Rust data-plane gates before any trusted effect.
- Elixir/OTP and Phoenix LiveView are local control and presentation only; Rust remains the data plane for validation, determinism, evidence, provenance, review/apply, scene/source-apply, and artifact semantics.
- No raw artifact, ledger, evidence, locale file, theme file, scene, source, release, merge, deploy, auto-apply, or reviewer-bypass authority exists in Elixir.
- No hosted, multi-user, collaborative, real-time remote Studio, new data store, or browser command bridge is introduced.
- CLI fallback remains sufficient, and a no-human run completes without waiting.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- Governance anchors #1 and #23 remain open.

## Regression Rows

| Row | Locks |
| --- | --- |
| `a11y-labels-and-focus-order-present` | Screen-reader labels and deterministic focus order cover the Studio panels. |
| `gettext-labels-do-not-change-behavior` | Locale changes localize labels while preserving semantic actions and fallback behavior. |
| `themes-meet-contrast-and-remain-presentational` | Theme tokens meet contrast requirements and carry no trusted write authority. |
| `keyboard-shortcuts-are-deterministic-and-non-conflicting` | Shortcuts are unique per scope and expose copy/review actions without command bridges. |
| `human-preference-routes-through-rust-gates` | Preference capture routes through a Rust-owned evaluator gate shape and requires later review/apply. |
| `no-raw-bypass-from-elixir-studio-surface` | Raw bypass strings, direct writes, trusted Studio authority, command bridges, and Elixir-owned semantics fail closed. |
| `loop-completes-without-human-input` | The autonomous default path completes with no human surface or wait. |
| `coverage-v74-boundaries` | The suite records local-first, two-plane, read + gated-write, no-new-store, no-hosted-collab, and #1/#23 governance boundaries. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
