# Public issue response snippets v1

Status: **governance-only maintainer response aid**. These snippets and
criteria help maintainers answer, convert, or close public alpha issues after
using `docs/public-issue-intake-triage-v1.md`. They do not create support SLAs,
accept work automatically, change repository visibility, configure GitHub
automation, publish a release, mutate labels/settings, or implement product
behavior.

Use snippets as starting text only. Replace bracketed fields with concrete issue
numbers, evidence refs, affected files, verification commands, and safe next
steps before posting. Keep public wording conservative: no production-ready,
compatibility-stable, secure-sandbox, native-export-ready, plugin-runtime-ready,
source-apply-ready, Godot-replacement, or support-SLA claims.

## Response rules

1. **State the category first:** use one category from
   `docs/public-issue-intake-triage-v1.md` before discussing next action.
2. **Tie action to evidence:** cite the command output, run id, doc section,
   screenshot, user report, or maintainer decision that supports the response.
3. **Preserve explicit non-goals:** repeat relevant no-launch, no-publication,
   no-visibility-change, no-hosted/cloud/auth, no-native-export, no-plugin
   runtime, no-source-apply, no-command-bridge, no-support-SLA, and generated
   state boundaries.
4. **Prefer the smallest safe conversion:** convert to a scoped issue or PR only
   when the acceptance fields and verification path are clear.
5. **Close conservatively:** close only when the issue is duplicate,
   answered/docs-linked, out of alpha scope, unsafe to track publicly, generated
   local state with no source-like follow-up, or lacks required information after
   maintainer follow-up.
6. **No implied response guarantee:** phrases like "when practical", "best
   effort", and "maintainer-owned" are acceptable; do not promise response
   times, availability, or ongoing support.

## Triage acknowledgement

```text
Thanks for the report. I am classifying this as [category] under
`docs/public-issue-intake-triage-v1.md`.

Evidence recorded: [run id / command output / doc section / user report].
Affected surface: [CLI/runtime/scenario/dashboard/cockpit/docs/template/security
boundary/roadmap].
Requested outcome: [fix / clarify / document / answer / convert / defer / close].

Guardrails: this does not authorize repository visibility changes, launch or
release automation, package publication, hosted/cloud/auth behavior, native
export, plugin runtime, source apply, browser trusted writes, command bridges,
production-ready claims, compatibility promises, secure-sandbox claims, Godot
replacement claims, support SLAs, or committing generated local state.
```

## Needs information

```text
Thanks for opening this. I cannot route it safely yet because [missing evidence /
missing reproduction / unclear affected surface / missing non-goals / missing
verification path] is not present.

Please add:
- category from `docs/public-issue-intake-triage-v1.md`;
- concrete evidence such as command output, run id, journal/verdict excerpt,
  screenshot, affected doc section, or user report;
- affected surface and smallest requested outcome;
- explicit non-goals;
- generated-state status;
- local verification command or docs audit that would prove completion.

Until then this remains `needs-info`; no implementation, launch, release,
visibility, or support commitment is accepted by this issue.
```

## Convert to a scoped implementation issue

```text
This can be converted to a scoped implementation issue because it has a clear
[category], evidence in [artifact/ref], affected surface [surface], smallest
requested outcome [outcome], explicit non-goals [non-goals], generated-state
policy [policy], and verification path [commands/audit].

Conversion requirements:
- keep the new issue limited to [files/surfaces];
- include dependency order and PR unit boundaries if needed;
- preserve no-launch/no-visibility/no-publication/no-support-SLA wording;
- keep generated artifacts untracked unless separately fixture-scoped;
- keep #1 and #23 open unless a separate explicit governance decision says
  otherwise.
```

## Convert to a documentation-only PR

```text
This is suitable for a documentation/template PR rather than product behavior.
The PR should change only [docs/templates/checklists], preserve conservative
public-alpha wording, include a wording scan and generated-state audit, and avoid
GitHub settings, label automation, repository visibility, launch/release,
package publication, or support commitments.

Expected verification: [docs audit / template parse / wording scan / local smoke
commands].
```

## Answer and close as resolved

```text
Thanks for the question. Based on [docs/ref/evidence], the answer is:
[concise answer].

This closes the issue as answered. The answer is best-effort public-alpha
guidance only; it does not create a support SLA, compatibility guarantee,
production-readiness claim, or commitment to implement follow-up work. If new
reproducible evidence appears, please open a new issue with the intake fields
from `docs/public-issue-intake-triage-v1.md`.
```

## Close as duplicate

```text
Closing as a duplicate of #[canonical issue]. Please continue discussion there
so evidence, acceptance criteria, and verification remain in one place.

No new scope is accepted here, and this closure does not change repository
visibility, launch/release status, public-readiness decisions, or #1/#23.
```

## Close as out of current alpha scope

```text
Thanks for the suggestion. I am closing this because it depends on [unsupported
scope: production readiness / compatibility guarantees / secure-sandbox
guarantees / hosted-cloud operation / support SLA / native export shipping /
plugin runtime / source apply / command bridge / browser trusted writes / Godot
replacement positioning / launch or release automation].

Ouroforge is a local-first evidence-native MVP. Reconsideration would require a
separate governance or design-gate issue with concrete evidence, explicit
non-goals, generated-state policy, and local verification.
```

## Defer generated or local state

```text
This evidence appears to be generated or local state ([runs/ target/
.openchrome/ .omc/ .omx/ .claude/ dashboard export / screenshot / local tool
output]). It is useful context but should not be committed as source unless a
separate fixture-scoped issue authorizes a tiny deterministic fixture.

Please summarize the finding in source-like docs or fixtures and link the local
evidence instead. Until that exists, this issue is deferred/closed without
accepting implementation work.
```

## Redirect security-sensitive details

```text
This may be security-sensitive. Please do not post exploit details, secrets,
tokens, private screenshots, private issue links, or sensitive local paths in
this public issue.

Use the public-safe summary and private coordination guidance in
`docs/security-response-playbook-v1.md`. Public tracking should include only the
least-sensitive category/status and sanitized follow-up issue or PR links when a
maintainer decides that is safe.
```

## Close/convert criteria

Use these criteria before changing issue state:

| Outcome | Required before action | Do not use when |
| --- | --- | --- |
| Keep open for triage | Category, evidence, affected surface, requested outcome, non-goals, generated-state status, and verification path are incomplete but likely obtainable. | The issue exposes sensitive details that should move private, or the request is clearly forbidden/out of scope. |
| Convert to implementation issue | Evidence is concrete, scope is bounded, affected files/surfaces are known, dependencies/PR units are clear, non-goals are explicit, generated-state policy is safe, and verification is local/reproducible. | The request touches a design-gate boundary without a merged design gate or implies unsupported public claims. |
| Convert to documentation/template PR | Change is docs/templates/checklists only, wording is conservative, no behavior/settings/automation are needed, and docs/template verification is enough. | The requested change alters product behavior, launch/release status, repository visibility, or support commitments. |
| Answer and close | The answer is in existing docs or can be answered with a conservative public-alpha note, and no follow-up work is accepted. | The answer would require debugging, product changes, private security handling, or ongoing support. |
| Close as duplicate | Canonical issue exists and covers the same evidence or requested outcome. | The new issue adds materially different evidence, affected surface, or verification requirements. |
| Close as out of scope | Request requires forbidden scope, unsupported claims, launch/release/publication automation, hosted/cloud/auth, native export shipping, plugin runtime, source apply, command bridge, browser trusted writes, production support, or Godot replacement positioning. | A smaller safe docs/governance/design-gate conversion can satisfy the user need without the forbidden scope. |
| Defer or close generated-state-only report | The only artifact is local/generated state and no source-like summary, deterministic fixture, or public-safe evidence is available. | A separate fixture-scoped issue authorizes a tiny deterministic artifact. |
| Redirect security-sensitive report | The report may expose secrets, local paths, exploit details, private screenshots, dependency/supply-chain risk, or trusted-boundary bypasses. | The report is already sanitized and maintainers decide public tracking is safe. |

## Wording audit for PLG1.4.2

Before merging response-snippet changes, verify:

```bash
gh issue view 381 --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
grep -RInE "will respond within|guaranteed support|support SLA|production-ready|compatibility-stable|secure sandbox|native export ready|plugin runtime ready|source apply ready|Godot replacement" docs .github README.md || true
git ls-files runs target .openchrome .omc .omx .claude examples/evidence-dashboard/dashboard-data.json
```

The grep output must be reviewed for conservative negations or explicit
non-goals. Any positive support guarantee, maturity claim, or launch/release
promise must be removed before merge.
