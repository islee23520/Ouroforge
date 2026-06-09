# Ouroforge Vision

> Extracted from #1 body during roadmap restructuring.

## Final Goal

Ouroforge aims to become an evidence-native, agentic game engine where every game change is produced, tested, judged, and evolved through an Ouroboros loop:

> **Seed → Build → Observe → Verify → Journal → Evolve**

The goal is not to build a conventional game engine first and add AI tooling later. Ouroforge should begin as a verification-centered game-development harness: a system where AI agents can author game behavior, run it in browser targets, inspect runtime state, collect evidence, record reasoning, evaluate acceptance criteria, and propose the next mutation based on reproducible failures.

In short:

> Ouroforge is a Godot-like, agent-first game engine whose core development primitive is not a scene edit, but an evidence-backed iteration loop.


## Core Thesis

Traditional engines are designed around a human clicking through an editor. Ouroforge should be designed around agents operating through structured APIs, observable runtime state, reproducible browser sessions, and append-only evidence trails.

The engine should preserve the approachable mental model of mature open-source engines such as Godot:

- scenes
- nodes/entities
- components
- scripts
- inspectors
- live preview
- easy 2D-first authoring

But internally it should be optimized for agentic development:

- deterministic simulation
- headless execution
- browser-native QA
- openchrome/CDP control instead of Playwright
- structured world-state export
- replayable inputs
- evidence artifacts
- journaled agent reasoning
- evaluator-gated mutation loops
- parallel QA workers


## Non-Goal for the First MVP

Ouroforge should **not** start by competing with Godot, Unity, Unreal, or Bevy as a full engine.

The first milestone is not a polished editor, advanced renderer, asset store, 3D pipeline, or native export system. The first milestone is an Ouroboros-compatible harness kernel that can prove this loop works:

1. A Seed defines the game objective and acceptance criteria.
2. Agents or scripts produce/modify a small browser game.
3. openchrome workers run parallel QA sessions.
4. Runtime probes expose world state, frame metrics, input logs, and screenshots.
5. Evidence is stored as immutable artifacts.
6. A Journal records hypotheses, observations, verdicts, and next mutations.
7. Evaluators decide pass/fail against the Seed.
8. Failed evidence drives the next evolution step.


## Architectural Pillars

### 1. Ouroboros-Compatible Harness Kernel

The harness is the center of the project.

Core concepts:

- **Seed**: The executable specification for a game objective, constraints, scenarios, and acceptance criteria.
- **Ledger**: Append-only event log of every agent, browser, engine, and evaluator action.
- **Evidence**: Immutable artifacts proving what happened during a run.
- **Journal**: Agent reasoning record connecting hypotheses, observations, evidence, verdicts, and proposed mutations.
- **Evaluate**: Mechanical, runtime, visual, and semantic gates that decide whether the Seed has been satisfied.
- **Evolve**: The loop that turns failed evidence into a proposed patch, mutation, or next Seed.

### 2. openchrome-First Browser QA

Playwright should not be the primary abstraction. Ouroforge should use openchrome/CDP as the first-class browser execution and observation surface.

openchrome workers should be able to:

- launch or attach to Chrome sessions
- isolate worker browser contexts/profiles
- open game targets
- inject runtime probes
- capture screenshots
- capture console logs
- capture CDP traces
- collect performance metrics
- send keyboard/mouse/controller input
- execute runtime probe calls
- collect world-state snapshots
- run in parallel as a QA swarm

The browser is not merely a test target. It is an agent-observable execution environment.

### 3. Runtime Probe API

Every browser build should expose a stable agent-facing probe API, for example:

```ts
window.__OUROFORGE__ = {
  getWorldState(),
  getFrameStats(),
  getEvents(),
  step(frames),
  pause(),
  resume(),
  setInput(input),
  snapshot(),
  restore(snapshotId)
}
```

This allows agents to evaluate the game through more than screenshots. They can inspect structured state, compare diffs, replay input, verify flags, and correlate visual evidence with world-state evidence.

### 4. Evidence-Native Storage

Each run should produce a durable evidence directory, such as:

```text
evidence/
  <seed-id>/
    <run-id>/
      screenshots/
      cdp-traces/
      console/
      performance/
      world-state/
      input-replays/
      videos/
      mutations/
      journal.md
      ledger.jsonl
      verdict.json
```

Evidence must be treated as first-class product data, not incidental test output.

### 5. Godot-Like UX Later, Not First

A Godot-like authoring layer is valuable, but it should come after the harness proves itself.

The eventual UX should include:

- scene tree
- entity/component inspector
- live browser preview
- scenario runner
- evidence viewer
- journal viewer
- run comparison
- mutation review
- replay controls

The studio should feel less like a traditional editor and more like an agentic game-development cockpit.


## Guiding Principle

If a game change cannot be tied to a Seed, a Ledger event, Evidence artifacts, a Journal entry, and an Evaluation verdict, then it is not yet an Ouroforge-native change.


## Updated Non-Goal Boundary

Ouroforge does not claim to be a Godot replacement today. Godot-class capability is a long-term measurable roadmap target, not a current maturity claim.

Until the relevant milestones are completed and verified, Ouroforge must not claim:

- production-ready engine status;
- broad Godot replacement status;
- broad engine compatibility guarantees;
- secure sandbox guarantees for arbitrary untrusted content;
- unrestricted source mutation or auto-merge capability;
- native export, plugin marketplace, hosted/cloud, or release automation maturity beyond explicitly completed gates.

The roadmap may target those capabilities, but each capability requires a separate scoped issue sequence, verification evidence, guardrail audit, generated-state audit, and governance handoff before it can be claimed as implemented.



---


## Definition of Done for the First Private MVP

The first private MVP is complete when this command works end-to-end:

```bash
ouroforge run seeds/platformer.yaml --workers 4
```

And produces:

- a run directory
- `ledger.jsonl`
- `journal.md`
- screenshots
- console logs
- performance metrics
- world-state snapshots
- scenario verdicts
- mutation proposal on failure
