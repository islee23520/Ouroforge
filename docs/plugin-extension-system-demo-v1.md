# Plugin / Extension System Demo v1

Issue: #754 — #1 Plugin / Extension System milestone (declarative local extension
foundation, not an executable plugin ecosystem).

## What this demo is

A read-only walkthrough of the full v1 plugin/extension flow over the fixture
plugin pack. It composes the existing plugin pipeline — local registry
discovery, manifest validation, valid descriptor rendering, blocked diagnostics
for invalid fixtures, plugin evidence output, and read-only dashboard/Studio
inspection.

The demo discovers **declarative descriptors only**. It never executes plugins,
loads native libraries, runs shell commands, installs dependencies, contacts the
network, mutates source, publishes, or deploys.

## Stages

| Stage id | Coverage | Focused check | Expected result |
| --- | --- | --- | --- |
| `BEP754.discover` | Local registry discovery scans the fixture pack read-only. | `discover_plugins_in_dir` over `examples/plugin-fixture-pack-v1`. | Deterministic registry with valid, invalid, and incompatible entries; no plugin code is executed. |
| `BEP754.validate-valid` | Valid plugins contribute declarative descriptors only. | Valid entries expose declared capabilities and extension points. | Valid descriptors appear in the registry/read model; nothing is run. |
| `BEP754.validate-invalid` | Invalid plugins (arbitrary JS, blocked capability, unsafe path) fail closed. | Invalid entries report diagnostics and contribute no extension points. | Blocked diagnostics are visible; invalid plugins contribute nothing. |
| `BEP754.incompatible` | A legacy-schema plugin is reported as incompatible. | Registry read model incompatible count. | The incompatible plugin is surfaced without execution. |
| `BEP754.evidence` | Plugin registry evidence validates and exposes a read model. | `PluginRegistryEvidenceArtifact::from_json_str` over the valid evidence sample. | Evidence parses and the read model serializes for read-only inspection. |
| `BEP754.evidence-blocked` | Evidence declaring executable capability or unsafe descriptors fails closed. | `PluginRegistryEvidenceArtifact::from_json_str` over an invalid evidence sample. | Validation rejects the unsafe evidence before it is treated as trusted. |
| `BEP754.read-only-inspection` | Dashboard/Studio inspection is read-only. | Registry read model boundary string. | The boundary states read-only, declarative discovery only, with no plugin execution and no trusted writes. |

## Commands

```bash
cargo test -p ouroforge-core --test plugin_extension_system_demo
```

## Fixture policy

The demo manifest under `examples/plugin-extension-system-demo-v1/`, the fixture
plugin pack under `examples/plugin-fixture-pack-v1/`, and the evidence samples
under `examples/plugin-registry-evidence-v1/` are small, deterministic, and
fixture-scoped. Generated registries, validation reports, and evidence artifacts
remain generated and ignored unless fixture-scoped.

## Non-goals and guardrails

This demo does not authorize executable plugins, arbitrary JavaScript, native
extensions, dynamic library loading, shell command execution, dependency
installation, network plugin install/update, marketplace behavior, credential
access, source mutation, export mutation, publish/deploy action, command bridge,
or CI/workflow mutation. Browser/dashboard/Studio surfaces remain read-only. No
production-ready plugin ecosystem, secure plugin sandbox, marketplace readiness,
Godot-equivalent extension parity, or current Godot replacement claim is made.

## Known gaps

- The demo renders descriptors only; it does not load or run any plugin behavior.
- Version-compatibility surfacing is limited to the declared schema/engine
  ranges in the fixture pack.

## Governance

- #1 remains open.
- #23 remains open.
