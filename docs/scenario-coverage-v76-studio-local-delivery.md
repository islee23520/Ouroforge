# Scenario Coverage v76 — Studio Packaging and Local Delivery Regression Suite

Coverage v76 locks Era N Milestone 86 Studio packaging and local delivery behavior after the local delivery demo. The suite covers the local Studio + Rust kernel install/run UX, generated built-artifact smoke evidence, optional human packaging intervention through existing gates, and the autonomous no-human CLI fallback.

## Boundary

- Studio packaging is local-first and single-user only; hosted, multi-user, collaborative, remote, release-channel, installer, updater, app-store, signing, deploy, publish, and cloud behavior remain out of scope.
- The autonomous CLI loop completes without Studio and without human input.
- Studio surfaces remain read + gated-write: every write-affecting human action is captured as intervention-as-evidence and routed to existing Rust gates.
- Rust remains the data plane for artifact truth, validation, determinism, evidence/provenance, review/apply, scene/source-apply, evaluator verdicts, and artifact semantics.
- Elixir/OTP + Phoenix LiveView is control + presentation only: local supervision, read-model rendering, form capture, routing envelopes, and display feedback.
- Built-artifact smoke output under `runs/` is generated evidence only. It is not trusted source, not a release artifact, not a package store, and not a bypass.
- No trusted Elixir write authority, direct artifact write, raw-bypass, command bridge, new data store, hosted collaboration, mandatory-human dependency, no-code/product overclaim, fun/taste automation, or release go/no-go automation is introduced.
- Governance anchors #1 and #23 remain open.

## Regression Rows

| Row | Locks |
| --- | --- |
| `local-delivery-manifest-install-run-ux` | The package manifest exposes Rust build, Mix compile, CLI run, and optional local Studio run commands. |
| `built-artifact-smoke-generated-only` | The smoke checks the Rust binary and compiled Studio app, then records generated evidence only under `runs/`. |
| `human-package-write-routes-through-gates` | Optional human packaging constraints are intervention-as-evidence queued for existing Rust evaluator gates. |
| `no-raw-bypass-or-command-bridge` | Trusted writes, direct artifact writes, command bridges, new stores, and raw bypass requests fail closed. |
| `mandatory-human-regression-fails-closed` | The no-human CLI fallback remains sufficient and Studio use is never required. |
| `hosted-release-scope-drift-fails-closed` | Hosted collaboration, signing, release, deploy, publish, installer, updater, app-store, and cloud delivery remain out of scope. |
| `demo-observe-intervene-fallback-remains-conservative` | The M86 demo observes generated evidence, queues one gated intervention, and proves autonomous fallback without overclaiming. |
| `coverage-v76-boundaries` | The suite records read + gated-write, intervention-as-evidence, two-plane, local-first, generated-only smoke evidence, hosted defer, and #1/#23 governance boundaries. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
