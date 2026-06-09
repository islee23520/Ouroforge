#!/usr/bin/env bash
# ci/fast-check.sh — the fast PR gate for Ouroforge, invoked by motjaengi/fast-ci
# (run: `bash ci/fast-check.sh`). Design goals: high CI value, minimal bottleneck,
# and CONFLICT-SAFE for a repo where parallel sessions + an autonomous pipeline are
# merging code continuously.
#
# Why changed-scoped (not whole-repo): a repo-wide `cargo fmt`/clippy gate would
# either reformat the world (conflicting with every in-flight PR) or red-flag
# pre-existing debt on unrelated PRs. Instead we gate only what the PR CHANGES:
#   - formatting of changed .rs files,
#   - clippy + tests of changed crates (+ a cheap compile-check of all test bins),
#   - Elixir checks only when studio/ changed.
# This keeps new code clean without a disruptive mass-reformat. A one-time
# `cargo fmt --all` + clippy sweep can be run later in a quiet window, or via the
# full lane below (CI_FULL=1) on merge/nightly.
#
# Speed levers (configure on the runner, not here): reuse a persistent
# CARGO_TARGET_DIR across runs and/or set RUSTC_WRAPPER=sccache so clippy/tests are
# incremental. fmt is free; the rest is compile-bound, so a warm cache is the win.
set -euo pipefail
export CARGO_TERM_COLOR=always

BASE="${CI_BASE_REF:-origin/main}"
git fetch origin "${BASE#origin/}" --depth=50 >/dev/null 2>&1 || true
RANGE="${BASE}...HEAD"
changed() { git diff --name-only "$RANGE" 2>/dev/null || true; }

FULL="${CI_FULL:-0}"   # CI_FULL=1 → whole-workspace lane (merge/nightly)

# ---------------------------------------------------------------------------
# 0) Formatting — free, fail fast. Changed .rs only (or whole repo in full lane).
# ---------------------------------------------------------------------------
if [ "$FULL" = "1" ]; then
  echo "::fast-check:: cargo fmt --all --check"
  cargo fmt --all --check
else
  RS=$(changed | grep -E '\.rs$' || true)
  if [ -n "$RS" ]; then
    N=$(printf '%s\n' "$RS" | grep -c .)
    echo "::fast-check:: rustfmt --check on ${N} changed .rs file(s)"
    printf '%s\0' $RS | xargs -0 rustfmt --edition 2021 --check
  else
    echo "::fast-check:: no changed .rs files — skipping fmt"
  fi
fi

# ---------------------------------------------------------------------------
# Determine crate scope. Foundation-crate changes ripple widely → whole workspace.
# ---------------------------------------------------------------------------
CHANGED_CRATES=$(changed | grep '^crates/' | cut -d/ -f2 | sort -u || true)
FOUNDATION=$(echo "$CHANGED_CRATES" | grep -E 'ouroforge-(core-types|evidence|ledger|evaluator)' || true)
RUST_TOUCHED=0; echo "$CHANGED_CRATES" | grep -q . && RUST_TOUCHED=1

if [ "$FULL" = "1" ] || [ -n "$FOUNDATION" ]; then
  SCOPE="--workspace"
elif [ "$RUST_TOUCHED" = "1" ]; then
  SCOPE=""; for c in $CHANGED_CRATES; do SCOPE="$SCOPE -p $c"; done
else
  SCOPE=""   # no Rust changed
fi

if [ "$FULL" = "1" ] || [ "$RUST_TOUCHED" = "1" ]; then
  # 1) clippy = build + lint in one pass (replaces a separate `cargo build`)
  echo "::fast-check:: cargo clippy ${SCOPE} --all-targets -- -D warnings"
  cargo clippy ${SCOPE} --all-targets --jobs 2 -- -D warnings

  # 2) tests: nextest on the scope (fast, relevant), with retries for the known
  #    flaky source_patch_stale_target_guard. Fall back to cargo test if no nextest.
  if command -v cargo-nextest >/dev/null 2>&1; then
    echo "::fast-check:: cargo nextest run ${SCOPE}"
    cargo nextest run ${SCOPE} --retries 2 --jobs 4
  else
    echo "::fast-check:: cargo test ${SCOPE} (nextest not found)"
    cargo test ${SCOPE} --jobs 4
  fi

  # 3) cheap safety net: compile (don't run) all test binaries to catch cross-crate breaks
  echo "::fast-check:: cargo test --workspace --no-run"
  cargo test --workspace --no-run --jobs 4
else
  echo "::fast-check:: no Rust crate changed — skipping clippy/tests"
fi

# ---------------------------------------------------------------------------
# 4) Elixir (studio/) — only when it changed. fast-ci is otherwise Rust-only.
# ---------------------------------------------------------------------------
if [ "$FULL" = "1" ] || changed | grep -q '^studio/'; then
  if [ -f studio/mix.exs ]; then
    echo "::fast-check:: studio/ Elixir checks (mix format/compile/test)"
    ( cd studio && mix deps.get && mix format --check-formatted && mix compile --warnings-as-errors && mix test )
  fi
fi

echo "::fast-check:: OK"
