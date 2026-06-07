# Scenario Coverage v30 — Generative Front Door Regression Suite

Issue: #1597. fixture-scoped regression for Generative Front Door v1 (#1592): intake
valid/malformed, promotion guard pass/gate-fail/over-solution, accessibility
verified/unverified, and review/apply/trust-gradient backward compatibility.

- Matrix: `examples/generative-front-door/scenario-coverage-v30/matrix.fixture.json`
- Runner: `examples/generative-front-door/scenario-coverage-v30-generative.test.cjs`
- Rust: `crates/ouroforge-core/tests/scenario_coverage_v30_generative_front_door.rs`

Browser/Studio read-only; no production-ready or Godot-replacement/parity claim.
#1 and #23 remain open.

proposal-only intake; promotion guard blocks without trusted write.
