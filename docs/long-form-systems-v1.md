# Long-Form Game Systems v1 Scope and Contract

Status: **Scope/contract — gated rungs and boundaries only; no executable behavior**

Issue: #1656 — Long-Form Game Systems v1 Scope and Contract
Anchor: #1 Era G Milestone 39 (Long-Form Game Systems)

This document is the canonical Long-Form Game Systems v1 design artifact. It is
a scope/contract document, not an implementation milestone. It defines the
systems a long-form game needs — meta-progression/unlocks, economy/currency,
save/profile and run-history at scale, UI/UX flow with onboarding and
accessibility, and an optional narrative/dialogue/event system — and binds each
to the Game Complexity Ladder v1 (Milestone 24) as a gated rung that is claimed
only after a loop-produced, evidence-backed demo proves it. It also specifies
the Rust-trusted-state vs JS-runtime-UI boundary for these systems and records
the follow-up dependency order. It adds no executable behavior, fixtures,
runtime features, Studio surfaces, browser authority, or engine capability.

This contract follows the project idiom for scope/contract work: the ADR #92
Distributed/Elixir gate, the Native Export Design Gate (#168), the Game
Complexity Ladder v1 gate (#1493, `docs/game-complexity-ladder-v1.md`), the
Trust Gradient v1 gate (#1476, `docs/trust-gradient-design.md`), and the Era F
game-class scope/contract documents (Grid-Puzzle #1573, Deck-Roguelike #1599,
Synthetic Player Balance #1605). No long-form-system behavior is implemented in
#1656; it only decides and bounds. Implementation is authorized only through the
gated follow-up issues listed below.

## Scope

This contract covers five system contracts, one boundary record, and one
governance record:

1. the meta-progression and unlocks system, defined as a Milestone 24 rung;
2. the economy and currency system, defined as a Milestone 24 rung;
3. the save/profile and run-history-at-scale system, defined as a Milestone 24
   rung;
4. the UI/UX flow, onboarding, and accessibility system, defined as a
   Milestone 24 rung;
5. the optional narrative/dialogue/event system, defined as a Milestone 24 rung;
6. the Rust-trusted-state vs JS-runtime-UI boundary shared by all five systems;
7. the dependency order and closure gates for the follow-up issues, preserving
   #1 and #23 as open anchors.

The contract applies to long-form-system claims, the run/profile persistence
path, in-game UI/flow wording, roadmap wording, scenario-coverage numbering, and
any future engine growth requested to satisfy these rungs. A documented contract
here does not authorize implementation by itself; each follow-up issue must
still scope concrete Rust/local changes and verification.

## Non-goals

Long-Form Game Systems v1 does not authorize:

- a new engine, runtime, writer, save service, or any parallel system that
  duplicates the existing runtime, probe, evaluator, evolve/campaign, compare,
  provenance, asset-manifest, dashboard, cockpit, source-apply, trust-gradient,
  QA-swarm, or CLI surfaces;
- direct trusted writes from generation, role agents, the producer, or from any
  browser/Studio surface; proposals flow only through the existing
  review/apply/trust-gradient path;
- autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden
  trusted writes; high-risk and source-affecting changes are never auto-applied;
- promotion of any unlicensed, uncredited, or unverified-style generated
  asset/audio/content; license/provenance and the function-specific QA gate are
  mandatory before promotion;
- any automated quality/fun/taste claim; "looks good / sounds good / is fun" and
  art/audio/UX/narrative direction remain human decisions;
- a browser command bridge, arbitrary shell execution, dependency install,
  CI/workflow mutation, credentialed operation, network install/update, or
  publish/deploy/sign/upload;
- shipping (native/store export), hosted/cloud capability, real-player
  telemetry, or live-ops absent an explicit Layer-3 GO (DEFER per Milestone 26 /
  #1508); distributed/Elixir remains NO-GO per ADR #92
  (`docs/distributed-elixir-design.md`);
- renderer, 3D, physics, audio, or animation breadth beyond what a specific
  long-form-system rung's gate justifies;
- treating in-game UI/HUD/menus as a Studio trusted-write surface; in-game UI is
  JS runtime, read-only with respect to trusted state;
- committing generated runs/profiles/saves/content/release artifacts unless
  explicitly fixture-scoped;
- closing, modifying, replacing, or narrowing #1 or #23 without a separate
  explicit governance decision.

## The systems as Milestone 24 rungs

Each long-form system below is a Milestone 24 ladder rung. A rung is claimed
only after a **loop-produced**, evidence-backed demo proves the system, with
four-gate evidence and a loop-coverage verdict; a hand-authored screenshot or
prose summary is not sufficient. Each system's trusted state is owned by
Rust/local; its in-game presentation is the deterministic JS runtime and is
read-only with respect to trusted state. None of the systems introduce a
parallel runtime or save service.

### 1. Meta-progression and unlocks (#1657)

Trusted, deterministic state describing what a profile has unlocked and how
progression advances across runs (for example unlock flags, progression
counters, and the deterministic rule that maps run outcomes to progression
deltas). The rung's machine-checkable acceptance shape is: applying a recorded
run outcome to a prior progression state yields a deterministic next state, and
replaying the same outcomes from the same start reproduces the same unlocks
exactly. State and the apply rule are Rust/local trusted logic; the in-game
unlock UI is read-only JS runtime presentation of exposed state. Generated
progression proposals flow through review/apply/trust-gradient; they are never a
direct trusted write.

### 2. Economy and currency (#1658)

Trusted, deterministic state describing currencies, balances, and the
deterministic transactions that change them (earn/spend/convert), with
fail-closed validation that rejects malformed or out-of-bounds transactions
(for example a spend that would drive a balance negative). The rung's
acceptance shape is: applying a validated transaction sequence to an initial
ledger yields a deterministic final ledger, conservation/invariants hold, and an
invalid transaction is rejected with a structured reason rather than silently
clamped. The ledger and transaction validation are Rust/local trusted logic; the
in-game economy UI is read-only JS runtime presentation. No real-money, store,
or paid capability is in scope (Layer-3 DEFER).

### 3. Save/profile and run-history at scale (#1659)

Trusted, deterministic persistence for a player profile, its save state, and a
bounded run-history, reusing the existing run/project-binding and persistence
surfaces rather than a parallel save service. The rung's acceptance shape is:
saving then loading a profile round-trips deterministically, run-history append
is bounded and ordered, and a malformed/partial/incompatible save is reported
explicitly and fails closed rather than loading corrupt state. Persistence,
schema/versioning, and validation are Rust/local trusted logic; the browser
reads exposed save/profile state read-only and never performs trusted writes.

### 4. UI/UX flow, onboarding, and accessibility (#1660)

The in-game UI/UX flow (menus, onboarding sequence, HUD, and accessibility
affordances) expressed as deterministic JS runtime UI driven by exposed trusted
state, with a machine-checkable flow contract (for example: the declared flow
states and transitions are reachable and deterministic, and declared
accessibility affordances are present). The rung's acceptance shape is a
deterministic, probe-observable flow whose state transitions replay identically;
accessibility presence is checked structurally, not as a taste judgement. The
flow contract and any trusted gating are Rust/local; the UI itself is JS runtime
and read-only with respect to trusted state. No automated UX-quality or
taste claim is made; UX direction remains a human decision.

### 5. Optional narrative/dialogue/event system (#1661)

An optional, data-first narrative/dialogue/event system: deterministic
narrative state and a declared event/dialogue graph evaluated by reusing the
existing event/signal and state-machine artifact surfaces, not a new scripting
engine or executable content. The rung's acceptance shape is: a declared
dialogue/event graph advances deterministically given recorded inputs/flags,
references only declared nodes/flags, and fails closed on a malformed or
dangling reference. Narrative state and validation are Rust/local trusted logic;
dialogue/event presentation is read-only JS runtime. Generated narrative content
is proposal-only through review/apply/trust-gradient and, where it includes
assets, requires license/provenance and the function QA gate before promotion;
no narrative tone/quality is automated.

## Rust-trusted-state vs JS-runtime-UI boundary

All five systems share one boundary, consistent with the project Language
Boundary:

- **Rust/local owns trusted state and trusted logic.** Progression state and
  apply rules, the economy ledger and transaction validation, save/profile
  persistence and schema/versioning, the UI/UX flow contract and any trusted
  gating, and narrative state and graph validation are Rust/local. Rust/local
  also owns evidence and provenance writing, run/project binding, the
  review/apply/trust-gradient path, asset-QA/license/provenance checks, and CLI
  behavior. Trusted state changes only through validated, fail-closed Rust/local
  logic.

- **JavaScript owns the deterministic runtime and read-only in-game UI.** The
  in-game UI/HUD/menus, onboarding flow presentation, unlock/economy/save
  displays, and dialogue/event presentation are the deterministic JS runtime.
  They render exposed trusted state through the existing read-only
  `window.__OUROFORGE__` probe and never perform trusted writes. The probe
  carries the existing `read-only` browser/Studio mode and its
  `disallowedActions` (no trusted writes, no command bridge, no live mutation).
  In-game UI is a JS runtime surface, never a Studio trusted-write surface.

- **Trusted reads the exposed state; the browser never writes back.** Trusted
  verification (gates, coverage, provenance) reads the exposed deterministic
  state. Any change a player/UI would "make" is a deterministic input to the
  runtime whose trusted consequence is computed and validated by Rust/local
  logic; the browser is never the source of trust.

- **Generation is proposal-only.** Generated progression/economy/narrative
  content or assets are proposals through the existing review/apply/trust-gradient
  path; they are never direct trusted writes or auto-applied, and generated
  assets require license/provenance plus the function-specific QA gate before
  promotion.

This boundary introduces no new dependency, language, or runtime: trusted logic
reuses Rust/local crates and the existing review/apply/trust-gradient and
persistence surfaces, and the in-game UI reuses the existing JavaScript runtime
and probe.

## Milestone 24 complexity-ladder linkage

Each long-form system records as a rung claim under the Game Complexity Ladder
v1 contract (`docs/game-complexity-ladder-v1.md`). The ladder's rule governs
each system: a rung is claimed only after a **loop-produced**, evidence-backed
demo proves it, with four-gate evidence and a loop-coverage verdict.

Concretely, each rung's claim package (delivered by that system's issue and
ultimately exercised by the Long-Form Game Systems Demo, #1662, not by this
contract) must include:

- a loop-produced demo created through the local authoring loop that exercises
  the system;
- four-gate evidence (required validation; runtime/scenario behavior;
  review/governance; generated-state/trust-boundary) showing the demo passed,
  aggregated through the existing evaluator (`declared-gate-and`) and `compare`
  operators;
- a loop-coverage verdict from the Milestone 20 loop-coverage surface showing
  the relevant loop path is covered, not merely hand-inspected;
- reproducible local commands and artifact paths for a reviewer;
- explicit missing/partial/malformed evidence reporting where a gate cannot be
  satisfied.

Engine-growth demand rule (inherited from the ladder): any renderer, runtime, or
other engine capability requested while building one of these rungs must cite
the specific system gate it is needed to satisfy and explain why the current
implemented surface cannot produce the required loop-backed evidence without it.
These rungs do not authorize 3D, physics, audio, or renderer breadth.

## Dependency and closure order

Milestone 39 follow-up work should remain in this order unless a later issue
documents a concrete blocker and replacement ordering:

1. #1656 — Long-Form Game Systems v1 Scope and Contract (this issue).
2. #1657 — Meta-Progression and Unlocks v1, after this contract is merged.
3. #1658 — Economy and Currency v1, after #1657 satisfies its closure gates.
4. #1659 — Save/Profile and Run-History at Scale v1, after #1658 satisfies its
   closure gates.
5. #1660 — UI/UX Flow, Onboarding and Accessibility v1, after #1659 satisfies
   its closure gates.
6. #1661 — Narrative/Dialogue/Event System v1, after #1660 satisfies its closure
   gates.
7. #1662 — Long-Form Game Systems Demo v1, after #1661 satisfies its closure
   gates.
8. #1663 — Scenario Coverage v37: Long-Form Game Systems Regression Suite, after
   #1662 satisfies its closure gates.
9. #1664 — Roadmap and #1 Governance Refresh after Long-Form Game Systems v1,
   after #1663 satisfies its closure gates. This ordering records the governed
   follow-up chain only; it does not authorize implementing, modifying, closing,
   or claiming #1664 from this contract alone.

```text
#1656 scope -> #1657 -> #1658 -> #1659 -> #1660 -> #1661 -> #1662 -> #1663 -> #1664
```

Closure gates for each follow-up issue:

- the prior issue in the dependency order is merged or explicitly documented as
  superseded by a maintainer-approved governance decision;
- the issue scopes one bounded prerequisite for a rung (one system, the demo,
  coverage, or governance) and does not combine independently verifiable
  behavior into one PR;
- any engine growth cites the specific rung gate and evidence gap;
- four-gate evidence and a loop-coverage verdict are present for any rung claim;
- generated-state and trust-boundary audits pass (run outputs, profiles, saves,
  dashboard exports, local tool state, and build outputs remain untracked unless
  explicitly fixture-scoped);
- public wording stays conservative (no auto-merge/quality/fun/production/Godot
  -replacement claim);
- Scenario Coverage numbering continues from v33 (Era F) onward; the regression
  suite for this milestone is v37;
- #1 and #23 are checked and remain open anchors unless a separate explicit
  governance decision exists.

## Governance audit for #1 and #23

#1 remains the broad vision and roadmap anchor for Era G. #23 remains the
repo-memory and design-context anchor. This contract preserves both as open
anchors and does not modify, close, replace, or narrow either issue. Long-form
game systems are a layer on the existing loop — generation is the front door and
the deterministic verification loop is the engine room; they are layers, not
alternatives. Each system is a specialized capability with its own verification
gate.

Future work that proposes to change either anchor must be a separate explicit
governance decision. That decision must identify the replacement source of
truth, record maintainer approval, and avoid implying that this contract itself
authorized the change.
