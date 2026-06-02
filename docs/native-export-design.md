# Native Export Design Gate

Status: **EE12.1 feasibility audit draft; ADR decision pending in EE12.2**

Issue: #168 — Engine Expansion v1 Native Export Design Gate

This document is the canonical native-export design artifact. EE12.1 audits
feasibility only. It does not approve native export, create implementation
issues, add build systems, scaffold Tauri/Electron/native shells, or change the
browser-first evidence loop.

## Current browser-first architecture

Ouroforge currently proves behavior through a local, browser-first,
evidence-native loop:

1. A Seed file declares a target and bounded scenarios.
2. The CLI creates a local run directory under `runs/`.
3. Local browser workers execute the static runtime through Chrome DevTools
   Protocol.
4. Scenario evidence captures world state, frame stats, console/CDP summaries,
   screenshots, verdicts, journals, and comparison artifacts.
5. Dashboard and cockpit surfaces read or preview generated artifacts; browser
   JavaScript does not write trusted project state.

Authoritative contracts today:

- Rust validates Seed, scenario, scene, evidence, dashboard, comparison,
  mutation, and journal artifacts.
- JavaScript owns static runtime behavior and static read-only/preview UIs.
- `runs/` and generated dashboard data are local inspection state and remain
  untracked.
- The browser runtime is the canonical executable environment for scenario QA.

Native export must not weaken any of those contracts.

## Concrete use cases that could justify export

Native export would need a specific evidence-backed use case. Plausible future
needs include:

- offline desktop inspection of already-generated runs without a local static
  server;
- distribution of a static runtime demo to reviewers who cannot run Cargo;
- controlled packaging of a single evidence-native template for smoke review;
- preserving local-only operation while adding OS window/menu integration.

Current #167 evidence does not prove that these needs outweigh the added
platform complexity. The present workflow already supports local browser runs,
dashboard export, and compare artifacts.

## Requirements if native export is ever adopted

Any future export approach must preserve:

- **Seed authority**: exported artifacts must be traceable to a validated Seed.
- **Run authority**: exported output must link to a `runs/<run-id>` or equivalent
  validated run manifest.
- **Scenario authority**: exported behavior must remain reproducible by the same
  scenario/evaluator commands.
- **Evidence authority**: screenshots, world state, frame stats, verdict,
  journal, and comparison artifacts must remain machine-readable.
- **Rust validation authority**: Rust artifact validation remains canonical;
  shell/browser code must not bypass it.
- **Browser-first parity**: native export must not replace browser-worker QA or
  make unverified platform behavior the source of truth.
- **Local-only boundary**: no server, cloud, auth, telemetry, marketplace,
  updater, installer, signing, notarization, or hosted deployment may be implied
  by the export gate.

## Feasibility gaps

Native export is not currently implementation-ready because the repository lacks
explicit contracts for:

- packaging manifests and bundle layouts;
- exported asset integrity and provenance;
- deterministic replay inside a packaged shell;
- platform-specific browser/webview differences;
- artifact path rewriting for packaged resources;
- CI coverage across desktop or mobile targets;
- installer, signing, notarization, updater, crash-reporting, and security
  boundaries;
- user-visible claims about compatibility, support, or production readiness.

None of these gaps should be solved inside #168. They are design inputs for the
ADR decision in EE12.2.

## Alternatives considered for the audit

| Alternative | Fit with current evidence loop | Main benefit | Main risk |
| --- | --- | --- | --- |
| Keep browser-first local workflow | Strong | No new platform complexity; preserves current QA source of truth | No packaged desktop artifact |
| Static hosting / static archive | Medium | Shares existing HTML/JS assets with minimal code change | Can drift into public launch/deployment claims |
| Browser packaging via existing static server instructions | Medium | Reviewer can run with known browser behavior | Still requires local setup and Chrome/Chromium |
| Tauri-style shell | Unproven | Rust-backed shell can wrap web assets through a webview/process model | Adds platform build, webview variance, IPC/security boundaries, and Rust shell code |
| Electron-style shell | Unproven | Mature browser-like packaging with main/renderer separation | Adds Node/native shell surface, larger runtime, IPC/security boundaries, and packaging work |
| Custom native runtime/renderer | Poor | Maximum control if a native engine eventually exists | Replaces the browser-first runtime and risks large engine scope |
| Defer native export | Strong | Keeps Engine Expansion v1 focused on evidence-native browser proof | Requires explicit revisit criteria to avoid ambiguity |

Reference context checked for the audit: official Tauri architecture/process
model documentation describes a Rust core plus webview message-passing model
(<https://v2.tauri.app/concept/architecture/>,
<https://tauri.app/concept/process-model/>); official Electron process-model
documentation describes main/renderer process separation and preload-mediated
capability boundaries (<https://www.electronjs.org/docs/latest/tutorial/process-model>).
Those models are relevant only as comparison inputs, not as selected implementations.

## Feasibility findings

1. The browser-first loop is currently sufficient for #167's playable template:
   workers=4 runs, scenario verdicts, dashboard export, and compare artifacts
   inspect the template without native packaging.
2. Native export would introduce platform-specific behavior that is not covered
   by current evidence artifacts.
3. The most dangerous drift path is treating a shell wrapper as proof of engine
   maturity or public launch readiness.
4. If export is revisited, the first implementation issue would need to define a
   narrow artifact contract before any shell/build system exists.
5. A custom native runtime is not compatible with the current milestone because
   it would bypass the completed browser runtime proof path.

## No-code / no-scaffold audit

EE12.1 intentionally changes documentation only. It adds no:

- native export implementation;
- Tauri, Electron, Wry, WebView2, WKWebView, Android, iOS, desktop, or mobile
  scaffold;
- platform-specific build config;
- packaging, installer, signing, notarization, updater, or deployment workflow;
- server, database, cloud, auth, telemetry, marketplace, or plugin mechanism;
- generated `runs/` or dashboard artifacts.

## ADR inputs for EE12.2

EE12.2 should make an explicit GO/NO-GO decision using this audit. The decision
should answer:

- Is native export justified now by evidence-backed user needs?
- Which alternative is selected or rejected?
- What Rust artifact contracts remain canonical?
- If GO, what exact follow-up issues are required before any implementation?
- If NO-GO, what concrete revisit criteria would change the decision?

Until EE12.2 is merged, this document records feasibility findings only and does
not authorize implementation.
