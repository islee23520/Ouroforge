# Objective Completion Proof v1

Issue: #634 - Objective Completion and Win Loss Proof v1.

Objective Completion Proof v1 records whether a scoped level objective can be
completed from local evidence. It links reachability, behavior, scenario,
verdict, and journal-summary evidence, then recomputes a deterministic proof
result from required actions, required events, expected flags, and expected state
transitions.

This is advisory evidence for agentic level design. It does not execute a game,
does not infer unsupported mechanics, does not prove subjective level quality,
does not write scene files, and does not apply drafts.

## Artifact Shape

The `objective-completion-proof-v1` artifact includes:

- stable proof, objective, intent, plan, placement, scenario, and scene refs;
- reachability, verdict, and behavior evidence refs;
- route steps and required objective actions;
- required events, expected flags, and expected state transitions;
- observed events, observed flags, and observed state transitions;
- a deterministic result with `complete`, `failed`, `missing_evidence`,
  `unsupported`, or `blocked` status;
- expected evidence paths under `evidence/objective-completion/<proof-id>/`;
- proof status, blocked reasons, and guardrails.

Validation recomputes the result from observed local evidence and rejects result
drift. Unsupported mechanics must be explicit on the required action. Missing
events or state transitions remain `missing_evidence`; failed flags are `failed`;
stale and blocked proofs require blocked reasons.

## Read Model

The read model is display data for dashboard, Studio, or journal summaries. It
reports proof status, result status, completion/win/loss booleans, required
action and event counts, missing evidence counts, failed flag counts, linked
evidence refs, blocked reasons, and a journal summary.

The boundary is read-only objective completion and win/loss proof evidence:
local scenario, verdict, reachability, and behavior evidence only, no subjective
quality guarantee, no scene write, no trusted apply, no browser command bridge,
no auto-apply, and no auto-merge.

## Non-Goals

- No autonomous full game generation.
- No runtime execution, physics simulation, or subjective fun evaluation.
- No browser trusted writes, command bridge, local server bridge, auto-apply, or
  auto-merge.
- No production editor, native export, plugin runtime, hosted/cloud behavior, or
  Godot replacement claim.
