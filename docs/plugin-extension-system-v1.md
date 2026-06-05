# Plugin / Extension System v1 Scope and Contract

Issue: #738
Roadmap anchor: #1 (Plugin / Extension System milestone).
Status: scope contract only; no executable behavior.

Plugin / Extension System v1 is a declarative, allowlisted, evidence-backed extension foundation. It lets Ouroforge discover local plugin manifests, validate declared capabilities, register explicitly allowed extension descriptors, expose read-only inspection surfaces, and record evidence. It does not authorize executable plugins, arbitrary JavaScript, native extensions, marketplace behavior, network install/update, command execution, dependency installation, source mutation, or publish/deploy actions.

This document is the canonical contract for all follow-up plugin/extension issues. It adds no executable plugin behavior; each follow-up issue implements one bounded slice against the boundaries defined here.

## Bounded target

The milestone covers the following bounded capabilities, each implemented and verified independently:

- Plugin manifest schema: a validated, declarative manifest with no executable entry points.
- Local registry and discovery: deterministic discovery of local plugin manifests into a registry.
- Extension point catalog: the explicitly allowed v1 catalog of extension points.
- Capability/permission model: declared capabilities validated against the allowlist.
- Version compatibility: declared version/compatibility checks.
- Evidence integration: validation and discovery evidence artifacts.
- Studio read-only plugin browser: a read-only browser for discovered plugins.
- Read-only dashboard panel descriptor: a declarative, render-only panel descriptor.
- Scenario template descriptor: a declarative scenario template descriptor.
- Asset metadata descriptor: a declarative asset metadata descriptor.
- Fixture plugin pack: a fixture-scoped pack exercising the descriptors.
- Security/threat model gate: an explicit threat-model gate that fails closed on executable/credentialed/network capabilities.
- Load-order/conflict detection: deterministic load-order and conflict detection.
- CLI inspection, regression suite, demo, and roadmap governance refresh.

## Trusted boundary

- Plugins may declare extension points and metadata only within the explicitly allowed v1 catalog. They never execute code.
- There is no arbitrary JavaScript execution, native dynamic library loading, shell command execution, dependency installation, network plugin install/update, marketplace, credential access, source mutation, export mutation, publish/deploy action, or CI/workflow mutation.
- Rust/local validation owns trusted plugin discovery, manifest validation, registry persistence, capability/permission checks, compatibility checks, evidence writing, run/project binding, and CLI contracts.
- Browser, dashboard, and Studio surfaces remain read-only inspection/rendering surfaces. They cannot install, update, delete, enable executable code, run commands, publish, deploy, sign, upload, or write trusted files. Plugin-provided content is rendered as inert data, never as executable code.

## Generated-state policy

Plugin outputs, generated registries, validation reports, evidence artifacts, fixture outputs, and local tool state remain ignored unless explicitly fixture-scoped. Each follow-up PR includes a generated-state audit (`git status --short --ignored`).

## Dependency order for follow-up issues

1. This scope and contract issue (#738) lands first.
2. The manifest schema, capability/permission model, and extension point catalog land before discovery.
3. Local registry and discovery, version compatibility, and the security/threat model gate build on the schema and catalog.
4. Evidence integration and load-order/conflict detection build on the registry.
5. The Studio plugin browser, dashboard panel descriptor, scenario template descriptor, asset metadata descriptor, and fixture plugin pack proceed once discovery and evidence exist.
6. CLI inspection, the regression suite, and the demo follow.
7. A roadmap and #1 governance refresh closes the milestone.

Each follow-up issue must verify its slice independently and must not combine manifest, registry, extension catalog, capabilities, compatibility, evidence, Studio, descriptors, security, CLI, regression, and demo behavior into a single PR when they can be verified separately.

## Verification and closure gates

Every follow-up PR must pass the standard repository gates (`cargo fmt --check`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, the dashboard/cockpit node smokes, `git diff --check`, and a clean `git status --short --ignored`) and must add focused tests/smokes for the exact plugin/extension behavior it implements. Closure evidence must include generated-state, no-executable-plugin, no-network-install, no-command-execution, no-publish/deploy, evidence, conservative-wording, and #1/#23 governance audits.

## Explicit non-goals

- No executable plugins, arbitrary JavaScript, native extensions, dynamic library loading, runtime script plugin execution, or editor tool scripts.
- No marketplace, plugin install/update from network, dependency installation, package manager integration, credential access, or remote trust model.
- No shell command execution, browser command bridge, local server command bridge, hidden trusted writes, source mutation, CI/workflow mutation, export/publish/deploy mutation, signing, upload, or release automation.
- No production-ready plugin ecosystem, secure plugin sandbox, marketplace readiness, Godot-equivalent extension parity, or current Godot replacement claim.
- No generated plugin registry/evidence artifacts committed unless explicitly fixture-scoped.
- No unrelated full editor, native export, store publishing, or Godot-plus demo implementation.

## Governance anchors

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor. Both remain open through this contract issue; this milestone does not close or modify either without a separate explicit governance decision.
