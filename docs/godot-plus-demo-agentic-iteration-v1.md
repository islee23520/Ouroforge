# Godot-Plus Demo Agentic Iteration v1

Issue: #789
Status: **GPD12.11 agentic-iteration contract.** This document records the
evidence-driven agentic iteration demonstration for the Godot-Plus Demonstration
Game v1 vertical slice (Signal Gate / Collect and Exit), on the canonical fixture
`examples/playable-demo-v2/collect-and-exit/`. It builds on #780–#788. The legacy
`examples/godot-plus-demo-v1/` tree is superseded and is not used. #1 and #23
remain open.

## The iteration

A controlled, deterministic failure is introduced on an **in-memory** scene copy
(committed source is never mutated), then resolved through a review-gated draft
apply:

1. **Hypothesis / controlled failure** (`agentic-iteration/failure-seed.json`):
   the key pickup trigger omits the `door_open` `setFlag`, so the gate never opens
   and the exit is unreachable.
2. **Failure evidence**: with the seed injected, the player collects the key but
   `door_open=false` and `exit_reached=false` (reproduced deterministically).
3. **Draft mutation proposal** (`agentic-iteration/draft-proposal.json`): an agent
   proposes restoring the `door_open` `setFlag` action. `reviewRequired=true`,
   `applied=false`, `autoApply=false`.
4. **Review decision** (`agentic-iteration/review-decision.json`): an **independent**
   reviewer (`reviewer:human-local`, not the proposal author) accepts. `selfApproval=false`.
5. **Review-gated apply**: the fix is applied through the Safe Source Apply handoff
   **only because** the review is accepted and not a self-approval — to the
   in-memory seed, never to committed source.
6. **Rerun comparison**: after apply, `door_open=true` and `exit_reached=true`; the
   before/after comparison shows improvement (`exit_reached` false → true).
7. **Journal** (`agentic-iteration/journal.json`): links hypothesis → failure
   evidence → draft proposal → review decision → apply → rerun comparison → result.

## Governance properties

- The mutation is **draft/review-gated, not auto-applied**: the smoke applies the
  fix only when `decision == accepted && selfApproval == false && reviewerId != author`.
- **No self-approval**: the reviewer id differs from the proposal author.
- **No committed-source mutation**: the smoke captures SHA-256 of watched source
  files before and after and asserts they are unchanged; all mutation happens on an
  in-memory clone.
- **No auto-apply / auto-merge / reviewer bypass / browser trusted write.**

## Verification

```bash
node examples/playable-demo-v2/collect-and-exit/agentic-iteration-smoke.test.cjs
```

The smoke reproduces the failure, enforces the review gate, applies the fix
in-memory, proves the before/after improvement, validates the journal stages, and
proves no committed source is mutated. It writes only to a temp dir and fails
closed on any committed generated root.

## Boundaries

The iteration reuses existing runtime and review-gated contracts and performs no
auto-apply, auto-merge, self-approval, reviewer bypass, committed-source mutation,
trusted browser write, command execution, network access, committed generated
output, production/native/store export, or full Godot parity / replacement /
production-ready / commercial-release claim. #1 and #23 remain open.
