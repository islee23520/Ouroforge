#!/usr/bin/env bash
set -euo pipefail
cargo run -p ouroforge-cli -- migration mapping-demo \
  --project examples/godot-2d-adapter-v1/sample-project \
  --output examples/godot-2d-adapter-v1/generated/mapping-fidelity-report.json
