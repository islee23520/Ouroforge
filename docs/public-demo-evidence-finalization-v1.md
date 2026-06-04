# Public Demo Evidence Finalization v1

Status: **finalization artifact** for issue #370 PA1.4.3.

This document closes the Public Demo Evidence Refresh v1 documentation loop. It
records the current run ids, generated paths, cleanup instructions, guardrails,
and known gaps after PA1.4.1 and PA1.4.2. It does not launch Ouroforge, change
repository visibility, publish packages, add release automation, or claim the
local MVP is production-ready.

## Fixed PR unit evidence

| Unit | Result | Evidence |
| --- | --- | --- |
| PA1.4.1 — Demo evidence inventory and stale reference audit | Complete | `docs/public-demo-evidence-refresh-audit-v1.md` inventories tracked screenshot refs, stale AL2/run-id history, later dashboard/Studio evidence areas, and generated-state policy. |
| PA1.4.2 — Screenshot/evidence refresh | Complete | `docs/assets/demo/runtime-demo.png`, `docs/assets/demo/evidence-dashboard.png`, `docs/assets/demo/authoring-cockpit.png`, and `docs/public-demo-evidence.md` were refreshed from a local latest-`main` capture. |
| PA1.4.3 — Demo evidence docs finalization | This artifact | Current generated paths, cleanup commands, known gaps, guardrails, and verification gates are recorded here. |

## Current generated run ids and paths

PA1.4.2 generated these local evidence paths on 2026-06-04. They are not source
files and must stay untracked:

| Evidence | Generated path | Status |
| --- | --- | --- |
| Platformer local run | `runs/run-1780575535986-2550` | Browser smoke succeeded 4/4; scenario verdict failed on current `objective-contact` assertions. |
| Engine Expansion v1 local run | `runs/run-1780575544692-26537` | Browser smoke succeeded 4/4; scenario verdict failed on current `objective-contact-integration` assertions. |
| Dashboard export | `examples/evidence-dashboard/dashboard-data.json` | Generated from the local `runs/` root for screenshot capture; intentionally ignored/untracked. |
| Runtime screenshot | `docs/assets/demo/runtime-demo.png` | Tracked public demo reference, refreshed from the local static page. |
| Evidence dashboard screenshot | `docs/assets/demo/evidence-dashboard.png` | Tracked public demo reference, refreshed from the local dashboard export and visibly showing failed-verdict drift. |
| Authoring cockpit screenshot | `docs/assets/demo/authoring-cockpit.png` | Tracked public demo reference, refreshed from the local static cockpit. |

The failed scenario verdicts are part of the current evidence. They are not hidden
behind older passing run ids and must not be described as a passing fresh-clone
MVP run unless a later issue produces and records fresh passing evidence.

## Cleanup instructions

After local evidence refresh or review, remove generated local state with:

```bash
rm -rf runs
rm -f examples/evidence-dashboard/dashboard-data.json
rm -rf /tmp/ouroforge-shot-runtime /tmp/ouroforge-shot-dashboard /tmp/ouroforge-shot-cockpit
rm -rf /tmp/ouroforge-shot-runtime-370 /tmp/ouroforge-shot-dashboard-370 /tmp/ouroforge-shot-cockpit-370 /tmp/ouroforge-shot-cockpit-370b
rm -f /tmp/issue370-platformer-run.log /tmp/issue370-engine-run.log /tmp/issue370-dashboard-export.log /tmp/issue370-http.log /tmp/issue370-http-cockpit.log
```

`target/` may also be removed when a fresh build is desired, but it is ordinary
Cargo build output and remains ignored.

## Docs references finalized

- `docs/public-demo-evidence.md` is the main reader-facing evidence page.
- `docs/public-demo-evidence-refresh-audit-v1.md` records the PA1.4.1 stale
  reference audit.
- This file records the PA1.4.3 finalization and closure gate context.
- `docs/public-readiness-audit.md` remains historical public-readiness context;
  it should not be read as a launch approval or as newer evidence than #370.
- `docs/public-alpha-readiness-gate-v1.md` remains the broader manual
  public-visibility review gate.

## Known gaps and non-goals

Known gaps after #370 are intentionally documented instead of fixed in this issue:

- Current Platformer and Engine Expansion v1 scenario verdicts fail on local
  objective-contact assertions, despite browser smoke 4/4 for both runs.
- Demo screenshots are static references, not an interactive hosted demo,
  polished trailer, release artifact, or compatibility proof.
- The dashboard screenshot depends on generated `dashboard-data.json`; that file
  remains out of git.
- The authoring cockpit remains browser read-only for trusted state and does not
  write files, execute commands, upload content, or apply patches.
- Public visibility remains a separate manual maintainer decision.
- No package/binary/crates.io/npm release, signing, upload, deployment, native
  export, plugin runtime, marketplace, hosted/cloud/server/auth feature, source
  patch apply, auto-apply, auto-merge, local command bridge, or production editor
  implementation is included.

## Closure guardrails

Before #370 closes, verify:

- #370 is still open before the final issue comment.
- #1 remains open.
- #23 remains open.
- Fixed PR units merged in order: PA1.4.1, PA1.4.2, PA1.4.3.
- Generated run/dashboard/local tool artifacts remain ignored/untracked.
- Public wording remains conservative: no production-ready, compatibility-stable,
  Godot replacement, autonomous launch, native export, plugin runtime, secure
  sandbox, source apply, or support-SLA claim.
- Repository visibility remains unchanged.

## Verification commands

```bash
gh issue view 370 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo audit
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```
