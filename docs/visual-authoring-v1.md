# Visual Authoring v1 Scope and Contract

Status: **planned local-first safe edit-draft milestone after Asset Pipeline v1**.
Asset Pipeline v1 is complete as recorded in
`docs/asset-pipeline-v1-governance-handoff.md`; #1 and #23 remain open.

Visual Authoring v1 / Safe Local Edit Cockpit is the next bounded milestone for
helping authors assemble visual edit drafts without making Studio a trusted
writer. Studio may hold draft state in memory, display previews, and expose
copyable JSON or CLI commands. Rust-trusted CLI flows own validation,
transaction preview, review-gated apply, rollback, and evidence writing.

This is a scope/control contract. It does not implement draft schemas, CLI
transaction behavior, Studio authoring behavior, visual diff behavior, browser
writes, command bridges, production-editor capabilities, visual scripting,
source mutation, plugins, native export, hosted infrastructure, or public launch
automation.

## Completed baseline

Visual Authoring v1 starts from the completed local-first baseline:

- **Evidence Fidelity & Trust Boundary Hardening v1**: runtime probe contracts,
  replay/evidence boundaries, reproducible command context, and Studio warnings
  that separate Rust-trusted evidence from browser observations.
- **Agentic Review & Regression Promotion v1**: review-gated scene-only mutation,
  regression promotion, run matrices, evidence-backed journals, and read-only
  review cockpit state.
- **Agentic Loop Orchestration v1**: data-only loop plans, dry-run sequencing,
  trusted CLI step execution, resume/failure evidence, agent handoff contracts,
  and read-only loop cockpit inspection.
- **Engine Expressiveness v2 / Playable Game Authoring v1 implemented subset**:
  expressive scene components, collision/physics rules, triggers/flags, HUD,
  animation/audio event evidence, manifest-declared scene transitions, a
  collect-and-exit demo, scenario coverage v3, and read-only scene inspection.
- **Source Mutation Design Gate v1**: conservative design controls for source
  patch work, with source mutation apply still blocked.
- **Asset Pipeline v1 / Content Authoring Foundation**: manifest-backed local
  assets, sprite/tileset/tilemap foundations, asset reference integrity, runtime
  loading evidence, preview evidence, Studio asset inspection, an asset-backed
  demo refresh, scenario coverage v4, and roadmap/#1 governance refresh.

That baseline supports richer local project content and evidence-native review,
but it still keeps persistence and command execution outside the browser.

## Why this milestone comes next

After Asset Pipeline v1, authors can reason about scenes, tilemaps, asset refs,
runtime evidence, preview evidence, and read-only Studio inspection. The next
safe authoring step is not a production editor or browser write path; it is a
local edit-draft workflow that lets authors compose intended changes visually
while preserving trusted persistence in Rust.

Visual Authoring v1 should answer:

1. What inert draft data describes intended scene, tilemap, and asset-reference
   edits?
2. How can Studio help assemble those drafts without writing files or running
   commands?
3. How does a Rust-trusted CLI validate a draft and turn it into a reviewable
   transaction preview?
4. What evidence proves the visual diff, review decision, apply, rollback, and
   scenario coverage behavior?
5. Which closure gates prove generated/local state remains untracked and #1/#23
   remain open?

## Target outcome

The milestone target is a safe draft-to-review loop:

```text
in-memory Studio draft state
  -> copyable draft JSON / inert CLI command
  -> Rust CLI validation and transaction preview
  -> visual diff preview evidence
  -> explicit review-gated apply
  -> rollback/evidence bundle
  -> demo and scenario coverage
  -> roadmap/#1 governance refresh
```

Studio is allowed to make authoring easier, but it is not allowed to become a
trusted persistence or command boundary.

## Dependency order

Follow-up Visual Authoring v1 issues should be completed in this order:

1. **Visual Edit Draft Model v1** — define the data-only draft schema, ids,
   operation categories, provenance, diagnostics, fixture policy, and read-model
   compatibility notes (see `docs/visual-edit-draft-model-v1.md`).
2. **Scene Visual Edit Draft v1** — support draft descriptions, Rust preflight,
   and preview-only transaction generation for bounded scene entity/component
   edits within the existing scene edit transaction model; unsupported scene
   categories remain rejected before preview.
3. **Tilemap Visual Edit Draft v1** — support draft descriptions, Rust
   preflight, preview summaries, and collision/trigger read-model metadata for
   tilemap, layer, tileset, collision-tag, and placement edits without direct
   writes.
4. **Asset Reference Edit Draft v1** — support draft descriptions for manifest,
   sprite, tilemap, audio, font, and scenario asset-reference edits with
   integrity expectations.
5. **Edit Draft to Transaction CLI v1** — add Rust-trusted validation and
   transaction preview generation from draft JSON; Studio remains copy-only.
   Implemented command documentation and smoke procedure are recorded in
   `docs/edit-draft-transaction-cli-v1.md`.
6. **Visual Diff Preview v1** — produce bounded visual/read-model diff evidence
   for accepted draft previews, malformed drafts, and no-op/stale states.
   Rust owns `visual-diff-summary-v1` validation/generation, and Studio may render
   exported `visual_diff_preview` / `visualDiffPreview` data as escaped
   before/after, operation, source-ref, collision/trigger, asset/entity/tile,
   and scenario-impact diagnostics only. The rendering surface has no browser
   apply controls, trusted writes, local command bridge, source mutation, review
   decision controls, or browser persistence. Generated diff summaries are
   review evidence only; scenario impact still requires separate run evidence.
7. **Review-Gated Visual Edit Apply v1** — apply only reviewed/accepted visual
   edit transactions through the trusted CLI boundary with rollback/audit
   evidence. The trusted apply record is exported as a `visual_draft_applied`
   lifecycle stage and journal evidence for rerun/regression/loop inspection,
   but the command context remains inert display-only text; no auto-rerun,
   auto-apply, or browser apply is authorized.
8. **Studio Draft Authoring Surface v1** — expose in-memory draft assembly,
   warnings, preview state, and copyable JSON/commands as escaped UI state only.
9. **Visual Authoring Demo v1** — demonstrate a local safe edit-draft workflow
   using deterministic collect-and-exit draft fixtures and generated evidence
   references without production-editor claims. VA1.10.1 source-like draft
   examples live under `examples/visual-edit-draft-v1/valid/` and cover key
   move, HUD text change, tile obstacle add/remove, and asset frame replacement
   intent while keeping generated transaction/run outputs untracked. VA1.10.2
   records ignored local smoke ids for preview, accepted review, review-gated
   apply, rerun, compare, and dashboard export evidence without committing those
   generated artifacts. VA1.10.3 audits the demo Studio/dashboard documentation
   and public wording: these surfaces display Rust-exported evidence and inert
   command/draft text only, preserve conservative private-MVP language, and make
   no public-launch, production-editor, hosted Studio, browser-write, command-
   bridge, native-export, plugin-runtime, visual-scripting, or Godot-replacement
   claims.
10. **Scenario Coverage v5** — add regression coverage for draft validation,
    transaction preview, visual diff evidence, review-gated apply, rollback,
    generated-state policy, and read-only Studio boundaries.
11. **Roadmap and #1 Governance Refresh after Visual Authoring v1** — record the
    milestone outcome, next sequencing, guardrails, and #1/#23 preservation.

Later issues may split a category only with an explicit governance comment that
records the blocker, changed PR boundary, verification impact, and guardrail
impact. They must not skip the trust-boundary sequence for convenience.

## Studio safe-authoring boundary

Studio may:

- maintain temporary draft state in browser memory;
- render disabled/read-only scene, tilemap, and asset-reference draft controls as planning aids only;
- render escaped preview/read-model state from trusted exported artifacts;
- render escaped tilemap draft preview summaries, affected-cell counts, hashes,
  and collision/trigger metadata as display-only diagnostics;
- render escaped asset-reference draft preview summaries, manifest ids,
  replacement asset ids, asset types, content hashes, and frame/event context as
  display-only diagnostics;
- show warnings for stale, invalid, unsupported, or unreviewed drafts;
- display copyable draft JSON and copyable CLI command text without executing, uploading, fetching, or writing from the browser; and
- explain which trusted CLI command a human can run outside the browser.

Studio must not:

- write trusted project, scene, asset, tilemap, source, config, or evidence files;
- treat tilemap draft preview metadata as an apply decision or persistence
  permission;
- treat asset-reference preview metadata as an apply decision, asset import,
  remote fetch, or persistence permission;
- execute shell commands, spawn local processes, install dependencies, or call a
  local write API;
- upload, fetch, or persist assets through a browser-trusted path;
- auto-apply, auto-accept, auto-promote, auto-rerun, auto-merge, or self-approve
  changes;
- become a production editor, visual scripting system, command bridge, plugin
  runtime, hosted service, or native export path; or
- close, replace, or narrow #1 or #23.

## Rust-trusted transaction boundary

A trusted Rust CLI boundary should own any durable operation introduced by later
implementation issues:

- parsing and validating draft JSON;
- checking project paths, manifest refs, hashes, schema versions, and stale
  preview state;
- producing transaction previews and visual diff evidence;
- enforcing explicit review-decision requirements before apply;
- writing rollback/audit/evidence records for approved applies; and
- rejecting unsupported file classes, source patch attempts, browser-originated
  writes, and generated-state drift.

Any implementation issue that adds persistence must include focused regression
tests for malformed drafts, stale evidence, denied review states, rollback/audit
records, and generated-state cleanup.

## Review-gated apply compatibility notes

Review-Gated Visual Edit Apply v1 records durable local evidence for later review, rerun, regression, and loop inspection. A visual edit application links the draft id, proposal id, patch draft id, accepted review decision id, transaction id, before/after scene hashes, rollback metadata, and reproducible CLI command context. Those links let existing review cockpit, journal, regression promotion, run matrix, loop status, bundle, and handoff surfaces show the operator what evidence exists and what remains missing.

The compatibility rule is intentionally conservative: apply evidence may inform runs, comparisons, regression promotion drafts, and loop blockers, but it does not schedule reruns, promote scenarios, mutate scenario packs, resume loops, or execute commands. Browser and Studio surfaces may render these fields as escaped read-only diagnostics and copyable inert command text only. Rust CLI/manual terminal actions remain the trusted boundary for any durable write or rerun.


## Visual Authoring Demo v1 display and public wording audit

The collect-and-exit visual authoring demo is documentation and evidence for a
local safe-edit workflow, not a public launch or productization milestone. Demo
Studio and dashboard pages may display Rust-exported read models, generated smoke
ids, escaped draft summaries, inert JSON, and copyable CLI command text. They
must describe those values as inspection aids only. Any durable validation,
transaction creation, review decision, apply, rerun, comparison, rollback, or
evidence write remains a Rust CLI/manual terminal action outside browser
JavaScript.

Public-facing wording for this demo must stay conservative:

- say **local demo**, **static cockpit**, **read-only dashboard**, **copyable
  command text**, **ignored generated evidence**, and **pre-release private MVP**;
- avoid **public launch**, **production editor**, **hosted Studio**, **visual
  scripting system**, **plugin runtime/marketplace**, **native export path**,
  **browser trusted writes**, **command bridge**, **automatic apply/rerun/merge**,
  and **Godot replacement** claims;
- identify tracked source-like fixtures separately from generated run,
  transaction, dashboard, comparison, and smoke outputs; and
- keep #1 and #23 open as governance/context anchors.

VA1.10.3 documentation changes are intentionally limited to this wording audit
and related smoke-test coverage. They do not add new Studio controls, dashboard
write paths, generated tracked artifacts, dependencies, or behavior changes.


## Scenario Coverage v5 / VA1.11.3 coverage matrix

VA1.11.3 closes the Scenario Coverage v5 documentation and Node compatibility
unit by recording the Studio/dashboard coverage matrix. This PR unit is
documentation and Node-test coverage only: it does not add product behavior, new
trusted write paths, generated tracked artifacts, dependencies, or broader
public/product claims.

| Coverage area | Existing source of truth | Studio / dashboard compatibility evidence | Guardrail preserved |
| --- | --- | --- | --- |
| Draft schema validation | Visual Edit Draft v1 fixtures and Rust validation tests | Studio may render draft ids, operation summaries, blocked reasons, and copyable inert preview command text only. | Browser draft state remains temporary and cannot persist trusted files. |
| Unsupported scene path rejection | Scene draft preflight and transaction preview rejection tests | Studio shows unsupported/blocked draft state as escaped diagnostics; dashboard may link the generated rejection evidence if exported. | Unsupported edits fail before writes and are not converted into apply controls. |
| Tilemap bounds and metadata | Tilemap draft preview fixtures/tests | Studio renders affected cells, hashes, collision/trigger metadata, and blocked bounds notes as read-only rows. | Tilemap previews do not write tilemaps, import assets, or imply review acceptance. |
| Asset reference type mismatch | Asset-reference draft and manifest validation tests | Studio/dashboard display manifest ids, replacement refs, content hashes, and mismatch diagnostics as escaped evidence. | No browser upload, fetch, asset import, manifest write, or remote dependency is authorized. |
| Draft-to-transaction hash mismatch | Rust trusted draft-preview hash preflight | Studio can show stale/hash mismatch warnings and copyable CLI text for manual rerun. | Hashes are stale-draft guards, not browser permission tokens. |
| Review-gated visual apply | Review decision ledger plus `visual_draft_applied` lifecycle evidence | Dashboard mutation lifecycle and Studio mutation surfaces show draft/proposal/patch/decision/transaction ids, before/after hashes, rollback/rerun context, and evidence refs as display-only state. | Review-gated apply remains a Rust CLI/manual terminal write; browser surfaces do not create decisions, apply drafts, rerun, or rollback. |
| Visual diff preview | `visual-diff-summary-v1` read model and focused Node smoke | Studio renders before/after summaries, operation summaries, collision/trigger counts, source refs, and scenario-impact notes as escaped read-only diagnostics. | Visual diff output is review evidence only and does not create apply, rerun, command-bridge, or browser persistence controls. |
| Dashboard/Studio generated-state policy | README/docs wording plus `git status --short --ignored` audit | Node smoke tests assert conservative wording and absence of browser persistence/command-execution APIs in static surfaces. | Drafts, transactions, previews, dashboard exports, runs, and smoke outputs remain generated/untracked unless explicitly fixture-scoped. |

VA1.11.3 Node compatibility evidence is the existing static surface gate:

```bash
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

The matrix above is intentionally conservative. It documents coverage for
Scenario Coverage v5 and keeps Studio/dashboard surfaces read-only while Rust
validation, transaction preview, review-gated apply, rollback/evidence writes,
and generated-state cleanup remain the trusted boundary.

### Known gaps and out-of-scope behavior

Scenario Coverage v5 is a regression suite, not a product-expansion milestone.
The coverage matrix intentionally does **not** claim:

- a production visual editor, visual scripting system, hosted Studio, plugin
  runtime, asset marketplace, native export, public launch, or Godot replacement;
- browser-side trusted file writes, uploads, fetch/import flows, command
  execution, local server bridges, review-decision creation, draft apply, rerun,
  rollback, auto-merge, or release automation;
- secure sandbox guarantees for arbitrary untrusted content; or
- committed generated runs, transactions, previews, dashboard exports, smoke
  outputs, screenshots, logs, or package bundles outside explicit source-like
  fixtures.

Remaining gaps after #353 are therefore roadmap scope, not regressions in this
suite: broader editor ergonomics, richer visual diff UI affordances, production
asset import, source mutation apply, export/build packaging, plugin execution,
and public launch readiness all require later issue contracts and fresh
guardrail evidence.

## Verification gates for follow-up issues

Every Visual Authoring v1 issue should run issue-specific focused checks plus the
broad gates required by its contract:

```bash
gh issue view <issue-number> --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
git status --short --ignored
```

If dashboard or Studio files change, also run the relevant Node gates:

```bash
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Closure evidence should record merged PRs in required order, latest-main
verification, generated/local artifact state, #1/#23 state, and an explicit audit
that Studio remains untrusted while Rust owns durable writes.

## Explicit non-goals

Visual Authoring v1 does not authorize:

- browser trusted writes, browser-local command bridges, hidden command execution,
  local server write APIs, dependency installation, or credentialed commands;
- source mutation, arbitrary patch apply, source patch apply to trusted main,
  branch merge/rebase automation, auto-merge, auto-apply, auto-accept, or
  self-approval;
- production editor, packaged editor, visual scripting, compatibility-stable
  engine API promises, production-ready claims, Godot replacement claims, or
  native export;
- plugin runtime, marketplace, dynamic extension loading, hosted/cloud/server
  infrastructure, database/auth, repository visibility changes, public launch
  automation, or package/binary publication;
- large asset drops, generated preview/checkpoint/run artifacts as tracked state,
  or remote asset dependencies unless a later issue explicitly scopes a tiny
  deterministic source-like fixture; or
- closing, replacing, narrowing, or superseding #1 or #23.

## Closure policy for this milestone

Visual Authoring v1 is complete only after the roadmap/#1 governance refresh for
that milestone records that all ordered follow-up issues have merged, latest-main
verification passed, generated/local artifacts remain untracked, and #1/#23 are
still open.

Completion of this milestone would be a bounded local safe-edit-draft claim. It
would not claim a production editor, visual scripting environment, browser write
path, native export flow, source mutation capability, plugin marketplace, hosted
service, public launch, or Godot replacement.
