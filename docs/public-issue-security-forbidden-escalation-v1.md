# Public issue security and forbidden-scope escalation v1

Status: **governance-only escalation path**. This document defines how public
alpha issue triage should escalate suspected security reports and forbidden-scope
requests after classification in `docs/public-issue-intake-triage-v1.md`. It
does not implement product behavior, configure GitHub automation, mutate labels
or repository settings, change repository visibility, publish a release, create a
security advisory process, create a bounty, or promise support/response SLAs.

Use this document when an issue is classified as **Security** or **Forbidden
scope request**, or when an otherwise ordinary bug, docs, demo/onboarding,
feature, roadmap/governance, or support question contains security-sensitive
details or unsupported authority requests.

## Escalation principles

1. **Public-safe first:** never ask reporters to add secrets, tokens, exploit
   details, private screenshots, private issue links, or sensitive local paths to
   a public issue.
2. **Least authority:** escalation can redirect, sanitize, split, or close an
   issue; it does not authorize source apply, browser trusted writes, command
   bridges, hosted services, release automation, or repository setting changes.
3. **Private when uncertain:** if public details could expose a boundary bypass,
   secret, local path, or exploitable reproduction, move details to the private
   maintainer coordination path in `docs/security-response-playbook-v1.md`.
4. **Design-gate before capability:** requests for risky capabilities must move
   to a governance/design-gate issue before implementation, and only if a safe
   alpha path exists.
5. **Close forbidden claims:** requests that depend on unsupported maturity,
   support, compatibility, security, shipping, or Godot-replacement claims should
   be closed or redirected with conservative wording.
6. **Protected anchors stay open:** #1 and #23 remain open unless a separate
   explicit governance decision says otherwise.

## Security escalation path

Use this path for suspected vulnerabilities, trusted-boundary bypasses, unsafe
generated artifacts, dependency/build-script concerns, browser automation leaks,
source preview/sandbox/source-apply risks, or security-sensitive public wording.

| Step | Maintainer action | Public issue content | Stop/continue condition |
| --- | --- | --- | --- |
| 1. Identify sensitivity | Decide whether the report plausibly involves secrets, local paths, exploit detail, private screenshots, dependency risk, trusted writes, command execution, browser automation, source preview/sandbox/apply, or generated artifact leakage. | Keep only a public-safe summary and classification. | If sensitive, continue privately before asking for more detail. |
| 2. Remove or avoid sensitive detail | Ask the reporter not to post exploit details, secrets, tokens, private screenshots, private links, or sensitive local paths. | Public comment may say "suspected security report received" and point to `docs/security-response-playbook-v1.md`. | If already public, avoid quoting sensitive details and ask maintainers to sanitize where possible. |
| 3. Route private coordination | Use the private maintainer-controlled path from `docs/security-response-playbook-v1.md`. | Public issue records only non-sensitive status such as `received`, `triaging`, `needs-info`, `mitigated`, or `not-applicable`. | Continue only when a maintainer decides what can be public. |
| 4. Decide public tracking | Choose sanitized public issue, scoped PR, docs-only clarification, duplicate, not-applicable close, or private-only handling. | Link only sanitized follow-up issues/PRs and high-level affected area. | Do not expose reproducer details unless explicitly sanitized. |
| 5. Verify before closure | Confirm no sensitive generated/local artifacts were committed and public wording avoids security guarantees or response promises. | Closure comment records public-safe evidence and known non-goals. | Close only when mitigation, docs clarification, duplicate, or not-applicable rationale is public-safe. |

Security escalation may produce a scoped mitigation PR, docs clarification, or
sanitized follow-up issue, but this document itself does not authorize any
product capability or advisory automation.

## Forbidden-scope escalation path

Use this path for requests that ask Ouroforge to do or claim something outside
the current public-alpha governance boundary.

Forbidden-scope examples include:

- repository visibility changes, launch announcements, release publication,
  crates.io/npm/binary shipping, or release automation;
- hosted/cloud/server/auth/accounts, remote execution, external production
  services, or secrets management;
- native export shipping, app-store readiness, packaged desktop/mobile/console
  claims, or compatibility guarantees;
- plugin runtime, marketplace, arbitrary third-party code loading, executable
  extension ecosystem, or secure sandbox guarantees;
- source patch apply, source merge, auto-apply, auto-merge, autonomous source
  repair, browser trusted file writes, command bridges, or hidden command
  execution;
- production-ready, compatibility-stable, secure-sandbox, support-SLA, or Godot
  replacement positioning.

| Step | Maintainer action | Public issue content | Stop/continue condition |
| --- | --- | --- | --- |
| 1. Name the forbidden request | Identify the exact unsupported authority or claim being requested. | Quote or paraphrase only the minimal public-safe request. | Continue if a smaller safe governance/docs/design-gate route may exist. |
| 2. Look for a safe smaller route | Decide whether docs clarification, roadmap/governance, or design-gate discussion can satisfy the user need without the forbidden capability. | State the smaller route, non-goals, and verification evidence needed. | Convert only when the smaller route is concrete and safe. |
| 3. Redirect to design gate when appropriate | For risky but potentially future-scoped areas, require a design gate before implementation. | Reference relevant roadmap/governance docs and require threat/trust boundary, generated-state policy, rollback/failure evidence, and conservative wording. | No implementation PR until the design gate is merged and a separate implementation issue exists. |
| 4. Close when no safe route exists | Close requests that depend on forbidden maturity, authority, automation, shipping, support, or compatibility claims. | Use `docs/public-issue-response-snippets-v1.md` out-of-scope wording and name the unsupported dependency. | Reopen only if a new governance issue supplies a safe smaller route. |
| 5. Audit boundaries | Before conversion or closure, check generated-state, wording, no-launch/no-visibility/no-publication, and #1/#23 state. | Closure/conversion comment records the audit. | Stop when the issue is safely converted, redirected, or closed. |

## Decision matrix

| Intake signal | Escalation owner | Public action | Private action | Allowed outcome |
| --- | --- | --- | --- | --- |
| Secret, token, private link, private screenshot, or sensitive local path appears or is requested | Maintainer/security triage | Ask for public-safe summary only; avoid repeating sensitive data | Move details to private maintainer channel | Sanitized follow-up, private-only handling, or close after sanitation |
| Trusted write, command execution, browser automation leak, source preview/sandbox/apply bypass, or dependency/build-script risk | Maintainer/security triage | Record high-level affected boundary and status only | Reproduce in clean worktree if needed | Scoped mitigation/docs PR or sanitized issue |
| Request for source apply, command bridge, browser trusted writes, auto-merge/apply | Governance/design-gate owner | State forbidden authority and design-gate requirement | None unless security-sensitive details are present | Close, redirect, or design-gate issue only |
| Request for native export shipping, plugin runtime, hosted/cloud/auth, marketplace, release/publication automation | Governance owner | State unsupported public-alpha scope and smaller safe route if any | None unless secrets/security details are present | Close, roadmap/governance issue, or design gate |
| Request for production-ready, compatibility-stable, secure-sandbox, support-SLA, or Godot-replacement claims | Governance owner | Close or request wording correction | None unless security-sensitive details are present | Wording-only docs PR or close |
| Generated/local artifact is the only evidence | Triage owner | Ask for source-like summary or fixture-scoped issue | None unless artifact contains sensitive details | Defer/close or convert to sanitized fixture scope |

## Public-safe snippets

### Security-sensitive redirect

```text
This may be security-sensitive, so please do not add exploit details, secrets,
tokens, private screenshots, private issue links, or sensitive local paths to
this public issue.

Public classification: Security under `docs/public-issue-intake-triage-v1.md`.
Next step: use the private maintainer coordination path in
`docs/security-response-playbook-v1.md`. Public tracking here should contain only
non-sensitive status and sanitized follow-up links when a maintainer decides they
are safe.

This does not create a bounty, response-time SLA, production security guarantee,
hosted-service guarantee, advisory automation, release process, or repository
settings change.
```

### Forbidden-scope redirect or close

```text
Closing or redirecting this as a forbidden-scope request because it depends on
[unsupported authority or claim]. Ouroforge public-alpha governance does not
authorize repository visibility changes, launch/release automation, package
publication, hosted/cloud/auth behavior, native export shipping, plugin runtime,
source apply, browser trusted writes, command bridges, production-ready claims,
compatibility promises, secure-sandbox claims, Godot replacement positioning, or
support SLAs.

A future reconsideration would need a separate governance/design-gate issue with
public-safe evidence, explicit non-goals, generated-state policy, and local
verification. This issue does not change #1 or #23.
```

### Safe smaller-route conversion

```text
The original request includes forbidden scope ([forbidden part]), but a smaller
safe governance/docs route may be possible: [safe route].

Conversion requirements:
- remove the forbidden capability or claim;
- keep changes to [docs/templates/checklists/design gate] only;
- state explicit non-goals and generated-state policy;
- include wording and no-launch/no-visibility/no-publication audits;
- keep #1 and #23 open.

No product behavior or GitHub automation is authorized by this conversion.
```

## Verification audit for PLG1.4.3

Before merging escalation-path changes, verify:

```bash
gh issue view 381 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
grep -RInE "will respond within|guaranteed support|support SLA|production-ready|compatibility-stable|secure sandbox|native export ready|plugin runtime ready|source apply ready|Godot replacement" docs .github README.md || true
git ls-files runs target .openchrome .omc .omx .claude examples/evidence-dashboard/dashboard-data.json
```

Review grep matches as conservative negations or explicit non-goals. Remove any
positive maturity claim, security guarantee, support promise, launch/release
promise, or unsupported authority before merge.
