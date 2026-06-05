# Scenario Coverage v11: GDD-to-Prototype Regression Suite

Issue: #660. #1 remains open. #23 remains open.

This matrix composes the existing GDD-to-prototype stage contracts into one
fixture-scoped regression suite. The goal is to prove regression detection across
GDD schema/design brief validation, requirement extraction, mechanics/core-loop
mapping, feasibility gates, scaffold plans, scene/level plans, gameplay behavior
plans, asset placeholder/reference plans, scenario/acceptance plans, prototype
task graphs, draft bundles, review/apply, run evidence, evidence journals,
dashboard read models, and Studio inspection.

## Boundary

The suite demonstrates evidence-gated bounded prototype generation only: no
autonomous unrestricted game creation. It does not authorize arbitrary source mutation,
arbitrary script execution, dynamic code loading, plugin loading, browser trusted
writes, command bridges, local server bridges, hidden command execution,
auto-apply, auto-merge, self-approval, uncontrolled asset generation, generated
proprietary assets, production game claims, production-ready claims, current
Godot replacement claims, native export, plugin runtime, marketplace behavior,
hosted/cloud behavior, or account/auth behavior.

Conservative wording checklist: no autonomous unrestricted game creation, no arbitrary source mutation, no generated proprietary asset claim, no production game, no current Godot replacement, no native export, no plugin runtime, no hosted/cloud, no auto-apply, and no auto-merge.

GDD-derived output remains untrusted until Rust/local validation and
review-gated apply. Browser/dashboard/Studio surfaces remain read-only or
draft-only unless a separately scoped Rust/local trusted API owns persistence.
Generated prototype drafts, plans, screenshots, runs, dashboard data, temp
projects, and local tool state remain untracked unless explicitly
fixture-scoped.

## Coverage matrix

| Scenario | Stage | Positive coverage | Negative coverage |
| --- | --- | --- | --- |
| GDD11.design-brief-schema | GDD schema/design brief | valid, partial, blocked | overbroad, unsafe, unsupported |
| GDD11.requirements | Requirement extraction | traceable valid, partial, blocked | missing source, low-confidence without blocker, unsafe boundary |
| GDD11.mechanics-core-loop | Mechanics/core loop | supported, partial, unsupported, deferred, contradictory | overbroad loop, unsupported without recommendation, unsafe boundary |
| GDD11.feasibility | Feasibility gate | pass, fail, defer, downgrade, blocked | missing acceptance, unsupported without risk, unsafe boundary |
| GDD11.scaffold | Project scaffold plan | valid, deferred, stale, blocked | direct write command, generated-root collision, stale without blocker |
| GDD11.scene-level | Scene/level plan | valid, partial, unsupported, stale, blocked | missing requirement, stale without blocker, unsafe ref |
| GDD11.behavior | Gameplay behavior plan | valid, partial, unsupported, script-needed, stale, blocked | script need without blocker, unsupported without blocker, unsafe ref |
| GDD11.assets | Asset placeholder/reference plan | valid, missing, unsupported, stale | missing license/source, proprietary ambiguity, generated root |
| GDD11.scenarios | Scenario/acceptance generation | valid, partial, unsupported, stale, blocked, contradictory blocked | missing evidence, unsupported without blocker, unsafe scenario ref |
| GDD11.task-graph | Prototype task graph | ordered valid, incomplete, stale, blocked | apply without review, cyclic dependency, unsupported scope |
| GDD11.bundle-apply-evidence-read-models | Draft/apply/run/evidence/journal/dashboard/Studio | valid draft bundle, accepted apply, passing run evidence, evidence journal bundle, demo manifest | missing component, self-approval, missing run without blocker, stale journal without blocker |

## Read-model compatibility

- Rust/local validators own trusted validation, generated evidence writing,
  trusted persistence, and CLI contracts.
- Dashboard read models stay display-only/read-only and must not gain trusted
  writes.
- Studio/cockpit inspection stays escaped and read-only; it may display GDD
  planning artifacts but does not run generation, apply changes, or persist
  trusted state.

## Known gaps

- v11 uses deterministic fixtures. It does not claim arbitrary GDD support.
- Unsupported mechanics, missing evidence, stale targets, and blocked
  review/apply cases are expected explicit failures, not hidden generation.
- Asset generation remains out of scope; fixtures use local placeholders and
  manifest references with license/source notes only.
- This matrix does not replace each stage contract test; it prevents coverage
  from hiding inside the end-to-end demo.

## Verification

Focused check:

```bash
cargo test -p ouroforge-core --test scenario_coverage_v11_gdd_to_prototype -- --nocapture
```

Required issue closure checks also include `cargo fmt --check`, `cargo test`,
`cargo clippy --all-targets --all-features -- -D warnings`, dashboard and Studio
Node checks, `git diff --check`, `git status --short --ignored`, and live checks
that #660 is handled while #1 and #23 remain open.

