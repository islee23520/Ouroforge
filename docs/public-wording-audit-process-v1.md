# Public Wording Audit Process v1

Status: **repeatable process for public-facing wording changes**.

This process implements the follow-up audit documentation for issue #374
PA1.8.2. It is documentation-only and does not change repository visibility,
release/package behavior, launch status, product behavior, source apply, browser
trusted writes, native export, plugin runtime, hosted/cloud/auth behavior, or
support commitments.

## When to run

Run this process before opening or merging a PR that changes public-facing text,
including:

- `README.md`, `CONTRIBUTING.md`, `SECURITY.md`;
- docs under `docs/` that describe maturity, launch, public alpha, security,
  release/versioning, source mutation, Studio/dashboard surfaces, examples, or
  roadmap status;
- issue/PR templates and public response snippets;
- example README files that users may treat as capability documentation.

## Steps

1. **Run the wording scan** from `docs/public-wording-guardrail-v1.md` against
   the changed public-facing paths, or the whole public docs surface when the PR
   is a broad governance update.
2. **Classify every match** as one of:
   - conservative negation;
   - boundary/non-goal;
   - future/design gate;
   - ambiguous/current claim.
3. **Rewrite ambiguous/current claims** before merge. Prefer local MVP,
   pre-release, readiness candidate, evidence-only, read-only, manual decision,
   or future design-gate language.
4. **Record evidence** in the PR body: command, changed paths, match count or
   examples, classification summary, and replacements made.
5. **Run generated-state audit** with `git status --short --ignored` and confirm
   generated/local roots remain ignored unless explicitly fixture-scoped.
6. **Confirm protected issues** #1 and #23 remain open.

## Replacement patterns

| If text implies... | Prefer... |
| --- | --- |
| present Godot parity/replacement | local-first MVP, bounded prototype, not a Godot replacement |
| present production readiness | pre-release, local demo, readiness candidate |
| stable compatibility/API promise | current contract, fixture-backed schema, no compatibility promise yet |
| secure sandbox guarantee | documented trust boundary, no secure-sandbox guarantee |
| source apply / auto-merge authority | preview/review evidence only; later governance required |
| browser trusted writes or command bridge | read-only browser surface or generated draft/evidence |
| native export/plugin/marketplace availability | deferred design gate or out-of-scope capability |
| launch/release/publication automation | separate manual maintainer decision |
| support/security SLA | no support SLA; documented reporting and limitations |

## PR evidence template

```markdown
Wording audit:
- Command: `<scan command>`
- Paths: `<changed public-facing paths>`
- Matches: `<count or examples>`
- Classifications: `<negation / boundary / future gate / rewritten ambiguous>`
- Replacements: `<summary or none>`
- Generated-state audit: `<git status --short --ignored summary>`
- Protected issues: #1 open; #23 open
```

## Merge rule

A PR that leaves an ambiguous/current claim must not merge unless a separate
explicit governance issue authorizes that claim and records the visibility,
release, security, support, and product-scope consequences.
