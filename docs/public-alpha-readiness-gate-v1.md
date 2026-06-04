# Public Alpha Readiness Gate v1

Status: **readiness checklist and report format** for issue #376 PA1.10.1.

This gate determines whether Ouroforge is prepared for a separate manual
maintainer-controlled public visibility decision. It does not change repository
visibility, publish packages, launch Ouroforge, automate releases, or authorize
production-ready, compatibility-stable, Godot replacement, secure-sandbox,
native-export, plugin-runtime, source-apply, marketplace, or support-SLA claims.

## Gate outcome vocabulary

Use exactly one outcome in a readiness report:

| Outcome | Meaning | Allowed next action |
| --- | --- | --- |
| `prepared-for-manual-review` | Required local checks, wording audits, generated-state audits, and governance anchors pass. | A maintainer may separately consider a manual visibility decision outside this repository change. |
| `blocked` | One or more required checks fail or required evidence is missing. | Keep visibility unchanged and file/fix blockers before another gate run. |
| `deferred` | Checks are intentionally not run because the timing, environment, or scope is not appropriate. | Record why, keep visibility unchanged, and rerun later. |

A `prepared-for-manual-review` outcome is not a launch approval. It only means
the repository evidence is ready for a later human decision record.

## Required readiness checklist

Run the checklist from a clean latest-`main` worktree or fresh clone unless the
report explains why a different clean checkout was used.

| Area | Required evidence | Pass condition | Blocker examples |
| --- | --- | --- | --- |
| Live issue state | `gh issue view 376`, `gh issue view 1`, and `gh issue view 23`. | #376 is open while the gate is being recorded; #1 and #23 remain open. | #1 or #23 is closed, replaced, or modified without a separate governance decision. |
| Rust formatting | `cargo fmt --check`. | Command exits 0. | Formatting drift. |
| Rust tests | `cargo test`. | Command exits 0. | Failing unit, integration, or doc test. |
| Rust lint | `cargo clippy --all-targets --all-features -- -D warnings`. | Command exits 0. | New clippy warning or lint failure. |
| Dependency/security audit | `cargo audit`. | Command exits 0 and records the advisory database scan summary. | Reported RustSec vulnerability or unavailable audit without explanation. |
| Dashboard syntax and smoke | `node --check examples/evidence-dashboard/dashboard.js` and `node examples/evidence-dashboard/dashboard.test.cjs`. | Both commands exit 0. | Syntax error, failed smoke assertion, or unsafe browser-write implication. |
| Authoring cockpit syntax and smoke | `node --check examples/authoring-cockpit/cockpit.js` and `node examples/authoring-cockpit/cockpit.test.cjs`. | Both commands exit 0. | Syntax error, failed smoke assertion, or command-bridge/trusted-write drift. |
| Demo/onboarding docs | README, `docs/public-readiness-audit.md`, `docs/public-launch-checklist.md`, `docs/public-demo-evidence.md`, and roadmap pointers. | A reader can find local-only quickstart, evidence, maturity boundaries, and manual visibility boundary. | Missing public-readiness pointer, stale demo instructions, or overclaiming launch status. |
| Security/trust docs | `SECURITY.md`, trust-boundary docs, issue templates, and public issue/PR intake docs. | Sensitive reporting, generated-state, browser-read-only, no-source-apply, and no-support-SLA boundaries are explicit. | Public secret handling gap, implied secure sandbox, hidden execution, or support guarantee. |
| Wording audit | Scan changed public-facing docs/templates for forbidden claims. | Matches are conservative negations, non-goals, or boundary statements only. | Claims of production readiness, Godot replacement, compatibility stability, secure sandbox, native export, plugin runtime, marketplace, source apply, hosted/cloud support, or launch approval. |
| Generated-state audit | `git status --short --ignored` plus tracked-path check for generated roots. | Only ignored/local generated roots appear; no generated run, dashboard, screenshot, local tool, or temp-project artifacts are tracked unless fixture-scoped. | Tracked `runs/`, `target/`, `.openchrome/`, `.omc/`, `.omx/`, `.claude/`, dashboard export, private screenshot, or machine-local path. |
| Launch boundary | Manual decision docs and launch-governance docs. | The report states visibility changes remain external/manual and no release/publish automation was added. | Repository settings changed, publish workflow added, or announcement/decision text presented as already approved. |

## Focused audit commands

The gate runner should include the broad command set above and these focused
audits when producing the PA1.10.2 execution report:

```bash
python3 - <<'PY'
from pathlib import Path
forbidden = [
    'production-ready', 'compatibility-stable', 'secure sandbox',
    'Godot replacement', 'native export', 'plugin runtime', 'marketplace',
    'source apply', 'support SLA', 'launch approval', 'public visibility changed',
]
paths = [
    Path('README.md'),
    Path('docs/roadmap.md'),
    Path('docs/public-readiness-audit.md'),
    Path('docs/public-launch-checklist.md'),
    Path('docs/public-alpha-readiness-gate-v1.md'),
    Path('SECURITY.md'),
]
for path in paths:
    text = path.read_text()
    hits = [term for term in forbidden if term.lower() in text.lower()]
    print(f'{path}: {hits or "no direct hits"}')
PY

git ls-files runs target .openchrome .omc .omx .claude \
  examples/evidence-dashboard/dashboard-data.json
```

The wording audit is a review aid, not a blind failure rule: conservative
negations such as "not a Godot replacement" or "does not authorize source apply"
are expected and should be recorded as pass evidence.

## Final readiness report format

Create a report under `docs/` for PA1.10.2 using this structure:

```markdown
# Public Alpha Readiness Final Report v1

Status: **<prepared-for-manual-review | blocked | deferred>** for issue #376 PA1.10.2.

## Scope boundary

- Repository visibility changed: no.
- Release/package/publication automation added: no.
- Product behavior added: no.
- Public launch approved: no.
- #1 open: <yes/no with issue-view evidence>.
- #23 open: <yes/no with issue-view evidence>.

## Command evidence

| Command | Result | Notes |
| --- | --- | --- |
| `gh issue view 376 --repo shaun0927/Ouroforge` |  |  |
| `gh issue view 1 --repo shaun0927/Ouroforge` |  |  |
| `gh issue view 23 --repo shaun0927/Ouroforge` |  |  |
| `cargo fmt --check` |  |  |
| `cargo test` |  |  |
| `cargo clippy --all-targets --all-features -- -D warnings` |  |  |
| `cargo audit` |  |  |
| `node --check examples/evidence-dashboard/dashboard.js` |  |  |
| `node examples/evidence-dashboard/dashboard.test.cjs` |  |  |
| `node --check examples/authoring-cockpit/cockpit.js` |  |  |
| `node examples/authoring-cockpit/cockpit.test.cjs` |  |  |
| `git diff --check` |  |  |
| `git status --short --ignored` |  |  |

## Focused audits

- Demo/onboarding docs:
- Security/trust docs:
- Issue templates:
- Wording scan:
- Generated-state audit:
- Known environment caveats:

## Blockers and non-blockers

| Type | Item | Decision |
| --- | --- | --- |
| Blocker |  |  |
| Non-blocker |  |  |

## Closure rationale

Summarize why the gate is prepared, blocked, or deferred. If prepared, restate
that actual repository visibility remains a separate manual maintainer action
outside this issue and outside any PR.
```

## PA1.10.1 closure checklist

PA1.10.1 is complete when this gate document exists, existing readiness docs
point to it, wording remains conservative, and verification confirms #376/#1/#23
are open. PA1.10.2 remains responsible for running the full gate and recording
final execution evidence.
