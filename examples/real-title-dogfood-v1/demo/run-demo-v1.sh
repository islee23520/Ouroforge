#!/usr/bin/env bash
set -euo pipefail

# Era L M68 real-title dogfood demo.
# Runs the existing deckbuilder seed through the existing openchrome/evidence
# loop, then resumes the existing dogfood harness against that run ledger.
# Generated run artifacts remain under runs/ and are not committed.

workers="${WORKERS:-2}"
seed="${SEED_PATH:-seeds/dogfood-deckbuilder.yaml}"
runs_root="${RUNS_ROOT:-runs}"

cargo run -p ouroforge-cli -- run "$seed" --workers "$workers"
run_dir="$(ls -td "$runs_root"/run-* | head -n 1)"

cargo run -p ouroforge-cli -- dogfood harness \
  --seed-path "$seed" \
  --runs-root "$runs_root" \
  --workers "$workers" \
  --resume-run-dir "$run_dir" \
  --friction-json '[]'

cargo run -p ouroforge-cli -- ledger list "$run_dir"
