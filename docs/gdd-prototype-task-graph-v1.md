# GDD Prototype Task Graph v1

Issue: #654.

`gdd-prototype-task-graph-v1` is an ordered dependency-checked task graph before apply. It represents scaffold, assets, scenes/levels, behavior, scenarios,
run/evidence, review gates, apply steps, and blocked/deferred items as data.

Each task records task id, kind, dependencies, producer/consumer artifacts, file
ownership, expected verification, status, and blocked reasons. Validation rejects
cycles or out-of-order dependencies, missing dependencies, invalid task kinds,
conflicting file ownership, missing producer artifacts, apply steps that do not
depend on review gates, and unsupported task scope.

The task graph is a planning/status artifact. It does not execute hidden commands,
does not mutate source, and does not grant browser trusted writes. Generated task
graphs remain untrusted until Rust/local validation and review-gated apply. GDD,
requirements, mechanics mapping, feasibility, plans, drafts, task graph, review,
apply, run evidence, and journal artifacts remain separate. Browser/dashboard/
Studio surfaces remain read-only or draft-only.

No autonomous unrestricted game creation, auto-apply, auto-merge, native export,
plugin runtime, production-ready claim, generated proprietary asset claim, or
asset generation authority is introduced.

#1 remains open. #23 remains open.
