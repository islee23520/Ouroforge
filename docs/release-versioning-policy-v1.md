# Release Versioning Policy v1

Status: **governance policy for local/public-alpha readiness**.

This document defines how Ouroforge describes versions, changelog entries, and
release notes while the project remains a local MVP/public-alpha candidate. It
does not publish packages or binaries, create release automation, change
repository visibility, or approve a public launch.

## Current release stance

Ouroforge has no crates.io, npm, binary, signing, upload, marketplace, or native
export release artifact approved by this policy. Maintainers may use Git commits,
PRs, and documentation milestones as evidence references, but those references
are not package releases.

Allowed wording:

- "local MVP"
- "public-alpha readiness candidate"
- "manual public-visibility review"
- "documentation milestone"
- "evidence-backed commit or PR"

Disallowed wording unless a later explicit governance issue approves it:

- "production-ready"
- "compatibility-stable"
- "secure sandbox guarantee"
- "Godot replacement"
- "released on crates.io/npm"
- "native export release"
- "plugin marketplace/runtime release"
- "support SLA"

## Versioning stance before package releases

Until a maintainer explicitly approves package or binary publication, Ouroforge
uses a conservative pre-release/version-note model:

1. **Repository state is identified by commit SHA and merged PRs.** Use exact
   Git commits for reproducible evidence.
2. **Milestone docs use contract versions.** Files such as `*-v1.md` and
   `*-v2.md` describe policy/schema/evidence contracts, not published package
   versions.
3. **Public-alpha readiness may be described as a candidate state only.** It is
   not a semantic-versioned release and does not imply launch, support, or
   compatibility guarantees.
4. **No SemVer promise is made for APIs or data formats yet.** Schema/version
   suffixes document current contracts and regression fixtures; they are not a
   backwards-compatibility pledge.
5. **Future SemVer adoption requires a separate decision record.** That record
   must define release artifacts, compatibility scope, deprecation policy,
   changelog categories, signing/checksum expectations, and rollback criteria.

## Changelog format

When maintainers need release-note-style documentation before formal releases,
add entries under a dated heading in a docs-only changelog or issue/PR evidence
comment using this shape:

```markdown
## YYYY-MM-DD — <milestone or governance note>

Status: <docs-only | local evidence | manual readiness candidate>

Changed:
- <user-visible docs, fixture, or evidence change>

Verification:
- `<command>`

Boundaries:
- No package/binary publication.
- No repository visibility change.
- No launch/release automation.
- #1 and #23 remain open unless a separate governance decision says otherwise.
```

Changelog entries must link exact PRs, commits, and verification evidence where
available. They must not imply that generated local outputs are official release
artifacts.

## Demo evidence version notes

Demo evidence may be identified by run id, generated artifact path, commit SHA,
scenario pack, and dashboard/cockpit smoke result. A demo evidence note is valid
only when it remains local/generated-output scoped and records:

- source commit or PR;
- command used to produce the evidence;
- scenario or demo fixture name;
- generated output root, usually under `runs/` or another ignored/generated
  location;
- cleanup or generated-state audit result;
- explicit statement that the evidence is not a published package, hosted demo,
  binary, or compatibility guarantee.

## Checklist for future release/version notes

Before adding any version note, changelog entry, or release-note-style document,
confirm:

- [ ] It identifies exact commits/PRs or local generated evidence.
- [ ] It keeps package, binary, signing, upload, and publish actions out of
      scope.
- [ ] It avoids production-ready, Godot replacement, compatibility-stable,
      secure-sandbox, native export, plugin runtime, marketplace, and support-SLA
      claims.
- [ ] It records verification commands or explains why the entry is docs-only.
- [ ] It keeps generated local state untracked unless an explicitly scoped
      fixture authorizes tracking.
- [ ] It preserves #1 and #23 open unless a separate explicit governance decision
      exists.

## Relationship to later governance

This policy supports Public Alpha Readiness and Launch Governance by defining
conservative versioning and changelog language.
`docs/release-artifact-policy-v1.md` adds the no-publish release artifact
checklist for current public-alpha readiness work. Later issues may add a
readiness gate report or manual visibility decision record, but those later
artifacts must not reinterpret this policy as publication authority.
