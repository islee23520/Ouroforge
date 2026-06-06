# Godot-Plus Capability Comparison Matrix v1

Issue: **#793**

This matrix is claim governance for the Signal Gate / `collect-and-exit` demo. A positive row means the repository has fixture-scoped evidence for that narrow vertical-slice capability. It does **not** claim broad Godot parity, Godot replacement, production readiness, commercial release readiness, or universal engine/editor superiority.

| Area | Demo proves in this repo | Concrete evidence | Remains behind Godot / explicit gap |
| --- | --- | --- | --- |
| Scene/node mental model | A small scene graph can describe entities, components, gameplay rules, and runtime debug metadata for one 2D vertical slice. | `examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json`; `examples/playable-demo-v2/collect-and-exit/scaffold-audit.test.cjs`; `examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs` | Godot has a mature native scene tree, native editor maturity, inspector, editor-time node lifecycle, and broad node ecosystem. |
| Editor / Studio inspection | The demo can be inspected through read-only or draft-only Studio/cockpit/dashboard surfaces without direct trusted source writes. | `examples/playable-demo-v2/collect-and-exit/README.md`; `examples/authoring-cockpit/cockpit.test.cjs`; `examples/evidence-dashboard/dashboard.test.cjs`; `examples/godot-plus-demo-studio-walkthrough-v790/godot-plus-studio-walkthrough-smoke.test.cjs` | Godot editor maturity remains ahead for native scene editing, previews, import workflows, debugging, and integrated authoring. |
| Gameplay logic | The small collect-key/open-door/reach-exit loop is represented and smoke-tested as fixture behavior. | `examples/playable-demo-v2/collect-and-exit/gameplay-loop-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/behavior-smoke.test.cjs` | No claim for large production titles, advanced physics, animation graphs, networking, complex AI, scripting ecosystem parity, or broad gameplay framework parity. |
| Export / package | The demo has a local web export/profile/package metadata contract with fail-closed boundary wording. | `examples/playable-demo-v2/collect-and-exit/export/export-profile.json`; `examples/playable-demo-v2/collect-and-exit/export/package-metadata.json`; `crates/ouroforge-core/tests/godot_plus_demo_export_package_contract.rs`; root `.gitignore` `/dist/` generated-output guard | No native/mobile/console platform export, store packaging, signing, upload, deploy, hosting, public release, or production packaging claim. |
| Plugin descriptors | The demo validates inert plugin descriptors through a local registry fixture. | `examples/playable-demo-v2/collect-and-exit/plugin-usage-v792-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/plugin-usage-evidence.json`; `examples/playable-demo-v2/collect-and-exit/plugins/*/ouroforge.plugin.json`; `examples/playable-demo-v2/collect-and-exit/plugins/registry/demo-plugin-registry-evidence.json` | No executable plugin runtime, marketplace/plugins ecosystem, plugin network install/update, dependency install, command bridge, or arbitrary extension ecosystem. |
| QA / playtest | The demo includes bounded QA/playtest and scenario smoke evidence for the vertical slice. | `examples/playable-demo-v2/collect-and-exit/qa-playtest-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/qa-swarm-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/scenario-matrix-smoke.test.cjs` | No production QA scale, production community validation, hosted telemetry, large device matrix, multiplayer soak, or release-candidate certification. |
| Evidence / journal | The demo keeps evidence-native read models and agentic iteration journals as fixture-scoped records. | `examples/playable-demo-v2/collect-and-exit/evidence-read-model-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/agentic-iteration/journal.json`; `examples/playable-demo-v2/collect-and-exit/agentic-iteration-smoke.test.cjs` | No claim that generated evidence, videos, screenshots, temp servers, or run artifacts are release artifacts; generated outputs remain untracked unless fixture-scoped. |
| Agentic mutation | The demo demonstrates a review-gated draft/proposal/decision chain, not direct autonomous writes. | `examples/playable-demo-v2/collect-and-exit/agentic-iteration/draft-proposal.json`; `examples/playable-demo-v2/collect-and-exit/agentic-iteration/review-decision.json`; `examples/playable-demo-v2/collect-and-exit/agentic-iteration-smoke.test.cjs` | No direct Studio trusted-source writes, auto-apply, auto-merge, self-approval, reviewer bypass, hidden writes, or arbitrary autonomous source mutation. |
| Asset pipeline | The demo has fixture-scoped asset manifest/provenance and pack smokes for the vertical slice. | `examples/playable-demo-v2/collect-and-exit/asset-manifest.json`; `examples/playable-demo-v2/collect-and-exit/asset-provenance.json`; `examples/playable-demo-v2/collect-and-exit/asset-pack-smoke.test.cjs`; `examples/playable-demo-v2/collect-and-exit/asset-evidence-smoke.test.cjs` | No broad import pipeline, marketplace/plugins asset ecosystem, external asset sync, DCC integration, asset-store workflow, or production asset build pipeline. |

## Wording audit

Allowed framing:

- “scoped evidence-native agentic workflow capability”
- “small playable vertical slice”
- “fixture-scoped evidence”
- “local web export/profile placeholder”
- “review-gated source mutation handoff”

Disallowed closure claims for this milestone:

- full Godot replacement or full Godot parity
- production-ready engine/editor/export claim
- commercial release, public deployment, app-store/Steam/itch publishing, signing, upload, or hosted release readiness
- executable plugin ecosystem, marketplace, network plugin install/update, dependency install, command bridge, or arbitrary shell/browser command bridge
- direct trusted source writes from Studio, auto-apply, auto-merge, self-approval, reviewer bypass, or hidden trusted writes

## Governance

- Before starting, before merge or closure, and after merge or closure, verify #793 state and confirm #1 and #23 remain open.
- Every positive claim in this document must stay tied to concrete repository evidence in the same matrix row.
- If future evidence expands the demo, add the new fixture or command beside the claim; do not promote a scoped row into a broad Godot replacement or production-ready claim.
- Protected issues #1 and #23 must remain open.

**#1 and #23 remain open.**
