# GDD Scenario Acceptance Plan v1

Issue: #653 — Scenario and Acceptance Criteria Generation v1.

GDD Scenario Acceptance Plan v1 is a non-mutating Rust/local artifact for making
GDD requirements testable as scenario drafts and acceptance-criteria drafts. It
records acceptance ids, requirement ids, scenario ids, feasibility outcome refs,
required setup, input/action steps, expected flags/events/states, evidence
needed, unsupported checks, requirement coverage, stale targets, and blocked
reasons.

## Boundary

Scenario drafts are not trusted tests. GDD-derived output remains untrusted until
Rust/local validation and review-gated apply. No hidden implementation of unsupported checks, no arbitrary source mutation, no arbitrary script execution,
no browser trusted writes, no command bridge, no local server bridge, no
auto-apply, no auto-merge, no native export, no plugin runtime, and no
autonomous unrestricted game creation are authorized. Browser/dashboard/Studio
surfaces remain read-only or draft-only unless a separate trusted Rust/local API
is explicitly scoped.

## Validation

Rust/local validation rejects missing requirement links, missing mechanics mapping
links, unsupported mechanics, unsupported assertion kinds, unsafe scenario refs,
contradictory acceptance criteria, missing evidence expectations, stale targets
without blockers, non-covered requirements without blockers, remote refs, and
overbroad v1 plans.

## Artifact separation

GDD, extracted requirements, mechanics mapping, feasibility, project scaffold,
scene/level plans, behavior plans, asset plans, scenario plans, prototype drafts,
review, apply, run evidence, and journal artifacts remain separate. Scenario
plans may point to existing scenario packs or future draft refs, but they do not
write trusted tests or mutate scenario packs.

## Read model

The read model is display/export data only: draft counts, assertion-kind counts,
blocked counts, validation notes, compatibility notes, and the conservative
boundary. It has no trusted test creation, apply, or command authority.

## Governance

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open. Public wording stays conservative:
this is evidence-gated bounded prototype planning, not a production game,
shipped-game claim, commercial readiness claim, hosted service, native export,
plugin runtime, or current Godot replacement.
