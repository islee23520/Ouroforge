#!/usr/bin/env bash
set -euo pipefail

PROJECT=${PROJECT:-examples/godot-2d-adapter-v1/sample-project}
OUTPUT=${OUTPUT:-examples/godot-2d-adapter-v1/generated/import-verification-report.json}
WORKERS=${WORKERS:-2}

cargo run -p ouroforge-cli -- migration verify-demo --project "$PROJECT" --output "$OUTPUT"
cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers "$WORKERS" || true

python3 - "$OUTPUT" <<'PY'
import json
import sys
from pathlib import Path

report = json.loads(Path(sys.argv[1]).read_text())
assert report["schema_version"] == "import-verification-report-v1"
assert report["skeleton_verification"]["status"] == "passed"
assert report["skeleton_verification"]["runner"] == "openchrome-local-skeleton-smoke"
assert report["claimed_ported_units"] == []
assert report["fidelity_report"]["clean"] > 0
assert report["fidelity_report"]["flagged"] > 0
assert report["fidelity_report"]["rederive"] > 0
assert report["verification_state_hash"].startswith("sha256:")
assert report["provenance"]["origin"] == "godot"
assert report["provenance"]["clean_room_source_only"] is True
assert report["provenance"]["decompiled_source_copied"] is False
print(
    "demo: skeleton imports best-effort; logic has Era R re-derivation tasks; "
    "openchrome skeleton smoke passed; no auto-port claim; state hash verified"
)
PY
