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
    if (present) return `<span class="status-ok">${escapeText(label)}</span>`;
    const fallback = label === undefined || label === null || label === '' || label === 'available'
      ? 'gap / unavailable'
      : label;
    return `<span class="status-idle">${escapeText(fallback)}</span>`;
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
      { id: 'render-breakdown-inspection', label: 'Render breakdown inspection', present: Boolean(run?.engine_summaries?.render_breakdown?.present || run?.engine_summaries?.renderBreakdown?.present), detail: `${(run?.engine_summaries?.render_breakdown?.elements || run?.engine_summaries?.renderBreakdown?.elements || []).length} renderable row(s)` },
      { id: 'expressive-scene-inspection', label: 'Expressive scene inspection', present: Boolean(run?.engine_summaries?.components?.present || run?.engine_summaries?.triggers?.present || run?.engine_summaries?.hud?.present), detail: run?.engine_summaries?.source_world_state || 'component/trigger/HUD summary unavailable' },
      { id: 'runtime-event-inspection', label: 'Collision/transition/event inspection', present: Boolean(run?.engine_summaries?.collision?.present || run?.engine_summaries?.transition?.present || run?.engine_summaries?.events?.present), detail: run?.engine_summaries?.source_world_state || 'collision/transition/event summary unavailable' },
      { id: 'runtime-asset-loading', label: 'Runtime asset loading', present: Boolean(run?.asset_loading?.present || run?.assetLoading?.present), detail: `${run?.asset_loading?.attempt_count ?? run?.assetLoading?.attemptCount ?? 0} load attempt(s)` },
      { id: 'asset-preview-evidence', label: 'Asset preview evidence', present: Boolean(run?.asset_preview?.present || run?.assetPreview?.present), detail: `${run?.asset_preview?.preview_count ?? run?.assetPreview?.previewCount ?? 0} preview record(s)` },
      { id: 'source-apply-context', label: 'Source apply context', present: Boolean(run?.source_apply_worktree_context?.present || run?.sourceApplyWorktreeContext?.present), detail: `${run?.source_apply_worktree_context?.target_count ?? run?.sourceApplyWorktreeContext?.targetCount ?? 0} target row(s)` },
      { id: 'studio-draft-authoring', label: 'Studio draft authoring', present: Boolean(studioDraftAuthoringState(run).present), detail: `${studioDraftAuthoringState(run).drafts.length} temporary draft(s)` },
      { id: 'visual-diff-preview', label: 'Visual diff preview', present: Boolean(run?.visual_diff_preview?.present || run?.visualDiffPreview?.present), detail: `${run?.visual_diff_preview?.summary_count ?? run?.visualDiffPreview?.summaryCount ?? (run?.visual_diff_preview?.summaries || run?.visualDiffPreview?.summaries || []).length ?? 0} summary row(s)` },
      { id: 'studio-asset-inspector', label: 'Asset inspector', present: Boolean(run?.asset_inspector?.present || run?.assetInspector?.present), detail: `${run?.asset_inspector?.asset_count ?? run?.assetInspector?.assetCount ?? 0} asset row(s)` },
      { id: 'loop-cockpit', label: 'Loop cockpit', present: Boolean(normalizeStudioLoopCockpit(run?.loop_cockpit || run?.loopCockpit || null).loops.length), detail: `${normalizeStudioLoopCockpit(run?.loop_cockpit || run?.loopCockpit || null).loops.length} loop(s)` },
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
      ['Render breakdown', `${(summary?.render_breakdown?.elements || summary?.renderBreakdown?.elements || []).length} element(s), ${(summary?.render_breakdown?.absenceDiagnostics || summary?.render_breakdown?.absence_diagnostics || summary?.renderBreakdown?.absenceDiagnostics || []).length} absence diagnostic(s)`],
      ['Tilemaps', `${summaryValue(summary, 'tilemaps', 'tilemapCount', 0)} tilemap(s), ${summaryValue(summary, 'tilemaps', 'layerCount', 0)} layer(s)`],
      ['Assets', `${summaryValue(summary, 'assets', 'manifestId')} · ${summaryValue(summary, 'assets', 'assetCount', 0)} loaded/ref(s)`],
      ['Animation', `${summaryValue(summary, 'animation', 'animatedEntityCount', 0)} animated entit(ies)`],
      ['Audio', `${summaryValue(summary, 'audio', 'audioEntityCount', 0)} audio entit(ies), ${summaryValue(summary, 'audio', 'audioEventCount', 0)} event(s)`],
      ['Physics/contact', `${summaryValue(summary, 'physics', 'colliderEntityCount', 0)} collider entit(ies), ${summaryValue(summary, 'physics', 'collisionEventCount', 0)} event(s)`],
      ['Gameplay/HUD', `${summaryValue(summary, 'gameplay', 'worldFlagCount', 0)} flag(s), ${summaryValue(summary, 'gameplay', 'trueFlagCount', 0)} true, ${summaryValue(summary, 'gameplay', 'triggerCollisionEventCount', 0)} trigger event(s), ${summaryValue(summary, 'gameplay', 'hudValueEntityCount', 0)} HUD value(s)`],
      ['Reload', `${summaryValue(summary, 'reload', 'reloadCount', 0)} reload(s), last ${summaryValue(summary, 'reload', 'lastStatus')}`],
      ['Composition', `${summaryValue(summary, 'composition', 'entityCount', 0)} composed entit(ies), ${summaryValue(summary, 'composition', 'parentedEntityCount', 0)} parented`],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    return `<section id="engine-expansion" class="panel"><h2>Engine Expansion state</h2>
      <p class="hint">Preview-only read model from exported evidence. The cockpit does not own scene state or persist edits; use generated Rust commands for validation-gated changes.</p>
      <div class="field-grid">${cards}</div>
      <p class="hint">Source world-state: ${escapeText(summary.source_world_state || 'unknown')}</p>
    </section>`;
  }

  function renderBreakdownValue(record, snakeKey, camelKey, fallback = 'unknown') {
    const value = record?.[snakeKey] ?? record?.[camelKey];
    if (value === null || value === undefined || value === '') return fallback;
    return value;
  }

  function renderRenderBreakdownInspectionSurface(run) {
    const summary = run?.engine_summaries;
    const breakdown = summary?.render_breakdown || summary?.renderBreakdown || null;
    if (!summary?.present || !breakdown || typeof breakdown !== 'object' || Array.isArray(breakdown)) {
      return `<section id="render-breakdown-inspection" class="panel"><h2>Render breakdown inspection</h2><p class="empty">Render breakdown evidence is missing or malformed for this run.</p><p class="hint">Read-only inspection only: this panel does not write files, execute commands, mutate scenes, or control the browser runtime.</p></section>`;
    }
    const elements = Array.isArray(breakdown.elements) ? breakdown.elements : [];
    const absenceDiagnostics = Array.isArray(breakdown.absenceDiagnostics || breakdown.absence_diagnostics)
      ? (breakdown.absenceDiagnostics || breakdown.absence_diagnostics)
      : [];
    const readOnlyInspection = breakdown.readOnlyInspection || breakdown.read_only_inspection || {};
    const disallowedActions = Array.isArray(readOnlyInspection.disallowedActions || readOnlyInspection.disallowed_actions)
      ? (readOnlyInspection.disallowedActions || readOnlyInspection.disallowed_actions)
      : ['writes', 'commands', 'scene mutation', 'browser runtime control'];
    const cards = [
      ['Frame', renderBreakdownValue(breakdown, 'frame_id', 'frameId', 'unrecorded')],
      ['Scene', renderBreakdownValue(breakdown, 'scene_id', 'sceneId', summary?.scene?.sceneId || 'unrecorded')],
      ['Element count', breakdown.element_count ?? breakdown.elementCount ?? elements.length],
      ['Absence diagnostics', breakdown.absence_diagnostic_count ?? breakdown.absenceDiagnosticCount ?? absenceDiagnostics.length],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const elementRows = elements.slice(0, 24).map((element) => {
      const renderableId = renderBreakdownValue(element, 'renderable_id', 'renderableId', renderBreakdownValue(element, 'entity_id', 'entityId', 'unknown renderable'));
      const entityId = renderBreakdownValue(element, 'entity_id', 'entityId', renderableId);
      const frameId = renderBreakdownValue(element, 'frame_id', 'frameId', renderBreakdownValue(breakdown, 'frame_id', 'frameId', 'unrecorded'));
      const drawOrder = renderBreakdownValue(element, 'draw_order', 'drawOrder', 'unrecorded');
      const primitiveCategory = renderBreakdownValue(element, 'primitive_category', 'primitiveCategory', renderBreakdownValue(element, 'primitive', 'primitive', 'unknown primitive'));
      const debugLabel = renderBreakdownValue(element, 'debug_label', 'debugLabel', 'no debug label');
      return `<div class="surface-row"><strong>${escapeText(renderableId)}</strong> ${surfaceState(true, primitiveCategory)}<br><small>entity ${escapeText(entityId)} · frame ${escapeText(frameId)} · draw order ${escapeText(drawOrder)} · layer ${escapeText(renderBreakdownValue(element, 'layer', 'layer', 'default'))} · debug ${escapeText(debugLabel)}</small></div>`;
    }).join('') || '<div class="surface-row">No renderable draw-order rows exported.</div>';
    const absenceRows = absenceDiagnostics.slice(0, 24).map((diagnostic) => {
      const target = renderBreakdownValue(diagnostic, 'renderable_id', 'renderableId', renderBreakdownValue(diagnostic, 'entity_id', 'entityId', 'unknown entity'));
      const reason = renderBreakdownValue(diagnostic, 'reason', 'reason', 'unrecorded reason');
      const detail = renderBreakdownValue(diagnostic, 'detail', 'detail', 'no detail');
      return `<div class="surface-row"><strong>${escapeText(target)}</strong> ${surfaceState(true, reason)}<br><small>layer ${escapeText(renderBreakdownValue(diagnostic, 'layer', 'layer', 'default'))} · detail ${escapeText(detail)}</small></div>`;
    }).join('') || '<div class="surface-row">No absence diagnostics exported.</div>';
    return `<section id="render-breakdown-inspection" class="panel"><h2>Render breakdown inspection</h2>
      <p class="hint">Read-only render breakdown from runtime world-state evidence. This surface performs no writes, no commands, no scene mutation, and no browser runtime control.</p>
      <div class="field-grid">${cards}</div>
      <h3>Renderable draw order</h3>${elementRows}
      <h3>Absence diagnostics</h3>${absenceRows}
      <p class="hint">Disallowed actions: ${escapeText(disallowedActions.join(' · '))}</p>
      <p class="hint">${escapeText(breakdown.boundary || readOnlyInspection.boundary || 'Display-only render breakdown inspection.')}</p>
    </section>`;
  }

  function objectEntries(value) {
    if (!value || typeof value !== 'object' || Array.isArray(value)) return [];
    return Object.entries(value).sort(([left], [right]) => left.localeCompare(right));
  }

  function compactJson(value) {
    if (value === undefined || value === null || value === '') return 'none';
    if (typeof value === 'string' || typeof value === 'number' || typeof value === 'boolean') return String(value);
    try {
      return JSON.stringify(value);
    } catch (_) {
      return 'unserializable';
    }
  }

  function renderExpressiveComponentHudSurface(run) {
    const summary = run?.engine_summaries;
    if (!summary?.present) {
      return `<section id="expressive-scene-inspection" class="panel"><h2>Expressive scene inspection</h2><p class="empty">${escapeText(summary?.empty_state || 'No expressive scene read model is available for this run.')}</p></section>`;
    }
    const components = summary.components && typeof summary.components === 'object' && !Array.isArray(summary.components)
      ? summary.components
      : null;
    const triggers = summary.triggers && typeof summary.triggers === 'object' && !Array.isArray(summary.triggers)
      ? summary.triggers
      : null;
    const hud = summary.hud && typeof summary.hud === 'object' && !Array.isArray(summary.hud)
      ? summary.hud
      : null;
    const warnings = [];
    if (!components) warnings.push('component summary missing or malformed');
    if (!triggers) warnings.push('trigger summary missing or malformed');
    if (!hud) warnings.push('HUD summary missing or malformed');

    const componentCounts = objectEntries(components?.componentCounts)
      .map(([name, count]) => `<div><strong>${escapeText(name)}</strong><br>${escapeText(count)}</div>`)
      .join('') || '<p class="empty compact">No component counts exported.</p>';
    const entityRows = Array.isArray(components?.entities) && components.entities.length
      ? components.entities.map((entity) => `<li><strong>${escapeText(entity?.entityId || 'entity')}</strong> · ${escapeText(Array.isArray(entity?.components) ? entity.components.join(', ') : 'no component list')}</li>`).join('')
      : '<li>No entity component rows exported.</li>';
    const triggerRows = Array.isArray(triggers?.triggers) && triggers.triggers.length
      ? triggers.triggers.map((trigger) => `<div class="surface-row"><strong>${escapeText(trigger?.id || 'trigger')}</strong> on ${escapeText(trigger?.entityId || 'unknown entity')}<br><small>${escapeText(trigger?.kind || 'unknown kind')} · target ${escapeText(trigger?.targetFlag || 'none')} · requires ${escapeText(Array.isArray(trigger?.requiredFlags) ? trigger.requiredFlags.join(', ') || 'none' : 'none')} · onEnter ${escapeText(trigger?.onEnterCount ?? 0)}</small></div>`).join('')
      : '<p class="empty compact">No trigger components exported.</p>';
    const hudRows = Array.isArray(hud?.values) && hud.values.length
      ? hud.values.map((value) => `<div class="surface-row"><strong>${escapeText(value?.label || value?.kind || value?.entityId || 'HUD value')}</strong><br><small>${escapeText(value?.text || [value?.label, value?.value].filter(Boolean).join(': ') || 'no text')} · bind ${escapeText(value?.bindFlag || 'none')}=${escapeText(value?.flagValue ?? 'unbound')}</small></div>`).join('')
      : '<p class="empty compact">No HUD values exported.</p>';
    return `<section id="expressive-scene-inspection" class="panel"><h2>Expressive scene inspection</h2>
      <p class="hint">Read-only component, trigger, flag, and HUD summaries from Rust-exported evidence. Copyable commands elsewhere remain inert; this panel does not execute commands, write files, or persist scene state.</p>
      ${warnings.length ? `<div class="error">${escapeText(warnings.join(' · '))}</div>` : '<div class="hint">Component/trigger/HUD summaries loaded.</div>'}
      <div class="field-grid">
        <div><strong>Entities</strong><br>${escapeText(components?.entityCount ?? 'unknown')}</div>
        <div><strong>Triggers</strong><br>${escapeText(triggers?.triggerCount ?? 0)} component(s), ${escapeText(triggers?.triggerCollisionEventCount ?? 0)} trigger event(s)</div>
        <div><strong>HUD</strong><br>${escapeText(hud?.hudValueEntityCount ?? 0)} HUD value component(s)</div>
      </div>
      <h3>Component counts</h3><div class="field-grid">${componentCounts}</div>
      <h3>Entity components</h3><ul>${entityRows}</ul>
      <h3>Triggers and flags</h3>${triggerRows}
      <h3>HUD values</h3>${hudRows}
    </section>`;
  }


  function renderRuntimeEventInspectionSurface(run) {
    const summary = run?.engine_summaries;
    if (!summary?.present) {
      return `<section id="runtime-event-inspection" class="panel"><h2>Collision/transition/event inspection</h2><p class="empty">${escapeText(summary?.empty_state || 'No runtime event read model is available for this run.')}</p></section>`;
    }
    const collision = summary.collision && typeof summary.collision === 'object' && !Array.isArray(summary.collision)
      ? summary.collision
      : null;
    const transition = summary.transition && typeof summary.transition === 'object' && !Array.isArray(summary.transition)
      ? summary.transition
      : null;
    const events = summary.events && typeof summary.events === 'object' && !Array.isArray(summary.events)
      ? summary.events
      : null;
    const warnings = [];
    if (!collision) warnings.push('collision summary missing or malformed');
    if (!transition) warnings.push('transition summary missing or malformed');
    if (!events) warnings.push('runtime event summary missing or malformed');

    const collisionRules = objectEntries(collision?.rules)
      .map(([rule, value]) => `<div><strong>${escapeText(rule)}</strong><br>${escapeText(compactJson(value))}</div>`)
      .join('') || '<p class="empty compact">No collision rules exported.</p>';
    const collisionRows = Array.isArray(collision?.events) && collision.events.length
      ? collision.events.map((event) => `<div class="surface-row"><strong>${escapeText(event?.type || event?.kind || 'collision event')}</strong><br><small>${escapeText(compactJson(event))}</small></div>`).join('')
      : '<p class="empty compact">No collision event rows exported.</p>';
    const transitionRows = Array.isArray(transition?.transitions) && transition.transitions.length
      ? transition.transitions.map((event) => `<div class="surface-row"><strong>${escapeText(event?.type || event?.kind || 'scene transition')}</strong><br><small>${escapeText(compactJson(event))}</small></div>`).join('')
      : '<p class="empty compact">No transition event rows exported.</p>';
    const declaredTransitionRows = Array.isArray(transition?.declaredTransitions) && transition.declaredTransitions.length
      ? transition.declaredTransitions.map((entry) => `<div class="surface-row"><strong>${escapeText(entry?.id || 'declared transition')}</strong><br><small>${escapeText(compactJson(entry))}</small></div>`).join('')
      : '<p class="empty compact">No manifest-validated declared transitions exported.</p>';
    const animationRows = Array.isArray(events?.animationEntities) && events.animationEntities.length
      ? events.animationEntities.map((entity) => `<li><strong>${escapeText(entity?.entityId || 'entity')}</strong> · ${escapeText(entity?.mode || 'mode unknown')} · clip ${escapeText(entity?.currentClip || 'none')} · frame ${escapeText(entity?.frameIndex ?? 'unknown')}</li>`).join('')
      : '<li>No animation event entity rows exported.</li>';
    const audioRows = Array.isArray(events?.audioEvents) && events.audioEvents.length
      ? events.audioEvents.map((event) => `<div class="surface-row"><strong>${escapeText(event?.type || event?.kind || 'audio event')}</strong><br><small>${escapeText(compactJson(event))}</small></div>`).join('')
      : '<p class="empty compact">No audio event rows exported.</p>';
    return `<section id="runtime-event-inspection" class="panel"><h2>Collision/transition/event inspection</h2>
      <p class="hint">Read-only runtime event summaries from Rust-exported evidence. This panel is inspection-only: it does not execute commands, mutate scenes, or persist browser state.</p>
      ${warnings.length ? `<div class="error">${escapeText(warnings.join(' · '))}</div>` : '<div class="hint">Collision/transition/event summaries loaded.</div>'}
      <div class="field-grid">
        <div><strong>Collision</strong><br>${escapeText(collision?.colliderEntityCount ?? 0)} collider(s), ${escapeText(collision?.collisionEventCount ?? 0)} event(s)</div>
        <div><strong>Transition</strong><br>${escapeText(transition?.currentSceneId ?? 'unknown scene')} · ${escapeText(transition?.declaredTransitionCount ?? 0)} declared · ${escapeText(transition?.transitionEventCount ?? 0)} event(s) · last reload ${escapeText(transition?.lastReloadStatus ?? 'none')}</div>
        <div><strong>Runtime events</strong><br>${escapeText(events?.animationEntityCount ?? 0)} animation entit(ies), ${escapeText(events?.audioEventCount ?? 0)} audio event(s), ${escapeText(events?.collisionEventCount ?? 0)} collision event(s)</div>
      </div>
      <h3>Collision rules</h3><div class="field-grid">${collisionRules}</div>
      <h3>Collision events</h3>${collisionRows}
      <h3>Declared scene transitions</h3>${declaredTransitionRows}
      <h3>Scene transition events</h3>${transitionRows}
      <h3>Animation entities</h3><ul>${animationRows}</ul>
      <h3>Audio events</h3>${audioRows}
    </section>`;
  }

  function renderRuntimeAssetLoadingSurface(run) {
    const loading = run?.asset_loading || run?.assetLoading || {};
    if (!loading.present) {
      return `<section id="runtime-asset-loading" class="panel"><h2>Runtime asset loading</h2><p class="empty">${escapeText(loading.empty_state || 'No runtime asset loading evidence is available for this run.')}</p><p class="hint">Read-only Studio surface. The browser does not fetch remote assets, upload files, write trusted state, or execute commands.</p></section>`;
    }
    const records = Array.isArray(loading.records) ? loading.records : [];
    const cards = [
      ['Attempts', loading.attempt_count ?? loading.attemptCount ?? records.length],
      ['Loaded', loading.loaded_count ?? loading.loadedCount ?? 0],
      ['Failed', loading.failed_count ?? loading.failedCount ?? 0],
      ['Rejected', loading.rejected_count ?? loading.rejectedCount ?? 0],
      ['Fallback', loading.fallback_count ?? loading.fallbackCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const rows = records.slice(0, 8).map((record) => {
      const assetId = record.assetId || record.asset_id || record.id || 'unknown asset';
      const status = record.status || 'unknown';
      const reason = record.failureReason || record.failure_reason ? ` · ${record.failureReason ?? record.failure_reason}` : '';
      const duration = record.loadDurationMs || record.load_duration_ms ? ` · ${record.loadDurationMs ?? record.load_duration_ms}ms` : '';
      return `<div class="surface-row"><strong>${escapeText(assetId)}</strong> ${surfaceState(Boolean(status), status)}<br><small>${escapeText(record.path || 'no path')} · ${escapeText(record.attemptId || record.attempt_id || 'attempt')}${escapeText(duration)}${escapeText(reason)}</small></div>`;
    }).join('') || '<div class="surface-row">No parsed runtime asset load records.</div>';
    return `<section id="runtime-asset-loading" class="panel"><h2>Runtime asset loading</h2>
      <p class="hint">Escaped read-only evidence from Rust-exported dashboard data. This surface does not load assets itself, fetch remote assets, upload files, write manifests/scenes, or execute commands.</p>
      <div class="field-grid">${cards}</div>${rows}
      <p class="hint">${escapeText(loading.boundary || 'Runtime loading evidence is display-only.')}</p>
    </section>`;
  }

  function assetInspectorAssetId(asset) {
    return asset.assetId || asset.asset_id || asset.id || 'unknown asset';
  }

  function renderStudioAssetInspectorSurface(run) {
    const inspector = run?.asset_inspector || run?.assetInspector || {};
    if (!inspector.present) {
      return `<section id="studio-asset-inspector" class="panel"><h2>Asset inspector</h2><p class="empty">${escapeText(inspector.empty_state || 'No asset inspector data is available for this run.')}</p><p class="hint">Read-only Studio surface. The browser does not upload assets, write manifests, fetch remote assets, or execute commands.</p></section>`;
    }
    const assets = Array.isArray(inspector.assets) ? inspector.assets : [];
    const preview = run?.asset_preview || run?.assetPreview || {};
    const previewRecords = Array.isArray(preview.records) ? preview.records : [];
    const loading = run?.asset_loading || run?.assetLoading || {};
    const loadRecords = Array.isArray(loading.records) ? loading.records : [];
    const refs = Array.isArray(inspector.evidence_refs || inspector.evidenceRefs) ? (inspector.evidence_refs || inspector.evidenceRefs) : [];
    const cards = [
      ['Status', inspector.status || 'unknown'],
      ['Assets', inspector.asset_count ?? inspector.assetCount ?? assets.length],
      ['Warnings', inspector.warning_count ?? inspector.warningCount ?? 0],
      ['Preview records', inspector.preview_count ?? inspector.previewCount ?? 0],
      ['Atlas frames', inspector.atlas_frame_count ?? inspector.atlasFrameCount ?? 0],
      ['Tilemaps', inspector.tilemap_count ?? inspector.tilemapCount ?? 0],
      ['Runtime attempts', inspector.runtime_attempt_count ?? inspector.runtimeAttemptCount ?? loadRecords.length],
      ['Loaded / failed', `${inspector.loaded_count ?? inspector.loadedCount ?? loading.loaded_count ?? loading.loadedCount ?? 0} / ${inspector.failed_count ?? inspector.failedCount ?? loading.failed_count ?? loading.failedCount ?? 0}`],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const rows = assets.slice(0, 12).map((asset) => {
      const runtimeStatuses = Array.isArray(asset.runtime_statuses || asset.runtimeStatuses) ? (asset.runtime_statuses || asset.runtimeStatuses).join(' | ') : '';
      const warnings = Array.isArray(asset.warnings) && asset.warnings.length ? ` · warnings ${asset.warnings.join(' | ')}` : '';
      const runtime = runtimeStatuses ? ` · runtime ${runtimeStatuses}` : '';
      const atlasFrames = asset.atlas_frame_count ?? asset.atlasFrameCount ?? 0;
      const tilemap = asset.tilemap ? ` · tilemap ${asset.tilemap.width ?? '?'}×${asset.tilemap.height ?? '?'}` : '';
      const hash = asset.contentHash || asset.content_hash || asset.hash || 'hash unrecorded in inspector row';
      return `<div class="surface-row"><strong>${escapeText(assetInspectorAssetId(asset))}</strong> ${surfaceState(true, asset.assetType || asset.asset_type || 'unknown')}<br><small>path ${escapeText(asset.sourcePath || asset.source_path || 'unrecorded')} · hash ${escapeText(hash)} · atlas frames ${escapeText(atlasFrames)}${escapeText(tilemap)}${escapeText(runtime)}${escapeText(warnings)}</small></div>`;
    }).join('') || '<div class="surface-row">No asset rows exported.</div>';
    const atlasRows = previewRecords.flatMap((record) => {
      const frames = Array.isArray(record.atlasFrames || record.atlas_frames) ? (record.atlasFrames || record.atlas_frames) : [];
      return frames.slice(0, 8).map((frame) => {
        const rect = frame.rect || {};
        return `<div class="surface-row"><strong>${escapeText(record.assetId || record.asset_id || 'atlas asset')}</strong> frame ${escapeText(frame.frameId || frame.frame_id || 'frame')}<br><small>rect ${escapeText(rect.x ?? '?')},${escapeText(rect.y ?? '?')} ${escapeText(rect.width ?? '?')}×${escapeText(rect.height ?? '?')}</small></div>`;
      });
    }).slice(0, 12).join('') || '<div class="surface-row">No atlas frame evidence exported.</div>';
    const tilemapRows = previewRecords.filter((record) => record.tilemap).slice(0, 8).map((record) => {
      const tilemap = record.tilemap || {};
      const tileset = tilemap.tilesetAssetId || tilemap.tileset_asset_id || 'unknown tileset';
      const layers = tilemap.layerCount ?? tilemap.layer_count ?? 0;
      const tiles = tilemap.tileCount ?? tilemap.tile_count ?? 0;
      return `<div class="surface-row"><strong>${escapeText(record.assetId || record.asset_id || 'tilemap asset')}</strong> ${surfaceState(true, 'tilemap')}<br><small>${escapeText(tilemap.width ?? '?')}×${escapeText(tilemap.height ?? '?')} · ${escapeText(layers)} layer(s) · ${escapeText(tiles)} tile(s) · tileset ${escapeText(tileset)}</small></div>`;
    }).join('') || '<div class="surface-row">No tilemap summary evidence exported.</div>';
    const loadRows = loadRecords.slice(0, 10).map((record) => {
      const status = record.status || 'unknown';
      const duration = record.loadDurationMs || record.load_duration_ms ? ` · ${record.loadDurationMs ?? record.load_duration_ms}ms` : '';
      const reason = record.failureReason || record.failure_reason ? ` · ${record.failureReason ?? record.failure_reason}` : '';
      return `<div class="surface-row"><strong>${escapeText(record.assetId || record.asset_id || 'runtime asset')}</strong> ${surfaceState(Boolean(status), status)}<br><small>${escapeText(record.path || 'no path')} · attempt ${escapeText(record.attemptId || record.attempt_id || 'unknown')}${escapeText(duration)}${escapeText(reason)}</small></div>`;
    }).join('') || '<div class="surface-row">No runtime load evidence exported.</div>';
    const refText = refs.length ? refs.slice(0, 6).join(' · ') : 'No inspector evidence refs recorded.';
    return `<section id="studio-asset-inspector" class="panel"><h2>Asset inspector</h2>
      <p class="hint">Read-only manifest/status panel from Rust-exported dashboard data. Copy commands manually if needed; this panel has no upload, write, fetch, or execute controls.</p>
      <div class="field-grid">${cards}</div>${rows}
      <h3>Atlas frame evidence</h3>${atlasRows}
      <h3>Tilemap evidence</h3>${tilemapRows}
      <h3>Runtime load evidence</h3>${loadRows}
      <p class="hint">Evidence refs: ${escapeText(refText)}</p>
      <p class="hint">${escapeText(inspector.boundary || 'Asset inspector data is display-only.')}</p>
    </section>`;
  }

  function renderAssetPreviewEvidenceSurface(run) {
    const preview = run?.asset_preview || run?.assetPreview || {};
    if (!preview.present) {
      return `<section id="asset-preview-evidence" class="panel"><h2>Asset preview evidence</h2><p class="empty">${escapeText(preview.empty_state || 'No asset preview evidence is available for this run.')}</p><p class="hint">Read-only Studio surface. The browser does not fetch remote assets, upload files, write trusted state, or execute commands.</p></section>`;
    }
    const records = Array.isArray(preview.records) ? preview.records : [];
    const warnings = Array.isArray(preview.warnings) ? preview.warnings : [];
    const cards = [
      ['Previews', preview.preview_count ?? preview.previewCount ?? records.length],
      ['Warnings', preview.warning_count ?? preview.warningCount ?? warnings.length],
      ['Images', preview.image_count ?? preview.imageCount ?? 0],
      ['Atlas frames', preview.atlas_frame_count ?? preview.atlasFrameCount ?? 0],
      ['Tilemaps', preview.tilemap_count ?? preview.tilemapCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const rows = records.slice(0, 8).map((record) => {
      const assetId = record.assetId || record.asset_id || 'unknown asset';
      const assetType = record.assetType || record.asset_type || 'unknown';
      const image = record.image ? ` · ${record.image.width ?? '?'}×${record.image.height ?? '?'}` : '';
      const atlasFrames = Array.isArray(record.atlasFrames || record.atlas_frames) ? (record.atlasFrames || record.atlas_frames).length : 0;
      const tilemap = record.tilemap ? ` · tilemap ${record.tilemap.width ?? '?'}×${record.tilemap.height ?? '?'}` : '';
      return `<div class="surface-row"><strong>${escapeText(assetId)}</strong> ${surfaceState(true, assetType)}<br><small>${escapeText(record.sourcePath || record.source_path || 'no source path')}${escapeText(image)}${atlasFrames ? ` · ${escapeText(atlasFrames)} frame(s)` : ''}${escapeText(tilemap)}</small></div>`;
    }).join('') || '<div class="surface-row">No parsed asset preview records.</div>';
    const warningText = warnings.length
      ? warnings.slice(0, 6).map((warning) => `${warning.assetId || warning.asset_id || 'manifest'}:${warning.kind || 'warning'}:${warning.message || ''}`).join(' · ')
      : 'No asset preview warnings recorded.';
    return `<section id="asset-preview-evidence" class="panel"><h2>Asset preview evidence</h2>
      <p class="hint">Escaped read-only preview metadata from Rust-exported dashboard data. This surface does not generate previews, load remote assets, upload files, write manifests/scenes, or execute commands.</p>
      <div class="field-grid">${cards}</div>${rows}
      <p class="hint">Warnings: ${escapeText(warningText)}</p>
      <p class="hint">${escapeText(preview.boundary || 'Asset preview evidence is display-only.')}</p>
    </section>`;
  }

  function renderSourceApplyWorktreeContextSurface(run) {
    const context = run?.source_apply_worktree_context || run?.sourceApplyWorktreeContext || {};
    if (!context.present) {
      return `<section id="source-apply-context" class="panel"><h2>Source apply context</h2><p class="empty">${escapeText(context.empty_state || 'No source apply worktree context evidence is available for this run.')}</p><p class="hint">Read-only Studio surface. The browser cannot apply source patches, execute commands, write trusted files, merge branches, or bypass review gates.</p></section>`;
    }
    const reports = Array.isArray(context.reports) ? context.reports : [];
    const cards = [
      ['Status', context.status || 'unknown'],
      ['Reports', reports.length],
      ['Targets', context.target_count ?? context.targetCount ?? 0],
      ['Blocked reasons', context.blocked_count ?? context.blockedCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const rows = reports.slice(0, 4).map((report) => {
      const lock = report.lockStatus || report.lock_status || {};
      const blocked = Array.isArray(report.blockedReasons || report.blocked_reasons) ? (report.blockedReasons || report.blocked_reasons) : [];
      const targetRows = (Array.isArray(report.targets) ? report.targets : []).slice(0, 8).map((target) => {
        const reasons = Array.isArray(target.blockedReasons || target.blocked_reasons) ? (target.blockedReasons || target.blocked_reasons) : [];
        return `<li><strong>${escapeText(target.path || 'unknown target')}</strong> ${surfaceState(true, reasons.length ? 'blocked' : 'clean')}<br><small>${escapeText(target.gitStatus || target.git_status || 'unknown git')} · ${escapeText(target.rootZone || target.root_zone || 'unknown root')} · ${escapeText(target.fileClassDecision || target.file_class_decision || 'unknown class')} · ${escapeText(reasons.length ? reasons.join(' · ') : 'no blocked target reasons')}</small></li>`;
      }).join('') || '<li>No target rows recorded.</li>';
      const blockedText = blocked.length ? blocked.slice(0, 8).join(' · ') : 'No blocked context reasons recorded.';
      return `<div class="surface-row"><strong>${escapeText(report.policyId || report.policy_id || 'source apply context')}</strong> ${surfaceState(true, report.status || 'unknown')}<br>
        <small>${escapeText(report.branch || 'unknown branch')} @ ${escapeText(report.headCommit || report.head_commit || 'unknown head')} · lock ${escapeText(lock.active ? 'active' : 'inactive')} ${escapeText(lock.attemptId || lock.attempt_id || '')}</small>
        <ul>${targetRows}</ul>
        <small>Blocked: ${escapeText(blockedText)}</small>
      </div>`;
    }).join('') || '<div class="surface-row">No parseable context reports recorded.</div>';
    return `<section id="source-apply-context" class="panel"><h2>Source apply context</h2>
      <p class="hint">Escaped read-only worktree eligibility evidence from Rust validation. This surface does not apply patches, execute commands, write trusted files, merge branches, or bypass review gates.</p>
      <div class="field-grid">${cards}</div>${rows}
      <p class="hint">${escapeText(context.boundary || 'Source apply context evidence is display-only.')}</p>
    </section>`;
  }


  function visualDiffList(value, fallback = []) {
    return Array.isArray(value) ? value : fallback;
  }

  function visualDiffSummaryCount(summary, key, camelKey) {
    const collection = summary?.[key] || summary?.[camelKey] || [];
    return Array.isArray(collection) ? collection.length : 0;
  }

  function visualDiffCollisionTriggerCount(summary, key, camelKey) {
    const value = summary?.collisionTriggerSummary || summary?.collision_trigger_summary || {};
    return value?.[key] ?? value?.[camelKey] ?? 0;
  }

  function renderVisualDiffPreviewSurface(run) {
    const preview = run?.visual_diff_preview || run?.visualDiffPreview || {};
    const summaries = visualDiffList(preview.summaries || preview.records || preview.visualDiffSummaries || preview.visual_diff_summaries);
    if (!preview.present && summaries.length === 0) {
      return `<section id="visual-diff-preview" class="panel"><h2>Visual diff preview</h2><p class="empty">${escapeText(preview.empty_state || 'No visual diff summary read model is available for this run.')}</p><p class="hint">Read-only Studio surface. The browser does not apply visual edits, write files, execute commands, or persist draft state.</p></section>`;
    }
    const operationCount = summaries.reduce((total, summary) => total + visualDiffSummaryCount(summary, 'operationSummaries', 'operation_summaries'), 0);
    const entityCount = summaries.reduce((total, summary) => total + visualDiffSummaryCount(summary?.after, 'entitySummaries', 'entity_summaries'), 0);
    const tileCount = summaries.reduce((total, summary) => total + visualDiffSummaryCount(summary?.after, 'tileSummaries', 'tile_summaries'), 0);
    const assetCount = summaries.reduce((total, summary) => total + visualDiffSummaryCount(summary?.after, 'assetSummaries', 'asset_summaries'), 0);
    const collisionCount = summaries.reduce((total, summary) => total + Number(visualDiffCollisionTriggerCount(summary?.after, 'collisionCellsAffected', 'collision_cells_affected') || 0), 0);
    const triggerCount = summaries.reduce((total, summary) => total + Number(visualDiffCollisionTriggerCount(summary?.after, 'triggerCellsAffected', 'trigger_cells_affected') || 0), 0);
    const cards = [
      ['Summaries', preview.summary_count ?? preview.summaryCount ?? summaries.length],
      ['Operations', preview.operation_count ?? preview.operationCount ?? operationCount],
      ['Entity rows', preview.entity_count ?? preview.entityCount ?? entityCount],
      ['Tile rows', preview.tile_count ?? preview.tileCount ?? tileCount],
      ['Asset rows', preview.asset_count ?? preview.assetCount ?? assetCount],
      ['Collision / trigger', `${collisionCount} / ${triggerCount}`],
      ['Status', preview.status || 'preview-only'],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const rows = summaries.slice(0, 8).map((summary) => {
      const target = summary.target || {};
      const sourceRefs = summary.sourceRefs || summary.source_refs || {};
      const before = summary.before || {};
      const after = summary.after || {};
      const operations = visualDiffList(summary.operationSummaries || summary.operation_summaries);
      const opRows = operations.slice(0, 6).map((operation) => {
        const collision = operation.collisionTriggerSummary || operation.collision_trigger_summary || {};
        const links = [
          operation.transactionId || operation.transaction_id ? `tx ${operation.transactionId || operation.transaction_id}` : null,
          operation.path ? `path ${operation.path}` : null,
          (operation.affectedEntityIds || operation.affected_entity_ids || []).length ? `entities ${(operation.affectedEntityIds || operation.affected_entity_ids).join(', ')}` : null,
          (operation.affectedTilemapIds || operation.affected_tilemap_ids || []).length ? `tilemaps ${(operation.affectedTilemapIds || operation.affected_tilemap_ids).join(', ')}` : null,
          (operation.affectedAssetIds || operation.affected_asset_ids || []).length ? `assets ${(operation.affectedAssetIds || operation.affected_asset_ids).join(', ')}` : null,
          collision.collisionCellsAffected || collision.collision_cells_affected ? `collision ${collision.collisionCellsAffected || collision.collision_cells_affected}` : null,
          collision.triggerCellsAffected || collision.trigger_cells_affected ? `trigger ${collision.triggerCellsAffected || collision.trigger_cells_affected}` : null,
        ].filter(Boolean).join(' · ');
        return `<li><strong>${escapeText(operation.operationId || operation.operation_id || 'operation')}</strong> ${surfaceState(true, operation.change || 'preview')}<br><small>${escapeText(operation.summary || 'No operation summary recorded.')}${links ? `<br>${escapeText(links)}` : ''}</small></li>`;
      }).join('') || '<li>No operation summaries exported.</li>';
      const refs = [
        sourceRefs.draftId || sourceRefs.draft_id ? `draft ${sourceRefs.draftId || sourceRefs.draft_id}` : null,
        sourceRefs.transactionId || sourceRefs.transaction_id ? `transaction ${sourceRefs.transactionId || sourceRefs.transaction_id}` : null,
        sourceRefs.proposalId || sourceRefs.proposal_id ? `proposal ${sourceRefs.proposalId || sourceRefs.proposal_id}` : null,
        sourceRefs.journalRef || sourceRefs.journal_ref ? `journal ${sourceRefs.journalRef || sourceRefs.journal_ref}` : null,
        sourceRefs.dashboardRef || sourceRefs.dashboard_ref ? `dashboard ${sourceRefs.dashboardRef || sourceRefs.dashboard_ref}` : null,
      ].filter(Boolean).join(' · ') || 'No source refs recorded.';
      const targetText = `${target.type || target.target_type || 'target'} ${target.id || ''} ${target.path || ''}`.trim();
      const impact = summary.expectedScenarioImpact || summary.expected_scenario_impact || {};
      const impactText = impact.status ? `${impact.status}: ${impact.summary || ''}` : 'Scenario impact requires separate evidence.';
      return `<div class="surface-row"><strong>${escapeText(summary.summaryId || summary.summary_id || 'visual diff summary')}</strong> ${surfaceState(true, target.type || target.target_type || 'preview')}<br>
        <small>${escapeText(targetText)}<br>Before: ${escapeText(before.summaryText || before.summary_text || 'No before summary.')}<br>After: ${escapeText(after.summaryText || after.summary_text || 'No after summary.')}<br>Refs: ${escapeText(refs)}<br>Scenario impact: ${escapeText(impactText)}</small>
        <ul>${opRows}</ul>
      </div>`;
    }).join('') || '<div class="surface-row">No visual diff summary records exported.</div>';
    return `<section id="visual-diff-preview" class="panel"><h2>Visual diff preview</h2>
      <p class="hint">Escaped read-only visual diff summaries from Rust-generated draft/transaction evidence. This panel has no apply buttons, trusted writes, command execution, local server bridge, or browser persistence.</p>
      <div class="field-grid">${cards}</div>${rows}
      <p class="hint">${escapeText(preview.boundary || 'Visual diff previews are display-only; trusted writes remain Rust CLI review-gated.')}</p>
    </section>`;
  }


  function studioDraftAuthoringState(run) {
    const source = run?.studio_draft_authoring || run?.studioDraftAuthoring || null;
    if (!source || typeof source !== 'object' || Array.isArray(source)) {
      return {
        present: false,
        boundary: 'Studio draft authoring state is browser-memory only; no trusted writes or command execution.',
        drafts: [],
        readError: null,
      };
    }
    const rawDrafts = Array.isArray(source.drafts) ? source.drafts : Array.isArray(source.records) ? source.records : [];
    const drafts = rawDrafts.map((draft, index) => {
      if (!draft || typeof draft !== 'object' || Array.isArray(draft)) {
        return {
          draftId: `malformed-draft-${index + 1}`,
          target: { type: 'malformed', path: 'unavailable' },
          proposedOperations: [],
          validationStatus: 'blocked',
          blockedReasons: ['malformed draft read-model entry'],
          malformed: true,
          original: draft,
        };
      }
      const target = draft.target && typeof draft.target === 'object' && !Array.isArray(draft.target)
        ? draft.target
        : { type: 'unknown', path: 'unknown' };
      const operations = Array.isArray(draft.proposedOperations)
        ? draft.proposedOperations
        : Array.isArray(draft.proposed_operations)
          ? draft.proposed_operations
          : [];
      const blockedReasons = Array.isArray(draft.blockedReasons)
        ? draft.blockedReasons
        : Array.isArray(draft.blocked_reasons)
          ? draft.blocked_reasons
          : [];
      return {
        ...draft,
        draftId: draft.draftId || draft.draft_id || `studio-draft-${index + 1}`,
        target,
        proposedOperations: operations,
        validationStatus: draft.validationStatus || draft.validation_status || 'unvalidated',
        blockedReasons,
        expectedAfterSummary: draft.expectedAfterSummary || draft.expected_after_summary || '',
        linkedEvidence: Array.isArray(draft.linkedEvidence) ? draft.linkedEvidence : Array.isArray(draft.linked_evidence) ? draft.linked_evidence : [],
        reviewGate: draft.reviewGate || draft.review_gate || null,
        author: draft.author && typeof draft.author === 'object' ? draft.author : null,
      };
    });
    return {
      present: Boolean(source.present ?? drafts.length),
      boundary: source.boundary || 'Studio draft authoring state is temporary browser/read-model data only; copy JSON/commands manually and run trusted CLI outside the browser.',
      drafts,
      readError: typeof source.read_error === 'string' ? source.read_error : typeof source.readError === 'string' ? source.readError : null,
    };
  }

  function studioDraftPreviewCommand(draft, run) {
    const project = projectContext(run);
    const manifest = project?.manifestPath || draft?.projectPath || '<project-manifest-or-root>';
    const draftPath = draft?.draftPath || '<draft-json>';
    return `cargo run -p ouroforge-cli -- edit draft-preview ${draftPath} --project ${manifest}`;
  }

  function studioDraftControlModel(draft) {
    const targetType = draft?.target?.type || draft?.target?.target_type || 'unknown';
    const operations = Array.isArray(draft?.proposedOperations) ? draft.proposedOperations : [];
    const blockedReasons = Array.isArray(draft?.blockedReasons) ? draft.blockedReasons : [];
    const validationStatus = draft?.validationStatus || 'unvalidated';
    const common = {
      targetType,
      validationStatus,
      disabled: true,
      blockedReasons,
      boundary: 'Display-only bounded controls; Studio does not persist drafts, write trusted files, execute commands, or apply edits.',
      controls: [],
    };
    if (targetType === 'scene') {
      return {
        ...common,
        kind: 'scene-draft-controls',
        label: 'Scene draft controls',
        controls: operations.slice(0, 4).map((operation, index) => ({
          id: operation.id || `scene-operation-${index + 1}`,
          label: operation.summary || operation.kind || 'Scene operation preview',
          field: operation.path || 'scene field',
          value: operation.value ?? operation.afterValue ?? operation.after_value ?? 'pending trusted preview',
          hint: 'Scene edits remain preview-only until copied to the Rust CLI transaction flow.',
        })),
      };
    }
    if (targetType === 'tilemap') {
      return {
        ...common,
        kind: 'tilemap-draft-controls',
        label: 'Tilemap draft controls',
        controls: operations.slice(0, 4).map((operation, index) => ({
          id: operation.id || `tilemap-operation-${index + 1}`,
          label: operation.summary || operation.kind || 'Tilemap operation preview',
          field: operation.path || operation.cell || 'tilemap cell/layer',
          value: operation.tileId || operation.tile_id || operation.value || 'pending trusted preview',
          hint: 'Tilemap edits remain preview-only until copied to the Rust CLI bounds/hash preflight.',
        })),
      };
    }
    if (targetType === 'asset-reference') {
      return {
        ...common,
        kind: 'asset-reference-draft-controls',
        label: 'Asset-reference draft controls',
        boundary: 'Display-only asset-reference controls; Studio does not upload assets, fetch remote assets, write manifests, execute commands, or apply edits.',
        controls: operations.slice(0, 4).map((operation, index) => ({
          id: operation.id || `asset-reference-operation-${index + 1}`,
          label: operation.summary || operation.kind || 'Asset-reference operation preview',
          field: operation.path || operation.assetPath || operation.asset_path || 'asset manifest reference',
          value: operation.assetId || operation.asset_id || operation.value || operation.afterValue || operation.after_value || draft?.target?.id || 'pending trusted preview',
          hint: 'Asset-reference edits remain inert until copied to the Rust CLI asset preflight/review flow.',
        })),
      };
    }
    return {
      ...common,
      kind: 'unsupported-draft-controls',
      label: 'Draft controls unavailable',
      blockedReasons: blockedReasons.length ? blockedReasons : [`${targetType} controls are outside this PR unit`],
    };
  }

  function renderStudioDraftControls(draft) {
    const model = studioDraftControlModel(draft);
    if (!['scene-draft-controls', 'tilemap-draft-controls', 'asset-reference-draft-controls'].includes(model.kind)) {
      return `<p class="hint">${escapeText(model.label)}: ${escapeText(model.blockedReasons.join('; '))}</p>`;
    }
    const controlRows = model.controls.map((control) => `<label>${escapeText(control.label)}<br><input type="text" value="${escapeText(control.value)}" disabled readonly data-draft-field="${escapeText(control.field)}"><small>${escapeText(control.field)} · ${escapeText(control.hint)}</small></label>`).join('') || '<p class="empty">No bounded controls are available for this draft.</p>';
    const blocked = model.blockedReasons.length
      ? `<p class="warn">Blocked state: ${escapeText(model.blockedReasons.join('; '))}</p>`
      : `<p class="hint">Validation status: ${escapeText(model.validationStatus)}; controls remain disabled and copy-only.</p>`;
    return `<fieldset class="draft-controls" disabled data-draft-control="${escapeText(model.targetType)}"><legend>${escapeText(model.label)}</legend>${controlRows}${blocked}<p class="hint">${escapeText(model.boundary)}</p></fieldset>`;
  }

  function renderStudioDraftAuthoringSurface(run) {
    const state = studioDraftAuthoringState(run);
    if (!state.present && !state.drafts.length) {
      return `<section id="studio-draft-authoring" class="panel"><h2>Studio draft authoring</h2><p class="empty">No Studio draft authoring read model is loaded yet.</p><p class="hint">${escapeText(state.boundary)}</p></section>`;
    }
    const readError = state.readError ? `<p class="error">${escapeText(state.readError)}</p>` : '';
    const rows = state.drafts.map((draft) => {
      const operationRows = draft.proposedOperations.slice(0, 4).map((operation) => `<li>${escapeText(operation.id || 'operation')} · ${escapeText(operation.kind || 'unknown')} · ${escapeText(operation.path || 'unknown path')}<br><small>${escapeText(operation.summary || '')}</small></li>`).join('') || '<li>No proposed operations recorded.</li>';
      const blocked = draft.blockedReasons.length
        ? `<p class="warn">Blocked: ${escapeText(draft.blockedReasons.join('; '))}</p>`
        : '<p class="hint">No blocked reasons recorded.</p>';
      const reviewGate = draft.reviewGate && typeof draft.reviewGate === 'object'
        ? `<p class="hint">Review gate: proposal ${escapeText(draft.reviewGate.proposalId || draft.reviewGate.proposal_id || 'unlinked')} · patch ${escapeText(draft.reviewGate.patchDraftId || draft.reviewGate.patch_draft_id || 'unlinked')} · decision ${escapeText(draft.reviewGate.reviewDecisionId || draft.reviewGate.review_decision_id || 'unlinked')}</p>`
        : '<p class="hint">No review gate is implied by this temporary draft state.</p>';
      const draftJson = JSON.stringify({
        schemaVersion: draft.schemaVersion || draft.schema_version || 'visual-edit-draft-v1',
        draftId: draft.draftId,
        target: draft.target,
        proposedOperations: draft.proposedOperations,
        beforeHash: draft.beforeHash || draft.before_hash || 'unknown',
        expectedAfterSummary: draft.expectedAfterSummary,
        linkedEvidence: draft.linkedEvidence,
        reviewGate: draft.reviewGate || undefined,
        author: draft.author || { type: 'studio', id: 'browser-memory', source: 'in-memory' },
        validationStatus: draft.validationStatus,
        blockedReasons: draft.blockedReasons,
      }, null, 2);
      return `<article class="surface-row">
        <strong>${escapeText(draft.draftId)}</strong> ${surfaceState(draft.validationStatus !== 'blocked' && !draft.malformed, draft.validationStatus)}
        <br><small>target ${escapeText(draft.target?.type || 'unknown')} · ${escapeText(draft.target?.path || 'unknown path')} · ${escapeText(draft.target?.id || 'no id')}</small>
        <p>${escapeText(draft.expectedAfterSummary || 'No after summary recorded.')}</p>
        ${blocked}
        ${reviewGate}
        <h4>Proposed operations</h4><ul>${operationRows}</ul>
        <h4>Bounded draft controls</h4>${renderStudioDraftControls(draft)}
        <h4>Copyable draft JSON</h4><pre>${escapeText(draftJson)}</pre>
        <h4>Copyable CLI preview command</h4><code>${escapeText(studioDraftPreviewCommand(draft, run))}</code>
      </article>`;
    }).join('');
    return `<section id="studio-draft-authoring" class="panel"><h2>Studio draft authoring</h2>
      <p class="hint">${escapeText(state.boundary)} The cockpit renders temporary draft data and inert copyable text only; it does not write trusted files, persist browser draft state, execute commands, upload assets, or apply edits.</p>
      ${readError}
      ${rows}
    </section>`;
  }

  function tilemapDraftCellSummary(cells) {
    if (!Array.isArray(cells) || cells.length === 0) return 'none';
    return cells.slice(0, 3).map((cell) => {
      if (!cell || typeof cell !== 'object' || Array.isArray(cell)) return 'malformed cell';
      const layer = cell.layerId || cell.layer_id || 'unknown layer';
      const x = cell.x ?? '?';
      const y = cell.y ?? '?';
      const tile = cell.tileId || cell.tile_id || 'unknown tile';
      const trigger = cell.trigger ? ` trigger ${cell.trigger}` : '';
      return `${layer}[${x},${y}] ${tile}${trigger}`;
    }).join('; ');
  }

  function renderTilemapDraftControl(record) {
    const blockedReasons = Array.isArray(record.blockedReasons)
      ? record.blockedReasons
      : Array.isArray(record.blocked_reasons)
        ? record.blocked_reasons
        : [];
    const status = record.validationStatus || record.validation_status || record.status || 'preview-only';
    const blocked = blockedReasons.length || status === 'blocked';
    const targetPath = record.tilemapPath || record.tilemap_path || record.targetPath || record.target_path || '<tilemap-json>';
    const layer = record.layerId || record.layer_id || 'unknown';
    const collisionCells = record.collisionCells || record.collision_cells || [];
    const triggerCells = record.triggerCells || record.trigger_cells || [];
    const x = record.x ?? record.cellX ?? record.cell_x ?? (Array.isArray(collisionCells) && collisionCells[0]?.x) ?? (Array.isArray(triggerCells) && triggerCells[0]?.x) ?? '';
    const y = record.y ?? record.cellY ?? record.cell_y ?? (Array.isArray(collisionCells) && collisionCells[0]?.y) ?? (Array.isArray(triggerCells) && triggerCells[0]?.y) ?? '';
    const tileId = record.tileId || record.tile_id || (Array.isArray(collisionCells) && collisionCells[0]?.tileId) || (Array.isArray(triggerCells) && triggerCells[0]?.tileId) || '';
    const draftJson = JSON.stringify({
      schemaVersion: record.schemaVersion || record.schema_version || 'tilemap-draft-control-v1',
      operationId: record.operationId || record.operation_id || 'tilemap operation',
      kind: record.kind || 'preview',
      target: { type: 'tilemap', path: targetPath, layerId: layer },
      cell: { x, y, tileId },
      collisionCells: Array.isArray(collisionCells) ? collisionCells : [],
      triggerCells: Array.isArray(triggerCells) ? triggerCells : [],
      validationStatus: status,
      blockedReasons,
    }, null, 2);
    const blockedMarkup = blocked
      ? `<p class="warn">Blocked: ${escapeText(blockedReasons.join('; ') || status)}</p>`
      : '<p class="hint">Draft controls are preview-only and require Rust validation plus review-gated apply outside the browser.</p>';
    return `<div class="tilemap-draft-control" data-draft-control="tilemap">
      <h4>Inert tilemap draft controls</h4>
      ${blockedMarkup}
      <label>Layer <input type="text" readonly value="${escapeText(layer)}"></label>
      <label>X <input type="text" readonly value="${escapeText(x)}"></label>
      <label>Y <input type="text" readonly value="${escapeText(y)}"></label>
      <label>Tile <input type="text" readonly value="${escapeText(tileId || 'unselected')}"></label>
      <pre>${escapeText(draftJson)}</pre>
      <p class="hint">Preview only — no browser apply control is rendered.</p>
    </div>`;
  }

  function renderTilemapDraftPreviewSurface(run) {
    const preview = run?.tilemap_draft_preview || run?.tilemapDraftPreview || {};
    if (!preview.present) {
      return `<section id="tilemap-draft-preview" class="panel"><h2>Tilemap draft previews</h2><p class="empty">${escapeText(preview.empty_state || 'No tilemap draft preview read model is available for this run.')}</p><p class="hint">Read-only Studio surface. The browser does not write tilemaps, execute commands, or apply draft previews.</p></section>`;
    }
    const records = Array.isArray(preview.records) ? preview.records : [];
    const collisionCount = records.reduce((total, record) => {
      const cells = record.collisionCells || record.collision_cells || [];
      return total + (Array.isArray(cells) ? cells.length : 0);
    }, 0);
    const triggerCount = records.reduce((total, record) => {
      const cells = record.triggerCells || record.trigger_cells || [];
      return total + (Array.isArray(cells) ? cells.length : 0);
    }, 0);
    const blockedCount = records.reduce((total, record) => {
      const reasons = record.blockedReasons || record.blocked_reasons || [];
      return total + ((Array.isArray(reasons) && reasons.length) || record.validationStatus === 'blocked' || record.validation_status === 'blocked' ? 1 : 0);
    }, 0);
    const cards = [
      ['Previews', preview.preview_count ?? preview.previewCount ?? records.length],
      ['Collision cells', preview.collision_cell_count ?? preview.collisionCellCount ?? collisionCount],
      ['Trigger cells', preview.trigger_cell_count ?? preview.triggerCellCount ?? triggerCount],
      ['Blocked drafts', preview.blocked_count ?? preview.blockedCount ?? blockedCount],
      ['Status', preview.status || 'preview-only'],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const rows = records.slice(0, 10).map((record) => {
      const collisionCells = record.collisionCells || record.collision_cells || [];
      const triggerCells = record.triggerCells || record.trigger_cells || [];
      const beforeHash = record.beforeTilemapHash || record.before_tilemap_hash || {};
      const afterHash = record.afterTilemapHash || record.after_tilemap_hash || {};
      const beforeText = beforeHash.algorithm && beforeHash.value ? `${beforeHash.algorithm}:${beforeHash.value}` : 'before hash unrecorded';
      const afterText = afterHash.algorithm && afterHash.value ? `${afterHash.algorithm}:${afterHash.value}` : 'after hash unrecorded';
      const cellText = `collision ${tilemapDraftCellSummary(collisionCells)} · trigger ${tilemapDraftCellSummary(triggerCells)}`;
      return `<div class="surface-row"><strong>${escapeText(record.operationId || record.operation_id || 'tilemap operation')}</strong> ${surfaceState(true, record.kind || 'preview')}<br><small>layer ${escapeText(record.layerId || record.layer_id || 'unknown')} · affected ${escapeText(record.affectedCells ?? record.affected_cells ?? 0)} cell(s) · collision ${escapeText(Array.isArray(collisionCells) ? collisionCells.length : 0)} · trigger ${escapeText(Array.isArray(triggerCells) ? triggerCells.length : 0)}<br>${escapeText(record.summary || 'No summary recorded.')}<br>${escapeText(cellText)}<br>${escapeText(beforeText)} → ${escapeText(afterText)}</small>${renderTilemapDraftControl(record)}</div>`;
    }).join('') || '<div class="surface-row">No tilemap draft preview records exported.</div>';
    return `<section id="tilemap-draft-preview" class="panel"><h2>Tilemap draft previews</h2>
      <p class="hint">Escaped read-only tilemap draft preview data from Rust-exported evidence. This panel renders inert draft controls only; it cannot write tilemaps, execute local commands, apply reviews, or persist browser state.</p>
      <div class="field-grid">${cards}</div>${rows}
      <p class="hint">${escapeText(preview.boundary || 'Tilemap draft previews are display-only and must stay review-gated before apply.')}</p>
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
    const visualApplied = mutationStage(lifecycle, 'visual_draft_applied');
    const runDir = run?.summary?.run_dir || (run?.summary?.id ? `runs/${run.summary.id}` : 'runs/run-1');
    const proposedRecords = Array.isArray(proposed?.records) ? proposed.records : [];
    const appliedRecords = Array.isArray(applied?.records) ? applied.records : [];
    const visualAppliedRecords = Array.isArray(visualApplied?.records) ? visualApplied.records : [];
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
    const visualApplicationRows = visualAppliedRecords.slice(0, 3).map((record) => {
      const command = record.commandContext?.command || 'no command context recorded';
      return `<div class="surface-row"><strong>${escapeText(record.id || 'visual draft application')}</strong> ${surfaceState(record.status !== 'failed', record.status || 'applied')}<br><small>draft ${escapeText(record.draftId || 'unknown')} · proposal ${escapeText(record.proposalId || 'unknown')} · patch draft ${escapeText(record.patchDraftId || 'unknown')}</small><br><small>review decision ${escapeText(record.reviewDecisionId || 'unknown')} · transaction ${escapeText(record.transactionId || 'unknown')}</small><br><small>${escapeText(record.beforeSceneHash?.value || 'before unknown')} → ${escapeText(record.afterSceneHash?.value || 'after unknown')}</small><br><small>display-only rerun context: ${escapeText(command)}</small></div>`;
    }).join('') || '<p class="empty compact">No visual draft application records loaded yet.</p>';
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
        <div><strong>Visual draft application stage</strong><br>${surfaceState(Boolean(visualApplied && visualApplied.state !== 'missing'), visualApplied?.state || 'missing')}<br><small>${escapeText(visualApplied?.record_count || 0)} record(s)</small></div>
        <div><strong>Project-scoped applications</strong><br>${escapeText(projectMutationRecords.length)} record(s)</div>
        <div><strong>Target scene</strong><br>${escapeText(targetScenePath)}</div>
        <div><strong>Project manifest</strong><br>${escapeText(projectPath || 'legacy/no project context')}</div>
      </div>
      <h4>Proposal context</h4><ul>${proposedRows}</ul>
      <h4>Application records</h4>${applicationRows}
      <h4>Visual draft application records</h4>${visualApplicationRows}
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
      <div class="surface-row"><strong>${escapeText(summary.loopId || 'unknown')}</strong> step <strong>${escapeText(summary.stepId || 'unknown')}</strong> ${surfaceState(Boolean(summary.status) && summary.status !== 'missing', summary.status || 'unknown')}<br><small>${escapeText(summary.kind || 'unknown')} · ledger ${escapeText(summary.ledgerPath || 'unrecorded')}</small></div>
      ${blocked.length ? `<div class="hint">Blocked by: ${escapeText(blocked.join(' · '))}</div>` : '<p class="hint">No blocked reasons reported.</p>'}
      ${rows}
      ${summary.boundary ? `<p class="hint">${escapeText(summary.boundary)}</p>` : ''}
    </section>`;
  }

  function normalizeLoopEvidenceBundles(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function normalizeAgentHandoffs(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function normalizeStudioLoopCockpit(value = null) {
    if (!value || typeof value !== 'object' || Array.isArray(value)) {
      return { schemaVersion: null, loops: [], boundary: null, warning: 'No loop cockpit read-model is attached to dashboard-data.json.' };
    }
    if (!Array.isArray(value.loops)) {
      return { schemaVersion: value.schemaVersion || null, loops: [], boundary: value.boundary || null, warning: 'Malformed loop cockpit read-model: loops must be an array.' };
    }
    return { schemaVersion: value.schemaVersion || null, loops: value.loops, boundary: value.boundary || null, warning: null };
  }

  function renderLoopCockpitTimeline(loop) {
    const steps = Array.isArray(loop?.steps) ? loop.steps : [];
    if (!steps.length) return '<p class="empty compact">No loop plan timeline steps exported.</p>';
    return `<ol class="timeline">${steps.map((step) => `<li><strong>${escapeText(step.stepId || step.id || 'step')}</strong> ${surfaceState(Boolean(step.status), step.status || 'unknown')}<br><small>${escapeText(step.kind || 'unknown')} · ${escapeText(step.path || step.artifactPath || 'no artifact path')}</small></li>`).join('')}</ol>`;
  }

  function renderStudioLoopCockpitSurface(run) {
    const cockpit = normalizeStudioLoopCockpit(run?.loop_cockpit || run?.loopCockpit || null);
    if (!cockpit.loops.length) {
      return `<section id="loop-cockpit" class="panel"><h2>Loop cockpit</h2><p class="empty">${escapeText(cockpit.warning || 'No loop cockpit read-model loops are available.')}</p><p class="hint">Read-only Studio loop cockpit. The browser does not execute commands, write files, resume loops, apply mutations, or promote regressions.</p></section>`;
    }
    const rows = cockpit.loops.map((loop) => {
      const blockers = Array.isArray(loop.blockers) ? loop.blockers : [];
      const decisions = Array.isArray(loop.requiredDecisions) ? loop.requiredDecisions : [];
      const allowed = Array.isArray(loop.allowedCommands) ? loop.allowedCommands : [];
      const forbidden = Array.isArray(loop.forbiddenActions) ? loop.forbiddenActions : [];
      const evidence = Array.isArray(loop.evidenceRefs) ? loop.evidenceRefs : [];
      const missing = Array.isArray(loop.bundleMissingRefs) ? loop.bundleMissingRefs : [];
      const current = loop.currentStep && typeof loop.currentStep === 'object' ? loop.currentStep : null;
      const status = loop.status || loop.bundleStatus || loop.handoffStatus || 'unknown';
      const commandText = allowed.map((command) => command.command || '').filter(Boolean).join(' · ');
      const evidenceText = evidence.map((ref) => `${ref.id || 'ref'}:${ref.path || 'missing'}`).join(' · ');
      return `<div class="surface-row"><strong>${escapeText(loop.loopId || 'unknown-loop')}</strong> ${surfaceState(Boolean(status), status)}<br>
        <small>Plan: ${escapeText(loop.planPath || 'unrecorded')} · bundle ${escapeText(loop.bundleStatus || 'unknown')} · handoff ${escapeText(loop.handoffStatus || 'unknown')}</small>
        <div class="hint">Current step: ${escapeText(current?.stepId || 'none')} · ${escapeText(current?.kind || 'unknown')} · ${escapeText(current?.status || 'unknown')}</div>
        ${renderLoopCockpitTimeline(loop)}
        <div class="hint">Next safe action: ${escapeText(loop.nextSafeAction || 'unrecorded')}</div>
        ${blockers.length ? `<div class="hint">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="hint">No blockers reported.</div>'}
        ${decisions.length ? `<div class="hint">Required decisions: ${escapeText(decisions.map((decision) => `${decision.id || 'decision'}:${decision.kind || 'unknown'}`).join(' · '))}</div>` : '<div class="hint">No required decisions reported.</div>'}
        <div class="hint">Allowed command text: ${escapeText(commandText || 'none')}</div>
        <div class="hint">Forbidden actions: ${escapeText(forbidden.join(' · ') || 'none')}</div>
        <div class="hint">Evidence refs: ${escapeText(evidenceText || 'none')}</div>
        ${missing.length ? `<div class="hint">Missing/stale bundle refs: ${escapeText(missing.join(' · '))}</div>` : '<div class="hint">No missing bundle refs reported.</div>'}
        <small>${escapeText(loop.boundary || 'Display-only loop cockpit row; no browser authority.')}</small>
      </div>`;
    }).join('');
    return `<section id="loop-cockpit" class="panel"><h2>Loop cockpit</h2>
      <p class="hint">Read-only loop plan/status timeline. Copyable or next-action text stays inert; the browser does not execute commands or write trusted state.</p>
      <div class="hint">Schema: ${escapeText(cockpit.schemaVersion || 'unknown')} · ${escapeText(cockpit.boundary || 'No boundary text exported.')}</div>
      ${rows}
    </section>`;
  }

  function renderAgentHandoffSurface(run) {
    const handoffs = normalizeAgentHandoffs(run?.agent_handoffs || run?.agentHandoffs || run?.agent_handoff || run?.agentHandoff || null);
    if (!handoffs.length) {
      return '<section id="agent-handoff" class="panel"><h2>Agent handoff</h2><p class="empty">No agent handoff is attached to dashboard-data.json. Generate one with the Rust CLI and keep it under local generated state.</p></section>';
    }
    const rows = handoffs.map((handoff) => {
      const blockers = Array.isArray(handoff.blockers) ? handoff.blockers : [];
      const decisions = Array.isArray(handoff.requiredDecisions) ? handoff.requiredDecisions : [];
      const allowed = Array.isArray(handoff.allowedCommands) ? handoff.allowedCommands : [];
      const forbidden = Array.isArray(handoff.forbiddenActions) ? handoff.forbiddenActions : [];
      const evidence = Array.isArray(handoff.evidenceRefs) ? handoff.evidenceRefs : [];
      const guardrails = Array.isArray(handoff.driftGuardrails) ? handoff.driftGuardrails : [];
      const commandText = allowed.map((command) => command.command || '').filter(Boolean).join(' · ');
      return `<div class="surface-row"><strong>${escapeText(handoff.loopId || 'unknown-loop')}</strong> ${surfaceState(Boolean(handoff.status), handoff.status || 'unknown')}<br>
        <small>Step: ${escapeText(handoff.currentStep?.stepId || 'none')} · ${escapeText(handoff.currentStep?.kind || 'unknown')}</small>
        <div class="hint">Next safe action: ${escapeText(handoff.nextSafeAction || 'unrecorded')}</div>
        ${blockers.length ? `<div class="hint">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="hint">No blockers reported.</div>'}
        ${decisions.length ? `<div class="hint">Required decisions: ${escapeText(decisions.map((decision) => `${decision.id || 'decision'}:${decision.kind || 'unknown'}`).join(' · '))}</div>` : '<div class="hint">No required decisions reported.</div>'}
        <div class="hint">Allowed command text: ${escapeText(commandText || 'none')}</div>
        <div class="hint">Forbidden actions: ${escapeText(forbidden.join(' · ') || 'none')}</div>
        <div class="hint">Evidence refs: ${escapeText(evidence.map((ref) => `${ref.id || 'ref'}:${ref.path || 'missing'}`).join(' · ') || 'none')}</div>
        <div class="hint">Guardrails: ${escapeText(guardrails.join(' · ') || 'none')}</div>
        <small>${escapeText(handoff.boundary || 'Advisory evidence only; cockpit is read-only.')}</small>
      </div>`;
    }).join('');
    return `<section id="agent-handoff" class="panel"><h2>Agent handoff</h2>
      <p class="hint">Read-only Handoff Studio surface. It displays allowed command text but does not create buttons, execute commands, grant authority, apply mutations, or merge changes.</p>
      ${rows}
    </section>`;
  }

  function renderLoopEvidenceBundleSurface(run) {
    const bundles = normalizeLoopEvidenceBundles(run?.loop_evidence_bundles || run?.loopEvidenceBundles || run?.loop_evidence_bundle || run?.loopEvidenceBundle || null);
    if (!bundles.length) {
      return '<section id="loop-evidence-bundle" class="panel"><h2>Authoring loop evidence bundle</h2><p class="empty">No loop evidence bundle is attached to dashboard-data.json. Generated bundles stay local under runs/authoring-loop-bundles.</p></section>';
    }
    const rows = bundles.map((bundle) => {
      const missing = Array.isArray(bundle.missingRefs) ? bundle.missingRefs : [];
      const steps = Array.isArray(bundle.steps) ? bundle.steps : [];
      const counts = [
        ['runs', bundle.runs],
        ['comparisons', bundle.comparisons],
        ['proposals', bundle.proposals],
        ['decisions', bundle.reviewDecisions],
        ['transactions', bundle.transactions],
        ['promotions', bundle.regressionPromotions],
        ['matrices', bundle.matrixSnapshots],
        ['journals', bundle.journalSummaries],
      ].map(([label, artifacts]) => `${label}:${Array.isArray(artifacts) ? artifacts.length : 0}`).join(' · ');
      const stepText = steps.map((step) => `${step.stepId || 'step'}:${step.status || 'unknown'}`).join(' · ');
      return `<div class="surface-row"><strong>${escapeText(bundle.loopId || 'unknown-loop')}</strong> <span class="status-idle">${escapeText(bundle.status || 'unknown')}</span><br>
        <small>${escapeText(counts)}</small>
        <div class="hint">Plan: ${escapeText(bundle.plan?.path || 'unrecorded')}</div>
        ${stepText ? `<div class="hint">Steps: ${escapeText(stepText)}</div>` : '<div class="hint">No step outputs recorded.</div>'}
        ${missing.length ? `<div class="hint">Missing/stale refs: ${escapeText(missing.join(' · '))}</div>` : '<div class="hint">No missing refs reported.</div>'}
        <small>${escapeText(bundle.boundary || 'Generated local index only; browser is read-only.')}</small>
      </div>`;
    }).join('');
    return `<section id="loop-evidence-bundle" class="panel"><h2>Authoring loop evidence bundle</h2>
      <p class="hint">Read-only generated index. The browser does not package artifacts, write bundle data, or execute commands.</p>
      ${rows}
    </section>`;
  }

  function renderLoopRecoverySurface(run) {
    const summary = run?.loop_recovery || run?.loopRecovery || run?.loop_status || run?.loopStatus || null;
    if (!summary || typeof summary !== 'object') {
      return '<section id="loop-recovery" class="panel"><h2>Authoring loop recovery</h2><p class="empty">No recovery status is attached to dashboard-data.json. Use the Rust CLI status/resume preflight commands for local recovery inspection.</p></section>';
    }
    const steps = Array.isArray(summary.steps) ? summary.steps : [];
    const rows = steps.length ? steps.map((step) => {
      const recovery = step.recovery || {};
      const manual = recovery.manualAction || {};
      const missing = Array.isArray(step.missingPrerequisites) ? step.missingPrerequisites : [];
      return `<div class="surface-row"><strong>${escapeText(step.id || 'step')}</strong> <span class="status-idle">${escapeText(step.status || 'unknown')}</span><br>
        <small>${escapeText(step.kind || 'unknown')}</small>
        ${recovery.failure ? `<div class="hint">Failure: ${escapeText(recovery.failure.reason || 'unspecified')}</div>` : ''}
        ${manual.description ? `<div class="hint">Manual action: ${escapeText(manual.description)}</div>` : ''}
        ${missing.length ? `<div class="hint">Missing: ${escapeText(missing.join(' · '))}</div>` : ''}
        <small>Next safe action: ${escapeText(step.nextSafeAction || 'Inspect manually.')}</small>
      </div>`;
    }).join('') : '<p class="empty">No recovery steps recorded.</p>';
    return `<section id="loop-recovery" class="panel"><h2>Authoring loop recovery</h2>
      <p class="hint">Read-only recovery state. The browser does not resume, retry, repair, apply mutations, promote regressions, or write trusted state.</p>
      <div class="surface-row"><strong>${escapeText(summary.loopId || 'unknown')}</strong> <span class="status-idle">${escapeText(summary.status || 'unknown')}</span><br><small>Next safe action: ${escapeText(summary.nextSafeAction || 'Inspect manually.')}</small></div>
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
    return `${renderProjectWorkspaceSurface(run)}${renderProjectRunSurface(run)}${renderEvidenceFidelitySurface(run)}${renderEvidenceBrowser(run)}${renderAuthoringProvenanceSurface(run)}${renderEngineExpansionSurface(run)}${renderRenderBreakdownInspectionSurface(run)}${renderExpressiveComponentHudSurface(run)}${renderRuntimeEventInspectionSurface(run)}${renderRuntimeAssetLoadingSurface(run)}${renderAssetPreviewEvidenceSurface(run)}${renderSourceApplyWorktreeContextSurface(run)}${renderStudioDraftAuthoringSurface(run)}${renderVisualDiffPreviewSurface(run)}${renderTilemapDraftPreviewSurface(run)}${renderStudioAssetInspectorSurface(run)}${renderJournalSurface(run)}${renderLoopDryRunSurface(run)}${renderLoopExecutionSurface(run)}${renderLoopRecoverySurface(run)}${renderStudioLoopCockpitSurface(run)}${renderAgentHandoffSurface(run)}${renderLoopEvidenceBundleSurface(run)}${renderMutationReviewSurface(run)}${renderRegressionPromotionSurface(run)}${renderRegressionMatrixSurface(run)}${renderReplaySurface(run)}${renderComparisonSurface(run)}`;
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
      if (latest && (dashboardData.regression_matrix || dashboardData.regressionMatrix || dashboardData.loop_evidence_bundles || dashboardData.loopEvidenceBundles || dashboardData.agent_handoffs || dashboardData.agentHandoffs || dashboardData.loop_cockpit || dashboardData.loopCockpit)) {
        latest = {
          ...latest,
          regression_matrix: dashboardData.regression_matrix || dashboardData.regressionMatrix,
          loop_evidence_bundles: dashboardData.loop_evidence_bundles || dashboardData.loopEvidenceBundles || [],
          agent_handoffs: dashboardData.agent_handoffs || dashboardData.agentHandoffs || [],
          loop_cockpit: dashboardData.loop_cockpit || dashboardData.loopCockpit || null,
        };
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

  return { EDITABLE_FIELDS, READ_ONLY_FIELDS, applyEdit, artifactHref, callPreviewProbe, cliCommand, compareRunsCommand, dashboardExportCommand, escapeText, getValue, init, latestRun, loadDashboardData, previewWindow, projectRunCommand, projectValidateCommand, qaCommand, qaTransactionCommand, readPreviewProbe, reloadPreview, renderAgentHandoffSurface, renderAssetPreviewEvidenceSurface, renderAuthoringProvenanceSurface, renderCommandGenerationPanel, renderComparisonSurface, renderEngineExpansionSurface, renderEvidenceBrowser, renderEvidenceFidelitySurface, renderEvidencePane, fidelityStatusClass, renderExpressiveComponentHudSurface, renderRenderBreakdownInspectionSurface, renderRuntimeEventInspectionSurface, renderRuntimeAssetLoadingSurface, renderVisualDiffPreviewSurface, renderTilemapDraftControl, renderTilemapDraftPreviewSurface, renderInspector, renderIntegration, renderJournalSurface, renderLoopDryRunSurface, renderLoopExecutionSurface, renderLoopEvidenceBundleSurface, renderLoopRecoverySurface, renderStudioLoopCockpitSurface, renderMutationReviewSurface, renderProposalRationaleSurface, renderReviewDecisionSurface, renderRegressionMatrixSurface, renderRegressionPromotionSurface, renderProjectRunSurface, renderProjectWorkspaceSurface, renderPreview, renderPreviewControls, renderQaPanel, renderReadOnlyFields, renderReviewCockpitStageCard, renderStudioReviewCockpitCards, renderRunCommandContext, renderSemanticComparisonSummary, renderSourceApplyWorktreeContextSurface, runtimeReloadPayloadCommand, sceneMutationApplyCommand, renderSceneMutationLifecycleSurface, renderStudioAssetInspectorSurface, renderStudioDraftAuthoringSurface, studioDraftAuthoringState, studioDraftControlModel, studioDraftPreviewCommand, sceneReloadValidateCommand, seedValidateCommand, sceneValidateCommand, transactionCommand, renderReplaySurface, renderStudioGaps, renderStudioNavigation, renderTree, resolvePreviewProbe, studioSurfaceSummary, validateEdit };
})();

if (typeof window !== 'undefined') {
  window.OuroforgeCockpit = OuroforgeCockpit;
  window.addEventListener('DOMContentLoaded', () => OuroforgeCockpit.init());
}

if (typeof module !== 'undefined') {
  module.exports = OuroforgeCockpit;
}
