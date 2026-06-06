const assert = require("assert");
const fs = require("fs");
const path = require("path");

const root = path.resolve(__dirname, "../..");
const requiredGates = ["mechanical", "runtime", "visual", "semantic"];
const requiredStages = ["seed", "build", "observe", "verify", "journal", "evolve"];
const forbiddenClaims = [
  "broad genre support",
  "production-ready",
  "production readiness",
  "Godot replacement",
  "Godot-replacement",
];

function readText(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(readText(relativePath));
}

function exists(relativePath) {
  return fs.existsSync(path.join(root, relativePath));
}

function assertRepoRef(relativePath, label = "repo ref") {
  assert(exists(relativePath), `${label} is stale or missing: ${relativePath}`);
}

function assertNoForbiddenClaims(relativePath) {
  const text = readText(relativePath);
  const lower = text.toLowerCase();
  for (const claim of forbiddenClaims) {
    const claimLower = claim.toLowerCase();
    assert(
      !lower.includes(claimLower)
        || lower.includes(`no ${claimLower}`)
        || lower.includes(`not a ${claimLower}`)
        || lower.includes(`not for a ${claimLower}`)
        || lower.includes(`does not claim ${claimLower}`)
        || lower.includes(`does not replace ${claimLower}`),
      `${relativePath} contains unsupported public wording: ${claim}`,
    );
  }
}

function assertSetEquals(actual, expected, label) {
  assert.deepStrictEqual([...actual].sort(), [...expected].sort(), label);
}

function validateLoopCoverage(relativePath) {
  const artifact = readJson(relativePath);
  assert.strictEqual(artifact.schemaVersion, "loop-coverage-metric-v1");
  assert.strictEqual(artifact.fixtureScoped, true);
  assert.strictEqual(artifact.summary.status, "computed");
  assert(artifact.summary.coverageFraction > 0);
  const stages = new Set();
  for (const input of artifact.inputs) {
    assertRepoRef(input.artifactRef, "loop coverage artifactRef");
    assert(input.sourceRef, "loop coverage input has sourceRef");
    for (const stage of input.loopStageRefs) {
      stages.add(stage);
    }
    assert(
      ["loop-produced", "loop-verified", "manual"].includes(input.provenanceClass),
      `unexpected provenance class ${input.provenanceClass}`,
    );
  }
  for (const stage of artifact.projectId === "signal_gate_platformer_demo" ? requiredStages : ["seed", "build", "observe", "verify"]) {
    assert(stages.has(stage), `${relativePath} missing loop stage ${stage}`);
  }
  assert(artifact.boundary.toLowerCase().includes("not a quality score"));
  return artifact;
}

function validateVerdict(relativePath) {
  const verdict = readJson(relativePath);
  assert.strictEqual(verdict.schemaVersion, "four-gate-verdict-v1");
  assert.strictEqual(verdict.fixtureScoped, true);
  assert.strictEqual(verdict.verdict, "pass");
  assertSetEquals(new Set(verdict.gates.map((gate) => gate.id)), requiredGates, "four gate ids");
  for (const gate of verdict.gates) {
    assert.strictEqual(gate.status, "pass");
    assert(gate.finding.length > 0, `${gate.id} finding is required`);
    for (const ref of gate.evidenceRefs) {
      assertRepoRef(ref, `${gate.id} evidenceRef`);
    }
  }
  assert(verdict.boundary.includes("no production-readiness claim"));
  return verdict;
}

function validateComparison(relativePath, expectedStatus) {
  const comparison = readJson(relativePath);
  assert.strictEqual(comparison.schemaVersion, "second-game-generalization-comparison-v1");
  assert.strictEqual(comparison.fixtureScoped, true);
  assert.strictEqual(comparison.status, expectedStatus);
  assert.strictEqual(comparison.classes.length, 2, "comparison must include both game classes");
  assertSetEquals(
    new Set(comparison.classes.map((entry) => entry.id)),
    ["collect-and-exit", "signal-gate-platformer"],
    "comparison class ids",
  );
  for (const entry of comparison.classes) {
    assertRepoRef(entry.seedRef, `${entry.id} seedRef`);
    assertRepoRef(entry.scenarioRef, `${entry.id} scenarioRef`);
    assertRepoRef(entry.loopCoverageRef, `${entry.id} loopCoverageRef`);
    assert.deepStrictEqual(entry.verdictGateIds, requiredGates, `${entry.id} gate ids`);
    assert.deepStrictEqual(entry.loopStageIds, requiredStages, `${entry.id} loop stages`);
  }
  assert(comparison.findings.length > 0, "comparison findings are required");
  for (const finding of comparison.findings) {
    assert(finding.id, "finding id is required");
    assert(finding.summary, "finding summary is required");
    assert(finding.evidenceRefs.length > 0, `${finding.id} evidence refs are required`);
    for (const ref of finding.evidenceRefs) {
      assertRepoRef(ref, `${finding.id} evidenceRef`);
    }
  }
  return comparison;
}

function comparisonIsValid(relativePath) {
  try {
    validateComparison(relativePath, "comparable");
    return true;
  } catch {
    return false;
  }
}

assertRepoRef("examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml");
assertRepoRef("examples/playable-demo-v2/collect-and-exit/scenarios/collect-and-exit.json");
assertRepoRef("examples/playable-demo-v2/collect-and-exit/ouroforge.project.json");

const signalManifest = readJson("examples/signal-gate-platformer/ouroforge.project.json");
assert.strictEqual(signalManifest.schemaVersion, "project-manifest-v1");
assert.strictEqual(signalManifest.generated.roots.includes(signalManifest.runsRoot), true);

const signalScenario = readJson("examples/signal-gate-platformer/scenarios/signal-gate-platformer.json");
assert.strictEqual(signalScenario.schemaVersion, "scenario-pack-v1");
assert.strictEqual(signalScenario.seed, "seeds/signal-gate-platformer.yaml");
assert.deepStrictEqual(signalScenario.scenes, ["scenes/signal-gate-platformer.scene.json"]);

validateVerdict("examples/signal-gate-platformer/fixtures/evidence/four-gate-verdict.fixture.json");
validateLoopCoverage("examples/signal-gate-platformer/fixtures/evidence/loop-coverage.fixture.json");
validateLoopCoverage("examples/signal-gate-platformer/fixtures/evidence/collect-and-exit-loop-coverage.golden.json");
validateComparison("examples/signal-gate-platformer/fixtures/generalization/comparable-pass.fixture.json", "comparable");
validateComparison("examples/signal-gate-platformer/fixtures/generalization/gap-found.fixture.json", "gap-found");

assert.strictEqual(
  comparisonIsValid("examples/signal-gate-platformer/fixtures/generalization/invalid/missing-comparison-input.fixture.json"),
  false,
  "missing comparison input fixture must fail",
);
assert.strictEqual(
  comparisonIsValid("examples/signal-gate-platformer/fixtures/generalization/invalid/incomparable-shape.fixture.json"),
  false,
  "incomparable shape fixture must fail",
);
assert.strictEqual(
  comparisonIsValid("examples/signal-gate-platformer/fixtures/generalization/invalid/stale-ref.fixture.json"),
  false,
  "stale ref fixture must fail",
);

for (const doc of [
  "docs/second-game-class-loop-generalization-v1.md",
  "docs/signal-gate-platformer-gdd-v1.md",
  "docs/second-game-class-loop-generalization-v1-demo.md",
  "docs/scenario-coverage-v22.md",
]) {
  const text = readText(doc);
  assert(text.includes("fixture"), `${doc} should state fixture boundary`);
  assertNoForbiddenClaims(doc);
}
