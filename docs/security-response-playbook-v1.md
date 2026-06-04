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

## Severity triage model

Use severity labels for coordination and prioritization only. They are not a
promise of response time, support coverage, public advisory timing, or production
security posture.

| Severity | Use when | Default coordination stance | Public wording |
| --- | --- | --- | --- |
| Critical | A report plausibly allows unexpected trusted file writes outside documented roots, secret exposure, arbitrary command execution, or bypass of source preview/sandbox/review boundaries with meaningful local impact | Keep details private, identify an owner, reproduce in a clean worktree, and prepare a minimal mitigation PR before public details | "Critical suspected local boundary issue under private triage" |
| High | A report shows a credible local boundary bypass, unsafe generated-state leak, dependency/build-script risk, or browser automation issue with limited preconditions | Keep exploit detail private, reproduce, and decide whether a sanitized public issue can track remediation | "High-severity alpha security report under maintainer triage" |
| Medium | A report affects conservative public wording, generated-state hygiene, dashboard/cockpit read-only assumptions, or hardening gaps without a direct trusted-write/command path | Public tracking may be acceptable after removing sensitive details | "Security hardening or governance follow-up" |
| Low | A report is documentation ambiguity, missing caveat, non-sensitive local path mention, or defense-in-depth wording improvement | Public issue/PR is usually acceptable if no exploit details or private data are included | "Security documentation or defense-in-depth cleanup" |
| Not applicable | The report describes expected generated local state, documented local Chrome requirements, or behavior outside supported alpha scope | Close or redirect with conservative explanation | "Not a security issue under the current local-first MVP boundary" |

If severity is unclear, start private and downgrade only after a maintainer has
reviewed whether public details could expose users, secrets, or local paths.

## Advisory and disclosure flow

This flow is manual and maintainer-owned. It does not create or trigger GitHub
Security Advisories, releases, repository setting changes, publication, or
notification automation.

1. **Receive privately** — acknowledge receipt when practical, without promising
   a response-time SLA.
2. **Scope and sanitize** — identify affected commit, command, artifact, or doc;
   remove secrets, local paths, screenshots, tokens, private issue links, and
   exploit details from any public record.
3. **Classify severity** — use the severity triage model above and record the
   least-sensitive classification publicly only when safe.
4. **Reproduce safely** — use a clean worktree and ignored/generated paths; do
   not commit repro outputs unless a separate sanitized fixture-scoped issue
   authorizes them.
5. **Mitigate in a scoped PR** — keep changes narrow, reviewable, and aligned to
   the affected boundary; do not use the security response issue to add product
   features or release automation.
6. **Decide disclosure** — after mitigation, decide whether a public issue,
   release note, advisory draft, or documentation note is appropriate. If a
   GitHub Security Advisory is warranted, create it manually as a separate
   maintainer decision with sanitized details.
7. **Close with evidence** — record PRs, verification, residual gaps, generated
   state audit, and whether public disclosure remains deferred.

Disclosure can stay private when public details would expose a still-unfixed
trusted-write path, command bridge, secret, local path, private screenshot, or
source preview/sandbox bypass.

## No bounty, no SLA, no production guarantee

Ouroforge alpha security handling is best-effort. Public wording must not imply:

- a bug bounty or monetary reward;
- guaranteed response, remediation, disclosure, or advisory timelines;
- production security support;
- compatibility-stable security commitments;
- secure sandboxing for arbitrary untrusted content;
- hosted/cloud security operations;
- safe browser trusted writes;
- authorized source patch apply or source merge;
- release, package, native export, or plugin marketplace security coverage.

Acceptable wording:

- "Maintainers triage alpha security reports on a best-effort basis.";
- "Please avoid posting exploit details publicly.";
- "A public advisory, if needed, is a separate maintainer decision.";
- "No bounty, response-time SLA, or production security guarantee is offered for
  the current MVP."

## Closure checklist for security-response PRs

Before closing a security-response governance issue, confirm:

- [ ] Fixed PR units were merged in the issue-defined order.
- [ ] No repository visibility, release, publication, advisory, or settings
      automation was added.
- [ ] No product feature, browser trusted write, command bridge, source apply,
      native export, plugin runtime, hosted/cloud, credentialed workflow, or
      marketplace behavior was introduced.
- [ ] Wording scan shows bounty/SLA/production/security guarantee terms only in
      explicit no/avoid/boundary contexts.
- [ ] Generated run, dashboard, screenshot, local tool, and private coordination
      artifacts remain untracked unless separately fixture-scoped.
- [ ] `cargo audit` and broad repository verification passed, or any unavailable
      check is explicitly documented.
- [ ] #1 and #23 remain open.
