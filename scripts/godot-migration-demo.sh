#!/usr/bin/env bash
set -euo pipefail
cargo run -p ouroforge-cli -- migration godot-demo \
  --project examples/godot-2d-adapter-v1/sample-project \
  --output examples/godot-2d-adapter-v1/generated/fidelity-report.json
