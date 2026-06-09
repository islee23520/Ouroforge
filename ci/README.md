# CI: `ci/fast-check.sh`

The fast PR gate for Ouroforge. `motjaengi/fast-ci` (the external runner) should invoke:

```bash
bash ci/fast-check.sh
```

Keeping the check logic in-repo (not only in the bot's environment) makes it
**version-controlled, reviewable, and runnable locally** — a fresh clone runs the
exact same gate the bot does.

## What it gates (changed-scoped, to stay conflict-safe)

The repo has parallel sessions + an autonomous pipeline merging code continuously,
so a whole-repo `cargo fmt`/clippy gate would either reformat the world (conflicting
with every in-flight PR) or red-flag pre-existing debt on unrelated PRs. Instead the
gate checks only what the PR **changes**:

1. **Formatting** — `rustfmt --check` on changed `.rs` files (free, fail-fast).
2. **Clippy** — `cargo clippy --all-targets -- -D warnings` on changed crates
   (foundation-crate changes → whole workspace). Clippy also builds, so there is no
   separate build step.
3. **Tests** — `cargo nextest run` on changed crates (with `--retries 2` for the known
   flaky `source_patch_stale_target_guard`), plus a cheap `cargo test --workspace
   --no-run` to catch cross-crate compile breaks. Full test **execution** of the whole
   workspace is intentionally left to the full lane (below), not every PR.
4. **Elixir** — `mix format --check / compile --warnings-as-errors / test` in `studio/`,
   only when `studio/` changed (fast-ci is otherwise Rust-only).

## Run modes

- **Fast lane (default, every PR):** changed-scoped as above.
- **Full lane (`CI_FULL=1`, recommended on merge-to-main / nightly):** whole-workspace
  `cargo fmt --all --check`, clippy `-D warnings`, full `nextest run`, and Elixir.

```bash
bash ci/fast-check.sh            # fast lane
CI_FULL=1 bash ci/fast-check.sh  # full lane
```

## Runner config (the real speed lever — set on the runner, not in this script)

- Reuse a **persistent `CARGO_TARGET_DIR`** across runs (don't use an isolated dir per
  run) and/or set `RUSTC_WRAPPER=sccache`. `fmt` is free; clippy/tests are compile-bound,
  so a warm cache turns them into fast incremental runs.
- Install `cargo-nextest` for parallel test execution + retries (the script falls back to
  `cargo test` if it's absent).

## Notes

- `CI_BASE_REF` overrides the diff base (default `origin/main`).
- A one-time repo-wide `cargo fmt --all` + clippy sweep can be done later in a quiet
  window (or seen via the full lane); the changed-scoped fast lane keeps **new** code
  clean without a disruptive mass-reformat.
