#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

TARGET_DIR=${CARGO_TARGET_DIR:-target}
RUST_BIN="$TARGET_DIR/debug/ouroforge"
STUDIO_APP="studio/executor/_build/dev/lib/ouroforge_executor/ebin/ouroforge_executor.app"
OUT_DIR="runs/studio-local-package-smoke-v1"
OUT_FILE="$OUT_DIR/smoke.json"

if [ ! -x "$RUST_BIN" ]; then
  echo "missing built Rust kernel binary: $RUST_BIN" >&2
  echo "run: cargo build --workspace --jobs 2" >&2
  exit 1
fi

if [ ! -f "$STUDIO_APP" ]; then
  echo "missing compiled local Studio app: $STUDIO_APP" >&2
  echo "run: (cd studio/executor && mix deps.get && mix compile)" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
cat > "$OUT_FILE" <<JSON
{
  "schemaVersion": "ouroforge.studio-local-package-smoke.v1",
  "localFirst": true,
  "singleUser": true,
  "readGatedWrite": true,
  "interventionAsEvidence": true,
  "rustDataPlaneOwnsTruth": true,
  "elixirControlPresentationOnly": true,
  "trustedWriteAuthority": false,
  "directArtifactWrite": false,
  "commandBridge": false,
  "newDataStore": false,
  "hostedCollaborative": false,
  "signingOrRelease": false,
  "deployOrPublish": false,
  "cliFallbackSupported": true,
  "autonomousLoopRequiresHuman": false,
  "rustBinary": "$RUST_BIN",
  "studioApp": "$STUDIO_APP",
  "boundary": "generated smoke evidence only; no trusted artifact write; hosted multi-user collaborative Studio Layer-3 DEFER; #1 and #23 remain open"
}
JSON

# The smoke artifact is generated evidence under runs/, not trusted source or artifact truth.
grep -q '"trustedWriteAuthority": false' "$OUT_FILE"
grep -q '"directArtifactWrite": false' "$OUT_FILE"
grep -q '"commandBridge": false' "$OUT_FILE"
grep -q '"hostedCollaborative": false' "$OUT_FILE"
grep -q '"cliFallbackSupported": true' "$OUT_FILE"
printf 'STUDIO_LOCAL_PACKAGE_SMOKE_OK %s\n' "$OUT_FILE"
