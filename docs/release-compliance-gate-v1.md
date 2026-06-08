# Release Compliance Reviewer Gate v1

This gate composes with existing reviewer/evaluator gates and the per-release provenance bundle. It is not a new evaluator, not a writer, and not release authority.

The gate checks release-candidate content policy evidence, age-rating signals, and asset license/provenance/QA completeness. Missing license, missing provenance, policy violations, unreviewed age-rating signals, or missing human go/no-go evidence block the release candidate. Humans retain release go/no-go.

The surface is Rust/local and read-only. browser/Studio surfaces remain read-only. It executes no commands, applies no trusted write, auto-merges nothing, self-approves nothing, bypasses no reviewer, and makes no quality/fun, production-ready, Godot replacement/parity, or autonomous-shipping claim. Generated compliance evidence stays untracked unless fixture-scoped. #1 and #23 remain open.
