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

  let lastEditCommand = null;

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

  function transactionCommand(scenePath, entityId, path, value, outputPath = 'runs/manual/transactions/scene-edit.json') {
    return `${cliCommand(scenePath, entityId, path, value)} --transaction-output ${outputPath}`;
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

  function projectRunCommand(seedPath = 'seeds/platformer.yaml', projectPath = 'ouroforge.project.json', workers = 4, scenarioPackId = null) {
    const scenarioPack = scenarioPackId ? ` --scenario-pack ${scenarioPackId}` : '';
    return `cargo run -p ouroforge-cli -- run ${seedPath} --project ${projectPath} --workers ${workers}${scenarioPack}`;
  }

  function compareRunsCommand(beforeRun = 'runs/before', afterRun = 'runs/after', outputDir = `${afterRun}/comparisons`) {
    return `cargo run -p ouroforge-cli -- compare ${beforeRun} ${afterRun} --output-dir ${outputDir}`;
  }

  function projectValidateCommand(projectPath = 'ouroforge.project.json') {
    return `cargo run -p ouroforge-cli -- project validate ${projectPath}`;
  }

  function seedValidateCommand(seedPath = 'seeds/platformer.yaml') {
    return `cargo run -p ouroforge-cli -- seed validate ${seedPath}`;
  }

  function qaTransactionCommand(seedPath = 'seeds/platformer.yaml', transactionPath = 'runs/manual/transactions/scene-edit.json', workers = 4) {
    return `cargo run -p ouroforge-cli -- run ${seedPath} --workers ${workers} --transaction ${transactionPath}`;
  }

  function dashboardExportCommand(output = 'examples/evidence-dashboard/dashboard-data.json') {
    return `cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output ${output}`;
  }

  function sceneMutationApplyCommand(runDir = 'runs/run-1', operationPath = 'mutation/scene-operation.json', transactionPath = 'mutation/scene-transaction.json', projectPath = null, decisionId = null) {
    const projectFlag = projectPath ? ` --project ${projectPath}` : '';
    const decisionFlag = decisionId ? ` --decision ${decisionId}` : '';
    return `cargo run -p ouroforge-cli -- mutation apply-scene ${runDir}${projectFlag} --operation ${operationPath}${decisionFlag} --transaction-output ${transactionPath}`;
  }

  function sceneValidateCommand(scenePath = DEFAULT_SCENE_PATH) {
    return `cargo run -p ouroforge-cli -- scene validate ${scenePath}`;
  }

  function sceneReloadValidateCommand(scenePath = DEFAULT_SCENE_PATH) {
    return `cargo run -p ouroforge-cli -- scene reload-validate ${scenePath}`;
  }

  function runtimeReloadPayloadCommand(scenePath = DEFAULT_SCENE_PATH) {
    return `# display-only payload shape for window.__OUROFORGE__.reload after Rust validation\n${sceneReloadValidateCommand(scenePath)}`;
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
    return `<div class="inspector"><div class="panel"><h2>${escapeText(entity.id)}</h2><p class="hint">Supported fields update browser memory only. Use the generated Rust command to persist through validation.</p>${error}<div class="field-grid">${fields}</div></div><div class="panel"><h3>Current component JSON</h3><pre>${escapeText(JSON.stringify(entity, null, 2))}</pre></div><div class="panel"><h3>Validated write path</h3><pre id="edit-command">${escapeText(lastEditCommand || cliCommand(scenePath, entity.id, 'components.transform.x', entity.components.transform.x))}</pre></div>${renderReadOnlyFields(entity)}</div>`;
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

  function renderCommandGenerationPanel(scenePath = DEFAULT_SCENE_PATH) {
    return `<section class="panel"><h2>Validation command generation</h2>
      <p class="hint">Display-only. Copy these Rust CLI commands into a terminal when you want validation-gated persistence; the browser never executes commands or writes files. Transaction output is a Rust CLI artifact, not browser persistence.</p>
      <div class="command-list">
        <code>${escapeText(sceneValidateCommand(scenePath))}</code>
        <code>${escapeText(sceneReloadValidateCommand(scenePath))}</code>
        <code>${escapeText(cliCommand(scenePath, 'player', 'components.transform.x', 48))}</code>
        <code>${escapeText(transactionCommand(scenePath, 'player', 'components.transform.x', 48))}</code>
        <code>${escapeText(runtimeReloadPayloadCommand(scenePath))}</code>
      </div>
    </section>`;
  }

  function renderQaPanel() {
    return `<section class="panel"><h2>Run QA</h2><p class="hint">Run the evidence-native QA command, then export dashboard data to refresh evidence and journal panes. Commands are display-only and are not executed by the browser.</p><button id="run-qa-button" class="primary" type="button">Show QA command</button><pre id="qa-command">${qaCommand()}</pre><pre>${dashboardExportCommand()}</pre></section>${renderCommandGenerationPanel()}`;
  }

  function renderRefLinks(refs = [], run) {
    if (!Array.isArray(refs) || !refs.length) return '<p class="empty compact">No evidence refs recorded.</p>';
    return `<div class="link-list">${refs.map((ref) => {
      const href = String(ref).startsWith('runs/') ? `../../${ref}` : `../../${run?.summary?.run_dir || ''}/${ref}`;
      return `<a href="${escapeText(href)}" target="_blank" rel="noreferrer">${escapeText(ref)}</a>`;
    }).join('')}</div>`;
  }

  function surfaceState(present, label = 'available') {
    return present ? `<span class="status-ok">${escapeText(label)}</span>` : '<span class="status-idle">gap / unavailable</span>';
  }

  function studioSurfaceSummary(run) {
    const hasRun = Boolean(run);
    const project = projectContext(run);
    return [
      { id: 'project-workspace', label: 'Project workspace', present: Boolean(project), detail: project?.id || 'project metadata unavailable' },
      { id: 'project-run', label: 'Project run summary', present: Boolean(project && run?.summary), detail: run?.summary?.id || 'project-bound run unavailable' },
      { id: 'run-browser', label: 'Run/evidence browser', present: hasRun && Array.isArray(run.evidence), detail: hasRun ? `${run.evidence.length} evidence artifact(s)` : 'dashboard data not loaded' },
      { id: 'journal-viewer', label: 'Journal viewer', present: Boolean(run?.journal_view?.exists || run?.journal), detail: run?.journal_view?.summary || 'journal artifact unavailable' },
      { id: 'mutation-review', label: 'Mutation review state', present: Boolean(run?.mutation_lifecycle), detail: run?.mutation_lifecycle?.terminal_state || 'no lifecycle read model' },
      { id: 'regression-matrix', label: 'Regression run matrix', present: Boolean(run?.regression_matrix?.projects?.length), detail: run?.regression_matrix ? `${(run.regression_matrix.projects || []).length} project(s)` : 'matrix export unavailable' },
      { id: 'replay-controls', label: 'Replay controls', present: Boolean(run?.replay?.present), detail: `${(run?.replay?.sequences || []).length} sequence(s)` },
      { id: 'live-preview', label: 'Live preview controls', present: true, detail: 'ephemeral probe controls' },
      { id: 'scene-editing', label: 'Scene editing commands', present: true, detail: 'Rust-validated command generation' },
      { id: 'authoring-provenance', label: 'Authoring provenance', present: Boolean(run?.transaction_provenance), detail: run?.transaction_provenance?.transactionId || 'no transaction-bound run loaded' },
      { id: 'engine-expansion', label: 'Engine Expansion state', present: Boolean(run?.engine_summaries?.present), detail: run?.engine_summaries?.source_world_state || 'world-state summary unavailable' },
      { id: 'run-comparison', label: 'Run comparison', present: Boolean(run?.comparison?.present), detail: `${(run?.comparison?.artifacts || []).length} comparison artifact(s)` },
    ];
  }

  function renderStudioNavigation(run) {
    const items = studioSurfaceSummary(run);
    return `<nav class="studio-nav" aria-label="Studio v2 surfaces">
      <h2>Studio v2 demo surfaces</h2>
      <p class="hint">Static local composition only. Browser UI inspects artifacts and generates Rust commands; it does not write files directly or claim production editor maturity.</p>
      <div class="surface-grid">${items.map((item) => `<a class="surface-card" href="#${escapeText(item.id)}">
        <strong>${escapeText(item.label)}</strong><br>${surfaceState(item.present)}<br><small>${escapeText(item.detail)}</small>
      </a>`).join('')}</div>
    </nav>`;
  }

  function summaryValue(summary, section, key, fallback = 'unknown') {
    const value = summary && summary[section] && summary[section][key];
    if (value === null || value === undefined || value === '') return fallback;
    return typeof value === 'object' ? JSON.stringify(value) : value;
  }

  function renderEngineExpansionSurface(run) {
    const summary = run?.engine_summaries;
    if (!summary?.present) {
      return `<section id="engine-expansion" class="panel"><h2>Engine Expansion state</h2><p class="empty">${escapeText(summary?.empty_state || 'No Engine Expansion world-state summary is available for this run.')}</p></section>`;
    }
    const cards = [
      ['Scene', `${summaryValue(summary, 'scene', 'sceneId')} · ${summaryValue(summary, 'scene', 'entityCount', 0)} entit(ies) · tick ${summaryValue(summary, 'scene', 'tick')}`],
      ['Renderer / camera', `v${summaryValue(summary, 'renderer', 'version')} · ${summaryValue(summary, 'renderer', 'renderedEntities', 0)} rendered · camera ${summaryValue(summary, 'renderer', 'camera')}`],
      ['Tilemaps', `${summaryValue(summary, 'tilemaps', 'tilemapCount', 0)} tilemap(s), ${summaryValue(summary, 'tilemaps', 'layerCount', 0)} layer(s)`],
      ['Assets', `${summaryValue(summary, 'assets', 'manifestId')} · ${summaryValue(summary, 'assets', 'assetCount', 0)} loaded/ref(s)`],
      ['Animation', `${summaryValue(summary, 'animation', 'animatedEntityCount', 0)} animated entit(ies)`],
      ['Audio', `${summaryValue(summary, 'audio', 'audioEntityCount', 0)} audio entit(ies), ${summaryValue(summary, 'audio', 'audioEventCount', 0)} event(s)`],
      ['Physics/contact', `${summaryValue(summary, 'physics', 'colliderEntityCount', 0)} collider entit(ies), ${summaryValue(summary, 'physics', 'collisionEventCount', 0)} event(s)`],
      ['Reload', `${summaryValue(summary, 'reload', 'reloadCount', 0)} reload(s), last ${summaryValue(summary, 'reload', 'lastStatus')}`],
      ['Composition', `${summaryValue(summary, 'composition', 'entityCount', 0)} composed entit(ies), ${summaryValue(summary, 'composition', 'parentedEntityCount', 0)} parented`],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    return `<section id="engine-expansion" class="panel"><h2>Engine Expansion state</h2>
      <p class="hint">Preview-only read model from exported evidence. The cockpit does not own scene state or persist edits; use generated Rust commands for validation-gated changes.</p>
      <div class="field-grid">${cards}</div>
      <p class="hint">Source world-state: ${escapeText(summary.source_world_state || 'unknown')}</p>
    </section>`;
  }

  function projectContext(run) {
    const project = run?.project || run?.summary?.project || null;
    return project && typeof project === 'object' && !Array.isArray(project) ? project : null;
  }

  function renderProjectWorkspaceSurface(run) {
    if (!run) {
      return '<section id="project-workspace" class="panel"><h2>Project workspace</h2><p class="empty">No dashboard-data.json run is loaded yet. Export a project-bound run to inspect manifest, scene, seed, and scenario pack context.</p></section>';
    }
    const project = projectContext(run);
    if (!project) {
      const malformed = run.project && typeof run.project !== 'object'
        ? `<p class="error">Malformed project metadata was ignored: ${escapeText(run.project)}</p>`
        : '';
      return `<section id="project-workspace" class="panel"><h2>Project workspace</h2><p class="empty">No project workspace metadata is recorded for this run. Use <code>ouroforge-cli run &lt;seed&gt; --project &lt;manifest&gt;</code> to bind project context.</p>${malformed}</section>`;
    }
    const scenes = Array.isArray(project.scenes) ? project.scenes : [];
    const sceneRows = scenes.length
      ? scenes.map((scene) => `<li><strong>${escapeText(scene.id || 'scene')}</strong> · ${escapeText(scene.path || 'unknown path')} · ${escapeText(scene.hash?.algorithm || '')}:${escapeText(scene.hash?.value || 'unknown hash')}</li>`).join('')
      : '<li>No scene sources are listed in project metadata.</li>';
    const pack = project.scenarioPack || project.scenario_pack || null;
    const scenarioIds = Array.isArray(pack?.scenarioIds) ? pack.scenarioIds : Array.isArray(pack?.scenario_ids) ? pack.scenario_ids : [];
    const scenarioPack = pack
      ? `<div><strong>Scenario pack</strong><br>${escapeText(pack.id || 'unknown')} · ${escapeText(pack.path || 'unknown path')}<br><small>${escapeText(scenarioIds.join(', ') || 'no scenario ids recorded')}</small></div>`
      : '<div><strong>Scenario pack</strong><br><span class="status-idle">not bound</span><br><small>No project scenario pack context is recorded.</small></div>';
    const manifestPath = project.manifestPath || 'ouroforge.project.json';
    const seedPath = project.seedPath || 'seeds/platformer.yaml';
    return `<section id="project-workspace" class="panel"><h2>Project workspace</h2>
      <p class="hint">Read-only project context from Rust-exported dashboard data. The cockpit displays manifest/seed/scenario information and copyable commands only; it does not validate, write, or execute anything from the browser.</p>
      <div class="field-grid">
        <div><strong>Project</strong><br>${escapeText(project.id || 'unknown')} · ${escapeText(project.name || 'unknown')}</div>
        <div><strong>Project root</strong><br>${escapeText(project.projectRoot || 'unknown')}</div>
        <div><strong>Manifest</strong><br>${escapeText(manifestPath)}</div>
        <div><strong>Manifest hash</strong><br>${escapeText(project.manifestHash?.algorithm || '')}:${escapeText(project.manifestHash?.value || 'unknown')}</div>
        <div><strong>Seed</strong><br>${escapeText(seedPath)}</div>
        ${scenarioPack}
      </div>
      <h3>Scene sources</h3><ul>${sceneRows}</ul>
      <h3>Display-only project commands</h3>
      <div class="command-list">
        <code>${escapeText(projectValidateCommand(manifestPath))}</code>
        <code>${escapeText(seedValidateCommand(seedPath))}</code>
        <code>${escapeText(dashboardExportCommand())}</code>
      </div>
    </section>`;
  }


  function runCommandContext(run) {
    const context = run?.command_context || run?.summary?.command_context || run?.run?.run_command_context || null;
    return context && typeof context === 'object' ? context : null;
  }

  function renderRunCommandContext(run) {
    const context = runCommandContext(run);
    if (!context) {
      return '<div class="command-context"><h3>Reproducible command context</h3><p class="empty compact">No run command context is recorded for this legacy export.</p></div>';
    }
    const hints = Array.isArray(context.environmentHints) ? context.environmentHints : [];
    return `<div class="command-context"><h3>Reproducible command context</h3>
      <p class="hint">Display-only. Copy manually if needed; the cockpit does not execute commands, start bridges, or rerun QA.</p>
      <code>${escapeText(context.command || 'No command string recorded.')}</code>
      <div class="field-grid">
        <div><strong>Seed</strong><br>${escapeText(context.seedPath || 'unknown')}</div>
        <div><strong>Workers</strong><br>${escapeText(context.workers ?? 'unknown')}</div>
        <div><strong>Runs root</strong><br>${escapeText(context.runsRoot || 'runs')}</div>
        <div><strong>Scenario pack</strong><br>${escapeText(context.scenarioPackId || 'none')}</div>
        <div><strong>Runtime</strong><br>${escapeText(context.runtimeTarget || 'unknown')}</div>
        <div><strong>Browser boundary</strong><br>${escapeText(context.browserBoundary || 'unknown')} / ${escapeText(context.cdpTransport || 'unknown')}</div>
      </div>
      ${hints.length ? `<ul>${hints.map((hint) => `<li>${escapeText(hint)}</li>`).join('')}</ul>` : ''}
    </div>`;
  }

  function renderProjectRunSurface(run) {
    if (!run) {
      return '<section id="project-run" class="panel"><h2>Project run summary</h2><p class="empty">No dashboard-data.json run is loaded yet. Run project QA and export dashboard data to inspect latest project-bound run state.</p></section>';
    }
    const project = projectContext(run);
    if (!project) {
      return '<section id="project-run" class="panel"><h2>Project run summary</h2><p class="empty">No project-bound run metadata is available for this run.</p></section>';
    }
    const summary = run.summary || {};
    const pack = project.scenarioPack || project.scenario_pack || null;
    const manifestPath = project.manifestPath || 'ouroforge.project.json';
    const seedPath = project.seedPath || 'seeds/platformer.yaml';
    const runDir = summary.run_dir || (summary.id ? `runs/${summary.id}` : 'runs/run-latest');
    return `<section id="project-run" class="panel"><h2>Project run summary</h2>
      <p class="hint">Latest project-bound run context from exported dashboard data. Generated run state is local evidence and should remain untracked.</p>
      <div class="field-grid">
        <div><strong>Run</strong><br>${escapeText(summary.id || 'unknown')}</div>
        <div><strong>Run dir</strong><br>${escapeText(runDir)}</div>
        <div><strong>Verdict</strong><br>${escapeText(summary.verdict_status || 'unknown')}</div>
        <div><strong>Scenario</strong><br>${escapeText(summary.scenario_status || 'unknown')}</div>
        <div><strong>Evidence</strong><br>${escapeText(summary.evidence_count ?? (run.evidence || []).length)}</div>
        <div><strong>Generated-state status</strong><br>local/untracked expected</div>
      </div>
      ${renderRunCommandContext(run)}
      <h3>Display-only project run commands</h3>
      <div class="command-list">
        <code>${escapeText(projectRunCommand(seedPath, manifestPath, 4, pack?.id || null))}</code>
        <code>${escapeText(dashboardExportCommand())}</code>
      </div>
    </section>`;
  }



  function fidelityStatusClass(status) {
    const token = String(status || 'unknown').toLowerCase();
    return ['present', 'partial', 'missing', 'malformed', 'legacy', 'unknown'].includes(token) ? token : 'unknown';
  }

  function evidenceFidelity(run) {
    const fidelity = run?.evidence_fidelity || run?.summary?.evidence_fidelity || null;
    return fidelity && typeof fidelity === 'object' ? fidelity : null;
  }

  function renderFidelityStatusCard(status, fallbackId, fallbackLabel, run) {
    const item = status && typeof status === 'object' ? status : null;
    if (!item) {
      return `<article class="fidelity-card warning"><h3>${escapeText(fallbackLabel)}</h3><p class="empty compact">No ${escapeText(fallbackLabel)} read-model data is available.</p></article>`;
    }
    const warnings = Array.isArray(item.warnings) ? item.warnings : [];
    const refs = Array.isArray(item.evidence_refs) ? item.evidence_refs : [];
    const state = item.status || 'unknown';
    const stateClass = fidelityStatusClass(state);
    return `<article class="fidelity-card ${stateClass}"><h3>${escapeText(item.label || fallbackLabel)}</h3>
      <p><strong>Status:</strong> ${escapeText(state)}</p>
      <p>${escapeText(item.summary || 'No summary recorded.')}</p>
      <div class="field-grid compact">
        <div><strong>Observed</strong><br>${escapeText(item.observed_count ?? 0)}</div>
        <div><strong>Missing</strong><br>${escapeText(item.missing_count ?? 0)}</div>
      </div>
      ${warnings.length ? `<div class="warning">${warnings.map(escapeText).join(' · ')}</div>` : ''}
      ${refs.length ? `<details><summary>Evidence refs</summary>${renderRefLinks(refs, run)}</details>` : ''}
    </article>`;
  }

  function renderEvidenceFidelitySurface(run) {
    const fidelity = evidenceFidelity(run);
    if (!run) {
      return '<section id="evidence-fidelity" class="panel"><h2>Evidence fidelity</h2><p class="empty">No dashboard-data.json run is loaded yet.</p></section>';
    }
    if (!fidelity) {
      return '<section id="evidence-fidelity" class="panel"><h2>Evidence fidelity</h2><p class="empty">No evidence fidelity read model is available. Export dashboard data with a newer Rust CLI.</p></section>';
    }
    return `<section id="evidence-fidelity" class="panel"><h2>Evidence fidelity</h2>
      <p class="hint">Read-only status from Rust-exported dashboard data. Missing evidence is shown as warnings; the cockpit does not write files, execute commands, rerun QA, or apply mutations.</p>
      <div class="fidelity-grid">
        ${renderFidelityStatusCard(fidelity.transaction, 'transaction', 'Transaction provenance', run)}
        ${renderFidelityStatusCard(fidelity.runtime_probe, 'runtime_probe', 'Runtime probe contract', run)}
        ${renderFidelityStatusCard(fidelity.input_replay, 'input_replay', 'Input replay evidence', run)}
        ${renderFidelityStatusCard(fidelity.openchrome_cdp, 'openchrome_cdp', 'Openchrome/CDP evidence', run)}
        ${renderFidelityStatusCard(fidelity.command_context, 'command_context', 'Reproducible command context', run)}
      </div>
    </section>`;
  }

  function renderEvidenceBrowser(run) {
    if (!run) {
      return '<section id="run-browser" class="panel"><h2>Run/evidence browser</h2><p class="empty">No dashboard-data.json run is loaded yet. Run QA and export dashboard data to populate this pane.</p></section>';
    }
    const evidence = Array.isArray(run.evidence) ? run.evidence : [];
    const evidenceLinks = evidence.slice(0, 8).map((artifact) => `<a href="${escapeText(artifactHref(artifact, run))}" target="_blank" rel="noreferrer">${escapeText(artifact.id || artifact.path)}</a>`).join('<br>') || 'No evidence artifacts recorded.';
    return `<section id="run-browser" class="panel"><h2>Run/evidence browser</h2>
      <div class="field-grid"><div><strong>Run</strong><br>${escapeText(run.summary?.id)}</div><div><strong>Verdict</strong><br>${escapeText(run.summary?.verdict_status)}</div><div><strong>Scenario</strong><br>${escapeText(run.summary?.scenario_status || 'unknown')}</div><div><strong>Evidence</strong><br>${escapeText(evidence.length)}</div></div>
      <p class="hint"><a href="../evidence-dashboard/">Open full evidence dashboard</a> for complete artifact inspection.</p>
      <h3>Evidence links</h3><p>${evidenceLinks}</p>
    </section>`;
  }

  function renderAuthoringProvenanceSurface(run) {
    const provenance = run?.transaction_provenance;
    if (!run) {
      return '<section id="authoring-provenance" class="panel"><h2>Authoring provenance</h2><p class="empty">No dashboard-data.json run is loaded yet. Run QA and export dashboard data to inspect transaction provenance.</p></section>';
    }
    if (!provenance) {
      return `<section id="authoring-provenance" class="panel"><h2>Authoring provenance</h2>
        <p class="empty">This run has no scene edit transaction binding. Use the Rust CLI <code>--transaction</code> option to bind a validated scene edit transaction to a QA run.</p>
        <div class="command-list">
          <code>${escapeText(qaTransactionCommand())}</code>
          <code>${escapeText(dashboardExportCommand())}</code>
        </div>
      </section>`;
    }
    const transactionPath = provenance.transactionArtifactPath || 'unknown transaction artifact';
    const scenePath = provenance.scenePath || DEFAULT_SCENE_PATH;
    return `<section id="authoring-provenance" class="panel"><h2>Authoring provenance</h2>
      <p class="hint">Read-only chain from Rust-authored transaction provenance. The cockpit does not write scene files or execute QA commands.</p>
      <div class="field-grid">
        <div><strong>Run</strong><br>${escapeText(run.summary?.id || 'unknown')}</div>
        <div><strong>Transaction</strong><br>${escapeText(provenance.transactionId || 'unknown')}</div>
        <div><strong>Scene</strong><br>${escapeText(scenePath)}</div>
        <div><strong>Evidence artifacts</strong><br>${escapeText((run.evidence || []).length)}</div>
        <div><strong>Before hash</strong><br>${escapeText(provenance.beforeSceneHash?.value || 'unknown')}</div>
        <div><strong>After hash</strong><br>${escapeText(provenance.afterSceneHash?.value || 'unknown')}</div>
      </div>
      <h3>Display-only follow-up commands</h3>
      <div class="command-list">
        <code>${escapeText(sceneValidateCommand(scenePath))}</code>
        <code>${escapeText(qaTransactionCommand('seeds/platformer.yaml', transactionPath))}</code>
        <code>${escapeText(dashboardExportCommand())}</code>
      </div>
    </section>`;
  }

  function renderJournalSurface(run) {
    const journal = run?.journal_view;
    if (!run || (!journal?.exists && !run.journal)) {
      return '<section id="journal-viewer" class="panel"><h2>Journal viewer</h2><p class="empty">No journal artifact is available in the loaded dashboard data.</p></section>';
    }
    return `<section id="journal-viewer" class="panel"><h2>Journal viewer</h2>
      <p class="hint">Read-only run journal preview from generated dashboard data.</p>
      <div class="field-grid"><div><strong>Path</strong><br>${escapeText(journal?.path || 'journal.md')}</div><div><strong>Entries</strong><br>${escapeText((journal?.entries || []).length)}</div></div>
      <h3>Evidence refs</h3>${renderRefLinks(journal?.evidence_refs || [], run)}
      <pre>${escapeText(journal?.summary || run.journal || 'No journal summary loaded.')}</pre>
    </section>`;
  }

  function mutationStage(lifecycle, id) {
    return (lifecycle?.stages || []).find((stage) => stage.id === id) || null;
  }

  function renderSceneMutationLifecycleSurface(run) {
    const lifecycle = run?.mutation_lifecycle;
    if (!lifecycle) {
      return '<div class="scene-mutation-lifecycle"><h3>Scene-only mutation lifecycle</h3><p class="empty compact">No mutation lifecycle read model is available.</p></div>';
    }
    const proposed = mutationStage(lifecycle, 'proposed');
    const applied = mutationStage(lifecycle, 'scene_applied');
    const runDir = run?.summary?.run_dir || (run?.summary?.id ? `runs/${run.summary.id}` : 'runs/run-1');
    const proposedRecords = Array.isArray(proposed?.records) ? proposed.records : [];
    const appliedRecords = Array.isArray(applied?.records) ? applied.records : [];
    const firstApplication = appliedRecords[0] || {};
    const targetScenePath = firstApplication.targetScenePath || DEFAULT_SCENE_PATH;
    const transactionPath = firstApplication.transactionArtifactPath || 'mutation/scene-transaction.json';
    const projectPath = firstApplication.project?.manifestPath || null;
    const projectMutationRecords = appliedRecords.filter((record) => record.project && typeof record.project === 'object');
    const proposedRows = proposedRecords.slice(0, 3).map((record) => {
      const id = record.id || record.proposalId || 'unknown proposal';
      const evidence = record.evidence_id || record.evidenceId || record.evidence || 'no evidence id';
      return `<li>${escapeText(id)} · ${escapeText(evidence)}</li>`;
    }).join('') || '<li>No scene-safe proposal records loaded.</li>';
    const applicationRows = appliedRecords.slice(0, 3).map((record) => {
      const project = record.project || {};
      const rollback = record.rollback || {};
      const projectLine = record.project
        ? `<br><small>project ${escapeText(project.projectId || 'unknown')} · manifest ${escapeText(project.manifestPath || 'unknown')} · scene ${escapeText(project.scenePath || record.targetScenePath || 'unknown')}</small>`
        : '<br><small>legacy/no project mutation context recorded</small>';
      const rollbackLine = record.rollback
        ? `<br><small>rollback ${escapeText(rollback.scenePath || 'unknown')} → ${escapeText(rollback.restoreHash?.value || 'unknown')}</small>`
        : '';
      const decisionLine = record.reviewDecisionId
        ? `<br><small>review decision ${escapeText(record.reviewDecisionId)}</small>`
        : '<br><small>legacy/no review decision linkage recorded</small>';
      return `<div class="surface-row"><strong>${escapeText(record.id || 'scene application')}</strong> ${surfaceState(record.status !== 'failed', record.status || 'applied')}<br><small>proposal ${escapeText(record.proposalId || 'unknown')} · transaction ${escapeText(record.transactionId || 'unknown')}</small>${decisionLine}<br><small>${escapeText(record.beforeSceneHash?.value || 'before unknown')} → ${escapeText(record.afterSceneHash?.value || 'after unknown')}</small>${projectLine}${rollbackLine}</div>`;
    }).join('') || '<p class="empty compact">No scene-only mutation application records loaded yet.</p>';
    // A recorded application's reviewDecisionId is, by definition, already
    // consumed: the Rust preflight rejects reusing a decision ("review-gated
    // scene apply decision ... was already used"). Embedding it here would make
    // the copyable command fail for the exact review-gated records this surface
    // displays, so the manual template intentionally omits a decision id. Each
    // application row still shows its consumed decision id as provenance.
    const applyCommand = sceneMutationApplyCommand(runDir, 'mutation/scene-operation.json', transactionPath, projectPath);
    const projectCommand = projectPath ? `<code>${escapeText(projectValidateCommand(projectPath))}</code>` : '';
    return `<div class="scene-mutation-lifecycle"><h3>Project-scoped scene mutation lifecycle</h3>
      <p class="hint">Scene-only project mutations remain manual and Rust-validated. The browser displays proposal/application state and safe CLI strings only; it does not apply, accept, reject, rollback, or merge anything.</p>
      <div class="field-grid">
        <div><strong>Proposal stage</strong><br>${surfaceState(Boolean(proposed && proposed.state !== 'missing'), proposed?.state || 'missing')}<br><small>${escapeText(proposed?.record_count || 0)} record(s)</small></div>
        <div><strong>Scene application stage</strong><br>${surfaceState(Boolean(applied && applied.state !== 'missing'), applied?.state || 'missing')}<br><small>${escapeText(applied?.record_count || 0)} record(s)</small></div>
        <div><strong>Project-scoped applications</strong><br>${escapeText(projectMutationRecords.length)} record(s)</div>
        <div><strong>Target scene</strong><br>${escapeText(targetScenePath)}</div>
        <div><strong>Project manifest</strong><br>${escapeText(projectPath || 'legacy/no project context')}</div>
      </div>
      <h4>Proposal context</h4><ul>${proposedRows}</ul>
      <h4>Application records</h4>${applicationRows}
      <h4>Display-only scene mutation commands</h4>
      <div class="command-list">
        ${projectCommand}
        <code>${escapeText(sceneValidateCommand(targetScenePath))}</code>
        <code>${escapeText(applyCommand)}</code>
        <code>${escapeText(dashboardExportCommand())}</code>
      </div>
    </div>`;
  }

  function renderProposalRationale(proposal) {
    const rationale = proposal?.rationale;
    if (!rationale || typeof rationale !== 'object') {
      return '<p class="empty compact">No proposal rationale recorded.</p>';
    }
    const evidenceIds = Array.isArray(rationale.evidence_artifact_ids) && rationale.evidence_artifact_ids.length
      ? rationale.evidence_artifact_ids.map((id) => `<code>${escapeText(id)}</code>`).join('')
      : '<span class="warn">missing evidence ids</span>';
    const scenarioRefs = Array.isArray(rationale.scenario_result_refs) && rationale.scenario_result_refs.length
      ? `<br><small>scenario refs ${rationale.scenario_result_refs.map((ref) => escapeText(ref)).join(', ')}</small>`
      : '';
    const verdictRefs = Array.isArray(rationale.verdict_refs) && rationale.verdict_refs.length
      ? `<br><small>verdict refs ${rationale.verdict_refs.map((ref) => escapeText(ref)).join(', ')}</small>`
      : '';
    return `<div class="surface-row"><strong>${escapeText(rationale.failure_classification || 'missing rationale')}</strong> ${surfaceState(Boolean(rationale.evidence_artifact_ids?.length), rationale.allowed_mutation_type || 'missing')}<br><small>${escapeText(rationale.expected_effect || 'No expected effect recorded')}</small><br><small>evidence ids ${evidenceIds}</small>${scenarioRefs}${verdictRefs}<br><small>confidence ${escapeText(rationale.confidence || 'missing')} · ${escapeText(rationale.reasoning_summary || 'No reasoning summary recorded')}</small></div>`;
  }

  function proposalRecordId(proposal) {
    return proposal?.id || proposal?.proposalId || proposal?.proposal_id || null;
  }

  function hasProposalRationale(proposal) {
    return proposal?.rationale && typeof proposal.rationale === 'object';
  }

  function mergeProposalRationaleRecords(direct, staged) {
    const stagedById = new Map();
    staged.forEach((proposal) => {
      const id = proposalRecordId(proposal);
      if (id && !stagedById.has(id)) stagedById.set(id, proposal);
    });
    const seen = new Set();
    const merged = direct.map((proposal) => {
      const id = proposalRecordId(proposal);
      const stagedProposal = id ? stagedById.get(id) : null;
      if (id) seen.add(id);
      if (!stagedProposal) return { ...proposal };
      return {
        ...stagedProposal,
        ...proposal,
        rationale: hasProposalRationale(proposal) ? proposal.rationale : stagedProposal.rationale,
      };
    });
    staged.forEach((proposal) => {
      const id = proposalRecordId(proposal);
      if (!id || !seen.has(id)) merged.push({ ...proposal });
    });
    return merged;
  }

  function renderProposalRationaleSurface(run) {
    const direct = Array.isArray(run?.mutations) ? run.mutations : [];
    const proposed = mutationStage(run?.mutation_lifecycle, 'proposed');
    const staged = Array.isArray(proposed?.records) ? proposed.records : [];
    const proposals = mergeProposalRationaleRecords(direct, staged);
    const rows = proposals.map((proposal) => `<div class="surface-row"><strong>${escapeText(proposal.id || 'unknown proposal')}</strong><br>${renderProposalRationale(proposal)}</div>`).join('') || '<p class="empty compact">No proposal rationale records loaded.</p>';
    return `<div class="proposal-rationale"><h3>Proposal rationale</h3><p class="hint">Read-only evidence-linked rationale. The cockpit does not accept, apply, promote, rerun, or execute proposal actions.</p>${rows}</div>`;
  }


  function renderReviewDecisionSurface(lifecycle, run) {
    const stage = (lifecycle?.stages || []).find((item) => item.id === 'reviewed');
    const records = Array.isArray(stage?.records) ? stage.records : [];
    if (!records.length) {
      return '<div class="proposal-rationale"><h3>Review decisions</h3><p class="empty compact">No review decisions recorded. Use the Rust CLI outside the browser to append decisions.</p></div>';
    }
    const rows = records.map((record) => `<div class="surface-row"><strong>${escapeText(record.id || 'review-decision')}</strong> ${surfaceState(true, record.decision_status || record.state || 'unknown')}<br><small>proposal ${escapeText(record.proposal_id || 'unlinked')} · reviewer ${escapeText(record.reviewer || 'unknown')} (${escapeText(record.reviewer_type || 'unknown')})</small><br><span>${escapeText(record.reason || '')}</span>${renderRefLinks(record.evidence_refs, run)}</div>`).join('');
    return `<div class="proposal-rationale"><h3>Review decisions</h3><p class="hint">Read-only append-only review ledger. The cockpit does not write, accept, apply, promote, rerun, or merge mutations.</p>${rows}</div>`;
  }

  function reviewCockpitStage(run, key, fallbackId) {
    const cockpit = run?.review_cockpit || run?.reviewCockpit || null;
    const stage = cockpit && typeof cockpit === 'object' ? cockpit[key] : null;
    if (stage && typeof stage === 'object' && !Array.isArray(stage)) return stage;
    const lifecycleStage = mutationStage(run?.mutation_lifecycle, fallbackId);
    if (!lifecycleStage) {
      return { id: fallbackId, label: key, state: 'missing', recordCount: 0, recordIds: [], evidenceRefs: [], readError: 'No review cockpit stage exported.' };
    }
    return {
      id: lifecycleStage.id,
      label: lifecycleStage.label,
      state: lifecycleStage.state,
      artifactPath: lifecycleStage.artifact_path || lifecycleStage.artifactPath,
      recordCount: lifecycleStage.record_count || lifecycleStage.recordCount || 0,
      recordIds: (lifecycleStage.records || []).map((record) => record?.id || record?.proposalId || record?.proposal_id || record?.decisionId || record?.applicationId).filter(Boolean),
      evidenceRefs: lifecycleStage.evidence_refs || lifecycleStage.evidenceRefs || [],
      readError: lifecycleStage.read_error || lifecycleStage.readError || null,
    };
  }

  function renderReviewCockpitStageCard(stage, run) {
    if (!stage || typeof stage !== 'object' || Array.isArray(stage)) {
      return '<div class="surface-row warning"><strong>Malformed review cockpit stage</strong><p class="empty compact">Stage data is missing or malformed in exported dashboard data.</p></div>';
    }
    const state = stage.state || 'missing';
    const count = stage.recordCount ?? stage.record_count ?? 0;
    const artifact = stage.artifactPath || stage.artifact_path || 'No artifact path';
    const ids = Array.isArray(stage.recordIds) ? stage.recordIds : Array.isArray(stage.record_ids) ? stage.record_ids : [];
    const refs = Array.isArray(stage.evidenceRefs) ? stage.evidenceRefs : Array.isArray(stage.evidence_refs) ? stage.evidence_refs : [];
    const readError = stage.readError || stage.read_error || '';
    const idsText = ids.length ? ids.map((id) => `<code>${escapeText(id)}</code>`).join('') : '<span class="empty compact">No record ids exported.</span>';
    const refsHtml = refs.length ? renderRefLinks(refs, run) : '<p class="empty compact">No evidence refs exported for this stage.</p>';
    const warning = readError ? `<p class="warn">${escapeText(readError)}</p>` : '';
    return `<div class="surface-row review-cockpit-card"><strong>${escapeText(stage.label || stage.id || 'review cockpit stage')}</strong> ${surfaceState(state !== 'missing' && state !== 'malformed', state)}<br><small>${escapeText(count)} record(s) · ${escapeText(artifact)}</small><div>${idsText}</div>${warning}${refsHtml}</div>`;
  }

  function renderStudioReviewCockpitCards(run) {
    const cockpit = run?.review_cockpit || run?.reviewCockpit || null;
    const schema = cockpit && typeof cockpit === 'object' ? (cockpit.schemaVersion || cockpit.schema_version || 'legacy lifecycle fallback') : 'legacy lifecycle fallback';
    const terminal = cockpit && typeof cockpit === 'object' ? (cockpit.terminalState || cockpit.terminal_state || 'missing') : (run?.mutation_lifecycle?.terminal_state || 'missing');
    const boundary = cockpit && typeof cockpit === 'object' ? (cockpit.boundary || 'read-only exported evidence') : 'read-only exported evidence';
    const commandHints = cockpit && typeof cockpit === 'object' && Array.isArray(cockpit.commandHints) ? cockpit.commandHints : Array.isArray(cockpit?.command_hints) ? cockpit.command_hints : (run?.mutation_lifecycle?.command_hints || []);
    const cards = [
      renderReviewCockpitStageCard(reviewCockpitStage(run, 'proposals', 'proposed'), run),
      renderReviewCockpitStageCard(reviewCockpitStage(run, 'decisions', 'reviewed'), run),
      renderReviewCockpitStageCard(reviewCockpitStage(run, 'applications', 'scene_applied'), run),
      renderReviewCockpitStageCard(reviewCockpitStage(run, 'comparisons', 'compared'), run),
      renderReviewCockpitStageCard(reviewCockpitStage(run, 'promotions', 'promoted'), run),
    ].join('');
    const hints = commandHints.length ? commandHints.map((hint) => `<code>${escapeText(hint)}</code>`).join('') : '<p class="empty compact">No inert manual review command hints exported.</p>';
    return `<div class="studio-review-cockpit"><h3>Studio review cockpit</h3><p class="hint">${escapeText(boundary)}. Schema ${escapeText(schema)}. Terminal state: ${escapeText(terminal)}.</p><div class="surface-list">${cards}</div><h4>Inert copyable review commands</h4><div class="command-list">${hints}</div></div>`;
  }

  function renderMutationReviewSurface(run) {
    const lifecycle = run?.mutation_lifecycle;
    if (!lifecycle) {
      return '<section id="mutation-review" class="panel"><h2>Mutation review state</h2><p class="empty">No mutation lifecycle read model is available for this run.</p></section>';
    }
    const stages = (lifecycle.stages || []).map((stage) => `<div class="surface-row"><strong>${escapeText(stage.label || stage.id)}</strong> ${surfaceState(stage.state !== 'missing', stage.state || 'missing')}<br><small>${escapeText(stage.artifact_path || 'No artifact path')}</small></div>`).join('') || '<p class="empty compact">No mutation stages recorded.</p>';
    const hints = (lifecycle.command_hints || []).map((hint) => `<code>${escapeText(hint)}</code>`).join('') || '<p class="empty compact">No manual review command hints are available.</p>';
    return `<section id="mutation-review" class="panel"><h2>Mutation review state</h2>
      <p class="hint">Inspect-only. The cockpit does not accept/reject mutations or apply patches.</p>
      <div><strong>Terminal state:</strong> ${escapeText(lifecycle.terminal_state || 'missing')}</div>
      ${renderStudioReviewCockpitCards(run)}
      <div class="surface-list">${stages}</div>
      ${renderProposalRationaleSurface(run)}
      ${renderReviewDecisionSurface(lifecycle, run)}
      ${renderSceneMutationLifecycleSurface(run)}
      <h3>Command hints</h3><div class="command-list">${hints}</div>
    </section>`;
  }

  function renderRegressionMatrixSurface(run) {
    const matrix = run?.regression_matrix || run?.regressionMatrix || null;
    const matrixSummary = renderReviewCockpitStageCard(reviewCockpitStage(run, 'matrix', 'matrix'), run);
    if (!matrix || typeof matrix !== 'object') {
      return `<section id="regression-matrix" class="panel"><h2>Regression run matrix</h2><p class="empty">No regression matrix export loaded. Run dashboard export with the latest Rust CLI.</p>${matrixSummary}</section>`;
    }
    const projects = Array.isArray(matrix.projects) ? matrix.projects : [];
    const skipped = Array.isArray(matrix.skippedRuns) ? matrix.skippedRuns : Array.isArray(matrix.skipped_runs) ? matrix.skipped_runs : [];
    const skippedText = skipped.length ? `${skipped.length} legacy/malformed run(s) skipped` : 'all matrix inputs project-bound';
    if (!projects.length) {
      return `<section id="regression-matrix" class="panel"><h2>Regression run matrix</h2><p class="empty">No project-bound scenario runs available.</p><p class="hint">${escapeText(skippedText)}</p>${matrixSummary}</section>`;
    }
    const rows = projects.flatMap((project) => {
      const packs = Array.isArray(project.scenarioPacks) ? project.scenarioPacks : Array.isArray(project.scenario_packs) ? project.scenario_packs : [];
      return packs.flatMap((pack) => {
        const scenarios = Array.isArray(pack.scenarios) ? pack.scenarios : [];
        return scenarios.map((scenario) => renderRegressionMatrixSurfaceRow(project, pack, scenario));
      });
    }).join('') || '<div class="surface-row">No scenario rows in matrix.</div>';
    return `<section id="regression-matrix" class="panel"><h2>Regression run matrix</h2>
      <p class="hint">Read-only local run history. The cockpit does not schedule CI, rerun scenarios, promote scenarios, execute commands, or write scenario packs from this matrix.</p>
      <p class="hint">${escapeText(skippedText)}</p>
      ${matrixSummary}
      <div class="surface-list">${rows}</div>
    </section>`;
  }

  function renderRegressionMatrixSurfaceRow(project, pack, scenario) {
    const current = scenario.currentStatus || scenario.current_status || 'unknown';
    const lastPass = scenario.lastPass || scenario.last_pass || null;
    const lastFail = scenario.lastFail || scenario.last_fail || null;
    const context = scenario.context || {};
    const mutationIds = context.mutationIds || context.mutation_ids || [];
    const reviewIds = context.reviewDecisionIds || context.review_decision_ids || [];
    const promotionIds = context.promotionIds || context.promotion_ids || [];
    return `<div class="surface-row"><strong>${escapeText(scenario.scenarioId || scenario.scenario_id || 'unknown scenario')}</strong> ${surfaceState(true, current)}<br>
      <small>project ${escapeText(project.projectId || project.project_id || 'unknown')} · pack ${escapeText(pack.scenarioPackId || pack.scenario_pack_id || 'unknown')} · runs ${escapeText((scenario.runs || []).length)}</small><br>
      <small>last pass ${escapeText(regressionMatrixObservationLabel(lastPass))} · last fail ${escapeText(regressionMatrixObservationLabel(lastFail))}</small><br>
      <small>context mutations ${escapeText(Array.isArray(mutationIds) ? mutationIds.length : 0)} · reviews ${escapeText(Array.isArray(reviewIds) ? reviewIds.length : 0)} · promotions ${escapeText(Array.isArray(promotionIds) ? promotionIds.length : 0)}</small></div>`;
  }

  function regressionMatrixObservationLabel(observation) {
    if (!observation) return 'none';
    return `${observation.runId || observation.run_id || 'unknown-run'} (${observation.status || 'unknown'})`;
  }

  function renderRegressionPromotionSurface(run) {
    const records = Array.isArray(run?.regression_promotions) ? run.regression_promotions : [];
    const context = run?.command_context || run?.summary?.command_context || {};
    const project = run?.project || run?.summary?.project || {};
    const projectPath = project.manifestPath || context.manifestPath || 'ouroforge.project.json';
    const promotionSummary = renderReviewCockpitStageCard(reviewCockpitStage(run, 'promotions', 'promotions'), run);
    if (!records.length) {
      return `<section id="regression-promotions" class="panel"><h2>Regression promotions</h2><p class="empty">No regression promotion records loaded. Use the Rust CLI outside the browser to generate drafts, dry-run promotion, and promote manually.</p>${promotionSummary}</section>`;
    }
    const rows = records.map((record) => {
      const target = record.target || {};
      const packId = target.scenarioPackId || target.scenario_pack_id || '<pack-id>';
      const command = `cargo run -p ouroforge-cli -- scenario promote <draft-json> --project ${projectPath} --scenario-pack ${packId} --dry-run`;
      return `<div class="surface-row"><strong>${escapeText(record.scenarioId || record.scenario_id || 'unknown scenario')}</strong> ${surfaceState(true, record.dryRun ? 'dry-run' : 'promoted')}<br>
        <small>pack ${escapeText(packId)} · before ${escapeText(record.beforeHash?.value || record.before_hash?.value || 'missing')} · after ${escapeText(record.afterHash?.value || record.after_hash?.value || 'missing')}</small><br>
        <small>record ${escapeText(record.recordPath || record.record_path || 'dry-run/no record')}</small>
        <div class="command-list"><code>${escapeText(command)}</code></div></div>`;
    }).join('');
    return `<section id="regression-promotions" class="panel"><h2>Regression promotions</h2><p class="hint">Display-only manual promotion records. The cockpit does not generate drafts, dry-run, promote, execute commands, or write scenario packs from browser JavaScript.</p>${promotionSummary}${rows}</section>`;
  }

  function renderReplaySurface(run) {
    const replay = run?.replay;
    if (!replay?.present) {
      return `<section id="replay-controls" class="panel"><h2>Replay controls</h2><p class="empty">${escapeText(replay?.empty_state || 'No replay evidence is available for this run.')}</p></section>`;
    }
    const sequences = (replay.sequences || []).map((sequence) => `<div class="surface-row"><strong>${escapeText(sequence.id)}</strong><br><small>${escapeText(sequence.event_count || 0)} event(s), frames ${(sequence.frames || []).map(escapeText).join(', ') || 'none'}</small>${renderRefLinks(sequence.evidence_refs, run)}</div>`).join('');
    return `<section id="replay-controls" class="panel"><h2>Replay controls</h2><p class="hint">Replay is displayed from generated evidence. Use the full evidence dashboard for frame stepping; cockpit composition remains read-only.</p>${sequences}</section>`;
  }

  function renderSemanticComparisonSummary(artifact) {
    const semantic = artifact?.semantic || artifact?.value?.semantic;
    if (!semantic || typeof semantic !== 'object') {
      return '<p class="empty compact">No semantic comparison summary is available for this artifact.</p>';
    }
    const reasons = Array.isArray(semantic.reasons) ? semantic.reasons : [];
    const warnings = Array.isArray(semantic.warnings) ? semantic.warnings : [];
    const project = semantic.project && typeof semantic.project === 'object' ? semantic.project : null;
    const projectChanges = Array.isArray(project?.changes) ? project.changes : [];
    const projectWarnings = Array.isArray(project?.warnings) ? project.warnings : [];
    const reasonItems = reasons.length
      ? reasons.slice(0, 5).map((reason) => `<li><span class="status-ok">${escapeText(reason.severity || 'changed')}</span> ${escapeText(reason.kind || 'reason')}: ${escapeText(reason.summary || '')}</li>`).join('')
      : '<li>No semantic reasons recorded.</li>';
    const warningBlock = warnings.length
      ? `<div class="error">Warnings: ${escapeText(warnings.join(' · '))}</div>`
      : '';
    const projectList = projectChanges.length
      ? `<ul>${projectChanges.map((change) => `<li><strong>${escapeText(change.kind || 'project')}</strong>: ${escapeText(change.summary || '')} (${escapeText(change.before ?? 'none')} → ${escapeText(change.after ?? 'none')})</li>`).join('')}</ul>`
      : '<p class="empty compact">No project context changes recorded.</p>';
    const projectBlock = project
      ? `<div class="semantic-project"><strong>Project comparison summary</strong>
        <div class="field-grid compact">
          <div><strong>Relation</strong><br>${escapeText(project.relation || 'unknown')}</div>
          <div><strong>Changed</strong><br>${escapeText(project.changed === true ? 'true' : 'false')}</div>
          <div><strong>Changes</strong><br>${escapeText(projectChanges.length)}</div>
        </div>
        ${projectList}
        ${projectWarnings.length ? `<div class="error">Project warnings: ${escapeText(projectWarnings.join(' · '))}</div>` : ''}
      </div>`
      : '<p class="empty compact">No project comparison fields are available for this artifact.</p>';
    return `<div class="semantic-summary"><strong>Semantic evidence diff</strong>
      <div class="field-grid">
        <div><strong>Schema</strong><br>${escapeText(semantic.schemaVersion || 'legacy')}</div>
        <div><strong>Scenario diffs</strong><br>${escapeText((semantic.scenarios || []).length)}</div>
        <div><strong>World changes</strong><br>${escapeText((semantic.worldState?.changed || []).length)}</div>
        <div><strong>Project</strong><br>${escapeText(project?.relation || 'unavailable')}</div>
        <div><strong>Transaction</strong><br>${escapeText(semantic.transactionProvenance?.changed ? 'changed' : 'unchanged')}</div>
      </div>
      <ul>${reasonItems}</ul>${projectBlock}${warningBlock}
    </div>`;
  }

  function renderComparisonSurface(run) {
    const comparison = run?.comparison;
    if (!comparison?.present) {
      return `<section id="run-comparison" class="panel"><h2>Run comparison</h2><p class="empty">${escapeText(comparison?.empty_state || 'No run comparison artifacts are available for this run.')}</p></section>`;
    }
    const artifacts = (comparison.artifacts || []).map((artifact) => `<div class="surface-row"><strong>${escapeText(artifact.before_run_id || 'unknown')}</strong> → <strong>${escapeText(artifact.after_run_id || 'unknown')}</strong> ${surfaceState(true, artifact.classification || 'unknown')}<br><small>${escapeText(artifact.path)}</small>${renderSemanticComparisonSummary(artifact)}${renderRefLinks(artifact.evidence_refs, run)}</div>`).join('');
    const first = (comparison.artifacts || [])[0] || {};
    const beforeRun = first.before_run_id ? `runs/${first.before_run_id}` : 'runs/before';
    const afterRun = first.after_run_id ? `runs/${first.after_run_id}` : run?.summary?.run_dir || 'runs/after';
    return `<section id="run-comparison" class="panel"><h2>Run comparison</h2><p class="hint">Displays existing comparison artifacts only; no browser-side comparison algorithm runs here.</p>${artifacts}<h3>Display-only compare command</h3><div class="command-list"><code>${escapeText(compareRunsCommand(beforeRun, afterRun, `${afterRun}/comparisons`))}</code></div></section>`;
  }


  function renderLoopDryRunSurface(run) {
    const summary = run?.loop_dry_run || run?.loopDryRun || null;
    if (!summary || typeof summary !== 'object') {
      return '<section id="loop-dry-run" class="panel"><h2>Authoring loop dry-run</h2><p class="empty">No dry-run summary is attached to dashboard-data.json. Run the Rust CLI dry-run command and keep generated reports untracked.</p></section>';
    }
    const steps = Array.isArray(summary.steps) ? summary.steps : [];
    const missing = Array.isArray(summary.missingPrerequisites) ? summary.missingPrerequisites : [];
    const rows = steps.length ? steps.map((step) => {
      const stepMissing = Array.isArray(step.missingPrerequisites) ? step.missingPrerequisites : [];
      const prerequisites = Array.isArray(step.prerequisites) ? step.prerequisites : [];
      const expectedArtifacts = Array.isArray(step.expectedArtifacts) ? step.expectedArtifacts : [];
      const expectedText = expectedArtifacts
        .map((artifact) => `${artifact?.id || 'artifact'}:${artifact?.path || 'unknown'}`)
        .join(' · ');
      return `<div class="surface-row"><strong>${escapeText(step.id || 'step')}</strong> ${surfaceState(step.readiness !== 'blocked', step.readiness || 'unknown')} <span>${escapeText(step.readiness || 'unknown')}</span><br>
        <small>${escapeText(step.kind || 'unknown')} · plan ${escapeText(step.status || 'unknown')}</small>
        <div class="command-list"><code>${escapeText(step.commandText || '')}</code></div>
        ${prerequisites.length ? `<small>Prerequisites: ${escapeText(prerequisites.join(' · '))}</small>` : '<small>No prerequisites recorded.</small>'}
        ${expectedArtifacts.length ? `<small>Expected artifacts: ${escapeText(expectedText)}</small>` : '<small>No expected artifacts recorded.</small>'}
        ${stepMissing.length ? `<div class="hint">Missing: ${escapeText(stepMissing.join(' · '))}</div>` : ''}
      </div>`;
    }).join('') : '<p class="empty">No dry-run steps recorded.</p>';
    return `<section id="loop-dry-run" class="panel"><h2>Authoring loop dry-run</h2>
      <p class="hint">Read-only inert summary. Command text is copyable display data only; the browser does not execute commands or write trusted state.</p>
      <div class="surface-row"><strong>${escapeText(summary.loopId || 'unknown')}</strong> ${surfaceState(summary.status !== 'blocked', summary.status || 'unknown')} <span>${escapeText(summary.status || 'unknown')}</span><br><small>${escapeText(summary.boundary || '')}</small></div>
      ${missing.length ? `<div class="hint">Blocked by: ${escapeText(missing.join(' · '))}</div>` : '<p class="hint">No missing prerequisites reported.</p>'}
      ${rows}
    </section>`;
  }

  function renderLoopExecutionSurface(run) {
    const summary = run?.loop_execution || run?.loopExecution || null;
    if (!summary || typeof summary !== 'object') {
      return '<section id="loop-execution" class="panel"><h2>Authoring loop execution</h2><p class="empty">No loop execution summary is attached to dashboard-data.json. The cockpit remains read-only and does not run loop steps.</p></section>';
    }
    const artifacts = Array.isArray(summary.generatedArtifacts) ? summary.generatedArtifacts : [];
    const blocked = Array.isArray(summary.blockedReasons) ? summary.blockedReasons : [];
    const rows = artifacts.length ? artifacts.map((artifact) => `<div class="surface-row"><strong>${escapeText(artifact.id || 'artifact')}</strong> ${escapeText(artifact.kind || 'unknown')}<br><small>${escapeText(artifact.path || '')}</small></div>`).join('') : '<p class="empty">No generated artifacts recorded.</p>';
    return `<section id="loop-execution" class="panel"><h2>Authoring loop execution</h2>
      <p class="hint">Read-only execution evidence from the Rust CLI. The browser never executes loop steps, writes trusted state, applies mutations, or promotes regressions.</p>
      <div class="surface-row"><strong>${escapeText(summary.loopId || 'unknown')}</strong> step <strong>${escapeText(summary.stepId || 'unknown')}</strong> ${surfaceState(summary.status === 'completed', summary.status || 'unknown')}<br><small>${escapeText(summary.kind || 'unknown')} · ledger ${escapeText(summary.ledgerPath || 'unrecorded')}</small></div>
      ${blocked.length ? `<div class="hint">Blocked by: ${escapeText(blocked.join(' · '))}</div>` : '<p class="hint">No blocked reasons reported.</p>'}
      ${rows}
      ${summary.boundary ? `<p class="hint">${escapeText(summary.boundary)}</p>` : ''}
    </section>`;
  }

  function renderStudioGaps() {
    return `<section class="panel"><h2>Known demo gaps</h2><ul>
      <li>No production editor, native shell, hosted studio, collaboration, plugin marketplace, or visual scripting.</li>
      <li>Generated dashboard data and run artifacts stay uncommitted local state.</li>
      <li>Scene persistence remains Rust-command-only through validated CLI commands.</li>
    </ul></section>`;
  }

  function renderEvidencePane(run) {
    return `${renderProjectWorkspaceSurface(run)}${renderProjectRunSurface(run)}${renderEvidenceFidelitySurface(run)}${renderEvidenceBrowser(run)}${renderAuthoringProvenanceSurface(run)}${renderEngineExpansionSurface(run)}${renderJournalSurface(run)}${renderLoopDryRunSurface(run)}${renderLoopExecutionSurface(run)}${renderMutationReviewSurface(run)}${renderRegressionPromotionSurface(run)}${renderRegressionMatrixSurface(run)}${renderReplaySurface(run)}${renderComparisonSurface(run)}`;
  }

  function renderIntegration(run, previewState = null) {
    return `${renderStudioNavigation(run)}<section id="live-preview">${renderPreview()}<div id="preview-controls-host">${renderPreviewControls(previewState)}</div></section><section id="scene-editing">${renderQaPanel()}</section>${renderEvidencePane(run)}${renderStudioGaps()}`;
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
      if (latest && (dashboardData.regression_matrix || dashboardData.regressionMatrix)) {
        latest = { ...latest, regression_matrix: dashboardData.regression_matrix || dashboardData.regressionMatrix };
      }
    } catch (_) {
      latest = null;
    }
    // Rebinds only the preview-control buttons inside the stable controls host.
    // Keeps the #runtime-preview iframe untouched so Pause/Resume/Step state persists.
    const bindPreviewControls = () => {
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
        const host = document.getElementById('preview-controls-host');
        if (host) {
          host.innerHTML = renderPreviewControls(previewState);
          bindPreviewControls();
        }
      }));
    };
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
          lastEditCommand = cliCommand(DEFAULT_SCENE_PATH, selectedId, path, getValue(entity, path));
          document.getElementById('edit-command').textContent = lastEditCommand;
          document.getElementById('edit-error').textContent = 'No validation errors.';
          document.getElementById('edit-error').className = 'hint';
        } catch (error) {
          editError = error.message;
          paint();
        }
      }));
      bindPreviewControls();
      document.getElementById('run-qa-button')?.addEventListener('click', () => {
        document.getElementById('qa-command').textContent = `${qaCommand()}\n${dashboardExportCommand()}`;
      });
    };
    paint();
  }

  return { EDITABLE_FIELDS, READ_ONLY_FIELDS, applyEdit, artifactHref, callPreviewProbe, cliCommand, compareRunsCommand, dashboardExportCommand, escapeText, getValue, init, latestRun, loadDashboardData, previewWindow, projectRunCommand, projectValidateCommand, qaCommand, qaTransactionCommand, readPreviewProbe, reloadPreview, renderAuthoringProvenanceSurface, renderCommandGenerationPanel, renderComparisonSurface, renderEngineExpansionSurface, renderEvidenceBrowser, renderEvidenceFidelitySurface, renderEvidencePane, fidelityStatusClass, renderInspector, renderIntegration, renderJournalSurface, renderLoopDryRunSurface, renderLoopExecutionSurface, renderMutationReviewSurface, renderProposalRationaleSurface, renderReviewDecisionSurface, renderRegressionMatrixSurface, renderRegressionPromotionSurface, renderProjectRunSurface, renderProjectWorkspaceSurface, renderPreview, renderPreviewControls, renderQaPanel, renderReadOnlyFields, renderReviewCockpitStageCard, renderStudioReviewCockpitCards, renderRunCommandContext, renderSemanticComparisonSummary, runtimeReloadPayloadCommand, sceneMutationApplyCommand, renderSceneMutationLifecycleSurface, sceneReloadValidateCommand, seedValidateCommand, sceneValidateCommand, transactionCommand, renderReplaySurface, renderStudioGaps, renderStudioNavigation, renderTree, resolvePreviewProbe, studioSurfaceSummary, validateEdit };
})();

if (typeof window !== 'undefined') {
  window.OuroforgeCockpit = OuroforgeCockpit;
  window.addEventListener('DOMContentLoaded', () => OuroforgeCockpit.init());
}

if (typeof module !== 'undefined') {
  module.exports = OuroforgeCockpit;
}
