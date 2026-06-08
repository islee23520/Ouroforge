# Design-Intent Decomposition and Production Plan v1

Issue: #1683

This contract decomposes a human design intent into a deterministic, proposal-only production plan. It reuses Milestone 30 generation references and the existing GDD/design-brief surfaces: design brief, requirement extraction, mechanics mapping, feasibility, scaffold plan, scene/level plan, gameplay behavior plan, asset placeholder plan, scenario acceptance plan, and prototype task graph.

The producer plan is not a new generator. It is a task graph of function-agent work and expected Rust/local evidence. Browser and Studio consumers receive a read model only. Any later source change remains outside this contract and must go through the existing review/apply/trust-gradient path with human approval gates.

Generated runs, assets, content, coverage, and other artifacts stay untracked unless explicitly fixture-scoped. Public wording is intentionally conservative: no auto-apply, no auto-merge, no self-approval, no reviewer bypass, no production-ready claim, no quality/fun guarantee, and no engine replacement or parity claim.

Issues #1 and #23 remain open; this surface adds planning evidence only and does not modify those issues.
