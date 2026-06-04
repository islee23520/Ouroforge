const OuroforgeDashboard = (() => {
  function statusClass(status) {
    const safeStatus = String(status || 'unknown')
      .toLowerCase()
      .replace(/[^a-z0-9_-]+/g, '-')
      .replace(/^-+|-+$/g, '') || 'unknown';
    return `status status-${safeStatus}`;
  }

  function artifactHref(artifact, run) {
    const runDir = run?.summary?.run_dir || run?.summary?.runDir || '';
    return `../../${runDir}/${artifact.path}`;
  }

  function runRelativeHref(path, run) {
    const runDir = run?.summary?.run_dir || run?.summary?.runDir || '';
    return `../../${runDir}/${path}`;
  }

  function summarizeRun(run) {
    const summary = run.summary || {};
    const project = run.project || summary.project || null;
    return {
      id: summary.id,
      seed: summary.seed_id,
      projectId: project && project.id ? project.id : null,
      projectName: project && project.name ? project.name : null,
      runStatus: summary.run_status || 'unknown',
      verdict: summary.verdict_status || 'unknown',
      scenario: summary.scenario_status || 'unknown',
      workerCount: summary.worker_count ?? 0,
      evidenceCount: summary.evidence_count ?? 0,
      mutationCount: summary.mutation_count ?? 0,
    };
  }

  function escapeText(value) {
    return String(value ?? '')
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#39;');
  }

  function renderRunList(runs, selectedId) {
    if (!runs.length) {
      return '<div class="empty-state">No runs found in dashboard-data.json.</div>';
    }
    return runs.map((run) => {
      const summary = summarizeRun(run);
      const active = summary.id === selectedId ? ' active' : '';
      const projectMeta = summary.projectId ? ` · project ${summary.projectId}` : '';
      return `<button class="run-button${active}" data-run-id="${escapeText(summary.id)}">
        <div class="run-id">${escapeText(summary.id)}</div>
        <div class="run-meta">${escapeText(summary.seed)}${escapeText(projectMeta)} · ${summary.evidenceCount} evidence · ${summary.mutationCount} mutations · ${summary.workerCount} workers</div>
        <div class="run-status-row">
          <span class="${statusClass(summary.runStatus)}">run ${escapeText(summary.runStatus)}</span>
          <span class="${statusClass(summary.verdict)}">verdict ${escapeText(summary.verdict)}</span>
          <span class="${statusClass(summary.scenario)}">scenario ${escapeText(summary.scenario)}</span>
        </div>
      </button>`;
    }).join('');
  }


  function renderArtifactMetadata(artifact) {
    const metadata = artifact && typeof artifact.metadata === 'object' && artifact.metadata ? artifact.metadata : {};
    const keys = [
      'artifact',
      'worker_id',
      'worker_session_id',
      'run_id',
      'execution_boundary',
      'cdp_transport',
      'phase',
      'bounded',
      'limit',
      'optional',
    ].filter((key) => Object.prototype.hasOwnProperty.call(metadata, key));
    if (!keys.length) return '';
    return `<dl class="artifact-metadata">${keys.map((key) => {
      const value = metadata[key];
      return `<dt>${escapeText(key.replace(/_/g, ' '))}</dt><dd>${escapeText(typeof value === 'object' ? JSON.stringify(value) : value)}</dd>`;
    }).join('')}</dl>`;
  }

  function renderArtifacts(title, artifacts, run, renderer = renderArtifactLink) {
    const body = artifacts.length
      ? `<div class="artifact-grid">${artifacts.map((artifact) => renderer(artifact, run)).join('')}</div>`
      : '<p class="empty-state">No artifacts in this category.</p>';
    return `<section class="panel"><h3>${title}</h3>${body}</section>`;
  }

  function renderArtifactLink(artifact, run) {
    const missing = artifact.exists === false ? '<div class="artifact-warning">Missing generated file</div>' : '';
    const readError = artifact.read_error ? `<div class="artifact-warning">${escapeText(artifact.read_error)}</div>` : '';
    return `<article class="artifact">
      <a href="${escapeText(artifactHref(artifact, run))}" target="_blank" rel="noreferrer">${escapeText(artifact.id)}</a>
      <div class="run-meta">${escapeText(artifact.kind)}</div>
      <div class="run-meta">${escapeText(artifact.path)}</div>
      ${renderArtifactMetadata(artifact)}
      ${missing}${readError}
    </article>`;
  }

  function renderScreenshot(artifact, run) {
    const image = artifact.exists === false
      ? '<div class="artifact-warning">Missing generated file</div>'
      : artifact.read_error
        ? `<div class="artifact-warning">${escapeText(artifact.read_error)}</div>`
        : `<img class="screenshot" src="${escapeText(artifactHref(artifact, run))}" alt="${escapeText(artifact.id)}" />`;
    return `<article class="artifact">
      <a href="${escapeText(artifactHref(artifact, run))}" target="_blank" rel="noreferrer">${escapeText(artifact.id)}</a>
      ${image}
    </article>`;
  }

  function renderJsonArtifact(artifact, run) {
    const preview = artifact.value === undefined || artifact.value === null
      ? '<p class="empty-state compact">No JSON preview available.</p>'
      : `<pre>${escapeText(JSON.stringify(artifact.value, null, 2))}</pre>`;
    return `<article class="artifact">
      <a href="${escapeText(artifactHref(artifact, run))}" target="_blank" rel="noreferrer">${escapeText(artifact.id)}</a>
      <div class="run-meta">${escapeText(artifact.path)}</div>
      ${renderArtifactMetadata(artifact)}
      ${artifact.read_error ? `<div class="artifact-warning">${escapeText(artifact.read_error)}</div>` : ''}
      ${preview}
    </article>`;
  }

  function artifacts(...groups) {
    return groups.flatMap((group) => Array.isArray(group) ? group : []);
  }

  function renderCategorySummary(categories = []) {
    if (!categories.length) {
      return '<p class="empty-state">No evidence category summaries are available.</p>';
    }
    const cards = categories.map((category) => {
      const warnings = [];
      if (category.missing_count) warnings.push(`${category.missing_count} missing`);
      if (category.malformed_count) warnings.push(`${category.malformed_count} malformed`);
      return `<article class="category-card">
        <div class="card-label">${escapeText(category.label || category.id)}</div>
        <div class="card-value">${escapeText(category.count ?? 0)}</div>
        ${warnings.length ? `<div class="artifact-warning">${escapeText(warnings.join(' · '))}</div>` : '<div class="run-meta">All indexed files readable</div>'}
      </article>`;
    }).join('');
    return `<div class="category-grid">${cards}</div>`;
  }

  function renderProbeContractStatus(status = {}) {
    const state = status.status || 'legacy';
    const label = `${status.contract_name || 'ouroforge-runtime-probe'} ${status.version || 'unknown'}`;
    const warnings = [];
    if (status.missing_count) warnings.push(`${status.missing_count} missing`);
    if (status.malformed_count) warnings.push(`${status.malformed_count} malformed`);
    const refs = Array.isArray(status.evidence_refs) ? status.evidence_refs : [];
    const refList = refs.length
      ? `<div class="run-meta">Evidence: ${escapeText(refs.slice(0, 4).join(' · '))}${refs.length > 4 ? ' …' : ''}</div>`
      : '<div class="run-meta">No v2 probe evidence refs recorded</div>';
    return `<article class="category-card probe-contract-status">
      <div class="card-label">Runtime probe contract</div>
      <div class="card-value"><span class="${statusClass(state)}">${escapeText(state)}</span></div>
      <div class="run-meta">${escapeText(label)} · observed ${escapeText(status.observed_count ?? 0)}</div>
      ${warnings.length ? `<div class="artifact-warning">${escapeText(warnings.join(' · '))}</div>` : '<div class="run-meta">No probe contract failures recorded</div>'}
      ${refList}
    </article>`;
  }


  function renderTilemapSummary(summary = {}) {
    const tilemaps = summary?.tilemaps || {};
    if (!summary?.present || !tilemaps.present) {
      return '<p class="empty-state">No tilemap world-state summary is available.</p>';
    }
    const authoring = tilemaps.authoring || {};
    const rows = [
      ['Tilemaps', tilemaps.tilemapCount ?? 0],
      ['Layer order entries', tilemaps.layerCount ?? 0],
      ['Collision cells', authoring.collisionCellCount ?? 0],
      ['Trigger cells', authoring.triggerCellCount ?? 0],
      ['Hazard cells', authoring.hazardCellCount ?? 0],
      ['Goal cells', authoring.goalCellCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const tilemapRows = Array.isArray(tilemaps.tilemaps) && tilemaps.tilemaps.length
      ? tilemaps.tilemaps.slice(0, 4).map((tilemap) => {
          const localAuthoring = tilemap?.authoring || {};
          const grid = tilemap?.grid && typeof tilemap.grid === 'object'
            ? `${tilemap.grid.width ?? '?'}×${tilemap.grid.height ?? '?'}`
            : 'unknown grid';
          return `<li><strong>${escapeText(tilemap?.id || 'unknown')}</strong>: ${escapeText(grid)}, ${escapeText(tilemap?.layerCount ?? 0)} layer(s), ${escapeText(localAuthoring.collisionCellCount ?? 0)} collision / ${escapeText(localAuthoring.triggerCellCount ?? 0)} trigger / ${escapeText(localAuthoring.hazardCellCount ?? 0)} hazard / ${escapeText(localAuthoring.goalCellCount ?? 0)} goal cell(s)</li>`;
        }).join('')
      : '<li>No tilemap entries recorded.</li>';
    return `<div class="field-grid">${rows}</div><ul class="run-meta-list">${tilemapRows}</ul><p class="run-meta">Source world-state: ${escapeText(summary.source_world_state || 'unknown')}. Read-only: the dashboard displays exported evidence only and cannot edit tilemaps.</p>`;
  }

  function renderRenderBreakdownSummary(summary = {}) {
    const breakdown = summary?.render_breakdown || summary?.renderBreakdown || {};
    if (!summary?.present || !breakdown.present) return '<p class="empty-state">No scene render breakdown evidence is available.</p>';
    const queue = summary?.render_queue || summary?.renderQueue || {};
    const elements = Array.isArray(breakdown.elements) ? breakdown.elements : [];
    const absence = Array.isArray(breakdown.absenceDiagnostics || breakdown.absence_diagnostics) ? (breakdown.absenceDiagnostics || breakdown.absence_diagnostics) : [];
    const boundary = breakdown.readOnlyInspection || breakdown.read_only_inspection || {};
    const disallowed = Array.isArray(boundary.disallowedActions || boundary.disallowed_actions) ? (boundary.disallowedActions || boundary.disallowed_actions).join(', ') : 'trusted writes, command bridge, live mutation';
    const queueRenderables = Array.isArray(queue.renderables) ? queue.renderables : [];
    const queueValidation = queue.validation || {};
    const rows = [['Frame', breakdown.frameId || breakdown.frame_id || 'unknown'], ['Scene', breakdown.sceneId || breakdown.scene_id || 'unknown'], ['Renderable elements', elements.length], ['Absence diagnostics', absence.length], ['Queue layers', queue.layerCount ?? queue.layer_count ?? 0], ['Queue renderables', queue.renderableCount ?? queue.renderable_count ?? queueRenderables.length], ['Draw calls', queue.drawCallCount ?? queue.draw_call_count ?? 0], ['Queue status', queueValidation.status || 'unreported']].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const elementRows = elements.slice(0, 6).map((element) => `<li><strong>${escapeText(element?.renderableId || element?.entityId || 'renderable')}</strong>: draw ${escapeText(element?.drawOrder ?? '?')} · ${escapeText(element?.layer || 'default')} · ${escapeText(element?.primitiveCategory || 'unknown')}</li>`).join('') || '<li>No renderable elements recorded.</li>';
    const absenceRows = absence.slice(0, 6).map((diag) => `<li><strong>${escapeText(diag?.entityId || diag?.renderableId || 'renderable')}</strong>: ${escapeText(diag?.reason || 'unknown')} · ${escapeText(diag?.detail || '')}</li>`).join('') || '<li>No hidden, skipped, fallback, or malformed diagnostics recorded.</li>';
    const queueRows = queueRenderables.slice(0, 6).map((renderable) => `<li><strong>${escapeText(renderable?.id || 'queue-renderable')}</strong>: draw ${escapeText(renderable?.drawOrder ?? '?')} · ${escapeText(renderable?.layer || 'default')} · ${escapeText(renderable?.primitiveKind || 'unknown')} · ${escapeText(renderable?.visible === false ? (renderable?.fallbackReason || 'skipped') : 'visible')}</li>`).join('') || '<li>No render queue renderables recorded.</li>';
    return `<div class="field-grid">${rows}</div><h4>Renderables</h4><ul class="run-meta-list">${elementRows}</ul><h4>Render queue</h4><ul class="run-meta-list">${queueRows}</ul><h4>Absence diagnostics</h4><ul class="run-meta-list">${absenceRows}</ul><p class="run-meta">Read-only inspection only; disallowed actions: ${escapeText(disallowed)}.</p>`;
  }

  function renderGameplaySummary(summary = {}) {
    const gameplay = summary?.gameplay || {};
    if (!summary?.present || !gameplay.present) {
      return '<p class="empty-state">No trigger/flag world-state summary is available.</p>';
    }
    const trueFlags = Array.isArray(gameplay.trueFlags) && gameplay.trueFlags.length
      ? gameplay.trueFlags.join(', ')
      : 'none';
    const hudValues = Array.isArray(gameplay.hudValues) && gameplay.hudValues.length
      ? gameplay.hudValues.map((hud) => hud?.text || [hud?.label, hud?.value].filter(Boolean).join(': ') || hud?.entityId || 'HUD value').join(', ')
      : 'none';
    const rows = [
      ['Declared flags', gameplay.declaredFlagCount ?? 0],
      ['World flags', `${gameplay.worldFlagCount ?? 0} (${gameplay.trueFlagCount ?? 0} true / ${gameplay.falseFlagCount ?? 0} false)`],
      ['Trigger components', gameplay.triggerEntityCount ?? 0],
      ['Goal flag components', gameplay.goalFlagEntityCount ?? 0],
      ['HUD value components', gameplay.hudValueEntityCount ?? 0],
      ['Trigger collision events', gameplay.triggerCollisionEventCount ?? 0],
      ['True flags', trueFlags],
      ['HUD values', hudValues],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    return `<div class="field-grid">${rows}</div><p class="run-meta">Source world-state: ${escapeText(summary.source_world_state || 'unknown')}</p>`;
  }

  function renderAssetIntegrity(run = {}) {
    const integrity = run.asset_integrity || run.assetIntegrity || {};
    if (!integrity.present) {
      return `<p class="empty-state">${escapeText(integrity.empty_state || 'No asset reference integrity evidence is available for this run.')}</p>`;
    }
    const warnings = Array.isArray(integrity.warnings) ? integrity.warnings : [];
    const refs = Array.isArray(integrity.evidence_refs || integrity.evidenceRefs) ? (integrity.evidence_refs || integrity.evidenceRefs) : [];
    const rows = [
      ['Warnings', integrity.warning_count ?? integrity.warningCount ?? warnings.length],
      ['Stale hashes', integrity.stale_hash_count ?? integrity.staleHashCount ?? 0],
      ['Missing refs/files', integrity.missing_ref_count ?? integrity.missingRefCount ?? 0],
      ['Invalid types', integrity.invalid_type_count ?? integrity.invalidTypeCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const warningRows = warnings.length
      ? warnings.slice(0, 8).map((warning) => {
          const path = warning.path ? ` · ${warning.path}` : '';
          return `<li><strong>${escapeText(warning.kind || 'warning')}</strong>: ${escapeText(warning.assetId || warning.asset_id || 'unknown asset')} — ${escapeText(warning.message || '')}${escapeText(path)}</li>`;
        }).join('')
      : '<li>No missing, stale, invalid-type, or unresolved asset reference warnings recorded.</li>';
    return `<div class="field-grid">${rows}</div>
      <ul class="run-meta-list">${warningRows}</ul>
      ${renderRefLinks('Integrity evidence refs', refs, run)}
      <p class="run-meta">Read-only Rust validation evidence. The dashboard never fetches remote assets, uploads files, writes trusted state, or executes commands.</p>`;
  }

  function renderAssetLoading(run = {}) {
    const loading = run.asset_loading || run.assetLoading || {};
    if (!loading.present) {
      return `<p class="empty-state">${escapeText(loading.empty_state || 'No runtime asset loading evidence is available for this run.')}</p>`;
    }
    const records = Array.isArray(loading.records) ? loading.records : [];
    const refs = Array.isArray(loading.evidence_refs || loading.evidenceRefs) ? (loading.evidence_refs || loading.evidenceRefs) : [];
    const rows = [
      ['Attempts', loading.attempt_count ?? loading.attemptCount ?? records.length],
      ['Loaded', loading.loaded_count ?? loading.loadedCount ?? records.filter((record) => record.status === 'loaded').length],
      ['Failed', loading.failed_count ?? loading.failedCount ?? records.filter((record) => record.status === 'failed').length],
      ['Rejected', loading.rejected_count ?? loading.rejectedCount ?? records.filter((record) => record.status === 'rejected').length],
      ['Fallback', loading.fallback_count ?? loading.fallbackCount ?? records.filter((record) => record.status === 'fallback').length],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const recordRows = records.length
      ? records.slice(0, 10).map((record) => {
          const assetId = record.assetId || record.asset_id || record.id || 'unknown asset';
          const status = record.status || 'unknown';
          const size = record.width && record.height ? ` · ${record.width}×${record.height}` : '';
          const duration = record.loadDurationMs || record.load_duration_ms ? ` · ${record.loadDurationMs ?? record.load_duration_ms}ms` : '';
          const reason = record.failureReason || record.failure_reason ? ` · ${record.failureReason ?? record.failure_reason}` : '';
          return `<li><strong>${escapeText(assetId)}</strong>: <span class="${statusClass(status)}">${escapeText(status)}</span>${escapeText(size)}${escapeText(duration)}${escapeText(reason)}<br><small>${escapeText(record.path || 'no path')} · ${escapeText(record.attemptId || record.attempt_id || 'attempt')}</small></li>`;
        }).join('')
      : '<li>No parsed runtime asset load records are available.</li>';
    return `<div class="field-grid">${rows}</div>
      <ul class="run-meta-list">${recordRows}</ul>
      ${renderRefLinks('Runtime asset loading evidence refs', refs, run)}
      <p class="run-meta">${escapeText(loading.boundary || 'Read-only runtime loading evidence. The dashboard never fetches remote assets, uploads files, writes trusted state, or executes commands.')}</p>`;
  }

  function renderAssetPreview(run = {}) {
    const preview = run.asset_preview || run.assetPreview || {};
    if (!preview.present) {
      return `<p class="empty-state">${escapeText(preview.empty_state || 'No asset preview evidence is available for this run.')}</p>`;
    }
    const records = Array.isArray(preview.records) ? preview.records : [];
    const warnings = Array.isArray(preview.warnings) ? preview.warnings : [];
    const refs = Array.isArray(preview.evidence_refs || preview.evidenceRefs) ? (preview.evidence_refs || preview.evidenceRefs) : [];
    const rows = [
      ['Previews', preview.preview_count ?? preview.previewCount ?? records.length],
      ['Warnings', preview.warning_count ?? preview.warningCount ?? warnings.length],
      ['Images', preview.image_count ?? preview.imageCount ?? 0],
      ['Atlas frames', preview.atlas_frame_count ?? preview.atlasFrameCount ?? 0],
      ['Tilemaps', preview.tilemap_count ?? preview.tilemapCount ?? 0],
      ['Audio/font', `${preview.audio_count ?? preview.audioCount ?? 0}/${preview.font_count ?? preview.fontCount ?? 0}`],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const recordRows = records.length
      ? records.slice(0, 10).map((record) => {
          const assetId = record.assetId || record.asset_id || 'unknown asset';
          const assetType = record.assetType || record.asset_type || 'unknown';
          const image = record.image ? ` · ${record.image.width ?? '?'}×${record.image.height ?? '?'}` : '';
          const atlasFrames = Array.isArray(record.atlasFrames || record.atlas_frames) ? (record.atlasFrames || record.atlas_frames).length : 0;
          const tilemap = record.tilemap ? ` · tilemap ${record.tilemap.width ?? '?'}×${record.tilemap.height ?? '?'}` : '';
          const media = record.audio?.durationMs || record.audio?.duration_ms ? ` · ${record.audio.durationMs ?? record.audio.duration_ms}ms` : '';
          const font = record.font?.family ? ` · ${record.font.family}` : '';
          return `<li><strong>${escapeText(assetId)}</strong>: ${escapeText(assetType)}${escapeText(image)}${atlasFrames ? ` · ${escapeText(atlasFrames)} frame(s)` : ''}${escapeText(tilemap)}${escapeText(media)}${escapeText(font)}<br><small>${escapeText(record.sourcePath || record.source_path || 'no source path')} · ${escapeText(record.previewKind || record.preview_kind || 'preview')}</small></li>`;
        }).join('')
      : '<li>No parsed asset preview records are available.</li>';
    const warningRows = warnings.length
      ? warnings.slice(0, 8).map((warning) => `<li><strong>${escapeText(warning.kind || 'warning')}</strong>: ${escapeText(warning.assetId || warning.asset_id || 'manifest')} — ${escapeText(warning.message || '')}${warning.path ? ` · ${escapeText(warning.path)}` : ''}</li>`).join('')
      : '<li>No asset preview warnings recorded.</li>';
    return `<div class="field-grid">${rows}</div>
      <h4>Preview records</h4><ul class="run-meta-list">${recordRows}</ul>
      <h4>Warnings</h4><ul class="run-meta-list">${warningRows}</ul>
      ${renderRefLinks('Asset preview evidence refs', refs, run)}
      <p class="run-meta">${escapeText(preview.boundary || 'Read-only asset preview evidence. The dashboard never fetches remote assets, uploads files, writes trusted state, or executes commands.')}</p>`;
  }

  function renderSourceApplyWorktreeContext(run = {}) {
    const context = run.source_apply_worktree_context || run.sourceApplyWorktreeContext || {};
    if (!context.present) {
      return `<p class="empty-state">${escapeText(context.empty_state || 'No source apply worktree context evidence is available for this run.')}</p>`;
    }
    const reports = Array.isArray(context.reports) ? context.reports : [];
    const refs = Array.isArray(context.evidence_refs || context.evidenceRefs) ? (context.evidence_refs || context.evidenceRefs) : [];
    const rows = [
      ['Status', context.status || 'unknown'],
      ['Reports', reports.length],
      ['Targets', context.target_count ?? context.targetCount ?? reports.reduce((count, report) => count + (Array.isArray(report.targets) ? report.targets.length : 0), 0)],
      ['Blocked reasons', context.blocked_count ?? context.blockedCount ?? reports.reduce((count, report) => count + (Array.isArray(report.blockedReasons || report.blocked_reasons) ? (report.blockedReasons || report.blocked_reasons).length : 0), 0)],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const reportRows = reports.length
      ? reports.slice(0, 5).map((report) => {
          const blocked = Array.isArray(report.blockedReasons || report.blocked_reasons) ? (report.blockedReasons || report.blocked_reasons) : [];
          const guardrails = Array.isArray(report.guardrails) ? report.guardrails : [];
          const lock = report.lockStatus || report.lock_status || {};
          const targets = Array.isArray(report.targets) ? report.targets : [];
          const targetRows = targets.slice(0, 8).map((target) => {
            const reasons = Array.isArray(target.blockedReasons || target.blocked_reasons) ? (target.blockedReasons || target.blocked_reasons) : [];
            return `<li><strong>${escapeText(target.path || 'unknown target')}</strong>: <span class="${statusClass(reasons.length ? 'blocked' : 'passed')}">${escapeText(reasons.length ? 'blocked' : 'passed')}</span> · ${escapeText(target.gitStatus || target.git_status || 'unknown git')} · ${escapeText(target.rootZone || target.root_zone || 'unknown root')} · ${escapeText(target.fileClassDecision || target.file_class_decision || 'unknown class')}<br><small>${escapeText(reasons.length ? reasons.join(' · ') : 'clean target context')}</small></li>`;
          }).join('') || '<li>No target rows recorded.</li>';
          const blockedRows = blocked.length
            ? blocked.slice(0, 8).map((reason) => `<li>${escapeText(reason)}</li>`).join('')
            : '<li>No blocked reasons recorded.</li>';
          const guardrailRows = guardrails.length
            ? guardrails.slice(0, 6).map((guardrail) => `<li>${escapeText(guardrail)}</li>`).join('')
            : '<li>No guardrails recorded.</li>';
          return `<article class="lifecycle-card">
            <div class="journal-entry-header"><h4>${escapeText(report.policyId || report.policy_id || 'source apply context')}</h4><span class="${statusClass(report.status)}">${escapeText(report.status || 'unknown')}</span></div>
            <dl class="project-mutation-context">
              <dt>Branch/head</dt><dd>${escapeText(report.branch || 'unknown')} @ ${escapeText(report.headCommit || report.head_commit || 'unknown')}</dd>
              <dt>Worktree</dt><dd>${escapeText(report.worktreeRoot || report.worktree_root || 'unknown')}</dd>
              <dt>Lock</dt><dd>${escapeText(lock.active ? `active ${lock.attemptId || lock.attempt_id || ''}` : `inactive ${lock.attemptId || lock.attempt_id || ''}`)}</dd>
            </dl>
            <h5>Targets</h5><ul class="run-meta-list">${targetRows}</ul>
            <h5>Blocked reasons</h5><ul class="run-meta-list">${blockedRows}</ul>
            <h5>Guardrails</h5><ul class="run-meta-list">${guardrailRows}</ul>
          </article>`;
        }).join('')
      : '<p class="empty-state compact">No parseable source apply context reports are available.</p>';
    return `<div class="field-grid">${rows}</div>
      <p class="run-meta">${escapeText(context.boundary || 'Read-only context evidence. The dashboard cannot apply patches, execute commands, write trusted files, merge branches, or bypass review gates.')}</p>
      ${renderRefLinks('Source apply context evidence refs', refs, run)}
      <div class="lifecycle-grid">${reportRows}</div>`;
  }

  function artifactRefHref(ref, run) {
    const text = String(ref ?? '');
    if (!text) return null;
    const evidence = Array.isArray(run?.evidence) ? run.evidence : [];
    const match = evidence.find((artifact) => artifact && (artifact.id === text || artifact.path === text));
    if (match) return artifactHref(match, run);
    if (text.includes('/') || /\.[a-z0-9]+$/i.test(text)) return runRelativeHref(text, run);
    return null;
  }

  function renderRefLinks(title, refs, run, kind = 'artifact') {
    if (!Array.isArray(refs) || !refs.length) return '';
    const links = refs.map((ref) => {
      const href = kind === 'mutation' ? null : artifactRefHref(ref, run);
      return href
        ? `<a class="ref-chip" href="${escapeText(href)}" target="_blank" rel="noreferrer">${escapeText(ref)}</a>`
        : `<span class="ref-chip">${escapeText(ref)}</span>`;
    }).join('');
    return `<div class="ref-group"><div class="card-label">${escapeText(title)}</div><div class="ref-list">${links}</div></div>`;
  }


  function comparisonRefHref(ref, run) {
    const text = String(ref || '');
    if (!text) return '';
    if (/^(https?:|data:|javascript:)/i.test(text)) return '';
    if (text.startsWith('runs/')) return `../../${text}`;
    return runRelativeHref(text, run);
  }

  function renderComparisonRefLinks(title, refs, run) {
    if (!Array.isArray(refs) || !refs.length) return '';
    const links = refs.map((ref) => {
      const href = comparisonRefHref(ref, run);
      return href
        ? `<a class="ref-chip" href="${escapeText(href)}" target="_blank" rel="noreferrer">${escapeText(ref)}</a>`
        : `<span class="ref-chip">${escapeText(ref)}</span>`;
    }).join('');
    return `<div class="ref-group"><div class="card-label">${escapeText(title)}</div><div class="ref-list">${links}</div></div>`;
  }

  function renderDeltaCards(deltas) {
    if (!deltas || typeof deltas !== 'object' || Array.isArray(deltas) || !Object.keys(deltas).length) {
      return '<p class="empty-state compact">No delta fields were recorded in this comparison artifact.</p>';
    }
    return `<div class="delta-grid">${Object.entries(deltas).map(([key, value]) => `<article class="delta-card">
      <div class="card-label">${escapeText(key.replace(/_/g, ' '))}</div>
      <div class="card-value">${escapeText(typeof value === 'object' ? JSON.stringify(value) : value)}</div>
    </article>`).join('')}</div>`;
  }

  function renderTransactionProvenance(run) {
    const provenance = run.transaction_provenance;
    if (!provenance) {
      return '<section class="panel"><h2>Scene Edit Transaction</h2><p class="empty">No scene edit transaction provenance is recorded for this run.</p></section>';
    }
    const refs = [provenance.transactionArtifactPath, provenance.scenePath].filter(Boolean);
    return `<section class="panel"><h2>Scene Edit Transaction</h2>
      <p class="hint">Read-only. Provenance was written by the Rust CLI run binding.</p>
      <dl>
        <dt>Transaction</dt><dd>${escapeText(provenance.transactionId)}</dd>
        <dt>Scene</dt><dd>${escapeText(provenance.scenePath)}</dd>
        <dt>Before hash</dt><dd>${escapeText(provenance.beforeSceneHash && provenance.beforeSceneHash.value)}</dd>
        <dt>After hash</dt><dd>${escapeText(provenance.afterSceneHash && provenance.afterSceneHash.value)}</dd>
      </dl>
      <div class="refs">${refs.map((ref) => String(ref).startsWith('runs/')
        ? `<a href="${escapeText(comparisonRefHref(ref, run))}">${escapeText(ref)}</a>`
        : `<span>${escapeText(ref)}</span>`).join('')}</div>
    </section>`;
  }


  function commandContext(run) {
    const context = run?.command_context || run?.summary?.command_context || null;
    return context && typeof context === 'object' ? context : null;
  }

  function renderCommandContext(run) {
    const context = commandContext(run);
    if (!context) {
      return '<section class="panel"><h3>Reproducible Command Context</h3><p class="empty-state">No run command context is recorded for this legacy run.</p></section>';
    }
    const argv = Array.isArray(context.argv) ? context.argv : [];
    const hints = Array.isArray(context.environmentHints) ? context.environmentHints : [];
    const fields = [
      ['Schema', context.schemaVersion || 'legacy'],
      ['Seed path', context.seedPath || 'unknown'],
      ['Workers', context.workers ?? 'unknown'],
      ['Runs root', context.runsRoot || 'runs'],
      ['Project root', context.projectRoot || 'none'],
      ['Manifest', context.manifestPath || 'none'],
      ['Scenario pack', context.scenarioPackId || 'none'],
      ['Transaction', context.transactionPath || 'none'],
      ['Runtime target', context.runtimeTarget || 'unknown'],
      ['Browser boundary', `${context.browserBoundary || 'unknown'} / ${context.cdpTransport || 'unknown'}`],
    ];
    return `<section class="panel"><h3>Reproducible Command Context</h3>
      <p class="run-meta">Read-only copy evidence from Rust-authored run metadata. The dashboard does not execute, rerun, or bridge this command.</p>
      <pre>${escapeText(context.command || argv.join(' ') || 'No command string recorded.')}</pre>
      <dl>${fields.map(([key, value]) => `<dt>${escapeText(key)}</dt><dd>${escapeText(value)}</dd>`).join('')}</dl>
      ${argv.length ? `<details class="raw-json"><summary>Command argv</summary><pre>${escapeText(JSON.stringify(argv, null, 2))}</pre></details>` : ''}
      ${hints.length ? `<ul>${hints.map((hint) => `<li>${escapeText(hint)}</li>`).join('')}</ul>` : ''}
    </section>`;
  }

  function renderProjectContext(run) {
    const project = run?.project || run?.summary?.project;
    if (!project) {
      return '<section class="panel"><h2>Project Context</h2><p class="empty">No project workspace metadata is recorded for this run.</p></section>';
    }
    const scenes = Array.isArray(project.scenes) ? project.scenes : [];
    const sceneRows = scenes.length
      ? `<ul>${scenes.map((scene) => `<li><code>${escapeText(scene.path || scene.id || 'unknown-scene')}</code> <span class="run-meta">${escapeText(scene.hash?.algorithm || 'hash')} ${escapeText(scene.hash?.value || '')}</span></li>`).join('')}</ul>`
      : '<p class="empty-state compact">No scene hashes were recorded.</p>';
    const pack = project.scenarioPack || project.scenario_pack || null;
    const packLine = pack
      ? `<dt>Scenario pack</dt><dd>${escapeText(pack.id)} (${escapeText(pack.path || 'no path')}) · ${escapeText(Array.isArray(pack.scenarioIds) ? pack.scenarioIds.length : 0)} scenario(s)</dd>`
      : '<dt>Scenario pack</dt><dd>none</dd>';
    return `<section class="panel"><h2>Project Context</h2>
      <p class="hint">Read-only. Project metadata was validated and written by the Rust CLI before run evidence was generated.</p>
      <dl>
        <dt>Project</dt><dd>${escapeText(project.id)} — ${escapeText(project.name)}</dd>
        <dt>Project root</dt><dd>${escapeText(project.projectRoot)}</dd>
        <dt>Manifest</dt><dd>${escapeText(project.manifestPath)}</dd>
        <dt>Manifest hash</dt><dd>${escapeText(project.manifestHash?.algorithm)}:${escapeText(project.manifestHash?.value)}</dd>
        <dt>Seed path</dt><dd>${escapeText(project.seedPath)}</dd>
        ${packLine}
        <dt>Linked transaction</dt><dd>${escapeText(project.transactionId || 'none')}</dd>
      </dl>
      <section class="panel"><h3>Scene hashes</h3>${sceneRows}</section>
    </section>`;
  }

  function renderSemanticDiffSummary(artifact) {
    const semantic = artifact?.semantic || artifact?.value?.semantic;
    if (!semantic || typeof semantic !== 'object') {
      return '<section class="panel"><h5>Semantic evidence diff</h5><p class="empty-state compact">No semantic diff section is available for this comparison artifact.</p></section>';
    }
    const reasons = Array.isArray(semantic.reasons) ? semantic.reasons : [];
    const warnings = Array.isArray(semantic.warnings) ? semantic.warnings : [];
    const reasonList = reasons.length
      ? `<ul>${reasons.map((reason) => `<li><span class="${statusClass(reason.severity || 'changed')}">${escapeText(reason.severity || 'changed')}</span> ${escapeText(reason.kind || 'reason')}: ${escapeText(reason.summary || '')}</li>`).join('')}</ul>`
      : '<p class="empty-state compact">No semantic reasons were recorded.</p>';
    const scenarioCount = Array.isArray(semantic.scenarios) ? semantic.scenarios.length : 0;
    const worldChanged = Array.isArray(semantic.worldState?.changed) ? semantic.worldState.changed.length : 0;
    const eventAdded = Array.isArray(semantic.events?.added) ? semantic.events.added.length : 0;
    const eventRemoved = Array.isArray(semantic.events?.removed) ? semantic.events.removed.length : 0;
    const perfChanged = Array.isArray(semantic.performance?.changed) ? semantic.performance.changed.length : 0;
    const evidenceAdded = Array.isArray(semantic.evidence?.added) ? semantic.evidence.added.length : 0;
    const evidenceRemoved = Array.isArray(semantic.evidence?.removed) ? semantic.evidence.removed.length : 0;
    const transactionChanged = semantic.transactionProvenance?.changed === true ? 'changed' : 'unchanged';
    const project = semantic.project && typeof semantic.project === 'object' ? semantic.project : null;
    const projectChanges = Array.isArray(project?.changes) ? project.changes : [];
    const projectWarnings = Array.isArray(project?.warnings) ? project.warnings : [];
    const projectSummary = project
      ? `<section class="panel"><h5>Project context diff</h5>
          <p class="run-meta">Read-only. Project comparison is loaded from Rust-authored comparison JSON.</p>
          <div class="cards">
            <div class="card"><div class="card-label">Relation</div><div class="card-value">${escapeText(project.relation || 'unknown')}</div></div>
            <div class="card"><div class="card-label">Changed</div><div class="card-value">${escapeText(project.changed === true ? 'true' : 'false')}</div></div>
            <div class="card"><div class="card-label">Project changes</div><div class="card-value">${escapeText(projectChanges.length)}</div></div>
          </div>
          ${projectChanges.length ? `<ul>${projectChanges.map((change) => `<li><span class="status">${escapeText(change.kind || 'project')}</span> ${escapeText(change.summary || '')}: ${escapeText(change.before ?? 'none')} → ${escapeText(change.after ?? 'none')}</li>`).join('')}</ul>` : '<p class="empty-state compact">No project context changes recorded.</p>'}
          ${projectWarnings.length ? `<div class="artifact-warning">Project warnings: ${escapeText(projectWarnings.join(' · '))}</div>` : ''}
        </section>`
      : '<section class="panel"><h5>Project context diff</h5><p class="empty-state compact">No project comparison fields are available for this artifact.</p></section>';
    const warningList = warnings.length
      ? `<div class="artifact-warning">Semantic warnings: ${escapeText(warnings.join(' · '))}</div>`
      : '';
    return `<section class="panel"><h5>Semantic evidence diff</h5>
      <p class="run-meta">Read-only summary from Rust-authored comparison JSON; browser does not compute or infer comparisons.</p>
      <div class="cards">
        <div class="card"><div class="card-label">Schema</div><div class="card-value">${escapeText(semantic.schemaVersion || 'legacy')}</div></div>
        <div class="card"><div class="card-label">Scenario diffs</div><div class="card-value">${escapeText(scenarioCount)}</div></div>
        <div class="card"><div class="card-label">World changes</div><div class="card-value">${escapeText(worldChanged)}</div></div>
        <div class="card"><div class="card-label">Events +/-</div><div class="card-value">${escapeText(`${eventAdded}/${eventRemoved}`)}</div></div>
        <div class="card"><div class="card-label">Performance changes</div><div class="card-value">${escapeText(perfChanged)}</div></div>
        <div class="card"><div class="card-label">Evidence +/-</div><div class="card-value">${escapeText(`${evidenceAdded}/${evidenceRemoved}`)}</div></div>
        <div class="card"><div class="card-label">Project</div><div class="card-value">${escapeText(project?.relation || 'unavailable')}</div></div>
        <div class="card"><div class="card-label">Transaction</div><div class="card-value">${escapeText(transactionChanged)}</div></div>
      </div>
      <h6>Top semantic reasons</h6>
      ${reasonList}
      ${projectSummary}
      ${warningList}
    </section>`;
  }

  function renderRunComparison(run) {
    const comparison = run?.comparison;
    if (!comparison || !comparison.present || !Array.isArray(comparison.artifacts) || !comparison.artifacts.length) {
      return `<section class="panel"><h3>Run Comparison</h3><p class="empty-state">${escapeText(comparison?.empty_state || 'No run comparison artifacts were found for this run.')}</p></section>`;
    }
    const artifactsHtml = comparison.artifacts.map((artifact) => {
      const raw = artifact.value === undefined || artifact.value === null
        ? '<p class="empty-state compact">No raw comparison preview is available.</p>'
        : `<pre>${escapeText(JSON.stringify(artifact.value, null, 2))}</pre>`;
      const unsupported = Array.isArray(artifact.unsupported) && artifact.unsupported.length
        ? `<div class="artifact-warning">Unsupported claims not inferred: ${escapeText(artifact.unsupported.join(' · '))}</div>`
        : '';
      return `<article class="comparison-artifact">
        <div class="journal-entry-header">
          <h4><a href="${escapeText(runRelativeHref(artifact.path, run))}" target="_blank" rel="noreferrer">${escapeText(artifact.id || artifact.path)}</a></h4>
          <span class="${statusClass(artifact.classification || 'unknown')}">${escapeText(artifact.classification || 'unknown')}</span>
        </div>
        <div class="cards">
          <div class="card"><div class="card-label">Before run</div><div class="card-value">${escapeText(artifact.before_run_id || 'unknown')}</div></div>
          <div class="card"><div class="card-label">After run</div><div class="card-value">${escapeText(artifact.after_run_id || 'unknown')}</div></div>
          <div class="card"><div class="card-label">Artifact path</div><div class="card-value">${escapeText(artifact.path)}</div></div>
        </div>
        ${artifact.read_error ? `<div class="artifact-warning">${escapeText(artifact.read_error)}</div>` : ''}
        ${artifact.exists === false ? '<div class="artifact-warning">Missing comparison artifact file</div>' : ''}
        <section class="panel"><h5>Scenario, verdict, performance, assertion, and evidence deltas</h5>${renderDeltaCards(artifact.deltas)}</section>
        ${renderSemanticDiffSummary(artifact)}
        ${renderComparisonRefLinks('Before/after evidence refs', artifact.evidence_refs, run)}
        ${unsupported}
        <details class="raw-json"><summary>Raw comparison artifact</summary>${raw}</details>
      </article>`;
    }).join('');
    return `<section class="panel">
      <h3>Run Comparison</h3>
      <p class="run-meta">Read-only. Displays existing comparison artifacts only; does not compute comparisons, mutate runs, accept/reject mutations, or generate AI summaries.</p>
      <div class="comparison-grid">${artifactsHtml}</div>
    </section>`;
  }

  function renderJournalViewer(run) {
    const journal = run?.journal_view;
    if (!journal) {
      return `<section class="panel"><h3>Journal Viewer</h3><p class="empty-state">No journal read model is available. Export dashboard data with the latest Rust CLI.</p></section>`;
    }
    if (!journal.exists) {
      return `<section class="panel"><h3>Journal Viewer</h3><p class="empty-state">${escapeText(journal.read_error || 'Journal artifact is missing.')}</p></section>`;
    }
    const entries = Array.isArray(journal.entries) ? journal.entries : [];
    const body = entries.length ? entries.map((entry) => `<article class="journal-entry">
      <div class="journal-entry-header">
        <h4>${escapeText(entry.heading)}</h4>
        <span class="status">${escapeText(entry.category || 'summary')}</span>
      </div>
      <pre>${escapeText(entry.body || '')}</pre>
      ${renderRefLinks('Evidence refs', entry.evidence_refs, run)}
      ${renderRefLinks('Verdict refs', entry.verdict_refs, run)}
      ${renderRefLinks('Mutation refs', entry.mutation_refs, run, 'mutation')}
    </article>`).join('') : '<p class="empty-state">Journal exists but has no entries.</p>';
    return `<section class="panel">
      <h3>Journal Viewer</h3>
      <div class="cards">
        <div class="card"><div class="card-label">Journal path</div><div class="card-value">${escapeText(journal.path)}</div></div>
        <div class="card"><div class="card-label">Entries</div><div class="card-value">${escapeText(entries.length)}</div></div>
      </div>
      <section class="panel"><h4>Journal summary</h4><p>${escapeText(journal.summary)}</p></section>
      ${renderRefLinks('All evidence refs', journal.evidence_refs, run)}
      ${renderRefLinks('All verdict refs', journal.verdict_refs, run)}
      ${renderRefLinks('All mutation refs', journal.mutation_refs, run, 'mutation')}
      <div class="journal-entry-list">${body}</div>
    </section>`;
  }

  function renderProposalRationale(proposal) {
    const rationale = proposal?.rationale;
    if (!rationale || typeof rationale !== 'object') {
      return '<p class="empty-state compact">No proposal rationale recorded.</p>';
    }
    const evidenceIds = Array.isArray(rationale.evidence_artifact_ids) && rationale.evidence_artifact_ids.length
      ? rationale.evidence_artifact_ids.map((id) => `<code>${escapeText(id)}</code>`).join('')
      : '<span class="artifact-warning">missing evidence ids</span>';
    const scenarioRefs = Array.isArray(rationale.scenario_result_refs) && rationale.scenario_result_refs.length
      ? `<dt>Scenario refs</dt><dd>${rationale.scenario_result_refs.map((ref) => `<code>${escapeText(ref)}</code>`).join('')}</dd>`
      : '';
    const verdictRefs = Array.isArray(rationale.verdict_refs) && rationale.verdict_refs.length
      ? `<dt>Verdict refs</dt><dd>${rationale.verdict_refs.map((ref) => `<code>${escapeText(ref)}</code>`).join('')}</dd>`
      : '';
    return `<dl class="project-mutation-context">
      <dt>Failure classification</dt><dd>${escapeText(rationale.failure_classification || 'missing')}</dd>
      <dt>Expected effect</dt><dd>${escapeText(rationale.expected_effect || 'missing')}</dd>
      <dt>Evidence artifact ids</dt><dd>${evidenceIds}</dd>
      ${scenarioRefs}
      ${verdictRefs}
      <dt>Allowed mutation</dt><dd>${escapeText(rationale.allowed_mutation_type || 'missing')}</dd>
      <dt>Confidence</dt><dd>${escapeText(rationale.confidence || 'missing')}</dd>
      <dt>Reasoning</dt><dd>${escapeText(rationale.reasoning_summary || 'missing')}</dd>
    </dl>`;
  }

  function renderProposalRationaleList(run) {
    const direct = Array.isArray(run?.mutations) ? run.mutations : [];
    const proposedStage = (run?.mutation_lifecycle?.stages || []).find((stage) => stage.id === 'proposed');
    const staged = Array.isArray(proposedStage?.records) ? proposedStage.records : [];
    const proposals = direct.length ? direct : staged;
    if (!proposals.length) {
      return '<section class="panel"><h4>Proposal rationale</h4><p class="empty-state compact">No mutation proposals recorded.</p></section>';
    }
    const cards = proposals.map((proposal) => `<article class="lifecycle-card">
      <div class="journal-entry-header"><h4>${escapeText(proposal.id || 'unknown proposal')}</h4><span class="${statusClass(proposal.status || 'missing')}">${escapeText(proposal.status || 'missing')}</span></div>
      <div class="run-meta">Evidence ${escapeText(proposal.evidence_id || 'unavailable')} · target ${escapeText(proposal.target || 'unavailable')}</div>
      ${renderProposalRationale(proposal)}
    </article>`).join('');
    return `<section class="panel"><h4>Proposal rationale</h4><p class="run-meta">Read-only proposal quality metadata. The dashboard does not apply, accept, reject, promote, or execute mutations.</p><div class="lifecycle-grid">${cards}</div></section>`;
  }


  function renderReviewDecisionRecords(stage, run) {
    if (!stage || stage.id !== 'reviewed' || !Array.isArray(stage.records) || !stage.records.length) {
      return '';
    }
    const cards = stage.records.map((record) => {
      const status = record.decision_status || record.state || 'unknown';
      const reviewerType = record.reviewer_type || 'unknown';
      const guardrails = record.guardrail_checklist && typeof record.guardrail_checklist === 'object'
        ? Object.entries(record.guardrail_checklist).map(([key, value]) => `${escapeText(key)}=${escapeText(value)}`).join(', ')
        : 'not recorded';
      return `<div class="review-decision-card">
        <strong>${escapeText(record.id || 'review-decision')}</strong> <span class="${statusClass(status)}">${escapeText(status)}</span>
        <dl>
          <dt>Proposal</dt><dd>${escapeText(record.proposal_id || 'unlinked')}</dd>
          <dt>Patch draft</dt><dd>${escapeText(record.patch_draft_id || 'unknown')}</dd>
          <dt>Reviewer</dt><dd>${escapeText(record.reviewer || 'unknown')} (${escapeText(reviewerType)})</dd>
          <dt>Reason</dt><dd>${escapeText(record.reason || '')}</dd>
          <dt>Guardrails</dt><dd>${guardrails}</dd>
        </dl>
        ${renderRefLinks('Decision evidence refs', record.evidence_refs, run)}
      </div>`;
    }).join('');
    return `<section class="review-decision-summary"><h5>Review decision ledger</h5><p class="run-meta">Read-only append-only decision records. Accepted decisions do not apply mutations.</p>${cards}</section>`;
  }

  function renderMutationLifecycle(run) {
    const lifecycle = run?.mutation_lifecycle;
    if (!lifecycle) {
      return `<section class="panel"><h3>Mutation Review</h3><p class="empty-state">No mutation lifecycle read model is available. Export dashboard data with the latest Rust CLI.</p></section>`;
    }
    const stages = Array.isArray(lifecycle.stages) ? lifecycle.stages : [];
    const stageCards = stages.length ? stages.map((stage) => {
      const projectMutationContext = stage.id === 'scene_applied' && Array.isArray(stage.records)
        ? stage.records.map(renderProjectMutationRecord).join('')
        : '';
      const visualDraftContext = stage.id === 'visual_draft_applied' && Array.isArray(stage.records)
        ? stage.records.map(renderVisualDraftApplicationRecord).join('')
        : '';
      return `<article class="lifecycle-card">
      <div class="journal-entry-header">
        <h4>${escapeText(stage.label || stage.id)}</h4>
        <span class="${statusClass(stage.state)}">${escapeText(stage.state || 'missing')}</span>
      </div>
      <div class="run-meta">${escapeText(stage.artifact_path || 'No artifact path')}</div>
      <div class="run-meta">${escapeText(stage.record_count ?? 0)} record(s)</div>
      ${stage.read_error ? `<div class="artifact-warning">${escapeText(stage.read_error)}</div>` : ''}
      ${renderRefLinks('Evidence refs', stage.evidence_refs, run)}
      ${projectMutationContext}
      ${visualDraftContext}
      ${renderReviewDecisionRecords(stage, run)}
      ${Array.isArray(stage.records) && stage.records.length ? `<pre>${escapeText(JSON.stringify(stage.records, null, 2))}</pre>` : '<p class="empty-state compact">No lifecycle records for this stage.</p>'}
    </article>`;
    }).join('') : '<p class="empty-state">No mutation lifecycle stages are available.</p>';
    const hints = Array.isArray(lifecycle.command_hints) && lifecycle.command_hints.length
      ? `<div class="command-list">${lifecycle.command_hints.map((hint) => `<code>${escapeText(hint)}</code>`).join('')}</div>`
      : '<p class="empty-state compact">No manual review command hints are available until patch drafts exist.</p>';
    return `<section class="panel">
      <h3>Mutation Review</h3>
      <p class="run-meta">Inspect-only. Browser UI does not apply patches, write review decisions, run Git, or call GitHub.</p>
      <div class="cards">
        <div class="card"><div class="card-label">Lifecycle state</div><div class="card-value"><span class="${statusClass(lifecycle.terminal_state)}">${escapeText(lifecycle.terminal_state || 'missing')}</span></div></div>
        <div class="card"><div class="card-label">Stages</div><div class="card-value">${escapeText(stages.length)}</div></div>
      </div>
      <section class="panel"><h4>Manual review command hints</h4>${hints}</section>
      ${renderProposalRationaleList(run)}
      <div class="lifecycle-grid">${stageCards}</div>
    </section>`;
  }

  function renderRegressionPromotions(run) {
    const promotions = Array.isArray(run?.regression_promotions) ? run.regression_promotions : [];
    const context = commandContext(run) || {};
    const project = run?.project || run?.summary?.project || {};
    const projectPath = project.manifestPath || context.manifestPath || 'ouroforge.project.json';
    if (!promotions.length) {
      return `<section class="panel"><h3>Regression Promotions</h3><p class="empty-state">No regression promotion records are available for this run.</p><p class="run-meta">Read-only. The dashboard may display promotion records and copyable CLI commands, but it does not promote, write scenario packs, or execute commands.</p></section>`;
    }
    const cards = promotions.map((record) => {
      const target = record.target || {};
      const packId = target.scenarioPackId || target.scenario_pack_id || '<pack-id>';
      const command = `cargo run -p ouroforge-cli -- scenario promote <draft-json> --project ${projectPath} --scenario-pack ${packId} --dry-run`;
      return `<article class="review-decision-card">
        <div class="journal-entry-header"><strong>${escapeText(record.id || 'regression-promotion')}</strong><span class="${statusClass(record.dryRun ? 'dry-run' : 'promoted')}">${escapeText(record.dryRun ? 'dry-run' : 'promoted')}</span></div>
        <dl class="project-mutation-context">
          <dt>Scenario</dt><dd>${escapeText(record.scenarioId || record.scenario_id || 'unknown')}</dd>
          <dt>Target pack</dt><dd>${escapeText(packId)} (${escapeText(target.scenarioPackPath || target.scenario_pack_path || 'unknown path')})</dd>
          <dt>Before hash</dt><dd>${escapeText(record.beforeHash?.value || record.before_hash?.value || 'missing')}</dd>
          <dt>After hash</dt><dd>${escapeText(record.afterHash?.value || record.after_hash?.value || 'missing')}</dd>
          <dt>Record path</dt><dd>${escapeText(record.recordPath || record.record_path || 'dry-run/no record')}</dd>
        </dl>
        <div class="command-list"><code>${escapeText(command)}</code></div>
      </article>`;
    }).join('');
    return `<section class="panel"><h3>Regression Promotions</h3><p class="run-meta">Inspect-only manual promotion records. Browser UI does not dry-run, promote, mutate scenario packs, or execute commands.</p><div class="lifecycle-grid">${cards}</div></section>`;
  }

  function renderRegressionMatrix(matrix) {
    if (!matrix || typeof matrix !== 'object') {
      return '<section class="panel"><h3>Regression Run Matrix</h3><p class="empty-state">No regression matrix export is available yet. Run the dashboard export command on latest CLI output.</p></section>';
    }
    const projects = Array.isArray(matrix.projects) ? matrix.projects : [];
    const skipped = Array.isArray(matrix.skippedRuns) ? matrix.skippedRuns : Array.isArray(matrix.skipped_runs) ? matrix.skipped_runs : [];
    const skippedNote = skipped.length
      ? `<p class="artifact-warning">${escapeText(skipped.length)} legacy or malformed run(s) skipped without inference.</p>`
      : '<p class="run-meta">All matrix inputs had project and scenario-pack context.</p>';
    if (!projects.length) {
      return `<section class="panel"><h3>Regression Run Matrix</h3><p class="empty-state">No project-bound scenario runs are available for the matrix.</p>${skippedNote}</section>`;
    }
    const projectSections = projects.map((project) => {
      const packs = Array.isArray(project.scenarioPacks) ? project.scenarioPacks : Array.isArray(project.scenario_packs) ? project.scenario_packs : [];
      const packSections = packs.map((pack) => {
        const scenarios = Array.isArray(pack.scenarios) ? pack.scenarios : [];
        const rows = scenarios.length ? scenarios.map((scenario) => renderRegressionMatrixScenarioRow(scenario)).join('') : '<tr><td colspan="6" class="empty-state compact">No scenarios recorded for this pack.</td></tr>';
        return `<article class="review-decision-card regression-matrix-pack">
          <h4>${escapeText(pack.scenarioPackId || pack.scenario_pack_id || 'unknown-pack')}</h4>
          <p class="run-meta">${escapeText(pack.scenarioPackPath || pack.scenario_pack_path || 'unknown path')}</p>
          <div class="table-scroll"><table class="regression-matrix-table">
            <thead><tr><th>Scenario</th><th>Current</th><th>Last pass</th><th>Last fail</th><th>Runs</th><th>Context</th></tr></thead>
            <tbody>${rows}</tbody>
          </table></div>
        </article>`;
      }).join('');
      return `<section class="panel regression-matrix-project">
        <h4>${escapeText(project.projectName || project.project_name || project.projectId || project.project_id || 'unknown project')}</h4>
        <p class="run-meta">Project id: ${escapeText(project.projectId || project.project_id || 'unknown')}</p>
        ${packSections || '<p class="empty-state compact">No scenario packs are available for this project.</p>'}
      </section>`;
    }).join('');
    return `<section class="panel" id="regression-run-matrix"><h3>Regression Run Matrix</h3>
      <p class="run-meta">Read-only local evidence projection. This browser surface does not schedule CI, rerun scenarios, promote scenarios, or write files.</p>
      ${skippedNote}
      ${projectSections}
    </section>`;
  }

  function renderRegressionMatrixScenarioRow(scenario) {
    const current = scenario.currentStatus || scenario.current_status || 'unknown';
    const runs = Array.isArray(scenario.runs) ? scenario.runs : [];
    const context = scenario.context || {};
    const lastPass = scenario.lastPass || scenario.last_pass || null;
    const lastFail = scenario.lastFail || scenario.last_fail || null;
    const contextBits = [
      ['mutations', context.mutationIds || context.mutation_ids],
      ['reviews', context.reviewDecisionIds || context.review_decision_ids],
      ['promotions', context.promotionIds || context.promotion_ids],
    ].map(([label, values]) => `${label} ${(Array.isArray(values) ? values.length : 0)}`);
    return `<tr>
      <td>${escapeText(scenario.scenarioId || scenario.scenario_id || 'unknown')}</td>
      <td><span class="${statusClass(current)}">${escapeText(current)}</span></td>
      <td>${renderRegressionMatrixObservation(lastPass)}</td>
      <td>${renderRegressionMatrixObservation(lastFail)}</td>
      <td>${escapeText(runs.length)}</td>
      <td>${escapeText(contextBits.join(' · '))}</td>
    </tr>`;
  }

  function renderRegressionMatrixObservation(observation) {
    if (!observation) return '<span class="empty-state compact">none</span>';
    const runId = observation.runId || observation.run_id || 'unknown-run';
    const runDir = observation.runDir || observation.run_dir || '';
    const path = observation.scenarioResultPath || observation.scenario_result_path || '';
    const label = `${runId} · ${observation.status || 'unknown'}`;
    if (runDir && path) {
      return `<a class="ref-chip" href="${escapeText(`../../${runDir}/${path}`)}" target="_blank" rel="noreferrer">${escapeText(label)}</a>`;
    }
    return `<span class="ref-chip">${escapeText(label)}</span>`;
  }

  function renderProjectMutationRecord(record) {
    if (!record || typeof record !== 'object') return '';
    const project = record.project && typeof record.project === 'object' ? record.project : null;
    const rollback = record.rollback && typeof record.rollback === 'object' ? record.rollback : null;
    const decisionId = typeof record.reviewDecisionId === 'string' && record.reviewDecisionId.trim() ? record.reviewDecisionId : null;
    if (!project && !rollback && !decisionId) return '';
    return `<div class="project-mutation-context">
      <strong>Project-scoped scene mutation</strong>
      <dl>
        <dt>Project</dt><dd>${escapeText(project?.projectId || 'legacy/no project context')}</dd>
        <dt>Manifest</dt><dd>${escapeText(project?.manifestPath || 'unavailable')}</dd>
        <dt>Manifest hash</dt><dd>${escapeText(project?.manifestHash?.algorithm || '')}:${escapeText(project?.manifestHash?.value || 'unavailable')}</dd>
        <dt>Scene</dt><dd>${escapeText(project?.scenePath || record.targetScenePath || 'unavailable')}</dd>
        <dt>Scene hash</dt><dd>${escapeText(project?.sceneHash?.algorithm || '')}:${escapeText(project?.sceneHash?.value || 'unavailable')}</dd>
        <dt>Review decision</dt><dd>${escapeText(decisionId || 'legacy/no review decision linkage recorded')}</dd>
        <dt>Rollback</dt><dd>${escapeText(rollback?.scenePath || 'unavailable')} → ${escapeText(rollback?.restoreHash?.value || 'unavailable')}</dd>
      </dl>
    </div>`;
  }

  function renderVisualDraftApplicationRecord(record) {
    if (!record || typeof record !== 'object') return '';
    const commandContext = record.commandContext && typeof record.commandContext === 'object' ? record.commandContext : null;
    return `<div class="project-mutation-context">
      <strong>Visual draft application</strong>
      <dl>
        <dt>Draft</dt><dd>${escapeText(record.draftId || 'unknown')}</dd>
        <dt>Proposal</dt><dd>${escapeText(record.proposalId || 'unknown')}</dd>
        <dt>Patch draft</dt><dd>${escapeText(record.patchDraftId || 'unknown')}</dd>
        <dt>Review decision</dt><dd>${escapeText(record.reviewDecisionId || 'unknown')}</dd>
        <dt>Transaction</dt><dd>${escapeText(record.transactionId || 'unknown')} (${escapeText(record.transactionArtifactPath || 'no artifact')})</dd>
        <dt>Target scene</dt><dd>${escapeText(record.targetScenePath || 'unknown')}</dd>
        <dt>Scene hash</dt><dd>${escapeText(record.beforeSceneHash?.value || 'before unknown')} → ${escapeText(record.afterSceneHash?.value || 'after unknown')}</dd>
        <dt>Rerun context</dt><dd>${escapeText(commandContext?.command || 'not recorded')}</dd>
      </dl>
      <p class="run-meta">Display-only rerun context; dashboard UI does not execute this command, apply drafts, or write trusted state.</p>
    </div>`;
  }

  function replaySequences(run) {
    return Array.isArray(run?.replay?.sequences) ? run.replay.sequences : [];
  }

  function replayFrames(sequence) {
    const frames = Array.isArray(sequence?.frames) ? sequence.frames : [];
    const checkpointFrames = Array.isArray(sequence?.checkpoints)
      ? sequence.checkpoints
          .map((checkpoint) => checkpoint.frame ?? checkpoint.tick)
          .filter((frame) => Number.isFinite(Number(frame)))
      : [];
    return [...new Set([...frames, ...checkpointFrames].map((frame) => Number(frame)))].sort((a, b) => a - b);
  }

  function createReplayState(run) {
    const sequences = replaySequences(run);
    return { sequenceIndex: sequences.length ? 0 : -1, frameIndex: 0 };
  }

  function currentReplayView(run, state = createReplayState(run)) {
    const sequences = replaySequences(run);
    const sequence = sequences[state.sequenceIndex] || null;
    if (!sequence) return { sequence: null, frames: [], frame: null, checkpoint: null, atEnd: true };
    const frames = replayFrames(sequence);
    const safeFrameIndex = Math.max(0, Math.min(state.frameIndex ?? 0, Math.max(frames.length - 1, 0)));
    const frame = frames.length ? frames[safeFrameIndex] : null;
    const checkpoints = Array.isArray(sequence.checkpoints) ? sequence.checkpoints : [];
    const checkpoint = checkpoints.find((item) => Number(item.frame ?? item.tick) === frame) || null;
    return {
      sequence,
      frames,
      frame,
      checkpoint,
      atEnd: !frames.length || safeFrameIndex >= frames.length - 1,
    };
  }

  function stepReplayForward(run, state = createReplayState(run)) {
    const view = currentReplayView(run, state);
    return {
      sequenceIndex: state.sequenceIndex ?? 0,
      frameIndex: view.atEnd ? (state.frameIndex ?? 0) : (state.frameIndex ?? 0) + 1,
    };
  }

  function resetReplay(run) {
    return createReplayState(run);
  }

  function jumpReplayToCheckpoint(run, state = createReplayState(run), checkpointIndex = 0) {
    const sequence = replaySequences(run)[state.sequenceIndex] || null;
    if (!sequence) return createReplayState(run);
    const frames = replayFrames(sequence);
    const checkpoint = (Array.isArray(sequence.checkpoints) ? sequence.checkpoints : [])[checkpointIndex];
    const targetFrame = Number(checkpoint?.frame ?? checkpoint?.tick ?? frames[0] ?? 0);
    const frameIndex = Math.max(0, frames.indexOf(targetFrame));
    return { sequenceIndex: state.sequenceIndex ?? 0, frameIndex };
  }

  function renderReplayControls(run, state = createReplayState(run)) {
    const replay = run?.replay;
    const sequences = replaySequences(run);
    if (!replay || !replay.present || !sequences.length) {
      return `<section class="panel"><h3>Replay Controls</h3><p class="empty-state">${escapeText(replay?.empty_state || 'No replay evidence is available for this run.')}</p></section>`;
    }
    const view = currentReplayView(run, state);
    const sequence = view.sequence;
    const checkpoints = Array.isArray(sequence.checkpoints) ? sequence.checkpoints : [];
    const checkpointButtons = checkpoints.length
      ? checkpoints.map((checkpoint, index) => `<button class="control-button" data-replay-jump="${escapeText(index)}">Jump to ${escapeText(checkpoint.label || checkpoint.id || `checkpoint ${index + 1}`)} (${escapeText(checkpoint.frame ?? checkpoint.tick ?? 'unknown')})</button>`).join('')
      : '<p class="empty-state compact">No replay checkpoints are available.</p>';
    const worldState = view.checkpoint?.world_state
      ? `<pre>${escapeText(JSON.stringify(view.checkpoint.world_state, null, 2))}</pre>`
      : '<p class="empty-state compact">No world-state snapshot is linked to the current replay frame.</p>';
    return `<section class="panel">
      <h3>Replay Controls</h3>
      <p class="run-meta">Inspect-only. Controls are local/in-memory and do not edit replay inputs, record inputs, or mutate run artifacts.</p>
      <div class="cards">
        <div class="card"><div class="card-label">Sequence</div><div class="card-value">${escapeText(sequence.id)}</div></div>
        <div class="card"><div class="card-label">Source</div><div class="card-value">${escapeText(sequence.source || 'unknown')}</div></div>
        <div class="card"><div class="card-label">Current frame</div><div class="card-value">${escapeText(view.frame ?? 'unknown')}</div></div>
        <div class="card"><div class="card-label">Current tick</div><div class="card-value">${escapeText(view.checkpoint?.tick ?? view.frame ?? 'unknown')}</div></div>
      </div>
      <div class="control-row">
        <button class="control-button" data-replay-step="forward"${view.atEnd ? ' disabled' : ''}>Step forward</button>
        <button class="control-button" data-replay-reset="true">Reset</button>
        ${checkpointButtons}
      </div>
      <section class="panel"><h4>Replay evidence</h4>
        <div class="run-meta">${escapeText(sequence.event_count ?? 0)} event(s) · frames ${escapeText((view.frames || []).join(', ') || 'none')}</div>
        ${renderRefLinks('Evidence refs', sequence.evidence_refs, run)}
        ${sequence.read_error ? `<div class="artifact-warning">${escapeText(sequence.read_error)}</div>` : ''}
      </section>
      <section class="panel"><h4>Current world-state snapshot</h4>
        <div class="run-meta">${escapeText(view.checkpoint?.world_state_path || 'No world-state artifact path')}</div>
        ${worldState}
      </section>
    </section>`;
  }


  function renderLoopDryRunSummary(summary = null) {
    if (!summary || typeof summary !== 'object') {
      return '<section class="panel"><h3>Authoring loop dry-run</h3><p class="empty-state">No dry-run summary is attached to this dashboard data.</p></section>';
    }
    const steps = Array.isArray(summary.steps) ? summary.steps : [];
    const missing = Array.isArray(summary.missingPrerequisites) ? summary.missingPrerequisites : [];
    const stepCards = steps.length
      ? steps.map((step) => {
        const prerequisites = Array.isArray(step.prerequisites) ? step.prerequisites : [];
        const stepMissing = Array.isArray(step.missingPrerequisites) ? step.missingPrerequisites : [];
        const artifacts = Array.isArray(step.expectedArtifacts) ? step.expectedArtifacts : [];
        return `<article class="artifact loop-dry-run-step">
          <h4>${escapeText(step.id || 'step')}</h4>
          <div class="run-meta">${escapeText(step.kind || 'unknown')} · <span class="${statusClass(step.readiness || 'unknown')}">${escapeText(step.readiness || 'unknown')}</span> · plan ${escapeText(step.status || 'unknown')}</div>
          <pre>${escapeText(step.commandText || '')}</pre>
          ${prerequisites.length ? `<div class="run-meta">Prerequisites: ${escapeText(prerequisites.join(' · '))}</div>` : '<div class="run-meta">No prerequisites recorded.</div>'}
          ${artifacts.length ? `<div class="run-meta">Expected: ${escapeText(artifacts.map((artifact) => `${artifact.id || 'artifact'}:${artifact.path || ''}`).join(' · '))}</div>` : ''}
          ${stepMissing.length ? `<div class="artifact-warning">Missing: ${escapeText(stepMissing.join(' · '))}</div>` : ''}
        </article>`;
      }).join('')
      : '<p class="empty-state compact">No dry-run steps recorded.</p>';
    return `<section class="panel loop-dry-run-summary"><h3>Authoring loop dry-run</h3>
      <p class="run-meta">Loop ${escapeText(summary.loopId || 'unknown')} · <span class="${statusClass(summary.status || 'unknown')}">${escapeText(summary.status || 'unknown')}</span></p>
      <p class="run-meta">Read-only inert summary. The browser does not execute command text or write trusted state.</p>
      ${missing.length ? `<div class="artifact-warning">Blocked by: ${escapeText(missing.join(' · '))}</div>` : '<div class="run-meta">No missing prerequisites reported.</div>'}
      <div class="artifact-grid">${stepCards}</div>
      ${summary.boundary ? `<p class="run-meta">${escapeText(summary.boundary)}</p>` : ''}
    </section>`;
  }

  function renderLoopExecutionSummary(summary = null) {
    if (!summary || typeof summary !== 'object') {
      return '<section class="panel"><h3>Authoring loop execution</h3><p class="empty-state">No loop execution summary is attached to this dashboard data.</p></section>';
    }
    const artifacts = Array.isArray(summary.generatedArtifacts) ? summary.generatedArtifacts : [];
    const blocked = Array.isArray(summary.blockedReasons) ? summary.blockedReasons : [];
    const artifactRows = artifacts.length
      ? artifacts.map((artifact) => `<li><code>${escapeText(artifact.id || 'artifact')}</code> ${escapeText(artifact.kind || 'unknown')} · ${escapeText(artifact.path || '')}</li>`).join('')
      : '<li>No generated artifacts recorded.</li>';
    return `<section class="panel loop-execution-summary"><h3>Authoring loop execution</h3>
      <p class="run-meta">Loop ${escapeText(summary.loopId || 'unknown')} · step ${escapeText(summary.stepId || 'unknown')} · <span class="${statusClass(summary.status || 'unknown')}">${escapeText(summary.status || 'unknown')}</span></p>
      <p class="run-meta">Read-only execution evidence. The browser displays Rust CLI output and never executes loop steps.</p>
      <div class="run-meta">Kind: ${escapeText(summary.kind || 'unknown')}</div>
      ${summary.ledgerPath ? `<div class="run-meta">Ledger: ${escapeText(summary.ledgerPath)}</div>` : ''}
      ${blocked.length ? `<div class="artifact-warning">Blocked by: ${escapeText(blocked.join(' · '))}</div>` : '<div class="run-meta">No blocked reasons reported.</div>'}
      <ul>${artifactRows}</ul>
      ${summary.boundary ? `<p class="run-meta">${escapeText(summary.boundary)}</p>` : ''}
    </section>`;
  }

  function normalizeLoopEvidenceBundles(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }


  function normalizeAgentRoleModels(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderAgentRoleModels(value = null) {
    const models = normalizeAgentRoleModels(value);
    if (!models.length) {
      return '<section class="panel agent-role-models"><h3>Agent role model</h3><p class="empty-state">No agent role model is attached to this dashboard data.</p></section>';
    }
    const cards = models.map((model) => {
      const roles = Array.isArray(model.roles) ? model.roles : [];
      const separation = Array.isArray(model.separationRequirements) ? model.separationRequirements : [];
      const forbidden = Array.isArray(model.forbiddenActions) ? model.forbiddenActions : [];
      const guardrails = Array.isArray(model.guardrails) ? model.guardrails : [];
      const roleRows = roles.length
        ? roles.map((role) => {
          const outputs = Array.isArray(role.allowedOutputs) ? role.allowedOutputs : [];
          const evidence = Array.isArray(role.requiredEvidence) ? role.requiredEvidence : [];
          const targets = Array.isArray(role.handoffTargets) ? role.handoffTargets : [];
          const roleForbidden = Array.isArray(role.forbiddenActions) ? role.forbiddenActions : [];
          return `<li><strong>${escapeText(role.role || 'unknown-role')}</strong>: ${escapeText(role.purpose || 'missing purpose')}<br><span class="run-meta">Outputs: ${escapeText(outputs.join(' · ') || 'missing')} · Evidence: ${escapeText(evidence.join(' · ') || 'missing')} · Handoffs: ${escapeText(targets.join(' · ') || 'missing')}</span><br><span class="run-meta">Forbidden: ${escapeText(roleForbidden.join(' · ') || 'missing')}</span></li>`;
        }).join('')
        : '<li class="artifact-warning">Missing or malformed roles list.</li>';
      const separationRows = separation.length
        ? separation.map((requirement) => `<li><strong>${escapeText(requirement.id || 'separation-requirement')}</strong>: ${escapeText(requirement.description || 'missing description')}<br><span class="run-meta">Blocked: ${escapeText(requirement.blockedCondition || 'missing blocked condition')}</span></li>`).join('')
        : '<li class="artifact-warning">Missing role separation requirements.</li>';
      return `<article class="artifact agent-role-model">
        <h4>${escapeText(model.milestone || 'unknown milestone')}</h4>
        <p class="run-meta">Read-only role accountability metadata. The dashboard does not spawn agents, execute commands, grant authority, apply mutations, approve reviews, or merge changes.</p>
        <div class="run-meta">Schema: ${escapeText(model.schemaVersion || 'unknown')} · roles ${escapeText(roles.length)}</div>
        <div class="run-meta">Forbidden actions: ${escapeText(forbidden.join(' · ') || 'missing')}</div>
        <div class="run-meta">Guardrails: ${escapeText(guardrails.join(' · ') || 'missing')}</div>
        <h5>Roles</h5><ul>${roleRows}</ul>
        <h5>Separation requirements</h5><ul>${separationRows}</ul>
      </article>`;
    }).join('');
    return `<section class="panel agent-role-models"><h3>Agent role model</h3><div class="artifact-grid">${cards}</div></section>`;
  }

  function normalizeAgentHandoffs(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderAgentHandoffs(value = null) {
    const handoffs = normalizeAgentHandoffs(value);
    if (!handoffs.length) {
      return '<section class="panel agent-handoffs"><h3>Agent handoff</h3><p class="empty-state">No agent handoff is attached to this dashboard data.</p></section>';
    }
    const cards = handoffs.map((handoff) => {
      const blockers = Array.isArray(handoff.blockers) ? handoff.blockers : [];
      const decisions = Array.isArray(handoff.requiredDecisions) ? handoff.requiredDecisions : [];
      const allowed = Array.isArray(handoff.allowedCommands) ? handoff.allowedCommands : [];
      const forbidden = Array.isArray(handoff.forbiddenActions) ? handoff.forbiddenActions : [];
      const evidence = Array.isArray(handoff.evidenceRefs) ? handoff.evidenceRefs : [];
      const guardrails = Array.isArray(handoff.driftGuardrails) ? handoff.driftGuardrails : [];
      return `<article class="artifact agent-handoff">
        <h4>${escapeText(handoff.loopId || 'unknown-loop')}</h4>
        <div class="run-meta"><span class="${statusClass(handoff.status || 'unknown')}">${escapeText(handoff.status || 'unknown')}</span> · step ${escapeText(handoff.currentStep?.stepId || 'none')}</div>
        <div class="run-meta">Next safe action: ${escapeText(handoff.nextSafeAction || 'unrecorded')}</div>
        ${blockers.length ? `<div class="artifact-warning">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="run-meta">No blockers reported.</div>'}
        ${decisions.length ? `<div class="run-meta">Required decisions: ${escapeText(decisions.map((decision) => `${decision.id || 'decision'}:${decision.kind || 'unknown'}`).join(' · '))}</div>` : '<div class="run-meta">No required decisions reported.</div>'}
        <div class="run-meta">Allowed command text: ${escapeText(allowed.map((command) => command.command || '').filter(Boolean).join(' · ') || 'none')}</div>
        <div class="run-meta">Forbidden actions: ${escapeText(forbidden.join(' · ') || 'none')}</div>
        <div class="run-meta">Evidence refs: ${escapeText(evidence.map((ref) => `${ref.id || 'ref'}:${ref.path || 'missing'}`).join(' · ') || 'none')}</div>
        <div class="run-meta">Guardrails: ${escapeText(guardrails.join(' · ') || 'none')}</div>
        <p class="run-meta">${escapeText(handoff.boundary || 'Advisory evidence only; browser is read-only.')}</p>
      </article>`;
    }).join('');
    return `<section class="panel agent-handoffs"><h3>Agent handoff</h3>
      <p class="run-meta">Read-only handoff evidence. The dashboard displays command text but does not execute commands, grant authority, apply mutations, or merge changes.</p>
      <div class="artifact-grid">${cards}</div>
    </section>`;
  }

  function renderLoopEvidenceBundles(value = null) {
    const bundles = normalizeLoopEvidenceBundles(value);
    if (!bundles.length) {
      return '<section class="panel"><h3>Authoring loop evidence bundle</h3><p class="empty-state">No loop evidence bundle is attached to this dashboard data.</p></section>';
    }
    const cards = bundles.map((bundle) => {
      const steps = Array.isArray(bundle.steps) ? bundle.steps : [];
      const missing = Array.isArray(bundle.missingRefs) ? bundle.missingRefs : [];
      const artifactGroups = [
        ['Runs', bundle.runs],
        ['Comparisons', bundle.comparisons],
        ['Proposals', bundle.proposals],
        ['Review decisions', bundle.reviewDecisions],
        ['Transactions', bundle.transactions],
        ['Promotions', bundle.regressionPromotions],
        ['Matrices', bundle.matrixSnapshots],
        ['Journals', bundle.journalSummaries],
      ].map(([label, artifacts]) => `${label}: ${Array.isArray(artifacts) ? artifacts.length : 0}`).join(' · ');
      const stepRows = steps.length ? steps.map((step) => `<li>${escapeText(step.stepId || 'step')} · ${escapeText(step.kind || 'unknown')} · ${escapeText(step.status || 'unknown')}</li>`).join('') : '<li>No step outputs recorded.</li>';
      return `<article class="artifact loop-evidence-bundle">
        <h4>${escapeText(bundle.loopId || 'unknown-loop')}</h4>
        <div class="run-meta"><span class="${statusClass(bundle.status || 'unknown')}">${escapeText(bundle.status || 'unknown')}</span> · ${escapeText(artifactGroups)}</div>
        <div class="run-meta">Plan: ${escapeText(bundle.plan?.path || 'unrecorded')}</div>
        ${missing.length ? `<div class="artifact-warning">Missing/stale refs: ${escapeText(missing.join(' · '))}</div>` : '<div class="run-meta">No missing refs reported.</div>'}
        <ul>${stepRows}</ul>
        <p class="run-meta">${escapeText(bundle.boundary || 'Generated local index only; browser is read-only.')}</p>
      </article>`;
    }).join('');
    return `<section class="panel loop-evidence-bundles"><h3>Authoring loop evidence bundle</h3>
      <p class="run-meta">Read-only generated index. The dashboard does not package artifacts, write bundle data, or execute commands.</p>
      <div class="artifact-grid">${cards}</div>
    </section>`;
  }

  function renderLoopRecoveryStatus(summary = null) {
    if (!summary || typeof summary !== 'object') {
      return '<section class="panel"><h3>Authoring loop recovery</h3><p class="empty-state">No recovery status is attached to this dashboard data.</p></section>';
    }
    const steps = Array.isArray(summary.steps) ? summary.steps : [];
    const stepRows = steps.length ? steps.map((step) => {
      const missing = Array.isArray(step.missingPrerequisites) ? step.missingPrerequisites : [];
      const recovery = step.recovery || {};
      const manual = recovery.manualAction || {};
      return `<article class="artifact loop-recovery-step">
        <h4>${escapeText(step.id || 'step')}</h4>
        <div class="run-meta">${escapeText(step.kind || 'unknown')} · <span class="${statusClass(step.status || 'unknown')}">${escapeText(step.status || 'unknown')}</span></div>
        ${recovery.failure ? `<div class="run-meta">Failure: ${escapeText(recovery.failure.reason || 'unspecified')}</div>` : '<div class="run-meta">No recovery failure metadata recorded.</div>'}
        ${manual.description ? `<div class="run-meta">Manual action: ${escapeText(manual.description)}</div>` : ''}
        ${missing.length ? `<div class="artifact-warning">Missing: ${escapeText(missing.join(' · '))}</div>` : ''}
        <div class="run-meta">Next safe action: ${escapeText(step.nextSafeAction || 'Inspect manually.')}</div>
      </article>`;
    }).join('') : '<p class="empty-state compact">No recovery steps recorded.</p>';
    return `<section class="panel loop-recovery-status"><h3>Authoring loop recovery</h3>
      <p class="run-meta">Loop ${escapeText(summary.loopId || 'unknown')} · <span class="${statusClass(summary.status || 'unknown')}">${escapeText(summary.status || 'unknown')}</span></p>
      <p class="run-meta">Read-only recovery status. The browser does not resume, retry, repair, apply, or promote loop steps.</p>
      <div class="run-meta">Next safe action: ${escapeText(summary.nextSafeAction || 'Inspect manually.')}</div>
      <div class="artifact-grid">${stepRows}</div>
      ${summary.boundary ? `<p class="run-meta">${escapeText(summary.boundary)}</p>` : ''}
    </section>`;
  }

  function sourcePatchEvidenceBundles(run) {
    return artifacts(run?.mutation_artifacts).filter((artifact) => artifact.id === 'source-patch-evidence-bundle' || artifact.path === 'mutation/source-patch-evidence-bundle.json');
  }

  function renderSourcePatchEvidenceBundles(run) {
    const bundles = sourcePatchEvidenceBundles(run);
    if (!bundles.length) {
      return '<section class="panel source-patch-evidence-bundles"><h3>Source patch evidence bundle</h3><p class="empty-state">No source patch evidence bundle is exported for this run.</p><p class="run-meta">Read-only dashboard surface. The browser cannot apply patches, merge branches, execute commands, or write trusted files.</p></section>';
    }
    const cards = bundles.map((artifact) => {
      const value = artifact.value || {};
      const notices = Array.isArray(value.forbiddenActionNotices) ? value.forbiddenActionNotices : [];
      const guardrails = Array.isArray(value.guardrails) ? value.guardrails : [];
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
      return `<article class="artifact source-patch-evidence-bundle">
        <h4>${escapeText(value.bundleId || artifact.id || 'source patch bundle')}</h4>
        <div class="run-meta"><span class="${statusClass(value.status || 'unknown')}">${escapeText(value.status || 'unknown')}</span> · preview ${escapeText(value.patchPreviewId || 'unknown')}</div>
        <div class="run-meta">Patch summary: ${escapeText(patchSummary.title || 'not recorded')} · targets ${escapeText(patchSummary.targetCount ?? 'unknown')} · changed lines ${escapeText(patchSummary.changedLines ?? 'unknown')}</div>
        <div class="run-meta">Expected behavior: ${escapeText(patchSummary.expectedBehaviorChange || 'not recorded')}</div>
        <div class="run-meta">File classes: ${escapeText(fileClassText)}</div>
        <div class="run-meta">Risk: ${escapeText(riskIds.join(' · ') || 'none')}</div>
        <div class="run-meta">Dry-run: ${escapeText(dryRunSummary.status || 'unknown')} · policy ${escapeText(dryRunSummary.allowlistPolicyId || 'unknown')} · report ${escapeText(dryRunRef)}</div>
        <div class="run-meta">Required tests: ${escapeText(requiredCommands.join(' · ') || 'none')} · policy ${escapeText(requiredTestSummary.allowlistPolicyId || 'unknown')}</div>
        <div class="run-meta">Review: ${escapeText(reviewSummary.status || 'unknown')} · decision ${escapeText(reviewRef)}</div>
        <div class="run-meta">Linked evidence: ${escapeText(linkedEvidenceText || 'none')}</div>
        ${blockedReasons.length ? `<div class="artifact-warning">Blocked: ${escapeText(blockedReasons.join(' · '))}</div>` : ''}
        <div class="run-meta">Refs: ${escapeText(refs.join(' · ') || artifact.path || 'none')}</div>
        <div class="run-meta">Forbidden actions: ${escapeText(notices.map((notice) => notice.action || 'unknown').join(' · ') || 'none')}</div>
        <p class="run-meta">${escapeText(guardrails.join(' · ') || 'Read-only bundle evidence; no browser apply/merge/execute/write authority.')}</p>
      </article>`;
    }).join('');
    return `<section class="panel source-patch-evidence-bundles"><h3>Source patch evidence bundle</h3>
      <p class="run-meta">Read-only source patch bundle evidence. This dashboard renders links and forbidden-action notices only; it does not apply patches, merge branches, execute commands, or write trusted files.</p>
      <div class="artifact-grid">${cards}</div>
    </section>`;
  }

  function sourcePatchApplyTransactions(run) {
    return artifacts(run?.mutation_artifacts).filter((artifact) => artifact.id === 'source-patch-apply-transaction' || artifact.path === 'mutation/source-patch-apply-transaction.json');
  }

  function sourcePatchStaleTargetGuards(run) {
    return artifacts(run?.mutation_artifacts).filter((artifact) => artifact.id === 'source-patch-stale-target-guard' || artifact.path === 'mutation/source-patch-stale-target-guard.json');
  }

  function renderSourcePatchStaleTargetGuards(run) {
    const guards = sourcePatchStaleTargetGuards(run);
    if (!guards.length) {
      return '<section class="panel source-patch-stale-target-guards"><h3>Source patch stale target guard</h3><p class="empty-state">No source patch stale target guard is exported for this run.</p><p class="run-meta">Read-only dashboard surface. The browser cannot apply patches, merge branches, execute commands, write trusted files, or bypass review gates.</p></section>';
    }
    const cards = guards.map((artifact) => {
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
      return `<article class="artifact source-patch-stale-target-guard">
        <h4>${escapeText(value.guardId || value.guard_id || artifact.id || 'source patch stale target guard')}</h4>
        <div class="run-meta"><span class="${statusClass(validation.status || value.status || 'unknown')}">${escapeText(validation.status || value.status || 'unknown')}</span> · ${escapeText(validation.readinessLabel || validation.readiness_label || 'stale-target readiness metadata only')}</div>
        <div class="run-meta">Targets: ${escapeText(targets.map((target) => `${target.path || 'unknown'}:${target.fileClass || target.file_class || 'unknown'}:${target.fileStatus || target.file_status || 'unknown'}`).join(' · ') || 'none')}</div>
        <div class="run-meta">Refs: ${escapeText(refs.join(' · ') || artifact.path || 'none')}</div>
        ${blockers.length ? `<div class="artifact-warning">Blocked: ${escapeText(blockers.join(' · '))}</div>` : ''}
        <div class="run-meta">Forbidden actions: ${escapeText(forbidden.join(' · '))}</div>
      </article>`;
    }).join('');
    return `<section class="panel source-patch-stale-target-guards"><h3>Source patch stale target guard</h3>
      <p class="run-meta">Read-only stale-target readiness evidence. This dashboard renders target freshness, blockers, and refs only; it does not apply patches, merge branches, execute commands, write trusted files, or bypass review gates.</p>
      <div class="artifact-grid">${cards}</div>
    </section>`;
  }

  function renderSourcePatchApplyTransactions(run) {
    const transactions = sourcePatchApplyTransactions(run);
    if (!transactions.length) {
      return '<section class="panel source-patch-apply-transactions"><h3>Source patch apply transaction</h3><p class="empty-state">No source patch apply transaction is exported for this run.</p><p class="run-meta">Read-only dashboard surface. The browser cannot apply patches, merge branches, execute commands, write trusted files, or bypass review gates.</p></section>';
    }
    const cards = transactions.map((artifact) => {
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
      return `<article class="artifact source-patch-apply-transaction">
        <h4>${escapeText(value.transactionId || value.transaction_id || artifact.id || 'source patch transaction')}</h4>
        <div class="run-meta"><span class="${statusClass(validation.status || value.status || 'unknown')}">${escapeText(validation.status || value.status || 'unknown')}</span> · ${escapeText(validation.readinessLabel || validation.readiness_label || 'readiness metadata only')}</div>
        <div class="run-meta">Targets: ${escapeText(targets.map((target) => `${target.path || 'unknown'}:${target.fileClass || target.file_class || 'unknown'}`).join(' · ') || 'none')}</div>
        <div class="run-meta">Refs: ${escapeText(refs.join(' · ') || artifact.path || 'none')}</div>
        ${blockers.length ? `<div class="artifact-warning">Blocked: ${escapeText(blockers.join(' · '))}</div>` : ''}
        <div class="run-meta">Forbidden actions: ${escapeText(forbidden.join(' · '))}</div>
      </article>`;
    }).join('');
    return `<section class="panel source-patch-apply-transactions"><h3>Source patch apply transaction</h3>
      <p class="run-meta">Read-only transaction readiness evidence. This dashboard renders readiness, blockers, targets, and refs only; it does not apply patches, merge branches, execute commands, write trusted files, or bypass review gates.</p>
      <div class="artifact-grid">${cards}</div>
    </section>`;
  }

  function renderRunDetail(run) {
    return renderRunDetailWithState(run, createReplayState(run), run?.regression_matrix || run?.regressionMatrix || null, run?.loop_evidence_bundles || run?.loopEvidenceBundles || run?.loop_evidence_bundle || run?.loopEvidenceBundle || null, run?.agent_handoffs || run?.agentHandoffs || run?.agent_handoff || run?.agentHandoff || null);
  }

  function renderRunDetailWithState(run, replayState, regressionMatrix = null, loopEvidenceBundles = null, agentHandoffs = null) {
    if (!run) return '<div class="empty-state">Select a run to inspect its evidence.</div>';
    const verdict = run.verdict || {};
    const summary = summarizeRun(run);
    const evidence = Array.isArray(run.evidence) ? run.evidence : [];
    const mutations = Array.isArray(run.mutations) ? run.mutations : [];
    return `<article>
      <h2>${escapeText(summary.id)}</h2>
      <div class="cards">
        <div class="card"><div class="card-label">Seed</div><div class="card-value">${escapeText(summary.seed)}</div></div>
        <div class="card"><div class="card-label">Run</div><div class="card-value"><span class="${statusClass(summary.runStatus)}">${escapeText(summary.runStatus)}</span></div></div>
        <div class="card"><div class="card-label">Verdict</div><div class="card-value"><span class="${statusClass(summary.verdict)}">${escapeText(summary.verdict)}</span></div></div>
        <div class="card"><div class="card-label">Scenario</div><div class="card-value"><span class="${statusClass(summary.scenario)}">${escapeText(summary.scenario)}</span></div></div>
        <div class="card"><div class="card-label">Workers</div><div class="card-value">${escapeText(summary.workerCount)}</div></div>
        <div class="card"><div class="card-label">Evidence</div><div class="card-value">${evidence.length}</div></div>
        <div class="card"><div class="card-label">Mutations</div><div class="card-value">${mutations.length}</div></div>
      </div>
      <section class="panel"><h3>Evidence categories</h3>${renderCategorySummary(run.summary?.evidence_categories || run.evidence_categories || [])}</section>
      <section class="panel"><h3>Runtime probe contract</h3>${renderProbeContractStatus(run.probe_contract_status || run.summary?.probe_contract_status || {})}</section>
      <section class="panel"><h3>Tilemap authoring evidence</h3>${renderTilemapSummary(run.engine_summaries || {})}</section>
      <section class="panel"><h3>Scene render breakdown</h3>${renderRenderBreakdownSummary(run.engine_summaries || {})}</section>
      <section class="panel"><h3>Gameplay trigger/flags</h3>${renderGameplaySummary(run.engine_summaries || {})}</section>
      <section class="panel"><h3>Asset reference integrity</h3>${renderAssetIntegrity(run)}</section>
      <section class="panel"><h3>Runtime asset loading</h3>${renderAssetLoading(run)}</section>
      <section class="panel"><h3>Asset preview evidence</h3>${renderAssetPreview(run)}</section>
      <section class="panel"><h3>Source apply worktree context</h3>${renderSourceApplyWorktreeContext(run)}</section>
      ${renderSourcePatchEvidenceBundles(run)}
      ${renderSourcePatchApplyTransactions(run)}
      ${renderSourcePatchStaleTargetGuards(run)}
      <section class="panel"><h3>Verdict summary</h3><pre>${escapeText(JSON.stringify(verdict, null, 2))}</pre></section>
      ${renderCommandContext(run)}
      ${renderLoopDryRunSummary(run.loop_dry_run || run.loopDryRun || null)}
      ${renderLoopExecutionSummary(run.loop_execution || run.loopExecution || null)}
      ${renderLoopRecoveryStatus(run.loop_recovery || run.loopRecovery || run.loop_status || run.loopStatus || null)}
      ${renderAgentRoleModels(run.agent_role_models || run.agentRoleModels || run.agent_role_model || run.agentRoleModel || null)}
      ${renderAgentHandoffs(agentHandoffs || run.agent_handoffs || run.agentHandoffs || run.agent_handoff || run.agentHandoff || null)}
      ${renderLoopEvidenceBundles(loopEvidenceBundles || run.loop_evidence_bundles || run.loopEvidenceBundles || run.loop_evidence_bundle || run.loopEvidenceBundle || null)}
      ${renderJournalViewer(run)}
      ${renderMutationLifecycle(run)}
      ${renderRegressionPromotions(run)}
      ${renderRegressionMatrix(regressionMatrix)}
      ${renderProjectContext(run)}
      ${renderTransactionProvenance(run)}
      ${renderReplayControls(run, replayState)}
      ${renderRunComparison(run)}
      ${renderArtifacts('Screenshots', artifacts(run.screenshots), run, renderScreenshot)}
      ${renderArtifacts('World-state snapshots', artifacts(run.world_states), run, renderJsonArtifact)}
      ${renderArtifacts('Frame/performance metrics', artifacts(run.frame_metrics, run.performance_metrics), run, renderJsonArtifact)}
      ${renderArtifacts('Console/CDP summaries', artifacts(run.console_logs, run.cdp_trace_summaries), run, renderJsonArtifact)}
      ${renderArtifacts('Scenario results', artifacts(run.scenario_results), run, renderJsonArtifact)}
      ${renderArtifacts('Mutation artifacts', artifacts(run.mutation_artifacts), run, renderJsonArtifact)}
      ${renderArtifacts('Evidence index', evidence, run, renderArtifactLink)}
    </article>`;
  }

  async function init() {
    const listEl = document.getElementById('run-list');
    const detailEl = document.getElementById('run-detail');
    try {
      const response = await fetch('dashboard-data.json', { cache: 'no-store' });
      if (!response.ok) throw new Error(`failed to load dashboard-data.json: ${response.status}`);
      const data = await response.json();
      if (!Array.isArray(data.runs)) throw new Error('malformed dashboard-data.json: runs must be an array');
      const runs = data.runs || [];
      let selected = runs[0] || null;
      const replayStates = new Map();
      const replayStateFor = (run) => {
        const id = run?.summary?.id || 'unknown-run';
        if (!replayStates.has(id)) replayStates.set(id, createReplayState(run));
        return replayStates.get(id);
      };
      const paint = () => {
        listEl.innerHTML = renderRunList(runs, selected && selected.summary.id);
        detailEl.innerHTML = renderRunDetailWithState(selected, replayStateFor(selected), data.regression_matrix || data.regressionMatrix || null, data.loop_evidence_bundles || data.loopEvidenceBundles || null, data.agent_handoffs || data.agentHandoffs || null);
        listEl.querySelectorAll('[data-run-id]').forEach((button) => {
          button.addEventListener('click', () => {
            selected = runs.find((run) => run.summary.id === button.dataset.runId) || null;
            paint();
          });
        });
        detailEl.querySelectorAll('[data-replay-step]').forEach((button) => {
          button.addEventListener('click', () => {
            replayStates.set(selected.summary.id, stepReplayForward(selected, replayStateFor(selected)));
            paint();
          });
        });
        detailEl.querySelectorAll('[data-replay-reset]').forEach((button) => {
          button.addEventListener('click', () => {
            replayStates.set(selected.summary.id, resetReplay(selected));
            paint();
          });
        });
        detailEl.querySelectorAll('[data-replay-jump]').forEach((button) => {
          button.addEventListener('click', () => {
            replayStates.set(selected.summary.id, jumpReplayToCheckpoint(selected, replayStateFor(selected), Number(button.dataset.replayJump)));
            paint();
          });
        });
      };
      paint();
    } catch (error) {
      listEl.innerHTML = `<div class="empty-state">${escapeText(error.message)}</div>`;
      detailEl.innerHTML = '<div class="empty-state">Generate dashboard data with the Rust CLI export command, then refresh.</div>';
    }
  }

  return { artifactHref, commandContext, comparisonRefHref, createReplayState, currentReplayView, init, jumpReplayToCheckpoint, renderAgentRoleModels, renderAgentHandoffs, renderAssetIntegrity, renderAssetLoading, renderAssetPreview, renderSourceApplyWorktreeContext, renderSourcePatchEvidenceBundles, renderSourcePatchApplyTransactions, renderSourcePatchStaleTargetGuards, renderCategorySummary, renderCommandContext, renderGameplaySummary, renderRenderBreakdownSummary, renderTilemapSummary, renderJournalViewer, renderLoopDryRunSummary, renderLoopExecutionSummary, renderLoopEvidenceBundles, renderLoopRecoveryStatus, renderMutationLifecycle, renderProposalRationaleList, renderProbeContractStatus, renderProjectContext, renderRegressionMatrix, renderRegressionPromotions, renderReplayControls, renderRunComparison, renderRunDetail, renderRunDetailWithState, renderRunList, renderSemanticDiffSummary, renderTransactionProvenance, resetReplay, runRelativeHref, statusClass, stepReplayForward, summarizeRun };
})();

if (typeof window !== 'undefined') {
  window.OuroforgeDashboard = OuroforgeDashboard;
  window.addEventListener('DOMContentLoaded', () => OuroforgeDashboard.init?.());
}

if (typeof module !== 'undefined') {
  module.exports = OuroforgeDashboard;
}
