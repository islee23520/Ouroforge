# Security response playbook v1

Status: **alpha governance playbook**. This document defines how maintainers
classify and privately coordinate suspected security reports for Ouroforge during
alpha. It does not create a bounty, support SLA, production security guarantee,
hosted service guarantee, release process, repository visibility change, GitHub
settings mutation, or automated advisory workflow.

## Scope boundary

Use this playbook for suspected vulnerabilities in the local-first Ouroforge MVP
and its public alpha documentation. It is documentation/governance only:

- it defines what maintainers should treat as a security report;
- it defines the contact and temporary private coordination path;
- it keeps exploit details, secrets, local paths, and private screenshots out of
  public issues;
- it aligns public wording with `SECURITY.md` and trust-boundary docs;
- it keeps #1 and #23 open unless a separate explicit governance issue says
  otherwise.

Out of scope for this playbook: production security certification, hosted/cloud
security operations, bug bounty terms, guaranteed response times, compatibility
promises, native export security, plugin marketplace security, source patch
apply authorization, browser trusted writes, command bridges, release
publication, or repository visibility changes.

## What counts as a security report

Treat a report as security-sensitive when it plausibly involves any of these
areas, even if impact is uncertain:

| Area | Security-sensitive examples | Non-security examples |
| --- | --- | --- |
| Local file writes | Writing outside documented generated roots; overwriting source, secrets, shell config, or repository metadata unexpectedly | A documented generated `runs/` artifact is created during an expected smoke run |
| Browser automation boundary | Browser path/profile leakage, unintended command bridge, trusted file write from dashboard/cockpit, CDP evidence exposing secrets | A local Chrome path must be configured with `OUROFORGE_CHROME` |
| Source preview / sandbox boundary | Preview artifact claims apply authority, sandbox dry-run writes outside sandbox, source patch evidence bypasses review/stale/hash checks | A preview fixture remains inert and read-only |
| Generated artifacts | Committed secrets, private screenshots, local absolute paths, token-like values, or private issue links in generated output | Ignored local build output remains untracked |
| Dependencies / supply chain | Vulnerable dependency, unexpected install/network command, lockfile tampering, build-script mutation | `cargo audit` reports no advisories |
| Public communication | Claiming secure sandbox, production readiness, support SLA, or compatibility guarantee without authorization | Conservative alpha wording with clear non-goals |

When in doubt, handle the initial report privately until maintainers decide what
can be public.

## Contact and private coordination path

Until a dedicated public security contact is published, use the reporting path in
`SECURITY.md`:

1. Ask reporters to avoid public exploit details, secrets, tokens, local paths,
   private screenshots, or private issue links.
2. If only GitHub issues are available, ask for a minimal public issue that says
   a private security report is needed, without technical details.
3. Move details to a private maintainer channel controlled by repository
   maintainers.
4. Record only non-sensitive coordination metadata publicly: affected area,
   current status (`received`, `triaging`, `needs-info`, `mitigated`,
   `not-applicable`), and whether a public follow-up issue/PR exists.
5. Keep any reproducer, screenshot, generated run, browser profile, `.omx`,
   `.omc`, `.openchrome`, or local path artifact out of git unless a separate
   sanitized fixture-scoped issue explicitly authorizes it.

A useful private report should include the affected commit/branch, exact command
or workflow, expected impact, reproduction steps that do not expose secrets, and
whether local file writes, browser automation, generated artifacts, dependencies,
or source preview/sandbox boundaries are involved.

## Source preview, sandbox, and browser-boundary examples

Security-sensitive examples:

- A dashboard or authoring cockpit path can write trusted source files directly
  from the browser.
- A preview, review, stale guard, sandbox, or evidence artifact accepts unknown
  `applyCommand`, `mergeCommand`, `browserCommandBridge`, or hidden command
  fields.
- A source sandbox dry-run writes outside the documented sandbox/generated root.
- A stale or mismatched source patch can be treated as ready without accepted
  review and current target evidence.
- Generated dashboard data includes secrets, private screenshots, private issue
  links, or machine-local paths that should not be public.

Not security by itself:

- A documented local command writes ignored run evidence under `runs/`.
- A dashboard or cockpit display is read-only and escapes untrusted text.
- A preview or sandbox fixture is committed only as inert, fixture-scoped
  evidence with no apply authority.

## Alignment references

- `SECURITY.md` defines the current public reporting language and local execution
  boundary.
- `docs/evidence-fidelity-trust-boundary-v1.md` defines evidence fidelity and
  trust-boundary language.
- `docs/source-mutation-threat-model-v1.md` and
  `docs/source-apply-threat-model-refresh-v1.md` define source mutation/source
  apply threat-model boundaries.
- `docs/source-mutation-sandbox-boundary-v1.md` defines sandbox dry-run limits.
- `docs/source-apply-worktree-boundary-v1.md` defines trusted worktree boundary
  expectations.
- `docs/public-demo-smoke-evidence-policy-v1.md` defines generated-state and
  public demo evidence retention boundaries.

## Conservative public wording

Allowed public wording:

- "suspected security report received";
- "alpha local-first security review";
- "private coordination path";
- "best-effort triage during MVP development";
- "no hosted service, account system, or production support guarantee".

Avoid public wording that promises:

- a bug bounty;
- a response-time SLA;
- production readiness;
- compatibility stability;
- secure sandboxing for arbitrary untrusted content;
- browser trusted writes are safe;
- source patch apply is authorized;
- native export, plugin marketplace, hosted/cloud, or release security support.
