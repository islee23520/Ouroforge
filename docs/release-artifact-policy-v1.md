# Release Artifact Policy v1

Status: **governance checklist, not release automation**.

This policy records which release artifacts are currently approved for Ouroforge
and how maintainers should evaluate any future release request. It does not
publish crates, npm packages, binaries, signed artifacts, hosted demos, native
exports, plugin/runtime bundles, marketplace entries, or public launch messages.
Repository visibility remains a separate manual maintainer decision.

## Current artifact authority

No publishable release artifact is approved by this policy.

| Artifact class | Current status | Boundary |
| --- | --- | --- |
| crates.io crate | Not approved | No package upload, token use, release job, or SemVer compatibility claim. |
| npm package | Not approved | No package upload, registry metadata, or install-support claim. |
| Binary/native export | Not approved | No desktop/mobile/native export bundle, signing, notarization, or installer. |
| Web/hosted demo | Not approved | Local demo evidence may exist, but no hosted release or public launch is authorized. |
| Plugin/runtime/marketplace bundle | Not approved | Plugin-system and marketplace claims remain out of scope. |
| Documentation/evidence note | Allowed when scoped | May reference commits, PRs, local generated runs, and verification commands without implying publication. |

The only currently allowed "release-like" artifacts are documentation and local
evidence records that remain inside the generated-state policy and link exact
commits or PRs.

## Manual release checklist

Before any future issue may approve a real package, binary, hosted demo, or
release artifact, maintainers should require a separate decision record that
answers every item below. This checklist is governance input only; it does not
create CI jobs, publication scripts, signing workflows, or registry actions.

### 1. Scope and artifact identity

- [ ] Name each artifact and target audience.
- [ ] State whether the artifact is docs-only, local generated evidence,
      package, binary, hosted demo, or another class.
- [ ] Link the exact Git commit, PR, and issue authorizing the artifact.
- [ ] Define who can perform the manual action and where evidence will be
      recorded.

### 2. Version and compatibility language

- [ ] Use `docs/release-versioning-policy-v1.md` for current pre-release wording.
- [ ] State whether SemVer applies; if not, use commit/PR identifiers.
- [ ] Avoid production-ready, compatibility-stable, Godot replacement,
      secure-sandbox, native export, plugin runtime, marketplace, and support-SLA
      claims unless a later explicit governance issue approves that exact claim.

### 3. Verification and rollback evidence

- [ ] Run fresh-clone or clean-worktree verification appropriate to the artifact.
- [ ] Run `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets
      --all-features -- -D warnings`, and `cargo audit` unless a documented
      blocker explains why not.
- [ ] Run the evidence dashboard and authoring cockpit Node checks/tests when
      public-facing docs or demo evidence are involved.
- [ ] Record generated-state status and cleanup steps.
- [ ] Define rollback/unpublish/hold criteria before publication.

### 4. Security and trust boundary review

- [ ] Confirm no browser trusted writes, command bridge, local server bridge,
      hidden command execution, auto-apply, auto-merge, or reviewer-bypass path
      is added.
- [ ] Confirm no dependency, CI, build-script, credential, or registry-token
      mutation is required unless separately approved.
- [ ] Link security/trust-boundary docs and known limitations.

### 5. Public communication boundary

- [ ] Separate artifact approval from repository visibility changes.
- [ ] Separate artifact approval from launch announcements, support promises, and
      public roadmap acceptance.
- [ ] Preserve #1 and #23 open unless a separate explicit governance decision
      authorizes changing them.

## Manual no-publish gate for current public-alpha readiness

For current Public Alpha Readiness / Open-Source Preparation work, the answer to
every publish-action question remains **No**:

- [ ] Did this issue publish a package, binary, hosted demo, installer, or native
      export?
- [ ] Did this issue add or modify release automation, registry workflows,
      signing, upload, or deployment steps?
- [ ] Did this issue change repository visibility or publish launch messaging?
- [ ] Did this issue claim production readiness, compatibility stability,
      secure-sandbox guarantees, native export readiness, plugin-runtime
      readiness, marketplace availability, or support SLAs?
- [ ] Did this issue commit generated local state outside explicitly scoped
      fixtures?

If any answer is **Yes**, the work has drifted out of this policy and must stop
until a separate governance issue authorizes the changed scope.

## Release-note evidence format

When a PR needs to record release-artifact policy evidence, include:

```markdown
Release artifact policy evidence:
- Artifact authority: none / docs-only / local generated evidence.
- Publication actions: none.
- Automation changes: none.
- Visibility changes: none.
- Verification: <commands or docs-only rationale>.
- Generated-state audit: <clean/ignored roots>.
- Protected issues: #1 open, #23 open.
```

## Relationship to versioning policy

`docs/release-versioning-policy-v1.md` defines how to describe versions,
changelog entries, and local demo evidence before real releases exist. This file
defines the no-publish artifact boundary and the checklist a later governance
issue must satisfy before any real release artifact can be approved.
