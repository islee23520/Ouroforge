# Docs Link and Wording Audit PA1.5.3

Status: **final docs information-architecture audit** for issue #371 PA1.5.3.

This audit records the final link, wording, stale-command, and generated-state
pass after the README information-architecture refresh and `docs/README.md`
navigation index. It is documentation-only evidence. It does not authorize a
repository visibility change, public launch, release publication, source apply,
browser trusted writes, command bridges, dependency/CI mutation, or production
support/security guarantee.

## Scope audited

Audited public-facing entry points and top-level docs navigation:

- `README.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `docs/*.md`
- `docs/README.md`

The audit preserves detailed milestone contracts as references. It does not
delete or rewrite milestone contracts.

## Link audit

Local Markdown links across `README.md`, `CONTRIBUTING.md`, `SECURITY.md`, and
`docs/*.md` were checked for missing target files. Result: **pass**.

Command used:

```bash
python3 - <<'PY'
from pathlib import Path
import re, sys
files=[Path('README.md'), Path('CONTRIBUTING.md'), Path('SECURITY.md')] + sorted(Path('docs').glob('*.md'))
missing=[]
for p in files:
    text=p.read_text(errors='replace')
    for m in re.finditer(r'(?<!!)\[[^\]]+\]\(([^)]+)\)', text):
        link=m.group(1).strip()
        if not link or link.startswith(('#','http://','https://','mailto:')): continue
        if link.startswith('app://'): continue
        if re.match(r'^[a-zA-Z]+:', link): continue
        target=link.split('#',1)[0]
        if target and not (p.parent / target).exists():
            missing.append(f'{p}:{m.start()}: {link}')
if missing:
    print('\n'.join(missing))
    sys.exit(1)
print('all local markdown link targets exist')
PY
```

README and `docs/README.md` anchor links were also checked after PA1.5.2 and
passed.

## Wording guardrail audit

The public wording scan was run over public-facing docs and examples. Matches are
accepted only when they are conservative boundary statements, explicit negations,
non-goals, or wording-audit examples.

Command used:

```bash
grep -RInE "Godot replacement|Godot parity|production-ready|production ready|commercial-release ready|ship-ready|compatibility-stable|stable public engine API|secure sandbox|sandbox guarantee|source apply ready|auto-apply|auto-merge|autonomous repair|browser trusted write|command bridge|local server bridge|native export ready|desktop/mobile export|installer|app-store ready|plugin runtime ready|extension marketplace|third-party code loading|hosted service|cloud runtime|multi-user auth|autonomous launch|public release automation|go-live automation|support SLA|guaranteed support|security guarantee" README.md CONTRIBUTING.md SECURITY.md docs examples .github || true
```

Result: **pass**. Matches remain in allowed contexts such as:

- explicit non-goals and negations in `README.md`, `SECURITY.md`, and
  public-alpha/security docs;
- trust-boundary language for read-only browser surfaces, no command bridge, no
  source apply, and no secure-sandbox guarantee;
- wording guardrail/audit examples that intentionally list forbidden phrases;
- roadmap or future-capability references that explicitly avoid current
  capability claims.

## Stale command audit

The current public quickstart and PR verification commands remain aligned with
repository commands used by this issue:

- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo audit`
- `cargo run -p ouroforge-cli -- seed validate seeds/platformer.yaml`
- `cargo run -p ouroforge-cli -- project validate examples/project-workspace-fixtures/valid`
- `cargo run -p ouroforge-cli -- project init .omx/tmp/project-scaffold-smoke --template minimal-2d`
- `cargo run -p ouroforge-cli -- run ...`
- `cargo run -p ouroforge-cli -- evidence list ...`
- `cargo run -p ouroforge-cli -- journal show ...`
- `cargo run -p ouroforge-cli -- mutation list ...`
- `cargo run -p ouroforge-cli -- compare ...`
- `cargo run -p ouroforge-cli -- dashboard export ...`
- `node --check examples/evidence-dashboard/dashboard.js`
- `node examples/evidence-dashboard/dashboard.test.cjs`
- `node --check examples/authoring-cockpit/cockpit.js`
- `node examples/authoring-cockpit/cockpit.test.cjs`

No command in the README/docs navigation refresh introduces install, publish,
release, deploy, credential, network, destructive cleanup outside the documented
`.omx/tmp/project-scaffold-smoke` path, browser command bridge, or source apply
authority.

## Generated-state audit

Generated/local state remains excluded from source changes. The final audit uses:

```bash
git status --short --ignored
```

Allowed ignored roots for this issue are `.claude/`, `.omc/`, `.omx/`,
`.openchrome/`, `runs/`, and `target/`. No generated dashboard data, screenshots,
run artifacts, sandbox worktrees, local tool state, or build output should be
tracked by this issue.

## Closure checklist mapping

- README is organized around what Ouroforge is, what works today, quickstart,
  core loop, demos, safety model, non-goals, roadmap, contributor guide, docs
  map, repository map, and generated state.
- `docs/README.md` provides the expanded docs navigation layer.
- Milestone contracts remain available as references.
- Link and wording audits pass.
- Broad Rust/Node/security verification is required before PR merge and issue
  closure.
