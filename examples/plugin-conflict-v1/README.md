# Plugin Conflict Fixtures (v1)

Fixture-scoped plugin tree used by plugin load-order and conflict-detection tests
(#751). Discovery is **read-only**; conflict detection only *reports* collisions
and never resolves, merges, overrides, or orders plugin execution.

- `dup-a/ouroforge.plugin.json` and `dup-b/ouroforge.plugin.json` declare the
  same plugin id (`fixture-duplicate-plugin`) — a duplicate-plugin-id conflict.
- `panel-x/ouroforge.plugin.json` and `panel-y/ouroforge.plugin.json` declare the
  same dashboard panel descriptor id (`shared-panel-descriptor`) — a
  duplicate-descriptor-id conflict.

All fixtures are declarative descriptors only and contain no executable code.
