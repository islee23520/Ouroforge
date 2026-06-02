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

  function qaTransactionCommand(seedPath = 'seeds/platformer.yaml', transactionPath = 'runs/manual/transactions/scene-edit.json', workers = 4) {
    return `cargo run -p ouroforge-cli -- run ${seedPath} --workers ${workers} --transaction ${transactionPath}`;
  }

  function dashboardExportCommand(output = 'examples/evidence-dashboard/dashboard-data.json') {
    return `cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output ${output}`;
  }

  function sceneMutationApplyCommand(runDir = 'runs/run-1', operationPath = 'mutation/scene-operation.json', transactionPath = 'mutation/scene-transaction.json') {
    return `cargo run -p ouroforge-cli -- mutation apply-scene ${runDir} --operation ${operationPath} --transaction-output ${transactionPath}`;
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
    return [
      { id: 'run-browser', label: 'Run/evidence browser', present: hasRun && Array.isArray(run.evidence), detail: hasRun ? `${run.evidence.length} evidence artifact(s)` : 'dashboard data not loaded' },
      { id: 'journal-viewer', label: 'Journal viewer', present: Boolean(run?.journal_view?.exists || run?.journal), detail: run?.journal_view?.summary || 'journal artifact unavailable' },
      { id: 'mutation-review', label: 'Mutation review state', present: Boolean(run?.mutation_lifecycle), detail: run?.mutation_lifecycle?.terminal_state || 'no lifecycle read model' },
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
    const proposedRows = proposedRecords.slice(0, 3).map((record) => {
      const id = record.id || record.proposalId || 'unknown proposal';
      const evidence = record.evidence_id || record.evidenceId || record.evidence || 'no evidence id';
      return `<li>${escapeText(id)} · ${escapeText(evidence)}</li>`;
    }).join('') || '<li>No scene-safe proposal records loaded.</li>';
    const applicationRows = appliedRecords.slice(0, 3).map((record) => `<div class="surface-row"><strong>${escapeText(record.id || 'scene application')}</strong> ${surfaceState(record.status !== 'failed', record.status || 'applied')}<br><small>proposal ${escapeText(record.proposalId || 'unknown')} · transaction ${escapeText(record.transactionId || 'unknown')}</small><br><small>${escapeText(record.beforeSceneHash?.value || 'before unknown')} → ${escapeText(record.afterSceneHash?.value || 'after unknown')}</small></div>`).join('') || '<p class="empty compact">No scene-only mutation application records loaded yet.</p>';
    const applyCommand = sceneMutationApplyCommand(runDir, 'mutation/scene-operation.json', transactionPath);
    return `<div class="scene-mutation-lifecycle"><h3>Scene-only mutation lifecycle</h3>
      <p class="hint">Scene-only mutations remain manual and Rust-validated. The browser displays proposal/application state and safe CLI strings only; it does not apply, accept, reject, or merge anything.</p>
      <div class="field-grid">
        <div><strong>Proposal stage</strong><br>${surfaceState(Boolean(proposed && proposed.state !== 'missing'), proposed?.state || 'missing')}<br><small>${escapeText(proposed?.record_count || 0)} record(s)</small></div>
        <div><strong>Scene application stage</strong><br>${surfaceState(Boolean(applied && applied.state !== 'missing'), applied?.state || 'missing')}<br><small>${escapeText(applied?.record_count || 0)} record(s)</small></div>
        <div><strong>Target scene</strong><br>${escapeText(targetScenePath)}</div>
      </div>
      <h4>Proposal context</h4><ul>${proposedRows}</ul>
      <h4>Application records</h4>${applicationRows}
      <h4>Display-only scene mutation commands</h4>
      <div class="command-list">
        <code>${escapeText(sceneValidateCommand(targetScenePath))}</code>
        <code>${escapeText(applyCommand)}</code>
        <code>${escapeText(dashboardExportCommand())}</code>
      </div>
    </div>`;
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
      <div class="surface-list">${stages}</div>
      ${renderSceneMutationLifecycleSurface(run)}
      <h3>Command hints</h3><div class="command-list">${hints}</div>
    </section>`;
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
    const reasonItems = reasons.length
      ? reasons.slice(0, 5).map((reason) => `<li><span class="status-ok">${escapeText(reason.severity || 'changed')}</span> ${escapeText(reason.kind || 'reason')}: ${escapeText(reason.summary || '')}</li>`).join('')
      : '<li>No semantic reasons recorded.</li>';
    const warningBlock = warnings.length
      ? `<div class="error">Warnings: ${escapeText(warnings.join(' · '))}</div>`
      : '';
    return `<div class="semantic-summary"><strong>Semantic evidence diff</strong>
      <div class="field-grid">
        <div><strong>Schema</strong><br>${escapeText(semantic.schemaVersion || 'legacy')}</div>
        <div><strong>Scenario diffs</strong><br>${escapeText((semantic.scenarios || []).length)}</div>
        <div><strong>World changes</strong><br>${escapeText((semantic.worldState?.changed || []).length)}</div>
        <div><strong>Transaction</strong><br>${escapeText(semantic.transactionProvenance?.changed ? 'changed' : 'unchanged')}</div>
      </div>
      <ul>${reasonItems}</ul>${warningBlock}
    </div>`;
  }

  function renderComparisonSurface(run) {
    const comparison = run?.comparison;
    if (!comparison?.present) {
      return `<section id="run-comparison" class="panel"><h2>Run comparison</h2><p class="empty">${escapeText(comparison?.empty_state || 'No run comparison artifacts are available for this run.')}</p></section>`;
    }
    const artifacts = (comparison.artifacts || []).map((artifact) => `<div class="surface-row"><strong>${escapeText(artifact.before_run_id || 'unknown')}</strong> → <strong>${escapeText(artifact.after_run_id || 'unknown')}</strong> ${surfaceState(true, artifact.classification || 'unknown')}<br><small>${escapeText(artifact.path)}</small>${renderSemanticComparisonSummary(artifact)}${renderRefLinks(artifact.evidence_refs, run)}</div>`).join('');
    return `<section id="run-comparison" class="panel"><h2>Run comparison</h2><p class="hint">Displays existing comparison artifacts only; no browser-side comparison algorithm runs here.</p>${artifacts}</section>`;
  }

  function renderStudioGaps() {
    return `<section class="panel"><h2>Known demo gaps</h2><ul>
      <li>No production editor, native shell, hosted studio, collaboration, plugin marketplace, or visual scripting.</li>
      <li>Generated dashboard data and run artifacts stay uncommitted local state.</li>
      <li>Scene persistence remains Rust-command-only through validated CLI commands.</li>
    </ul></section>`;
  }

  function renderEvidencePane(run) {
    return `${renderEvidenceBrowser(run)}${renderAuthoringProvenanceSurface(run)}${renderEngineExpansionSurface(run)}${renderJournalSurface(run)}${renderMutationReviewSurface(run)}${renderReplaySurface(run)}${renderComparisonSurface(run)}`;
  }

  function renderIntegration(run, previewState = null) {
    return `${renderStudioNavigation(run)}<section id="live-preview">${renderPreview()}${renderPreviewControls(previewState)}</section><section id="scene-editing">${renderQaPanel()}</section>${renderEvidencePane(run)}${renderStudioGaps()}`;
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

  return { EDITABLE_FIELDS, READ_ONLY_FIELDS, applyEdit, artifactHref, callPreviewProbe, cliCommand, dashboardExportCommand, escapeText, getValue, init, latestRun, loadDashboardData, previewWindow, qaCommand, qaTransactionCommand, readPreviewProbe, reloadPreview, renderAuthoringProvenanceSurface, renderCommandGenerationPanel, renderComparisonSurface, renderEngineExpansionSurface, renderEvidenceBrowser, renderEvidencePane, renderInspector, renderIntegration, renderJournalSurface, renderMutationReviewSurface, renderPreview, renderPreviewControls, renderQaPanel, renderReadOnlyFields, renderSemanticComparisonSummary, runtimeReloadPayloadCommand, sceneMutationApplyCommand, renderSceneMutationLifecycleSurface, sceneReloadValidateCommand, sceneValidateCommand, transactionCommand, renderReplaySurface, renderStudioGaps, renderStudioNavigation, renderTree, resolvePreviewProbe, studioSurfaceSummary, validateEdit };
})();

if (typeof window !== 'undefined') {
  window.OuroforgeCockpit = OuroforgeCockpit;
  window.addEventListener('DOMContentLoaded', () => OuroforgeCockpit.init());
}

if (typeof module !== 'undefined') {
  module.exports = OuroforgeCockpit;
}
