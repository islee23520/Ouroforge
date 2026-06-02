# Plugin System Design Gate

Status: **EE13.1 risk and boundary audit draft; ADR decision pending in EE13.2**

Issue: #169 — Engine Expansion v1 Plugin System Design Gate

This document is the canonical plugin-system design artifact. EE13.1 audits
risk, boundaries, and non-plugin alternatives only. It does not approve a plugin
system, create implementation issues, add dynamic loading, add a plugin manager
UI, add a marketplace, add a script sandbox, or permit arbitrary source
mutation.

## Current evidence-native authority model

Ouroforge currently protects evidence integrity by keeping core artifact
semantics inside Rust-owned contracts:

- Seeds define goals, constraints, scenarios, and targets.
- Runs are created by the CLI under `runs/` with ledger/evidence artifacts.
- Scenario execution captures bounded browser/runtime evidence.
- Evaluator verdicts, journals, comparisons, and mutation proposals remain
  machine-readable artifacts.
- Browser UIs are read-only or in-memory preview surfaces; they do not write
  trusted persisted project state.
- Mutation behavior is explicit, evidence-linked, and reviewed through CLI-owned
  artifact flows rather than arbitrary code execution.

A plugin system would pressure these boundaries. The default posture must remain
that Rust artifact contracts are authoritative and browser/runtime/plugin code
cannot bypass validation.

## Concrete unmet needs that might motivate plugins

Potential future needs include:

- adding new dashboard panels for already-validated artifact categories;
- adding static authoring widgets for existing scene schema fields;
- adding specialized scenario templates or Seed presets;
- adding deterministic lint/check commands over generated artifacts;
- experimenting with optional visualization surfaces for local evidence.

Current Engine Expansion v1 evidence does not prove that these needs require a
plugin runtime. Most can be served by issue-specific built-in features, static
configuration, or docs until repeated demand appears.

## Risk analysis

### Artifact integrity risk

Plugins could create, rewrite, or hide `run.json`, evidence indexes, verdicts,
journals, comparisons, or mutation artifacts. That would weaken the audit trail
unless every plugin output is typed, versioned, validated, and evidence-linked by
Rust.

### Evidence provenance risk

A plugin that emits evidence without deterministic provenance could make a run
look verified when it is not. Evidence must remain traceable to a scenario,
worker, command, or explicit review action.

### Permission and trust risk

Dynamic loading or script execution would require permissions for file access,
network access, source mutation, UI actions, and command execution. The current
repo has no permission model, sandbox, registry trust model, signing policy, or
review pipeline.

### Mutation safety risk

Plugins could blur the line between deterministic mutation proposals and
arbitrary source edits. Current mutation flows are artifact-based and reviewed;
plugins must not gain authority to modify source, apply patches, or accept
mutations without Rust validation and explicit review.

### Schema drift risk

Plugin-defined schema extensions could fragment Seed, scenario, scene, and
evidence semantics. Core artifacts must remain understandable without a plugin
installed.

### Marketplace and compatibility risk

A plugin ecosystem implies compatibility, packaging, support, versioning, and
trust expectations. Ouroforge is not ready to make third-party compatibility,
marketplace, production, or public-launch claims.

## Boundary model if plugins are ever considered

Any future plugin proposal must start from an allowlist, not broad execution.
Potentially safe ownership areas:

- read-only dashboard renderers for validated artifact categories;
- static docs/templates for Seed or scenario examples;
- deterministic checks that output Rust-validated advisory artifacts;
- local-only visualization of existing evidence.

Forbidden ownership areas unless a later issue changes the charter explicitly:

- Seed, Run, Ledger, Evidence, Verdict, Journal, Compare, Mutation, or Scene
  schema authority;
- dynamic loading of arbitrary code;
- browser-side writes to trusted persisted state;
- direct source mutation or patch application;
- mutation acceptance/rejection without CLI/Rust review flow;
- server, database, cloud, auth, telemetry, marketplace, registry, package
  manager, updater, or remote plugin discovery;
- public compatibility or production-readiness claims.

## Permission and validation requirements if adopted

A future GO would require, before implementation:

1. a typed plugin manifest schema validated by Rust;
2. explicit permissions for every artifact category and side effect;
3. deterministic output paths under generated artifact directories;
4. evidence provenance linking plugin output to command/run/scenario/review
   context;
5. a no-network/no-cloud default;
6. no direct source writes without a separately reviewed mutation mechanism;
7. reproducible verification that works without trusting browser UI code;
8. a compatibility policy that does not imply marketplace support.

## Alternatives considered for the audit

| Alternative | Fit with evidence-native model | Main benefit | Main risk |
| --- | --- | --- | --- |
| Issue-specific built-in features | Strong | Keeps Rust validation and tests close to behavior | More core code when needs repeat |
| Static configuration | Strong | Handles presets/templates without executable extension code | Limited flexibility |
| Documentation/examples | Strong | Lowest risk for emerging patterns | Does not automate repeated work |
| Read-only dashboard extension points | Medium | Could localize presentation-only additions | Needs manifest and artifact boundaries |
| CLI advisory checks | Medium | Can output validated artifacts without UI trust | Can drift into command/plugin framework |
| Full plugin runtime | Weak currently | Maximum extensibility | Dynamic loading, permissions, sandbox, compatibility, and trust burden |
| Marketplace/registry | Poor currently | Distribution/discovery | Implies support, trust, auth/cloud, and compatibility claims |
| Defer plugin system | Strong | Preserves current artifact authority while demand matures | Requires explicit revisit criteria |

## Feasibility findings

1. Current #167/#168 evidence proves inspectability and design-gated native export,
   not a need for third-party extension execution.
2. Most plausible plugin needs are presentation, templates, or advisory checks;
   none currently require dynamic loading.
3. The highest-risk domains are artifact authority and mutation authority.
4. Any plugin-like capability must be comprehensible and auditable without the
   plugin installed.
5. A full plugin runtime or marketplace would be premature before an allowlisted
   extension contract exists.

## No-code / no-scaffold audit

EE13.1 intentionally changes documentation only. It adds no:

- plugin runtime;
- dynamic loading;
- plugin manager UI;
- marketplace, registry, package manager, remote discovery, or compatibility
  promise;
- script sandbox;
- source mutation path;
- permission bypass of Rust validation;
- server, database, cloud, auth, or telemetry;
- generated `runs/` or dashboard artifacts.

## ADR inputs for EE13.2

EE13.2 should make an explicit GO/NO-GO decision using this audit. The decision
should answer:

- Is a plugin system justified now by evidence-backed unmet needs?
- Which alternative is selected or rejected?
- What artifact and mutation domains remain Rust-owned?
- If GO, what exact follow-up issues are required before any implementation?
- If NO-GO, what concrete revisit criteria would change the decision?

Until EE13.2 is merged, this document records risk/boundary findings only and
does not authorize implementation.
