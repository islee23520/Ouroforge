# Autonomous QA Playtest Demo v1

Issue: #696 — Autonomous QA Playtest Demo v1.

This demo shows bounded, evidence-gated QA/playtest exploration on a bounded
playable fixture project. It wires scenario candidates, fuzz plans, worker
assignments, invariant checks, route attempts, visual/performance/error
evidence, flake handling, failure classification, backlog items, a run matrix,
and a QA evidence bundle into one demonstrable pipeline. It does **not** perform
auto-fix, auto-apply, or run hidden workers.

The demo is described by a manifest validated by
`ouroforge_core::qa_playtest_demo`. The manifest references each stage's evidence
by path; running the demo produces generated evidence under the declared output
roots, which stay untracked.

## Commands

```bash
# Bounded, local, review-gated QA/playtest demo (illustrative).
cargo run -p ouroforge-cli -- qa-demo plan   --manifest examples/qa-playtest-demo-v1/demo.manifest.json
cargo run -p ouroforge-cli -- qa-demo collect --manifest examples/qa-playtest-demo-v1/demo.manifest.json --out runs/qa-playtest-demo
cargo run -p ouroforge-cli -- qa-demo bundle  --run runs/qa-playtest-demo
```

These commands are illustrative of the bounded flow; the manifest and fixtures in
this PR are the authoritative, validated artifacts.

## Expected evidence

The manifest's `expectedEvidenceRefs` list the evidence each stage should
produce: invariant checks, route attempts, visual/performance/error evidence,
flake handling, failure classification, backlog, run matrix, and the final
evidence bundle. All evidence remains an input for review.

## Known gaps

- The demo runs on a bounded fixture project, not a full game; coverage is
  intentionally narrow.
- Visual, performance, and error signals are heuristic evidence inputs, not
  proof of fun, quality, or production safety.
- Stage evidence is illustrative fixture data; live capture is out of scope for
  v1 and is left as a follow-up.

## Cleanup policy

Generated runs, fuzz inputs, screenshots, traces, and bundle exports are written
only under the declared output roots (`runs/qa-playtest-demo/...`) and are
deleted after review. They are never tracked in git.

## Boundary and governance

QA/playtest outputs are evidence and backlog inputs only and remain review-gated
until reviewed. Reruns, fuzz inputs, and workers are bounded; there are no hidden
workers, no remote swarm, no auto-fix, no auto-apply, and no auto-merge. Browser/
dashboard/Studio surfaces remain read-only or draft-only.

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open. Public wording stays conservative:
this is bounded evidence-gated QA exploration, not a production game, a
shipped-game claim, a current Godot replacement, or a production-ready claim.
