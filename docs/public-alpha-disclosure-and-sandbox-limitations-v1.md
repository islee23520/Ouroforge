# Public Alpha Responsible Disclosure and Sandbox Limitations v1

Status: **public-alpha disclosure and sandbox limitation guidance** for issue
#372 PA1.6.2.

This document gives fresh-clone readers and maintainers one conservative path for
security reports and sandbox/dry-run expectations during the local-first public
alpha. It is documentation-only. It does not create a bounty, response-time SLA,
production security guarantee, secure-sandbox guarantee, repository visibility
change, release process, GitHub Security Advisory automation, browser trusted
write path, command bridge, or source apply authority.

## Responsible disclosure guidance

Ouroforge is a pre-release, local-first MVP. Suspected vulnerabilities should be
handled with the least public sensitive detail necessary.

Report privately when practical if a finding involves or may involve:

- secrets, tokens, credentials, private issue links, private screenshots, or
  machine-local paths;
- unexpected writes outside documented generated roots;
- browser automation, dashboard/cockpit read-only boundaries, or possible command
  bridges;
- source preview, sandbox dry-run, stale-target, review, rollback, or source
  apply boundaries;
- dependency, build-script, network, install, publish, signing, upload, or deploy
  behavior;
- public wording that implies production readiness, secure sandboxing, support
  commitments, or authorized source apply.

Until maintainers publish a dedicated security contact, follow `SECURITY.md`:
use a private maintainer channel where possible. If GitHub issues are the only
available path, file a minimal public issue that says a private security report
is needed, without exploit details, secrets, tokens, local paths, or private
screenshots.

A useful private report includes:

- affected commit, branch, command, or document;
- expected impact and the trust boundary involved;
- reproduction steps that avoid exposing secrets or private data;
- whether generated artifacts, browser automation, dependencies, or source
  preview/sandbox evidence are involved;
- a sanitized suggestion for mitigation, if known.

## Maintainer disclosure flow

Security disclosure is manual and maintainer-owned:

1. **Receive privately when practical.** Acknowledge without promising a response
   time, bounty, advisory, release, or production support outcome.
2. **Sanitize public records.** Do not quote secrets, tokens, exploit payloads,
   local private paths, private screenshots, or private issue links.
3. **Classify the affected boundary.** Use `docs/security-response-playbook-v1.md`
   for severity triage and public-safe status wording.
4. **Reproduce in a clean local worktree.** Keep generated runs, browser profiles,
   screenshots, `.omx`, `.omc`, `.openchrome`, `runs/`, `sandbox/`, and `target/`
   artifacts untracked unless a separate sanitized fixture scope explicitly
   authorizes them.
5. **Mitigate narrowly.** Use a scoped PR for documentation or code fixes. Do not
   use a disclosure report to add launch, release, source apply, browser trusted
   writes, command bridges, or dependency workflow changes.
6. **Decide public disclosure separately.** Public issue updates, advisory drafts,
   release notes, or visibility changes require separate maintainer/governance
   decisions.
7. **Close with evidence.** Record PRs, verification, generated-state audit,
   known gaps, and whether public disclosure remains deferred.

## Sandbox and dry-run limitations

Ouroforge docs use "sandbox" and "dry-run" only in bounded local-evidence
contexts. These terms do not mean arbitrary untrusted-code isolation.

Current sandbox/dry-run boundaries:

- Source patch sandbox dry-runs may operate only on generated local worktree
  copies under documented sandbox roots.
- Sandbox evidence can support review, but it is not trusted source apply and
  does not authorize writes to the maintainer worktree.
- Required test commands must be normalized argv data matched by the repository
  allowlist and forbidden-command classifier before execution.
- Network, install/bootstrap, credential/cloud-auth, dependency mutation,
  CI/workflow mutation, destructive filesystem, Git apply/merge/rebase/push,
  browser bridge, and local server command bridge commands remain forbidden.
- Browser, dashboard, and Studio surfaces may display sandbox evidence read-only;
  they must not invoke local commands or write trusted files.
- Generated sandbox reports, copied worktrees, run artifacts, screenshots,
  dashboard exports, and local tool state remain untracked unless a separate
  fixture-scoped issue explicitly authorizes a tiny deterministic artifact.

Known limitations and non-goals:

- no secure sandbox for arbitrary user content;
- no hosted/cloud isolation, remote execution worker, account model, or
  multi-tenant boundary;
- no protection for running untrusted seeds, scene files, browser pages, shell
  commands, or project workspaces on an operator's machine;
- no source patch apply, branch merge, auto-apply, auto-merge, or reviewer bypass
  authority;
- no package publish, binary release, signing, upload, deployment, or launch
  automation;
- no production security monitoring, support SLA, compatibility guarantee, or
  advisory timeline.

## Public wording guardrail

Allowed wording:

- "local-first public alpha trust boundary";
- "responsible disclosure guidance";
- "private coordination when sensitive details are involved";
- "sandbox dry-run evidence";
- "no secure-sandbox guarantee";
- "generated local artifacts remain untracked".

Avoid wording that claims or implies:

- production-ready, compatibility-stable, or secure for arbitrary untrusted code;
- Godot replacement, native export readiness, plugin runtime, marketplace, or
  hosted/cloud support;
- browser trusted writes, local command bridge, local server bridge, source apply
  ready, autonomous launch, auto-merge, auto-apply, support SLA, or security
  guarantee.

Use `docs/public-wording-guardrail-v1.md` and
`docs/public-wording-audit-process-v1.md` for scans before public-facing docs are
merged.

## Verification expectations

PRs touching disclosure or sandbox limitation wording should record:

- `gh issue view 372 --repo shaun0927/Ouroforge`;
- `gh issue view 1 --repo shaun0927/Ouroforge` and `gh issue view 23 --repo
  shaun0927/Ouroforge`;
- focused wording scan for production/security/sandbox/source-apply overclaims;
- generated-state audit with `git status --short --ignored`;
- broad Rust/Node/security gates when issue-level closure or broad docs changes
  require them.
