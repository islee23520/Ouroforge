const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const fixtureDir = __dirname;
const report = JSON.parse(fs.readFileSync(path.join(fixtureDir, 'fidelity-report.fixture.json'), 'utf8'));

assert.equal(report.schemaVersion, 'ouroforge.gltf-25d-import-report.v1');
assert.match(report.boundary, /one-way source-project glTF presentation import only/);
assert.match(report.nativeScene.logicAuthority, /cannot mutate gameplay truth/);
assert.equal(report.nativeScene.sceneKind, '2.5d-presentation');
assert.equal(report.nativeScene.cameras[0].projection, 'orthographic');
assert.equal(report.nativeScene.meshes[0].fidelityGrade, 'green');
assert.ok(report.fidelityRows.some((row) => row.unit === 'extension:VENDOR_custom_shader_note' && row.grade === 'yellow'));
assert.ok(report.reDerivationTasks.some((task) => /logic|physics/.test(task.unit)));
assert.match(report.stateHashPrimary, /^sha256:[0-9a-f]{64}$/);
assert.equal(report.perceptualRenderSecondary.role, 'secondary corroboration only');

function renderPresentation(scene) {
  const camera = scene.cameras.find((item) => item.id === scene.activeCameraId);
  const visibleMeshes = scene.nodes.filter((node) => node.meshRef).map((node) => node.meshRef).sort();
  return {
    status: 'rendered-perceptual-secondary',
    cameraProjection: camera.projection,
    visibleObjectCount: visibleMeshes.length,
    visibleMeshes,
    stateHashPrimary: report.stateHashPrimary,
    boundary: 'Perceptual render smoke only; Rust-owned stateHashPrimary remains authoritative.'
  };
}

const render = renderPresentation(report.nativeScene);
assert.equal(render.cameraProjection, 'orthographic');
assert.equal(render.visibleObjectCount, 1);
assert.deepEqual(render.visibleMeshes, ['tile-mesh']);
assert.match(render.boundary, /Perceptual render smoke only/);
console.log('2.5d glTF import render smoke passed');
