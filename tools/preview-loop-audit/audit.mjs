#!/usr/bin/env node
// M131.4 live preview loop audit driver (Era X #2521).
//
// Exercises the full live preview loop on a real scene and records a
// product-observed evidence bundle in the M116 file layout:
//   serve -> browser connect (?preview=) -> N parameter tweaks -> M scene
//   reloads -> transcript fetch -> draft export -> draft preflight preview.
//
// Latency evidence: the harness clock around POST /intent until the page's
// applied counter advances, observed through CDP Runtime.evaluate (CDP-side
// primary per the #2517 Q2 resolution); in-page receivedAt/appliedAt
// timestamps are captured as secondary corroboration. Budgets are
// RECORD-ONLY this cycle (first dogfood records, hard gate from the next
// cycle).
//
// The WS/CDP helpers mirror tools/live-observability-runner/runner.mjs
// (M116); kept separate so the two run kinds stay independently evolvable.

import { randomBytes } from 'node:crypto';
import { spawn } from 'node:child_process';
import { mkdir, writeFile, rm } from 'node:fs/promises';
import { mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import net from 'node:net';

const SCHEMA_VERSION = 'preview-loop-audit-v1';
const TWEAK_BUDGET_MS = 1000;
const RELOAD_BUDGET_MS = 5000;

function parseArgs(argv) {
  const args = {
    scene: 'examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json',
    project: 'examples/playable-demo-v2/collect-and-exit/ouroforge.project.json',
    draftTargetPath: 'scenes/collect-and-exit.scene.json',
    outRoot: 'runs/preview-loop-audit',
    tweaks: 12,
    reloads: 3,
    bin: null,
    chrome: null,
    runId: null,
  };
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg === '--scene') args.scene = argv[++i];
    else if (arg === '--project') args.project = argv[++i];
    else if (arg === '--draft-target-path') args.draftTargetPath = argv[++i];
    else if (arg === '--out-root') args.outRoot = argv[++i];
    else if (arg === '--tweaks') args.tweaks = Number(argv[++i]);
    else if (arg === '--reloads') args.reloads = Number(argv[++i]);
    else if (arg === '--bin') args.bin = argv[++i];
    else if (arg === '--chrome') args.chrome = argv[++i];
    else if (arg === '--run-id') args.runId = argv[++i];
    else throw new Error(`unknown argument: ${arg}`);
  }
  if (!args.bin) throw new Error('--bin <path to ouroforge binary> is required');
  return args;
}

function freePort() {
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.listen(0, '127.0.0.1', () => {
      const { port } = server.address();
      server.close(() => resolve(port));
    });
    server.on('error', reject);
  });
}

function chromeCandidates(explicit) {
  return [
    explicit,
    process.env.OUROFORGE_CHROME,
    process.env.CHROME_PATH,
    '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
    '/Applications/Chromium.app/Contents/MacOS/Chromium',
    '/usr/bin/google-chrome',
    '/usr/bin/chromium',
  ].filter(Boolean);
}

const children = [];
function spawnChild(command, commandArgs, options = {}) {
  const child = spawn(command, commandArgs, { stdio: ['ignore', 'pipe', 'pipe'], ...options });
  children.push(child);
  return child;
}
function killChildren() {
  for (const child of children) {
    if (!child.killed) {
      try { child.kill('SIGTERM'); } catch {}
    }
  }
}
process.on('exit', killChildren);
process.on('SIGINT', () => { killChildren(); process.exit(130); });

async function waitFor(predicate, timeoutMs, intervalMs = 10, label = 'condition') {
  const start = Date.now();
  for (;;) {
    const value = await predicate();
    if (value) return value;
    if (Date.now() - start > timeoutMs) throw new Error(`timeout waiting for ${label}`);
    await new Promise(resolve => setTimeout(resolve, intervalMs));
  }
}

function readLines(stream, onLine) {
  let buffer = '';
  stream.on('data', chunk => {
    buffer += chunk.toString('utf8');
    let index;
    while ((index = buffer.indexOf('\n')) !== -1) {
      onLine(buffer.slice(0, index));
      buffer = buffer.slice(index + 1);
    }
  });
}

// --- minimal WS + CDP client (mirrors the M116 runner) ---------------------
class WsClient {
  constructor(wsUrl) {
    this.url = new URL(wsUrl);
    this.listeners = new Map();
    this.buffer = Buffer.alloc(0);
    this.handshakeDone = false;
    this.socket = net.createConnection(
      { host: this.url.hostname, port: Number(this.url.port) },
      () => this.handshake()
    );
    this.socket.on('data', chunk => this.onData(chunk));
    this.socket.on('error', error => this.dispatch('error', error));
    this.socket.on('close', () => this.dispatch('close', {}));
  }
  addEventListener(type, handler, options = {}) {
    if (!this.listeners.has(type)) this.listeners.set(type, []);
    this.listeners.get(type).push({ handler, once: Boolean(options.once) });
  }
  dispatch(type, event) {
    const listeners = this.listeners.get(type) ?? [];
    this.listeners.set(type, listeners.filter(listener => {
      listener.handler(event);
      return !listener.once;
    }));
  }
  handshake() {
    const key = randomBytes(16).toString('base64');
    this.socket.write([
      `GET ${this.url.pathname}${this.url.search} HTTP/1.1`,
      `Host: ${this.url.host}`,
      'Upgrade: websocket',
      'Connection: Upgrade',
      `Sec-WebSocket-Key: ${key}`,
      'Sec-WebSocket-Version: 13',
      '',
      '',
    ].join('\r\n'));
  }
  onData(chunk) {
    this.buffer = Buffer.concat([this.buffer, chunk]);
    if (!this.handshakeDone) {
      const end = this.buffer.indexOf('\r\n\r\n');
      if (end === -1) return;
      const header = this.buffer.subarray(0, end).toString('utf8');
      if (!header.includes(' 101 ')) {
        this.dispatch('error', new Error(`WebSocket handshake failed: ${header.split('\r\n')[0]}`));
        return;
      }
      this.handshakeDone = true;
      this.buffer = this.buffer.subarray(end + 4);
      this.dispatch('open', {});
    }
    while (this.buffer.length >= 2) {
      const first = this.buffer[0];
      const opcode = first & 0x0f;
      const second = this.buffer[1];
      let length = second & 0x7f;
      let offset = 2;
      if (length === 126) {
        if (this.buffer.length < 4) return;
        length = this.buffer.readUInt16BE(2);
        offset = 4;
      } else if (length === 127) {
        if (this.buffer.length < 10) return;
        const big = this.buffer.readBigUInt64BE(2);
        if (big > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error('WebSocket frame too large');
        length = Number(big);
        offset = 10;
      }
      const masked = Boolean(second & 0x80);
      const maskOffset = offset;
      if (masked) offset += 4;
      if (this.buffer.length < offset + length) return;
      let payload = this.buffer.subarray(offset, offset + length);
      if (masked) {
        const mask = this.buffer.subarray(maskOffset, maskOffset + 4);
        payload = Buffer.from(payload.map((byte, index) => byte ^ mask[index % 4]));
      }
      this.buffer = this.buffer.subarray(offset + length);
      if (opcode === 1) this.dispatch('message', { data: payload.toString('utf8') });
      else if (opcode === 8) this.close();
    }
  }
  send(text) {
    const payload = Buffer.from(text);
    const mask = randomBytes(4);
    let header;
    if (payload.length < 126) {
      header = Buffer.from([0x81, 0x80 | payload.length]);
    } else if (payload.length < 65536) {
      header = Buffer.alloc(4);
      header[0] = 0x81;
      header[1] = 0x80 | 126;
      header.writeUInt16BE(payload.length, 2);
    } else {
      header = Buffer.alloc(10);
      header[0] = 0x81;
      header[1] = 0x80 | 127;
      header.writeBigUInt64BE(BigInt(payload.length), 2);
    }
    const masked = Buffer.from(payload.map((byte, index) => byte ^ mask[index % 4]));
    this.socket.write(Buffer.concat([header, mask, masked]));
  }
  close() {
    if (!this.socket.destroyed) this.socket.end();
  }
}

class CdpClient {
  constructor(wsUrl) {
    this.ws = new WsClient(wsUrl);
    this.nextId = 1;
    this.pending = new Map();
    this.handlers = new Map();
  }
  async open() {
    await new Promise((resolve, reject) => {
      this.ws.addEventListener('open', resolve, { once: true });
      this.ws.addEventListener('error', reject, { once: true });
    });
    this.ws.addEventListener('message', event => this.onMessage(event));
  }
  onMessage(event) {
    const message = JSON.parse(event.data);
    if (message.id && this.pending.has(message.id)) {
      const { resolve, reject } = this.pending.get(message.id);
      this.pending.delete(message.id);
      if (message.error) reject(new Error(message.error.message));
      else resolve(message.result ?? {});
      return;
    }
    if (message.method && this.handlers.has(message.method)) {
      for (const handler of this.handlers.get(message.method)) handler(message.params ?? {});
    }
  }
  on(method, handler) {
    if (!this.handlers.has(method)) this.handlers.set(method, []);
    this.handlers.get(method).push(handler);
  }
  send(method, params = {}) {
    const id = this.nextId++;
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.ws.send(JSON.stringify({ id, method, params }));
    });
  }
  close() { this.ws.close(); }
}

async function evaluateJson(cdp, expression) {
  const result = await cdp.send('Runtime.evaluate', {
    expression,
    awaitPromise: true,
    returnByValue: true,
    userGesture: false,
  });
  if (result.exceptionDetails) {
    throw new Error(result.exceptionDetails.text ?? 'Runtime.evaluate failed');
  }
  return result.result?.value;
}

async function capturePng(cdp, file) {
  const screenshot = await cdp.send('Page.captureScreenshot', { format: 'png', captureBeyondViewport: false });
  await writeFile(file, Buffer.from(screenshot.data, 'base64'));
}

function percentile(sorted, fraction) {
  if (sorted.length === 0) return null;
  const index = Math.min(sorted.length - 1, Math.ceil(fraction * sorted.length) - 1);
  return sorted[Math.max(0, index)];
}

function latencyStats(samples) {
  const sorted = [...samples].sort((a, b) => a - b);
  return {
    count: samples.length,
    minMs: sorted[0] ?? null,
    p50Ms: percentile(sorted, 0.5),
    p95Ms: percentile(sorted, 0.95),
    maxMs: sorted[sorted.length - 1] ?? null,
  };
}

async function postIntent(serveUrl, intent) {
  const response = await fetch(`${serveUrl}/intent`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(intent),
  });
  return response.json();
}

function runCli(bin, cliArgs, cwd) {
  return new Promise((resolve, reject) => {
    const child = spawn(bin, cliArgs, { cwd, stdio: ['ignore', 'pipe', 'pipe'] });
    let stdout = '';
    let stderr = '';
    child.stdout.on('data', chunk => { stdout += chunk; });
    child.stderr.on('data', chunk => { stderr += chunk; });
    child.on('close', code => {
      if (code === 0) resolve(stdout);
      else reject(new Error(`${cliArgs.join(' ')} exited ${code}: ${stderr}`));
    });
  });
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  const repoRoot = process.cwd();
  const runId = args.runId ?? `preview-loop-audit-${process.pid}`;
  const bundleDir = path.join(repoRoot, args.outRoot, runId);
  await mkdir(path.join(bundleDir, 'screenshots'), { recursive: true });

  const consoleLines = [];
  const worldSamples = [];
  const frameSamples = [];

  // 1) Static server for the runtime page.
  const httpPort = await freePort();
  spawnChild('python3', ['-m', 'http.server', String(httpPort), '--bind', '127.0.0.1', '--directory', repoRoot]);

  // 2) Preview validation server.
  const serve = spawnChild(args.bin, ['preview', 'serve', args.scene], { cwd: repoRoot });
  let serveInfo = null;
  let serveReport = '';
  readLines(serve.stdout, line => {
    if (!serveInfo && line.includes('"url"')) serveInfo = JSON.parse(line);
    else serveReport += `${line}\n`;
  });
  await waitFor(() => serveInfo, 10000, 25, 'preview serve startup');
  const serveUrl = serveInfo.url;
  const servePort = new URL(serveUrl).port;
  const sessionId = serveInfo.sessionId;

  // 3) Headless Chrome with CDP.
  const cdpPort = await freePort();
  const profileDir = mkdtempSync(path.join(tmpdir(), 'ouroforge-preview-audit-'));
  let chromeBin = null;
  for (const candidate of chromeCandidates(args.chrome)) {
    try {
      spawnChild(candidate, [
        '--headless=new', '--disable-gpu', '--remote-debugging-address=127.0.0.1',
        `--remote-debugging-port=${cdpPort}`, `--user-data-dir=${profileDir}`, 'about:blank',
      ]);
      chromeBin = candidate;
      break;
    } catch { /* try next */ }
  }
  if (!chromeBin) throw new Error('no Chrome/Chromium binary found; set OUROFORGE_CHROME');
  await waitFor(async () => {
    try { return (await fetch(`http://127.0.0.1:${cdpPort}/json/version`)).ok; } catch { return false; }
  }, 15000, 100, 'Chrome CDP endpoint');
  const targetResponse = await fetch(`http://127.0.0.1:${cdpPort}/json/new?about:blank`, { method: 'PUT' });
  const target = await targetResponse.json();
  const cdp = new CdpClient(target.webSocketDebuggerUrl);
  await cdp.open();
  await cdp.send('Page.enable');
  await cdp.send('Runtime.enable');
  cdp.on('Runtime.consoleAPICalled', params => {
    consoleLines.push(JSON.stringify({
      type: params.type,
      text: (params.args ?? []).map(a => a.value ?? a.description ?? '').join(' '),
    }));
  });

  // 4) Navigate with ?preview= auto-connect and wait for channel + scene.
  const scenePath = `/${args.scene}`;
  const pageUrl = `http://127.0.0.1:${httpPort}/examples/game-runtime/?scene=${scenePath}&preview=ws://127.0.0.1:${servePort}/channel`;
  await cdp.send('Page.navigate', { url: pageUrl });
  await waitFor(
    () => evaluateJson(cdp, 'Boolean(window.__OUROFORGE__)'),
    15000, 50, 'runtime api'
  );
  await evaluateJson(cdp, 'window.__OUROFORGE__.whenReady()');
  await waitFor(
    () => evaluateJson(cdp, 'window.__OUROFORGE__.previewChannelState().connected'),
    10000, 25, 'preview channel connection'
  );

  const sampleWorld = async label => {
    const world = await evaluateJson(cdp, `(() => { const w = window.__OUROFORGE__.getWorldState(); return { label: ${JSON.stringify('')} || undefined, tick: w.tick, sceneId: w.sceneId, digest: w.runtimeState ? w.runtimeState.digest : null, player: (w.entities.find(e => e.id === 'player') || {}).components || null, preview: window.__OUROFORGE__.previewChannelState(), diagnostics: w.runtimeDiagnostics }; })()`);
    worldSamples.push(JSON.stringify({ label, ...world }));
    const frames = await evaluateJson(cdp, 'window.__OUROFORGE__.getFrameStats ? window.__OUROFORGE__.getFrameStats() : null');
    if (frames) frameSamples.push(JSON.stringify({ label, ...frames }));
  };

  await sampleWorld('start');
  await capturePng(cdp, path.join(bundleDir, 'screenshots', 'start.png'));

  // 5) Parameter tweaks with CDP-primary latency measurement.
  const interactions = [];
  const tweakLatencies = [];
  for (let i = 1; i <= args.tweaks; i += 1) {
    const intent = i % 2 === 1
      ? {
          schemaVersion: 'ouroforge.preview-intent.v1',
          intentId: `audit-tweak-${i}`,
          sessionId,
          kind: 'parameterSet',
          entityId: 'player',
          path: 'components.input.moveSpeed',
          value: 60 + i,
        }
      : {
          schemaVersion: 'ouroforge.preview-intent.v1',
          intentId: `audit-tweak-${i}`,
          sessionId,
          kind: 'entityTransform',
          entityId: 'player',
          x: 30 + i,
        };
    const t0 = performance.now();
    const delta = await postIntent(serveUrl, intent);
    await waitFor(
      () => evaluateJson(cdp, `window.__OUROFORGE__.previewChannelState().deltasApplied >= ${i}`),
      5000, 5, `delta ${i} applied in page`
    );
    const latencyMs = performance.now() - t0;
    tweakLatencies.push(latencyMs);
    interactions.push({ kind: 'tweak', intentId: intent.intentId, status: delta.status, latencyMs });
  }
  await sampleWorld('after-tweaks');
  await capturePng(cdp, path.join(bundleDir, 'screenshots', 'after-tweaks.png'));

  // 6) Scene reloads.
  const reloadLatencies = [];
  for (let j = 1; j <= args.reloads; j += 1) {
    const intent = {
      schemaVersion: 'ouroforge.preview-intent.v1',
      intentId: `audit-reload-${j}`,
      sessionId,
      kind: 'sceneReload',
    };
    const t0 = performance.now();
    const delta = await postIntent(serveUrl, intent);
    await waitFor(
      () => evaluateJson(cdp, `window.__OUROFORGE__.previewChannelState().sceneReloads >= ${j} && window.__OUROFORGE__.getWorldState().runtimeEvents.filter(e => e.type === 'runtime.preview.scene_reloaded').length >= 1`),
      10000, 10, `scene reload ${j} completed in page`
    );
    const latencyMs = performance.now() - t0;
    reloadLatencies.push(latencyMs);
    interactions.push({ kind: 'reload', intentId: intent.intentId, status: delta.status, latencyMs });
    if (j < args.reloads) {
      // Re-apply one tweak so subsequent reloads have state to discard.
      const reseed = {
        schemaVersion: 'ouroforge.preview-intent.v1',
        intentId: `audit-reseed-${j}`,
        sessionId,
        kind: 'parameterSet',
        entityId: 'player',
        path: 'components.input.moveSpeed',
        value: 80 + j,
      };
      await postIntent(serveUrl, reseed);
      await waitFor(
        () => evaluateJson(cdp, `window.__OUROFORGE__.previewChannelState().deltasApplied >= ${args.tweaks + j}`),
        5000, 5, `reseed ${j} applied`
      );
      interactions.push({ kind: 'reseed', intentId: reseed.intentId, status: 'applied', latencyMs: null });
    }
  }
  await sampleWorld('final');
  await capturePng(cdp, path.join(bundleDir, 'screenshots', 'final.png'));

  // Secondary (in-page) instrumentation timestamps.
  const pageTimestamps = await evaluateJson(cdp, 'window.__OUROFORGE__.previewChannelState().timestamps');
  const pageDiagnostics = await evaluateJson(cdp, 'window.__OUROFORGE__.getDiagnostics()');
  const pageEvents = await evaluateJson(cdp, 'window.__OUROFORGE__.getEvents ? window.__OUROFORGE__.getEvents() : []');

  // 7) Transcript fetch + draft export + draft preflight preview (CLI legs).
  const transcriptPath = path.join(bundleDir, 'transcript.json');
  await runCli(args.bin, ['preview', 'transcript', '--url', serveUrl, '--output', transcriptPath], repoRoot);
  const draftPath = path.join(bundleDir, 'draft.json');
  await runCli(args.bin, [
    'preview', 'export-proposal',
    '--transcript', transcriptPath,
    '--draft-id', `${runId}-draft`,
    '--target-path', args.draftTargetPath,
    '--output', draftPath,
  ], repoRoot);
  // --transaction-output is single-operation only; multi-edit drafts emit the
  // full transaction previews on stdout, which we persist as the artifact.
  const preflightStdout = await runCli(args.bin, [
    'edit', 'draft-preview', draftPath,
    '--project', args.project,
  ], repoRoot);
  await writeFile(path.join(bundleDir, 'draft-preflight-transactions.json'), preflightStdout);

  // 8) Shutdown serve and collect its report.
  await runCli(args.bin, ['preview', 'stop', '--url', serveUrl], repoRoot);
  await new Promise(resolve => setTimeout(resolve, 250));
  cdp.close();

  // 9) Stats + verdict.
  const tweakStats = latencyStats(tweakLatencies);
  const reloadStats = latencyStats(reloadLatencies);
  const tweakWithinBudget = tweakLatencies.filter(ms => ms <= TWEAK_BUDGET_MS).length;
  const reloadWithinBudget = reloadLatencies.filter(ms => ms <= RELOAD_BUDGET_MS).length;
  const diagnosticsClean = (pageDiagnostics ?? []).length === 0;

  const latencyReport = {
    schemaVersion: `${SCHEMA_VERSION}.latency`,
    primary: 'cdp-harness-clock (POST /intent -> page applied-counter advance observed via Runtime.evaluate)',
    secondary: 'in-page performance.now receivedAt/appliedAt instrumentation',
    budgets: { tweakMs: TWEAK_BUDGET_MS, reloadMs: RELOAD_BUDGET_MS, mode: 'record-only (first cycle per #2517 Q2)' },
    tweaks: { ...tweakStats, withinBudget: tweakWithinBudget, samplesMs: tweakLatencies },
    reloads: { ...reloadStats, withinBudget: reloadWithinBudget, samplesMs: reloadLatencies },
    pageTimestamps,
  };

  await writeFile(path.join(bundleDir, 'latency-stats.json'), JSON.stringify(latencyReport, null, 2));
  await writeFile(path.join(bundleDir, 'console.jsonl'), consoleLines.join('\n') + '\n');
  await writeFile(path.join(bundleDir, 'world-samples.jsonl'), worldSamples.join('\n') + '\n');
  await writeFile(path.join(bundleDir, 'frame-stats.jsonl'), frameSamples.join('\n') + '\n');
  await writeFile(path.join(bundleDir, 'events.json'), JSON.stringify(pageEvents, null, 2));
  await writeFile(path.join(bundleDir, 'input-replay.json'), JSON.stringify({
    schemaVersion: `${SCHEMA_VERSION}.interactions`,
    note: 'Preview-loop interactions (validated intents), not keyboard replay.',
    interactions,
  }, null, 2));
  await writeFile(path.join(bundleDir, 'serve-report.json'), JSON.stringify({ raw: serveReport.trim() }, null, 2));
  await writeFile(path.join(bundleDir, 'draft-preflight-result.json'), preflightStdout);

  const manifest = {
    schemaVersion: SCHEMA_VERSION,
    runId,
    issue: 2521,
    scene: args.scene,
    pageUrl,
    serveSession: sessionId,
    artifacts: [
      'manifest.json', 'console.jsonl', 'frame-stats.jsonl', 'world-samples.jsonl',
      'events.json', 'input-replay.json', 'latency-stats.json', 'transcript.json',
      'draft.json', 'draft-preflight-transactions.json', 'draft-preflight-result.json',
      'serve-report.json', 'screenshots/start.png', 'screenshots/after-tweaks.png',
      'screenshots/final.png', 'verdict.md',
    ],
  };
  await writeFile(path.join(bundleDir, 'manifest.json'), JSON.stringify(manifest, null, 2));

  const verdict = [
    `# Preview Loop Audit Verdict (${runId})`,
    '',
    `- Scene: \`${args.scene}\``,
    `- Tweaks: ${args.tweaks} applied; latency p50 ${tweakStats.p50Ms?.toFixed(1)} ms / p95 ${tweakStats.p95Ms?.toFixed(1)} ms (budget ${TWEAK_BUDGET_MS} ms, record-only): ${tweakWithinBudget}/${args.tweaks} within budget.`,
    `- Scene reloads: ${args.reloads}; latency p50 ${reloadStats.p50Ms?.toFixed(1)} ms / p95 ${reloadStats.p95Ms?.toFixed(1)} ms (budget ${RELOAD_BUDGET_MS} ms, record-only): ${reloadWithinBudget}/${args.reloads} within budget.`,
    `- Runtime diagnostics: ${diagnosticsClean ? 'none recorded' : `${pageDiagnostics.length} recorded (see world-samples.jsonl)`}.`,
    '- Transcript fetched, draft exported, and draft passed `edit draft-preview` preflight (see draft-preflight-transactions.json).',
    '',
    '## Boundary and gaps',
    '',
    '- Latency budgets are record-only this cycle per the #2517 Q2 resolution; hard gating begins next cycle.',
    '- The review -> apply -> rerun-comparison leg beyond draft preflight is NOT exercised by this automated audit. It was product-observed under M130 #2392; the Era X re-exercise through the Studio surface is owned by M132.2/M132.3 (#2524/#2525). This is a recorded gap, not a pass.',
    '',
    `Verdict: preview-loop latency and export-handoff claims ${diagnosticsClean ? 'OBSERVED' : 'OBSERVED WITH DIAGNOSTICS (see gaps)'}; classification authority remains the issue closure comment.`,
  ].join('\n');
  await writeFile(path.join(bundleDir, 'verdict.md'), verdict);

  await rm(profileDir, { recursive: true, force: true });
  killChildren();

  console.log(JSON.stringify({
    status: 'completed',
    bundle: path.relative(repoRoot, bundleDir),
    tweakStats,
    reloadStats,
    diagnosticsClean,
  }, null, 2));
}

main().catch(error => {
  console.error(`preview-loop audit failed: ${error.message}`);
  killChildren();
  process.exit(1);
});
