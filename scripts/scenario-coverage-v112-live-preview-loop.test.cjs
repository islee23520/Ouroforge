#!/usr/bin/env node
// Scenario Coverage v112 smoke validator (Era X M131, #2522).
const fs = require('node:fs');

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

const coverage = fs.readFileSync('docs/scenario-coverage-v112-live-preview-loop.md', 'utf8');
const requiredRows = [
  'v112-validation-parity',
  'v112-loopback-only',
  'v112-serve-write-free',
  'v112-channel-idle-determinism',
  'v112-mechanical-application',
  'v112-transcript-fidelity',
  'v112-no-auto-apply',
  'v112-latency-evidence-shape',
  'v112-audit-linkage',
];
for (const row of requiredRows) {
  assert(coverage.includes(row), `missing v112 row ${row}`);
}

// Every locking artifact named by the coverage doc must exist and still
// contain the assertion that locks its row.
const locks = [
  ['crates/ouroforge-core/tests/preview_session_parity.rs', 'parity_across_all_supported_paths'],
  ['crates/ouroforge-core/tests/preview_session_server_lifecycle.rs', 'serve_lifecycle_round_trip'],
  ['crates/ouroforge-core/tests/preview_session_channel.rs', 'channel_subscriber_receives_pushed_deltas'],
  ['crates/ouroforge-core/tests/preview_transcript_fidelity.rs', 'tampered_transcript_fails_closed'],
  ['examples/game-runtime/preview-channel.test.cjs', 'matches the Rust allowlist'],
  ['examples/game-runtime/preview-channel-client.test.cjs', 'channel-idle'],
  ['tools/preview-loop-audit/audit.mjs', 'record-only'],
  ['docs/preview-session-v1.md', 'Apply-path parity'],
  ['docs/preview-loop-audit-2521.md', 'review → apply → rerun'],
];
for (const [file, needle] of locks) {
  assert(fs.existsSync(file), `missing locking artifact ${file}`);
  const text = fs.readFileSync(file, 'utf8');
  assert(text.includes(needle), `${file} no longer contains locking text: ${needle}`);
}

// Boundary wording guards.
for (const phrase of [
  'in-memory only',
  'record-only',
  'no command bridge, no auto-apply',
  '#1 and #23 remain open',
]) {
  assert(coverage.includes(phrase), `coverage doc missing boundary phrase: ${phrase}`);
}

// The roadmap must record M131 as complete-but-bounded.
const roadmap = fs.readFileSync('docs/roadmap.md', 'utf8');
assert(
  roadmap.includes('Live Preview Loop v1') && roadmap.includes('scenario-coverage-v112-live-preview-loop.md'),
  'roadmap missing the M131 governance wording or coverage pointer'
);

console.log('scenario coverage v112 validator passed');
