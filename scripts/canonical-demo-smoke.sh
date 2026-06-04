#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/canonical-demo-smoke.sh [--keep] [--work-dir PATH]

Runs the canonical non-destructive local demo smoke for issue #369 PA1.3.2.
Generated artifacts are written under an isolated temporary work directory by
calling the Rust CLI through this repository's Cargo.toml. The script never
applies mutations, applies source patches, publishes, merges, changes repository
visibility, writes trusted browser state, or commits generated artifacts.

Options:
  --keep           Keep the generated work directory for inspection.
  --work-dir PATH  Use an explicit generated work directory. It must not be the
                   repository root and will be created if missing.
  -h, --help       Show this help.

Environment:
  OUROFORGE_DEMO_WORKERS   Worker count for run commands (default: 4).
  CARGO_TARGET_DIR         Optional Cargo target dir. Defaults to repo target/.
USAGE
}

KEEP=0
WORK_DIR=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --keep)
      KEEP=1
      shift
      ;;
    --work-dir)
      if [[ $# -lt 2 ]]; then
        echo "error: --work-dir requires a path" >&2
        exit 2
      fi
      WORK_DIR="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
REPO_ROOT=$(cd -- "$SCRIPT_DIR/.." && pwd)
MANIFEST="$REPO_ROOT/Cargo.toml"
WORKERS="${OUROFORGE_DEMO_WORKERS:-4}"

if [[ ! "$WORKERS" =~ ^[1-9][0-9]*$ ]]; then
  echo "error: OUROFORGE_DEMO_WORKERS must be a positive integer" >&2
  exit 2
fi

if [[ -z "$WORK_DIR" ]]; then
  WORK_DIR=$(mktemp -d "${TMPDIR:-/tmp}/ouroforge-canonical-demo-XXXXXX")
else
  WORK_DIR=$(python3 -c 'import os,sys; print(os.path.abspath(sys.argv[1]))' "$WORK_DIR")
  mkdir -p "$WORK_DIR"
fi

REPO_REAL=$(python3 -c 'import os,sys; print(os.path.realpath(sys.argv[1]))' "$REPO_ROOT")
WORK_REAL=$(python3 -c 'import os,sys; print(os.path.realpath(sys.argv[1]))' "$WORK_DIR")
if [[ "$WORK_REAL" == "$REPO_REAL" || "$WORK_REAL" == "$REPO_REAL"/* ]]; then
  echo "error: work directory must be outside the repository: $WORK_DIR" >&2
  exit 2
fi

cleanup() {
  if [[ "$KEEP" -eq 0 ]]; then
    rm -rf -- "$WORK_DIR"
  else
    echo "Kept generated demo work directory: $WORK_DIR"
  fi
}
trap cleanup EXIT

log() {
  printf '\n==> %s\n' "$*"
}

cargo_cli() {
  cargo run --manifest-path "$MANIFEST" -q -p ouroforge-cli -- "$@"
}

run_and_capture() {
  local label=$1
  shift
  local log_file="$WORK_DIR/${label}.log"
  set +e
  "$@" >"$log_file" 2>&1
  local status=$?
  set -e
  echo "$status" >"$WORK_DIR/${label}.status"
  echo "[$label] exit=$status log=$log_file"
  return 0
}

log "Canonical demo smoke workspace"
echo "repo=$REPO_ROOT"
echo "work_dir=$WORK_DIR"
echo "workers=$WORKERS"
mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

log "Project validation"
cargo_cli project validate "$REPO_ROOT/examples/playable-demo-v2/collect-and-exit/ouroforge.project.json" \
  | tee "$WORK_DIR/project-validate.log"

log "Local run/evidence generation"
before_repo_runs=$(mktemp "$WORK_DIR/repo-runs-before.XXXXXX")
after_repo_runs=$(mktemp "$WORK_DIR/repo-runs-after.XXXXXX")
find "$REPO_ROOT/runs" -maxdepth 1 -type d -name 'run-*' -print 2>/dev/null | sort >"$before_repo_runs"
(cd "$REPO_ROOT" && run_and_capture platformer-run cargo_cli run seeds/platformer.yaml --workers "$WORKERS")
(cd "$REPO_ROOT" && run_and_capture engine-expansion-run cargo_cli run seeds/engine-expansion-v1-demo.yaml --workers "$WORKERS")
find "$REPO_ROOT/runs" -maxdepth 1 -type d -name 'run-*' -print 2>/dev/null | sort >"$after_repo_runs"
mkdir -p "$WORK_DIR/runs"
while IFS= read -r repo_run_dir; do
  [[ -n "$repo_run_dir" ]] || continue
  if ! grep -Fxq "$repo_run_dir" "$before_repo_runs"; then
    mv "$repo_run_dir" "$WORK_DIR/runs/"
  fi
done <"$after_repo_runs"
RUN_DIRS=()
while IFS= read -r run_dir; do
  RUN_DIRS+=("$run_dir")
done < <(find "$WORK_DIR/runs" -maxdepth 1 -type d -name 'run-*' | sort)
if [[ "${#RUN_DIRS[@]}" -lt 2 ]]; then
  echo "error: expected at least two generated runs under $WORK_DIR/runs" >&2
  exit 1
fi
BEFORE_RUN="${RUN_DIRS[0]}"
AFTER_RUN="${RUN_DIRS[1]}"
echo "before_run=$BEFORE_RUN" | tee "$WORK_DIR/run-summary.txt"
echo "after_run=$AFTER_RUN" | tee -a "$WORK_DIR/run-summary.txt"
for run_dir in "$BEFORE_RUN" "$AFTER_RUN"; do
  if [[ -f "$run_dir/verdict.json" ]]; then
    status=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("status", "unknown"))' "$run_dir/verdict.json")
    echo "verdict[$(basename "$run_dir")]=$status" | tee -a "$WORK_DIR/run-summary.txt"
  fi
done

log "Dashboard export"
cargo_cli dashboard export --runs-root "$WORK_DIR/runs" --output "$WORK_DIR/dashboard-data.json" \
  | tee "$WORK_DIR/dashboard-export.log"
test -f "$WORK_DIR/dashboard-data.json"

log "Run comparison"
cargo_cli compare "$BEFORE_RUN" "$AFTER_RUN" --output-dir "$WORK_DIR/runs/comparisons" \
  | tee "$WORK_DIR/compare.log"

log "Mutation review decision without apply"
if [[ -f "$AFTER_RUN/mutation/patch-drafts.json" ]]; then
  cargo_cli mutation review --defer \
    --reason "canonical demo records review decision only; no apply" \
    --evidence mutation/patch-drafts.json \
    "$AFTER_RUN" | tee "$WORK_DIR/mutation-review.log"
else
  echo "No patch draft exists in $AFTER_RUN; skipping review decision without treating it as a failure." \
    | tee "$WORK_DIR/mutation-review.log"
fi

log "Visual draft preview without trusted writes"
cargo_cli edit draft-preview \
  --project "$REPO_ROOT/examples/playable-demo-v2/collect-and-exit/ouroforge.project.json" \
  "$REPO_ROOT/examples/visual-edit-draft-v1/valid/collect-and-exit-scene-demo.visual-edit-draft.json" \
  >"$WORK_DIR/visual-scene-preview.json"
cargo_cli edit draft-preview \
  --project "$REPO_ROOT/examples/playable-demo-v2/collect-and-exit/ouroforge.project.json" \
  "$REPO_ROOT/examples/visual-edit-draft-v1/valid/collect-and-exit-asset-frame-demo.visual-edit-draft.json" \
  >"$WORK_DIR/visual-asset-preview.json"

log "Source patch preview validation without apply"
cargo_cli patch-preview validate "$REPO_ROOT/examples/source-mutation-preview-demo-v1/patch-preview-demo.sample.json" \
  >"$WORK_DIR/source-patch-preview-validation.json"

log "Read-only static surface checks"
(cd "$REPO_ROOT" && node --check examples/evidence-dashboard/dashboard.js)
(cd "$REPO_ROOT" && node examples/evidence-dashboard/dashboard.test.cjs)
(cd "$REPO_ROOT" && node --check examples/authoring-cockpit/cockpit.js)
(cd "$REPO_ROOT" && node examples/authoring-cockpit/cockpit.test.cjs)

log "Generated-state audit"
if git -C "$REPO_ROOT" status --short --ignored -- runs examples/evidence-dashboard/dashboard-data.json | grep -v '^!! target/' | grep -q .; then
  echo "error: repository generated-state paths changed unexpectedly" >&2
  git -C "$REPO_ROOT" status --short --ignored -- runs examples/evidence-dashboard/dashboard-data.json >&2
  exit 1
fi

cat >"$WORK_DIR/canonical-demo-summary.txt" <<SUMMARY
canonical-demo-smoke=passed
work_dir=$WORK_DIR
workers=$WORKERS
before_run=$BEFORE_RUN
after_run=$AFTER_RUN
dashboard_data=$WORK_DIR/dashboard-data.json
comparison_output=$WORK_DIR/runs/comparisons
mutation_review_log=$WORK_DIR/mutation-review.log
non_goals=not applied, not published, not merged, no visibility change, no browser trusted writes
SUMMARY
cat "$WORK_DIR/canonical-demo-summary.txt"

log "Canonical demo smoke complete"
