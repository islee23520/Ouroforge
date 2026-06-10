#!/usr/bin/env node
import { createHash } from 'node:crypto';
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
        await waitForJsonVersion(port, 8000);
        return { child, port, profile, attempts: attempt };
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

function nowIso() { return new Date().toISOString(); }
function jsonLine(value) { return `${JSON.stringify(value)}\n`; }
async function sha256(file) { return createHash('sha256').update(await readFile(file)).digest('hex'); }

async function buildInventory(bundleDir) {
  const entries = [];
  for (const required of REQUIRED) {
    const full = path.join(bundleDir, required.replace(/\/$/, ''));
    if (required.endsWith('/')) {
      await stat(full);
      entries.push({ path: required, kind: 'directory', required: true });
    } else {
      await stat(full);
      entries.push({ path: required, kind: kindFor(required), sha256: required === 'manifest.json' ? undefined : await sha256(full), required: true });
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
    const screenshot = await cdp.send('Page.captureScreenshot', { format: 'png', captureBeyondViewport: false });
    await writeFile(path.join(screenshotDir, 'start.png'), Buffer.from(screenshot.data, 'base64'));
    const metrics = await cdp.send('Performance.getMetrics');
    const metricMap = Object.fromEntries((metrics.metrics ?? []).map(metric => [metric.name, metric.value]));
    await writeFile(path.join(bundleDir, 'frame-stats.jsonl'), jsonLine({ schema_version: SCHEMA_VERSION, timestamp: nowIso(), frame: 0, fps: null, delta_ms: null, metrics: { Frames: metricMap.Frames ?? null, Timestamp: metricMap.Timestamp ?? null } }));
    await writeFile(path.join(bundleDir, 'world-samples.jsonl'), jsonLine({ schema_version: SCHEMA_VERSION, timestamp: nowIso(), tick: null, scene_id: null, goal_flags: {}, recent_events: [], screenshot: 'screenshots/start.png' }));
    await writeFile(path.join(bundleDir, 'events.json'), JSON.stringify({ schema_version: SCHEMA_VERSION, events: [{ timestamp: nowIso(), type: 'page-load', target_url: targetUrl }] }, null, 2) + '\n');
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
      browser: { cdp_port: chrome.port },
      retry_attempts: chrome.attempts,
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
