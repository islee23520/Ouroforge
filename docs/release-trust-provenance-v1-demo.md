# Release Trust, Provenance, and Compliance Demo v1

This deterministic fixture-scoped demo composes the existing release auto-apply,
release compliance gate, and per-release provenance bundle contracts. It is a
local evidence demo only: Rust/local validation owns the trusted logic, browser and Studio surfaces remain read-only, and the human release go/no-go remains
pending.

## Fixtures

- `examples/release-trust-provenance-v1/demo/eligible-auto-apply.json` shows an
  eligible low-risk proposal that can use the bounded release auto-apply tier
  because all required gates pass and a one-command game-scale rollback is
  present.
- `examples/release-trust-provenance-v1/demo/compliance.blocked-missing-license.json`
  shows the compliance gate blocking a release candidate when an asset license is
  missing.
- `examples/release-trust-provenance-v1/demo/release-bundle.complete.json` carries
  a complete per-release provenance bundle: intent, content, assets, QA,
  per-change provenance, compliance, release-candidate, deterministic replay, and
  fixture-scoped generated-state evidence.

## Conservative boundary

The demo performs no network access and requires no live browser. It does not add
a new runtime, writer, evaluator, compliance engine, or provenance engine. It
does not execute rollback commands, write trusted browser state, release,
publish, auto-merge, self-approve, bypass reviewers, or claim production-ready,
quality/fun, Godot replacement/parity, or autonomous shipping behavior. Generated
artifacts are tracked only because they are fixture-scoped. #1 and #23 remain open.
