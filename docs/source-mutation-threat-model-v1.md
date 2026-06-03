# Source Mutation Threat Model v1

Source Mutation Threat Model v1 is a Source Mutation Design Gate v1 artifact. It
identifies risks that must be controlled before any future milestone may propose
source-code patch application. It is design/control documentation only and does
not implement source mutation, patch preview production, sandbox execution,
review automation, or browser command execution.

This model inherits the existing Ouroforge trust boundary:

- trusted persistence and validation remain Rust-owned and local-filesystem
  backed;
- browser and Studio examples remain read-only evidence surfaces with copyable
  commands only;
- generated state, run evidence, caches, local worktrees, and build outputs
  remain ignored unless a later issue explicitly scopes deterministic fixtures;
- #1 remains the roadmap/vision anchor and #23 remains the repo-memory/design
  context anchor.

## Assets and trust boundaries

| Asset / boundary | Why it matters | Current authority |
| --- | --- | --- |
| Source files | Define build, runtime, tests, docs, examples, and repository behavior. | Human-reviewed git history. |
| Build scripts and Cargo configuration | Can execute code, select dependencies, and shape test/build behavior. | Human-reviewed source plus local Rust tooling. |
| Dependency manifests and lockfiles | Can introduce transitive code and supply-chain risk. | Human-reviewed source plus package tooling. |
| CI/workflow configuration | Can run privileged automation and expose repository secrets. | Repository maintainers and platform configuration. |
| Generated run evidence | Supports review decisions but can be stale, malformed, or attacker-influenced. | Rust-trusted artifacts with explicit browser/CDP observation boundaries. |
| Browser/Studio surfaces | Useful for inspection but must not become trusted writers or command bridges. | Read-only static/browser displays. |
| Local worktrees/sandboxes | Useful for future preview evaluation but can contaminate source state if not isolated. | Not yet authorized for source mutation apply. |
| Review and rollback records | Needed to explain what changed, why, and how to recover. | Existing review/evidence concepts; future source-patch contracts pending. |

## Actors and assumptions

| Actor | Capability | Threat assumption |
| --- | --- | --- |
| Maintainer | Reviews and merges source changes. | Can make mistakes; needs explicit evidence and non-goal checks. |
| Local agent | Can propose text and run local commands under user authority. | Must not silently expand scope, apply source patches, or bypass review. |
| Browser/Studio user | Can inspect exported evidence and copy commands. | Must not gain trusted write or command-execution authority through UI drift. |
| Malicious or compromised dependency | May attempt build-time, install-time, or runtime execution. | Dependency changes require strict review and are not authorized by this gate. |
| Malicious patch author | May hide risky changes in benign-looking diffs. | Review artifacts must expose file classes, risk labels, tests, and rollback context. |
| Compromised/generated evidence source | May provide stale, partial, or misleading run evidence. | Evidence must be validated and never treated as sole authority for source writes. |

## Risk matrix

Severity values: **Critical**, **High**, **Medium**, **Low**. Likelihood values:
**Likely**, **Possible**, **Unlikely**. Residual acceptance is intentionally not
assigned in SMG1.2.1; required mitigations and blocked-by-design categories are
specified in SMG1.2.2.

| ID | Risk | Example failure mode | Severity | Likelihood |
| --- | --- | --- | --- | --- |
| STM-01 | Arbitrary code execution | A proposed patch modifies a build script, test harness, or runtime entrypoint to execute unexpected commands. | Critical | Possible |
| STM-02 | Build script abuse | `build.rs`, package scripts, generated code hooks, or tool config run code during normal verification. | Critical | Possible |
| STM-03 | Dependency injection | Manifest or lockfile changes add malicious or unreviewed transitive code. | High | Possible |
| STM-04 | CI/secrets leakage | Workflow or config edits expose tokens, environment variables, or private artifacts. | Critical | Unlikely |
| STM-05 | Generated-state contamination | A patch proposal or preview writes into ignored run/cache/build state and later treats it as reviewed source. | High | Possible |
| STM-06 | Browser command bridge drift | A read-only Studio/dashboard surface gradually gains trusted writes or command execution. | Critical | Possible |
| STM-07 | Malicious patch hiding | Large diffs, formatting churn, binary data, generated files, or misleading rationale obscure risky edits. | High | Possible |
| STM-08 | File-class expansion | A narrowly scoped source patch workflow expands to unsafe file classes without a new design decision. | High | Likely |
| STM-09 | Test bypass or evidence spoofing | Patch changes tests, fixtures, or evidence readers to report false confidence. | High | Possible |
| STM-10 | Rollback failure | A future apply path cannot restore prior source state, audit records, or generated cleanup context. | High | Possible |
| STM-11 | Stale preview application | A patch preview is applied after main, dependencies, or target files changed. | High | Possible |
| STM-12 | Path traversal or symlink escape | Patch metadata targets files outside the authorized repository/worktree boundary. | Critical | Possible |
| STM-13 | Source/generated boundary confusion | A source-like deterministic fixture and ignored generated state are mixed in one review. | Medium | Possible |
| STM-14 | Review self-approval | The same actor or agent that proposes a patch records the acceptance without independent review. | High | Possible |
| STM-15 | Public readiness overclaim | Docs imply production/source-mutation readiness before controls and implementation evidence exist. | Medium | Possible |
| STM-16 | Host-specific command assumptions | Preview or verification relies on local tools, shell state, or credentials that are not reproducible. | Medium | Possible |
| STM-17 | Multi-file invariant breakage | A patch edits related files inconsistently, causing latent runtime or documentation drift. | Medium | Likely |
| STM-18 | Audit truncation | Large diffs or long evidence logs omit the actual risky part from reviewer-visible records. | High | Possible |
| STM-19 | Unsafe rollback target | Rollback data points at stale hashes, wrong branches, generated paths, or untrusted backups. | High | Possible |
| STM-20 | Governance anchor drift | #1 or #23 is closed, replaced, or narrowed by source-mutation work without explicit governance. | Medium | Possible |

## Design-only boundary for this artifact

SMG1.2.1 records risks and threat assumptions only. It does not define the final
mitigation policy, blocked source classes, preview schema, source patch review
state machine, sandbox runner, rollback format, Studio implementation, or source
mutation apply path. Those remain separate design/control issues in the Source
Mutation Design Gate sequence.
