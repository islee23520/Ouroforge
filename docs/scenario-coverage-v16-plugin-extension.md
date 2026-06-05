# Scenario Coverage v16: Plugin Extension Regression Suite

Issue: #753 — Scenario Coverage v16: Plugin Extension Regression Suite.  
Status: PES10.16.3 verification integration.

Scenario Coverage v16 locks Plugin / Extension System v1 as a declarative,
allowlisted, evidence-backed plugin descriptor surface. It is a regression suite,
not a product-expansion milestone. It does not add executable plugins, arbitrary
JavaScript, native extensions, dependency installation, network install/update,
command execution, source mutation, publish/deploy behavior, marketplace
behavior, or a production-ready plugin ecosystem.

## Coverage intent

The suite covers both accepted declarative plugin descriptors and blocked unsafe
plugin drift. Rust/local trusted validation owns manifest/evidence checks,
registry persistence, compatibility checks, generated evidence writing, and CLI
behavior. Dashboard and Studio surfaces remain escaped read-only inspection
surfaces over validated evidence.

Generated plugin registries, validation reports, evidence artifacts, fixture
outputs, run outputs, dashboard exports, screenshots, local plugin state, and
local tool state remain ignored unless a later issue explicitly scopes them as
fixtures.

## Success scenario definitions

| Scenario id | Fixture/evidence input | Expected result | Boundary locked |
| --- | --- | --- | --- |
| PES10.16.valid-manifest | Valid declarative manifest evidence with local manifest path, hash, version, and allowlisted capabilities. | Accepted as plugin registry evidence. | Metadata only; no executable plugin runtime. |
| PES10.16.registry-discovery | Fixture-scoped registry containing multiple plugin descriptors. | Registry read model reports plugin count, blocked count, capabilities, and extension points. | Discovery is evidence-backed and local; no network install/update. |
| PES10.16.dashboard-panel | Read-only dashboard panel descriptor using allowlisted data source, template, and layout. | Dashboard read model and browser display render escaped metadata. | No JavaScript execution, command hooks, trusted writes, or remote templates. |
| PES10.16.studio-display | Studio/cockpit plugin registry inspection over the same read model. | Studio display remains read-only and shows blocked reasons. | Browser/Studio cannot install, update, delete, enable executable code, or write trusted files. |
| PES10.16.scenario-template | Declarative scenario template descriptor with bounded parameters, supported game types, expected evidence type, and validation hints. | Accepted as metadata for a trusted scenario runner to interpret later. | Template metadata does not own QA execution, scripts, command hooks, network fetches, or source mutation. |
| PES10.16.asset-metadata-descriptor | Asset metadata provider capability and `assets.metadata.readOnly` extension point. | Accepted only as metadata descriptor evidence. | No asset generation, remote asset loading, marketplace, or trusted source writes. |
| PES10.16.compatible-version | Compatible manifest version with explicit hash and compatibility status. | Accepted and surfaced as compatible in read models. | Compatibility is status evidence, not a production-stable plugin API claim. |
| PES10.16.evidence-bundle | Plugin evidence bundle with generated-state policy and evidence refs. | Evidence writes under generated run evidence roots and indexes dashboard/Studio inspection data. | Runtime outputs remain generated/ignored unless fixture-scoped. |

## Failure scenario definitions

| Scenario id | Blocked input | Expected diagnostic class | Boundary locked |
| --- | --- | --- | --- |
| PES10.16.block-arbitrary-js | Descriptor contains inline script, remote JavaScript template, event handler, or executable code claim. | Block before acceptance with JavaScript/executable descriptor diagnostic. | No arbitrary JavaScript or runtime plugin execution. |
| PES10.16.block-command-execution | Capability, validation hint, or descriptor requests shell command execution or command hook. | Block before execution with command authority diagnostic. | No shell command execution, browser command bridge, or local server command bridge. |
| PES10.16.block-dependency-install | Descriptor requests package-manager, dependency install, or update authority. | Block with dependency install/network trust diagnostic. | No dependency installation or package-manager integration. |
| PES10.16.block-network-install-update | Descriptor references network plugin install/update, remote registry, remote template, or remote asset loading. | Block with network install/update diagnostic. | No marketplace or remote trust model. |
| PES10.16.block-credential-access | Descriptor requests secrets, tokens, credentials, private paths, or environment access. | Block with credential access diagnostic. | No credential access or secret exfiltration path. |
| PES10.16.block-source-mutation | Descriptor requests source mutation, trusted file writes, auto-apply, or apply hooks. | Block with source/trusted-write diagnostic. | No source mutation, hidden trusted writes, auto-apply, or self-approval. |
| PES10.16.block-export-publish-deploy | Descriptor requests export, package, publish, deploy, upload, signing, or release automation. | Block with publish/deploy authority diagnostic. | No native export, publish/deploy mutation, signing, upload, or release automation. |
| PES10.16.block-path-traversal | Manifest path, evidence ref, generated root, parameter name, or descriptor ref escapes the local fixture/root. | Block with unsafe path/local id diagnostic. | No path traversal or hidden writes. |
| PES10.16.block-duplicate-ids | Duplicate plugin ids, evidence ids, panel ids, or template ids. | Block with duplicate id diagnostic. | Registry remains deterministic and attributable. |
| PES10.16.block-incompatible-version | Unsupported or incompatible manifest/API version. | Block or mark incompatible with visible reason. | No broad compatibility-stable public plugin API claim. |
| PES10.16.block-native-extension | Descriptor requests native dynamic library loading, binary plugin, editor script, or runtime extension code. | Block with native extension/executable plugin diagnostic. | No native extension or dynamic library loading. |
| PES10.16.block-ci-mutation | Descriptor requests workflow, CI, dependency graph, or release configuration mutation. | Block with CI/workflow mutation diagnostic. | No CI/workflow mutation or reviewer bypass. |

## Verification integration

PES10.16.3 integrates the Scenario Coverage v16 matrix with local verification
without adding plugin runtime scope. The existing `cargo test` gate runs the
focused Rust plugin evidence contract tests that audit this document, validate
the success and blocked fixture matrices, and reject unsafe fixture drift. The
existing dashboard and cockpit Node smoke tests now read this document as an
integration sentinel so browser-facing verification fails if the Scenario
Coverage v16 local-gate wording, read-only display boundary, or no-executable /
no-network / no-command / no-publish guardrails are removed.

Required local verification for each slice remains:

```bash
gh issue view 753 --repo shaun0927/Ouroforge
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

Issue-specific evidence must include generated-state, no-executable-plugin,
no-network-install, no-command-execution, no-publish/deploy, wording,
fixture-scope, local cargo/node integration, and #1/#23 governance audits.

## Explicit non-goals

Scenario Coverage v16 does not authorize:

- executable plugins;
- arbitrary JavaScript;
- native extensions or dynamic library loading;
- editor tool scripts;
- marketplace behavior;
- plugin install/update from network;
- dependency installation or package-manager integration;
- credential access;
- shell command execution;
- browser command bridges or local server command bridges;
- hidden trusted writes;
- source mutation;
- CI/workflow mutation;
- export/publish/deploy mutation, signing, upload, or release automation;
- production-ready plugin ecosystem claims;
- secure plugin sandbox claims;
- Godot-equivalent extension parity;
- current Godot replacement claims;
- unrelated full editor work, native export, store publishing, or Godot-plus demo implementation.

#1 remains the broad roadmap/governance anchor and #23 remains the protected
repo-memory/design context anchor. Both must stay open unless a separate explicit
governance decision exists.
