#!/usr/bin/env node
import { createHash, randomBytes } from 'node:crypto';
import { spawn } from 'node:child_process';
import { mkdtemp, rm, mkdir, writeFile, readFile, stat } from 'node:fs/promises';
import { createWriteStream } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import net from 'node:net';

const SCHEMA_VERSION = 'live-observability-v1';
const REQUIRED = [
  'manifest.json',
  'console.jsonl',
  'frame-stats.jsonl',
  'world-samples.jsonl',
  'events.json',
  'input-replay.json',
  'screenshots/',
  'verdict.md',
];

function parseArgs(argv) {
  const args = { outRoot: 'runs/live-observability', waitMs: 750, retries: 1, runKind: 'runtime', validatorManifest: 'crates/ouroforge-observability/Cargo.toml', validate: true };
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg === '--url') args.url = argv[++i];
    else if (arg === '--run-id') args.runId = argv[++i];
    else if (arg === '--out-root') args.outRoot = argv[++i];
    else if (arg === '--chrome') args.chrome = argv[++i];
    else if (arg === '--wait-ms') args.waitMs = Number(argv[++i]);
    else if (arg === '--retries') args.retries = Number(argv[++i]);
    else if (arg === '--run-kind') args.runKind = argv[++i];
    else if (arg === '--validator-manifest') args.validatorManifest = argv[++i];
    else if (arg === '--skip-validation') args.validate = false;
    else if (arg === '--help' || arg === '-h') args.help = true;
    else throw new Error(`unknown argument: ${arg}`);
  }
  return args;
}

function usage() {
  return `usage: node tools/live-observability-runner/runner.mjs --url <local-url> [--run-id id] [--out-root runs/live-observability] [--chrome path] [--wait-ms 750] [--retries 1] [--validator-manifest crates/ouroforge-observability/Cargo.toml] [--skip-validation]\n`;
}

function assertLocalHttpUrl(value) {
  let url;
  try { url = new URL(value); } catch { throw new Error(`invalid URL: ${value}`); }
  if (url.protocol !== 'http:' || !['127.0.0.1', 'localhost'].includes(url.hostname)) {
    throw new Error(`target URL is not allowed; only http://127.0.0.1:<port>/... and http://localhost:<port>/... are permitted: ${value}`);
  }
  if (!url.port) throw new Error(`target URL must include an explicit local port: ${value}`);
  return url.toString();
}

function defaultChromeCandidates() {
  return [
    process.env.CHROME_PATH,
    '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
    '/Applications/Chromium.app/Contents/MacOS/Chromium',
    'google-chrome',
    'chromium',
    'chromium-browser',
  ].filter(Boolean);
}

async function findFreePort() {
  const server = net.createServer();
  await new Promise((resolve, reject) => server.listen(0, '127.0.0.1', resolve).once('error', reject));
  const { port } = server.address();
  await new Promise(resolve => server.close(resolve));
  return port;
}

async function launchChrome({ chrome, retries }) {
  const candidates = chrome ? [chrome] : defaultChromeCandidates();
  let lastError;
  for (let attempt = 1; attempt <= Math.max(1, retries + 1); attempt += 1) {
    for (const candidate of candidates) {
      const port = await findFreePort();
      const profile = await mkdtemp(path.join(tmpdir(), 'ouroforge-live-observability-chrome-'));
      const child = spawn(candidate, [
        `--remote-debugging-port=${port}`,
        '--remote-debugging-address=127.0.0.1',
        `--user-data-dir=${profile}`,
        '--headless=new',
        '--disable-gpu',
        '--no-first-run',
        '--no-default-browser-check',
        'about:blank',
      ], { stdio: ['ignore', 'ignore', 'pipe'] });
      child.stderr.on('data', () => {});
      try {
        const version = await waitForJsonVersion(port, 8000);
        return { child, port, profile, attempts: attempt, version };
      } catch (error) {
        lastError = error;
        child.kill('SIGTERM');
        await rm(profile, { recursive: true, force: true });
      }
    }
  }
  throw new Error(`failed to launch/connect Chrome: ${lastError?.message ?? 'unknown error'}`);
}

async function waitForJsonVersion(port, timeoutMs) {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    try {
      const response = await fetch(`http://127.0.0.1:${port}/json/version`);
      if (response.ok) return await response.json();
    } catch {}
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  throw new Error(`Chrome DevTools endpoint did not become ready on port ${port}`);
}

async function createPage(port) {
  const response = await fetch(`http://127.0.0.1:${port}/json/new?about:blank`, { method: 'PUT' });
  if (!response.ok) throw new Error(`failed to create CDP target: HTTP ${response.status}`);
  const target = await response.json();
  return target.webSocketDebuggerUrl;
}


class WebSocket {
  constructor(wsUrl) {
    this.url = new URL(wsUrl);
    this.listeners = new Map();
    this.buffer = Buffer.alloc(0);
    this.handshakeDone = false;
    this.socket = net.createConnection({ host: this.url.hostname, port: Number(this.url.port) }, () => this.handshake());
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
    const request = [
      `GET ${this.url.pathname}${this.url.search} HTTP/1.1`,
      `Host: ${this.url.host}`,
      'Upgrade: websocket',
      'Connection: Upgrade',
      `Sec-WebSocket-Key: ${key}`,
      'Sec-WebSocket-Version: 13',
      '',
      '',
    ].join('\r\n');
    this.socket.write(request);
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
    this.ws = new WebSocket(wsUrl);
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
    const payload = JSON.stringify({ id, method, params });
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.ws.send(payload);
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

async function sampleOuroforgeRuntime(cdp) {
  return await evaluateJson(cdp, `
    (async () => {
      const api = window.__OUROFORGE__;
      if (!api || typeof api !== 'object') {
        return {
          supported: false,
          available_keys: [],
          used_keys: [],
          diagnostics: [{ code: 'unsupported-target', message: 'window.__OUROFORGE__ is missing' }]
        };
      }
      const available = Object.keys(api).sort();
      const used = [];
      const diagnostics = [];
      function need(key) {
        if (typeof api[key] !== 'function') {
          diagnostics.push({ code: 'unsupported-target', message: 'window.__OUROFORGE__.' + key + ' is missing' });
          return false;
        }
        used.push(key);
        return true;
      }
      if (typeof api.whenReady === 'function') {
        used.push('whenReady');
        await api.whenReady();
      }
      if (!need('getWorldState') || !need('getFrameStats') || !need('getEvents')) {
        return { supported: false, available_keys: available, used_keys: used, diagnostics };
      }
      const world = api.getWorldState();
      const frameStats = api.getFrameStats();
      const events = api.getEvents();
      return {
        supported: true,
        available_keys: available,
        used_keys: used,
        diagnostics,
        sample: {
          tick: world?.tick ?? world?.runtimeState?.tick ?? null,
          scene_id: world?.sceneId ?? world?.runtimeState?.sceneId ?? null,
          player: world?.object ?? null,
          goal_flags: world?.componentModel?.goalFlags ?? {},
          recent_events: Array.isArray(events) ? events.slice(-10) : [],
          frame_stats: frameStats ?? null,
          runtime_diagnostics: world?.runtimeDiagnostics ?? [],
        }
      };
    })()
  `);
}

function nowIso() { return new Date().toISOString(); }
function jsonLine(value) { return `${JSON.stringify(value)}\n`; }
async function sha256(file) { return createHash('sha256').update(await readFile(file)).digest('hex'); }

async function buildInventory(bundleDir) {
  const entries = [];
  for (const required of REQUIRED) {
    const full = path.join(bundleDir, required.replace(/\/$/, ''));
    if (required === 'manifest.json') {
      entries.push({ path: required, kind: 'json', required: true });
    } else if (required.endsWith('/')) {
      await stat(full);
      entries.push({ path: required, kind: 'directory', required: true });
    } else {
      await stat(full);
      entries.push({ path: required, kind: kindFor(required), sha256: await sha256(full), required: true });
    }
  }
  const screenshotDir = path.join(bundleDir, 'screenshots');
  for (const name of ['start.png']) {
    const file = path.join(screenshotDir, name);
    try {
      await stat(file);
      entries.push({ path: `screenshots/${name}`, kind: 'png', sha256: await sha256(file), required: false });
    } catch {}
  }
  return entries;
}

function kindFor(file) {
  if (file.endsWith('.jsonl')) return 'jsonl';
  if (file.endsWith('.json')) return 'json';
  if (file.endsWith('.md')) return 'markdown';
  return 'other';
}


async function validateBundleWithRust(manifestPath, bundleDir) {
  const child = spawn('cargo', ['run', '--quiet', '--manifest-path', manifestPath, '--', 'validate', bundleDir], {
    stdio: ['ignore', 'pipe', 'pipe'],
    env: process.env,
  });
  let stdout = '';
  let stderr = '';
  child.stdout.on('data', chunk => { stdout += chunk; });
  child.stderr.on('data', chunk => { stderr += chunk; });
  const code = await new Promise((resolve, reject) => {
    child.on('error', reject);
    child.on('close', resolve);
  });
  if (code !== 0) {
    throw new Error(`Rust observability validator failed with exit ${code}: ${stderr || stdout}`);
  }
  return stdout.trim();
}

async function run() {
  const args = parseArgs(process.argv.slice(2));
  if (args.help || !args.url) {
    process.stderr.write(usage());
    return args.help ? 0 : 2;
  }
  if (!['runtime', 'studio', 'authoring'].includes(args.runKind)) throw new Error(`invalid --run-kind: ${args.runKind}`);
  const targetUrl = assertLocalHttpUrl(args.url);
  const runId = args.runId ?? `run-${new Date().toISOString().replace(/[:.]/g, '-')}`;
  const bundleDir = path.join(args.outRoot, runId);
  const screenshotDir = path.join(bundleDir, 'screenshots');
  await mkdir(screenshotDir, { recursive: true });

  let chrome;
  const consoleStream = createWriteStream(path.join(bundleDir, 'console.jsonl'));
  try {
    chrome = await launchChrome({ chrome: args.chrome, retries: args.retries });
    const wsUrl = await createPage(chrome.port);
    const cdp = new CdpClient(wsUrl);
    await cdp.open();
    cdp.on('Runtime.consoleAPICalled', params => {
      consoleStream.write(jsonLine({ schema_version: SCHEMA_VERSION, timestamp: nowIso(), level: params.type, text: params.args?.map(arg => arg.value ?? arg.description ?? '').join(' ') ?? '', source: 'console' }));
    });
    cdp.on('Runtime.exceptionThrown', params => {
      consoleStream.write(jsonLine({ schema_version: SCHEMA_VERSION, timestamp: nowIso(), level: 'error', text: params.exceptionDetails?.text ?? 'exception', source: 'exception' }));
    });
    await cdp.send('Runtime.enable');
    await cdp.send('Page.enable');
    await cdp.send('Performance.enable');
    const loaded = new Promise(resolve => cdp.on('Page.loadEventFired', resolve));
    await cdp.send('Page.navigate', { url: targetUrl });
    await Promise.race([loaded, new Promise(resolve => setTimeout(resolve, 5000))]);
    await new Promise(resolve => setTimeout(resolve, args.waitMs));
    const runtimeSample = await sampleOuroforgeRuntime(cdp);
    const screenshot = await cdp.send('Page.captureScreenshot', { format: 'png', captureBeyondViewport: false });
    await writeFile(path.join(screenshotDir, 'start.png'), Buffer.from(screenshot.data, 'base64'));
    const metrics = await cdp.send('Performance.getMetrics');
    const metricMap = Object.fromEntries((metrics.metrics ?? []).map(metric => [metric.name, metric.value]));
    const sampledFrameStats = runtimeSample?.sample?.frame_stats ?? {};
    await writeFile(path.join(bundleDir, 'frame-stats.jsonl'), jsonLine({ schema_version: SCHEMA_VERSION, timestamp: nowIso(), frame: sampledFrameStats.tick ?? 0, fps: null, delta_ms: sampledFrameStats.fixedDeltaMs ?? null, metrics: { Frames: metricMap.Frames ?? null, Timestamp: metricMap.Timestamp ?? null }, runtime_frame_stats: sampledFrameStats }));
    await writeFile(path.join(bundleDir, 'world-samples.jsonl'), jsonLine({ schema_version: SCHEMA_VERSION, timestamp: nowIso(), ...(runtimeSample?.sample ?? { tick: null, scene_id: null, goal_flags: {}, recent_events: [] }), supported: Boolean(runtimeSample?.supported), diagnostics: runtimeSample?.diagnostics ?? [], screenshot: 'screenshots/start.png' }));
    await writeFile(path.join(bundleDir, 'events.json'), JSON.stringify({ schema_version: SCHEMA_VERSION, events: [{ timestamp: nowIso(), type: 'page-load', target_url: targetUrl }, ...((runtimeSample?.sample?.recent_events ?? []).map(event => ({ timestamp: nowIso(), type: 'runtime-event-sample', event })))] }, null, 2) + '\n');
    await writeFile(path.join(bundleDir, 'input-replay.json'), JSON.stringify({ schema_version: SCHEMA_VERSION, steps: [] }, null, 2) + '\n');
    await writeFile(path.join(bundleDir, 'verdict.md'), `# Live observability verdict stub\n\nTarget: ${targetUrl}\n\nStatus: contract-pass / product-observed-pending\n\nArtifacts captured: console, frame stats, world sample placeholder, events, input replay, start screenshot.\n`);
    consoleStream.end();
    await new Promise(resolve => consoleStream.on('finish', resolve));
    await writeFile(path.join(bundleDir, 'manifest.json'), JSON.stringify({
      schema_version: SCHEMA_VERSION,
      run_id: runId,
      created_at: nowIso(),
      target_url: targetUrl,
      run_kind: args.runKind,
      tool_versions: { runner: 'live-observability-runner-v1', node: process.version, schema: SCHEMA_VERSION },
      browser: { cdp_port: chrome.port, browser: chrome.version?.Browser ?? null, protocol_version: chrome.version?.['Protocol-Version'] ?? null },
      retry_attempts: chrome.attempts,
      observability_api_keys_used: runtimeSample?.used_keys ?? [],
      observability_api_keys_available: runtimeSample?.available_keys ?? [],
      diagnostics: runtimeSample?.diagnostics ?? [],
      artifact_inventory: await buildInventory(bundleDir),
    }, null, 2) + '\n');
    if (args.validate) {
      const validation = await validateBundleWithRust(args.validatorManifest, bundleDir);
      process.stderr.write(`${validation}\n`);
    }
    cdp.close();
    process.stdout.write(`${bundleDir}\n`);
    return 0;
  } finally {
    consoleStream.end();
    if (chrome) {
      chrome.child.kill('SIGTERM');
      await rm(chrome.profile, { recursive: true, force: true });
    }
  }
}

run().then(code => process.exit(code)).catch(error => {
  console.error(error.message);
  process.exit(1);
});
