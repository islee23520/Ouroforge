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

  function evidenceArtifacts(run, ...keys) {
    return keys.flatMap((key) => Array.isArray(run?.[key]) ? run[key] : []);
  }

  function summarizeTimelineArtifactGroup(run, label, keys) {
    const items = evidenceArtifacts(run, ...keys);
    const missing = items.filter((artifact) => artifact?.exists === false);
    const broken = items.filter((artifact) => artifact?.read_error || artifact?.readError);
    return {
      label,
      count: items.length,
      ids: items.map((artifact) => artifact?.id || artifact?.path || 'unknown'),
      missing: missing.map((artifact) => artifact?.id || artifact?.path || 'unknown'),
      broken: broken.map((artifact) => ({ id: artifact?.id || artifact?.path || 'unknown', reason: artifact?.read_error || artifact?.readError })),
    };
  }

  function timelineDiagnosticsForRun(run, groups) {
    const runId = run?.summary?.id || 'unknown-run';
    return groups.flatMap((group) => [
      ...group.missing.map((id) => ({ runId, kind: 'missing-evidence', artifactId: id, message: `${group.label} artifact is missing` })),
      ...group.broken.map((item) => ({ runId, kind: 'broken-evidence', artifactId: item.id, message: `${group.label} artifact could not be read: ${item.reason}` })),
    ]);
  }

  function timelineMutationLinks(run) {
    return evidenceArtifacts(run, 'mutation_artifacts', 'mutationArtifacts').map((artifact) => ({
      id: artifact?.id || artifact?.path || 'mutation-artifact',
      path: artifact?.path || '',
      kind: artifact?.kind || 'application/json',
      readOnly: artifact?.metadata?.read_only !== false,
    }));
  }

  function timelineSourceApplyLinks(run) {
    return timelineMutationLinks(run).filter((artifact) => /source-patch|source-apply/.test(`${artifact.id} ${artifact.path}`));
  }


  function summarizeTimelineEvidenceArtifact(artifact) {
    const value = artifact?.value;
    const valuePreview = value === undefined
      ? ''
      : JSON.stringify(value).slice(0, 160);
    return {
      id: artifact?.id || artifact?.path || 'unknown-artifact',
      path: artifact?.path || '',
      exists: artifact?.exists !== false,
      readError: artifact?.read_error || artifact?.readError || '',
      valuePreview,
    };
  }

  function timelineArtifactSummaries(run, keys) {
    return evidenceArtifacts(run, ...keys).map(summarizeTimelineEvidenceArtifact);
  }

  function buildTimelineComparisonView(sortedRuns, entries, comparisonCandidates) {
    const runsById = new Map(sortedRuns.map((run) => [run?.summary?.id || 'unknown-run', run]));
    return comparisonCandidates.map((candidate) => {
      const beforeRun = runsById.get(candidate.beforeRunId) || {};
      const afterRun = runsById.get(candidate.afterRunId) || {};
      const beforeScreenshots = timelineArtifactSummaries(beforeRun, ['screenshots']);
      const afterScreenshots = timelineArtifactSummaries(afterRun, ['screenshots']);
      const beforeWorldStates = timelineArtifactSummaries(beforeRun, ['world_states', 'worldStates']);
      const afterWorldStates = timelineArtifactSummaries(afterRun, ['world_states', 'worldStates']);
      const comparisonArtifacts = evidenceArtifacts(afterRun, 'comparison_artifacts', 'comparisonArtifacts')
        .concat(Array.isArray(afterRun?.comparison?.artifacts) ? afterRun.comparison.artifacts : []);
      return {
        beforeRunId: candidate.beforeRunId,
        afterRunId: candidate.afterRunId,
        comparisonRefs: candidate.comparisonRefs,
        classifications: comparisonArtifacts.map((artifact) => artifact?.classification || 'unknown'),
        screenshots: {
          before: beforeScreenshots,
          after: afterScreenshots,
          missing: beforeScreenshots.concat(afterScreenshots).filter((artifact) => !artifact.exists || artifact.readError).map((artifact) => artifact.id),
        },
        worldState: {
          before: beforeWorldStates,
          after: afterWorldStates,
          changed: JSON.stringify(beforeWorldStates.map((artifact) => artifact.valuePreview)) !== JSON.stringify(afterWorldStates.map((artifact) => artifact.valuePreview)),
        },
        guardrail: 'display-only before/after comparison; no evidence mutation, rerun, source apply, command execution, publish, deploy, or trusted write authority',
        diagnostics: entries.find((entry) => entry.runId === candidate.beforeRunId)?.diagnostics.concat(entries.find((entry) => entry.runId === candidate.afterRunId)?.diagnostics || []) || [],
      };
    });
  }


  function buildEvidenceTimelineDiagnosticSummary(diagnostics = []) {
    const byKind = diagnostics.reduce((acc, diagnostic) => {
      const kind = diagnostic?.kind || 'unknown-diagnostic';
      acc[kind] = (acc[kind] || 0) + 1;
      return acc;
    }, {});
    const affectedRuns = [...new Set(diagnostics.map((diagnostic) => diagnostic?.runId || 'unknown-run'))];
    return {
      status: diagnostics.length ? 'attention_required' : 'ready',
      total: diagnostics.length,
      byKind,
      affectedRuns,
      reviewerActions: diagnostics.length
        ? ['inspect generated evidence paths outside Studio', 'restore or regenerate missing evidence through trusted local workflows', 'keep Studio browser surface read-only']
        : ['no missing or broken fixture evidence detected'],
      forbiddenActions: ['browser_rerun_tests', 'browser_write_evidence', 'browser_apply_source_patch', 'execute_command', 'publish_deploy'],
    };
  }

  function buildEvidenceTimelineModel(runs = []) {
    const sortedRuns = [...(Array.isArray(runs) ? runs : [])].sort((left, right) => Number(left?.summary?.created_at_unix_ms || left?.summary?.createdAtUnixMs || 0) - Number(right?.summary?.created_at_unix_ms || right?.summary?.createdAtUnixMs || 0));
    const entries = sortedRuns.map((run) => {
      const summary = run?.summary || {};
      const groups = [
        summarizeTimelineArtifactGroup(run, 'screenshots', ['screenshots']),
        summarizeTimelineArtifactGroup(run, 'world-state', ['world_states', 'worldStates']),
        summarizeTimelineArtifactGroup(run, 'console/crash', ['console_logs', 'consoleLogs', 'cdp_trace_summaries', 'cdpTraceSummaries']),
        summarizeTimelineArtifactGroup(run, 'performance', ['frame_metrics', 'frameMetrics', 'performance_metrics', 'performanceMetrics']),
        summarizeTimelineArtifactGroup(run, 'mutation/source-apply', ['mutation_artifacts', 'mutationArtifacts']),
      ];
      return {
        runId: summary.id || 'unknown-run',
        runDir: summary.run_dir || summary.runDir || '',
        createdAtUnixMs: Number(summary.created_at_unix_ms || summary.createdAtUnixMs || 0),
        seedId: summary.seed_id || summary.seedId || 'unknown-seed',
        verdictStatus: summary.verdict_status || summary.verdictStatus || 'unknown',
        scenarioStatus: summary.scenario_status || summary.scenarioStatus || 'unknown',
        runStatus: summary.run_status || summary.runStatus || 'unknown',
        evidence: Object.fromEntries(groups.map((group) => [group.label, { count: group.count, ids: group.ids }])),
        mutationLinks: timelineMutationLinks(run),
        sourceApplyLinks: timelineSourceApplyLinks(run),
        diagnostics: timelineDiagnosticsForRun(run, groups),
      };
    });
    const comparisonCandidates = entries.slice(1).map((entry, index) => ({
      beforeRunId: entries[index].runId,
      afterRunId: entry.runId,
      comparisonRefs: evidenceArtifacts(sortedRuns[index + 1], 'comparison_artifacts', 'comparisonArtifacts')
        .concat(Array.isArray(sortedRuns[index + 1]?.comparison?.artifacts) ? sortedRuns[index + 1].comparison.artifacts : [])
        .map((artifact) => artifact?.path || artifact?.id || 'comparison-artifact'),
    }));
    const diagnostics = entries.flatMap((entry) => entry.diagnostics);
    return {
      schemaVersion: 'studio-evidence-timeline-model-v1',
      status: diagnostics.length ? 'diagnostics' : 'ready',
      entries,
      comparisonCandidates,
      comparisonView: buildTimelineComparisonView(sortedRuns, entries, comparisonCandidates),
      diagnostics,
      diagnosticSummary: buildEvidenceTimelineDiagnosticSummary(diagnostics),
      guardrails: [
        'timeline model is read-only exported evidence',
        'browser Studio does not mutate evidence, run tests, apply source patches, execute commands, publish, deploy, or write trusted files',
        'source mutation links remain Safe Source Apply handoff evidence only',
      ],
      forbiddenActions: ['write_trusted_file', 'execute_command', 'apply_patch', 'merge_branch', 'publish_deploy', 'rerun_tests_from_browser'],
    };
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
      { id: 'studio-3d-inspection', label: '3D inspection', present: Boolean(run?.engine_summaries?.scene3d_hierarchy?.present || run?.engine_summaries?.scene3d_camera?.present || run?.engine_summaries?.scene3d_render?.present || run?.engine_summaries?.scene3d_collision?.present || run?.engine_summaries?.scene3d_animation?.present || run?.engine_summaries?.scene3d_scenario_verdicts?.present), detail: run?.engine_summaries?.scene3d_probe?.status || run?.engine_summaries?.scene3dProbe?.status || '3D read models unavailable' },
      { id: 'camera-layer-inspection', label: 'Camera/layer inspection', present: Boolean(run?.engine_summaries?.camera || run?.engine_summaries?.camera_state || run?.engine_summaries?.cameraState), detail: run?.engine_summaries?.camera?.scene3dCamera?.activeCameraId || run?.engine_summaries?.camera?.activeCameraId || run?.engine_summaries?.camera_state?.activeCameraId || run?.engine_summaries?.cameraState?.activeCameraId || 'camera read model unavailable' },
      { id: 'render-breakdown-inspection', label: 'Render breakdown inspection', present: Boolean(run?.engine_summaries?.render_breakdown?.present || run?.engine_summaries?.renderBreakdown?.present), detail: `${(run?.engine_summaries?.render_breakdown?.elements || run?.engine_summaries?.renderBreakdown?.elements || []).length} renderable row(s)` },
      { id: 'runtime-profiler-inspection', label: 'Runtime profiler inspection', present: Boolean(run?.engine_summaries?.runtime_frame_budget || run?.engine_summaries?.runtimeFrameBudget), detail: run?.engine_summaries?.runtime_frame_budget?.status || run?.engine_summaries?.runtimeFrameBudget?.status || 'frame-budget read model unavailable' },
      { id: 'runtime-state-inspection', label: 'Runtime save/state inspection', present: Boolean(run?.engine_summaries?.runtime_state?.present || run?.engine_summaries?.runtimeState?.present), detail: `${run?.engine_summaries?.runtime_state?.saveEventCount ?? run?.engine_summaries?.runtimeState?.saveEventCount ?? 0} save event(s), ${run?.engine_summaries?.runtime_state?.replayDigestComparedCount ?? run?.engine_summaries?.runtimeState?.replayDigestComparedCount ?? 0} replay digest check(s)` },
      { id: 'expressive-scene-inspection', label: 'Expressive scene inspection', present: Boolean(run?.engine_summaries?.components?.present || run?.engine_summaries?.triggers?.present || run?.engine_summaries?.hud?.present), detail: run?.engine_summaries?.source_world_state || 'component/trigger/HUD summary unavailable' },
      { id: 'runtime-event-inspection', label: 'Collision/transition/event inspection', present: Boolean(run?.engine_summaries?.collision?.present || run?.engine_summaries?.transition?.present || run?.engine_summaries?.events?.present), detail: run?.engine_summaries?.source_world_state || 'collision/transition/event summary unavailable' },
      { id: 'runtime-asset-loading', label: 'Runtime asset loading', present: Boolean(run?.asset_loading?.present || run?.assetLoading?.present), detail: `${run?.asset_loading?.attempt_count ?? run?.assetLoading?.attemptCount ?? 0} load attempt(s)` },
      { id: 'asset-preview-evidence', label: 'Asset preview evidence', present: Boolean(run?.asset_preview?.present || run?.assetPreview?.present), detail: `${run?.asset_preview?.preview_count ?? run?.assetPreview?.previewCount ?? 0} preview record(s)` },
      { id: 'plugin-registry-browser', label: 'Plugin registry browser', present: Boolean(run?.plugin_registry?.present || run?.pluginRegistry?.present), detail: `${run?.plugin_registry?.plugin_count ?? run?.pluginRegistry?.pluginCount ?? 0} plugin row(s)` },
      { id: 'source-apply-context', label: 'Source apply context', present: Boolean(run?.source_apply_worktree_context?.present || run?.sourceApplyWorktreeContext?.present), detail: `${run?.source_apply_worktree_context?.target_count ?? run?.sourceApplyWorktreeContext?.targetCount ?? 0} target row(s)` },
      { id: 'route-attempt-evidence', label: 'Route attempt evidence', present: Boolean(run?.route_attempts?.present || run?.routeAttempts?.present), detail: `${run?.route_attempts?.attempt_count ?? run?.routeAttempts?.attemptCount ?? 0} attempt row(s)` },
      { id: 'visual-comparison-evidence', label: 'Visual comparison evidence', present: Boolean(run?.visual_comparisons?.present || run?.visualComparisons?.present), detail: `${run?.visual_comparisons?.comparison_count ?? run?.visualComparisons?.comparisonCount ?? 0} comparison row(s)` },
      { id: 'studio-level-design-inspection', label: 'Level design inspection', present: Boolean(normalizeStudioLevelDesignInspection(run).present), detail: `${normalizeStudioLevelDesignInspection(run).panels.length} read-only panel(s)` },
      { id: 'behavior-draft-status', label: 'Behavior draft status', present: Boolean(behaviorDraftReadModel(run).present), detail: `${behaviorDraftReadModel(run).drafts.length} behavior draft(s)` },
      { id: 'behavior-list-panel', label: 'Behavior list panel', present: Boolean(behaviorInspectionModel(run).behaviors.length), detail: `${behaviorInspectionModel(run).behaviors.length} behavior row(s)` },
      { id: 'behavior-event-signal-panel', label: 'Event/signal panel', present: Boolean(behaviorInspectionModel(run).events.length), detail: `${behaviorInspectionModel(run).events.length} event row(s)` },
      { id: 'behavior-state-machine-panel', label: 'State machine panel', present: Boolean(behaviorInspectionModel(run).stateMachines.length), detail: `${behaviorInspectionModel(run).stateMachines.length} machine row(s)` },
      { id: 'behavior-ability-action-panel', label: 'Ability/action panel', present: Boolean(behaviorInspectionModel(run).abilities.length), detail: `${behaviorInspectionModel(run).abilities.length} ability/action row(s)` },
      { id: 'behavior-review-apply-status', label: 'Review/apply status', present: Boolean(behaviorInspectionModel(run).reviews.length || behaviorInspectionModel(run).applies.length), detail: `${behaviorInspectionModel(run).reviews.length} review(s), ${behaviorInspectionModel(run).applies.length} apply row(s)` },
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
      ['Camera/layers', `${summaryValue(summary, 'camera', 'cameraCount', 0)} 2D camera(s), ${summary?.camera?.scene3dCamera?.cameraCount ?? summary?.camera?.scene3d_camera?.camera_count ?? 0} 3D camera(s), ${summaryValue(summary, 'camera', 'layerCount', 0)} layer(s), active ${summary?.camera?.scene3dCamera?.activeCameraId || summaryValue(summary, 'camera', 'activeCameraId')}`],
      ['Render breakdown', `${(summary?.render_breakdown?.elements || summary?.renderBreakdown?.elements || []).length} element(s), ${(summary?.render_breakdown?.absenceDiagnostics || summary?.render_breakdown?.absence_diagnostics || summary?.renderBreakdown?.absenceDiagnostics || []).length} absence diagnostic(s)`],
      ['Runtime profiler', `${summary?.runtime_frame_budget?.status || summary?.runtimeFrameBudget?.status || 'unreported'} · ${(summary?.runtime_frame_budget?.violations || summary?.runtimeFrameBudget?.violations || []).length} violation(s)`],
      ['Tilemaps', `${summaryValue(summary, 'tilemaps', 'tilemapCount', 0)} tilemap(s), ${summaryValue(summary, 'tilemaps', 'layerCount', 0)} layer(s)`],
      ['Assets', `${summaryValue(summary, 'assets', 'manifestId')} · ${summaryValue(summary, 'assets', 'assetCount', 0)} loaded/ref(s)`],
      ['Animation', `${summaryValue(summary, 'animation', 'animatedEntityCount', 0)} animated entit(ies)`],
      ['VFX', `${summaryValue(summary, 'vfx', 'vfxEntityCount', 0)} VFX entit(ies), ${summaryValue(summary, 'vfx', 'vfxEventCount', 0)} event(s)`],
      ['Audio', `${summaryValue(summary, 'audio', 'audioEntityCount', 0)} audio entit(ies), ${summaryValue(summary, 'audio', 'audioEventCount', 0)} event(s), ${summaryValue(summary, 'audio', 'audioWarningCount', 0)} warning(s)`],
      ['Physics/contact', `${summaryValue(summary, 'physics', 'colliderEntityCount', 0)} collider entit(ies), ${summaryValue(summary, 'physics', 'collisionEventCount', 0)} event(s)`],
      ['Input actions', `${summaryValue(summary, 'input', 'mappedActionCount', 0)} mapped, ${summaryValue(summary, 'input', 'activeActionCount', 0)} active, ${summaryValue(summary, 'input', 'warningCount', 0)} warning(s)`],
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


  function renderRuntimeProfilerInspectionSurface(run) {
    const summary = run?.engine_summaries;
    const profiler = summary?.runtime_frame_budget || summary?.runtimeFrameBudget || null;
    if (!summary?.present || !profiler || typeof profiler !== 'object' || Array.isArray(profiler)) {
      return `<section id="runtime-profiler-inspection" class="panel"><h2>Runtime profiler inspection</h2><p class="empty">Runtime profiler/frame-budget evidence is missing or malformed for this run.</p><p class="hint">Read-only inspection only: this panel does not write files, execute commands, mutate scenes, send telemetry, or control the browser runtime.</p></section>`;
    }
    const timings = profiler.timings && typeof profiler.timings === 'object' && !Array.isArray(profiler.timings) ? profiler.timings : {};
    const budget = profiler.budget && typeof profiler.budget === 'object' && !Array.isArray(profiler.budget) ? profiler.budget : {};
    const counts = profiler.counts && typeof profiler.counts === 'object' && !Array.isArray(profiler.counts) ? profiler.counts : {};
    const violations = Array.isArray(profiler.violations) ? profiler.violations : [];
    const readOnlyInspection = profiler.readOnlyInspection || profiler.read_only_inspection || {};
    const disallowedActions = Array.isArray(readOnlyInspection.disallowedActions || readOnlyInspection.disallowed_actions)
      ? (readOnlyInspection.disallowedActions || readOnlyInspection.disallowed_actions)
      : ['trusted writes', 'command bridge', 'scene mutation', 'browser runtime control', 'remote telemetry'];
    const cards = [
      ['Frame', profiler.frameId || profiler.frame_id || 'unrecorded'],
      ['Scene', profiler.sceneId || profiler.scene_id || 'unrecorded'],
      ['Scenario', profiler.scenarioId || profiler.scenario_id || 'none'],
      ['Status', profiler.status || (violations.length ? 'violated' : 'within-budget')],
      ['Update ms', `${timings.updateMs ?? timings.update_ms ?? 'missing'} / ${budget.updateMs ?? budget.update_ms ?? 'missing'}`],
      ['Render ms', `${timings.renderMs ?? timings.render_ms ?? 'missing'} / ${budget.renderMs ?? budget.render_ms ?? 'missing'}`],
      ['Evidence ms', `${timings.evidenceMs ?? timings.evidence_ms ?? 'missing'} / ${budget.evidenceMs ?? budget.evidence_ms ?? 'missing'}`],
      ['Total ms', `${timings.totalMs ?? timings.total_ms ?? 'missing'} / ${budget.totalMs ?? budget.total_ms ?? 'missing'}`],
      ['Entities/draws/layers', `${counts.entityCount ?? counts.entity_count ?? 0} / ${counts.drawCallCount ?? counts.draw_call_count ?? 0} / ${counts.layerCount ?? counts.layer_count ?? 0}`],
      ['Collision/animation/VFX/audio', `${counts.collisionPairCount ?? counts.collision_pair_count ?? 0} / ${counts.activeAnimationCount ?? counts.active_animation_count ?? 0} / ${counts.activeVfxCount ?? counts.active_vfx_count ?? 0} / ${counts.audioEventCount ?? counts.audio_event_count ?? 0}`],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const violationRows = violations.length
      ? violations.slice(0, 24).map((violation) => `<div class="surface-row"><strong>${escapeText(violation?.field || 'metric')}</strong> ${surfaceState(false, 'violated')}<br><small>actual ${escapeText(violation?.actualMs ?? violation?.actual_ms ?? 'missing')}ms · budget ${escapeText(violation?.budgetMs ?? violation?.budget_ms ?? 'missing')}ms</small></div>`).join('')
      : '<div class="surface-row">No frame-budget violations recorded.</div>';
    return `<section id="runtime-profiler-inspection" class="panel"><h2>Runtime profiler inspection</h2>
      <p class="hint">Escaped read-only runtime frame-budget evidence from browser-local probes. Browser observations are evidence inputs, not trusted profiler authority, and this panel cannot persist trusted state.</p>
      <div class="field-grid">${cards}</div>
      <h3>Budget violations</h3>${violationRows}
      <p class="hint">Authority: ${escapeText(profiler.authority || 'browser_runtime_evidence_input_not_profiler_truth')}.</p>
      <p class="hint">Disallowed actions: ${escapeText(disallowedActions.join(' · '))}</p>
    </section>`;
  }

  function renderRuntimeStateInspectionSurface(run) {
    const summary = run?.engine_summaries;
    const runtimeState = summary?.runtime_state || summary?.runtimeState || null;
    if (!summary?.present || !runtimeState || typeof runtimeState !== 'object' || Array.isArray(runtimeState) || runtimeState.present === false) {
      return `<section id="runtime-state-inspection" class="panel"><h2>Runtime save/state inspection</h2><p class="empty">${escapeText(runtimeState?.emptyState || runtimeState?.empty_state || 'No runtime save/load/replay state read model is available for this run.')}</p><p class="hint">Read-only inspection only: this panel does not write files, execute commands, mutate saves, persist browser state, or control the browser runtime.</p></section>`;
    }
    const readOnlyInspection = runtimeState.readOnlyInspection || runtimeState.read_only_inspection || {};
    const disallowedActions = Array.isArray(readOnlyInspection.disallowedActions || readOnlyInspection.disallowed_actions)
      ? (readOnlyInspection.disallowedActions || readOnlyInspection.disallowed_actions)
      : ['trusted writes', 'command bridge', 'save mutation', 'browser runtime control'];
    const digest = runtimeState.digest && typeof runtimeState.digest === 'object' && !Array.isArray(runtimeState.digest) ? runtimeState.digest : {};
    const saveEvents = Array.isArray(runtimeState.saveEvents || runtimeState.save_events) ? (runtimeState.saveEvents || runtimeState.save_events) : [];
    const replayEvents = Array.isArray(runtimeState.replayEvents || runtimeState.replay_events) ? (runtimeState.replayEvents || runtimeState.replay_events) : [];
    const snapshots = Array.isArray(runtimeState.snapshots) ? runtimeState.snapshots : [];
    const cards = [
      ['State id', runtimeState.stateId || runtimeState.state_id || 'unrecorded'],
      ['Scene', runtimeState.sceneId || runtimeState.scene_id || 'unrecorded'],
      ['Tick', runtimeState.tick ?? 'unknown'],
      ['Digest', `${digest.algorithm || 'unknown'}:${digest.value || 'missing'}`],
      ['Snapshots', runtimeState.snapshotCount ?? runtimeState.snapshot_count ?? snapshots.length],
      ['Save events', `${runtimeState.saveCreatedCount ?? runtimeState.save_created_count ?? 0} created / ${runtimeState.saveLoadedCount ?? runtimeState.save_loaded_count ?? 0} loaded`],
      ['Replay digest checks', runtimeState.replayDigestComparedCount ?? runtimeState.replay_digest_compared_count ?? replayEvents.length],
      ['Authority', runtimeState.authority || 'browser_runtime_evidence_input_not_trusted_persistence'],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const saveRows = saveEvents.length
      ? saveEvents.slice(0, 24).map((event) => `<div class="surface-row"><strong>${escapeText(event?.payload?.saveId || event?.payload?.save_id || event?.type || 'save event')}</strong> ${surfaceState(true, event?.type || 'save')}<br><small>slot ${escapeText(event?.payload?.slotId || event?.payload?.slot_id || 'unknown')} · digest ${escapeText(event?.payload?.stateDigest?.value || event?.payload?.state_digest?.value || 'missing')}</small></div>`).join('')
      : '<div class="surface-row">No save/load events recorded.</div>';
    const replayRows = replayEvents.length
      ? replayEvents.slice(0, 24).map((event) => `<div class="surface-row"><strong>${escapeText(event?.payload?.frameId || event?.payload?.frame_id || 'replay frame')}</strong> ${surfaceState(event?.payload?.status === 'matched', event?.payload?.status || 'digest compared')}<br><small>expected ${escapeText(event?.payload?.expected?.value || 'missing')} · actual ${escapeText(event?.payload?.actual?.value || 'missing')}</small></div>`).join('')
      : '<div class="surface-row">No replay digest comparison events recorded.</div>';
    const snapshotRows = snapshots.length
      ? snapshots.slice(0, 24).map((snapshot) => `<div class="surface-row"><strong>${escapeText(snapshot?.snapshotId || snapshot?.snapshot_id || 'snapshot')}</strong><br><small>tick ${escapeText(snapshot?.tick ?? 'unknown')}</small></div>`).join('')
      : '<div class="surface-row">No runtime snapshots recorded.</div>';
    return `<section id="runtime-state-inspection" class="panel"><h2>Runtime save/state inspection</h2>
      <p class="hint">Escaped read-only runtime save/load, snapshot, and replay digest evidence. Browser observations are evidence inputs only; trusted persistence remains Rust/local generated evidence.</p>
      <div class="field-grid">${cards}</div>
      <h3>Save/load events</h3>${saveRows}
      <h3>Replay digest comparisons</h3>${replayRows}
      <h3>Snapshots</h3>${snapshotRows}
      <p class="hint">Disallowed actions: ${escapeText(disallowedActions.join(' · '))}</p>
    </section>`;
  }

  function renderInputActionInspectionSurface(run) {
    const summary = run?.engine_summaries;
    const input = summary?.input || null;
    if (!summary?.present || !input || typeof input !== 'object' || Array.isArray(input) || input.present === false) {
      return `<section id="input-action-inspection" class="panel"><h2>Input action mapping</h2><p class="empty">${escapeText(input?.emptyState || input?.empty_state || 'No input action read model is available for this run.')}</p><p class="hint">Read-only inspection only: this panel does not write files, execute commands, mutate scenes, or control the browser runtime.</p></section>`;
    }
    const diagnostics = input.diagnostics && typeof input.diagnostics === 'object' && !Array.isArray(input.diagnostics) ? input.diagnostics : {};
    const readOnlyInspection = diagnostics.readOnlyInspection || diagnostics.read_only_inspection || {};
    const disallowedActions = Array.isArray(readOnlyInspection.disallowedActions || readOnlyInspection.disallowed_actions)
      ? (readOnlyInspection.disallowedActions || readOnlyInspection.disallowed_actions)
      : ['trusted writes', 'command bridge', 'scene mutation', 'browser runtime control'];
    const activeActions = Array.isArray(input.activeActions || input.active_actions) ? (input.activeActions || input.active_actions) : [];
    const warningItems = [
      ['Missing actions', diagnostics.missingActions || diagnostics.missing_actions],
      ['Unmapped actions', diagnostics.unmappedActions || diagnostics.unmapped_actions],
      ['Duplicate actions', diagnostics.duplicateActions || diagnostics.duplicate_actions],
      ['Unresolved overrides', diagnostics.unresolvedOverrides || diagnostics.unresolved_overrides],
    ].map(([label, values]) => `<li><strong>${escapeText(label)}</strong>: ${escapeText(Array.isArray(values) && values.length ? values.join(', ') : 'none')}</li>`).join('');
    const conflicts = Array.isArray(diagnostics.conflictingBindings || diagnostics.conflicting_bindings)
      ? (diagnostics.conflictingBindings || diagnostics.conflicting_bindings)
      : [];
    const conflictRows = conflicts.length
      ? conflicts.map((conflict) => `<div class="surface-row"><strong>${escapeText(conflict?.key || 'key')}</strong> ${surfaceState(false, 'conflict')}<br><small>${escapeText(Array.isArray(conflict?.actions) ? conflict.actions.join(' / ') : 'unknown actions')}</small></div>`).join('')
      : '<div class="surface-row">No conflicting keyboard bindings recorded.</div>';
    const rawKeys = input.rawInput?.keys || input.raw_input?.keys || {};
    const activeKeys = objectEntries(rawKeys).filter(([, value]) => value === true).map(([key]) => key);
    const cards = [
      ['Mapped actions', input.mappedActionCount ?? input.mapped_action_count ?? 0],
      ['Active actions', `${input.activeActionCount ?? input.active_action_count ?? activeActions.length} (${activeActions.join(', ') || 'none'})`],
      ['Warnings', input.warningCount ?? input.warning_count ?? diagnostics.warningCount ?? diagnostics.warning_count ?? 0],
      ['Active raw keys', activeKeys.join(', ') || 'none'],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    return `<section id="input-action-inspection" class="panel"><h2>Input action mapping</h2>
      <p class="hint">Read-only action-state and diagnostics evidence from runtime world-state. This surface cannot write scene state, execute commands, mutate sources, or control the browser runtime.</p>
      <div class="field-grid">${cards}</div>
      <h3>Diagnostics</h3><ul>${warningItems}</ul>
      <h3>Conflicts</h3>${conflictRows}
      <p class="hint">Disallowed actions: ${escapeText(disallowedActions.join(' · '))}</p>
    </section>`;
  }

  function renderBreakdownValue(record, snakeKey, camelKey, fallback = 'unknown') {
    const value = record?.[snakeKey] ?? record?.[camelKey];
    if (value === null || value === undefined || value === '') return fallback;
    return value;
  }

  function scene3dReadModel(summary, snakeKey, camelKey) {
    const value = summary?.[snakeKey] ?? summary?.[camelKey];
    if (!value || typeof value !== 'object' || Array.isArray(value)) return null;
    return value;
  }

  function scene3dMalformed(summary, snakeKey, camelKey) {
    const value = summary?.[snakeKey] ?? summary?.[camelKey];
    return value !== undefined && value !== null && (typeof value !== 'object' || Array.isArray(value));
  }

  function scene3dListRows(items, emptyText, renderRow, limit = 12) {
    if (!Array.isArray(items) || !items.length) return `<div class="surface-row">${escapeText(emptyText)}</div>`;
    return items.slice(0, limit).map(renderRow).join('');
  }

  function scene3dDisallowedActions(...models) {
    const actions = models.flatMap((model) => {
      const inspection = model?.readOnlyInspection || model?.read_only_inspection || {};
      const values = inspection.disallowedActions || inspection.disallowed_actions || [];
      return Array.isArray(values) ? values : [];
    });
    return [...new Set(actions.concat(['trusted writes', 'command bridge', 'viewport persistence', 'scene mutation', 'browser runtime control']))];
  }

  function renderStudio3dInspectionSurface(run) {
    const summary = run?.engine_summaries;
    if (!summary?.present) {
      return `<section id="studio-3d-inspection" class="panel"><h2>3D inspection</h2><p class="empty">${escapeText(summary?.empty_state || 'No Engine Expansion read model is available for 3D inspection.')}</p><p class="hint">Read-only Studio surface. The browser does not write files, execute commands, persist viewport manipulation, or act as a 3D editor.</p></section>`;
    }
    const hierarchy = scene3dReadModel(summary, 'scene3d_hierarchy', 'scene3dHierarchy');
    const camera = scene3dReadModel(summary, 'scene3d_camera', 'scene3dCamera');
    const probe = scene3dReadModel(summary, 'scene3d_probe', 'scene3dProbe');
    const render = scene3dReadModel(summary, 'scene3d_render', 'scene3dRender');
    const collision = scene3dReadModel(summary, 'scene3d_collision', 'scene3dCollision');
    const animation = scene3dReadModel(summary, 'scene3d_animation', 'scene3dAnimation');
    const verdicts = scene3dReadModel(summary, 'scene3d_scenario_verdicts', 'scene3dScenarioVerdicts');
    const malformed = [
      ['hierarchy', 'scene3d_hierarchy', 'scene3dHierarchy'],
      ['camera', 'scene3d_camera', 'scene3dCamera'],
      ['probe', 'scene3d_probe', 'scene3dProbe'],
      ['render', 'scene3d_render', 'scene3dRender'],
      ['collision', 'scene3d_collision', 'scene3dCollision'],
      ['animation', 'scene3d_animation', 'scene3dAnimation'],
      ['scenario verdicts', 'scene3d_scenario_verdicts', 'scene3dScenarioVerdicts'],
    ].filter(([, snake, camel]) => scene3dMalformed(summary, snake, camel)).map(([label]) => label);
    const transforms = Array.isArray(hierarchy?.transforms) ? hierarchy.transforms : [];
    const cameras = Array.isArray(camera?.cameras) ? camera.cameras : [];
    const renderables = Array.isArray(render?.renderables) ? render.renderables : [];
    const collisionEvents = Array.isArray(collision?.events) ? collision.events : [];
    const invalidColliders = Array.isArray(collision?.invalidColliders || collision?.invalid_colliders) ? (collision.invalidColliders || collision.invalid_colliders) : [];
    const animationStates = Array.isArray(animation?.states) ? animation.states : [];
    const animationEvents = Array.isArray(animation?.events) ? animation.events : [];
    const scenarioVerdicts = Array.isArray(verdicts?.verdicts) ? verdicts.verdicts : [];
    const meshMaterialRefs = renderables.map((renderable) => ({
      id: renderable?.id || renderable?.nodeId || renderable?.node_id || 'scene3d-renderable',
      nodeId: renderable?.nodeId || renderable?.node_id || 'unknown node',
      meshRef: renderable?.meshRef || renderable?.mesh_ref || 'none',
      materialRef: renderable?.materialRef || renderable?.material_ref || 'none',
      visible: renderable?.visible !== false,
      fallbackReason: renderable?.fallbackReason || renderable?.fallback_reason || '',
    }));
    const cards = [
      ['Probe status', probe?.status || (probe?.present === false ? 'missing' : 'unknown')],
      ['Scene kind', probe?.sceneKind || probe?.scene_kind || 'unknown'],
      ['Hierarchy nodes', hierarchy?.nodeCount ?? hierarchy?.node_count ?? transforms.length],
      ['Root / parented', `${hierarchy?.rootCount ?? hierarchy?.root_count ?? 0} / ${hierarchy?.parentedNodeCount ?? hierarchy?.parented_node_count ?? 0}`],
      ['Active 3D camera', camera?.activeCameraId || camera?.active_camera_id || 'unrecorded'],
      ['Camera count', camera?.cameraCount ?? camera?.camera_count ?? cameras.length],
      ['Mesh / material refs', `${render?.meshCount ?? render?.mesh_count ?? 0} / ${render?.materialCount ?? render?.material_count ?? 0}`],
      ['Render visible/skipped', `${render?.visibleObjectCount ?? render?.visible_object_count ?? 0} / ${render?.skippedObjectCount ?? render?.skipped_object_count ?? 0}`],
      ['Collision contact/trigger', `${collision?.contactCount ?? collision?.contact_count ?? 0} / ${collision?.triggerCount ?? collision?.trigger_count ?? 0}`],
      ['Animation playing/total', `${animation?.playingStateCount ?? animation?.playing_state_count ?? 0} / ${animation?.stateCount ?? animation?.state_count ?? animationStates.length}`],
      ['Scenario verdicts', `${verdicts?.failedVerdictCount ?? verdicts?.failed_verdict_count ?? 0} failed / ${verdicts?.verdictCount ?? verdicts?.verdict_count ?? scenarioVerdicts.length} total`],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const hierarchyRows = scene3dListRows(transforms, hierarchy?.emptyState || hierarchy?.empty_state || 'No 3D hierarchy rows exported.', (entry) => `<div class="surface-row"><strong>${escapeText(entry?.nodeId || entry?.node_id || 'node')}</strong> ${surfaceState(entry?.parentId || entry?.parent_id, entry?.parentId || entry?.parent_id ? 'parented' : 'root')}<br><small>parent ${escapeText(entry?.parentId || entry?.parent_id || 'none')} · transform ${escapeText(compactJson(entry?.worldTransform || entry?.world_transform || entry?.localTransform || entry?.local_transform || {}))}</small></div>`);
    const cameraRows = scene3dListRows(cameras, camera?.emptyState || camera?.empty_state || 'No 3D camera rows exported.', (entry) => `<div class="surface-row"><strong>${escapeText(entry?.id || 'camera')}</strong> ${surfaceState(Boolean(entry?.active), entry?.active ? 'active' : 'inactive')}<br><small>projection ${escapeText(entry?.projection?.kind || 'unknown')} · fov ${escapeText(entry?.projection?.fovDegrees ?? entry?.projection?.fov_degrees ?? 'n/a')} · near/far ${escapeText(entry?.projection?.near ?? '?')}/${escapeText(entry?.projection?.far ?? '?')} · viewport ${escapeText(compactJson(entry?.viewport || {}))}</small></div>`);
    const meshRows = scene3dListRows(meshMaterialRefs, 'No mesh/material refs exported.', (entry) => `<div class="surface-row"><strong>${escapeText(entry.id)}</strong> ${surfaceState(entry.visible, entry.visible ? 'visible' : 'skipped')}<br><small>node ${escapeText(entry.nodeId)} · mesh ${escapeText(entry.meshRef)} · material ${escapeText(entry.materialRef)}${entry.fallbackReason ? ` · ${escapeText(entry.fallbackReason)}` : ''}</small></div>`);
    const renderRows = scene3dListRows(renderables, render?.emptyState || render?.empty_state || 'No 3D render rows exported.', (entry) => `<div class="surface-row"><strong>${escapeText(entry?.id || entry?.nodeId || entry?.node_id || 'renderable')}</strong> ${surfaceState(entry?.visible !== false, entry?.primitive || entry?.meshKind || entry?.mesh_kind || 'renderable')}<br><small>camera ${escapeText(entry?.cameraId || entry?.camera_id || render?.cameraId || render?.camera_id || 'none')} · screenshot ${escapeText(render?.screenshotArtifact || render?.screenshot_artifact || 'not produced')}</small></div>`);
    const collisionRows = scene3dListRows(collisionEvents.concat(invalidColliders), collision?.emptyState || collision?.empty_state || 'No 3D collision rows exported.', (entry) => `<div class="surface-row"><strong>${escapeText(entry?.type || entry?.colliderRef || entry?.collider_ref || entry?.nodeId || entry?.node_id || '3D collision evidence')}</strong><br><small>${escapeText(compactJson(entry))}</small></div>`);
    const animationRows = scene3dListRows(animationStates.concat(animationEvents), animation?.emptyState || animation?.empty_state || 'No 3D animation rows exported.', (entry) => `<div class="surface-row"><strong>${escapeText(entry?.clipId || entry?.clip_id || entry?.type || '3D animation state')}</strong> ${surfaceState(entry?.playing !== false, entry?.playing === false ? 'paused' : 'playing/evidence')}<br><small>${escapeText(compactJson(entry))}</small></div>`);
    const verdictRows = scene3dListRows(scenarioVerdicts, verdicts?.emptyState || verdicts?.empty_state || 'No 3D scenario verdict rows exported.', (entry) => `<div class="surface-row"><strong>${escapeText(entry?.scenarioId || entry?.scenario_id || 'scenario')}</strong> ${surfaceState(entry?.status === 'passed', entry?.status || 'unknown')}<br><small>assertions ${escapeText(entry?.assertionCount ?? entry?.assertion_count ?? 'unknown')} · ${escapeText(compactJson(entry))}</small></div>`);
    const boundaries = [hierarchy, camera, probe, render, collision, animation, verdicts]
      .map((model) => model?.boundary)
      .filter(Boolean)
      .map((boundary) => `<li>${escapeText(boundary)}</li>`)
      .join('') || '<li>Display-only 3D evidence inspection; no editor, trusted write, or command authority.</li>';
    const malformedMarkup = malformed.length
      ? `<p class="error">Malformed 3D read model(s): ${escapeText(malformed.join(', '))}</p>`
      : '<p class="hint">3D read models loaded or reported as visibly missing.</p>';
    return `<section id="studio-3d-inspection" class="panel"><h2>3D inspection</h2>
      <p class="hint">Escaped read-only 3D capability evidence. This panel is not a 3D editor, does not persist viewport manipulation, does not execute commands, and does not write trusted files.</p>
      ${malformedMarkup}
      <div class="field-grid">${cards}</div>
      <h3>Scene hierarchy</h3>${hierarchyRows}
      <h3>Active camera/projection</h3>${cameraRows}
      <h3>Mesh/material refs</h3>${meshRows}
      <h3>Render summary</h3>${renderRows}
      <h3>Collision/trigger evidence</h3>${collisionRows}
      <h3>Animation state</h3>${animationRows}
      <h3>Scenario verdicts</h3>${verdictRows}
      <h3>3D boundaries</h3><ul>${boundaries}</ul>
      <p class="hint">Disallowed actions: ${escapeText(scene3dDisallowedActions(hierarchy, camera, probe, render, collision, animation, verdicts).join(' · '))}</p>
    </section>`;
  }

  function renderCameraLayerInspectionSurface(run) {
    const summary = run?.engine_summaries;
    const camera = summary?.camera || summary?.cameraState || summary?.camera_state || null;
    const renderer = summary?.renderer || {};
    if (!summary?.present || !camera || typeof camera !== 'object' || Array.isArray(camera) || camera.present === false) {
      return `<section id="camera-layer-inspection" class="panel"><h2>Camera/layer inspection</h2><p class="empty">${escapeText(camera?.emptyState || camera?.empty_state || 'No camera/layer read model is available; camera/layer evidence is missing or malformed for this run.')}</p><p class="hint">Read-only inspection only: this panel does not write files, execute commands, mutate scenes, or control the browser runtime.</p></section>`;
    }
    const cameras = Array.isArray(camera.cameras) ? camera.cameras : [];
    const scene3dCamera = camera.scene3dCamera || camera.scene3d_camera || camera.camera3d || {};
    const scene3dCameras = Array.isArray(scene3dCamera.cameras) ? scene3dCamera.cameras : [];
    const layers = Array.isArray(camera.layers) ? camera.layers : (Array.isArray(renderer.layers) ? renderer.layers : []);
    const worldToScreen = camera.worldToScreen && typeof camera.worldToScreen === 'object' && !Array.isArray(camera.worldToScreen) ? camera.worldToScreen : {};
    const readOnlyInspection = camera.readOnlyInspection || camera.read_only_inspection || {};
    const disallowedActions = Array.isArray(readOnlyInspection.disallowedActions || readOnlyInspection.disallowed_actions) ? (readOnlyInspection.disallowedActions || readOnlyInspection.disallowed_actions) : ['trusted writes', 'command bridge', 'scene mutation', 'browser runtime control'];
    const cards = [
      ['Active camera', camera.activeCameraId || camera.active_camera_id || 'default'],
      ['Renderer camera', JSON.stringify(camera.rendererCamera || camera.renderer_camera || renderer.camera || {})],
      ['Viewport', JSON.stringify(camera.viewport || renderer.viewport || {})],
      ['Camera records', camera.cameraCount ?? camera.camera_count ?? cameras.length],
      ['3D camera records', scene3dCamera.cameraCount ?? scene3dCamera.camera_count ?? scene3dCameras.length],
      ['Layer records', camera.layerCount ?? camera.layer_count ?? layers.length],
      ['Parallax layers', camera.parallaxLayerCount ?? camera.parallax_layer_count ?? 0],
      ['Camera-excluded layers', camera.cameraExcludedLayerCount ?? camera.camera_excluded_layer_count ?? 0],
      ['World-to-screen samples', camera.worldToScreenSampleCount ?? camera.world_to_screen_sample_count ?? Object.keys(worldToScreen).length],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const cameraRows = cameras.slice(0, 12).map((entry) => `<div class="surface-row"><strong>${escapeText(entry?.id || 'camera')}</strong> ${surfaceState(Boolean(entry?.active), entry?.active ? 'active' : 'inactive')}<br><small>follow ${escapeText(entry?.followTarget || entry?.follow_target || 'none')} · position ${escapeText(JSON.stringify(entry?.position || {}))} · clamp ${escapeText(JSON.stringify(entry?.clampBounds || entry?.clamp_bounds || {}))}</small></div>`).join('') || '<div class="surface-row">No camera records exported.</div>';
    const camera3dRows = scene3dCameras.slice(0, 12).map((entry) => `<div class="surface-row"><strong>${escapeText(entry?.id || 'camera3d')}</strong> ${surfaceState(Boolean(entry?.active), entry?.active ? 'active' : 'inactive')}<br><small>projection ${escapeText(entry?.projection?.kind || 'unknown')} · fov ${escapeText(entry?.projection?.fovDegrees ?? 'n/a')} · near/far ${escapeText(entry?.projection?.near ?? '?')}/${escapeText(entry?.projection?.far ?? '?')} · aspect×1000 ${escapeText(entry?.aspectRatioX1000 ?? 'n/a')} · viewport ${escapeText(JSON.stringify(entry?.viewport || {}))}</small></div>`).join('') || `<div class="surface-row">${escapeText(scene3dCamera.emptyState || scene3dCamera.empty_state || 'No 3D camera evidence exported.')}</div>`;
    const layerRows = layers.slice(0, 12).map((layer) => `<div class="surface-row"><strong>${escapeText(layer?.id || 'layer')}</strong> ${surfaceState(layer?.cameraParticipation !== false && layer?.camera_participation !== false, layer?.cameraParticipation === false || layer?.camera_participation === false ? 'screen-space' : 'camera-space')}<br><small>order ${escapeText(layer?.order ?? '?')} · parallax ${escapeText(layer?.parallaxFactor ?? layer?.parallax_factor ?? 'n/a')} · ${escapeText(layer?.visible === false ? 'hidden' : 'visible')}</small></div>`).join('') || '<div class="surface-row">No layer records exported.</div>';
    const sampleRows = Object.entries(worldToScreen).slice(0, 12).map(([entityId, sample]) => `<div class="surface-row"><strong>${escapeText(entityId)}</strong> ${surfaceState(true, sample?.layer || 'default')}<br><small>screen ${escapeText(JSON.stringify({ x: sample?.x, y: sample?.y }))} · offset ${escapeText(JSON.stringify(sample?.cameraOffset || sample?.camera_offset || {}))}</small></div>`).join('') || '<div class="surface-row">No world-to-screen samples exported.</div>';
    return `<section id="camera-layer-inspection" class="panel"><h2>Camera/layer inspection</h2>
      <p class="hint">Escaped read-only camera/layer evidence from runtime world-state. This surface cannot write scene state, execute commands, mutate sources, or control the browser runtime.</p>
      <div class="field-grid">${cards}</div>
      <h3>Cameras</h3>${cameraRows}
      <h3>3D cameras</h3><p class="hint">Read-only 3D camera evidence only; no viewport persistence, cinematic timeline, or camera editor tooling.</p>${camera3dRows}
      <h3>Layers</h3>${layerRows}
      <h3>World-to-screen samples</h3>${sampleRows}
      <p class="hint">Disallowed actions: ${escapeText(disallowedActions.join(' · '))}</p>
    </section>`;
  }

  function renderRenderBreakdownInspectionSurface(run) {
    const summary = run?.engine_summaries;
    const breakdown = summary?.render_breakdown || summary?.renderBreakdown || null;
    const queue = summary?.render_queue || summary?.renderQueue || {};
    const scene3dRender = summary?.scene3d_render || summary?.scene3dRender || {};
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
    const queueRenderables = Array.isArray(queue.renderables) ? queue.renderables : [];
    const scene3dRenderables = Array.isArray(scene3dRender.renderables) ? scene3dRender.renderables : [];
    const scene3dFallbacks = Array.isArray(scene3dRender.fallbackReasons || scene3dRender.fallback_reasons)
      ? (scene3dRender.fallbackReasons || scene3dRender.fallback_reasons)
      : [];
    const queueValidation = queue.validation || {};
    const tilemapStats = queue.tilemapStats || queue.tilemap_stats || {};
    const cards = [
      ['Frame', renderBreakdownValue(breakdown, 'frame_id', 'frameId', 'unrecorded')],
      ['Scene', renderBreakdownValue(breakdown, 'scene_id', 'sceneId', summary?.scene?.sceneId || 'unrecorded')],
      ['Element count', breakdown.element_count ?? breakdown.elementCount ?? elements.length],
      ['Absence diagnostics', breakdown.absence_diagnostic_count ?? breakdown.absenceDiagnosticCount ?? absenceDiagnostics.length],
      ['Queue renderables', queue.renderable_count ?? queue.renderableCount ?? queueRenderables.length],
      ['Draw calls', queue.draw_call_count ?? queue.drawCallCount ?? 0],
      ['Queue status', queueValidation.status || 'unreported'],
      ['3D smoke visible/skipped', scene3dRender.present ? `${scene3dRender.visibleObjectCount ?? scene3dRender.visible_object_count ?? 0}/${scene3dRender.skippedObjectCount ?? scene3dRender.skipped_object_count ?? 0}` : 'not exported'],
      ['Tilemap draw tiles', tilemapStats.drawnTileCount ?? tilemapStats.drawn_tile_count ?? 0],
      ['Asset-backed tiles', tilemapStats.assetTileCount ?? tilemapStats.asset_tile_count ?? 0],
      ['Missing tile refs', tilemapStats.missingTileRefCount ?? tilemapStats.missing_tile_ref_count ?? 0],
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
    const queueRows = queueRenderables.slice(0, 24).map((renderable) => `<div class="surface-row"><strong>${escapeText(renderable?.id || 'queue-renderable')}</strong> ${surfaceState(renderable?.visible !== false, renderable?.primitiveKind || 'unknown')}<br><small>draw order ${escapeText(renderable?.drawOrder ?? '?')} · layer ${escapeText(renderable?.layer || 'default')} · source ${escapeText(renderable?.sourceKind || 'unknown')}:${escapeText(renderable?.sourceId || 'unknown')} · ${escapeText(renderable?.visible === false ? (renderable?.fallbackReason || 'skipped') : 'visible')} · tiles ${escapeText(renderable?.tileCount ?? 0)} · missing ${escapeText(renderable?.missingTileRefCount ?? 0)}</small></div>`).join('') || '<div class="surface-row">No render queue rows exported.</div>';
    const scene3dRows = scene3dRender.present
      ? scene3dRenderables.slice(0, 24).map((renderable) => `<div class="surface-row"><strong>${escapeText(renderable?.id || renderable?.nodeId || 'scene3d-renderable')}</strong> ${surfaceState(renderable?.visible !== false, renderable?.primitive || renderable?.meshKind || 'unknown')}<br><small>node ${escapeText(renderable?.nodeId || 'unknown')} · mesh ${escapeText(renderable?.meshRef || 'none')} · material ${escapeText(renderable?.materialRef || 'none')} · camera ${escapeText(renderable?.cameraId || scene3dRender.cameraId || 'none')} · ${escapeText(renderable?.visible === false ? (renderable?.fallbackReason || 'skipped') : 'visible')} · screenshot ${escapeText(scene3dRender.screenshotArtifact || scene3dRender.screenshot_artifact || 'not produced')}</small></div>`).join('') || '<div class="surface-row">No 3D render smoke rows exported.</div>'
      : `<div class="surface-row">${escapeText(scene3dRender.emptyState || 'No 3D render smoke evidence is available.')}</div>`;
    const scene3dFallbackRows = scene3dFallbacks.slice(0, 24).map((reason) => `<div class="surface-row">${escapeText(reason)}</div>`).join('') || '<div class="surface-row">No 3D render fallback reasons exported.</div>';
    return `<section id="render-breakdown-inspection" class="panel"><h2>Render breakdown inspection</h2>
      <p class="hint">Read-only render breakdown from runtime world-state evidence. This surface performs no writes, no commands, no scene mutation, and no browser runtime control.</p>
      <div class="field-grid">${cards}</div>
      <h3>Renderable draw order</h3>${elementRows}
      <h3>Render queue</h3>${queueRows}
      <h3>3D render smoke</h3>${scene3dRows}
      <h3>3D render fallbacks</h3>${scene3dFallbackRows}
      <h3>Absence diagnostics</h3>${absenceRows}
      <p class="hint">Disallowed actions: ${escapeText(disallowedActions.join(' · '))}</p>
      <p class="hint">${escapeText(scene3dRender.boundary || breakdown.boundary || readOnlyInspection.boundary || 'Display-only render breakdown inspection.')}</p>
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
    const physics = summary.physics && typeof summary.physics === 'object' && !Array.isArray(summary.physics)
      ? summary.physics
      : {};
    const scene3dCollision = summary.scene3d_collision && typeof summary.scene3d_collision === 'object' && !Array.isArray(summary.scene3d_collision)
      ? summary.scene3d_collision
      : (summary.scene3dCollision && typeof summary.scene3dCollision === 'object' && !Array.isArray(summary.scene3dCollision) ? summary.scene3dCollision : null);
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
    const scene3dCollisionRows = Array.isArray(scene3dCollision?.events) && scene3dCollision.events.length
      ? scene3dCollision.events.map((event) => `<div class="surface-row"><strong>${escapeText(event?.type || '3D collision event')}</strong><br><small>${escapeText(compactJson(event))}</small></div>`).join('')
      : '<p class="empty compact">No 3D collision event rows exported.</p>';
    const invalid3dColliderRows = Array.isArray(scene3dCollision?.invalidColliders) && scene3dCollision.invalidColliders.length
      ? scene3dCollision.invalidColliders.map((entry) => `<div class="surface-row"><strong>${escapeText(entry?.colliderId || entry?.colliderRef || entry?.nodeId || 'invalid 3D collider')}</strong><br><small>${escapeText(compactJson(entry))}</small></div>`).join('')
      : '<p class="empty compact">No invalid 3D collider rows exported.</p>';
    const transitionRows = Array.isArray(transition?.transitions) && transition.transitions.length
      ? transition.transitions.map((event) => `<div class="surface-row"><strong>${escapeText(event?.type || event?.kind || 'scene transition')}</strong><br><small>${escapeText(compactJson(event))}</small></div>`).join('')
      : '<p class="empty compact">No transition event rows exported.</p>';
    const declaredTransitionRows = Array.isArray(transition?.declaredTransitions) && transition.declaredTransitions.length
      ? transition.declaredTransitions.map((entry) => `<div class="surface-row"><strong>${escapeText(entry?.id || 'declared transition')}</strong><br><small>${escapeText(compactJson(entry))}</small></div>`).join('')
      : '<p class="empty compact">No manifest-validated declared transitions exported.</p>';
    const animationRows = Array.isArray(events?.animationEntities) && events.animationEntities.length
      ? events.animationEntities.map((entity) => `<li><strong>${escapeText(entity?.entityId || 'entity')}</strong> · ${escapeText(entity?.mode || 'mode unknown')} · state ${escapeText(entity?.activeState || 'none')} · clip ${escapeText(entity?.currentClip || 'none')} · frame ${escapeText(entity?.frameIndex ?? 'unknown')}</li>`).join('')
      : '<li>No animation event entity rows exported.</li>';
    const vfxRows = Array.isArray(events?.vfxEvents) && events.vfxEvents.length
      ? events.vfxEvents.map((event) => `<div class="surface-row"><strong>${escapeText(event?.emitterId || event?.type || 'vfx event')}</strong><br><small>${escapeText(compactJson(event))}</small></div>`).join('')
      : '<p class="empty compact">No VFX event rows exported.</p>';
    const audioRows = Array.isArray(events?.audioEvents) && events.audioEvents.length
      ? events.audioEvents.map((event) => `<div class="surface-row"><strong>${escapeText(event?.name || event?.type || event?.kind || 'audio event')}</strong><br><small>${escapeText(event?.intentKind || event?.kind || 'sound')} · bus ${escapeText(event?.busId || 'default')} · volume ${escapeText(event?.volume ?? 'unknown')} · ${escapeText(compactJson(event))}</small></div>`).join('')
      : '<p class="empty compact">No audio event rows exported.</p>';
    const audioWarnings = Array.isArray(events?.audioWarnings) && events.audioWarnings.length
      ? events.audioWarnings.map((warning) => `<div class="surface-row"><strong>${escapeText(warning?.warning || 'audio warning')}</strong><br><small>${escapeText(compactJson(warning))}</small></div>`).join('')
      : '<p class="empty compact">No audio limitation warnings exported.</p>';
    return `<section id="runtime-event-inspection" class="panel"><h2>Collision/transition/event inspection</h2>
      <p class="hint">Read-only runtime event summaries from Rust-exported evidence. This panel is inspection-only: it does not execute commands, mutate scenes, or persist browser state.</p>
      ${warnings.length ? `<div class="error">${escapeText(warnings.join(' · '))}</div>` : '<div class="hint">Collision/transition/event summaries loaded.</div>'}
      <div class="field-grid">
        <div><strong>Collision</strong><br>${escapeText(collision?.colliderEntityCount ?? 0)} collider(s), ${escapeText(collision?.collisionEventCount ?? 0)} event(s)</div>
        <div><strong>3D collision</strong><br>${escapeText(scene3dCollision?.contactCount ?? 0)} contact(s), ${escapeText(scene3dCollision?.triggerCount ?? 0)} trigger(s), ${escapeText(scene3dCollision?.invalidColliderCount ?? 0)} invalid</div>
        <div><strong>Physics contacts</strong><br>${escapeText(physics?.contactPairCount ?? 0)} pair(s), pairs ${escapeText(compactJson(physics?.contactPairs || []))}, blocked ${escapeText(compactJson(physics?.blockedMovement || {}))}</div>
        <div><strong>Transition</strong><br>${escapeText(transition?.currentSceneId ?? 'unknown scene')} · ${escapeText(transition?.declaredTransitionCount ?? 0)} declared · ${escapeText(transition?.transitionEventCount ?? 0)} event(s) · last reload ${escapeText(transition?.lastReloadStatus ?? 'none')}</div>
        <div><strong>Runtime events</strong><br>${escapeText(events?.animationEntityCount ?? 0)} animation entit(ies), ${escapeText(events?.audioEventCount ?? 0)} audio event(s), ${escapeText(events?.audioWarningCount ?? 0)} audio warning(s), ${escapeText(events?.collisionEventCount ?? 0)} collision event(s), ${escapeText(events?.vfxEventCount ?? 0)} VFX event(s)</div>
      </div>
      <h3>Collision rules</h3><div class="field-grid">${collisionRules}</div>
      <h3>Collision events</h3>${collisionRows}
      <h3>3D collision events</h3>${scene3dCollisionRows}
      <h3>Invalid 3D colliders</h3>${invalid3dColliderRows}
      <p class="hint">${escapeText(scene3dCollision?.boundary || 'Read-only bounded 3D collision evidence; no full 3D physics engine claim.')}</p>
      <h3>Declared scene transitions</h3>${declaredTransitionRows}
      <h3>Scene transition events</h3>${transitionRows}
      <h3>Animation entities</h3><ul>${animationRows}</ul>
      <h3>VFX events</h3>${vfxRows}
      <h3>Audio events</h3>${audioRows}
      <h3>Audio limitation warnings</h3>${audioWarnings}
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

  function sourcePatchEvidenceBundleArtifacts(run) {
    return Array.isArray(run?.mutation_artifacts || run?.mutationArtifacts)
      ? (run.mutation_artifacts || run.mutationArtifacts).filter((artifact) => artifact.id === 'source-patch-evidence-bundle' || artifact.path === 'mutation/source-patch-evidence-bundle.json')
      : [];
  }

  function behaviorEvidenceModel(run) {
    return run?.behavior_evidence || run?.behaviorEvidence || {};
  }

  function renderBehaviorEvidenceLifecycleSurface(run) {
    const model = behaviorEvidenceModel(run);
    const bundles = Array.isArray(model.bundles) ? model.bundles : [];
    if (!model.present && !bundles.length) {
      return '<section id="behavior-evidence-lifecycle" class="panel"><h2>Behavior evidence lifecycle</h2><p class="empty">No behavior evidence bundle is indexed for this run.</p><p class="hint">Read-only Studio surface. The browser cannot execute scripts, run command bridges, auto-apply behavior changes, or write trusted files.</p></section>';
    }
    const rows = bundles.map((bundle) => {
      const failures = Array.isArray(bundle.observed_failures || bundle.observedFailures) ? (bundle.observed_failures || bundle.observedFailures) : [];
      const hypotheses = Array.isArray(bundle.next_step_hypotheses || bundle.nextStepHypotheses) ? (bundle.next_step_hypotheses || bundle.nextStepHypotheses) : [];
      const blockers = Array.isArray(bundle.blocked_reasons || bundle.blockedReasons) ? (bundle.blocked_reasons || bundle.blockedReasons) : [];
      const refs = Array.isArray(bundle.evidence_refs || bundle.evidenceRefs) ? (bundle.evidence_refs || bundle.evidenceRefs) : [];
      const guardrails = Array.isArray(bundle.guardrails) ? bundle.guardrails : [];
      const failureText = failures.map((failure) => `${failure.scenario_id || failure.scenarioId || 'scenario'}:${failure.summary || 'missing'}:${failure.evidence_ref || failure.evidenceRef || 'missing'}`).join(' · ');
      const hypothesisText = hypotheses.map((hypothesis) => `${hypothesis.id || 'hypothesis'}:${hypothesis.summary || 'missing'}`).join(' · ');
      return `<div class="surface-row"><strong>${escapeText(bundle.bundle_id || bundle.bundleId || 'behavior evidence bundle')}</strong> ${surfaceState(!bundle.read_error && !bundle.readError, bundle.status || 'unknown')}<br>
        <small>${escapeText(bundle.path || 'no path')} · refs ${escapeText(bundle.lifecycle_ref_count ?? bundle.lifecycleRefCount ?? 0)}</small><br>
        ${bundle.read_error || bundle.readError ? `<small>Malformed: ${escapeText(bundle.read_error || bundle.readError)}</small><br>` : ''}
        ${blockers.length ? `<small>Blocked: ${escapeText(blockers.join(' · '))}</small><br>` : ''}
        <small>Observed failures: ${escapeText(failureText || 'none')}</small><br>
        <small>Next-step hypotheses: ${escapeText(hypothesisText || 'none')}</small><br>
        <small>Evidence refs: ${escapeText(refs.join(' · ') || 'none')}</small><br>
        <small>Guardrails: ${escapeText(guardrails.join(' · ') || 'read-only rust/local untracked behavior evidence.')}</small>
      </div>`;
    }).join('');
    return `<section id="behavior-evidence-lifecycle" class="panel"><h2>Behavior evidence lifecycle</h2>
      <p class="hint">Status ${escapeText(model.status || 'unknown')} · bundles ${escapeText(model.bundle_count ?? model.bundleCount ?? bundles.length)} · malformed ${escapeText(model.malformed_count ?? model.malformedCount ?? 0)} · lifecycle refs ${escapeText(model.lifecycle_ref_count ?? model.lifecycleRefCount ?? 0)} · failures ${escapeText(model.observed_failure_count ?? model.observedFailureCount ?? 0)} · hypotheses ${escapeText(model.next_step_hypothesis_count ?? model.nextStepHypothesisCount ?? 0)}</p>
      <p class="hint">${escapeText(model.boundary || 'read-only structured behavior lifecycle evidence; no command bridge, auto-apply, or trusted writes.')}</p>
      ${rows || '<div class="surface-row">No readable behavior evidence bundles.</div>'}
    </section>`;
  }

  function renderSourcePatchEvidenceBundleSurface(run) {
    const bundles = sourcePatchEvidenceBundleArtifacts(run);
    if (!bundles.length) {
      return '<section id="source-patch-evidence-bundle" class="panel"><h2>Source patch evidence bundle</h2><p class="empty">No source patch evidence bundle is exported for this run.</p><p class="hint">Read-only Studio surface. The browser cannot apply source patches, merge branches, execute commands, write trusted files, or bypass review gates.</p></section>';
    }
    const rows = bundles.map((artifact) => {
      const value = artifact.value || {};
      const notices = Array.isArray(value.forbiddenActionNotices) ? value.forbiddenActionNotices : [];
      const patchSummary = value.patchSummary || {};
      const fileClassSummary = value.fileClassSummary || {};
      const riskIds = Array.isArray(value.riskIds) ? value.riskIds : [];
      const blockedReasons = Array.isArray(value.blockedReasons) ? value.blockedReasons : [];
      const dryRunSummary = value.dryRunSummary || {};
      const requiredTestSummary = value.requiredTestSummary || {};
      const reviewSummary = value.reviewSummary || {};
      const linkedEvidence = Array.isArray(value.linkedEvidence) ? value.linkedEvidence : [];
      const refs = [value.previewRef, value.fileClassReportRef, value.diffIntegrityReportRef, value.sandboxReportRef, value.testSummaryRef, value.reviewDecisionRef]
        .filter(Boolean)
        .map((ref) => `${ref.kind || 'artifact'}:${ref.path || 'missing'}`);
      const linkedEvidenceText = linkedEvidence.map((ref) => `${ref.kind || 'artifact'}:${ref.path || 'missing'}`).join(' · ');
      const fileClassText = `allowed:${fileClassSummary.allowed ?? 0} review-held:${fileClassSummary.reviewHeld ?? 0} blocked:${fileClassSummary.blocked ?? 0} highest-risk:${fileClassSummary.highestRisk || 'unknown'}`;
      const requiredCommands = Array.isArray(requiredTestSummary.commands) ? requiredTestSummary.commands : [];
      const dryRunRef = dryRunSummary.reportRef ? `${dryRunSummary.reportRef.kind || 'artifact'}:${dryRunSummary.reportRef.path || 'missing'}` : 'none';
      const reviewRef = reviewSummary.decisionRef ? `${reviewSummary.decisionRef.kind || 'artifact'}:${reviewSummary.decisionRef.path || 'missing'}` : 'none';
      return `<div class="surface-row"><strong>${escapeText(value.bundleId || artifact.id || 'source patch bundle')}</strong> ${surfaceState(true, value.status || 'unknown')}<br>
        <small>Preview ${escapeText(value.patchPreviewId || 'unknown')} · ${escapeText(artifact.path || 'mutation/source-patch-evidence-bundle.json')}</small><br>
        <small>Patch summary: ${escapeText(patchSummary.title || 'not recorded')} · targets ${escapeText(patchSummary.targetCount ?? 'unknown')} · changed lines ${escapeText(patchSummary.changedLines ?? 'unknown')}</small><br>
        <small>Expected behavior: ${escapeText(patchSummary.expectedBehaviorChange || 'not recorded')}</small><br>
        <small>File classes: ${escapeText(fileClassText)}</small><br>
        <small>Risk: ${escapeText(riskIds.join(' · ') || 'none')}</small><br>
        <small>Dry-run: ${escapeText(dryRunSummary.status || 'unknown')} · policy ${escapeText(dryRunSummary.allowlistPolicyId || 'unknown')} · report ${escapeText(dryRunRef)}</small><br>
        <small>Required tests: ${escapeText(requiredCommands.join(' · ') || 'none')} · policy ${escapeText(requiredTestSummary.allowlistPolicyId || 'unknown')}</small><br>
        <small>Review: ${escapeText(reviewSummary.status || 'unknown')} · decision ${escapeText(reviewRef)}</small><br>
        <small>Linked evidence: ${escapeText(linkedEvidenceText || 'none')}</small><br>
        ${blockedReasons.length ? `<small>Blocked: ${escapeText(blockedReasons.join(' · '))}</small><br>` : ''}
        <small>Refs: ${escapeText(refs.join(' · ') || 'none')}</small><br>
        <small>Forbidden: ${escapeText(notices.map((notice) => notice.action || 'unknown').join(' · ') || 'none')}</small>
      </div>`;
    }).join('');
    return `<section id="source-patch-evidence-bundle" class="panel"><h2>Source patch evidence bundle</h2>
      <p class="hint">Escaped read-only bundle evidence from Rust-exported dashboard data. This surface does not apply patches, merge branches, execute commands, write trusted files, or add a command bridge.</p>
      ${rows}
    </section>`;
  }

  function sourcePatchApplyTransactionArtifacts(run) {
    return Array.isArray(run?.mutation_artifacts || run?.mutationArtifacts)
      ? (run.mutation_artifacts || run.mutationArtifacts).filter((artifact) => artifact.id === 'source-patch-apply-transaction' || artifact.path === 'mutation/source-patch-apply-transaction.json')
      : [];
  }

  function sourcePatchStaleTargetGuardArtifacts(run) {
    return Array.isArray(run?.mutation_artifacts || run?.mutationArtifacts)
      ? (run.mutation_artifacts || run.mutationArtifacts).filter((artifact) => artifact.id === 'source-patch-stale-target-guard' || artifact.path === 'mutation/source-patch-stale-target-guard.json')
      : [];
  }

  function renderSourcePatchStaleTargetGuardSurface(run) {
    const guards = sourcePatchStaleTargetGuardArtifacts(run);
    if (!guards.length) {
      return '<section id="source-patch-stale-target-guard" class="panel"><h2>Source patch stale target guard</h2><p class="empty">No source patch stale target guard is exported for this run.</p><p class="hint">Read-only Studio surface. The browser cannot apply source patches, merge branches, execute commands, write trusted files, or bypass review gates.</p></section>';
    }
    const rows = guards.map((artifact) => {
      const value = artifact.value || {};
      const freshness = value.evidenceFreshness || value.evidence_freshness || {};
      const validation = value.readModel || value.read_model || value.validation || {};
      const targets = Array.isArray(value.targets) ? value.targets : [];
      const blockers = Array.isArray(validation.blockedReasons || validation.blocked_reasons)
        ? (validation.blockedReasons || validation.blocked_reasons)
        : (Array.isArray(value.blockedReasons || value.blocked_reasons) ? (value.blockedReasons || value.blocked_reasons) : []);
      const forbidden = Array.isArray(validation.forbiddenActions || validation.forbidden_actions)
        ? (validation.forbiddenActions || validation.forbidden_actions)
        : ['apply_patch', 'merge_branch', 'execute_command', 'write_trusted_file', 'browser_command_bridge'];
      const refs = [freshness.patchPreviewRef, freshness.sandboxReportRef, freshness.reviewDecisionRef, freshness.fileClassReportRef, freshness.diffIntegrityReportRef, freshness.applyTransactionRef, value.worktreeContextRef || value.worktree_context_ref]
        .filter(Boolean)
        .map((ref) => typeof ref === 'string' ? ref : `${ref.kind || 'artifact'}:${ref.path || 'missing'}`);
      return `<div class="surface-row"><strong>${escapeText(value.guardId || value.guard_id || artifact.id || 'source patch stale target guard')}</strong> ${surfaceState(true, validation.status || value.status || 'unknown')}<br>
        <small>${escapeText(validation.readinessLabel || validation.readiness_label || 'stale-target readiness metadata only')} · ${escapeText(artifact.path || 'mutation/source-patch-stale-target-guard.json')}</small><br>
        <small>Targets: ${escapeText(targets.map((target) => `${target.path || 'unknown'}:${target.fileClass || target.file_class || 'unknown'}:${target.fileStatus || target.file_status || 'unknown'}`).join(' · ') || 'none')}</small><br>
        <small>Refs: ${escapeText(refs.join(' · ') || 'none')}</small><br>
        ${blockers.length ? `<small>Blocked: ${escapeText(blockers.join(' · '))}</small><br>` : ''}
        <small>Forbidden: ${escapeText(forbidden.join(' · '))}</small>
      </div>`;
    }).join('');
    return `<section id="source-patch-stale-target-guard" class="panel"><h2>Source patch stale target guard</h2>
      <p class="hint">Escaped read-only stale-target readiness evidence from Rust-exported dashboard data. This surface does not apply patches, merge branches, execute commands, write trusted files, or add a command bridge.</p>
      ${rows}
    </section>`;
  }

  function renderSourcePatchApplyTransactionSurface(run) {
    const transactions = sourcePatchApplyTransactionArtifacts(run);
    if (!transactions.length) {
      return '<section id="source-patch-apply-transaction" class="panel"><h2>Source patch apply transaction</h2><p class="empty">No source patch apply transaction is exported for this run.</p><p class="hint">Read-only Studio surface. The browser cannot apply source patches, merge branches, execute commands, write trusted files, or bypass review gates.</p></section>';
    }
    const rows = transactions.map((artifact) => {
      const value = artifact.value || {};
      const evidence = value.evidence || {};
      const validation = value.readModel || value.read_model || value.validation || {};
      const targets = Array.isArray(value.targets) ? value.targets : [];
      const blockers = Array.isArray(validation.blockedReasons || validation.blocked_reasons)
        ? (validation.blockedReasons || validation.blocked_reasons)
        : (Array.isArray(value.blockedReasons || value.blocked_reasons) ? (value.blockedReasons || value.blocked_reasons) : []);
      const forbidden = Array.isArray(validation.forbiddenActions || validation.forbidden_actions)
        ? (validation.forbiddenActions || validation.forbidden_actions)
        : ['apply_patch', 'merge_branch', 'execute_command', 'write_trusted_file', 'browser_command_bridge'];
      const refs = [evidence.patchPreviewRef, evidence.sandboxReportRef, evidence.reviewDecisionRef, evidence.fileClassReportRef, evidence.diffIntegrityReportRef, value.rollbackRef?.rollbackPlanRef || value.rollback_ref?.rollback_plan_ref]
        .filter(Boolean)
        .map((ref) => typeof ref === 'string' ? ref : `${ref.kind || 'artifact'}:${ref.path || 'missing'}`);
      return `<div class="surface-row"><strong>${escapeText(value.transactionId || value.transaction_id || artifact.id || 'source patch transaction')}</strong> ${surfaceState(true, validation.status || value.status || 'unknown')}<br>
        <small>${escapeText(validation.readinessLabel || validation.readiness_label || 'readiness metadata only')} · ${escapeText(artifact.path || 'mutation/source-patch-apply-transaction.json')}</small><br>
        <small>Targets: ${escapeText(targets.map((target) => `${target.path || 'unknown'}:${target.fileClass || target.file_class || 'unknown'}`).join(' · ') || 'none')}</small><br>
        <small>Refs: ${escapeText(refs.join(' · ') || 'none')}</small><br>
        ${blockers.length ? `<small>Blocked: ${escapeText(blockers.join(' · '))}</small><br>` : ''}
        <small>Forbidden: ${escapeText(forbidden.join(' · '))}</small>
      </div>`;
    }).join('');
    return `<section id="source-patch-apply-transaction" class="panel"><h2>Source patch apply transaction</h2>
      <p class="hint">Escaped read-only transaction readiness evidence from Rust-exported dashboard data. This surface does not apply patches, merge branches, execute commands, write trusted files, or add a command bridge.</p>
      ${rows}
    </section>`;
  }

  function renderPluginRegistryBrowserSurface(run) {
    const registry = run?.plugin_registry || run?.pluginRegistry || {};
    if (!registry.present) {
      return `<section id="plugin-registry-browser" class="panel"><h2>Plugin registry browser</h2><p class="empty">${escapeText(registry.empty_state || 'No plugin registry evidence is available for this run.')}</p><p class="hint">Read-only Studio surface. The browser does not install, update, delete, enable, or execute plugins.</p></section>`;
    }
    const registries = Array.isArray(registry.registries) ? registry.registries : [];
    const refs = Array.isArray(registry.evidence_refs || registry.evidenceRefs) ? (registry.evidence_refs || registry.evidenceRefs) : [];
    const cards = [
      ['Status', registry.status || 'unknown'],
      ['Registries', registry.registry_count ?? registry.registryCount ?? registries.length],
      ['Plugins', registry.plugin_count ?? registry.pluginCount ?? 0],
      ['Blocked', registry.blocked_count ?? registry.blockedCount ?? 0],
      ['Malformed', registry.malformed_count ?? registry.malformedCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const rows = registries.flatMap((item) => {
      const plugins = Array.isArray(item.plugins) ? item.plugins : [];
      return plugins.map((plugin) => {
        const caps = Array.isArray(plugin.declared_capabilities || plugin.declaredCapabilities) ? (plugin.declared_capabilities || plugin.declaredCapabilities).join(', ') : 'none';
        const points = Array.isArray(plugin.extension_points || plugin.extensionPoints) ? (plugin.extension_points || plugin.extensionPoints).join(', ') : 'none';
        const reasons = Array.isArray(plugin.blocked_reasons || plugin.blockedReasons) ? (plugin.blocked_reasons || plugin.blockedReasons).join(' · ') : '';
        const panels = Array.isArray(plugin.dashboard_panels || plugin.dashboardPanels) ? (plugin.dashboard_panels || plugin.dashboardPanels) : [];
        const panelRows = panels.length
          ? `<div class="hint">Dashboard panels: ${escapeText(panels.slice(0, 6).map((panel) => `${panel.panel_id || panel.panelId || 'panel'}:${panel.data_source_key || panel.dataSourceKey || 'unknown'}:${panel.template_ref || panel.templateRef || 'unknown'}:${panel.layout_hint || panel.layoutHint || 'unknown'}`).join(' · '))}</div>`
          : '';
        const panelBoundaries = panels.length
          ? `<small>${escapeText(panels.map((panel) => panel.boundary || 'Declarative allowlisted read-only dashboard panel descriptor; no JavaScript, no commands, no trusted writes.').join(' · '))}</small>`
          : '';
        const templates = Array.isArray(plugin.scenario_templates || plugin.scenarioTemplates) ? (plugin.scenario_templates || plugin.scenarioTemplates) : [];
        const templateRows = templates.length
          ? `<div class="hint">Scenario templates: ${escapeText(templates.slice(0, 6).map((template) => {
              const params = Array.isArray(template.parameters) ? template.parameters : [];
              const paramSummary = params.slice(0, 8).map((parameter) => `${parameter.name || 'parameter'}:${parameter.parameter_type || parameter.parameterType || parameter.type || 'unknown'}${parameter.required ? ':required' : ''}`).join(',');
              const games = Array.isArray(template.supported_game_types || template.supportedGameTypes) ? (template.supported_game_types || template.supportedGameTypes).join(',') : 'none';
              return `${template.template_id || template.templateId || 'scenario-template'}:${template.expected_evidence_type || template.expectedEvidenceType || 'unknown'}:${games}:${paramSummary || 'no-parameters'}`;
            }).join(' · '))}</div>`
          : '';
        const templateBoundaries = templates.length
          ? `<small>${escapeText(templates.map((template) => {
              const hints = Array.isArray(template.validation_hints || template.validationHints) ? (template.validation_hints || template.validationHints).join(' · ') : 'no validation hints';
              return `${hints} · ${template.boundary || 'Declarative read-only scenario template metadata only; no executable scripts, no commands, no network, no source mutation, no trusted writes.'}`;
            }).join(' · '))}</small>`
          : '';
        return `<div class="surface-row"><strong>${escapeText(plugin.plugin_id || plugin.pluginId || 'unknown plugin')}</strong> ${surfaceState(true, plugin.validation_status || plugin.validationStatus || 'unknown')} ${surfaceState(true, plugin.compatibility_status || plugin.compatibilityStatus || 'unknown')}<br><small>registry ${escapeText(item.registry_id || item.registryId || 'unknown')} · version ${escapeText(plugin.manifest_version || plugin.manifestVersion || 'unknown')} · hash ${escapeText(plugin.manifest_hash || plugin.manifestHash || 'missing')} · capabilities ${escapeText(caps)} · extension points ${escapeText(points)} · manifest ${escapeText(plugin.manifest_path || plugin.manifestPath || 'missing')}</small>${panelRows}${panelBoundaries}${templateRows}${templateBoundaries}${reasons ? `<div class="hint">Blocked: ${escapeText(reasons)}</div>` : ''}</div>`;
      });
    }).slice(0, 24).join('') || '<div class="surface-row">No plugin descriptor rows exported.</div>';
    return `<section id="plugin-registry-browser" class="panel"><h2>Plugin registry browser</h2>
      <p class="hint">${escapeText(registry.boundary || 'Read-only plugin registry evidence; Studio does not execute plugins or write trusted files.')}</p>
      <div class="field-grid">${cards}</div>${rows}
      <p class="hint">Evidence refs: ${escapeText(refs.join(' · ') || 'none')}</p>
      <small>No install, update, delete, enable, command execution, network install, marketplace, credential, publish, deploy, signing, upload, source mutation, or trusted-write controls are rendered.</small>
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



  function arrayField(value, ...keys) {
    if (Array.isArray(value)) return value;
    if (!value || typeof value !== 'object') return [];
    for (const key of keys) {
      if (Array.isArray(value[key])) return value[key];
    }
    return [];
  }

  function refText(ref) {
    if (!ref || typeof ref !== 'object') return String(ref ?? '');
    return ref.path || ref.pathHint || ref.id || ref.kind || JSON.stringify(ref);
  }

  function summarizeObjectRef(value) {
    if (!value || typeof value !== 'object') return 'none';
    return Object.entries(value)
      .filter(([, entryValue]) => entryValue !== undefined && entryValue !== null && entryValue !== '')
      .map(([key, entryValue]) => `${key}:${typeof entryValue === 'object' ? JSON.stringify(entryValue) : entryValue}`)
      .join(' · ') || 'none';
  }

  function behaviorInspectionModel(run) {
    const source = run?.behavior_inspection || run?.behaviorInspection || {};
    const behaviorSource = source.behaviors || source.behavior_model || source.behaviorModel || run?.behavior_model || run?.behaviorModel || run?.gameplay_behavior_model || run?.gameplayBehaviorModel || {};
    const eventSource = source.eventSignals || source.event_signals || source.event_signal_model || source.eventSignalModel || run?.event_signals || run?.eventSignals || run?.gameplay_event_signals || run?.gameplayEventSignals || {};
    const stateSource = source.stateMachines || source.state_machines || source.state_machine_model || source.stateMachineModel || run?.state_machines || run?.stateMachines || run?.gameplay_state_machines || run?.gameplayStateMachines || {};
    const abilitySource = source.abilities || source.ability_actions || source.abilityActionModel || source.ability_action_model || run?.ability_actions || run?.abilityActions || run?.gameplay_ability_actions || run?.gameplayAbilityActions || {};
    const reviewApplySource = source.reviewApply || source.review_apply || run?.behavior_review_apply || run?.behaviorReviewApply || {};
    const behaviors = arrayField(behaviorSource, 'behaviors', 'records', 'rows');
    const events = arrayField(eventSource, 'events', 'records', 'rows');
    const stateMachines = arrayField(stateSource, 'stateMachines', 'state_machines', 'machines', 'records', 'rows');
    const abilities = arrayField(abilitySource, 'abilities', 'actions', 'records', 'rows');
    const reviews = arrayField(reviewApplySource, 'reviews', 'reviewDecisions', 'review_decisions', 'records');
    const applies = arrayField(reviewApplySource, 'applies', 'applyTransactions', 'apply_transactions', 'transactions');
    const malformedReasons = [source, behaviorSource, eventSource, stateSource, abilitySource, reviewApplySource]
      .flatMap((item) => arrayField(item, 'malformedReasons', 'malformed_reasons'));
    return {
      present: Boolean(source.present ?? (behaviors.length || events.length || stateMachines.length || abilities.length || reviews.length || applies.length)),
      status: source.status || behaviorSource.status || eventSource.status || stateSource.status || abilitySource.status || reviewApplySource.status || 'missing',
      boundary: source.boundary || 'Studio behavior inspection is escaped read-only local evidence. It does not execute scripts, eval, dynamic import, load plugins, run a command bridge, write trusted files, mutate source, auto-apply, auto-merge, self-approve, or persist browser state.',
      behaviorSource,
      eventSource,
      stateSource,
      abilitySource,
      reviewApplySource,
      behaviors,
      events,
      stateMachines,
      abilities,
      reviews,
      applies,
      malformedReasons,
    };
  }

  function renderBehaviorListPanel(run) {
    const model = behaviorInspectionModel(run);
    if (!model.present && model.behaviors.length === 0) {
      return `<section id="behavior-list-panel" class="panel"><h2>Behavior list panel</h2><p class="empty">No structured behavior list read model is available for this run.</p><p class="hint">${escapeText(model.boundary)}</p></section>`;
    }
    const rows = model.behaviors.slice(0, 12).map((behavior, index) => {
      const trigger = behavior.trigger || behavior.triggers?.[0] || {};
      const conditions = arrayField(behavior, 'conditions', 'guards');
      const actions = arrayField(behavior, 'actions', 'effects');
      const blockers = arrayField(behavior, 'blockedReasons', 'blocked_reasons');
      const refs = arrayField(behavior, 'evidenceRefs', 'evidence_refs').map(refText);
      return `<article class="surface-row"><strong>${escapeText(behavior.id || behavior.behaviorId || `behavior-${index + 1}`)}</strong> ${surfaceState((behavior.status || model.status) !== 'blocked', behavior.status || model.status || 'unknown')}<br>
        <small>${escapeText(behavior.label || behavior.summary || 'structured behavior row')} · target ${escapeText(summarizeObjectRef(behavior.target || behavior.targetRef || behavior.target_ref))}</small><br>
        <small>Trigger: ${escapeText(summarizeObjectRef(trigger))}</small><br>
        <small>Conditions ${escapeText(conditions.length)} · actions ${escapeText(actions.length)}</small><br>
        ${blockers.length ? `<small>Blocked: ${escapeText(blockers.join(' · '))}</small><br>` : ''}
        <small>Evidence refs: ${escapeText(refs.join(' · ') || 'none')}</small>
      </article>`;
    }).join('') || '<div class="surface-row">No behavior rows exported.</div>';
    return `<section id="behavior-list-panel" class="panel"><h2>Behavior list panel</h2>
      <p class="hint">Status ${escapeText(model.behaviorSource.status || model.status)} · behaviors ${escapeText(model.behaviors.length)}. ${escapeText(model.boundary)}</p>
      ${rows}
    </section>`;
  }

  function renderBehaviorEventSignalPanel(run) {
    const model = behaviorInspectionModel(run);
    const ordered = [...model.events].sort((left, right) => Number(left.tick ?? 0) - Number(right.tick ?? 0) || Number(left.orderingIndex ?? left.ordering_index ?? 0) - Number(right.orderingIndex ?? right.ordering_index ?? 0) || String(left.id || '').localeCompare(String(right.id || '')));
    if (!model.present && ordered.length === 0) {
      return `<section id="behavior-event-signal-panel" class="panel"><h2>Event/signal panel</h2><p class="empty">No gameplay event/signal read model is available for this run.</p><p class="hint">${escapeText(model.boundary)}</p></section>`;
    }
    const consumedCount = ordered.filter((event) => event.consumed === true).length;
    const rows = ordered.slice(0, 12).map((event, index) => {
      const blockers = [event.blockedReason || event.blocked_reason].filter(Boolean).concat(arrayField(event, 'blockedReasons', 'blocked_reasons'));
      const refs = arrayField(event, 'evidenceRefs', 'evidence_refs').map(refText);
      return `<article class="surface-row"><strong>${escapeText(event.id || `event-${index + 1}`)}</strong> ${surfaceState(event.consumed === true, event.consumed === true ? 'consumed' : 'visible-unconsumed')}<br>
        <small>${escapeText(event.eventType || event.event_type || 'unknown-event')} · signal ${escapeText(event.signalName || event.signal_name || 'none')} · tick ${escapeText(event.tick ?? 'unknown')}</small><br>
        <small>Source ${escapeText(summarizeObjectRef(event.source))} · target ${escapeText(summarizeObjectRef(event.target))}</small><br>
        ${blockers.length ? `<small>Blocked: ${escapeText(blockers.join(' · '))}</small><br>` : ''}
        <small>Consumed by: ${escapeText(arrayField(event, 'consumedBy', 'consumed_by').join(' · ') || 'none')} · evidence ${escapeText(refs.join(' · ') || 'none')}</small>
      </article>`;
    }).join('') || '<div class="surface-row">No event/signal rows exported.</div>';
    return `<section id="behavior-event-signal-panel" class="panel"><h2>Event/signal panel</h2>
      <p class="hint">Status ${escapeText(model.eventSource.status || model.status)} · events ${escapeText(ordered.length)} · consumed ${escapeText(consumedCount)} · unconsumed ${escapeText(ordered.length - consumedCount)}. Read-only queue inspection; the browser does not dispatch events or signals.</p>
      ${rows}
    </section>`;
  }

  function renderBehaviorStateMachinePanel(run) {
    const model = behaviorInspectionModel(run);
    if (!model.present && model.stateMachines.length === 0) {
      return `<section id="behavior-state-machine-panel" class="panel"><h2>State machine panel</h2><p class="empty">No gameplay state machine read model is available for this run.</p><p class="hint">${escapeText(model.boundary)}</p></section>`;
    }
    const rows = model.stateMachines.slice(0, 10).map((machine, index) => {
      const states = arrayField(machine, 'states');
      const transitions = arrayField(machine, 'transitions');
      const blockers = arrayField(machine, 'blockedReasons', 'blocked_reasons');
      const refs = arrayField(machine, 'evidenceRefs', 'evidence_refs').map(refText);
      const transitionText = transitions.slice(0, 4).map((transition) => `${transition.id || 'transition'}:${transition.from || '?'}→${transition.to || '?'}:${transition.trigger?.kind || 'trigger'}`).join(' · ');
      return `<article class="surface-row"><strong>${escapeText(machine.id || machine.machineId || `state-machine-${index + 1}`)}</strong> ${surfaceState((machine.status || model.status) !== 'blocked', machine.status || model.status || 'unknown')}<br>
        <small>${escapeText(machine.label || 'structured state machine')} · target ${escapeText(summarizeObjectRef(machine.target))} · initial ${escapeText(machine.initialStateId || machine.initial_state_id || 'unknown')}</small><br>
        <small>States: ${escapeText(states.map((state) => state.id || state.label || 'state').join(' · ') || 'none')}</small><br>
        <small>Transitions: ${escapeText(transitionText || 'none')}</small><br>
        ${blockers.length ? `<small>Blocked: ${escapeText(blockers.join(' · '))}</small><br>` : ''}
        <small>Evidence refs: ${escapeText(refs.join(' · ') || 'none')}</small>
      </article>`;
    }).join('') || '<div class="surface-row">No state machine rows exported.</div>';
    return `<section id="behavior-state-machine-panel" class="panel"><h2>State machine panel</h2>
      <p class="hint">Status ${escapeText(model.stateSource.status || model.status)} · state machines ${escapeText(model.stateMachines.length)}. Display-only transitions; no runtime state changes are dispatched.</p>
      ${rows}
    </section>`;
  }

  function renderBehaviorAbilityActionPanel(run) {
    const model = behaviorInspectionModel(run);
    if (!model.present && model.abilities.length === 0) {
      return `<section id="behavior-ability-action-panel" class="panel"><h2>Ability/action panel</h2><p class="empty">No gameplay ability/action read model is available for this run.</p><p class="hint">${escapeText(model.boundary)}</p></section>`;
    }
    const rows = model.abilities.slice(0, 12).map((ability, index) => {
      const blockers = arrayField(ability, 'blockedReasons', 'blocked_reasons');
      const refs = arrayField(ability, 'evidenceRefs', 'evidence_refs').map(refText);
      const costs = arrayField(ability, 'costs').map((cost) => summarizeObjectRef(cost)).join(' · ');
      return `<article class="surface-row"><strong>${escapeText(ability.id || ability.abilityId || `ability-${index + 1}`)}</strong> ${surfaceState((ability.status || model.status) !== 'blocked', ability.runtimeStatus || ability.runtime_status || ability.status || model.status || 'unknown')}<br>
        <small>${escapeText(ability.label || ability.actionId || ability.action_id || 'structured ability/action')} · target ${escapeText(summarizeObjectRef(ability.target))}</small><br>
        <small>Trigger: ${escapeText(summarizeObjectRef(ability.trigger))}</small><br>
        <small>Effect: ${escapeText(summarizeObjectRef(ability.effect))} · costs ${escapeText(costs || 'none')}</small><br>
        ${blockers.length ? `<small>Blocked: ${escapeText(blockers.join(' · '))}</small><br>` : ''}
        <small>Evidence refs: ${escapeText(refs.join(' · ') || 'none')}</small>
      </article>`;
    }).join('') || '<div class="surface-row">No ability/action rows exported.</div>';
    return `<section id="behavior-ability-action-panel" class="panel"><h2>Ability/action panel</h2>
      <p class="hint">Status ${escapeText(model.abilitySource.status || model.status)} · abilities/actions ${escapeText(model.abilities.length)}. Display-only action metadata; no ability is executed from Studio.</p>
      ${rows}
    </section>`;
  }

  function renderBehaviorReviewApplyStatusSurface(run) {
    const model = behaviorInspectionModel(run);
    if (!model.present && model.reviews.length === 0 && model.applies.length === 0) {
      return `<section id="behavior-review-apply-status" class="panel"><h2>Review/apply status panel</h2><p class="empty">No behavior review/apply status read model is available for this run.</p><p class="hint">${escapeText(model.boundary)}</p></section>`;
    }
    const reviewRows = model.reviews.slice(0, 8).map((review, index) => {
      const refs = arrayField(review, 'evidenceRefs', 'evidence_refs', 'linkedEvidence', 'linked_evidence').map(refText);
      return `<li><strong>${escapeText(review.id || review.decisionId || review.decision_id || `review-${index + 1}`)}</strong> ${escapeText(review.status || review.decision || 'unknown')} · reviewer ${escapeText(review.reviewer || review.reviewerId || review.reviewer_id || 'unrecorded')} · evidence ${escapeText(refs.join(' · ') || 'none')}</li>`;
    }).join('') || '<li>No review decision rows exported.</li>';
    const applyRows = model.applies.slice(0, 8).map((apply, index) => {
      const blockers = arrayField(apply, 'blockedReasons', 'blocked_reasons');
      const refs = arrayField(apply, 'evidenceRefs', 'evidence_refs', 'linkedEvidence', 'linked_evidence').map(refText);
      return `<li><strong>${escapeText(apply.id || apply.transactionId || apply.transaction_id || `apply-${index + 1}`)}</strong> ${escapeText(apply.status || 'unknown')} · rollback ${escapeText(refText(apply.rollbackRef || apply.rollback_ref) || 'not recorded')} · blockers ${escapeText(blockers.join(' · ') || 'none')} · evidence ${escapeText(refs.join(' · ') || 'none')}</li>`;
    }).join('') || '<li>No apply transaction rows exported.</li>';
    const malformed = model.malformedReasons.length ? `<p class="warn">Malformed behavior inspection input: ${escapeText(model.malformedReasons.join(' · '))}</p>` : '<p class="hint">No malformed behavior inspection rows reported.</p>';
    return `<section id="behavior-review-apply-status" class="panel"><h2>Review/apply status panel</h2>
      <p class="hint">${escapeText(model.reviewApplySource.boundary || model.boundary)} Review/apply status is read-only; Studio does not approve, apply, merge, execute commands, or write trusted files.</p>
      ${malformed}
      <h3>Review decisions</h3><ul>${reviewRows}</ul>
      <h3>Apply transactions</h3><ul>${applyRows}</ul>
    </section>`;
  }

  function behaviorDraftReadModel(run) {
    const source = run?.behavior_drafts || run?.behaviorDrafts || run?.behavior_draft_status || run?.behaviorDraftStatus || {};
    const rawDrafts = source.records || source.drafts || source.behaviorDrafts || source.behavior_drafts || [];
    const drafts = Array.isArray(rawDrafts) ? rawDrafts.map((draft, index) => {
      const target = draft?.target && typeof draft.target === 'object' ? draft.target : {};
      const targetCheck = draft?.targetCheck || draft?.target_check || {};
      const blockedReasons = Array.isArray(draft?.blockedReasons)
        ? draft.blockedReasons
        : Array.isArray(draft?.blocked_reasons)
          ? draft.blocked_reasons
          : [];
      const diagnostics = Array.isArray(draft?.diagnostics) ? draft.diagnostics : [];
      return {
        draftId: draft?.draftId || draft?.draft_id || `behavior-draft-${index + 1}`,
        draftPath: draft?.draftPath || draft?.draft_path || '',
        validationStatus: draft?.validationStatus || draft?.validation_status || draft?.status || 'unknown',
        target: {
          projectId: target.projectId || target.project_id || 'unknown-project',
          scenePath: target.scenePath || target.scene_path || 'unknown.scene.json',
          sceneHash: target.sceneHash || target.scene_hash || 'unknown-hash',
        },
        linkedEvidenceCount: draft?.linkedEvidenceCount ?? draft?.linked_evidence_count ?? (Array.isArray(draft?.linkedEvidence) ? draft.linkedEvidence.length : Array.isArray(draft?.linked_evidence) ? draft.linked_evidence.length : 0),
        expectedScenarioImpactCount: draft?.expectedScenarioImpactCount ?? draft?.expected_scenario_impact_count ?? (Array.isArray(draft?.expectedScenarioImpact) ? draft.expectedScenarioImpact.length : Array.isArray(draft?.expected_scenario_impact) ? draft.expected_scenario_impact.length : 0),
        behaviorCount: draft?.behaviorCount ?? draft?.behavior_count ?? 'unknown',
        targetCheck,
        blockedReasons,
        diagnostics,
        guardrail: draft?.guardrail || draft?.boundary || 'read-only behavior draft status; does not apply trusted files or execute scripts',
      };
    }) : [];
    return {
      present: Boolean(source.present ?? drafts.length),
      status: source.status || (drafts.length ? 'available' : 'missing'),
      boundary: source.boundary || 'Behavior draft read model is escaped display-only data from Rust validation/preview; Studio does not apply drafts, write trusted files, execute scripts, open command bridges, or persist browser draft state.',
      drafts,
      malformedReasons: Array.isArray(source.malformedReasons) ? source.malformedReasons : Array.isArray(source.malformed_reasons) ? source.malformed_reasons : [],
    };
  }

  function behaviorDraftPreviewCommand(draft, run) {
    const project = projectContext(run);
    const projectRoot = project?.projectRoot || project?.manifestPath || '<project-root>';
    const draftPath = draft?.draftPath || '<behavior-draft-json>';
    return `cargo run -p ouroforge-cli -- behavior draft preview ${draftPath} --project-root ${projectRoot}`;
  }

  function renderBehaviorDraftStatusSurface(run) {
    const model = behaviorDraftReadModel(run);
    if (!model.present && model.drafts.length === 0) {
      return `<section id="behavior-draft-status" class="panel"><h2>Behavior draft status</h2><p class="empty">No behavior draft read model is available for this run.</p><p class="hint">${escapeText(model.boundary)}</p></section>`;
    }
    const cards = [
      ['Status', model.status],
      ['Drafts', model.drafts.length],
      ['Malformed rows', model.malformedReasons.length],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const rows = model.drafts.slice(0, 8).map((draft) => {
      const targetCheck = draft.targetCheck && typeof draft.targetCheck === 'object' ? draft.targetCheck : {};
      const stale = targetCheck.stale === true ? 'stale target' : targetCheck.stale === false ? 'fresh target' : 'target not checked';
      const actualHash = targetCheck.actualHash || targetCheck.actual_hash || 'not read';
      const expectedHash = targetCheck.expectedHash || targetCheck.expected_hash || draft.target.sceneHash;
      const diagnostics = draft.diagnostics.map((diagnostic) => diagnostic?.message || diagnostic?.kind || diagnostic).join(' · ') || 'none';
      const blocked = draft.blockedReasons.length
        ? `<p class="warn">Blocked: ${escapeText(draft.blockedReasons.join('; '))}</p>`
        : '<p class="hint">No blocked reasons recorded.</p>';
      return `<article class="surface-row">
        <strong>${escapeText(draft.draftId)}</strong> ${surfaceState(draft.validationStatus !== 'blocked' && targetCheck.stale !== true, draft.validationStatus)}<br>
        <small>project ${escapeText(draft.target.projectId)} · scene ${escapeText(draft.target.scenePath)}<br>target ${escapeText(stale)} · expected ${escapeText(expectedHash)} · actual ${escapeText(actualHash)}</small>
        <div class="field-grid"><div><strong>Behaviors</strong><br>${escapeText(draft.behaviorCount)}</div><div><strong>Evidence refs</strong><br>${escapeText(draft.linkedEvidenceCount)}</div><div><strong>Scenario impacts</strong><br>${escapeText(draft.expectedScenarioImpactCount)}</div></div>
        ${blocked}
        <p class="hint">Diagnostics: ${escapeText(diagnostics)}</p>
        <p class="hint">${escapeText(draft.guardrail)}</p>
        <h4>Copyable CLI preview command</h4><code>${escapeText(behaviorDraftPreviewCommand(draft, run))}</code>
      </article>`;
    }).join('') || '<div class="surface-row">No behavior draft records exported.</div>';
    const malformed = model.malformedReasons.length ? `<p class="warn">Malformed input: ${escapeText(model.malformedReasons.join(' · '))}</p>` : '<p class="hint">No malformed behavior draft rows reported.</p>';
    return `<section id="behavior-draft-status" class="panel"><h2>Behavior draft status</h2>
      <p class="hint">${escapeText(model.boundary)} Copyable command text is inert; the browser cannot run CLI commands, apply drafts, write files, persist trusted draft state, or approve behavior changes.</p>
      <div class="field-grid">${cards}</div>
      ${malformed}
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

  function normalizeProductionEvidenceBundles(value = null) {
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
      const risks = Array.isArray(loop.openRisks) ? loop.openRisks : [];
      const stale = Array.isArray(loop.staleStateIndicators) ? loop.staleStateIndicators : [];
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
        ${risks.length ? `<div class="hint">Open risks: ${escapeText(risks.map((risk) => `${risk.id || 'risk'}:${risk.severity || 'unknown'}:${risk.description || 'missing'}`).join(' · '))}</div>` : '<div class="hint">No open risks reported.</div>'}
        ${stale.length ? `<div class="hint">Stale state: ${escapeText(stale.map((item) => `${item.id || 'stale'}:${item.reason || 'missing'}:${item.nextAction || 'inspect'}`).join(' · '))}</div>` : '<div class="hint">No stale state indicators reported.</div>'}
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


  function pipelineInspectionModel(run = {}) {
    return run?.studio_multi_agent_pipeline_inspection || run?.studioMultiAgentPipelineInspection || run?.multi_agent_pipeline_inspection || run?.multiAgentPipelineInspection || null;
  }

  function renderStudioMultiAgentPipelineInspectionSurface(run) {
    const model = pipelineInspectionModel(run);
    if (!model || typeof model !== 'object') {
      return '<section id="studio-multi-agent-pipeline" class="panel"><h2>Studio multi-agent pipeline inspection</h2><p class="empty">No Studio multi-agent pipeline inspection read model is attached to dashboard-data.json. Studio remains read-only and does not run agents.</p></section>';
    }
    const sections = Array.isArray(model.sections) ? model.sections : [];
    const malformed = Array.isArray(model.malformedReasons) ? model.malformedReasons : [];
    const rows = sections.length ? sections.map((section) => {
      const blockers = Array.isArray(section.blockers) ? section.blockers : [];
      const reasons = Array.isArray(section.malformedReasons) ? section.malformedReasons : [];
      return `<li class="surface-row"><strong>${escapeText(section.label || section.id || 'pipeline section')}</strong> ${surfaceState(Boolean(section.status), section.status || 'unknown')}<br><small>ID: ${escapeText(section.id || 'unknown')} · items ${escapeText(section.itemCount ?? 0)}</small>${blockers.length ? `<div class="hint">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="hint">No blockers reported.</div>'}${reasons.length ? `<div class="hint">Malformed: ${escapeText(reasons.join(' · '))}</div>` : ''}</li>`;
    }).join('') : '<li class="surface-row artifact-warning">Missing or malformed pipeline inspection sections.</li>';
    return `<section id="studio-multi-agent-pipeline" class="panel"><h2>Studio multi-agent pipeline inspection</h2>
      <p class="hint">Read-only Studio multi-agent pipeline surface. Studio displays section status, blockers, and malformed reasons only; it does not execute commands, spawn hidden agents, write trusted state, bridge to local commands, use cloud orchestration, auto-apply, auto-merge, or self-approve.</p>
      <div class="field-grid"><div><strong>Schema</strong><br>${escapeText(model.schemaVersion || 'unknown')}</div><div><strong>Status</strong><br>${escapeText(model.status || 'unknown')}</div><div><strong>Sections</strong><br>${escapeText(sections.length)}</div></div>
      ${malformed.length ? `<div class="hint">Malformed input: ${escapeText(malformed.join(' · '))}</div>` : '<div class="hint">No malformed input reported.</div>'}
      <ul>${rows}</ul>
      <small>${escapeText(model.boundary || 'Pipeline inspection surface is read-only and command-inert.')}</small>
    </section>`;
  }

  function normalizeProductionTaskBoards(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderProductionTaskBoardSurface(run) {
    const boards = normalizeProductionTaskBoards(run?.production_task_boards || run?.productionTaskBoards || run?.production_task_board || run?.productionTaskBoard || null);
    if (!boards.length) {
      return '<section id="production-task-board" class="panel"><h2>Production task board</h2><p class="empty">No production task board is attached to dashboard-data.json. Generated task boards remain local unless fixture-scoped.</p></section>';
    }
    const rows = boards.map((board) => {
      const tasks = Array.isArray(board.tasks) ? board.tasks : [];
      const blockers = tasks.flatMap((task) => Array.isArray(task?.blockedReasons) ? task.blockedReasons.map((reason) => `${task.id || 'task'}: ${reason}`) : []);
      const taskRows = tasks.length ? tasks.map((task) => {
        const evidence = Array.isArray(task.requiredEvidence) ? task.requiredEvidence : [];
        const targets = Array.isArray(task.targetArtifacts) ? task.targetArtifacts : [];
        return `<li><strong>${escapeText(task.id || 'task')}</strong> ${surfaceState(Boolean(task.status), task.status || 'unknown')}<div class="hint">${escapeText(task.role || 'unknown-role')} · owner ${escapeText(task.ownerAgent || 'unknown-owner')}</div><div class="hint">Targets: ${escapeText(targets.map((target) => target.path || target.id || 'target').join(' · ') || 'missing')}</div><div class="hint">Evidence: ${escapeText(evidence.join(' · ') || 'missing')}</div></li>`;
      }).join('') : '<li class="artifact-warning">Missing or malformed tasks list.</li>';
      const forbidden = Array.isArray(board.forbiddenActions) ? board.forbiddenActions : [];
      return `<div class="surface-row"><strong>${escapeText(board.boardId || 'unknown-board')}</strong><br><small>Schema: ${escapeText(board.schemaVersion || 'unknown')} · tasks ${escapeText(tasks.length)}</small>${blockers.length ? `<div class="hint">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="hint">No blockers reported.</div>'}<div class="hint">Forbidden: ${escapeText(forbidden.join(' · ') || 'missing')}</div><ul>${taskRows}</ul><small>${escapeText(board.boundary || 'Task board surface is read-only.')}</small></div>`;
    }).join('');
    return `<section id="production-task-board" class="panel"><h2>Production task board</h2><p class="hint">Read-only production task board surface. Studio does not spawn agents, execute commands, apply changes, write trusted state, auto-merge, or self-approve.</p>${rows}</section>`;
  }

  function normalizeOwnershipPolicies(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderOwnershipPolicySurface(run) {
    const policies = normalizeOwnershipPolicies(run?.ownership_policies || run?.ownershipPolicies || run?.ownership_policy || run?.ownershipPolicy || null);
    if (!policies.length) {
      return '<section id="ownership-policy" class="panel"><h2>Ownership policy</h2><p class="empty">No file/artifact ownership policy is attached to dashboard-data.json. Generated ownership evidence remains local unless fixture-scoped.</p></section>';
    }
    const rows = policies.map((policy) => {
      const entries = Array.isArray(policy.entries) ? policy.entries : [];
      const blockers = entries.flatMap((entry) => Array.isArray(entry?.blockedReasons) ? entry.blockedReasons.map((reason) => `${entry.id || 'entry'}: ${reason}`) : []);
      const escalations = entries.filter((entry) => entry?.escalation).map((entry) => `${entry.id || 'entry'}: ${entry.escalation.requiredDecision || entry.escalation.required_decision || 'decision required'}`);
      const forbidden = Array.isArray(policy.forbiddenActions) ? policy.forbiddenActions : [];
      const entryRows = entries.length ? entries.map((entry) => {
        const target = entry.target || {};
        const evidence = Array.isArray(entry.evidenceRefs) ? entry.evidenceRefs : [];
        const workPackages = Array.isArray(entry.workPackageRefs) ? entry.workPackageRefs : [];
        return `<li><strong>${escapeText(entry.id || 'entry')}</strong> ${surfaceState(Boolean(entry.state), entry.state || 'unknown')}<div class="hint">${escapeText(entry.role || 'unknown-role')} · owner ${escapeText(entry.ownerAgent || 'unknown-owner')} · mode ${escapeText(entry.mode || 'unknown')}</div><div class="hint">Target: ${escapeText(target.kind || 'unknown')}:${escapeText(target.path || target.id || 'missing')}</div><div class="hint">Work packages: ${escapeText(workPackages.join(' · ') || 'missing')} · Evidence: ${escapeText(evidence.map((ref) => ref.path || ref.id || 'ref').join(' · ') || 'missing')}</div></li>`;
      }).join('') : '<li class="artifact-warning">Missing or malformed ownership entries list.</li>';
      return `<div class="surface-row"><strong>${escapeText(policy.policyId || 'unknown-policy')}</strong><br><small>Schema: ${escapeText(policy.schemaVersion || 'unknown')} · entries ${escapeText(entries.length)} · milestone ${escapeText(policy.milestone || 'unknown')}</small>${blockers.length ? `<div class="hint">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="hint">No blockers reported.</div>'}${escalations.length ? `<div class="hint">Escalations: ${escapeText(escalations.join(' · '))}</div>` : '<div class="hint">No escalations reported.</div>'}<div class="hint">Forbidden: ${escapeText(forbidden.join(' · ') || 'missing')}</div><ul>${entryRows}</ul><small>${escapeText(policy.boundary || 'Ownership policy surface is read-only.')}</small></div>`;
    }).join('');
    return `<section id="ownership-policy" class="panel"><h2>Ownership policy</h2><p class="hint">Read-only ownership policy surface. Studio reports blockers, deferred states, and escalation requirements only; it does not lock files, spawn agents, execute commands, apply changes, write trusted state, auto-merge, or self-approve.</p>${rows}</section>`;
  }

  function normalizeAgentWorkPackages(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderAgentWorkPackageSurface(run) {
    const packages = normalizeAgentWorkPackages(run?.agent_work_packages || run?.agentWorkPackages || run?.agent_work_package || run?.agentWorkPackage || null);
    if (!packages.length) {
      return '<section id="agent-work-package" class="panel"><h2>Agent work package</h2><p class="empty">No agent work package is attached to dashboard-data.json. Work package fixtures remain untrusted assignments and do not execute commands.</p></section>';
    }
    const rows = packages.map((pkg) => {
      const blockers = Array.isArray(pkg.blockedReasons) ? pkg.blockedReasons : Array.isArray(pkg.blockers) ? pkg.blockers : [];
      const malformed = Array.isArray(pkg.malformedReasons) ? pkg.malformedReasons : [];
      const allowed = Array.isArray(pkg.allowedArtifacts) ? pkg.allowedArtifacts : [];
      const criteria = Array.isArray(pkg.acceptanceCriteria) ? pkg.acceptanceCriteria : [];
      const expected = Array.isArray(pkg.expectedEvidence) ? pkg.expectedEvidence : [];
      const ownership = Array.isArray(pkg.ownershipRefs) ? pkg.ownershipRefs : [];
      const commands = Array.isArray(pkg.verificationCommands) ? pkg.verificationCommands : [];
      const forbidden = Array.isArray(pkg.forbiddenActions) ? pkg.forbiddenActions : [];
      const criteriaRows = criteria.length ? criteria.map((criterion) => `<li><strong>${escapeText(criterion.id || 'criterion')}</strong>: ${escapeText(criterion.description || 'missing description')}</li>`).join('') : '<li class="artifact-warning">Missing or malformed acceptance criteria.</li>';
      const allowedText = allowed.map((ref) => ref.path || ref.id || 'artifact').join(' · ') || 'missing';
      const expectedText = expected.map((ref) => typeof ref === 'string' ? ref : ref.path || ref.id || 'ref').join(' · ') || 'missing';
      const ownershipText = ownership.map((ref) => typeof ref === 'string' ? ref : ref.path || ref.id || 'ref').join(' · ') || 'missing';
      const commandText = commands.map((command) => command.command || '').filter(Boolean).join(' · ') || 'missing';
      const handoff = pkg.handoffTarget?.path || pkg.handoffTargetPath || pkg.handoffTarget?.id || 'missing';
      return `<div class="surface-row"><strong>${escapeText(pkg.workPackageId || 'unknown-work-package')}</strong> ${surfaceState(Boolean(pkg.status), pkg.status || 'unknown')}<br>
        <small>Schema: ${escapeText(pkg.schemaVersion || 'unknown')} · task ${escapeText(pkg.taskId || 'unknown')} · role ${escapeText(pkg.role || 'unknown-role')}</small>
        ${blockers.length ? `<div class="hint">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="hint">No blockers reported.</div>'}
        ${malformed.length ? `<div class="hint">Malformed: ${escapeText(malformed.join(' · '))}</div>` : ''}
        <div class="hint">Allowed artifacts: ${escapeText(allowedText)}</div>
        <div class="hint">Expected evidence: ${escapeText(expectedText)}</div>
        <div class="hint">Ownership refs: ${escapeText(ownershipText)}</div>
        <div class="hint">Handoff target: ${escapeText(handoff)}</div>
        <div class="hint">Inert verification command text: ${escapeText(commandText)}</div>
        <div class="hint">Forbidden: ${escapeText(forbidden.join(' · ') || 'missing')}</div>
        <h3>Acceptance criteria</h3><ul>${criteriaRows}</ul>
        <small>${escapeText(pkg.boundary || 'Agent work package surface is read-only and command-inert.')}</small></div>`;
    }).join('');
    return `<section id="agent-work-package" class="panel"><h2>Agent work package</h2><p class="hint">Read-only agent work package surface. Studio displays status, blockers, expected evidence, ownership refs, and handoff target only; it does not execute commands, spawn hidden agents, apply changes, write trusted state, auto-merge, or self-approve.</p>${rows}</section>`;
  }

  function normalizeQaSwarmInspectionPanels(value = null) {
    const panels = value?.panels || value?.panelSummaries || value?.panel_summaries;
    if (Array.isArray(panels)) return panels;
    if (Array.isArray(value)) return value;
    return [];
  }

  function renderQaSwarmInspectionSurface(run) {
    const model = run?.qa_swarm_inspection || run?.qaSwarmInspection || null;
    const panels = normalizeQaSwarmInspectionPanels(model);
    if (!model || !panels.length) {
      return '<section id="qa-swarm-inspection" class="panel"><h2>QA swarm inspection</h2><p class="empty">No QA swarm inspection summary is attached to dashboard-data.json. Studio remains read-only and does not spawn workers, run commands, or write trusted state.</p></section>';
    }
    const rows = panels.map((panel) => {
      const refs = Array.isArray(panel.evidenceRefs || panel.evidence_refs) ? (panel.evidenceRefs || panel.evidence_refs) : [];
      return `<div class="surface-row"><strong>${escapeText(panel.title || panel.panelId || panel.panel_id || 'QA panel')}</strong> ${surfaceState(Boolean(panel.status), panel.status || 'unknown')}<br>
        <small>Panel ${escapeText(panel.panelId || panel.panel_id || 'unknown')} · items ${escapeText(panel.itemCount ?? panel.item_count ?? 0)} · malformed ${escapeText(panel.malformedCount ?? panel.malformed_count ?? 0)}</small>
        <div class="hint">Evidence refs: ${escapeText(refs.join(' · ') || 'none')}</div>
        <small>${escapeText(panel.boundary || 'QA swarm panel summary is read-only and command-inert.')}</small></div>`;
    }).join('');
    return `<section id="qa-swarm-inspection" class="panel"><h2>QA swarm inspection</h2><p class="hint">Read-only Studio QA swarm inspection surface. It displays scenario candidates, fuzzing plans, worker budgets/assignments, QA run matrix, invariants, route attempts, visual/performance/error evidence, flaky rerun/backlog, and evidence bundle summaries only; it does not spawn workers, execute commands, bridge to local/cloud runners, write trusted state, auto-fix, auto-apply, auto-merge, self-approve, or claim quality guarantees.</p>
      <div class="surface-row"><strong>Status</strong> ${surfaceState(Boolean(model.status), model.status || 'unknown')}<br>
        <small>Panels ${escapeText(model.panelCount ?? model.panel_count ?? panels.length)} · missing ${escapeText(model.missingPanelCount ?? model.missing_panel_count ?? 0)} · malformed ${escapeText(model.malformedPanelCount ?? model.malformed_panel_count ?? 0)} · items ${escapeText(model.itemCount ?? model.item_count ?? 0)}</small>
        <div class="hint">${escapeText(model.emptyState || model.empty_state || '')}</div>
        <small>${escapeText(model.boundary || 'QA swarm inspection is read-only and command-inert.')}</small></div>${rows}</section>`;
  }

  function normalizeQaAgentWorkQueues(value = null) {
    if (value?.queues && Array.isArray(value.queues)) return value.queues;
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderQaAgentWorkQueueSurface(run) {
    const model = run?.qa_agent_work_queues || run?.qaAgentWorkQueues || null;
    const queues = normalizeQaAgentWorkQueues(model || run?.qa_agent_work_queue || run?.qaAgentWorkQueue || null);
    if (!queues.length) {
      return '<section id="qa-agent-work-queue" class="panel"><h2>QA agent work queue</h2><p class="empty">No QA agent work queue is attached to dashboard-data.json. Studio remains read-only and does not run QA commands.</p></section>';
    }
    const rows = queues.map((queue) => {
      const items = Array.isArray(queue.items) ? queue.items : [];
      const itemRows = items.length ? items.map((item) => {
        const target = item.scenarioTarget || item.scenario_target || {};
        const risk = item.riskArea || item.risk_area || {};
        const command = item.runCommandContext || item.run_command_context || {};
        const expected = Array.isArray(item.expectedEvidence || item.expected_evidence) ? (item.expectedEvidence || item.expected_evidence) : [];
        const runRefs = Array.isArray(item.runEvidenceRefs || item.run_evidence_refs) ? (item.runEvidenceRefs || item.run_evidence_refs) : [];
        const evaluatorRefs = Array.isArray(item.evaluatorEvidenceRefs || item.evaluator_evidence_refs) ? (item.evaluatorEvidenceRefs || item.evaluator_evidence_refs) : [];
        const blocked = Array.isArray(item.blockedReasons || item.blocked_reasons) ? (item.blockedReasons || item.blocked_reasons) : [];
        const stale = Array.isArray(item.staleRunRefs || item.stale_run_refs) ? (item.staleRunRefs || item.stale_run_refs) : [];
        const review = item.reviewGateRef || item.review_gate_ref || {};
        const task = item.taskRef || item.task_ref || {};
        const workPackage = item.workPackageRef || item.work_package_ref || {};
        return `<li><strong>${escapeText(item.queueItemId || item.queue_item_id || 'qa-item')}</strong> ${surfaceState(Boolean(item.status), item.status || 'unknown')}<br>
          <small>Scenario ${escapeText(target.scenarioId || target.scenario_id || 'scenario')} · risk ${escapeText(risk.riskId || risk.risk_id || 'risk')} · role ${escapeText(item.assignedRole || item.assigned_role || 'role')}</small>
          <div class="hint">Task/work package/review: ${escapeText(task.path || task.id || 'missing')} · ${escapeText(workPackage.path || workPackage.id || 'missing')} · ${escapeText(review.path || review.id || 'missing')}</div>
          <div class="hint">Expected/run/evaluator refs: ${escapeText(expected.length)} · ${escapeText(runRefs.length)} · ${escapeText(evaluatorRefs.length)}</div>
          ${blocked.length || stale.length ? `<div class="hint">Blockers/stale: ${escapeText([...blocked, ...stale].join(' · '))}</div>` : '<div class="hint">No blockers or stale refs reported.</div>'}
          <div class="hint">Inert command text: ${escapeText(command.command || 'missing')}</div></li>`;
      }).join('') : '<li class="artifact-warning">Missing or malformed QA queue items.</li>';
      const forbidden = Array.isArray(queue.forbiddenActions) ? queue.forbiddenActions : [];
      return `<div class="surface-row"><strong>${escapeText(queue.queueId || queue.queue_id || 'qa-agent-work-queue')}</strong> ${surfaceState(true, queue.status || model?.status || 'present')}<br>
        <small>Schema: ${escapeText(queue.schemaVersion || queue.schema_version || 'unknown')} · items ${escapeText(items.length)} · milestone ${escapeText(queue.milestone || 'unknown')}</small>
        <div class="hint">Forbidden: ${escapeText(forbidden.join(' · ') || 'missing')}</div>
        <ul>${itemRows}</ul>
        <small>${escapeText(queue.boundary || model?.boundary || 'QA queue surface is read-only and command-inert.')}</small></div>`;
    }).join('');
    return `<section id="qa-agent-work-queue" class="panel"><h2>QA agent work queue</h2><p class="hint">Read-only QA queue Studio surface. It displays linked scenario, evaluator, run, task, work-package, review-gate, stale-ref, and expected-evidence refs only; it does not execute commands, spawn agents, write trusted state, bridge to local commands, auto-apply, auto-merge, or self-approve.</p>${rows}</section>`;
  }

  function normalizePerformanceRegressionLanes(value = null) {
    if (value?.lanes && Array.isArray(value.lanes)) return value.lanes;
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderPerformanceRegressionLaneSurface(run) {
    const model = run?.performance_regression_lanes || run?.performanceRegressionLanes || null;
    const lanes = normalizePerformanceRegressionLanes(model || run?.performance_regression_lane || run?.performanceRegressionLane || null);
    if (!lanes.length) {
      return '<section id="performance-regression-lane" class="panel"><h2>Performance/regression lane</h2><p class="empty">No performance/regression lane is attached to dashboard-data.json. Studio remains read-only and does not promote regressions.</p></section>';
    }
    const rows = lanes.map((lane) => {
      const links = lane.evidenceLinks || lane.evidence_links || {};
      const metrics = Array.isArray(lane.metrics) ? lane.metrics : [];
      const thresholds = Array.isArray(lane.thresholds) ? lane.thresholds : [];
      const blocked = Array.isArray(lane.blockedReasons || lane.blocked_reasons) ? (lane.blockedReasons || lane.blocked_reasons) : [];
      const stale = Array.isArray(lane.staleRunRefs || lane.stale_run_refs) ? (lane.staleRunRefs || lane.stale_run_refs) : [];
      const warnings = Array.isArray(links.browserMetricWarnings || links.browser_metric_warnings) ? (links.browserMetricWarnings || links.browser_metric_warnings) : [];
      const linkRows = [
        ['comparison', links.runComparisonRefs || links.run_comparison_refs],
        ['frame budget', links.frameBudgetRefs || links.frame_budget_refs],
        ['scenario matrix', links.scenarioMatrixRefs || links.scenario_matrix_refs],
        ['QA queue', links.qaQueueRefs || links.qa_queue_refs],
        ['review gate', links.reviewGateRefs || links.review_gate_refs],
      ].map(([label, refs]) => `${label}: ${Array.isArray(refs) ? refs.map((ref) => ref.path || ref.id || 'ref').join(', ') : 'missing'}`).join(' · ');
      return `<div class="surface-row"><strong>${escapeText(lane.laneId || lane.lane_id || 'performance-regression-lane')}</strong> ${surfaceState(Boolean(lane.classification), lane.classification || 'unknown')}<br>
        <small>Schema: ${escapeText(lane.schemaVersion || lane.schema_version || 'unknown')} · role ${escapeText(lane.assignedRole || lane.assigned_role || 'role')} · metrics/thresholds ${escapeText(metrics.length)}/${escapeText(thresholds.length)}</small>
        <div class="hint">Linked refs: ${escapeText(linkRows)}</div>
        <div class="hint">Browser metric warnings: ${escapeText(warnings.join(' · ') || 'none')}</div>
        ${blocked.length || stale.length ? `<div class="hint">Blockers/stale: ${escapeText([...blocked, ...stale].join(' · '))}</div>` : '<div class="hint">No blockers or stale refs reported.</div>'}
        <small>${escapeText(lane.boundary || model?.boundary || 'Performance/regression lane surface is read-only and command-inert.')}</small></div>`;
    }).join('');
    return `<section id="performance-regression-lane" class="panel"><h2>Performance/regression lane</h2><p class="hint">Read-only performance/regression Studio surface. It displays comparison, frame-budget, scenario-matrix, QA queue, review-gate, stale-ref, and browser-warning evidence only; it does not execute commands, spawn agents, write trusted state, bridge to local commands, promote regressions, auto-apply, auto-merge, or self-approve.</p>${rows}</section>`;
  }

  function normalizeAgentRoleModels(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderAgentRoleModelSurface(run) {
    const models = normalizeAgentRoleModels(run?.agent_role_models || run?.agentRoleModels || run?.agent_role_model || run?.agentRoleModel || null);
    if (!models.length) {
      return '<section id="agent-role-model" class="panel"><h2>Agent role model</h2><p class="empty">No agent role model is attached to dashboard-data.json. Role model fixtures remain metadata only and do not spawn agents.</p></section>';
    }
    const rows = models.map((model) => {
      const roles = Array.isArray(model.roles) ? model.roles : [];
      const separation = Array.isArray(model.separationRequirements) ? model.separationRequirements : [];
      const forbidden = Array.isArray(model.forbiddenActions) ? model.forbiddenActions : [];
      const roleRows = roles.length
        ? roles.map((role) => {
          const evidence = Array.isArray(role.requiredEvidence) ? role.requiredEvidence : [];
          const handoffs = Array.isArray(role.handoffTargets) ? role.handoffTargets : [];
          const roleForbidden = Array.isArray(role.forbiddenActions) ? role.forbiddenActions : [];
          return `<li><strong>${escapeText(role.role || 'unknown-role')}</strong><br><small>${escapeText(role.purpose || 'missing purpose')}</small><div class="hint">Evidence: ${escapeText(evidence.join(' · ') || 'missing')} · Handoffs: ${escapeText(handoffs.join(' · ') || 'missing')}</div><div class="hint">Forbidden: ${escapeText(roleForbidden.join(' · ') || 'missing')}</div></li>`;
        }).join('')
        : '<li class="artifact-warning">Missing or malformed roles list.</li>';
      const separationRows = separation.length
        ? separation.map((requirement) => `<li><strong>${escapeText(requirement.id || 'separation-requirement')}</strong>: ${escapeText(requirement.description || 'missing description')}<div class="hint">Blocked: ${escapeText(requirement.blockedCondition || 'missing blocked condition')}</div></li>`).join('')
        : '<li class="artifact-warning">Missing no-self-review/no-self-approval separation requirements.</li>';
      return `<div class="surface-row"><strong>${escapeText(model.milestone || 'unknown milestone')}</strong> ${surfaceState(Boolean(roles.length && separation.length), roles.length && separation.length ? 'ready' : 'needs-attention')}<br><small>Schema: ${escapeText(model.schemaVersion || 'unknown')} · roles ${escapeText(roles.length)}</small><div class="hint">Forbidden actions: ${escapeText(forbidden.join(' · ') || 'missing')}</div><h3>Roles</h3><ul>${roleRows}</ul><h3>Separation requirements</h3><ul>${separationRows}</ul></div>`;
    }).join('');
    return `<section id="agent-role-model" class="panel"><h2>Agent role model</h2><p class="hint">Read-only role accountability surface. Studio does not spawn agents, execute commands, grant authority, self-approve, apply mutations, or merge changes.</p>${rows}</section>`;
  }


  function normalizeReviewCriticGates(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderReviewCriticGateSurface(run) {
    const gates = normalizeReviewCriticGates(run?.review_critic_gates || run?.reviewCriticGates || run?.review_critic_gate || run?.reviewCriticGate || run?.review_gate || run?.reviewGate || null);
    if (!gates.length) {
      return '<section id="review-critic-gate" class="panel"><h2>Review/critic gate</h2><p class="empty">No review/critic gate is attached to dashboard-data.json. Review gates remain inert evidence and do not promote work by themselves.</p></section>';
    }
    const refText = (refs) => refs.map((ref) => typeof ref === 'string' ? ref : ref.path || ref.id || 'ref').join(' · ') || 'missing';
    const rows = gates.map((gate) => {
      const stateSnapshots = Array.isArray(gate.stateSnapshotRefs) ? gate.stateSnapshotRefs : Array.isArray(gate.stateSnapshotRefPaths) ? gate.stateSnapshotRefPaths : [];
      const qaRefs = Array.isArray(gate.qaEvidenceRefs) ? gate.qaEvidenceRefs : Array.isArray(gate.qaEvidenceRefPaths) ? gate.qaEvidenceRefPaths : [];
      const regressionRefs = Array.isArray(gate.regressionEvidenceRefs) ? gate.regressionEvidenceRefs : Array.isArray(gate.regressionEvidenceRefPaths) ? gate.regressionEvidenceRefPaths : [];
      const reviewed = Array.isArray(gate.evidenceReviewed) ? gate.evidenceReviewed : Array.isArray(gate.evidenceReviewedRefPaths) ? gate.evidenceReviewedRefPaths : [];
      const blockers = Array.isArray(gate.blockedReasons) ? gate.blockedReasons : Array.isArray(gate.blockers) ? gate.blockers : [];
      const stale = Array.isArray(gate.staleStateIndicators) ? gate.staleStateIndicators : [];
      const fixes = Array.isArray(gate.requiredFixes) ? gate.requiredFixes : [];
      const malformed = Array.isArray(gate.malformedReasons) ? gate.malformedReasons : [];
      const forbidden = Array.isArray(gate.forbiddenActions) ? gate.forbiddenActions : [];
      const implementer = gate.implementer?.actorId || gate.implementerActorId || 'unknown-implementer';
      const reviewer = gate.reviewer?.actorId || gate.reviewerActorId || 'unknown-reviewer';
      const critic = gate.critic?.actorId || gate.criticActorId || 'unknown-critic';
      const workPackage = gate.workPackageRef?.path || gate.workPackageRefPath || 'missing';
      const handoff = gate.handoffRef?.path || gate.handoffRefPath || 'missing';
      const ledger = gate.decisionLedgerRef?.path || gate.decisionLedgerRefPath || 'missing';
      return `<div class="surface-row"><strong>${escapeText(gate.gateId || 'unknown-review-critic-gate')}</strong> ${surfaceState(Boolean(gate.decision), gate.decision || 'unknown')}<br>
        <small>Schema: ${escapeText(gate.schemaVersion || 'unknown')} · task ${escapeText(gate.taskId || 'unknown')} · recommendation ${escapeText(gate.promotionRecommendation || 'unknown')}</small>
        <div class="hint">Actors: implementer ${escapeText(implementer)} · reviewer ${escapeText(reviewer)} · critic ${escapeText(critic)}</div>
        ${blockers.length ? `<div class="hint">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="hint">No blockers reported.</div>'}
        ${stale.length ? `<div class="hint">Stale state: ${escapeText(stale.join(' · '))}</div>` : '<div class="hint">No stale state indicators reported.</div>'}
        ${fixes.length ? `<div class="hint">Required fixes: ${escapeText(fixes.join(' · '))}</div>` : ''}
        ${malformed.length ? `<div class="hint">Malformed: ${escapeText(malformed.join(' · '))}</div>` : ''}
        <div class="hint">Work package: ${escapeText(workPackage)}</div>
        <div class="hint">Handoff: ${escapeText(handoff)}</div>
        <div class="hint">State snapshots: ${escapeText(refText(stateSnapshots))}</div>
        <div class="hint">QA evidence: ${escapeText(refText(qaRefs))}</div>
        <div class="hint">Regression evidence: ${escapeText(refText(regressionRefs))}</div>
        <div class="hint">Decision ledger: ${escapeText(ledger)}</div>
        <div class="hint">Evidence reviewed: ${escapeText(refText(reviewed))}</div>
        <div class="hint">Forbidden: ${escapeText(forbidden.join(' · ') || 'missing')}</div>
        <small>${escapeText(gate.boundary || 'Review/critic gate surface is read-only and command-inert.')}</small></div>`;
    }).join('');
    return `<section id="review-critic-gate" class="panel"><h2>Review/critic gate</h2><p class="hint">Read-only review/critic gate surface. Studio displays linkage and blocker evidence only; it does not execute commands, spawn agents, apply changes, promote outputs, write trusted state, auto-merge, or self-approve.</p>${rows}</section>`;
  }

  function renderAgentHandoffSurface(run) {
    const handoffs = [
      ...normalizeAgentHandoffs(run?.agent_handoffs || run?.agentHandoffs || run?.agent_handoff || run?.agentHandoff || null),
      ...normalizeAgentHandoffs(run?.agent_handoff_v2s || run?.agentHandoffV2s || null),
    ];
    if (!handoffs.length) {
      return '<section id="agent-handoff" class="panel"><h2>Agent handoff</h2><p class="empty">No agent handoff is attached to dashboard-data.json. Generate one with the Rust CLI and keep it under local generated state.</p></section>';
    }
    const rows = handoffs.map((handoff) => {
      const isV2 = handoff.schemaVersion === 'agent-handoff-v2';
      const blockers = Array.isArray(handoff.blockers) ? handoff.blockers : [];
      const decisions = Array.isArray(handoff.requiredDecisions) ? handoff.requiredDecisions : Array.isArray(handoff.decisions) ? handoff.decisions : [];
      const allowed = Array.isArray(handoff.allowedCommands) ? handoff.allowedCommands : [];
      const forbidden = Array.isArray(handoff.forbiddenActions) ? handoff.forbiddenActions : [];
      const evidence = Array.isArray(handoff.evidenceRefs) ? handoff.evidenceRefs : Array.isArray(handoff.evidenceLinks) ? handoff.evidenceLinks : [];
      const guardrails = Array.isArray(handoff.driftGuardrails) ? handoff.driftGuardrails : [];
      const risks = Array.isArray(handoff.openRisks) ? handoff.openRisks : [];
      const stale = Array.isArray(handoff.staleStateIndicators) ? handoff.staleStateIndicators : [];
      const checklist = Array.isArray(handoff.acceptanceChecklist) ? handoff.acceptanceChecklist : [];
      const commandText = allowed.map((command) => command.command || '').filter(Boolean).join(' · ');
      const title = handoff.loopId || handoff.handoffId || handoff.taskId || 'unknown-handoff';
      const nextAction = handoff.nextSafeAction || handoff.nextRecommendedAction || 'unrecorded';
      return `<div class="surface-row"><strong>${escapeText(title)}</strong> ${surfaceState(Boolean(handoff.status), handoff.status || 'unknown')}<br>
        <small>${isV2 ? `Task: ${escapeText(handoff.taskId || 'unknown')} · ${escapeText(handoff.fromRole || 'unknown')} → ${escapeText(handoff.toRole || 'unknown')}` : `Step: ${escapeText(handoff.currentStep?.stepId || 'none')} · ${escapeText(handoff.currentStep?.kind || 'unknown')}`}</small>
        <div class="hint">Next safe action: ${escapeText(nextAction)}</div>
        ${blockers.length ? `<div class="hint">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="hint">No blockers reported.</div>'}
        ${risks.length ? `<div class="hint">Open risks: ${escapeText(risks.map((risk) => `${risk.id || 'risk'}:${risk.severity || 'unknown'}:${risk.description || 'missing'}`).join(' · '))}</div>` : '<div class="hint">No open risks reported.</div>'}
        ${stale.length ? `<div class="hint">Stale state: ${escapeText(stale.map((item) => `${item.id || 'stale'}:${item.reason || 'missing'}:${item.nextAction || 'inspect'}`).join(' · '))}</div>` : '<div class="hint">No stale state indicators reported.</div>'}
        ${decisions.length ? `<div class="hint">Required decisions: ${escapeText(decisions.map((decision) => `${decision.id || 'decision'}:${decision.kind || 'unknown'}`).join(' · '))}</div>` : '<div class="hint">No required decisions reported.</div>'}
        <div class="hint">Acceptance checklist: ${escapeText(checklist.map((item) => `${item.id || 'item'}:${item.checked ? 'checked' : 'unchecked'}`).join(' · ') || 'none')}</div>
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

  function productionBundleRefCount(bundle) {
    return [
      bundle.taskBoardRef,
      bundle.roleModelRef,
      bundle.ownershipPolicyRef,
      ...(Array.isArray(bundle.workPackageRefs) ? bundle.workPackageRefs : []),
      ...(Array.isArray(bundle.handoffRefs) ? bundle.handoffRefs : []),
      ...(Array.isArray(bundle.stateSnapshotRefs) ? bundle.stateSnapshotRefs : []),
      ...(Array.isArray(bundle.reviewDecisionRefs) ? bundle.reviewDecisionRefs : []),
      ...(Array.isArray(bundle.qaResultRefs) ? bundle.qaResultRefs : []),
      ...(Array.isArray(bundle.performanceRegressionRefs) ? bundle.performanceRegressionRefs : []),
      ...(Array.isArray(bundle.decisionLedgerRefs) ? bundle.decisionLedgerRefs : []),
      ...(Array.isArray(bundle.outcomeRefs) ? bundle.outcomeRefs : []),
    ].filter(Boolean).length;
  }

  function renderProductionEvidenceBundleSurface(run) {
    const bundles = normalizeProductionEvidenceBundles(run?.production_evidence_bundles || run?.productionEvidenceBundles || run?.production_evidence_bundle || run?.productionEvidenceBundle || null);
    if (!bundles.length) {
      return '<section id="production-evidence-bundle" class="panel"><h2>Production evidence bundle</h2><p class="empty">No production evidence bundle is attached to dashboard-data.json.</p><p class="hint">Read-only Studio surface. The browser cannot spawn agents, execute commands, apply changes, auto-merge, self-approve, or write trusted state.</p></section>';
    }
    const rows = bundles.map((bundle) => {
      const laneOutputs = Array.isArray(bundle.laneOutputs) ? bundle.laneOutputs : [];
      const missing = Array.isArray(bundle.missingRefs) ? bundle.missingRefs : [];
      const stale = Array.isArray(bundle.staleRefs) ? bundle.staleRefs : [];
      const blocked = Array.isArray(bundle.blockedReasons) ? bundle.blockedReasons : [];
      const malformed = Array.isArray(bundle.malformedReasons) ? bundle.malformedReasons : [];
      const conflicts = Array.isArray(bundle.unresolvedConflicts) ? bundle.unresolvedConflicts : [];
      const missingReviews = Array.isArray(bundle.missingReviews) ? bundle.missingReviews : [];
      const forbidden = Array.isArray(bundle.forbiddenActions) ? bundle.forbiddenActions : [];
      const generatedRoots = Array.isArray(bundle.generatedState?.roots) ? bundle.generatedState.roots : [];
      const lanes = laneOutputs.map((lane) => `${lane.lane || lane.id || 'lane'}:${lane.status || 'unknown'}`).join(' · ');
      const blockerText = [
        ...blocked,
        ...conflicts.map((conflict) => `${conflict.id || 'conflict'}:${conflict.summary || 'unresolved conflict'}`),
        ...missingReviews.map((review) => `${review.id || 'missing-review'}:${review.requiredReviewerRole || 'reviewer'}`),
      ];
      return `<div class="surface-row"><strong>${escapeText(bundle.bundleId || 'unknown-production-bundle')}</strong> ${surfaceState(Boolean(bundle.status) && bundle.status !== 'malformed', bundle.status || 'unknown')}<br>
        <small>${escapeText(bundle.milestone || 'unrecorded milestone')} · refs:${productionBundleRefCount(bundle)} · lanes:${laneOutputs.length}</small>
        ${lanes ? `<div class="hint">Lanes: ${escapeText(lanes)}</div>` : '<div class="hint">No lane outputs recorded.</div>'}
        ${missing.length ? `<div class="hint">Missing refs: ${escapeText(missing.join(' · '))}</div>` : '<div class="hint">No missing refs reported.</div>'}
        ${stale.length ? `<div class="hint">Stale refs: ${escapeText(stale.join(' · '))}</div>` : '<div class="hint">No stale refs reported.</div>'}
        ${blockerText.length ? `<div class="hint">Blockers: ${escapeText(blockerText.join(' · '))}</div>` : '<div class="hint">No blockers reported.</div>'}
        ${malformed.length ? `<div class="hint">Malformed: ${escapeText(malformed.join(' · '))}</div>` : ''}
        <div class="hint">Generated roots: ${escapeText(generatedRoots.join(' · ') || 'none')}</div>
        <div class="hint">Forbidden actions: ${escapeText(forbidden.join(' · ') || 'none')}</div>
        <small>${escapeText(bundle.boundary || 'Inert local audit artifact; Studio is read-only.')}</small>
      </div>`;
    }).join('');
    return `<section id="production-evidence-bundle" class="panel"><h2>Production evidence bundle</h2>
      <p class="hint">Escaped read-only production bundle evidence. This surface displays state, blockers, missing/stale refs, lane outputs, and generated-state boundaries only; it does not spawn agents, execute commands, apply changes, auto-merge, self-approve, or write trusted state.</p>
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



  function renderEvidenceDiagnosticsSurface(model) {
    const summary = model?.diagnosticSummary || buildEvidenceTimelineDiagnosticSummary(model?.diagnostics || []);
    const diagnostics = Array.isArray(model?.diagnostics) ? model.diagnostics : [];
    if (!diagnostics.length) {
      return '<section id="evidence-diagnostics" class="panel"><h2>Evidence diagnostics</h2><p class="hint">No missing or broken fixture evidence was detected. This diagnostic surface is read-only and cannot rerun tests, mutate evidence, apply source patches, execute commands, publish, deploy, or write trusted files.</p></section>';
    }
    const kindRows = Object.entries(summary.byKind || {}).map(([kind, count]) => `<li>${escapeText(kind)}: ${escapeText(count)}</li>`).join('');
    const diagnosticRows = diagnostics.map((diagnostic) => `<li><strong>${escapeText(diagnostic.runId)}</strong> · ${escapeText(diagnostic.kind)} · ${escapeText(diagnostic.artifactId)}<br><small>${escapeText(diagnostic.message)}</small></li>`).join('');
    const actions = (summary.reviewerActions || []).map((action) => `<li>${escapeText(action)}</li>`).join('');
    return `<section id="evidence-diagnostics" class="panel"><h2>Evidence diagnostics</h2><p class="artifact-warning">${escapeText(summary.total)} missing/broken fixture evidence diagnostic(s) require reviewer attention.</p><div class="field-grid compact"><div><strong>Status</strong><br>${escapeText(summary.status)}</div><div><strong>Affected runs</strong><br>${escapeText((summary.affectedRuns || []).join(' · '))}</div></div><h3>Diagnostic classes</h3><ul>${kindRows}</ul><h3>Details</h3><ul class="warning-list">${diagnosticRows}</ul><h3>Safe reviewer actions</h3><ul>${actions}</ul><p class="hint">Display-only diagnostics. Studio cannot rerun tests, mutate evidence, apply source patches, execute commands, publish, deploy, merge branches, or write trusted files.</p></section>`;
  }

  function renderEvidenceComparisonView(model) {
    const comparisons = Array.isArray(model?.comparisonView) ? model.comparisonView : [];
    if (!comparisons.length) {
      return '<section id="evidence-comparison-view" class="panel"><h2>Before/after evidence comparison</h2><p class="empty">No fixture before/after comparison is available.</p><p class="hint">Display-only Studio surface; no evidence mutation, rerun, source apply, command execution, publish, deploy, merge, or trusted write controls are rendered.</p></section>';
    }
    const rows = comparisons.map((comparison) => {
      const screenshotSummary = ['before', 'after'].map((side) => {
        const artifacts = comparison.screenshots?.[side] || [];
        return `<div><strong>${escapeText(side)} screenshots</strong><ul>${artifacts.length ? artifacts.map((artifact) => `<li>${escapeText(artifact.id)} ${surfaceState(artifact.exists && !artifact.readError, artifact.exists ? (artifact.readError ? 'broken' : 'available') : 'missing')}<br><small>${escapeText(artifact.path || 'no path')}</small></li>`).join('') : '<li>No screenshot evidence.</li>'}</ul></div>`;
      }).join('');
      const worldSummary = ['before', 'after'].map((side) => {
        const artifacts = comparison.worldState?.[side] || [];
        return `<div><strong>${escapeText(side)} world-state</strong><ul>${artifacts.length ? artifacts.map((artifact) => `<li>${escapeText(artifact.id)}<br><small>${escapeText(artifact.valuePreview || artifact.path || 'no summary')}</small></li>`).join('') : '<li>No world-state evidence.</li>'}</ul></div>`;
      }).join('');
      const refs = comparison.comparisonRefs?.length ? comparison.comparisonRefs.join(' · ') : 'no comparison artifact exported yet';
      const classifications = comparison.classifications?.length ? comparison.classifications.join(' · ') : 'unknown';
      const diagnostics = comparison.diagnostics?.length ? `<ul class="warning-list">${comparison.diagnostics.map((diagnostic) => `<li>${escapeText(diagnostic.kind)}: ${escapeText(diagnostic.artifactId)}</li>`).join('')}</ul>` : '<p class="hint compact">No comparison diagnostics for this pair.</p>';
      return `<li class="surface-row"><strong>${escapeText(comparison.beforeRunId)}</strong> → <strong>${escapeText(comparison.afterRunId)}</strong> ${surfaceState(true, classifications)}<br><small>${escapeText(refs)}</small><div class="field-grid compact">${screenshotSummary}${worldSummary}</div><p class="hint compact">World-state changed: ${escapeText(comparison.worldState?.changed ? 'true' : 'false')}</p>${diagnostics}</li>`;
    }).join('');
    return `<section id="evidence-comparison-view" class="panel"><h2>Before/after evidence comparison</h2><p class="hint">Read-only fixture comparison view. The browser cannot mutate evidence, rerun tests, apply source patches, execute commands, publish, deploy, merge branches, or write trusted files.</p><ol>${rows}</ol></section>`;
  }

  function renderEvidenceTimelineSurface(input) {
    const model = input?.schemaVersion === 'studio-evidence-timeline-model-v1'
      ? input
      : buildEvidenceTimelineModel(Array.isArray(input?.runs) ? input.runs : (Array.isArray(input) ? input : [input].filter(Boolean)));
    if (!model.entries.length) {
      return '<section id="evidence-timeline" class="panel"><h2>Evidence timeline</h2><p class="empty">No run evidence is available for the Studio timeline.</p><p class="hint">Read-only timeline surface; the browser does not run tests, mutate evidence, apply patches, execute commands, or write trusted files.</p></section>';
    }
    const rows = model.entries.map((entry) => {
      const evidenceSummary = Object.entries(entry.evidence).map(([label, value]) => `${label}: ${value.count}`).join(' · ');
      const sourceApply = entry.sourceApplyLinks.length
        ? `<div class="hint">Source-apply links: ${entry.sourceApplyLinks.map((link) => escapeText(link.id)).join(' · ')}</div>`
        : '<div class="hint">No source-apply links exported for this run.</div>';
      const diagnostics = entry.diagnostics.length
        ? `<ul class="warning-list">${entry.diagnostics.map((diagnostic) => `<li>${escapeText(diagnostic.kind)}: ${escapeText(diagnostic.artifactId)} — ${escapeText(diagnostic.message)}</li>`).join('')}</ul>`
        : '<div class="hint">No missing or broken evidence diagnostics.</div>';
      return `<li class="surface-row"><strong>${escapeText(entry.runId)}</strong> ${surfaceState(true, entry.verdictStatus)}<br><small>${escapeText(new Date(entry.createdAtUnixMs || 0).toISOString())} · scenario ${escapeText(entry.scenarioStatus)} · ${escapeText(evidenceSummary)}</small>${sourceApply}${diagnostics}</li>`;
    }).join('');
    const comparisons = model.comparisonCandidates.length
      ? `<h3>Before/after comparison candidates</h3><ul>${model.comparisonCandidates.map((candidate) => `<li><strong>${escapeText(candidate.beforeRunId)}</strong> → <strong>${escapeText(candidate.afterRunId)}</strong><br><small>${escapeText(candidate.comparisonRefs.length ? candidate.comparisonRefs.join(' · ') : 'no comparison artifact exported yet')}</small></li>`).join('')}</ul>`
      : '<p class="empty compact">No before/after candidates are available.</p>';
    const diagnostics = model.diagnostics.length
      ? `<p class="artifact-warning">${escapeText(model.diagnostics.length)} timeline diagnostic(s) require reviewer attention.</p>`
      : '<p class="hint">All indexed timeline evidence is readable.</p>';
    return `<section id="evidence-timeline" class="panel"><h2>Evidence timeline</h2><p class="hint">Read-only Studio timeline. The browser cannot mutate evidence, rerun tests, apply source patches, execute commands, publish, deploy, merge branches, or write trusted files.</p>${diagnostics}${renderEvidenceDiagnosticsSurface(model)}<ol class="timeline">${rows}</ol>${comparisons}${renderEvidenceComparisonView(model)}</section>`;
  }

  function renderRouteAttemptEvidenceSurface(run) {
    const model = run?.route_attempts || run?.routeAttempts || {};
    if (!model.present) {
      return `<section id="route-attempt-evidence" class="panel"><h2>Route attempt evidence</h2><p class="empty">${escapeText(model.empty_state || 'No route attempt evidence is available for this run.')}</p><p class="hint">Read-only Studio surface; the browser does not run solvers, spawn workers, execute commands, write trusted state, auto-fix, auto-apply, or auto-merge.</p></section>`;
    }
    const attempts = Array.isArray(model.attempts) ? model.attempts : [];
    const refs = Array.isArray(model.evidence_refs || model.evidenceRefs) ? (model.evidence_refs || model.evidenceRefs) : [];
    const rows = attempts.slice(0, 8).map((attempt) => {
      const start = attempt.startState || attempt.start_state || {};
      const budget = attempt.budgetUsed || attempt.budget_used || {};
      return `<li class="surface-row"><strong>${escapeText(attempt.attemptId || attempt.attempt_id || 'route attempt')}</strong> ${surfaceState(true, attempt.outcome || 'unknown')}<br><small>${escapeText(attempt.objectiveId || attempt.objective_id || 'objective')} · ${escapeText(attempt.scenarioId || attempt.scenario_id || 'scenario')} · start ${escapeText(start.stateId || start.state_id || 'state')} · actions ${escapeText(budget.actionsUsed ?? budget.actions_used ?? '?')}/${escapeText(budget.maxActions ?? budget.max_actions ?? '?')}</small></li>`;
    }).join('') || '<li class="surface-row">No parseable route attempts are available.</li>';
    return `<section id="route-attempt-evidence" class="panel"><h2>Route attempt evidence</h2>
      <p class="hint">${escapeText(model.boundary || 'Read-only route attempt evidence; Studio does not run solvers, spawn workers, execute commands, write trusted state, auto-fix, auto-apply, or auto-merge.')}</p>
      <div class="field-grid"><div><strong>Status</strong><br>${escapeText(model.status || 'unknown')}</div><div><strong>Attempts</strong><br>${escapeText(model.attempt_count ?? model.attemptCount ?? attempts.length)}</div><div><strong>Malformed</strong><br>${escapeText(model.malformed_count ?? model.malformedCount ?? 0)}</div></div>
      <ul>${rows}</ul>
      ${renderRefLinks(refs, run)}
    </section>`;
  }

  function renderVisualComparisonEvidenceSurface(run) {
    const model = run?.visual_comparisons || run?.visualComparisons || {};
    if (!model.present) {
      return `<section id="visual-comparison-evidence" class="panel"><h2>Visual comparison evidence</h2><p class="empty">${escapeText(model.empty_state || 'No visual comparison evidence is available for this run.')}</p><p class="hint">Read-only Studio surface; the browser does not compute trusted diffs, execute commands, write trusted state, auto-fix, auto-apply, auto-merge, or claim aesthetic quality.</p></section>`;
    }
    const summaries = Array.isArray(model.summaries) ? model.summaries : [];
    const refs = Array.isArray(model.evidence_refs || model.evidenceRefs) ? (model.evidence_refs || model.evidenceRefs) : [];
    const rows = summaries.slice(0, 8).map((summary) => {
      const classification = summary.failureClassification || summary.failure_classification || 'visual_unclassified';
      const changedPixels = summary.changedPixels ?? summary.changed_pixels ?? 0;
      const changedPercent = summary.changedPercentX1000 ?? summary.changed_percent_x1000 ?? 0;
      return `<li class="surface-row"><strong>${escapeText(summary.comparisonId || summary.comparison_id || 'visual comparison')}</strong> ${surfaceState(true, summary.outcome || 'unknown')}<br><small>${escapeText(summary.scenarioId || summary.scenario_id || 'scenario')} · ${escapeText(summary.checkpointId || summary.checkpoint_id || 'checkpoint')} · ${escapeText(classification)} · changed ${escapeText(changedPixels)} px (${escapeText(changedPercent)} x1000)</small></li>`;
    }).join('') || '<li class="surface-row">No parseable visual comparisons are available.</li>';
    return `<section id="visual-comparison-evidence" class="panel"><h2>Visual comparison evidence</h2>
      <p class="hint">${escapeText(model.boundary || 'Read-only visual comparison evidence; Studio does not compute trusted diffs, execute commands, write trusted state, auto-fix, auto-apply, auto-merge, or claim aesthetic quality.')}</p>
      <div class="field-grid"><div><strong>Status</strong><br>${escapeText(model.status || 'unknown')}</div><div><strong>Comparisons</strong><br>${escapeText(model.comparison_count ?? model.comparisonCount ?? summaries.length)}</div><div><strong>Changed</strong><br>${escapeText(model.changed_count ?? model.changedCount ?? 0)}</div><div><strong>Malformed</strong><br>${escapeText(model.malformed_count ?? model.malformedCount ?? 0)}</div></div>
      <ul>${rows}</ul>
      ${renderRefLinks(refs, run)}
    </section>`;
  }

  function normalizeStudioLevelDesignInspection(run = {}) {
    const model = run?.level_design_inspection || run?.levelDesignInspection || run?.studio_level_design_inspection || run?.studioLevelDesignInspection || null;
    if (!model || typeof model !== 'object') {
      return {
        present: false,
        schemaVersion: null,
        status: 'missing',
        panels: [],
        malformedReasons: [],
        boundary: 'No Studio level design inspection read model is attached to dashboard-data.json.',
      };
    }
    const panelsAreArray = Array.isArray(model.panels);
    const panels = panelsAreArray ? model.panels : [];
    const malformedReasons = Array.isArray(model.malformedReasons || model.malformed_reasons)
      ? (model.malformedReasons || model.malformed_reasons)
      : [];
    if (!panelsAreArray && typeof model.panels !== 'undefined' && !malformedReasons.length) {
      malformedReasons.push('panels must be an array');
    }
    return {
      present: Boolean(model.present ?? panels.length),
      schemaVersion: model.schemaVersion || model.schema_version || null,
      status: model.status || (malformedReasons.length ? 'malformed' : 'ready'),
      panels,
      malformedReasons,
      boundary: model.boundary || 'Read-only Studio level design inspection surface; no writes or command execution.',
    };
  }

  function renderStudioLevelDesignInspectionSurface(run) {
    const model = normalizeStudioLevelDesignInspection(run);
    if (!model.present) {
      return '<section id="studio-level-design-inspection" class="panel"><h2>Level design inspection</h2><p class="empty">No level design inspection read model is attached to dashboard-data.json.</p><p class="hint">Read-only Studio surface; no browser trusted writes, command bridge, auto-apply, auto-merge, self-approval, production editor, or autonomous full game generation claim.</p></section>';
    }
    const rows = model.panels.length ? model.panels.map((panel) => {
      const items = Array.isArray(panel.items) ? panel.items : [];
      const refs = Array.isArray(panel.refs) ? panel.refs : [];
      const commands = Array.isArray(panel.commands) ? panel.commands : [];
      const itemRows = items.length
        ? `<dl>${items.slice(0, 8).map((item) => `<dt>${escapeText(item.label || item.id || 'item')}</dt><dd>${escapeText(item.value ?? item.summary ?? '')}</dd>`).join('')}</dl>`
        : '<p class="empty">No read-model rows exported for this panel.</p>';
      const refRows = refs.length ? `<div class="hint">Refs: ${escapeText(refs.map((ref) => ref.path || ref.id || ref).join(' · '))}</div>` : '';
      const commandRows = commands.length
        ? `<pre>${escapeText(commands.map((command) => command.command || command).join('\n'))}</pre><p class="hint">Copyable command text only; Studio does not execute commands.</p>`
        : '';
      return `<li class="surface-row"><strong>${escapeText(panel.label || panel.id || 'level design panel')}</strong> ${surfaceState(panel.status !== 'missing', panel.status || 'unknown')}<br><small>${escapeText(panel.kind || 'read-only')}</small>${itemRows}${refRows}${commandRows}</li>`;
    }).join('') : '<li class="surface-row artifact-warning">Malformed level design inspection: panels must be an array.</li>';
    return `<section id="studio-level-design-inspection" class="panel"><h2>Level design inspection</h2>
      <p class="hint">${escapeText(model.boundary)}</p>
      <div class="field-grid"><div><strong>Schema</strong><br>${escapeText(model.schemaVersion || 'unknown')}</div><div><strong>Status</strong><br>${escapeText(model.status)}</div><div><strong>Panels</strong><br>${escapeText(model.panels.length)}</div></div>
      ${model.malformedReasons.length ? `<div class="hint">Malformed input: ${escapeText(model.malformedReasons.join(' · '))}</div>` : '<div class="hint">No malformed input reported.</div>'}
      <ul>${rows}</ul>
      <small>Read-only: no browser trusted writes, no command bridge, no auto-apply, no auto-merge, no self-approval, no production editor, and no autonomous full game generation.</small>
    </section>`;
  }

  function renderEvidencePane(run) {
    return `${renderProjectWorkspaceSurface(run)}${renderProjectRunSurface(run)}${renderEvidenceFidelitySurface(run)}${renderEvidenceTimelineSurface(run)}${renderEvidenceBrowser(run)}${renderAuthoringProvenanceSurface(run)}${renderEngineExpansionSurface(run)}${renderStudio3dInspectionSurface(run)}${renderCameraLayerInspectionSurface(run)}${renderRenderBreakdownInspectionSurface(run)}${renderRuntimeProfilerInspectionSurface(run)}${renderRuntimeStateInspectionSurface(run)}${renderInputActionInspectionSurface(run)}${renderExpressiveComponentHudSurface(run)}${renderRuntimeEventInspectionSurface(run)}${renderRuntimeAssetLoadingSurface(run)}${renderAssetPreviewEvidenceSurface(run)}${renderPluginRegistryBrowserSurface(run)}${renderBehaviorEvidenceLifecycleSurface(run)}${renderSourceApplyWorktreeContextSurface(run)}${renderRouteAttemptEvidenceSurface(run)}${renderVisualComparisonEvidenceSurface(run)}${renderStudioLevelDesignInspectionSurface(run)}${renderBehaviorDraftStatusSurface(run)}${renderBehaviorListPanel(run)}${renderBehaviorEventSignalPanel(run)}${renderBehaviorStateMachinePanel(run)}${renderBehaviorAbilityActionPanel(run)}${renderBehaviorReviewApplyStatusSurface(run)}${renderSourcePatchEvidenceBundleSurface(run)}${renderSourcePatchApplyTransactionSurface(run)}${renderSourcePatchStaleTargetGuardSurface(run)}${renderStudioDraftAuthoringSurface(run)}${renderVisualDiffPreviewSurface(run)}${renderTilemapDraftPreviewSurface(run)}${renderStudioAssetInspectorSurface(run)}${renderJournalSurface(run)}${renderLoopDryRunSurface(run)}${renderLoopExecutionSurface(run)}${renderLoopRecoverySurface(run)}${renderStudioLoopCockpitSurface(run)}${renderStudioMultiAgentPipelineInspectionSurface(run)}${renderProductionTaskBoardSurface(run)}${renderOwnershipPolicySurface(run)}${renderAgentRoleModelSurface(run)}${renderAgentWorkPackageSurface(run)}${renderQaSwarmInspectionSurface(run)}${renderQaAgentWorkQueueSurface(run)}${renderPerformanceRegressionLaneSurface(run)}${renderAgentHandoffSurface(run)}${renderReviewCriticGateSurface(run)}${renderProductionEvidenceBundleSurface(run)}${renderLoopEvidenceBundleSurface(run)}${renderMutationReviewSurface(run)}${renderRegressionPromotionSurface(run)}${renderRegressionMatrixSurface(run)}${renderReplaySurface(run)}${renderComparisonSurface(run)}`;
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
      if (latest && (dashboardData.regression_matrix || dashboardData.regressionMatrix || dashboardData.loop_evidence_bundles || dashboardData.loopEvidenceBundles || dashboardData.production_evidence_bundles || dashboardData.productionEvidenceBundles || dashboardData.production_evidence_bundle || dashboardData.productionEvidenceBundle || dashboardData.agent_handoffs || dashboardData.agentHandoffs || dashboardData.loop_cockpit || dashboardData.loopCockpit)) {
        latest = {
          ...latest,
          regression_matrix: dashboardData.regression_matrix || dashboardData.regressionMatrix,
          loop_evidence_bundles: dashboardData.loop_evidence_bundles || dashboardData.loopEvidenceBundles || [],
          production_evidence_bundles: dashboardData.production_evidence_bundles || dashboardData.productionEvidenceBundles || dashboardData.production_evidence_bundle || dashboardData.productionEvidenceBundle || [],
          agent_handoffs: [
            ...normalizeAgentHandoffs(dashboardData.agent_handoffs || dashboardData.agentHandoffs || []),
            ...normalizeAgentHandoffs(dashboardData.agent_handoff_v2s || dashboardData.agentHandoffV2s || []),
          ],
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

  return { EDITABLE_FIELDS, READ_ONLY_FIELDS, applyEdit, artifactHref, buildEvidenceTimelineModel, callPreviewProbe, cliCommand, compareRunsCommand, dashboardExportCommand, escapeText, getValue, init, latestRun, loadDashboardData, normalizeStudioLevelDesignInspection, previewWindow, projectRunCommand, projectValidateCommand, qaCommand, qaTransactionCommand, readPreviewProbe, reloadPreview, renderAgentHandoffSurface, renderAgentRoleModelSurface, renderAgentWorkPackageSurface, renderQaSwarmInspectionSurface, renderOwnershipPolicySurface, renderProductionTaskBoardSurface, renderProductionEvidenceBundleSurface, renderReviewCriticGateSurface, renderQaAgentWorkQueueSurface, renderPerformanceRegressionLaneSurface, renderAssetPreviewEvidenceSurface, renderBehaviorEvidenceLifecycleSurface, renderPluginRegistryBrowserSurface, renderAuthoringProvenanceSurface, renderCameraLayerInspectionSurface, renderCommandGenerationPanel, renderComparisonSurface, renderEngineExpansionSurface, renderStudio3dInspectionSurface, renderEvidenceBrowser, renderEvidenceFidelitySurface, renderEvidencePane, renderEvidenceTimelineSurface, renderEvidenceDiagnosticsSurface, renderEvidenceComparisonView, fidelityStatusClass, renderExpressiveComponentHudSurface, renderRenderBreakdownInspectionSurface, renderInputActionInspectionSurface, renderRuntimeEventInspectionSurface, renderRuntimeProfilerInspectionSurface, renderRuntimeStateInspectionSurface, renderRuntimeAssetLoadingSurface, renderVisualDiffPreviewSurface, renderVisualComparisonEvidenceSurface, renderStudioLevelDesignInspectionSurface, behaviorDraftReadModel, behaviorDraftPreviewCommand, behaviorInspectionModel, renderBehaviorDraftStatusSurface, renderBehaviorListPanel, renderBehaviorEventSignalPanel, renderBehaviorStateMachinePanel, renderBehaviorAbilityActionPanel, renderBehaviorReviewApplyStatusSurface, renderTilemapDraftControl, renderTilemapDraftPreviewSurface, renderInspector, renderIntegration, renderJournalSurface, renderLoopDryRunSurface, renderLoopExecutionSurface, renderLoopEvidenceBundleSurface, renderLoopRecoverySurface, renderStudioLoopCockpitSurface, renderStudioMultiAgentPipelineInspectionSurface, renderMutationReviewSurface, renderProposalRationaleSurface, renderReviewDecisionSurface, renderRegressionMatrixSurface, renderRegressionPromotionSurface, renderProjectRunSurface, renderProjectWorkspaceSurface, renderPreview, renderPreviewControls, renderQaPanel, renderReadOnlyFields, renderReviewCockpitStageCard, renderStudioReviewCockpitCards, renderRunCommandContext, renderSemanticComparisonSummary, renderSourcePatchEvidenceBundleSurface, renderSourcePatchApplyTransactionSurface, renderSourcePatchStaleTargetGuardSurface, renderSourceApplyWorktreeContextSurface, renderRouteAttemptEvidenceSurface, runtimeReloadPayloadCommand, sceneMutationApplyCommand, renderSceneMutationLifecycleSurface, renderStudioAssetInspectorSurface, renderStudioDraftAuthoringSurface, studioDraftAuthoringState, studioDraftControlModel, studioDraftPreviewCommand, sceneReloadValidateCommand, seedValidateCommand, sceneValidateCommand, transactionCommand, renderReplaySurface, renderStudioGaps, renderStudioNavigation, renderTree, resolvePreviewProbe, studioSurfaceSummary, validateEdit };
})();

if (typeof window !== 'undefined') {
  window.OuroforgeCockpit = OuroforgeCockpit;
  window.addEventListener('DOMContentLoaded', () => OuroforgeCockpit.init());
}

if (typeof module !== 'undefined') {
  module.exports = OuroforgeCockpit;
}
