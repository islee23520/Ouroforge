# Contributor Template Audit v1

Issue: #373, PR unit PA1.7.3.

This audit verifies that the public-alpha contribution guidance, pull request
checklist, and issue templates align with generated-state policy, conservative
public wording, and forbidden-scope boundaries. It is an evidence and governance
artifact only; it does not implement product behavior, CI enforcement, release
automation, publication, repository visibility changes, source patch apply,
browser trusted writes, command bridges, or launch actions.

## Reviewed surfaces

| Surface | Purpose | Audit result |
| --- | --- | --- |
| `CONTRIBUTING.md` | Contributor workflow, verification commands, public-alpha checklist, generated-state guardrails. | Pass: names issue/PR-unit workflow, generated-state exclusions, broad verification, wording audit, and #1/#23 preservation. |
| `.github/PULL_REQUEST_TEMPLATE.md` | PR drift lock, non-goals, public-alpha contributor guardrails, verification checklist. | Pass: requires expected files, authorized behavior, non-goals, generated-state review, wording audit, and #1/#23 preservation. |
| `.github/ISSUE_TEMPLATE/bug_report.yml` | Local MVP bug intake with evidence and safety boundaries. | Pass: asks for reproducible evidence and blocks secrets, sensitive generated artifacts, source apply, command bridges, releases, and visibility changes. |
| `.github/ISSUE_TEMPLATE/feature_request.yml` | Scoped feature intake tied to roadmap/design-gate buckets. | Pass: requires non-goals, evidence, roadmap bucket, generated-state boundary, and maintainer triage for dependencies or forbidden capability areas. |
| `.github/ISSUE_TEMPLATE/public_readiness.yml` | Readiness/governance/demo-evidence tasks without launch execution. | Pass: preserves no visibility change, no publication, no overstatement, #1/#23 preservation, and no sensitive public detail. |
| `.github/ISSUE_TEMPLATE/security_report.yml` | Public-safe routing for security-sensitive reports. | Pass: requests only high-level routing details and forbids exploit details, secrets, private paths, private screenshots, sensitive generated artifacts, and security-guarantee claims. |
| `.github/ISSUE_TEMPLATE/config.yml` | Issue-template routing. | Pass: blank issues stay disabled and security-sensitive contact link routes to the safe security report template. |

## Wording audit

The public-facing contribution/template surfaces intentionally contain terms such
as source apply, browser trusted writes, command bridges, release publication,
visibility changes, production-ready, compatibility-stable, secure sandbox, and
Godot replacement only as explicit negations, guardrails, non-goals, or wording
audit examples.

Acceptable matches are limited to:

- warning contributors not to request or claim forbidden capabilities;
- requiring maintainer triage for dependency or design-gate scope;
- explaining that public readiness and security routing do not authorize launch,
  release, visibility, source apply, browser command execution, or security
  guarantees.

## Generated-state audit

Contributor and template guidance consistently treats local outputs as generated
or sensitive unless explicitly fixture-scoped:

- `runs/`, `target/`, `.openchrome/`, `.omc/`, `.omx/`, `.claude/`, dashboard
  exports, screenshots, launch reports, and local tool output remain untracked.
- Issue templates ask reporters not to post secrets, private paths, private
  screenshots, or unsanitized generated artifacts.
- The PR template requires `git status --short --ignored` review when generated
  or local artifacts may be affected.

## Verification commands

PA1.7.3 should be verified with:

```bash
gh issue view 373 --repo shaun0927/Ouroforge --json number,state,title,updatedAt
gh issue view 1 --repo shaun0927/Ouroforge --json number,state,title
gh issue view 23 --repo shaun0927/Ouroforge --json number,state,title
ruby -e 'require "yaml"; ARGV.each { |f| YAML.load_file(f); puts "parsed #{f}" }' .github/ISSUE_TEMPLATE/*.yml
python3 - <<'PY'
from pathlib import Path
checks = {
    'CONTRIBUTING.md': ['Public-alpha contribution checklist', 'Generated demo, run, dashboard, screenshot'],
    '.github/PULL_REQUEST_TEMPLATE.md': ['Public-alpha contributor guardrails', 'git status --short --ignored'],
    '.github/ISSUE_TEMPLATE/security_report.yml': ['Do not post exploit details', 'does not authorize source patch apply'],
}
for path, terms in checks.items():
    text = Path(path).read_text()
    missing = [term for term in terms if term not in text]
    if missing:
        raise SystemExit(f'{path} missing {missing}')
    print(f'{path}: required terms present')
PY
grep -RInE "Godot replacement|production-ready|compatibility-stable|secure sandbox|source apply|auto-apply|auto-merge|browser trusted write|command bridge|local server bridge|native export ready|plugin runtime ready|release publication|visibility changes|security guarantee" CONTRIBUTING.md .github docs/contributor-template-audit-v1.md || true
git diff --check
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo audit
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git status --short --ignored
```

Matches from the grep command are acceptable only when they are conservative
boundary statements, explicit negations, non-goals, or wording-audit examples.

## Known gaps and boundaries

- This audit does not open GitHub issue forms in a browser; static validation is
  by YAML parsing and required-term checks.
- The repository currently has `bug` and `enhancement` labels available in live
  GitHub label inventory; the security-routing template therefore avoids adding
  a new missing label.
- This audit does not change public visibility, publish a release, announce a
  launch, add dependency policy automation, or alter product/runtime behavior.
