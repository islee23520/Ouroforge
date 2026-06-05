# GDD Prototype Evidence Bundle v1

Issue: #657

`gdd-prototype-evidence-bundle-v1` records a bounded prototype run result against linked GDD requirements and scenario verdicts. It links the GDD, requirement extraction, feasibility result, prototype bundle, review decision, apply artifact, scenario run outputs, requirement coverage, generated evidence artifacts, journal summary, scoped next mutation proposals, and blocked reasons.

## Validation gates

Rust/local validation rejects unsafe refs, duplicate scenario or requirement ids, scenario verdicts that reference missing requirements, requirement coverage that references missing scenarios, passing bundles with failed/unsupported/missing/skipped evidence, failing bundles without failure evidence, missing-run/unsupported/stale states without visible blockers, malformed evidence refs, unsafe journal wording, and unsafe authority claims.

## Boundaries

This contract supports evidence-gated prototype generation, not autonomous unrestricted game creation. GDD-derived output remains untrusted until Rust/local validation and review-gated apply. Browser, dashboard, and Studio consumers remain read-only or draft-only. Generated run/evidence output remains untracked unless explicitly fixture-scoped. No direct trusted writes, arbitrary source mutation, arbitrary script execution, dynamic code loading, command bridge, browser trusted write, auto-apply, auto-merge, native export, plugin runtime, production-ready claim, commercial readiness claim, current Godot replacement claim, or generated proprietary asset claim is added.

#1 remains open. #23 remains open.
