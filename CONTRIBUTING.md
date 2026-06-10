# Contributing to Ouroforge

Ouroforge is a local-first, pre-release MVP. Contributions should preserve the evidence-native loop and avoid expanding scope beyond the active issue or design gate.

Public-alpha preparation is not a public launch. Contributor-facing docs, templates, and checklists must not change repository visibility, publish packages, automate releases, or claim production readiness.

## Development workflow

1. Start from `main` or the current stacked dependency branch.
2. Link every change to an open issue and, when the issue has fixed PR units, work on exactly one unit at a time.
3. Keep changes small, reversible, and reviewable.
4. Do not commit generated local state such as `runs/`, `.openchrome/`, `.omc/`, `.omx/`, `.claude/`, `target/`, screenshots, dashboard exports, or `examples/evidence-dashboard/dashboard-data.json` unless a fixture-scoped issue explicitly authorizes it.
5. Include verification evidence in every PR.
6. Use decision-style commit messages that explain why the change exists and what was tested.

## Required verification

Run the relevant targeted tests plus the standard checks for the files you changed:

```bash
cargo fmt --check
cargo test
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
cargo clippy --all-targets --all-features -- -D warnings
```

For public-alpha readiness, dependency, security, template, or governance changes, also run:

```bash
cargo audit
git status --short --ignored
```

When changing MVP run behavior, also run:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
```

Record the generated run id and relevant artifact inspection commands in the PR. Generated run output remains local/untracked unless a fixture-scoped issue explicitly says otherwise.

## Public-alpha contribution checklist

Before opening a public-alpha PR, confirm:

- The linked issue and PR unit or scoped slice are named in the PR body.
- The exact file list is intentional and matches the authorized scope.
- The PR does not change repository visibility, publish releases, add package/binary publication, or automate launch actions.
- The PR does not add source patch apply, native export, plugin runtime, marketplace, hosted/cloud/server/auth behavior, browser trusted writes, command bridges, auto-merge, auto-apply, or hidden command execution unless a specific design gate and implementation issue authorize that bounded work.
- Public wording does not claim Ouroforge is production-ready, compatibility-stable, a secure sandbox, a Godot replacement, native-export ready, plugin-runtime ready, source-apply ready, or covered by a support/security SLA.
- Generated demo, run, dashboard, screenshot, launch-report, and local tool artifacts remain ignored/untracked unless explicitly fixture-scoped.
- #1 and #23 remain open unless a separate explicit governance decision says otherwise.

## Public-readiness changes

For documentation or release-readiness PRs, include the relevant checklist from `docs/public-launch-checklist.md`. If the change affects the MVP demo path, verify from a fresh clone or clearly state why fresh-clone verification was not applicable.

Use `docs/public-wording-guardrail-v1.md` and the wording audit below for public-facing text:

```bash
grep -RInE "Godot replacement|Godot parity|production-ready|production ready|commercial-release ready|ship-ready|compatibility-stable|stable public engine API|secure sandbox|sandbox guarantee|source apply ready|auto-apply|auto-merge|autonomous repair|browser trusted write|command bridge|local server bridge|native export ready|desktop/mobile export|installer|app-store ready|plugin runtime ready|extension marketplace|third-party code loading|hosted service|cloud runtime|multi-user auth|autonomous launch|public release automation|go-live automation|support SLA|guaranteed support|security guarantee" README.md CONTRIBUTING.md SECURITY.md docs examples .github || true
```

Matches are acceptable only when they are conservative boundary statements, explicit negations, non-goals, or audit examples.

## Scope rules

Do not add these unless an issue explicitly authorizes them:

- Playwright;
- cloud services;
- auth/accounts;
- database/server infrastructure;
- plugin systems or marketplace concepts;
- native packaging or export publication;
- source patch apply or browser trusted writes;
- release, launch, merge, or visibility automation;
- claims that Ouroforge replaces Godot or any mature engine.

## PR checklist

- [ ] Issue scope and PR unit/scoped slice are named.
- [ ] Expected files and authorized behavior are listed.
- [ ] Verification commands and outputs are included.
- [ ] Guardrails/non-goals are acknowledged.
- [ ] Generated local state is not committed.
- [ ] Public wording is conservative and audited when applicable.
- [ ] Documentation is updated when command behavior changes.

## M115 Wording Guard (completion semantics)
Run `node scripts/m115-completion-semantics.test.cjs` (or equivalent) before claiming practical usability.
Forbidden unqualified claims in docs/PR/issue text for runtime/Studio/gameplay: production-ready, commercial-release-ready, secure-sandbox, full Godot parity, Godot replacement, plugin runtime ready, etc.
See docs/product-observed-completion.md.
