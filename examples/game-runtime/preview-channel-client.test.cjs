'use strict';
// M131.2 PR-2 wiring tests (Era X #2519): the runtime's receive-only preview
// channel client. Uses the established vm-context harness with a stubbed
// WebSocket to drive onmessage end-to-end through runtime.js: connect
// guards, delta application into the live world, instrumentation
// timestamps, best-effort acks, reload requests, diagnostics on failure,
// and channel-idle behavior.

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const runtimeDir = __dirname;
const scripts = [
  'collision.js',
  'snapshot.js',
  'assets.js',
  'animation.js',
  'audio.js',
  'renderer.js',
  'tilemap.js',
  'preview-channel.js',
  'runtime.js',
];

function createRuntime() {
  const sockets = [];
  function FakeWebSocket(url) {
    this.url = url;
    this.sent = [];
    this.onopen = null;
    this.onmessage = null;
    this.onclose = null;
    this.onerror = null;
    sockets.push(this);
  }
  FakeWebSocket.prototype.send = function send(payload) {
    this.sent.push(String(payload));
  };
  const context = {
    console,
    Image: function Image() {},
    document: { getElementById: () => null },
    fetch: () => Promise.reject(new Error('fetch disabled in preview channel test')),
    addEventListener: () => {},
    WebSocket: FakeWebSocket,
    performance: { now: () => 42 },
  };
  context.window = context;
  context.globalThis = context;
  vm.createContext(context);
  for (const script of scripts) {
    vm.runInContext(fs.readFileSync(path.join(runtimeDir, script), 'utf8'), context, {
      filename: script,
    });
  }
  return { api: context.__OUROFORGE__, sockets };
}

const scene = {
  id: 'preview-client-test-scene',
  bounds: { width: 320, height: 180 },
  entities: [
    {
      id: 'player',
      sprite: { color: '#8be9fd' },
      components: {
        transform: { x: 32, y: 120 },
        velocity: { x: 0, y: 0 },
        size: { width: 12, height: 14 },
        controllable: true,
        input: { moveSpeed: 60, jumpImpulse: 140 },
      },
    },
  ],
};

function appliedDelta(overrides = {}) {
  return {
    schemaVersion: 'ouroforge.preview-delta.v1',
    deltaId: 'preview-delta-test-1',
    sessionId: 's',
    intentId: 'i',
    sequence: 1,
    kind: 'parameterSet',
    status: 'applied',
    edits: [{ entityId: 'player', path: 'components.input.moveSpeed', value: 90 }],
    beforeSceneHash: { algorithm: 'fnv1a64', value: '0' },
    afterSceneHash: { algorithm: 'fnv1a64', value: '1' },
    ...overrides,
  };
}

let failures = 0;
function run(name, fn) {
  try {
    fn();
    console.log(`ok - ${name}`);
  } catch (error) {
    failures += 1;
    console.error(`not ok - ${name}`);
    console.error(`  ${error && error.message ? error.message : error}`);
  }
}

run('channel-idle: state is idle and api inventory includes the channel surface', () => {
  const { api } = createRuntime();
  const state = api.previewChannelState();
  assert.equal(state.mode, 'idle');
  assert.equal(state.connected, false);
  assert.equal(state.deltasApplied, 0);
  assert.equal(state.timestamps.length, 0);
  const inventory = api.apiInventory();
  const keys = inventory.entries.map((entry) => entry.key);
  assert.ok(keys.includes('connectPreviewChannel'));
  assert.ok(keys.includes('previewChannelState'));
  assert.ok(!inventory.undocumented.includes('connectPreviewChannel'));
  assert.ok(!inventory.undocumented.includes('previewChannelState'));
});

run('connect rejects non-loopback urls with a diagnostic', () => {
  const { api } = createRuntime();
  const result = api.connectPreviewChannel('ws://evil.example:9999/channel');
  assert.equal(result.ok, false);
  const diagnostics = api.getDiagnostics();
  assert.ok(
    diagnostics.some((d) => d.code === 'preview_channel_failed'),
    JSON.stringify(diagnostics)
  );
});

run('pushed applied delta mutates the live world and records instrumentation + ack', () => {
  const { api, sockets } = createRuntime();
  api.loadScene(scene);
  const result = api.connectPreviewChannel('ws://127.0.0.1:5555/channel');
  assert.equal(result.ok, true);
  const socket = sockets[0];
  socket.onopen();
  socket.onmessage({ data: JSON.stringify(appliedDelta()) });

  const world = api.getWorldState();
  const player = world.entities.find((entity) => entity.id === 'player');
  assert.equal(player.components.input.moveSpeed, 90);

  const state = api.previewChannelState();
  assert.equal(state.mode, 'connected');
  assert.equal(state.deltasApplied, 1);
  assert.equal(state.timestamps.length, 1);
  assert.equal(state.timestamps[0].deltaId, 'preview-delta-test-1');
  assert.equal(state.timestamps[0].receivedAt, 42);

  assert.equal(socket.sent.length, 1);
  const ack = JSON.parse(socket.sent[0]);
  assert.equal(ack.type, 'ack');
  assert.equal(ack.deltaId, 'preview-delta-test-1');

  assert.ok(
    world.runtimeEvents.some((event) => event.type === 'runtime.preview.delta_applied'),
    'delta application must be recorded as a runtime event'
  );
});

run('rejected deltas are skipped without touching the world', () => {
  const { api, sockets } = createRuntime();
  api.loadScene(scene);
  api.connectPreviewChannel('ws://127.0.0.1:5555/channel');
  const socket = sockets[0];
  socket.onopen();
  socket.onmessage({
    data: JSON.stringify(appliedDelta({ status: 'rejected', errors: ['nope'] })),
  });
  const world = api.getWorldState();
  const player = world.entities.find((entity) => entity.id === 'player');
  assert.equal(player.components.input.moveSpeed, 60);
  const state = api.previewChannelState();
  assert.equal(state.deltasSkipped, 1);
  assert.equal(state.deltasApplied, 0);
});

run('malformed payloads and apply failures surface as typed diagnostics', () => {
  const { api, sockets } = createRuntime();
  api.loadScene(scene);
  api.connectPreviewChannel('ws://127.0.0.1:5555/channel');
  const socket = sockets[0];
  socket.onopen();
  socket.onmessage({ data: 'not-json' });
  socket.onmessage({
    data: JSON.stringify(
      appliedDelta({
        deltaId: 'preview-delta-test-bad',
        edits: [{ entityId: 'ghost', path: 'components.transform.x', value: 1 }],
      })
    ),
  });
  const state = api.previewChannelState();
  assert.equal(state.deltasFailed, 2);
  const codes = api.getDiagnostics().map((d) => d.code);
  assert.ok(codes.includes('preview_channel_failed'), codes.join(','));
  assert.ok(codes.includes('preview_delta_apply_failed'), codes.join(','));
  const types = api.diagnosticTypes().types.map((t) => t.code);
  for (const code of [
    'preview_delta_apply_failed',
    'preview_delta_schema_unsupported',
    'preview_channel_failed',
  ]) {
    assert.ok(types.includes(code), `diagnostic registry missing ${code}`);
  }
});

run('sceneReload deltas record a reload request', () => {
  const { api, sockets } = createRuntime();
  api.loadScene(scene);
  api.connectPreviewChannel('ws://127.0.0.1:5555/channel');
  const socket = sockets[0];
  socket.onopen();
  socket.onmessage({
    data: JSON.stringify(
      appliedDelta({ kind: 'sceneReload', edits: [], deltaId: 'preview-delta-reload' })
    ),
  });
  const state = api.previewChannelState();
  assert.equal(state.sceneReloads, 1);
  const world = api.getWorldState();
  assert.ok(
    world.runtimeEvents.some((event) => event.type === 'runtime.preview.scene_reload_requested')
  );
});

if (failures > 0) {
  console.error(`\n${failures} preview-channel client test(s) failed`);
  process.exit(1);
}
console.log('\npreview-channel client wiring tests passed');
