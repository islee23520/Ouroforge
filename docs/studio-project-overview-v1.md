# Studio Project Overview v1

Issue: #758
Roadmap anchor: #1.
Status: read-only Studio aggregation surface.

The Studio project overview aggregates a project summary **read-only** from
exported JSON: project id / title / version, scene / asset / scenario / evidence
run counts, export status, plugin registry status, and a validation summary.
Missing or invalid sub-state (scene, asset, scenario, evidence, export, or
plugin) is surfaced as actionable diagnostics rather than failing silently.

## Boundary

- The overview aggregation is side-effect free: it reads exported JSON only and
  never mutates trusted files or runs commands. No trusted-write or
  command-execution control is added.
- Rust/local validation owns trusted persistence and validation; the Studio
  surface remains read-only.

This surface makes no production-ready or Godot replacement claim. #1 and #23 remain open.
