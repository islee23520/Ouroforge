# Multi-Agent Production Pipeline v1

Multi-Agent Production Pipeline v1 is the Milestone 13 scope contract for local-first, evidence-gated collaboration between role-specialized agents on a bounded Ouroforge game project. It is an accountability and review framework, not an autonomous unrestricted project mutation system, hidden worker runtime, hosted/cloud orchestrator, browser command bridge, release pipeline, current Godot replacement, or production-ready claim.

This document is the control contract for follow-up issues #666 through #680. It defines what later task board, ownership, handoff, review, QA, regression, decision-ledger, evidence-bundle, Studio inspection, demo, and coverage work may implement without re-opening the milestone boundary.

## Bounded target

The v1 target is a deterministic local pipeline with explicit artifacts:

1. role model for designer, gameplay engineer, level designer, asset/import planner, QA agent, performance/regression agent, reviewer, critic, and build/release candidate agent;
2. production task board with ownership, dependencies, status, evidence references, and generated-state policy;
3. file and artifact ownership conflict policy;
4. work package and acceptance contract;
5. agent handoff artifact v2;
6. shared project state snapshot;
7. multi-agent review and independent critic gate;
8. QA agent work queue;
9. performance and regression agent lane;
10. build and release candidate lane design gate;
11. agent decision ledger;
12. production evidence bundle;
13. read-only Studio/dashboard/cockpit inspection;
14. deterministic demo fixtures and regression coverage.

All artifacts are source-like contracts or fixture-scoped examples unless a follow-up issue explicitly scopes a trusted Rust/local writer. Generated task boards, handoffs, work packages, snapshots, evidence bundles, runs, traces, screenshots, dashboard exports, temporary projects, and local tool state remain untracked unless explicitly fixture-scoped.

## Trusted boundary

Agent outputs are untrusted proposals until Rust/local validation and review-gated apply or promotion accepts them. The trusted side owns:

- schema and invariant validation;
- project/run binding;
- artifact path and generated-state checks;
- evidence-bundle writing when explicitly scoped;
- CLI contracts and local persistence;
- review/promotion decision validation.

Browser, dashboard, and Studio surfaces are read-only or draft-only consumers of trusted state. They may render tasks, handoffs, queues, reviews, ledgers, evidence, and gaps, but they must not spawn workers, execute commands, write trusted files, apply source changes, auto-promote outputs, merge PRs, alter visibility, or bypass review.

## Role and lane separation

Role assignment is accountability metadata, not authority escalation. A producing agent may prepare drafts and evidence, but trusted acceptance requires independent review where scoped.

- Designers, gameplay engineers, level designers, and asset/import planners produce bounded work packages and handoffs.
- QA agents produce scenario/playtest findings, repro evidence, and mutation backlog candidates.
- Performance/regression agents produce budget and regression evidence.
- Reviewers validate package completeness, generated-state hygiene, compatibility, and acceptance evidence.
- Critics challenge assumptions, unsafe promotion, ownership conflicts, missing rollback metadata, and public wording drift.
- Build/release candidate agents remain design-gate-only in v1: they can inspect readiness evidence and blockers, but cannot publish, sign, deploy, edit CI/CD, package native exports, or change public visibility.

No self-review, auto-apply, auto-merge, hidden promotion, reviewer bypass, or silent ownership override is allowed.

## Dependency order

Follow-up issues should proceed in this dependency-safe order unless a later issue body explicitly proves a narrower independent slice:

| Order | Issue | Dependency purpose |
| --- | --- | --- |
| 1 | #664 | Scope contract and milestone boundary. |
| 2 | #666 | Production task board and ownership model. |
| 3 | #668 | File/artifact ownership conflict policy. |
| 4 | #669 | Work package and acceptance contract. |
| 5 | #667 | Handoff artifact v2, informed by ownership and package requirements. |
| 6 | #670 | Shared project state snapshot. |
| 7 | #671 | Review and critic gate. |
| 8 | #672 | QA agent work queue. |
| 9 | #673 | Performance and regression lane. |
| 10 | #674 | Build/release candidate lane design gate. |
| 11 | #675 | Agent decision ledger. |
| 12 | #676 | Production evidence bundle. |
| 13 | #677 | Studio multi-agent pipeline inspection surface. |
| 14 | #678 | Deterministic production demo. |
| 15 | #679 | Scenario coverage/regression suite. |
| 16 | #680 | Roadmap and #1 governance refresh after implementation evidence exists. |

A later PR may reference already-merged fixtures and docs, but it must keep its issue boundary small and verifiable.

## Verification and closure gates

Each issue and PR unit must record evidence for its own boundary:

- live GitHub checks for the issue and for #1/#23 remaining open;
- focused Rust tests or Node smokes for new schemas, read models, dashboard/cockpit display, or fixture contracts;
- `cargo fmt --check`, relevant `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings` when Rust contracts change;
- `node --check` and dashboard/cockpit tests when browser display surfaces change;
- `git diff --check` and `git status --short --ignored` for whitespace and generated-state hygiene;
- issue closure comment listing PRs, commits, verification commands, generated-state policy, conservative wording audit, and #1/#23 status.

A closure gate passes only when the issue-specific acceptance criteria are met without broadening authority beyond this document.

## Compatibility policy

Multi-agent v1 must preserve existing project, scaffold, scene, tilemap, asset, behavior, level, GDD, scenario, dashboard, and Studio contracts unless a PR includes an explicit migration note and targeted compatibility tests. Source-like fixtures may be added only when deterministic and issue-scoped.

## Conservative wording policy

Public wording must describe v1 as local-first, evidence-gated collaboration/accountability. Do not claim autonomous arbitrary game completion, production readiness, commercial readiness, current Godot replacement, hosted orchestration, cloud worker pools, secure sandboxing, production CI/CD, native export shipping, release automation, or publication support.

## Explicit non-goals

Out of scope for v1:

- autonomous unrestricted project mutation or arbitrary game completion;
- hidden background agents, unbounded spawning, remote worker pools, hosted/cloud/server orchestration, accounts, or production CI/CD automation;
- auto-apply, auto-merge, self-approval, reviewer bypass, hidden promotion, release automation, signing, publishing, or visibility changes;
- browser trusted writes, browser command bridges, local server command bridges, credentialed commands, network/install commands, dependency mutation, CI/workflow/build-script mutation, or dynamic code loading;
- unrestricted source mutation, plugin runtime loading, visual scripting implementation, native export/platform packaging implementation, or current Godot replacement.

## Definition of done for the milestone

The milestone is done when the follow-up issues produce validated local artifacts and read-only inspection surfaces that demonstrate accountable multi-agent collaboration, with generated-state hygiene, independent review boundaries, conservative wording, and #1/#23 governance preservation recorded in final evidence.
