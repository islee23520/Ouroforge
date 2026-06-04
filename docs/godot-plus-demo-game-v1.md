# Godot-Plus Demo Game v1 scope contract

Status: **scope contract only** for #778. This document defines the bounded Godot-Plus Demonstration Game v1 milestone under #1. It does not implement gameplay, export/package behavior, QA swarm automation, source apply, plugin runtime, marketplace behavior, Studio trusted writes, public deployment, release publication, or repository visibility changes.

## Purpose

Godot-plus means a scoped evidence-native agentic workflow demonstration: a small 2D-first playable vertical slice that combines local runtime behavior, deterministic scenario evidence, Studio inspection/draft surfaces, review-gated source mutation handoff, export/package verification, plugin descriptors as inert metadata, and conservative governance.

It does not mean full Godot parity, full Godot replacement, commercial release readiness, production engine/editor maturity, native/mobile/console export, app-store publishing, a secure sandbox, hosted/cloud service, executable plugin ecosystem, or support SLA.

## Bounded demo target

The v1 demo target is a small complete 2D vertical slice with enough gameplay to prove the workflow end-to-end:

- genre/design pillars: one concise arcade/adventure loop with clear win/fail/readability goals;
- demo GDD and acceptance criteria;
- project scaffold using existing local project manifest, scene, scenario, asset, evidence, and CLI contracts;
- core gameplay loop with deterministic player objective, hazards/obstacles, feedback, and completion state;
- level set small enough for reliable local smoke verification;
- enemy/NPC/system behavior only when it supports the core loop;
- UI/HUD/feedback sufficient for state readability;
- local asset pack with explicit asset-reference integrity evidence;
- scenario matrix and regression suite covering success, failure, and key interactions;
- QA/playtest evidence captured as local generated state and summarized in PR/issue records;
- agentic iteration demonstration through proposals, review decisions, and safe handoff evidence;
- Studio walkthrough showing inspect/draft/review surfaces without trusted browser writes;
- local web export/package verification only, not native/mobile/store publication;
- plugin descriptors as metadata only, not executable plugin runtime;
- Godot-plus comparison matrix focused on scoped workflow evidence, not superiority claims;
- performance/stability budget suitable for the small demo;
- documentation and reproducibility notes;
- roadmap governance refresh after the demo chain completes.

## Follow-up issue dependency order

Implement follow-up work in this order unless a later issue documents a concrete blocker and revised boundary:

1. Genre and design pillars.
2. Demo GDD and acceptance criteria.
3. Project scaffold.
4. Core gameplay loop.
5. Demo level set.
6. Enemy/NPC/system behavior.
7. UI/HUD/feedback.
8. Demo asset pack.
9. Demo scenario matrix.
10. QA swarm/playtest evidence.
11. Agentic iteration demonstration.
12. Studio walkthrough.
13. Local export/package verification.
14. Plugin descriptor usage.
15. Godot-plus comparison matrix.
16. Performance/stability budget.
17. Documentation/reproducibility.
18. Regression suite.
19. Roadmap/#1 governance refresh.

Each follow-up issue should prefer one reviewable PR unit per independently verifiable behavior slice.

## Verification gates

A claim about the demo is not complete until the relevant issue records:

- focused tests or smokes for the changed demo behavior;
- scenario evidence for gameplay claims;
- generated-state audit showing local outputs remain untracked unless fixture-scoped;
- dashboard/Studio smoke evidence for display claims;
- source mutation evidence proving Safe Source Apply review gates remain in force;
- export/package evidence for local web package claims only;
- plugin descriptor evidence proving metadata-only behavior;
- conservative wording scan for Godot replacement, production readiness, commercial release, native export, secure sandbox, support SLA, and universal superiority claims;
- #1 and #23 open-state verification.

Baseline commands for this scope issue and later closure gates:

```bash
gh issue view <issue-number> --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

## Generated-state policy

Generated demo outputs, exports, QA/playtest runs, evidence bundles, screenshots, videos, temp servers, package bundles, browser profiles, and local tool state remain ignored/untracked unless a later fixture-scoped issue explicitly authorizes a deterministic artifact.

Expected local/generated roots include `runs/`, `target/`, `.openchrome/`, `.omc/`, `.omx/`, `.claude/`, browser profiles, dashboard exports, screenshots, and package output directories.

## Claim boundaries

Allowed wording:

- "small playable Godot-plus vertical slice";
- "scoped evidence-native agentic workflow demonstration";
- "local web package verification";
- "Studio inspect/draft/review walkthrough";
- "review-gated Safe Source Apply handoff";
- "plugin descriptor metadata".

Avoid or reject claims that Ouroforge is:

- a full Godot replacement;
- fully Godot-compatible;
- production-ready or commercial-release ready;
- a secure sandbox;
- a native/mobile/console export solution;
- an executable plugin runtime or marketplace;
- a hosted/cloud/server/auth platform;
- an autonomous source-apply/auto-merge system;
- covered by a support SLA.

## Trusted-boundary rules

- Rust/local trusted code owns validation, persistence, scenario verification, source-apply/export/evidence contracts, run/project binding, and CLI behavior.
- Browser/Studio surfaces remain read-only or draft-only except existing allowlisted runtime interactions.
- Source mutation must remain review-gated through Safe Source Mutation Apply controls.
- No direct Studio trusted source writes, browser command bridge, hidden command execution, self-approval, reviewer bypass, arbitrary shell, dependency install, CI/workflow mutation, network install/update, credentialed operation, auto-apply, or auto-merge.
- Plugin descriptors are metadata until a separate explicit issue authorizes any executable runtime.

## Closure requirements for #778

#778 is complete when this scope contract exists, the dependency graph and bounded target are clear, no gameplay/product behavior was added, baseline verification passes, generated state remains untracked, public wording is conservative, and #1/#23 remain open. The closure record must also state that #1 and #23 remain open.
