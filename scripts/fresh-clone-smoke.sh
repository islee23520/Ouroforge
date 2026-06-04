#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/fresh-clone-smoke.sh [--keep] [--work-dir PATH]

Runs the issue #368 PA1.2.2 fresh-clone-style onboarding smoke from an isolated
copy of the tracked repository files. Generated Cargo, run, dashboard, and
project-scaffold outputs stay under the isolated work directory, not the
maintainer worktree.

The script does not install dependencies, publish artifacts, change repository
visibility, apply source patches, run browser trusted writes, or commit generated
state.

Options:
  --keep           Keep the generated work directory for inspection.
  --work-dir PATH  Use an explicit generated work directory. It must be outside
                   the repository root and empty or absent.
  -h, --help       Show this help.

Environment:
  OUROFORGE_FRESH_CLONE_WORKERS  Worker count for run commands (default: 2; use 2+ for browser/scenario evidence).
  OUROFORGE_CHROME               Optional Chrome/Chromium executable path when not found at a standard path.
  CARGO_TARGET_DIR               Optional Cargo target dir. Must be outside the repository. Defaults to WORK_DIR/target.
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
REPO_REAL=$(python3 -c 'import os,sys; print(os.path.realpath(sys.argv[1]))' "$REPO_ROOT")
WORKERS="${OUROFORGE_FRESH_CLONE_WORKERS:-2}"

if [[ ! "$WORKERS" =~ ^[1-9][0-9]*$ ]]; then
  echo "error: OUROFORGE_FRESH_CLONE_WORKERS must be a positive integer" >&2
  exit 2
fi
if (( WORKERS < 2 )); then
  echo "error: OUROFORGE_FRESH_CLONE_WORKERS must be at least 2 so browser/scenario evidence is generated" >&2
  exit 2
fi

WORK_DIR_OWNED=0
if [[ -z "$WORK_DIR" ]]; then
  WORK_DIR=$(mktemp -d "${TMPDIR:-/tmp}/ouroforge-fresh-clone-smoke-XXXXXX")
  WORK_DIR_OWNED=1
else
  WORK_DIR=$(python3 -c 'import os,sys; print(os.path.abspath(sys.argv[1]))' "$WORK_DIR")
  WORK_REAL=$(python3 -c 'import os,sys; print(os.path.realpath(sys.argv[1]))' "$WORK_DIR")
  if [[ "$WORK_REAL" == "$REPO_REAL" || "$WORK_REAL" == "$REPO_REAL"/* ]]; then
    echo "error: work directory must be outside the repository: $WORK_DIR" >&2
    exit 2
  fi
  if [[ -e "$WORK_DIR" ]]; then
    if [[ ! -d "$WORK_DIR" ]]; then
      echo "error: --work-dir must be a directory: $WORK_DIR" >&2
      exit 2
    fi
    if [[ -n "$(ls -A -- "$WORK_DIR" 2>/dev/null)" ]]; then
      echo "error: --work-dir must be empty or absent so this smoke can own cleanup: $WORK_DIR" >&2
      exit 2
    fi
  else
    mkdir -p "$WORK_DIR"
  fi
  WORK_DIR_OWNED=1
fi

cleanup() {
  if [[ "$KEEP" -eq 0 ]]; then
    if [[ "$WORK_DIR_OWNED" -eq 1 ]]; then
      rm -rf -- "$WORK_DIR"
    else
      echo "Not removing unowned work directory: $WORK_DIR" >&2
    fi
  else
    echo "Kept generated fresh-clone smoke work directory: $WORK_DIR"
  fi
}
trap cleanup EXIT

log() {
  printf '\n==> %s\n' "$*"
}

CLONE_DIR="$WORK_DIR/repo"
GENERATED_DIR="$WORK_DIR/generated"
mkdir -p "$CLONE_DIR" "$GENERATED_DIR"

GENERATED_STATE_PATHS=(
  runs
  target
  .omx
  .omc
  .openchrome
  .claude
  examples/evidence-dashboard/dashboard-data.json
)
REPO_GENERATED_STATE_BEFORE="$GENERATED_DIR/maintainer-generated-state-before.txt"
REPO_GENERATED_STATE_AFTER="$GENERATED_DIR/maintainer-generated-state-after.txt"

snapshot_repo_generated_state() {
  git -C "$REPO_ROOT" status --short --ignored -- "${GENERATED_STATE_PATHS[@]}" | sort
}

snapshot_repo_generated_state >"$REPO_GENERATED_STATE_BEFORE"

log "Fresh-clone-style workspace"
echo "source_repo=$REPO_ROOT"
echo "work_dir=$WORK_DIR"
echo "clone_dir=$CLONE_DIR"
echo "generated_dir=$GENERATED_DIR"
echo "workers=$WORKERS"

log "Copy tracked repository files into isolated clone directory"
(
  cd "$REPO_ROOT"
  git ls-files -z | tar --null -T - -cf -
) | tar -xf - -C "$CLONE_DIR"

# Resolve and boundary-check the Cargo target dir. A caller-provided
# CARGO_TARGET_DIR controls generated build-output placement just like
# --work-dir, so it must stay outside the maintainer worktree or the smoke
# would leak generated state into the repository it claims to keep clean.
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$WORK_DIR/target}"
CARGO_TARGET_DIR=$(python3 -c 'import os,sys; print(os.path.abspath(sys.argv[1]))' "$CARGO_TARGET_DIR")
CARGO_TARGET_REAL=$(python3 -c 'import os,sys; print(os.path.realpath(sys.argv[1]))' "$CARGO_TARGET_DIR")
if [[ "$CARGO_TARGET_REAL" == "$REPO_REAL" || "$CARGO_TARGET_REAL" == "$REPO_REAL"/* ]]; then
  echo "error: CARGO_TARGET_DIR must be outside the repository so generated build output stays isolated: $CARGO_TARGET_DIR" >&2
  exit 2
fi
export CARGO_TARGET_DIR
MANIFEST="$CLONE_DIR/Cargo.toml"

cargo_cli() {
  cargo run --manifest-path "$MANIFEST" -q -p ouroforge-cli -- "$@"
}

run_and_capture() {
  local label=$1
  shift
  local log_file="$GENERATED_DIR/${label}.log"
  set +e
  "$@" >"$log_file" 2>&1
  local status=$?
  set -e
  echo "$status" >"$GENERATED_DIR/${label}.status"
  echo "[$label] exit=$status log=$log_file"
  if [[ "$status" != "0" ]]; then
    cat "$log_file" >&2 2>/dev/null || true
    return "$status"
  fi
}

cd "$CLONE_DIR"

log "Fresh clone formatting check"
run_and_capture cargo-fmt cargo fmt --check

log "README quickstart validation commands"
run_and_capture seed-validate cargo_cli seed validate seeds/platformer.yaml
run_and_capture project-validate cargo_cli project validate examples/project-workspace-fixtures/valid
run_and_capture project-init cargo_cli project init "$GENERATED_DIR/project-scaffold-smoke" --template minimal-2d
run_and_capture project-bound-run cargo_cli run "$GENERATED_DIR/project-scaffold-smoke/seeds/platformer.yaml" \
  --project "$GENERATED_DIR/project-scaffold-smoke" --scenario-pack smoke --workers "$WORKERS"

log "Baseline local demo run"
run_and_capture platformer-run cargo_cli run seeds/platformer.yaml --workers "$WORKERS"

log "Dashboard export from isolated clone runs"
run_and_capture dashboard-export cargo_cli dashboard export --runs-root runs --output "$GENERATED_DIR/dashboard-data.json"
test -f "$GENERATED_DIR/dashboard-data.json"

log "Read-only static surface checks"
run_and_capture dashboard-node-check node --check examples/evidence-dashboard/dashboard.js
run_and_capture dashboard-node-test node examples/evidence-dashboard/dashboard.test.cjs
run_and_capture cockpit-node-check node --check examples/authoring-cockpit/cockpit.js
run_and_capture cockpit-node-test node examples/authoring-cockpit/cockpit.test.cjs

log "Maintainer worktree generated-state audit"
snapshot_repo_generated_state >"$REPO_GENERATED_STATE_AFTER"
if ! diff -u "$REPO_GENERATED_STATE_BEFORE" "$REPO_GENERATED_STATE_AFTER" >"$GENERATED_DIR/maintainer-generated-state.diff"; then
  echo "error: maintainer generated-state status changed unexpectedly" >&2
  cat "$GENERATED_DIR/maintainer-generated-state.diff" >&2
  exit 1
fi

run_count=$(find "$CLONE_DIR/runs" -maxdepth 1 -type d -name 'run-*' 2>/dev/null | wc -l | tr -d ' ')
cat >"$GENERATED_DIR/fresh-clone-smoke-summary.txt" <<SUMMARY
fresh-clone-smoke=passed
source_repo=$REPO_ROOT
work_dir=$WORK_DIR
clone_dir=$CLONE_DIR
generated_dir=$GENERATED_DIR
workers=$WORKERS
cargo_target_dir=$CARGO_TARGET_DIR
run_count=$run_count
dashboard_data=$GENERATED_DIR/dashboard-data.json
maintainer_generated_state_before=$REPO_GENERATED_STATE_BEFORE
maintainer_generated_state_after=$REPO_GENERATED_STATE_AFTER
non_goals=not installed, not published, not merged, no visibility change, no source apply, no browser trusted writes
SUMMARY
cat "$GENERATED_DIR/fresh-clone-smoke-summary.txt"

log "Fresh-clone onboarding smoke complete"
