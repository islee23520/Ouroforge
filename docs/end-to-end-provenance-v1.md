# End-to-End Provenance v1

Issue: **#1499** (Era E Milestone 25 scope and contract)

End-to-End Provenance v1 unifies existing provenance and evidence artifacts into one **per-change provenance bundle** a human can audit in read-only surfaces and replay for re-verification. This contract is **composition by reference** over scene/transaction provenance, rollback metadata, evidence links, review decisions, and regression promotion records. It does **not** introduce a separate provenance authority or new trusted browser write paths.

## Provenance bundle (additive schema)

Each trusted change SHOULD be describable as a bundle with ordered references:

1. **Intent** — design brief / mutation intent / journal entry refs.
2. **Artifact** — generated or edited source/scene/seed refs and hashes.
3. **Trusted validation** — seed/scene/project/asset validation evidence refs.
4. **Runtime observation** — world state, probe, or scenario runtime refs.
5. **Evaluator verdict** — scenario/evaluator verdict and assertion refs.
6. **Regression comparison** — before/after run comparison refs when applicable.
7. **Review / promotion** — review decision, promotion/rollback metadata refs.

Bundles are **additive**: workflows without a bundle remain valid. Rust/local tooling owns bundle composition and validation; dashboard/Studio render exported bundles read-only.

## Read-only audit surface contract

- One bundle per trusted change under review.
- Human sign-off is explicit; no auto-promotion or auto-merge from the audit surface.
- Studio/dashboard surfaces inspect bundle links only; they do not execute apply, merge, or shell commands.

## Replayability contract

A merged change SHOULD be reconstructable from its bundle references and re-verified locally (fixture-scoped or ignored generated runs). Replay results are enumerated (for example **reproduced**, **diverged**, **not-replayable**) in follow-up issues; this scope doc defines the contract only.

## Reuse and compatibility

- **Compose by reference** to existing provenance/evidence/review/promotion contracts.
- **Backward-compatible** and **additive**; no breaking changes to existing artifact shapes in this issue.
- Generated bundle outputs remain **untracked** unless explicitly scoped as source-like fixtures.

## Boundaries

Conservative wording: audit/replay workflow evidence, not production readiness,
quality guarantees, or engine-replacement positioning.

**#1 and #23 remain open.**
