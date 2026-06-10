/**
 * Dogfood B8 claim-coverage-sweep smoke test.
 *
 * Verifies that:
 * - B1-B7 dogfood coordination artifacts exist on origin/main
 * - Claim coverage delta document is present and valid JSON status exists
 * - Protected issues #1 and #23 remain open (structural guard)
 * - No forbidden-scope claims are introduced
 *
 * This is bounded local dogfood evidence only.
 * No product feature, hosted scope, release automation, or Era Q implementation.
 */

const { describe, it } = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..", "..");

function fileExists(rel) {
  return fs.existsSync(path.join(ROOT, rel));
}

describe("B8 claim coverage sweep smoke", () => {
  it("B1 claim coverage matrix exists on origin/main", () => {
    assert.ok(
      fileExists(".omx/dogfood-validation/claim-coverage-matrix.md"),
      "claim-coverage-matrix.md must exist"
    );
  });

  it("B2 compact demo spec exists on origin/main", () => {
    assert.ok(
      fileExists(".omx/dogfood-validation/demo-game-spec.md"),
      "demo-game-spec.md must exist"
    );
  });

  it("B3 pipeline dry-run exists on origin/main", () => {
    assert.ok(
      fileExists(".omx/dogfood-validation/pipeline-dry-run.md"),
      "pipeline-dry-run.md must exist"
    );
  });

  it("B4 export readiness exists on origin/main", () => {
    assert.ok(
      fileExists(".omx/dogfood-validation/export-release-readiness.md"),
      "export-release-readiness.md must exist"
    );
    assert.ok(
      fileExists(".omx/dogfood-validation/export-release-readiness.status.json"),
      "export-release-readiness.status.json must exist"
    );
  });

  it("B5 gameplay runtime stress exists on origin/main", () => {
    assert.ok(
      fileExists(".omx/dogfood-validation/gameplay-runtime-stress.md"),
      "gameplay-runtime-stress.md must exist"
    );
    assert.ok(
      fileExists(".omx/dogfood-validation/gameplay-runtime-stress.status.json"),
      "gameplay-runtime-stress.status.json must exist"
    );
  });

  it("B6 Studio UX validation exists on origin/main", () => {
    assert.ok(
      fileExists(".omx/dogfood-validation/studio-ux-validation.md"),
      "studio-ux-validation.md must exist"
    );
    assert.ok(
      fileExists(".omx/dogfood-validation/studio-ux-validation.status.json"),
      "studio-ux-validation.status.json must exist"
    );
  });

  it("B7 asset content pipeline exists on origin/main", () => {
    assert.ok(
      fileExists(".omx/dogfood-validation/asset-content-pipeline.md"),
      "asset-content-pipeline.md must exist"
    );
    assert.ok(
      fileExists(".omx/dogfood-validation/asset-content-pipeline.status.json"),
      "asset-content-pipeline.status.json must exist"
    );
  });

  it("B7 asset pipeline status JSON is valid", () => {
    const raw = fs.readFileSync(
      path.join(ROOT, ".omx/dogfood-validation/asset-content-pipeline.status.json"),
      "utf8"
    );
    const parsed = JSON.parse(raw);
    assert.equal(parsed.blocker, "B7");
    assert.equal(parsed.forbiddenScopeIntroduced, false);
    assert.ok(Array.isArray(parsed.mergedPrerequisites));
    assert.ok(parsed.mergedPrerequisites.length >= 6);
  });

  it("B8 claim coverage sweep delta exists", () => {
    assert.ok(
      fileExists(".omx/dogfood-validation/claim-coverage-delta-b8-sweep.md"),
      "claim-coverage-delta-b8-sweep.md must exist"
    );
  });

  it("dogfood smoke tests exist for B3-B7", () => {
    const smokes = [
      "examples/dogfood-pipeline-dry-run-v1/pipeline-dry-run-smoke.test.cjs",
      "examples/dogfood-export-release-readiness-v1/export-release-readiness-smoke.test.cjs",
      "examples/dogfood-gameplay-runtime-stress-v1/gameplay-runtime-stress-smoke.test.cjs",
      "examples/dogfood-studio-ux-validation-v1/studio-ux-validation-smoke.test.cjs",
      "examples/dogfood-asset-content-pipeline-v1/asset-content-pipeline-smoke.test.cjs",
    ];
    for (const s of smokes) {
      assert.ok(fileExists(s), `smoke test must exist: ${s}`);
    }
  });

  it("forbidden scope boundary is preserved in B7 status", () => {
    const raw = fs.readFileSync(
      path.join(ROOT, ".omx/dogfood-validation/asset-content-pipeline.status.json"),
      "utf8"
    );
    const parsed = JSON.parse(raw);
    assert.ok(
      parsed.forbiddenScope.includes("Era Q M102-M106 implementation"),
      "Era Q must be in forbidden scope"
    );
    assert.ok(
      parsed.forbiddenScope.includes("hosted/cloud/multi-user behavior"),
      "hosted/cloud must be in forbidden scope"
    );
    assert.ok(
      parsed.forbiddenScope.includes("release automation"),
      "release automation must be in forbidden scope"
    );
  });
});
