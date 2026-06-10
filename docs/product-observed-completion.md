# Product-observed completion semantics

This is the canonical completion-semantics and closure-template document for the
Product-Observed Rebaseline. It governs future Ouroforge issue, PR, roadmap, and
closure wording until a later governance issue replaces it.

Ownership boundary: this document defines **completion semantics** and the
**closure-comment template**. The artifact-level checklist is owned by the
M115.3 product-observed checklist and must be cross-linked from here once that
issue lands.

Historical classification for earlier milestones is seeded in [`m115-historical-classification-ledger.md`](m115-historical-classification-ledger.md) and should be updated or superseded by later governance evidence rather than used for practical usability overclaims.

## Stable terms

### `contract-complete`

`contract-complete` means the scoped repository contract for an issue or
milestone has landed with reviewable source changes, deterministic fixtures,
schema validation, CLI behavior, docs, and/or regression tests that prove the
stated contract. It does **not** by itself claim practical engine, Studio,
gameplay, export, or agentic-loop usability.

Use `contract-complete` when the evidence is any combination of:

- Rust schema or CLI validation;
- deterministic fixtures, golden files, state hashes, or read-model tests;
- static docs, design gates, governance handoffs, or conservative roadmaps;
- dashboard or Studio read-only rendering of fixture evidence;
- VM-only, synthetic, or static fixture smoke tests.

### `product-observed complete`

`product-observed complete` means the scoped claim has been observed in the
actual product surface that a user would exercise, with committed or linked
evidence sufficient to reproduce the observation and diagnose failures.

A practical runtime, Studio, gameplay, asset-workflow, export, or agentic-loop
claim may use `product-observed complete` only when the issue provides live
browser/editor or local product-surface evidence appropriate to the claim. At a
minimum, future artifact checklists must cover live URL or command entry point,
console/runtime diagnostics, screenshots or equivalent captures, input replay or
interaction transcript, world-state or event samples, frame/performance stats
where relevant, before/after comparison where relevant, verdict, and
generated-state audit.

## Closure classifications

Every issue closure comment in Eras S-W must include exactly one classification
line:

```text
Closure classification: contract-complete
```

or:

```text
Closure classification: product-observed complete
```

If a milestone mixes both evidence types, classify the closure by the strongest
claim being made. A docs/schema issue that enables later dogfood evidence should
normally close as `contract-complete`. A runtime, Studio, gameplay, export, or
agentic-loop issue must not close as `product-observed complete` unless its own
verification evidence includes the product-surface observations required by the
artifact checklist.

## Allowed examples

Allowed `contract-complete` wording:

> M115.1 is contract-complete: the canonical semantics and closure template are
> documented, linked from the roadmap, and wording guards were run. This does
> not claim practical runtime or Studio usability.

Allowed `product-observed complete` wording:

> M118.3 is product-observed complete: the runtime shell was exercised at the
> linked live URL, console diagnostics were clean, the attached replay reached
> the expected world state, frame stats stayed inside the scoped budget, and the
> generated-state audit is clean.

Allowed historical wording:

> Earlier milestones remain preserved as contract-complete unless their linked
> evidence already satisfies the current product-observed checklist.

## Disallowed overclaims

Do not use practical-engine wording unless product-observed evidence is attached.
The following claims are forbidden unless they are explicit non-goals,
negations, audit examples, or quoted historical text being narrowed:

- production-ready;
- commercial-release-ready;
- ship-ready;
- compatibility-stable public engine API;
- secure sandbox;
- Godot replacement or Godot parity;
- native-export ready;
- plugin-runtime ready;
- source-apply ready;
- autonomous repair;
- public release automation;
- unqualified `complete` for runtime, Studio, gameplay, asset, export, or
  agentic-loop usability.

Prefer one of these precise replacements:

- `contract-complete`;
- `product-observed complete`;
- `complete on merged contract evidence`;
- `fixture-backed contract evidence`;
- `read-only product-surface inspection evidence`;
- `product-observed FAIL with recorded gaps`.

## Domain-specific evidence requirements

Runtime claims require live runtime entry-point evidence, console/runtime
logs, input replay or scripted interaction, world-state/event samples, frame or
performance stats where relevant, and generated-state audit.

Studio claims require live editor/workspace evidence, screenshots or equivalent
captures, diagnostics, interaction transcript, before/after or draft/read-model
comparison where relevant, and proof that trusted writes still route through the
Rust/local gates.

Gameplay claims require a playable scenario or dogfood slice, input replay,
world-state/event samples, player-visible outcome evidence, and an explicit
verdict separating mechanical pass/fail from fun, taste, or release judgment.

Asset-workflow claims require source asset refs, load/preview evidence,
missing/invalid asset diagnostics, before/after comparison where relevant, and
proof generated or downloaded assets did not pollute trusted source unless a
fixture explicitly authorizes them.

Export/package claims require local package or bundle evidence, provenance,
checksums where relevant, smoke execution, and explicit non-claims for store
submission, signing, upload, release button, or public launch automation.

Agentic-loop claims require proposal rationale, bounded mutation scope,
review/gate evidence, before/after rerun comparison, failure/backlog handling,
and proof that no hidden trusted write, self-approval, auto-merge, or command
bridge was introduced.

## Runnable wording guard

Run this documented guard before closing any Product-Observed Rebaseline issue:

```bash
grep -RInE \
  "production-ready|production ready|commercial-release-ready|commercial release ready|ship-ready|ship ready|compatibility-stable|stable public engine API|secure sandbox|sandbox guarantee|Godot replacement|Godot parity|native-export ready|native export ready|plugin-runtime ready|plugin runtime ready|source-apply ready|source apply ready|autonomous repair|public release automation|go-live automation|unqualified complete" \
  README.md CONTRIBUTING.md SECURITY.md docs .github examples || true
```

Interpretation rule: every match must be an explicit non-goal, negation,
conservative boundary, audit example, or historical claim being narrowed by this
document. If a match is a practical capability claim without product-observed
evidence, the issue must not close green; record the gap/backlog instead.

For newly edited files, also run a stricter check by replacing the path list
above with only the files touched by the PR and manually confirming every
`complete` term is qualified as `contract-complete`, `product-observed complete`,
`complete on merged evidence`, or another conservative bounded phrase.

## Closure-comment template

```markdown
Closure classification: contract-complete | product-observed complete

Scope landed:
- <issue/PR unit and exact bounded scope>

Evidence:
- <commands/tests/docs/live URL/screenshots/replay/artifacts as applicable>

Product-observed checklist result:
- <PASS/FAIL/N/A with link to per-item checklist or explanation>

Gap/backlog handling:
- <none, or explicit issue/backlog refs; do not hide failed usability evidence>

Generated-state audit:
- <git status / ignored-output summary and any generated artifacts kept out of source>

Governance anchors:
- #1 state: OPEN verified at <timestamp or command evidence>
- #23 state: OPEN verified at <timestamp or command evidence>

Non-goals preserved:
- No production-ready, commercial-release-ready, Godot replacement/parity,
  native/mobile/console/store export, public release automation, executable
  plugin runtime, marketplace, arbitrary scripting expansion, browser trusted
  write, command bridge, hidden command execution, self-approval, auto-apply, or
  auto-merge claim was introduced unless explicitly scoped by a later governance
  issue.
```
