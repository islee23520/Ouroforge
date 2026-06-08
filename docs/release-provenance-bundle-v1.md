# Per-Release Provenance Bundle v1

This contract extends the Milestone 25 provenance bundle to a per-release bundle by composition. It references existing per-change provenance, content, asset, QA, compliance, and release-candidate evidence; it does not create a new provenance engine and does not fabricate missing chain links.

A complete release bundle is replayable only when all required release links are present, referenced per-change Milestone 25 bundles evaluate complete, deterministic replay refs are present, and compliance plus human release go/no-go evidence is represented by reference. Missing links produce an explicit incomplete state. Dangling or stale refs are surfaced, not silently repaired.

The surface is Rust/local and read-only. browser/Studio surfaces remain read-only. The bundle executes no commands, applies no patch, auto-merges nothing, releases nothing, and makes no quality/fun, production-ready, Godot replacement/parity, or autonomous-shipping claim. Generated release provenance stays untracked unless fixture-scoped. #1 and #23 remain open.
