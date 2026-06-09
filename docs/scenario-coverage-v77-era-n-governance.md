# Scenario Coverage v77 — Era N Governance Regression Suite

Coverage v77 locks the Era N governance refresh after Milestones 82-86. It is a governance regression suite over documentation and checked-in evidence references; it adds no runtime behavior and no new write path.

## Boundary

- Era N is complete only on merged evidence for M82-M86 plus this M87 governance refresh.
- Newcomer/adoption UX improves time-to-first-verified-game structurally: guided front door, templates, first-run docs, accessibility, human-grade Studio, local delivery, and generated smoke evidence.
- Accessibility and onboarding never bypass gates, determinism, evidence/provenance, or review/apply.
- Studio remains local-first and single-user; hosted/multi-user/collaborative Studio remains Layer-3 DEFER.
- Rust remains the data plane; Elixir/OTP + Phoenix LiveView remains control + presentation only.
- Studio is read + gated-write: every write-affecting human intervention is a validated, recorded proposal/constraint/directive/correction/amendment/takeover/handback/review envelope through existing gates.
- The autonomous loop completes without human input; Studio and human intervention are optional.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- #1 and #23 remain open.

## Regression Rows

| Row | Locks |
| --- | --- |
| `era-n-complete-on-merged-evidence` | M82-M86 completion is recorded only with issue/PR/scenario coverage evidence. |
| `adoption-ux-path-is-structural-not-marketing` | Time-to-first-verified-game is assessed as a local gate-backed path, not a no-code or release-readiness promise. |
| `a11y-never-bypasses-gates` | Accessibility/i18n/theme/keyboard changes remain presentation/control improvements and never artifact truth. |
| `studio-local-first-defer-hosted` | Local single-user Studio remains the only implemented Studio delivery; hosted collaboration is deferred. |
| `two-plane-boundary-preserved` | Rust owns validation/truth; Elixir/Phoenix renders/captures/routes only. |
| `no-raw-bypass-or-command-bridge` | Raw bypass, direct Elixir writes, command bridges, new stores, and release/deploy paths stay forbidden. |
| `autonomous-loop-does-not-wait-for-human` | CLI fallback and autonomous loop remain sufficient without Studio or human input. |
| `governance-anchors-remain-open` | #1 and #23 remain open after the refresh. |

## Verification

```bash
grep -qi "Era M\|Era N" docs/roadmap.md || true
cargo build --workspace --jobs 2
```
