# Visual Authoring v1 Governance Handoff

Issue #354 records the roadmap and #1 governance refresh after completing Visual
Authoring v1 / Safe Local Edit Cockpit (#343-#354).

## Completed milestone summary

Visual Authoring v1 is complete as a bounded local-first safe edit-draft
milestone. The completed surfaces include:

- data-only visual edit draft schemas and source-like fixtures;
- Rust-owned scene, tilemap, and asset-reference draft validation/preflight;
- preview-only scene transaction, tilemap, asset-reference, and visual diff
  evidence;
- review-gated visual draft apply through the trusted CLI boundary;
- dashboard, journal, and Studio read models for draft/diff/review/apply
  evidence;
- Visual Authoring Demo v1 collect-and-exit source-like draft fixtures plus
  ignored generated smoke ids;
- Scenario Coverage v5 regression coverage; and
- conservative read-only Studio/dashboard wording.

## Reconfirmed guardrails and non-goals

The completion handoff does not authorize:

- browser trusted writes, uploads, command execution, command bridges, or local
  server bridges;
- browser persistence of trusted draft state;
- source mutation apply, arbitrary patch apply, dependency/CI/build-script
  mutation, auto-merge, hidden apply, auto-rerun, or auto-promotion;
- production editor, visual scripting, plugin runtime, asset marketplace, native
  export, hosted/cloud/server/auth, public launch automation, or Godot
  replacement claims; or
- tracked generated runs, transactions, previews, dashboard exports, screenshots,
  logs, package bundles, or smoke outputs outside explicitly source-like
  fixtures.

#1 remains the broad roadmap/vision anchor and #23 remains the repo-memory/design
anchor. Both stay open unless a separate explicit governance decision says
otherwise.

## Recommended next milestone candidates

Recommended next sequence after #354:

1. **Source Mutation Preview v1 (#356-#366)** — proceed only as inert
   preview/evidence work. It must not authorize source mutation apply or browser
   trusted writes.
2. **Public Alpha Readiness (#367-#377)** — run only after preview/evidence
   contracts and public wording remain conservative.
3. **Public Alpha Launch Governance (#378-#387)** — manual governance only; no
   automated repository visibility, package, binary, signing, app-store, Steam,
   itch, or commercial release path.

Native Export Design Gate, Plugin Design Gate, and Visual Authoring v2 remain
possible later governance topics, but they are not authorized implementation
scope by this handoff.
