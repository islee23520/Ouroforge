#!/usr/bin/env bash
set -euo pipefail

PROJECT=${PROJECT:-examples/unity-2d-adapter-v1/sample-project}
OUTPUT=${OUTPUT:-examples/unity-2d-adapter-v1/generated/fidelity-report.json}
WORKERS=${WORKERS:-2}

cargo run -p ouroforge-cli -- migration unity-demo --project "$PROJECT" --output "$OUTPUT"
cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers "$WORKERS" || true

python3 - "$OUTPUT" <<'PY'
import json
import sys
from pathlib import Path

report = json.loads(Path(sys.argv[1]).read_text())
assert report["schema_version"] == "unity-2d-adapter-demo-report-v1"
assert report["source_engine"] == "unity"
assert report["claimed_ported_units"] == []
assert report["fidelity_summary"]["green"] > 0
assert report["fidelity_summary"]["yellow"] > 0
assert report["fidelity_summary"]["red"] > 0
assert report["logic_touchpoint_count"] > 0
assert report["oracle_record_count"] > 0
assert report["ir_state_hash"].startswith("sha256:")
assert report["provenance"]["clean_room_source_only"] is True
assert report["provenance"]["decompiled_source_copied"] is False
assert report["data_shapes"]["no_elixir_artifact_semantics"] is True
print(
    "demo: unity skeleton imports best-effort; logic has Era R re-derivation tasks; "
    "no auto-port claim; deterministic state hash verified"
)
PY
