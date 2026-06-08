# Autonomous Producer Demo v1

Issue: #1686

This fixture-scoped demo shows a deterministic path from human design intent to release-candidate consideration with a full local audit trail, a mandatory human release gate, and a separate safe budget-halt case.

- `examples/autonomous-producer-v1/demo/demo.fixture.json` declares the deterministic stages: intent, concept, content, assets, QA, and release-candidate.
- `examples/autonomous-producer-v1/demo/release-gate.policy.json` reuses `producer-budget-gates-v1` and blocks release-candidate promotion at `gate-release` while the release owner gate is pending.
- `examples/autonomous-producer-v1/demo/budget-halt.policy.json` reuses the same budget/stop-condition contract to show a safe `halted-budget-exhausted` case with diagnosis evidence.

The demo is deterministic and requires no network or live browser. Rust/local validation owns the evidence checks. Browser, dashboard, and Studio surfaces may inspect the fixture read-only. Generated runs, assets, content, coverage, release-candidate artifacts, and local state remain untracked unless explicitly fixture-scoped.

The producer never performs a direct trusted write or release. Outputs are proposal-only through the existing review/apply/trust-gradient path, with mandatory human gates for vision, legal, and release go/no-go.

Conservative wording is preserved: no auto-merge, auto-apply, self-approval, reviewer bypass, production-ready claim, Godot replacement/parity claim, autonomous shipping claim, or quality/fun guarantee is introduced.

Issues #1 and #23 remain open.
