# Localization Pipeline v1 Scope and Contract

Status: **Design gate — scope and contracts only; no executable behavior**

Issue: #1832 — Localization Pipeline v1 Scope and Contract
Anchor: #1 Era I Milestone 53 (Localization for global Steam)

This document is the canonical Localization Pipeline v1 design artifact. It
defines the multi-language support contract for string externalization,
mechanical multi-language generation, validation for completeness and placeholder
integrity, dependency order, and closure gates. It adds no executable behavior,
localization engine, fixtures, browser authority, runtime feature, or trusted
write path.

Localization v1 is additive and mechanical. It may verify that user-facing text
is externalized, translations are complete, placeholders are preserved, and
localized bundles are traceable. It does not automate creative tone, cultural
judgment, release approval, market demand, or whether a title is good/fun/
shippable.

## Existing surfaces this milestone reuses

| Concern | Reused surface |
| --- | --- |
| Generation proposals | Existing proposal-only generation, provenance, review/apply, and trust-gradient surfaces. |
| Runtime/UI display | Existing JS runtime, deckbuilder UI/in-game UI, `window.__OUROFORGE__` probe, dashboard, and cockpit read-only display surfaces. |
| Trusted validation | Existing Rust/local validation, persistence, evidence writing, compare, evaluator, and CLI patterns. |
| Provenance | Existing provenance-bundle and generated-evidence conventions for source, run, and artifact lineage. |
| Generated state | Existing ignored generated roots for runs, localization reports, dashboard exports, and temporary artifacts. |

No follow-up may introduce a parallel engine, trusted browser write path, remote
translation service dependency, or new language/runtime without explicit issue
scope and governance approval.

## Scope

The contract applies to:

- externalization of user-facing strings into deterministic string keys and
  catalogs;
- detection and prevention of hard-coded user-facing text in scoped runtime/UI
  surfaces;
- multi-language generation as proposal-only, provenance-carrying output;
- validation for completeness, placeholder integrity, schema/version drift,
  stale catalogs, and unsafe strings;
- dependency order and closure gates for the follow-up issues.

The contract governs claims and boundaries. Follow-up issues own concrete
implementation and tests.

## Non-goals

Localization Pipeline v1 does not authorize:

- executable behavior in this scope issue;
- a new localization engine, runtime, UI framework, remote service, or command
  bridge;
- direct trusted writes from generation or any browser/Studio surface;
- autonomous apply, auto-merge, self-approval, reviewer bypass, hidden trusted
  writes, or browser-side promotion of generated translations;
- creative/tone automation, cultural suitability scoring, or automated quality
  judgment;
- production-ready, Godot replacement/parity, market-success, public-launch, or
  Steam-release claims;
- hosted/cloud/mobile Layer-3 capability, Steam account/signing/release actions,
  or market-demand work;
- committed generated translations, reports, dashboard exports, or local
  artifacts unless explicitly fixture-scoped by a follow-up issue;
- closing, modifying, replacing, or narrowing #1 or #23.

## String externalization contract

User-facing text must be represented by stable string keys and validated catalogs
instead of hard-coded display text in scoped surfaces.

- Scoped runtime/UI/user-visible text uses deterministic string ids. Code may
  carry internal diagnostics or developer-only identifiers, but player-facing
  strings must resolve through catalogs where the follow-up issue scopes them.
- String ids must be stable, namespaced, and deterministic. Renames require an
  explicit migration or stale-key diagnostic so saved/generated references do not
  silently drift.
- Catalog entries include source text, target language entries where available,
  placeholder metadata, provenance, and validation status. Missing entries are
  visible and fail mechanical completeness checks when the language is declared
  required.
- Hard-coded user-facing text detection is mechanical and scoped. It must report
  evidence with file/surface, string candidate, classification, and pass/fail or
  allowed-exception status.
- Browser/Studio surfaces may display localized text and validation evidence
  read-only. They must not edit trusted catalogs, apply generated translations,
  or bypass review.

## Multi-language generation contract

Generated translations are proposals, never trusted writes.

- A generated localization candidate must carry source language, target language,
  string ids, source catalog digest, generation run id, provenance, and declared
  license/usage boundary where applicable.
- Generation output routes through the existing review/apply/trust-gradient path.
  It may be inspected, compared, and rejected, but it cannot directly mutate
  trusted catalogs.
- Generation is mechanical support, not creative/tone authority. Human review
  owns wording, cultural fit, and release go/no-go.
- Candidates with missing provenance, stale source catalog digest, unknown string
  ids, unsupported language tags, unsafe markup, or placeholder drift fail closed.
- Local/demo evidence remains generated state unless a follow-up issue explicitly
  scopes tiny deterministic fixture catalogs.

## Localization validation contract

Validation is additive and fails closed for mechanical drift.

- **Completeness** — every required language has an entry for every required
  string id, or the report records the exact missing language/key pairs.
- **Placeholder integrity** — placeholders in localized text match the source
  contract by name, count, and supported formatting class. Missing, extra,
  reordered-when-disallowed, or type-drift placeholders fail.
- **Catalog integrity** — schema version, language tags, key namespaces, duplicate
  keys, stale source digests, unsafe paths, and unknown fields are validated
  before a catalog can be accepted.
- **Runtime/UI integrity** — scoped UI/runtime surfaces resolve localized strings
  through catalogs and expose read-only evidence for missing keys/fallbacks.
- **Escaping and safety** — localized strings displayed in browser/dashboard/
  cockpit surfaces are escaped/read-only and cannot introduce command execution,
  trusted writes, network fetches, or HTML/script injection authority.

Validation reports are generated evidence with deterministic ordering and bounded
diagnostics. They do not claim translation quality or tone.

## Generated state policy

Generated translation candidates, localization reports, catalog diffs, dashboard
exports, cockpit exports, temporary localized bundles, and run evidence are
generated/local state. They remain untracked unless a follow-up issue explicitly
scopes tiny deterministic fixtures as source-like regression data.

## Rust-trusted / browser-read-only boundary

Rust/local owns trusted validation, catalog persistence, evidence writing,
provenance, run/project binding, source apply, and the review/apply/trust-gradient
path. TypeScript/JavaScript owns deterministic runtime/UI display, the
`window.__OUROFORGE__` probe, and browser-local read-only inspection. Browser and
Studio surfaces may display localized text, fallback state, draft proposals, and
validation evidence only as escaped/read-only data; they must not apply changes,
write trusted files, execute commands, promote generated translations, fetch
remote translation services, or bypass review.

## Dependency order and closure gates

Follow-up Localization Pipeline v1 issues are implemented in this order:

1. **Scope and Contract** (this issue, #1832) — define the contracts,
   boundaries, reuse statement, and dependency order; no executable behavior.
2. **String Externalization and Multi-Language Generation v1** (#1833) — implement
   scoped externalization, proposal-only generation, and mechanical validation for
   required languages.
3. **Localization Demo v1** (#1834) — demonstrate externalization, generation,
   completeness, and placeholder validation with local generated evidence and
   conservative claims.
4. **Scenario Coverage v48: Localization Regression Suite** (#1835) — add
   regression coverage for catalogs, missing strings, placeholder drift, unsafe
   display, stale generated proposals, and backward compatibility.
5. **Roadmap and #1 Governance Refresh after Localization v1** (#1836) — record
   the milestone outcome while keeping #1 and #23 open.

```text
#1832 scope -> #1833 -> #1834 -> #1835 -> #1836
```

Closure gates for each follow-up issue:

- the prior issue in the dependency order is merged or explicitly superseded by
  a maintainer-approved governance decision;
- independently verifiable behavior is not combined across externalization,
  generation/validation, demo, coverage, and governance steps;
- the implementation reuses existing generation/provenance/review/apply/trust-
  gradient/runtime/probe/dashboard/cockpit/CLI surfaces and does not add a
  parallel engine;
- localization is additive and mechanically validated; creative/tone judgment
  remains human;
- generated-state and trust-boundary audits pass;
- public wording stays conservative: no auto-merge, quality, fun, production,
  Steam-release, or Godot-replacement claim;
- #1 and #23 are reconfirmed open in final evidence and are not closed or
  narrowed by this milestone.

## Governance audit

As of this contract, #1 remains the open roadmap anchor and #23 remains the open
governance/constraint anchor. Localization Pipeline v1 adds a bounded Era I
mechanical localization layer while preserving those anchors, local/Rust trusted
ownership, browser/Studio read-only constraints, and the existing trust model.

## String Externalization and Multi-Language Generation v1 implementation (#1833)

Issue: #1833 adds the first executable Rust/local localization surface under the
Localization Pipeline v1 contract. It externalizes a scoped set of user-facing
Deckbuilder UI strings into
`examples/localization-v1/string-catalog.complete.fixture.json`, validates a
fixture-scoped translated locale proposal at
`examples/localization-v1/locale.es.fixture.json`, and locks fail-closed fixtures
for incomplete translations and placeholder drift:

- `examples/localization-v1/invalid/locale.missing.fixture.json`
- `examples/localization-v1/invalid/locale.placeholder-mismatch.fixture.json`

The implementation stays mechanical and conservative: generated locale catalogs
are proposal-only, completeness and placeholder integrity are validated by
Rust/local code, and browser/dashboard/cockpit/Studio surfaces remain read-only
or draft-only. Generated runs/artifacts remain untracked unless fixture-scoped.
This adds no new engine, runtime, UI framework, direct trusted browser/Studio
write, auto-apply, auto-merge, self-approval, reviewer bypass, production-ready
claim, quality/fun claim, shippability claim, release authority, or Godot replacement/parity claim. #1 and #23 remain open.
