# M115 Historical Milestone Classification Ledger (initial seed)

**For:** #2344  
**Classification date:** 2026-06-10 (live from #1 rebaseline)  
**Scope:** Major eras A–R (contract-complete baseline); current collect-and-exit as example.

| Era/Milestone Group | Classification | Evidence Refs | Gap / Product-Observed Notes | Confidence |
|---------------------|----------------|---------------|------------------------------|------------|
| A–E (Foundation + early agentic) | contract-complete | docs/roadmap/milestones/*.md, scenario-coverage-*.md, prior PRs | No live dogfood for engine usability claims at the time | High |
| F–H (Godot-class core + autonomy) | contract-complete | production-2d-*.md, autonomous-*.md, dogfood-*.md (B series) | Contract fixtures + tests; recent B5–B8 dogfood are evidence but scoped | High |
| I–R (Genre vertical, 2D/2.5D/3D on-ramp, semantic re-derivation) | contract-complete | docs/roadmap/milestones/era-*.md, godot-plus-demo-*.md (superseded), 2-5d-*, deterministic-re-expression-* | One-way import + clean-room only; no live bridge; Godot-plus demo closed as vertical slice evidence, not production engine | High |
| Current collect-and-exit (runtime probe) | contract-complete (fixture) | examples/game-runtime/, docs/playable-demo-v2-collect-and-exit.md, #1 rebaseline note | Fails product-observed bar for practical game-production engine UX (minimal probe + raw JSON); see M118–M120 for corrective | High |
| M115 (this) | contract-complete (semantics/ledger) | This doc, #2343–#2345, #1 updated body | Enables later product-observed claims; no usability impl here | High |

**Rule:** Future issues in Eras S–W must cite this ledger or the canonical semantics doc and attach live evidence for any 🟩 claim. Unresolved gaps go to backlog (#2383 etc.) rather than hidden behind completion language.

**Audit:** `git status --short --ignored` must be clean of generated pollution for trusted changes.
