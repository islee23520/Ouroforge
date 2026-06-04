# Public Wording Guardrail v1

Status: **public-alpha wording policy and scan vocabulary**.

This guardrail keeps Ouroforge public-facing text conservative while the project
remains a local MVP/public-alpha readiness candidate. It does not change
repository visibility, publish packages, add release automation, implement
product behavior, or authorize launch messaging.

## Forbidden claim families

Future public-facing README, docs, templates, issue comments, release notes, and
demo evidence should not claim or imply that Ouroforge currently provides:

| Claim family | Forbidden or high-risk wording | Conservative replacement |
| --- | --- | --- |
| Godot replacement | "Godot replacement", "Godot parity", "better than Godot" | "local-first evidence-native prototype" or "bounded local MVP demo" |
| Production readiness | "production-ready", "commercial-release ready", "ship-ready" | "pre-release", "local MVP", "readiness candidate" |
| Compatibility guarantee | "compatibility-stable", "stable public engine API", "backwards-compatible" | "current contract", "fixture-backed schema", "no compatibility promise yet" |
| Secure sandbox guarantee | "secure sandbox", "safe for untrusted code", "sandbox guarantee" | "trust boundary documented", "not a secure-sandbox guarantee" |
| Autonomous source mutation | "source apply ready", "auto-apply", "auto-merge", "autonomous repair" | "preview/review evidence only", "trusted apply requires later governance" |
| Browser trusted writes | "browser writes files", "browser command bridge", "local server bridge" | "browser surfaces are read-only or draft/evidence producers" |
| Native export/shipping | "native export ready", "desktop/mobile export", "installer", "app-store ready" | "native export is not approved" |
| Plugin/marketplace runtime | "plugin runtime ready", "extension marketplace", "third-party code loading" | "plugin runtime/marketplace remains out of scope" |
| Hosted/cloud/auth support | "hosted service", "cloud runtime", "multi-user auth" | "local-only; hosted/cloud/auth not implemented" |
| Launch/release automation | "autonomous launch", "public release automation", "go-live automation" | "manual maintainer decision; no automation" |
| Support guarantees | "support SLA", "guaranteed support", "security guarantee" | "best-effort docs only; no support SLA" |

## Caution phrases

These terms are allowed only when they are clearly negated, scoped, or used as a
future/design topic rather than a current claim:

- production engine/editor;
- compatibility, stable API, public API;
- sandbox, secure, safe;
- source apply, trusted apply, patch apply;
- native export, packaging, release artifact;
- plugin, marketplace, dynamic loading;
- hosted, cloud, auth, server;
- launch, public release, go-live;
- support, SLA.

## Allowed conservative patterns

Allowed public wording should use one of these forms:

- Explicit negation: "not a Godot replacement" or "does not provide native
  export".
- Boundary statement: "source apply remains blocked until a later governance
  issue".
- Evidence scope: "local generated evidence under ignored `runs/`".
- Manual decision scope: "repository visibility remains a separate manual
  maintainer action".
- Current maturity scope: "pre-release private MVP", "local MVP", or
  "public-alpha readiness candidate".

## Scan command for initial and PR-level audits

Use a read-only grep before opening public-facing docs/template PRs:

```bash
grep -RInE "Godot replacement|Godot parity|production-ready|production ready|commercial-release ready|ship-ready|compatibility-stable|stable public engine API|secure sandbox|sandbox guarantee|source apply ready|auto-apply|auto-merge|autonomous repair|browser trusted write|command bridge|local server bridge|native export ready|desktop/mobile export|installer|app-store ready|plugin runtime ready|extension marketplace|third-party code loading|hosted service|cloud runtime|multi-user auth|autonomous launch|public release automation|go-live automation|support SLA|guaranteed support|security guarantee" README.md CONTRIBUTING.md SECURITY.md docs examples .github || true
```

Every match must be classified before merge as one of:

1. **Conservative negation** — the text explicitly says the claim is not true.
2. **Boundary/non-goal** — the text describes forbidden or out-of-scope work.
3. **Future/design gate** — the text names a later governance/design topic.
4. **Ambiguous/current claim** — the text could be read as a present capability;
   fix or replace it before merging.

## PR evidence expectation

PRs that touch public-facing wording should include:

- scan command used;
- changed files scanned;
- count or examples of matches;
- classification summary;
- any wording replacements made;
- generated-state audit;
- confirmation that #1 and #23 remain open.
