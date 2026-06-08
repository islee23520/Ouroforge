# Scenario Coverage v41 — Scaled Trust and Release Provenance Regression Suite

Issue: #1695

Scenario Coverage v41 locks Scaled Trust Gradient, Release Provenance and
Compliance v1 behavior with deterministic state/shape regressions only. It
covers release auto-apply eligibility, rollback, manual fallback, kill-switch
handling, per-release provenance bundle states, compliance pass/block outcomes,
and backward compatibility for Milestone 22 per-change auto-apply and Milestone 25 per-change provenance.

Fixtures live under `examples/release-trust-provenance-v1/scenario-coverage-v41/`
and reference existing release auto-apply, release provenance, compliance,
trust-gradient, and provenance-bundle fixtures. The runner is Rust/local owned
and requires no network, no live browser, no timing checks, no browser command
bridge, and no trusted writes.

Browser, dashboard, and Studio surfaces may inspect read-only evidence only.
Generation and role-agent outputs remain proposal-only through the existing
review/apply/trust-gradient path. Generated runs, assets, release artifacts, and
coverage outputs stay untracked unless explicitly fixture-scoped.

Conservative wording is preserved: no autonomous apply beyond the bounded
low-risk contract, no auto-merge, no self-approval, no reviewer bypass, no production-ready claim, no quality/fun guarantee, no Godot replacement/parity claim, no autonomous shipping claim, and no release without compliance plus a
human go/no-go.

Issues #1 and #23 remain open governance anchors.
