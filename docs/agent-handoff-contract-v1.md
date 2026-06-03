# Agent Handoff Contract v1

Agent Handoff Contract v1 is a generated advisory artifact for continuing an authoring loop safely. It summarizes the current loop state, next safe action, blockers, required decisions, inert allowed command text, forbidden actions, evidence refs, drift guardrails, generated-state policy, and an explicit boundary.

Generated handoff files live under local generated state, commonly:

```text
runs/agent-handoffs/<loop-id>/handoff.json
```

The schema is `agent-handoff-contract-v1`.

## Generate a handoff

```bash
cargo run -p ouroforge-cli -- loop handoff <loop-plan.json> --output runs/agent-handoffs/<loop-id>/handoff.json
```

Generation reads the authoring loop plan, status/recovery state, and evidence bundle read-model information. It writes the handoff JSON and records a ledger summary under `runs/authoring-loop-ledgers/<loop-id>/ledger.jsonl`.

## Boundary

A handoff is advisory evidence only. It does not execute commands, grant authority beyond documented loop state, apply mutations, merge changes, start a scheduler, add hosted services, or create browser command controls. `allowedCommands` are inert display text for a human or agent to inspect and run explicitly outside the browser if authorized.

## Dashboard and cockpit display

Dashboard export includes generated handoffs as `agent_handoffs`. The evidence dashboard and authoring cockpit render handoff state as escaped read-only Handoff Studio evidence: next safe action, blockers, required decisions, allowed command text, forbidden actions, evidence refs, guardrails, and boundary. Browser surfaces do not write handoff data, execute commands, apply mutations, repair references, or merge changes.

## Generated state policy

Handoffs are local generated state. Keep `runs/`, dashboard exports, and generated handoff files untracked unless a later issue explicitly changes the policy.
