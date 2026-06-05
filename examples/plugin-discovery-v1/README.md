# Plugin Discovery Fixture (v1)

Fixture-scoped plugin tree used by the local plugin registry/discovery tests
(#740). Discovery is a **read-only** filesystem scan: it never executes plugin
code, follows symlinks, descends hidden directories, traverses outside the scan
root, or installs plugins from the network.

The tree exercises the registry's reported states:

- `plugins/read-only-dashboard-panel/ouroforge.plugin.json` — **valid**
- `plugins/broken-capability/ouroforge.plugin.json` — **invalid** (well-formed
  JSON that fails manifest validation)
- `plugins/legacy-schema/ouroforge.plugin.json` — **incompatible** (unsupported
  manifest schema version)
- `plugins/future-engine/ouroforge.plugin.json` — **future-version** (structurally
  valid but requires a newer Ouroforge engine; reported and blocked from
  extension contribution)
- `plugins/asset-metadata/ouroforge.plugin.json` — **valid** with a declarative
  asset metadata descriptor (read-only; no asset generation/import/export)

These manifests are declarative descriptors only and contain no executable code.
