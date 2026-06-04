# Security Policy

Ouroforge is currently a local-first private MVP moving toward public open-source readiness. It is not a hosted service and does not provide account, authentication, authorization, cloud storage, or multi-tenant execution features.

## Supported versions

| Version | Supported |
| --- | --- |
| `main` pre-release | Best-effort security review during MVP development |
| Published releases | None yet |

## Alpha response playbook

For alpha security report classification, temporary private coordination, source preview/sandbox/browser-boundary examples, and conservative public wording, see [`docs/security-response-playbook-v1.md`](docs/security-response-playbook-v1.md). The playbook is governance-only and does not create a bounty, support SLA, production security guarantee, release process, repository visibility change, or automated advisory workflow.

## Reporting a vulnerability

Until a dedicated security contact is published, please report suspected vulnerabilities through a private maintainer channel rather than a public issue. If only GitHub issues are available, file a minimal issue that says a private security report is needed and avoid posting exploit details, secrets, tokens, local paths, or private screenshots.

A useful private report should include:

- affected commit or branch;
- exact command or workflow involved;
- whether the issue touches local file writes, browser automation, generated run artifacts, or dependency behavior;
- reproduction steps that avoid exposing secrets or private data;
- expected impact and suggested mitigation, if known.

## Local execution boundary

Ouroforge runs local commands and local browser automation as part of its evidence-native MVP:

- `cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4` starts a local static runtime server and a local Chrome/Chromium process through the Chrome DevTools Protocol.
- Chrome is discovered from standard local paths or from `OUROFORGE_CHROME`.
- Generated run artifacts are written under `runs/`, which is ignored by git.
- Dashboard export writes a generated `examples/evidence-dashboard/dashboard-data.json`, which should not be committed.
- Local tool/runtime state such as `.openchrome/` and `.omc/` must remain untracked.

Only run seeds, runtime pages, and scene files that you trust. The MVP is intended for local development and inspection, not for executing untrusted web content or untrusted project files.

## Dependency posture

Rust dependencies are locked by `Cargo.lock`. Public-readiness evidence in `docs/public-demo-evidence.md` records `cargo audit` output against the lockfile. Maintainers should re-run `cargo audit` immediately before any later visibility change or tagged release.

## Non-goals for the current MVP

- No hosted service security model.
- No authentication or authorization guarantees.
- No sandbox guarantee for arbitrary user content.
- No compatibility promise for public releases until a separate stabilization issue explicitly creates one.
