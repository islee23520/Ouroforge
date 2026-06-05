# Plugin / Extension System — Threat Model (v1)

This document fixes the plugin threat model for v1 so the milestone cannot drift
into unsafe code execution, supply-chain, or privileged mutation behavior. v1 is
a **declarative, allowlisted, evidence-backed** extension foundation. It does not
authorize executable plugins, arbitrary JavaScript, native extensions, marketplace
behavior, network install/update, command execution, dependency installation,
source mutation, or publish/deploy actions.

The model is encoded as data in `crates/ouroforge-core/src/plugin_threat_model.rs`
(`THREAT_MODEL`), with a `gate()` that fails closed if a threat is left
uncontrolled. The contract tests exercise each highest-risk path against the real
validators, so this document and the enforced controls stay in sync.

## Trust boundary

Rust/local trusted code owns plugin discovery, manifest validation, registry
persistence, capability/permission checks, compatibility checks, and evidence
writing. Browser/dashboard/Studio surfaces are **read-only**: they render
allowlisted descriptors and cannot install, update, delete, enable executable
code, run commands, publish, deploy, sign, upload, or write trusted files.

## Threats and fail-closed controls

| Threat | Risk | Control | Status |
| --- | --- | --- | --- |
| `untrusted-manifest` | Manifest declares unsupported fields/values | `deny_unknown_fields` + allowlist validation | Blocked |
| `symlink-escape` | Symlinked manifest points outside the tree | Discovery never follows symlinks; recorded as blocked | Blocked |
| `path-traversal` | Path/reference escapes the plugin tree | Reject leading `/`, `..`, and `\` separators | Blocked |
| `arbitrary-js` | Smuggled JS / executable entry point | Reject unknown executable fields; script/eval text fails closed | Blocked |
| `native-extension` | Native dynamic library / extension loading | `native_extension` permission + native extension points blocked | Blocked |
| `dependency-install` | Dependency installation | `install_dependency` permission + install points blocked | Blocked |
| `network-install` | Network install/update or remote sources | URL text fails closed; no network access performed | Blocked |
| `credential-access` | Credential access | `access_credentials` permission + credential text fail closed | Blocked |
| `source-mutation` | Source mutation | `write_source` permission + source/write points blocked | Blocked |
| `export-publish-deploy` | Export/publish/deploy mutation | `publish_export` permission + export/publish/deploy points blocked | Blocked |
| `ci-mutation` | CI/workflow mutation | `mutate_ci` permission + CI/workflow points blocked | Blocked |
| `studio-rendering` | Plugin content rendered as code | Read-only descriptors with explicit read-only boundaries | Mitigated |

## Fail-closed validation policy

All plugin inputs are validated before use, and validation rejects anything not
explicitly allowlisted. Unknown capabilities, extension points, permissions,
asset types, field types, schema versions, and descriptor kinds fail closed with
actionable diagnostics. Validation performs no command execution, code loading,
network access, or trusted writes.

## Future-work boundary (explicitly out of scope for v1)

Executable plugins, runtime script plugins, native extensions, marketplace,
network install/update, dependency installation, and any privileged mutation are
**not** part of v1 and are not enabled by this milestone. Any such capability
requires a separate, explicitly-authorized governance decision and its own threat
model; it must never be introduced as a side effect of v1 work.

## Closure-evidence checklist

Closure evidence for plugin issues should reference the threat-model checklist
ids (`plugin_threat_model::checklist_ids()`) and confirm each remains controlled:
`untrusted-manifest`, `symlink-escape`, `path-traversal`, `arbitrary-js`,
`native-extension`, `dependency-install`, `network-install`, `credential-access`,
`source-mutation`, `export-publish-deploy`, `ci-mutation`, `studio-rendering`.
