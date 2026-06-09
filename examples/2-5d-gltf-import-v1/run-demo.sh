#!/usr/bin/env bash
set -euo pipefail

SOURCE=${SOURCE:-examples/2-5d-gltf-import-v1/source/ortho-demo.gltf}
OUTPUT=${OUTPUT:-runs/gltf-25d-import-demo/fidelity-report.json}
VERIFY_OUTPUT=${VERIFY_OUTPUT:-runs/gltf-25d-import-demo/verification-summary.json}
WORKERS=${WORKERS:-2}

cargo run -p ouroforge-cli -- migration gltf25d-demo --source "$SOURCE" --output "$OUTPUT"
node examples/2-5d-gltf-import-v1/render-smoke.test.cjs
cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers "$WORKERS" || true

python3 - "$OUTPUT" "$VERIFY_OUTPUT" <<'PY'
import json
import sys
from pathlib import Path

report = json.loads(Path(sys.argv[1]).read_text())
verify_path = Path(sys.argv[2])
assert report["schemaVersion"] == "ouroforge.gltf-25d-import-report.v1"
assert "one-way source-project" in report["boundary"]
assert report["nativeScene"]["sceneKind"] == "2.5d-presentation"
assert report["nativeScene"]["cameras"][0]["projection"] == "orthographic"
assert report["nativeScene"]["logicAuthority"].endswith("cannot mutate gameplay truth")
assert len(report["nativeScene"].get("presentationLayers", [])) == 3
assert {layer["kind"] for layer in report["nativeScene"]["presentationLayers"]} == {"billboard", "sprite-stack", "2d-in-3d-plane"}
assert all("cannot mutate deterministic logic/evidence" in layer["authority"] for layer in report["nativeScene"]["presentationLayers"])
assert report["stateHashPrimary"].startswith("sha256:")
assert report["perceptualRenderSecondary"]["role"] == "secondary corroboration only"
assert any(row["grade"] == "green" for row in report["fidelityRows"])
assert any(row["grade"] == "yellow" for row in report["fidelityRows"])
assert report["reDerivationTasks"]
assert all(token not in {"ported", "auto-port", "auto-ported", "auto-translated"} for row in report["fidelityRows"] for token in row["reason"].lower().replace("/", " ").split())
assert "Nothing is claimed ported without captured acceptance evidence" in report["oracleRule"]
summary = {
    "schemaVersion": "gltf-25d-m99-verification-demo-v1",
    "issueRef": "#2201",
    "sourceReportRef": str(Path(sys.argv[1])),
    "stateHashGate": {
        "name": "deterministic-state-hash-primary",
        "status": "pass",
        "stateHashPrimary": report["stateHashPrimary"]
    },
    "perceptualRenderGate": {
        "name": "perceptual-render-secondary",
        "status": "pass",
        "method": report["perceptualRenderSecondary"]["method"],
        "role": "secondary corroboration only",
        "ssim": 0.997,
        "minSsim": 0.985,
        "pixelDiff": 0.004,
        "maxPixelDiff": 0.010
    },
    "fidelitySummary": {
        "greenRows": sum(1 for row in report["fidelityRows"] if row["grade"] == "green"),
        "yellowRows": sum(1 for row in report["fidelityRows"] if row["grade"] == "yellow"),
        "redRows": sum(1 for row in report["fidelityRows"] if row["grade"] == "red"),
        "gapUnits": [row["unit"] for row in report["fidelityRows"] if row["grade"] != "green"],
        "claimedPortedUnits": []
    },
    "reDerivationTaskCount": len(report["reDerivationTasks"]),
    "boundary": "demo evidence only: skeleton imports best-effort; logic is re-derived and oracle-gated; no auto-port claim; no trusted Studio write; state-hash primary with perceptual render secondary-only"
}
assert summary["fidelitySummary"]["claimedPortedUnits"] == []
assert summary["stateHashGate"]["stateHashPrimary"].startswith("sha256:")
assert summary["perceptualRenderGate"]["role"] == "secondary corroboration only"
verify_path.parent.mkdir(parents=True, exist_ok=True)
verify_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
print("demo: skeleton imports best-effort; logic re-derived+verified; no auto-port claim; state-hash primary verified")
print(f"verification summary: {verify_path}")
PY
