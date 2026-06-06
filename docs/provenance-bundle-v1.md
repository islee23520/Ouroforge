# Provenance Bundle Model v1

Issue: **#1500**.

Provenance Bundle Model v1 is an additive, read-only audit and replay surface. A bundle composes references to intent/design brief, generated or edited artifact, validation result, runtime observation, evaluator verdict, regression comparison, journal or review decision, and promotion or rollback record.

The bundle composes by reference only. It reuses existing transaction provenance, rollback metadata, evidence links, Milestone 20 provenance classes, review decisions, regression comparisons, and promotion records instead of duplicating those records.

Missing links, dangling references, stale references, and declared incomplete states are explicit incomplete states. The model never fabricates a complete chain.

Generated bundles remain untracked unless fixture-scoped.

Compatibility boundary: existing artifacts remain valid without a bundle, and browser/Studio/dashboard consumers inspect exported bundles read-only. Rust/local tooling owns validation and any later scoped replay reconstruction. This is not a quality guarantee, production-readiness claim, or Godot replacement claim.

Governance: #1 remains open and #23 remains open. This document is scoped to #1500 only and does not implement #1502, #1503, #1504, #1505, or #1506.
