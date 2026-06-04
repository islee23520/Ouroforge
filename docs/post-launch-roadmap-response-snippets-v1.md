# Post-launch roadmap response snippets v1

Status: **governance-only communication aid**. These snippets help maintainers
respond consistently to post-launch feedback routed through
`docs/post-launch-roadmap-triage-v1.md`. They do not accept work automatically,
change repository visibility, publish a release, or promise support.

Use snippets as starting text only. Replace bracketed fields with concrete issue
numbers, evidence refs, files, and verification commands before posting.

## Accept into a roadmap bucket

```text
Thanks for the request. Based on the evidence in [artifact/run/doc/user report],
this fits the [roadmap bucket] bucket from docs/post-launch-roadmap-triage-v1.md.

Accepted next step: open or refine a scoped issue that names the smallest useful
change, exact files/surfaces, explicit non-goals, generated-state policy, and
verification commands.

Guardrails: this does not authorize repository visibility changes, launch or
release automation, source patch apply, native export, plugin runtime,
distributed QA runtime, hosted/cloud/auth behavior, browser trusted writes,
command bridges, production-ready claims, compatibility promises, secure
sandbox claims, Godot replacement claims, or support SLAs.
```

## Request clarification

```text
Thanks for the request. I cannot route this yet because [evidence / affected
files / non-goals / verification commands / user value] are missing.

Please add:
- the roadmap bucket you believe applies;
- concrete evidence such as a run id, journal/verdict, screenshot, doc gap, or
  maintainer decision;
- the smallest proposed scope;
- explicit non-goals;
- local verification commands or artifact checks.

Until that is present, this remains clarification-only and does not authorize
implementation or public-readiness action.
```

## Move to a design gate

```text
This request touches [source mutation / native export / plugins / distributed QA
/ hosted services / trusted browser writes / command execution / untrusted code],
so it should move to a design gate before implementation.

Required design-gate evidence: threat/trust boundary, allowed and forbidden
files or capabilities, generated-state policy, rollback or failure evidence,
review gates, and conservative public wording.

No implementation PR should add this capability until the design gate is merged
and a separate implementation issue authorizes a bounded slice.
```

## Reject for current roadmap

```text
Thanks for the suggestion. This is out of the current roadmap because it depends
on [production readiness / compatibility guarantees / secure sandbox guarantees
/ hosted-cloud operation / support SLA / native export shipping / plugin runtime
/ source apply / Godot replacement positioning].

Ouroforge remains a local-first evidence-native MVP. A future reconsideration
would need a separate governance issue with evidence, explicit non-goals, and a
safe design gate where applicable.
```

## Defer generated or local state

```text
This evidence is useful context, but it appears to be generated or local state
([runs/ target/ .openchrome/ .omc/ .omx/ .claude/ dashboard export / screenshot /
local tool output]). It should remain untracked unless a separate issue scopes a
tiny deterministic fixture.

Please summarize the finding in source-like docs or fixtures and link the local
evidence instead of committing generated output.
```

## PR review drift reminder

```text
Before this PR can be reviewed as in-scope, please confirm:
- current issue number and PR unit id;
- exact changed files;
- authorized behavior;
- explicit non-goals;
- generated artifacts that remain untracked;
- why this remains inside the named roadmap bucket or design gate;
- #1 and #23 remain open.
```
