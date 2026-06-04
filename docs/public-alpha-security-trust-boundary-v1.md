# Public Alpha Security and Trust Boundary v1

Status: **public-alpha security/trust boundary policy** for issue #372 PA1.6.1.

Ouroforge is a local-first, pre-release MVP. This policy documents the current
trust boundary for public-alpha readiness. It does not create a production
security guarantee, secure-sandbox guarantee, hosted service, repository
visibility change, release process, source apply authority, browser trusted-write
path, command bridge, or support SLA.

## Boundary summary

| Surface | Current trust boundary | Not authorized |
| --- | --- | --- |
| Rust CLI/core | Trusted persistence, validation, generated artifact creation, and local evidence commands. | Hidden command execution, credentialed network/install commands, dependency/CI mutation without separate governance. |
| Browser runtime/demo | Local demo/probe execution for evidence capture. | Trusted source writes, command bridges, local server command execution, hosted/cloud behavior. |
| Evidence dashboard/cockpit | Read-only display over exported/generated JSON; copy-only command text when documented. | File writes, shell execution, source apply, auto-merge, auto-apply, hidden approval. |
| Generated state | Local ignored roots such as `runs/`, `.openchrome/`, `.omc/`, `.omx/`, `.claude/`, `target/`, and dashboard exports. | Tracking generated/local state unless an explicit fixture scope allows it. |
| Source mutation | Preview, review, sandbox, and stale-target evidence may exist as policy/design artifacts. | Trusted source apply to the maintainer worktree until a later explicit governance issue authorizes it. |

## Local-first execution

Ouroforge commands run on the operator's machine. A fresh-clone reader should
treat the project as local development software, not a hosted service. Current
public-alpha readiness does not provide:

- accounts, authentication, authorization, cloud storage, or multi-tenant
  execution;
- remote job runners or hosted browser farms;
- package, binary, installer, signing, upload, or deployment workflows;
- production security monitoring or response-time commitments.

## Browser read-only boundary

Browser-facing examples may inspect local generated artifacts and runtime state.
They must remain read-only for trusted state:

- no browser-side trusted file writes;
- no local command bridge or local server command bridge;
- no hidden command execution;
- no source apply, auto-apply, auto-merge, auto-accept, or reviewer bypass;
- no credential, token, secret, local private path, or private screenshot capture
  beyond explicitly generated local evidence.

If a future issue proposes browser trusted writes or command execution, it must
start with a separate security/trust-boundary design gate before implementation.

## Source apply boundary

Current source-mutation docs and fixtures are preview/review evidence only. They
may describe sandbox dry-runs, stale-target guards, review decisions, and audit
bundles, but they do not authorize trusted source writes. Any future source apply
capability must separately define:

- target file classes and forbidden classes;
- rollback and stale-target checks;
- reviewer separation and evidence links;
- command allowlists and credential/network/install exclusions;
- generated-state and audit-ledger behavior;
- explicit proof that browser surfaces remain read-only or copy-only.

## Forbidden command/network/credential behavior

Public-alpha docs and examples must not introduce or imply authority for:

- credentialed commands, token use, registry publish, signing, upload, deploy, or
  release automation;
- dependency installation, CI/workflow mutation, build-script mutation, or
  package-manager mutation without a separate governance issue;
- arbitrary shell execution, destructive filesystem operations, hidden commands,
  or local server command bridges;
- auto-merge, auto-apply, auto-accept, auto-promote, or reviewer bypass.

## Security wording boundary

Use conservative wording:

- Allowed: "documented trust boundary", "local-first MVP", "read-only browser
  surface", "generated local evidence", "no secure-sandbox guarantee".
- Not allowed as current claims: production-ready, compatibility-stable, secure
  sandbox, Godot replacement, native export, plugin runtime, marketplace, source
  apply ready, launch automation, or support SLA.

For wording scans and replacements, use
`docs/public-wording-guardrail-v1.md` and
`docs/public-wording-audit-process-v1.md`.

## Verification expectations

PRs touching this boundary should record:

- `gh issue view 372 --repo shaun0927/Ouroforge`;
- `gh issue view 1 --repo shaun0927/Ouroforge` and `gh issue view 23 --repo
  shaun0927/Ouroforge`;
- focused security/trust wording audit over changed files;
- generated-state audit with `git status --short --ignored`;
- broad Rust/Node/security gates when issue-level closure or broad docs changes
  require them.
