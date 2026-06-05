# GDD Prototype Evidence Bundle v1

Issue: #657.

`gdd-prototype-evidence-bundle-v1` records the result of evaluating an applied prototype against its GDD requirements. It is an evidence bundle, dashboard/read-model, and journal-summary contract: it does not run commands, mutate trusted files, or apply prototype changes.

The bundle links run id, scenario verdicts, requirement coverage, failed requirements, skipped or unsupported requirements, generated artifact refs, journal summary, and next mutation proposals back to the GDD, extracted requirements, feasibility result, prototype bundle, review decision, apply artifact, scenarios, and run outputs. Missing run, failed run, partial coverage, unsupported requirements, stale bundle evidence, malformed evidence, and blocked states remain visible instead of being hidden.

Dashboard/Studio compatibility is read-only: the read model exposes scenario counts, requirement counts, failed requirements, unsupported requirements, dashboard summary rows, and journal summary rows. The journal summary records GDD satisfaction, failures, observed gaps, and next-step hypotheses.

This issue enables evidence-gated prototype generation, not autonomous unrestricted game creation. GDD-derived output remains untrusted until Rust/local validation and review-gated apply. Browser/dashboard/Studio surfaces remain read-only or draft-only. Generated run/evidence output remains untracked unless fixture-scoped. No hidden command execution, arbitrary source mutation, arbitrary script execution, dynamic code loading, plugin loading, browser trusted writes, command bridge, local server bridge, auto-apply, auto-merge, generated proprietary asset claim, production game, shipped-game, commercial readiness, current Godot replacement, production-ready, native export, hosted/cloud, or plugin runtime is introduced.

#1 remains open. #23 remains open.
