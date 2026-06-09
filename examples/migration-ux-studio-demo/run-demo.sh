#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="${TMPDIR:-/tmp}/ouroforge-migration-ux-demo"
mkdir -p "$OUT"
cd "$ROOT"

cargo run -p ouroforge-cli -- migration verify-demo \
  --project examples/godot-2d-adapter-v1/sample-project \
  --output "$OUT/godot-import-verification-report.json"

cargo run -p ouroforge-cli -- migration unity-demo \
  --project examples/unity-2d-adapter-v1/sample-project \
  --output "$OUT/unity-fidelity-report.json"

cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers 2 || true
(
  cd studio/executor
  mix test test/ouroforge_executor/migration_ux_demo_test.exs
)

echo "demo: skeleton imports best-effort; logic re-derived+verified by later Era R oracles; no auto-port claim"
echo "reports: $OUT/godot-import-verification-report.json $OUT/unity-fidelity-report.json"
