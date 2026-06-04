# Public Alpha Communication Pack v1

Status: **final communication artifact for issue #385 PLG1.8.1 and PLG1.8.2**.

This pack gives maintainers conservative wording they may adapt when preparing a
future public-alpha review. It is not an announcement, launch approval,
repository visibility decision, release note, support promise, or product
roadmap acceptance. Publishing, changing repository visibility, tagging a
release, uploading packages, and making support commitments remain separate
manual maintainer decisions outside this document.

## Short project description

Ouroforge is a local-first, evidence-native prototype for game-authoring loops.
It turns a declared goal into a local run, captures runtime evidence, journals
what happened, and records reviewable mutation proposals without giving agents or
browser surfaces trusted write authority.

## Public alpha summary draft

Use this only after maintainers separately decide that a public-alpha review may
be discussed. Reusing this text does not publish an announcement or change the
repository visibility state:

> Ouroforge is an inspectable pre-release MVP for evidence-driven game-authoring
> experiments. The current repository demonstrates a local Seed → Run → Evidence
> → Evaluation → Journal → Mutation loop, read-only evidence surfaces, and
> governance documents for safe contribution review. It is useful for local
> inspection and documentation review, not for production game development or
> broad engine replacement.

Required surrounding context when the summary is reused:

- The repository remains pre-release unless a separate visibility decision says
  otherwise.
- Local demo commands may generate ignored `runs/`, `target/`, dashboard export,
  screenshot, sandbox, and local tool state.
- Browser/dashboard/cockpit surfaces are read-only evidence viewers, not trusted
  write or command surfaces.
- Mutation artifacts are proposals/evidence until a later issue explicitly
  authorizes trusted apply behavior.

## What works today

The current checked-in MVP demonstrates:

- seed validation and local run execution for the platformer seed;
- generated run evidence, ledger, journal, evaluator, mutation, comparison, and
  dashboard read models;
- local project validation and minimal 2D project scaffolding;
- a minimal browser runtime/probe path using local Chrome/Chromium;
- read-only static evidence dashboard and authoring cockpit surfaces over
  exported JSON;
- fixture-backed contracts for scenes, assets, tilemaps, source-preview,
  sandbox/review boundaries, and public-readiness documentation.

## What does not work / explicit non-goals

Do not describe Ouroforge as currently providing:

- production-ready editor or engine behavior;
- compatibility stability or a Godot replacement;
- secure sandboxing for arbitrary untrusted content;
- hosted/cloud/server/auth/account behavior;
- native export, packaging, signing, deployment, release, or publication
  automation;
- plugin runtime, marketplace, visual scripting, or third-party code loading;
- browser trusted writes, local command bridges, auto-apply, auto-merge, reviewer
  bypass, or source patch apply to the trusted maintainer worktree;
- support, security, uptime, response-time, or maintenance SLA commitments.

## Demo instructions pointer

For local demo commands, point readers to the repository README and the audited
fresh-clone references instead of copying stale commands into announcements:

- [`README.md`](../README.md)
- [`fresh-clone-onboarding-command-audit-v1.md`](fresh-clone-onboarding-command-audit-v1.md)
- [`fresh-clone-smoke-v1.md`](fresh-clone-smoke-v1.md)
- [`fresh-clone-troubleshooting-cleanup-v1.md`](fresh-clone-troubleshooting-cleanup-v1.md)
- [`public-demo-evidence.md`](public-demo-evidence.md)
- [`public-demo-smoke-evidence-policy-v1.md`](public-demo-smoke-evidence-policy-v1.md)

Every demo pointer should say that generated state remains local and ignored
unless a fixture-scoped issue explicitly authorizes tracking it.

## Safety model summary for communication

When describing the safety model, keep the language factual and narrow:

- Ouroforge is local-first: users run local commands and inspect local evidence.
- Rust CLI/core code and the local filesystem own trusted persistence.
- Agents, browser workers, dashboards, cockpits, and Chrome DevTools Protocol
  observations are evidence inputs or read-only displays.
- Browser surfaces do not write source files, execute commands, operate a local
  command server, or apply patches.
- Source-preview, sandbox, stale-target, rollback, and review artifacts are
  governance/evidence boundaries. They are not permission to mutate the trusted
  maintainer worktree.
- Generated evidence remains local ignored state unless a future fixture-scoped
  issue explicitly authorizes tracking it.

Do not use this safety summary to imply a sandbox guarantee for arbitrary
untrusted projects, browser content, dependencies, or user-supplied commands.

## Issue and security reporting pointers

Use existing contribution and security routing docs rather than promising a new
support channel:

- [`CONTRIBUTING.md`](../CONTRIBUTING.md) for contribution workflow and public
  wording checks.
- [`SECURITY.md`](../SECURITY.md) for private vulnerability-reporting guidance.
- [`public-issue-intake-triage-v1.md`](public-issue-intake-triage-v1.md) for
  public issue categories and safe routing.
- [`public-issue-response-snippets-v1.md`](public-issue-response-snippets-v1.md)
  for conservative maintainer replies.
- [`public-pr-intake-policy-v1.md`](public-pr-intake-policy-v1.md) for public PR
  review boundaries.
- [`security-response-playbook-v1.md`](security-response-playbook-v1.md) for
  alpha security-response governance.

## Maintainer response snippets

These snippets are intentionally conservative and should be adapted with current
evidence before use. They do not create a support queue or response-time promise.

### General project reply

> Ouroforge is a local-first pre-release MVP for evidence-native authoring-loop
> experiments. The current repository is useful for inspecting the local demo,
> evidence artifacts, and governance boundaries. It is not a production editor,
> compatibility-stable engine, hosted service, native exporter, plugin runtime,
> secure sandbox, or Godot replacement.

### Demo reply

> Start with the README and the fresh-clone smoke/audit docs. Demo commands write
> generated local state such as `runs/`, `target/`, dashboard exports,
> screenshots, and local tool output; those files should remain ignored unless a
> fixture-scoped issue explicitly says otherwise.

### Issue routing reply

> Please include the smallest reproducible local command, affected commit,
> expected versus actual evidence, and whether generated artifacts are involved.
> This issue does not authorize launch/release work, visibility changes, source
> apply, command bridges, hosted/cloud/auth behavior, or support commitments.

### Security routing reply

> Please avoid posting exploit details, secrets, tokens, private paths, or
> private screenshots in public. Use a private maintainer channel when available;
> if only a public issue is available, file a minimal routing issue that says a
> private security report is needed.

## Forbidden overclaim checklist

Before reusing any text from this pack, answer **No** to every question:

- Does the wording imply launch has happened or repository visibility changed?
- Does it claim release, package, binary, crates.io, npm, signing, upload, native
  export, or deployment availability?
- Does it claim production readiness, compatibility stability, secure sandboxing,
  Godot replacement status, plugin-runtime readiness, source-apply readiness, or
  support/security SLA coverage?
- Does it imply browser surfaces can write trusted files or run local commands?
- Does it imply generated demo, run, dashboard, screenshot, launch-report, or
  local tool artifacts should be committed?
- Does it close, replace, or weaken #1 or #23 as governance anchors?

## Publication boundary

This pack is a source document for maintainers, not a publication event. Before
copying any text into a public announcement, maintainers should separately record:

1. the exact visibility/publication decision and date under the public visibility
   decision process;
2. fresh verification of README/demo commands and public wording scans;
3. known gaps and non-goals that must accompany the announcement;
4. whether #1 remains the roadmap anchor and #23 remains the protected memory
   anchor;
5. who owns follow-up triage without creating support/security SLA claims.

## PLG1.8.2 finalization audit

PLG1.8.2 finalized this pack as a documentation/governance artifact only:

- short project description and public-alpha summary are conservative and
  reusable only after a separate maintainer decision;
- what-works and non-goal sections distinguish current local MVP behavior from
  unsupported launch/product claims;
- safety language points to local-first, read-only browser surfaces, no command
  bridge, no source apply, and generated-state isolation;
- issue and security reporting pointers reuse existing docs instead of creating
  a support channel or security guarantee;
- forbidden-overclaim and publication-boundary checklists require separate
  manual visibility/publication decisions;
- no repository visibility, GitHub settings, release, package publication,
  announcement publication, or product behavior was changed by this pack;
- generated demo, run, dashboard, screenshot, launch-report, and local tool
  artifacts remain ignored/untracked unless explicitly fixture-scoped;
- #1 and #23 remain protected anchors and must stay open unless a separate
  explicit governance decision authorizes otherwise.
