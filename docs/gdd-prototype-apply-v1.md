# GDD Prototype Apply v1

Issue: #656.

`gdd-prototype-apply-v1` is an inert Rust/local validation contract for review-gated prototype apply. It converts accepted prototype draft bundles into scoped project scaffold, scene/level, behavior, and scenario transactions only after accepted review, target hash validation, source/license notes, transaction-output safety, rollback metadata, rerun command context, and generated-state audit are present.

The contract records GDD id, bundle id, review decision id, transaction ids, before/after hashes, rollback metadata, rerun command context, generated-state audit refs, asset/source refs, scenario refs, behavior refs, and blocked reasons. It rejects missing review, rejected review, self-approval, auto-apply, stale targets without blockers, source-like fixture targets, unsafe refs, generated-output collisions, missing asset/source notes, missing scenario/behavior validity refs, rollback mismatch, and hidden command execution.

This document and fixtures do not perform writes. Rust/local validation owns trusted persistence. Browser/dashboard/Studio surfaces remain read-only or draft-only unless an explicitly scoped Rust/local trusted API owns persistence. GDD, extracted requirements, mechanics mapping, feasibility, plans, drafts, task graph, review, apply, run evidence, and journal artifacts remain clearly separated. GDD-derived output remains untrusted until Rust/local validation and review-gated apply.

No autonomous unrestricted game creation, arbitrary source mutation, arbitrary script execution, dynamic code loading, plugin loading, browser trusted writes, command bridge, local server bridge, auto-apply, auto-merge, self-approval, generated proprietary asset claim, production game, shipped-game, commercial readiness, current Godot replacement, production-ready, native export, hosted/cloud, or plugin runtime is introduced.

#1 remains open. #23 remains open.
