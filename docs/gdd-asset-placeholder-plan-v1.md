# GDD Asset Placeholder Plan v1

Issue: #652 — Asset Placeholder and Reference Plan v1.

GDD Asset Placeholder Plan v1 is a non-mutating Rust/local artifact for planning
prototype assets safely. It records placeholder ids, local fixture refs, existing
manifest refs, style notes, asset types, missing asset warnings, license/source
notes, required dimensions/metadata, stale refs, and blocked reasons.

## Boundary

Use placeholder assets or known local refs only. GDD-derived output remains
untrusted until Rust/local validation and review-gated apply. No asset generation,
no remote fetch, no uncontrolled copyrighted/proprietary content, no browser
trusted writes, no command bridge, no local server bridge, no auto-apply, no
auto-merge, no native export, no plugin runtime, and no autonomous unrestricted
game creation are authorized. Browser/dashboard/Studio surfaces stay read-only
or draft-only unless a separate Rust/local trusted API is explicitly scoped.

## Validation

Rust/local validation rejects unsafe paths, remote refs, generated-root or
evidence-output collisions, missing license/source notes, proprietary/copyright
ambiguity, unsupported asset types, missing manifest declarations, stale manifest
refs without blockers, and overbroad v1 plans. Asset generation is out of scope
unless separately authorized.

## Artifact separation

GDD, extracted requirements, mechanics mapping, feasibility, project scaffold,
scene/level plans, behavior plans, asset plans, prototype drafts, review, apply,
run evidence, and journal artifacts remain separate. Asset plans reference
existing manifests and local fixtures; they do not write generated assets or
fetch remote files.

## Read model

The read model is display/export data only: counts by asset type/source kind,
blocked counts, validation notes, compatibility notes, and the conservative
boundary. It has no trusted write path and no asset generation or remote fetch
authority.

## Governance

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open. Public wording stays conservative:
this is evidence-gated bounded prototype planning, not a production game,
shipped-game claim, commercial readiness claim, hosted service, native export,
plugin runtime, or current Godot replacement.
