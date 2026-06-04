# Test Command Allowlist v1

Test Command Allowlist v1 defines inert safe-command fixture vocabulary for
SMP1.5.1. The allowlist is a data contract for deciding whether a proposed test
command string matches a known safe fixture; it does not execute commands,
launch a sandbox runner, apply patches, write browser state, or mutate CI,
dependencies, or build scripts.

## Safe command fixture vocabulary

Allowed fixtures are intentionally narrow and deterministic:

| Fixture family | Intended match shape | Notes |
| --- | --- | --- |
| Rust formatting | exact `cargo fmt --check` | Formatting verification only. |
| Focused Rust tests | prefix for explicit `cargo test ...` invocations | The command must remain a test command, not a shell script or runner bridge. |
| Rust linting | exact `cargo clippy --all-targets --all-features -- -D warnings` | Lint verification only. |
| Node syntax checks | prefix for known `node --check ...` files | Syntax verification for checked-in JavaScript fixtures. |
| Node smoke tests | prefix for known `node ...test.cjs` files | Focused checked-in smoke tests only. |

A fixture may normalize whitespace before matching, but the fixture remains
copyable/read-model text. Matching an allowed fixture is not permission to run it;
execution must stay outside this schema until a later issue explicitly scopes an
executor or sandbox boundary.

## Boundary

This document only names the safe allowlist vocabulary used by schema fixtures
and focused tests. It does not add forbidden-command breadth, command execution,
sandbox implementation, browser command bridges, source mutation apply behavior,
workflow changes, dependency changes, or generated-state tracking.
