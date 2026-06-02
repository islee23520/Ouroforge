const OuroforgeCockpit = (() => {
  const EDITABLE_FIELDS = [
    ['sprite.color', 'color'],
    ['components.transform.x', 'number'],
    ['components.transform.y', 'number'],
    ['components.velocity.x', 'number'],
    ['components.velocity.y', 'number'],
    ['components.size.width', 'number'],
    ['components.size.height', 'number'],
    ['components.controllable', 'boolean'],
  ];

  const DEFAULT_SCENE_PATH = 'examples/game-runtime/scene.json';
  const DEFAULT_DASHBOARD_DATA_PATH = '../evidence-dashboard/dashboard-data.json';
  const READ_ONLY_FIELDS = [
    'schemaVersion',
    'id',
    'bounds',
    'sprite.asset',
    'components.collider',
    'components.animation',
    'components.audio',
    'tags',
    'metadata',
  ];

  function escapeText(value) {
    return String(value ?? '')
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#39;');
  }

  function cloneScene(scene) {
    return JSON.parse(JSON.stringify(scene));
  }

  function getValue(entity, path) {
    return path.split('.').reduce((value, part) => value && value[part], entity);
  }

  function coerceValue(raw, kind) {
    if (kind === 'number') {
      // Treat a blank/cleared input as invalid instead of silently coercing to 0.
      if (typeof raw === 'string' && raw.trim() === '') return NaN;
      return Number(raw);
    }
    if (kind === 'boolean') return raw === true || raw === 'true';
    return String(raw);
  }

  function validateEdit(path, value) {
    const field = EDITABLE_FIELDS.find(([candidate]) => candidate === path);
    if (!field) throw new Error(`Unsupported edit path: ${path}`);
    const kind = field[1];
    if (kind === 'number' && (!Number.isInteger(value) || (path.includes('size') && value <= 0))) {
      throw new Error(`Invalid numeric value for ${path}`);
    }
    if (kind === 'color' && !/^#[0-9a-fA-F]{6}$/.test(value)) {
      throw new Error('Color must be #RRGGBB');
    }
    return true;
  }

  function applyEdit(scene, entityId, path, rawValue) {
    const next = cloneScene(scene);
    const entity = next.entities.find((candidate) => candidate.id === entityId);
    if (!entity) throw new Error(`Entity not found: ${entityId}`);
    const [, kind] = EDITABLE_FIELDS.find(([candidate]) => candidate === path) || [];
    const value = coerceValue(rawValue, kind);
    validateEdit(path, value);
    const parts = path.split('.');
    const leaf = parts.pop();
    const target = parts.reduce((current, part) => current[part], entity);
    target[leaf] = value;
    return next;
  }

  function cliCommand(scenePath, entityId, path, value) {
    return `cargo run -p ouroforge-cli -- scene edit ${scenePath} --entity ${entityId} --path ${path} --value '${JSON.stringify(value)}'`;
  }

  function readOnlyFieldValue(entity, path) {
    if (path === 'schemaVersion' || path === 'id' || path === 'bounds') return 'scene-level read-only';
    return getValue(entity, path);
  }

  function renderReadOnlyFields(entity) {
    const rows = READ_ONLY_FIELDS.map((path) => {
      const value = readOnlyFieldValue(entity, path);
      return `<div><strong>${escapeText(path)}</strong><pre>${escapeText(JSON.stringify(value ?? null, null, 2))}</pre></div>`;
    }).join('');
    return `<section class="panel"><h3>Read-only / unsupported fields</h3><p class="hint">These fields are visible for context only. Persistence is limited to the supported Rust scene edit paths.</p><div class="field-grid">${rows}</div></section>`;
  }

  function qaCommand(seedPath = 'seeds/platformer.yaml', workers = 4) {
    return `cargo run -p ouroforge-cli -- run ${seedPath} --workers ${workers}`;
  }

  function dashboardExportCommand(output = 'examples/evidence-dashboard/dashboard-data.json') {
    return `cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output ${output}`;
  }

  function artifactHref(artifact, run) {
    const runDir = run?.summary?.run_dir || '';
    return `../../${runDir}/${artifact.path}`;
  }

  function latestRun(runs = []) {
    return [...runs].sort((left, right) => Number(right.summary.created_at_unix_ms || 0) - Number(left.summary.created_at_unix_ms || 0))[0] || null;
  }

  function renderTree(scene, selectedId) {
    return scene.entities.map((entity) => `<button class="tree-button ${entity.id === selectedId ? 'active' : ''}" data-entity-id="${escapeText(entity.id)}">${escapeText(entity.id)}<br><small>${entity.components.controllable ? 'controllable' : 'static'}</small></button>`).join('');
  }

  function renderInspector(scene, entityId, scenePath = DEFAULT_SCENE_PATH, editError = null) {
    const entity = scene.entities.find((candidate) => candidate.id === entityId);
    if (!entity) return '<div class="empty">Select an entity to inspect supported properties.</div>';
    const fields = EDITABLE_FIELDS.map(([path, kind]) => {
      const value = getValue(entity, path);
      const input = kind === 'boolean'
        ? `<select data-edit-path="${escapeText(path)}"><option value="true" ${value ? 'selected' : ''}>true</option><option value="false" ${!value ? 'selected' : ''}>false</option></select>`
        : `<input data-edit-path="${escapeText(path)}" type="${kind === 'number' ? 'number' : 'text'}" value="${escapeText(value)}" />`;
      return `<label>${escapeText(path)}${input}</label>`;
    }).join('');
    const error = editError ? `<div class="error" id="edit-error">${escapeText(editError)}</div>` : '<div class="hint" id="edit-error">No validation errors.</div>';
    return `<div class="inspector"><div class="panel"><h2>${escapeText(entity.id)}</h2><p class="hint">Supported fields update browser memory only. Use the generated Rust command to persist through validation.</p>${error}<div class="field-grid">${fields}</div></div><div class="panel"><h3>Current component JSON</h3><pre>${escapeText(JSON.stringify(entity, null, 2))}</pre></div><div class="panel"><h3>Validated write path</h3><pre id="edit-command">${escapeText(cliCommand(scenePath, entity.id, 'components.transform.x', entity.components.transform.x))}</pre></div>${renderReadOnlyFields(entity)}</div>`;
  }

  function renderPreview(scenePath = '../game-runtime/index.html') {
    return `<section class="panel"><h2>Live browser preview</h2><iframe id="runtime-preview" class="preview" title="Game runtime preview" src="${scenePath}"></iframe></section>`;
  }

  function previewWindow(source) {
    if (!source) return null;
    return source.contentWindow || source;
  }

  function unavailablePreviewProbe(reason) {
    return { ok: false, reason, error: reason };
  }

  function resolvePreviewProbe(source, requiredMethods = []) {
    const runtimeWindow = previewWindow(source);
    if (!runtimeWindow) return unavailablePreviewProbe('runtime preview window is unavailable');
    const probe = runtimeWindow.__OUROFORGE__;
    if (!probe) return unavailablePreviewProbe('window.__OUROFORGE__ probe is unavailable');
    const missing = requiredMethods.filter((method) => typeof probe[method] !== 'function');
    if (missing.length) return unavailablePreviewProbe(`window.__OUROFORGE__ is missing method(s): ${missing.join(', ')}`);
    return { ok: true, probe, window: runtimeWindow };
  }

  function readPreviewProbe(source) {
    const resolved = resolvePreviewProbe(source, ['getWorldState', 'getFrameStats']);
    if (!resolved.ok) return resolved;
    try {
      return {
        ok: true,
        worldState: resolved.probe.getWorldState(),
        frameStats: resolved.probe.getFrameStats(),
      };
    } catch (error) {
      return unavailablePreviewProbe(`runtime probe read failed: ${error.message}`);
    }
  }

  function callPreviewProbe(source, method, ...args) {
    const resolved = resolvePreviewProbe(source, [method, 'getWorldState', 'getFrameStats']);
    if (!resolved.ok) return resolved;
    try {
      const result = resolved.probe[method](...args);
      const state = readPreviewProbe(source);
      if (!state.ok) return state;
      return { ok: true, method, result, worldState: state.worldState, frameStats: state.frameStats };
    } catch (error) {
      return unavailablePreviewProbe(`runtime probe ${method} failed: ${error.message}`);
    }
  }

  function reloadPreview(source) {
    const runtimeWindow = previewWindow(source);
    if (!runtimeWindow) return unavailablePreviewProbe('runtime preview window is unavailable');
    if (!runtimeWindow.location || typeof runtimeWindow.location.reload !== 'function') {
      return unavailablePreviewProbe('runtime preview reload is unavailable');
    }
    runtimeWindow.location.reload();
    return { ok: true, method: 'reset', message: 'runtime preview reload requested' };
  }

  function renderPreviewControls(state = null) {
    const status = state
      ? state.ok ? '<span class="status-ok">probe ready</span>' : `<span class="status-error">${escapeText(state.error || 'probe unavailable')}</span>`
      : '<span class="status-idle">waiting for probe action</span>';
    const frameStats = state?.frameStats
      ? `<pre>${escapeText(JSON.stringify(state.frameStats, null, 2))}</pre>`
      : '<p class="empty compact">No frame stats loaded yet.</p>';
    const worldState = state?.worldState
      ? `<pre>${escapeText(JSON.stringify(state.worldState, null, 2))}</pre>`
      : '<p class="empty compact">No world state loaded yet.</p>';
    return `<div class="preview-controls">
      <p class="hint">Ephemeral local controls. Uses <code>window.__OUROFORGE__</code>; does not write scene files or persist preview state.</p>
      <div class="control-row">
        <button class="secondary" type="button" data-preview-action="pause">Pause</button>
        <button class="secondary" type="button" data-preview-action="resume">Resume</button>
        <button class="secondary" type="button" data-preview-action="step">Step 1 frame</button>
        <button class="secondary" type="button" data-preview-action="reset">Reset / reload</button>
      </div>
      <div class="field-grid">
        <div><strong>Probe status</strong><br>${status}</div>
        <div><strong>Current tick</strong><br>${escapeText(state?.frameStats?.tick ?? state?.worldState?.tick ?? 'unknown')}</div>
      </div>
      <h3>Frame stats</h3>${frameStats}
      <h3>World state</h3>${worldState}
    </div>`;
  }

  function renderQaPanel() {
    return `<section class="panel"><h2>Run QA</h2><p class="hint">Run the evidence-native QA command, then export dashboard data to refresh evidence and journal panes.</p><button id="run-qa-button" class="primary" type="button">Show QA command</button><pre id="qa-command">${qaCommand()}</pre><pre>${dashboardExportCommand()}</pre></section>`;
  }

  function renderEvidencePane(run) {
    if (!run) {
      return '<section class="panel"><h2>Evidence + Journal</h2><p class="empty">No dashboard-data.json run is loaded yet. Run QA and export dashboard data to populate this pane.</p></section>';
    }
    const screenshots = (run.screenshots || []).slice(0, 4).map((artifact) => `<a href="${escapeText(artifactHref(artifact, run))}" target="_blank" rel="noreferrer">${escapeText(artifact.id)}</a>`).join('<br>') || 'No screenshots recorded.';
    return `<section class="panel"><h2>Evidence + Journal</h2><div class="field-grid"><div><strong>Run</strong><br>${escapeText(run.summary.id)}</div><div><strong>Verdict</strong><br>${escapeText(run.summary.verdict_status)}</div><div><strong>Evidence</strong><br>${run.evidence.length}</div><div><strong>Mutations</strong><br>${run.mutations.length}</div></div><h3>Screenshots</h3><p>${screenshots}</p><h3>Journal</h3><pre>${escapeText(run.journal || 'No journal loaded.')}</pre></section>`;
  }

  function renderIntegration(run, previewState = null) {
    return `${renderPreview()}${renderPreviewControls(previewState)}${renderQaPanel()}${renderEvidencePane(run)}`;
  }

  async function loadDashboardData(path = DEFAULT_DASHBOARD_DATA_PATH) {
    const response = await fetch(path, { cache: 'no-store' });
    if (!response.ok) throw new Error(`failed to load dashboard data: ${response.status}`);
    return response.json();
  }

  async function init() {
    const treeEl = document.getElementById('scene-tree');
    const inspectorEl = document.getElementById('inspector');
    const integrationEl = document.getElementById('integration');
    let scene = await fetch('../game-runtime/scene.json').then((response) => response.json());
    let selectedId = scene.entities[0]?.id;
    let latest = null;
    let previewState = null;
    let editError = null;
    try {
      const dashboardData = await loadDashboardData();
      latest = latestRun(dashboardData.runs || []);
    } catch (_) {
      latest = null;
    }
    const paint = () => {
      treeEl.innerHTML = renderTree(scene, selectedId);
      inspectorEl.innerHTML = renderInspector(scene, selectedId, DEFAULT_SCENE_PATH, editError);
      integrationEl.innerHTML = renderIntegration(latest, previewState);
      treeEl.querySelectorAll('[data-entity-id]').forEach((button) => button.addEventListener('click', () => { selectedId = button.dataset.entityId; paint(); }));
      inspectorEl.querySelectorAll('[data-edit-path]').forEach((input) => input.addEventListener('change', () => {
        const path = input.dataset.editPath;
        try {
          scene = applyEdit(scene, selectedId, path, input.value);
          editError = null;
          const entity = scene.entities.find((candidate) => candidate.id === selectedId);
          document.getElementById('edit-command').textContent = cliCommand(DEFAULT_SCENE_PATH, selectedId, path, getValue(entity, path));
          document.getElementById('edit-error').textContent = 'No validation errors.';
          document.getElementById('edit-error').className = 'hint';
        } catch (error) {
          editError = error.message;
          paint();
        }
      }));
      integrationEl.querySelectorAll('[data-preview-action]').forEach((button) => button.addEventListener('click', () => {
        const previewFrame = document.getElementById('runtime-preview');
        const action = button.dataset.previewAction;
        if (action === 'reset') {
          previewState = reloadPreview(previewFrame);
        } else if (action === 'step') {
          previewState = callPreviewProbe(previewFrame, 'step', 1);
        } else {
          previewState = callPreviewProbe(previewFrame, action);
        }
        paint();
      }));
      document.getElementById('run-qa-button')?.addEventListener('click', () => {
        document.getElementById('qa-command').textContent = `${qaCommand()}\n${dashboardExportCommand()}`;
      });
    };
    paint();
  }

  return { EDITABLE_FIELDS, READ_ONLY_FIELDS, applyEdit, artifactHref, callPreviewProbe, cliCommand, dashboardExportCommand, escapeText, getValue, init, latestRun, loadDashboardData, previewWindow, qaCommand, readPreviewProbe, reloadPreview, renderEvidencePane, renderInspector, renderIntegration, renderPreview, renderPreviewControls, renderQaPanel, renderReadOnlyFields, renderTree, resolvePreviewProbe, validateEdit };
})();

if (typeof window !== 'undefined') {
  window.OuroforgeCockpit = OuroforgeCockpit;
  window.addEventListener('DOMContentLoaded', () => OuroforgeCockpit.init());
}

if (typeof module !== 'undefined') {
  module.exports = OuroforgeCockpit;
}
