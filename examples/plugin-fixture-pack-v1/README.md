# Plugin Fixture Pack (v1)

A small, fixture-scoped pack of declarative plugins (#749) that proves valid
plugins contribute descriptors and unsafe plugins are blocked. It is consumed by
regression tests (and future demo issues) via
`plugin_registry::discover_plugins_in_dir`. Discovery is **read-only**: it never
executes plugin code, follows symlinks, descends hidden directories, traverses
outside the scan root, or installs plugins from the network.

## Valid plugins (contribute descriptors only)

- `valid/dashboard-panel/ouroforge.plugin.json` — read-only dashboard panel
- `valid/scenario-template/ouroforge.plugin.json` — read-only scenario template
- `valid/asset-metadata/ouroforge.plugin.json` — read-only asset metadata descriptor

## Invalid plugins (blocked diagnostics)

- `invalid/blocked-capability/ouroforge.plugin.json` — non-allowlisted capability
- `invalid/arbitrary-js/ouroforge.plugin.json` — smuggled executable entry field
- `invalid/unsafe-path/ouroforge.plugin.json` — path traversal outside the tree
- `invalid/legacy-schema/ouroforge.plugin.json` — unsupported (incompatible) schema

All fixtures are declarative descriptors only and contain no executable code.
