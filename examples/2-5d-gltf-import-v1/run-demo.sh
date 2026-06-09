#!/usr/bin/env bash
set -euo pipefail

SOURCE=${SOURCE:-examples/2-5d-gltf-import-v1/source/ortho-demo.gltf}
OUTPUT=${OUTPUT:-runs/gltf-25d-import-demo/fidelity-report.json}
WORKERS=${WORKERS:-2}

cargo run -p ouroforge-cli -- migration gltf25d-demo --source "$SOURCE" --output "$OUTPUT"
node examples/2-5d-gltf-import-v1/render-smoke.test.cjs
cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers "$WORKERS" || true

python3 - "$OUTPUT" <<'PY'
import json
import sys
from pathlib import Path

report = json.loads(Path(sys.argv[1]).read_text())
assert report["schemaVersion"] == "ouroforge.gltf-25d-import-report.v1"
assert "one-way source-project" in report["boundary"]
assert report["nativeScene"]["sceneKind"] == "2.5d-presentation"
assert report["nativeScene"]["cameras"][0]["projection"] == "orthographic"
assert report["nativeScene"]["logicAuthority"].endswith("cannot mutate gameplay truth")
assert report["stateHashPrimary"].startswith("sha256:")
assert report["perceptualRenderSecondary"]["role"] == "secondary corroboration only"
assert any(row["grade"] == "green" for row in report["fidelityRows"])
assert any(row["grade"] == "yellow" for row in report["fidelityRows"])
assert report["reDerivationTasks"]
assert all("ported" not in row["reason"].lower().split() for row in report["fidelityRows"])
assert "Nothing is claimed ported without captured acceptance evidence" in report["oracleRule"]
print("demo: skeleton imports best-effort; logic re-derived+verified; no auto-port claim; state-hash primary verified")
PY
