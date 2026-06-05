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
    const scene3dRender = summary?.scene3d_render || summary?.scene3dRender || {};
    const elements = Array.isArray(breakdown.elements) ? breakdown.elements : [];
    const absence = Array.isArray(breakdown.absenceDiagnostics || breakdown.absence_diagnostics) ? (breakdown.absenceDiagnostics || breakdown.absence_diagnostics) : [];
    const boundary = breakdown.readOnlyInspection || breakdown.read_only_inspection || {};
    const disallowed = Array.isArray(boundary.disallowedActions || boundary.disallowed_actions) ? (boundary.disallowedActions || boundary.disallowed_actions).join(', ') : 'trusted writes, command bridge, live mutation';
    const queueRenderables = Array.isArray(queue.renderables) ? queue.renderables : [];
    const scene3dRenderables = Array.isArray(scene3dRender.renderables) ? scene3dRender.renderables : [];
    const scene3dFallbacks = Array.isArray(scene3dRender.fallbackReasons || scene3dRender.fallback_reasons) ? (scene3dRender.fallbackReasons || scene3dRender.fallback_reasons) : [];
    const queueValidation = queue.validation || {};
    const tilemapStats = queue.tilemapStats || queue.tilemap_stats || {};
    const rows = [['Frame', breakdown.frameId || breakdown.frame_id || 'unknown'], ['Scene', breakdown.sceneId || breakdown.scene_id || 'unknown'], ['Renderable elements', elements.length], ['Absence diagnostics', absence.length], ['Queue layers', queue.layerCount ?? queue.layer_count ?? 0], ['Queue renderables', queue.renderableCount ?? queue.renderable_count ?? queueRenderables.length], ['Draw calls', queue.drawCallCount ?? queue.draw_call_count ?? 0], ['Queue status', queueValidation.status || 'unreported'], ['3D smoke visible/skipped', scene3dRender.present ? `${scene3dRender.visibleObjectCount ?? scene3dRender.visible_object_count ?? 0}/${scene3dRender.skippedObjectCount ?? scene3dRender.skipped_object_count ?? 0}` : 'not exported'], ['Tilemap draw tiles', tilemapStats.drawnTileCount ?? tilemapStats.drawn_tile_count ?? 0], ['Asset-backed tiles', tilemapStats.assetTileCount ?? tilemapStats.asset_tile_count ?? 0], ['Missing tile refs', tilemapStats.missingTileRefCount ?? tilemapStats.missing_tile_ref_count ?? 0]].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const elementRows = elements.slice(0, 6).map((element) => `<li><strong>${escapeText(element?.renderableId || element?.entityId || 'renderable')}</strong>: draw ${escapeText(element?.drawOrder ?? '?')} · ${escapeText(element?.layer || 'default')} · ${escapeText(element?.primitiveCategory || 'unknown')}</li>`).join('') || '<li>No renderable elements recorded.</li>';
    const absenceRows = absence.slice(0, 6).map((diag) => `<li><strong>${escapeText(diag?.entityId || diag?.renderableId || 'renderable')}</strong>: ${escapeText(diag?.reason || 'unknown')} · ${escapeText(diag?.detail || '')}</li>`).join('') || '<li>No hidden, skipped, fallback, or malformed diagnostics recorded.</li>';
    const queueRows = queueRenderables.slice(0, 6).map((renderable) => `<li><strong>${escapeText(renderable?.id || 'queue-renderable')}</strong>: draw ${escapeText(renderable?.drawOrder ?? '?')} · ${escapeText(renderable?.layer || 'default')} · ${escapeText(renderable?.primitiveKind || 'unknown')} · ${escapeText(renderable?.visible === false ? (renderable?.fallbackReason || 'skipped') : 'visible')} · tiles ${escapeText(renderable?.tileCount ?? 0)} · missing ${escapeText(renderable?.missingTileRefCount ?? 0)}</li>`).join('') || '<li>No render queue renderables recorded.</li>';
    const scene3dRows = scene3dRender.present
      ? scene3dRenderables.slice(0, 6).map((renderable) => `<li><strong>${escapeText(renderable?.id || renderable?.nodeId || 'scene3d-renderable')}</strong>: ${escapeText(renderable?.primitive || renderable?.meshKind || 'unknown')} · camera ${escapeText(renderable?.cameraId || scene3dRender.cameraId || 'none')} · ${escapeText(renderable?.visible === false ? (renderable?.fallbackReason || 'skipped') : 'visible')}</li>`).join('') || '<li>No 3D render smoke rows recorded.</li>'
      : `<li>${escapeText(scene3dRender.emptyState || 'No 3D render smoke evidence is available.')}</li>`;
    const fallbackRows = scene3dFallbacks.slice(0, 6).map((reason) => `<li>${escapeText(reason)}</li>`).join('') || '<li>No 3D render fallback reasons recorded.</li>';
    return `<div class="field-grid">${rows}</div><h4>Renderables</h4><ul class="run-meta-list">${elementRows}</ul><h4>Render queue</h4><ul class="run-meta-list">${queueRows}</ul><h4>3D render smoke</h4><p class="run-meta">Frame ${escapeText(scene3dRender.frameId || scene3dRender.frame_id || 'unknown')} · scene ${escapeText(scene3dRender.sceneId || scene3dRender.scene_id || 'unknown')} · mesh/material ${escapeText(scene3dRender.meshCount ?? scene3dRender.mesh_count ?? 0)}/${escapeText(scene3dRender.materialCount ?? scene3dRender.material_count ?? 0)} · screenshot ${escapeText(scene3dRender.screenshotArtifact || scene3dRender.screenshot_artifact || 'not produced')}</p><ul class="run-meta-list">${scene3dRows}</ul><h4>3D render fallbacks</h4><ul class="run-meta-list">${fallbackRows}</ul><h4>Absence diagnostics</h4><ul class="run-meta-list">${absenceRows}</ul><p class="run-meta">Read-only inspection only; disallowed actions: ${escapeText(disallowed)}. ${escapeText(scene3dRender.boundary || '')}</p>`;
  }

  function renderCameraLayerSummary(summary = {}) {
    const camera = summary?.camera || summary?.camera_state || summary?.cameraState || {};
    const renderer = summary?.renderer || {};
    const queue = summary?.render_queue || summary?.renderQueue || {};
    if (!summary?.present || !camera || typeof camera !== 'object' || Array.isArray(camera)) {
      return '<p class="empty-state">No camera/layer read model is available.</p>';
    }
    const cameras = Array.isArray(camera.cameras) ? camera.cameras : [];
    const layers = Array.isArray(renderer.layers)
      ? renderer.layers
      : (Array.isArray(queue.layers) ? queue.layers : []);
    const worldToScreen = camera.worldToScreen && typeof camera.worldToScreen === 'object' && !Array.isArray(camera.worldToScreen)
      ? camera.worldToScreen
      : {};
    const active = camera.activeCamera || cameras.find((entry) => entry && entry.id === camera.activeCameraId) || cameras.find((entry) => entry && entry.active) || {};
    const scene3dCamera = camera.scene3dCamera || camera.scene3d_camera || camera.camera3d || {};
    const scene3dCameras = Array.isArray(scene3dCamera.cameras) ? scene3dCamera.cameras : [];
    const active3d = scene3dCamera.activeCamera || scene3dCamera.active_camera || scene3dCameras.find((entry) => entry && entry.id === scene3dCamera.activeCameraId) || {};
    const rendererCamera = camera.rendererCamera || renderer.camera || {};
    const viewport = camera.viewport || renderer.viewport || active.viewport || {};
    const rows = [
      ['Active camera', camera.activeCameraId || active.id || 'default'],
      ['Renderer camera', JSON.stringify(rendererCamera)],
      ['Viewport', JSON.stringify(viewport)],
      ['Camera records', cameras.length],
      ['3D camera records', scene3dCameras.length],
      ['Layer records', layers.length],
      ['World-to-screen samples', Object.keys(worldToScreen).length],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const cameraRows = cameras.slice(0, 6).map((entry) => `<li><strong>${escapeText(entry?.id || 'camera')}</strong>: ${escapeText(entry?.active ? 'active' : 'inactive')} · follow ${escapeText(entry?.followTarget || 'none')} · position ${escapeText(JSON.stringify(entry?.position || {}))} · clamp ${escapeText(JSON.stringify(entry?.clampBounds || {}))}</li>`).join('') || '<li>No camera records exported.</li>';
    const camera3dRows = scene3dCameras.slice(0, 6).map((entry) => `<li><strong>${escapeText(entry?.id || 'camera3d')}</strong>: ${escapeText(entry?.active ? 'active' : 'inactive')} · projection ${escapeText(entry?.projection?.kind || 'unknown')} · fov ${escapeText(entry?.projection?.fovDegrees ?? 'n/a')} · near/far ${escapeText(entry?.projection?.near ?? '?')}/${escapeText(entry?.projection?.far ?? '?')} · aspect×1000 ${escapeText(entry?.aspectRatioX1000 ?? 'n/a')} · viewport ${escapeText(JSON.stringify(entry?.viewport || {}))}</li>`).join('') || '<li>No 3D camera records exported.</li>';
    const layerRows = layers.slice(0, 8).map((layer) => `<li><strong>${escapeText(layer?.id || 'layer')}</strong>: order ${escapeText(layer?.order ?? '?')} · ${escapeText(layer?.visible === false ? 'hidden' : 'visible')} · parallax ${escapeText(layer?.parallaxFactor ?? 'n/a')} · camera ${escapeText(layer?.cameraParticipation === false ? 'disabled' : 'participates')}</li>`).join('') || '<li>No layer records exported.</li>';
    const sampleRows = Object.entries(worldToScreen).slice(0, 6).map(([entityId, sample]) => `<li><strong>${escapeText(entityId)}</strong>: screen ${escapeText(JSON.stringify({ x: sample?.x, y: sample?.y }))} · layer ${escapeText(sample?.layer || 'default')} · offset ${escapeText(JSON.stringify(sample?.cameraOffset || {}))}</li>`).join('') || '<li>No world-to-screen samples exported.</li>';
    const camera3dSection = scene3dCamera.present
      ? `<h4>3D cameras</h4><p class="run-meta">Active 3D camera: ${escapeText(scene3dCamera.activeCameraId || active3d.id || 'none')}. Read-only 3D camera evidence; no viewport persistence or camera editor tooling.</p><ul class="run-meta-list">${camera3dRows}</ul>`
      : `<h4>3D cameras</h4><p class="empty-state compact">${escapeText(scene3dCamera.emptyState || 'No 3D camera evidence is available.')}</p>`;
    return `<div class="field-grid">${rows}</div><h4>Cameras</h4><ul class="run-meta-list">${cameraRows}</ul>${camera3dSection}<h4>Layers</h4><ul class="run-meta-list">${layerRows}</ul><h4>World-to-screen samples</h4><ul class="run-meta-list">${sampleRows}</ul><p class="run-meta">Read-only camera/layer evidence only; the dashboard cannot write scene state, execute commands, or control the browser runtime.</p>`;
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

  function renderAnimationVfxSummary(summary = {}) {
    const animation = summary?.animation || {};
    const vfx = summary?.vfx || {};
    const events = summary?.events || {};
    if (!summary?.present || (!animation.animatedEntityCount && !vfx.present && !events.present)) {
      return '<p class="empty-state">No animation/VFX read model is available.</p>';
    }
    const animationRows = Array.isArray(events.animationEntities) && events.animationEntities.length
      ? events.animationEntities.map((entity) => `<li><strong>${escapeText(entity?.entityId || 'entity')}</strong>: state ${escapeText(entity?.activeState || 'none')} · clip ${escapeText(entity?.currentClip || 'none')} · frame ${escapeText(entity?.frameIndex ?? 'unknown')}</li>`).join('')
      : '<li>No animation entity rows exported.</li>';
    const vfxRows = Array.isArray(events.vfxEvents) && events.vfxEvents.length
      ? events.vfxEvents.map((event) => `<li><strong>${escapeText(event?.emitterId || 'vfx emitter')}</strong>: ${escapeText(event?.kind || 'vfx')} · particles ${escapeText(event?.particleCount ?? 'unknown')}</li>`).join('')
      : '<li>No VFX event rows exported.</li>';
    const rows = [
      ['Animated entities', animation.animatedEntityCount ?? events.animationEntityCount ?? 0],
      ['Active animation states', animation.activeStateCount ?? 0],
      ['VFX entities', vfx.vfxEntityCount ?? 0],
      ['VFX emitters', vfx.vfxEmitterCount ?? 0],
      ['VFX events', vfx.vfxEventCount ?? events.vfxEventCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    return `<div class="field-grid">${rows}</div><h4>Animation entities</h4><ul class="run-meta-list">${animationRows}</ul><h4>VFX events</h4><ul class="run-meta-list">${vfxRows}</ul><p class="run-meta">Read-only animation/VFX evidence only; the dashboard cannot write scene state, execute commands, or control the browser runtime.</p>`;
  }

  function renderAudioEvidenceSummary(summary = {}) {
    const audio = summary?.audio || {};
    const events = summary?.events || {};
    if (!summary?.present || (!audio.audioEventCount && !events.audioEventCount)) {
      return '<p class="empty-state">No audio intent evidence is available.</p>';
    }
    const audioEvents = Array.isArray(audio.audioEvents) && audio.audioEvents.length
      ? audio.audioEvents
      : (Array.isArray(events.audioEvents) ? events.audioEvents : []);
    const audioWarnings = Array.isArray(audio.audioWarnings) && audio.audioWarnings.length
      ? audio.audioWarnings
      : (Array.isArray(events.audioWarnings) ? events.audioWarnings : []);
    const eventRows = audioEvents.length
      ? audioEvents.map((event) => `<li><strong>${escapeText(event?.name || event?.type || event?.kind || 'audio event')}</strong>: ${escapeText(event?.intentKind || event?.kind || 'sound')} · bus ${escapeText(event?.busId || 'default')} · volume ${escapeText(event?.volume ?? 'unknown')}</li>`).join('')
      : '<li>No audio intent event rows exported.</li>';
    const warningRows = audioWarnings.length
      ? audioWarnings.map((warning) => `<li><strong>${escapeText(warning?.warning || 'audio warning')}</strong>: request ${escapeText(warning?.requestId || 'unknown')}</li>`).join('')
      : '<li>No browser audio limitation warnings exported.</li>';
    const rows = [
      ['Audio entities', audio.audioEntityCount ?? 0],
      ['Audio intent events', audio.audioEventCount ?? events.audioEventCount ?? audioEvents.length],
      ['Audio warnings', audio.audioWarningCount ?? events.audioWarningCount ?? audioWarnings.length],
      ['Authority', audio.browserAudioAuthority || 'intent_evidence_only'],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    return `<div class="field-grid">${rows}</div><h4>Audio intent events</h4><ul class="run-meta-list">${eventRows}</ul><h4>Browser limitation warnings</h4><ul class="run-meta-list">${warningRows}</ul><p class="run-meta">Read-only audio intent evidence only; the dashboard cannot verify audible output, write scene state, execute commands, or control the browser audio device.</p>`;
  }


  function renderRuntimeProfilerSummary(summary = {}) {
    const profiler = summary?.runtime_frame_budget || summary?.runtimeFrameBudget || summary?.runtime_profiler || summary?.runtimeProfiler || null;
    if (!summary?.present || !profiler || typeof profiler !== 'object' || Array.isArray(profiler)) {
      return '<p class="empty-state">No runtime profiler/frame-budget read model is available.</p>';
    }
    const timings = profiler.timings && typeof profiler.timings === 'object' && !Array.isArray(profiler.timings) ? profiler.timings : {};
    const budget = profiler.budget && typeof profiler.budget === 'object' && !Array.isArray(profiler.budget) ? profiler.budget : {};
    const counts = profiler.counts && typeof profiler.counts === 'object' && !Array.isArray(profiler.counts) ? profiler.counts : {};
    const scene3dCollision = summary?.scene3d_collision && typeof summary.scene3d_collision === 'object' && !Array.isArray(summary.scene3d_collision)
      ? summary.scene3d_collision
      : (summary?.scene3dCollision && typeof summary.scene3dCollision === 'object' && !Array.isArray(summary.scene3dCollision) ? summary.scene3dCollision : null);
    const violations = Array.isArray(profiler.violations) ? profiler.violations : [];
    const boundary = profiler.readOnlyInspection || profiler.read_only_inspection || {};
    const disallowed = Array.isArray(boundary.disallowedActions || boundary.disallowed_actions)
      ? (boundary.disallowedActions || boundary.disallowed_actions).join(', ')
      : 'trusted writes, command bridge, live mutation, remote telemetry';
    const status = profiler.status || (violations.length ? 'violated' : 'within-budget');
    const rows = [
      ['Frame', profiler.frameId || profiler.frame_id || 'unknown'],
      ['Scene', profiler.sceneId || profiler.scene_id || 'unknown'],
      ['Scenario', profiler.scenarioId || profiler.scenario_id || 'none'],
      ['Status', status],
      ['Slow frame', profiler.slowFrame ?? profiler.slow_frame ?? violations.length > 0],
      ['Update ms', `${timings.updateMs ?? timings.update_ms ?? 'missing'} / ${budget.updateMs ?? budget.update_ms ?? 'missing'}`],
      ['Render ms', `${timings.renderMs ?? timings.render_ms ?? 'missing'} / ${budget.renderMs ?? budget.render_ms ?? 'missing'}`],
      ['Evidence ms', `${timings.evidenceMs ?? timings.evidence_ms ?? 'missing'} / ${budget.evidenceMs ?? budget.evidence_ms ?? 'missing'}`],
      ['Total ms', `${timings.totalMs ?? timings.total_ms ?? 'missing'} / ${budget.totalMs ?? budget.total_ms ?? 'missing'}`],
      ['Entities', counts.entityCount ?? counts.entity_count ?? 0],
      ['Draw calls', counts.drawCallCount ?? counts.draw_call_count ?? 0],
      ['Layers', counts.layerCount ?? counts.layer_count ?? 0],
      ['Collision pairs', counts.collisionPairCount ?? counts.collision_pair_count ?? 0],
      ['3D collision', `${scene3dCollision?.contactCount ?? 0} contact / ${scene3dCollision?.triggerCount ?? 0} trigger / ${scene3dCollision?.invalidColliderCount ?? 0} invalid`],
      ['Animations/VFX/Audio', `${counts.activeAnimationCount ?? counts.active_animation_count ?? 0} / ${counts.activeVfxCount ?? counts.active_vfx_count ?? 0} / ${counts.audioEventCount ?? counts.audio_event_count ?? 0}`],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const violationRows = violations.length
      ? violations.map((violation) => `<li><strong>${escapeText(violation?.field || 'metric')}</strong>: actual ${escapeText(violation?.actualMs ?? violation?.actual_ms ?? 'missing')}ms / budget ${escapeText(violation?.budgetMs ?? violation?.budget_ms ?? 'missing')}ms</li>`).join('')
      : '<li>No frame-budget violations recorded.</li>';
    const invalidColliderRows = Array.isArray(scene3dCollision?.invalidColliders) && scene3dCollision.invalidColliders.length
      ? scene3dCollision.invalidColliders.map((entry) => `<li><strong>${escapeText(entry?.colliderId || entry?.colliderRef || entry?.nodeId || 'invalid 3D collider')}</strong>: ${escapeText(JSON.stringify(entry))}</li>`).join('')
      : '<li>No invalid 3D collider evidence recorded.</li>';
    const authority = profiler.authority || 'browser_runtime_evidence_input_not_profiler_truth';
    return `<div class="field-grid">${rows}</div><h4>Budget violations</h4><ul class="run-meta-list">${violationRows}</ul><h4>Invalid 3D colliders</h4><ul class="run-meta-list">${invalidColliderRows}</ul><p class="run-meta">Read-only runtime profiler evidence only; browser observations are evidence inputs, not trusted authority. Authority: ${escapeText(authority)}. Disallowed actions: ${escapeText(disallowed)}. ${escapeText(scene3dCollision?.boundary || '')}</p>`;
  }

  function renderInputActionSummary(summary = {}) {
    const input = summary?.input || {};
    if (!summary?.present || !input.present) {
      return '<p class="empty-state">No input action read model is available.</p>';
    }
    const activeActions = Array.isArray(input.activeActions) && input.activeActions.length
      ? input.activeActions.join(', ')
      : 'none';
    const diagnostics = input.diagnostics && typeof input.diagnostics === 'object' ? input.diagnostics : {};
    const warningRows = [
      ['Missing actions', diagnostics.missingActions],
      ['Unmapped actions', diagnostics.unmappedActions],
      ['Duplicate actions', diagnostics.duplicateActions],
      ['Unresolved overrides', diagnostics.unresolvedOverrides],
    ].map(([label, values]) => `<li><strong>${escapeText(label)}</strong>: ${escapeText(Array.isArray(values) && values.length ? values.join(', ') : 'none')}</li>`).join('');
    const conflictRows = Array.isArray(diagnostics.conflictingBindings) && diagnostics.conflictingBindings.length
      ? diagnostics.conflictingBindings.map((conflict) => `<li><strong>Conflict ${escapeText(conflict.key || 'key')}</strong>: ${escapeText(Array.isArray(conflict.actions) ? conflict.actions.join(' / ') : 'unknown')}</li>`).join('')
      : '<li><strong>Conflicting bindings</strong>: none</li>';
    const rows = [
      ['Mapped actions', input.mappedActionCount ?? 0],
      ['Active actions', `${input.activeActionCount ?? 0} (${activeActions})`],
      ['Warnings', input.warningCount ?? diagnostics.warningCount ?? 0],
      ['Raw keys', Object.keys(input.rawInput?.keys || {}).filter((key) => input.rawInput.keys[key]).join(', ') || 'none'],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const boundary = diagnostics.readOnlyInspection || {};
    const disallowed = Array.isArray(boundary.disallowedActions) ? boundary.disallowedActions.join(', ') : 'trusted writes, command bridge, live mutation';
    return `<div class="field-grid">${rows}</div><ul class="run-meta-list">${warningRows}${conflictRows}</ul><p class="run-meta">Read-only input action evidence; disallowed actions: ${escapeText(disallowed)}</p>`;
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

  function renderPluginRegistry(run = {}) {
    const registry = run.plugin_registry || run.pluginRegistry || {};
    if (!registry.present) {
      return `<p class="empty-state">${escapeText(registry.empty_state || 'No plugin registry evidence is available for this run.')}</p>`;
    }
    const registries = Array.isArray(registry.registries) ? registry.registries : [];
    const refs = Array.isArray(registry.evidence_refs || registry.evidenceRefs) ? (registry.evidence_refs || registry.evidenceRefs) : [];
    const rows = [
      ['Status', registry.status || 'unknown'],
      ['Registries', registry.registry_count ?? registry.registryCount ?? registries.length],
      ['Plugins', registry.plugin_count ?? registry.pluginCount ?? registries.reduce((count, item) => count + (item.plugin_count ?? item.pluginCount ?? (Array.isArray(item.plugins) ? item.plugins.length : 0)), 0)],
      ['Blocked', registry.blocked_count ?? registry.blockedCount ?? 0],
      ['Malformed', registry.malformed_count ?? registry.malformedCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const registryRows = registries.length
      ? registries.map((item) => {
          const plugins = Array.isArray(item.plugins) ? item.plugins : [];
          const pluginRows = plugins.slice(0, 12).map((plugin) => {
            const caps = Array.isArray(plugin.declared_capabilities || plugin.declaredCapabilities) ? (plugin.declared_capabilities || plugin.declaredCapabilities).join(', ') : 'none';
            const points = Array.isArray(plugin.extension_points || plugin.extensionPoints) ? (plugin.extension_points || plugin.extensionPoints).join(', ') : 'none';
            const reasons = Array.isArray(plugin.blocked_reasons || plugin.blockedReasons) ? (plugin.blocked_reasons || plugin.blockedReasons).join(' · ') : '';
            const panels = Array.isArray(plugin.dashboard_panels || plugin.dashboardPanels) ? (plugin.dashboard_panels || plugin.dashboardPanels) : [];
            const panelRows = panels.length
              ? `<ul class="run-meta-list">${panels.slice(0, 6).map((panel) => {
                  const hints = Array.isArray(panel.display_hints || panel.displayHints) ? (panel.display_hints || panel.displayHints).join(', ') : 'none';
                  return `<li><strong>${escapeText(panel.panel_id || panel.panelId || 'panel')}</strong>: ${escapeText(panel.title || 'Untitled panel')} · data ${escapeText(panel.data_source_key || panel.dataSourceKey || 'unknown')} · template ${escapeText(panel.template_ref || panel.templateRef || 'unknown')} · layout ${escapeText(panel.layout_hint || panel.layoutHint || 'unknown')} · hints ${escapeText(hints)}<br><small>${escapeText(panel.boundary || 'Declarative allowlisted read-only dashboard panel descriptor; no JavaScript, no commands, no trusted writes.')}</small></li>`;
                }).join('')}</ul>`
              : '';
            const templates = Array.isArray(plugin.scenario_templates || plugin.scenarioTemplates) ? (plugin.scenario_templates || plugin.scenarioTemplates) : [];
            const templateRows = templates.length
              ? `<ul class="run-meta-list">${templates.slice(0, 6).map((template) => {
                  const params = Array.isArray(template.parameters) ? template.parameters : [];
                  const paramSummary = params.slice(0, 8).map((parameter) => `${parameter.name || 'parameter'}:${parameter.parameter_type || parameter.parameterType || parameter.type || 'unknown'}${parameter.required ? ':required' : ''}`).join(', ') || 'none';
                  const games = Array.isArray(template.supported_game_types || template.supportedGameTypes) ? (template.supported_game_types || template.supportedGameTypes).join(', ') : 'none';
                  const tags = Array.isArray(template.tags) ? template.tags.join(', ') : 'none';
                  const hints = Array.isArray(template.validation_hints || template.validationHints) ? (template.validation_hints || template.validationHints).join(' · ') : 'none';
                  return `<li><strong>${escapeText(template.template_id || template.templateId || 'scenario template')}</strong>: ${escapeText(template.description || 'No description')} · evidence ${escapeText(template.expected_evidence_type || template.expectedEvidenceType || 'unknown')} · games ${escapeText(games)} · tags ${escapeText(tags)} · parameters ${escapeText(paramSummary)}<br><small>${escapeText(hints)} · ${escapeText(template.boundary || 'Declarative read-only scenario template metadata only; no executable scripts, no commands, no network, no source mutation, no trusted writes.')}</small></li>`;
                }).join('')}</ul>`
              : '';
            return `<li><strong>${escapeText(plugin.plugin_id || plugin.pluginId || 'unknown plugin')}</strong>: <span class="${statusClass(plugin.validation_status || plugin.validationStatus)}">${escapeText(plugin.validation_status || plugin.validationStatus || 'unknown')}</span> / <span class="${statusClass(plugin.compatibility_status || plugin.compatibilityStatus)}">${escapeText(plugin.compatibility_status || plugin.compatibilityStatus || 'unknown')}</span><br><small>version ${escapeText(plugin.manifest_version || plugin.manifestVersion || 'unknown')} · hash ${escapeText(plugin.manifest_hash || plugin.manifestHash || 'hash missing')} · manifest ${escapeText(plugin.manifest_path || plugin.manifestPath || 'manifest missing')} · capabilities ${escapeText(caps)} · extension points ${escapeText(points)}</small>${panelRows}${templateRows}${reasons ? `<div class="artifact-warning">${escapeText(reasons)}</div>` : ''}</li>`;
          }).join('') || '<li>No plugin rows exported.</li>';
          const blocked = Array.isArray(item.blocked_reasons || item.blockedReasons) ? (item.blocked_reasons || item.blockedReasons) : [];
          return `<article class="artifact"><h4>${escapeText(item.registry_id || item.registryId || 'plugin registry')}</h4>
            <div class="run-meta">project ${escapeText(item.project_id || item.projectId || 'unknown')} · run ${escapeText(item.run_id || item.runId || 'unknown')} · ledger ${escapeText(item.ledger_ref || item.ledgerRef || 'none')}</div>
            <div class="run-meta">status ${escapeText(item.status || 'unknown')} · plugins ${escapeText(item.plugin_count ?? item.pluginCount ?? plugins.length)} · blocked ${escapeText(item.blocked_count ?? item.blockedCount ?? blocked.length)}</div>
            ${blocked.length ? `<div class="artifact-warning">Blocked: ${escapeText(blocked.join(' · '))}</div>` : ''}
            <ul class="run-meta-list">${pluginRows}</ul>
          </article>`;
        }).join('')
      : '<p class="empty-state compact">No parsed plugin registry rows are available.</p>';
    return `<div class="field-grid">${rows}</div>
      <div class="artifact-grid">${registryRows}</div>
      ${renderRefLinks('Plugin registry evidence refs', refs, run)}
      <p class="run-meta">${escapeText(registry.boundary || 'Read-only plugin registry evidence. The dashboard never installs plugins, runs commands, writes trusted files, or loads executable extensions.')}</p>`;
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



  function renderRouteAttempts(run = {}) {
    const model = run.route_attempts || run.routeAttempts || {};
    if (!model.present) {
      return `<p class="empty-state">${escapeText(model.empty_state || 'No route attempt evidence is available for this run.')}</p>`;
    }
    const refs = Array.isArray(model.evidence_refs || model.evidenceRefs) ? (model.evidence_refs || model.evidenceRefs) : [];
    const attempts = Array.isArray(model.attempts) ? model.attempts : [];
    const rows = [
      ['Status', model.status || 'unknown'],
      ['Attempts', model.attempt_count ?? model.attemptCount ?? 0],
      ['Passed/failed', `${model.passed_count ?? model.passedCount ?? 0}/${model.failed_count ?? model.failedCount ?? 0}`],
      ['Blocked/inconclusive', `${model.blocked_count ?? model.blockedCount ?? 0}/${model.inconclusive_count ?? model.inconclusiveCount ?? 0}`],
      ['Unsupported', model.unsupported_count ?? model.unsupportedCount ?? 0],
      ['Malformed', model.malformed_count ?? model.malformedCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br><span class="${label === 'Status' ? statusClass(value) : ''}">${escapeText(value)}</span></div>`).join('');
    const attemptRows = attempts.slice(0, 12).map((attempt) => {
      const startState = attempt.startState || attempt.start_state || {};
      const budget = attempt.budgetUsed || attempt.budget_used || {};
      const blockers = Array.isArray(attempt.blockers) ? attempt.blockers : [];
      return `<li><strong>${escapeText(attempt.attemptId || attempt.attempt_id || 'route attempt')}</strong>: <span class="${statusClass(attempt.outcome)}">${escapeText(attempt.outcome || 'unknown')}</span> · ${escapeText(attempt.strategyKind || attempt.strategy_kind || 'strategy')}<br><small>${escapeText(attempt.objectiveId || attempt.objective_id || 'objective')} · ${escapeText(attempt.scenarioId || attempt.scenario_id || 'scenario')} · start ${escapeText(startState.stateId || startState.state_id || 'state')} · actions ${escapeText(budget.actionsUsed ?? budget.actions_used ?? '?')}/${escapeText(budget.maxActions ?? budget.max_actions ?? '?')} · route ${escapeText(budget.routeNodesUsed ?? budget.route_nodes_used ?? '?')}/${escapeText(budget.maxRouteNodes ?? budget.max_route_nodes ?? '?')} · ${escapeText((attempt.unsupportedReason || attempt.unsupported_reason) || blockers.map((blocker) => blocker.reason).join(' · ') || 'bounded route evidence')}</small></li>`;
    }).join('') || '<li>No parseable route attempts are available.</li>';
    return `<div class="field-grid">${rows}</div>
      <h4>Route attempts</h4><ul class="run-meta-list">${attemptRows}</ul>
      ${renderRefLinks('Route attempt evidence refs', refs, run)}
      <p class="run-meta">${escapeText(model.boundary || 'Read-only route attempt evidence; dashboard surfaces do not run solvers, spawn workers, execute commands, write trusted state, auto-fix, auto-apply, or auto-merge.')}</p>`;
  }

  function renderVisualComparisons(run = {}) {
    const model = run.visual_comparisons || run.visualComparisons || {};
    if (!model.present) {
      return `<p class="empty-state">${escapeText(model.empty_state || 'No visual comparison evidence is available for this run.')}</p>`;
    }
    const refs = Array.isArray(model.evidence_refs || model.evidenceRefs) ? (model.evidence_refs || model.evidenceRefs) : [];
    const summaries = Array.isArray(model.summaries) ? model.summaries : [];
    const rows = [
      ['Status', model.status || 'unknown'],
      ['Comparisons', model.comparison_count ?? model.comparisonCount ?? summaries.length],
      ['Changed/unchanged', `${model.changed_count ?? model.changedCount ?? 0}/${model.unchanged_count ?? model.unchangedCount ?? 0}`],
      ['Missing/malformed screenshots', `${model.missing_screenshot_count ?? model.missingScreenshotCount ?? 0}/${model.malformed_screenshot_count ?? model.malformedScreenshotCount ?? 0}`],
      ['Mismatched/unsupported', `${model.mismatched_dimensions_count ?? model.mismatchedDimensionsCount ?? 0}/${model.unsupported_count ?? model.unsupportedCount ?? 0}`],
      ['Blocked/malformed artifacts', `${model.blocked_count ?? model.blockedCount ?? 0}/${model.malformed_count ?? model.malformedCount ?? 0}`],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br><span class="${label === 'Status' ? statusClass(value) : ''}">${escapeText(value)}</span></div>`).join('');
    const summaryRows = summaries.slice(0, 12).map((summary) => {
      const classification = summary.failureClassification || summary.failure_classification || 'visual_unclassified';
      const changedPixels = summary.changedPixels ?? summary.changed_pixels ?? 0;
      const changedPercent = summary.changedPercentX1000 ?? summary.changed_percent_x1000 ?? 0;
      const regionCount = summary.changedRegionCount ?? summary.changed_region_count ?? 0;
      return `<li><strong>${escapeText(summary.comparisonId || summary.comparison_id || 'visual comparison')}</strong>: <span class="${statusClass(summary.outcome)}">${escapeText(summary.outcome || 'unknown')}</span> · ${escapeText(classification)}<br><small>${escapeText(summary.scenarioId || summary.scenario_id || 'scenario')} · ${escapeText(summary.checkpointId || summary.checkpoint_id || 'checkpoint')} · changed ${escapeText(changedPixels)} px (${escapeText(changedPercent)} x1000) · regions ${escapeText(regionCount)} · ${escapeText(summary.beforeScreenshotRef || summary.before_screenshot_ref || 'before missing')} → ${escapeText(summary.afterScreenshotRef || summary.after_screenshot_ref || 'after missing')}</small></li>`;
    }).join('') || '<li>No parseable visual comparisons are available.</li>';
    return `<div class="field-grid">${rows}</div>
      <h4>Visual comparisons</h4><ul class="run-meta-list">${summaryRows}</ul>
      ${renderRefLinks('Visual comparison evidence refs', refs, run)}
      <p class="run-meta">${escapeText(model.boundary || 'Read-only visual comparison evidence; dashboard surfaces do not compute trusted diffs, execute commands, write trusted state, auto-fix, auto-apply, auto-merge, or claim aesthetic quality.')}</p>`;
  }

  function renderQaScenarioCandidates(run = {}) {
    const model = run.qa_scenario_candidates || run.qaScenarioCandidates || {};
    if (!model.present) {
      return `<p class="empty-state">${escapeText(model.empty_state || 'No QA scenario candidates are available for this run.')}</p>`;
    }
    const refs = Array.isArray(model.evidence_refs || model.evidenceRefs) ? (model.evidence_refs || model.evidenceRefs) : [];
    const candidates = Array.isArray(model.candidates) ? model.candidates : [];
    const rows = [
      ['Status', model.status || 'unknown'],
      ['Candidates', model.candidate_count ?? model.candidateCount ?? 0],
      ['High priority', model.high_priority_count ?? model.highPriorityCount ?? 0],
      ['Blocked/deferred', `${model.blocked_count ?? model.blockedCount ?? 0}/${model.deferred_count ?? model.deferredCount ?? 0}`],
      ['Malformed', model.malformed_count ?? model.malformedCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br><span class="${label === 'Status' ? statusClass(value) : ''}">${escapeText(value)}</span></div>`).join('');
    const candidateRows = candidates.slice(0, 12).map((candidate) => {
      const risk = candidate.sourceRisk || candidate.source_risk || {};
      const objective = candidate.targetObjective || candidate.target_objective || {};
      const input = candidate.inputStrategy || candidate.input_strategy || {};
      const budget = candidate.budget || {};
      const blocked = Array.isArray(candidate.blockedReasons || candidate.blocked_reasons) ? (candidate.blockedReasons || candidate.blocked_reasons) : [];
      const expected = Array.isArray(candidate.expectedEvidence || candidate.expected_evidence) ? (candidate.expectedEvidence || candidate.expected_evidence) : [];
      return `<li><strong>${escapeText(candidate.candidateId || candidate.candidate_id || 'scenario candidate')}</strong>: <span class="${statusClass(candidate.status)}">${escapeText(candidate.status || 'unknown')}</span> · ${escapeText(candidate.priority || 'priority')}<br><small>${escapeText(risk.riskId || risk.risk_id || 'risk')} → ${escapeText(objective.objectiveId || objective.objective_id || 'objective')} · ${escapeText(input.kind || 'input')} · maxRuns ${escapeText(budget.maxRuns ?? budget.max_runs ?? '?')} · expected ${escapeText(expected.length)} · ${escapeText(blocked.join(' · ') || objective.description || 'reviewable untrusted candidate')}</small></li>`;
    }).join('') || '<li>No parseable QA scenario candidates are available.</li>';
    return `<div class="field-grid">${rows}</div>
      <h4>Scenario candidates</h4><ul class="run-meta-list">${candidateRows}</ul>
      ${renderRefLinks('QA scenario candidate refs', refs, run)}
      <p class="run-meta">${escapeText(model.boundary || 'Read-only QA scenario candidates; dashboard surfaces do not run candidates, spawn workers, execute commands, write trusted state, auto-fix, auto-apply, or auto-merge.')}</p>`;
  }

  function renderFuzzingPlans(run = {}) {
    const model = run.fuzzing_plans || run.fuzzingPlans || {};
    if (!model.present) {
      return `<p class="empty-state">${escapeText(model.empty_state || 'No adversarial input fuzzing plans are available for this run.')}</p>`;
    }
    const refs = Array.isArray(model.evidence_refs || model.evidenceRefs) ? (model.evidence_refs || model.evidenceRefs) : [];
    const plans = Array.isArray(model.plans) ? model.plans : [];
    const rows = [
      ['Status', model.status || 'unknown'],
      ['Plans', model.plan_count ?? model.planCount ?? 0],
      ['Blocked', model.blocked_count ?? model.blockedCount ?? 0],
      ['Exhausted', model.exhausted_count ?? model.exhaustedCount ?? 0],
      ['Malformed', model.malformed_count ?? model.malformedCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br><span class="${label === 'Status' ? statusClass(value) : ''}">${escapeText(value)}</span></div>`).join('');
    const planRows = plans.slice(0, 12).map((plan) => {
      const inputDomain = plan.inputDomain || plan.input_domain || {};
      const budget = plan.budget || {};
      const cleanup = plan.cleanupPolicy || plan.cleanup_policy || {};
      const blocked = Array.isArray(plan.blockedReasons || plan.blocked_reasons) ? (plan.blockedReasons || plan.blocked_reasons) : [];
      const expected = Array.isArray(plan.expectedEvidence || plan.expected_evidence) ? (plan.expectedEvidence || plan.expected_evidence) : [];
      return `<li><strong>${escapeText(plan.planId || plan.plan_id || 'fuzzing plan')}</strong>: <span class="${statusClass(plan.status)}">${escapeText(plan.status || 'unknown')}</span> · seed ${escapeText(plan.deterministicSeed ?? plan.deterministic_seed ?? '?')}<br><small>${escapeText(inputDomain.scenarioId || inputDomain.scenario_id || 'scenario')} · maxRuns ${escapeText(budget.maxRuns ?? budget.max_runs ?? '?')} · maxSteps ${escapeText(budget.maxSteps ?? budget.max_steps ?? '?')} · cleanup ${escapeText(cleanup.mode || 'unknown')} · ${escapeText(plan.outputRoot || plan.output_root || 'no output root')} · expected ${escapeText(expected.length)} · ${escapeText(blocked.join(' · ') || 'bounded deterministic evidence plan')}</small></li>`;
    }).join('') || '<li>No parseable adversarial input fuzzing plans are available.</li>';
    return `<div class="field-grid">${rows}</div>
      <h4>Fuzzing plans</h4><ul class="run-meta-list">${planRows}</ul>
      ${renderRefLinks('Adversarial input fuzzing plan refs', refs, run)}
      <p class="run-meta">${escapeText(model.boundary || 'Read-only adversarial input fuzzing plans; dashboard surfaces do not run fuzzers, spawn workers, execute commands, write trusted state, auto-fix, auto-apply, or auto-merge.')}</p>`;
  }

  function renderQaWorkerAssignments(run = {}) {
    const model = run.qa_worker_assignments || run.qaWorkerAssignments || {};
    if (!model.present) {
      return `<p class="empty-state">${escapeText(model.empty_state || 'No QA worker assignment evidence is available for this run.')}</p>`;
    }
    const refs = Array.isArray(model.evidence_refs || model.evidenceRefs) ? (model.evidence_refs || model.evidenceRefs) : [];
    const plans = Array.isArray(model.plans) ? model.plans : [];
    const rows = [
      ['Status', model.status || 'unknown'],
      ['Assignments', model.assignment_count ?? model.assignmentCount ?? 0],
      ['Passed/failed', `${model.passed_count ?? model.passedCount ?? 0}/${model.failed_count ?? model.failedCount ?? 0}`],
      ['Deferred/blocked', `${model.deferred_count ?? model.deferredCount ?? 0}/${model.blocked_count ?? model.blockedCount ?? 0}`],
      ['Exhausted', model.exhausted_count ?? model.exhaustedCount ?? 0],
      ['Malformed', model.malformed_count ?? model.malformedCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br>${escapeText(value)}</div>`).join('');
    const assignmentRows = plans.flatMap((plan) => Array.isArray(plan.assignments) ? plan.assignments.map((assignment) => ({ plan, assignment })) : []).slice(0, 12).map(({ plan, assignment }) => {
      const budget = assignment.budget || {};
      const target = assignment.target || {};
      const cleanup = assignment.cleanupPolicy || assignment.cleanup_policy || {};
      const blocked = Array.isArray(assignment.blockedReasons || assignment.blocked_reasons) ? (assignment.blockedReasons || assignment.blocked_reasons) : [];
      return `<li><strong>${escapeText(assignment.assignmentId || assignment.assignment_id || 'assignment')}</strong>: <span class="${statusClass(assignment.status)}">${escapeText(assignment.status || 'unknown')}</span> · ${escapeText(assignment.workerId || assignment.worker_id || 'worker')} · ${escapeText(assignment.assignedLane || assignment.assigned_lane || 'lane')}<br><small>${escapeText(target.targetType || target.target_type || 'target')} ${escapeText(target.targetId || target.target_id || '')} · maxRuns ${escapeText(budget.maxRuns ?? budget.max_runs ?? '?')} · timeout ${escapeText(assignment.timeoutMs ?? assignment.timeout_ms ?? '?')}ms · cleanup ${escapeText(cleanup.mode || 'unknown')} · ${escapeText(assignment.outputRoot || assignment.output_root || 'no output root')} · ${escapeText(blocked.join(' · ') || plan.planId || plan.plan_id || 'bounded local assignment')}</small></li>`;
    }).join('') || '<li>No parseable QA worker assignments are available.</li>';
    return `<div class="field-grid">${rows}</div>
      <h4>Assignments</h4><ul class="run-meta-list">${assignmentRows}</ul>
      ${renderRefLinks('QA worker assignment refs', refs, run)}
      <p class="run-meta">${escapeText(model.boundary || 'Read-only QA worker assignment evidence; dashboard surfaces do not spawn workers, execute commands, write trusted state, auto-fix, auto-apply, or auto-merge.')}</p>`;
  }

  function renderQaAgentWorkQueues(run = {}) {
    const model = run.qa_agent_work_queues || run.qaAgentWorkQueues || {};
    if (!model.present) {
      return `<p class="empty-state">${escapeText(model.empty_state || 'No QA agent work queues are available for this run.')}</p>`;
    }
    const refs = Array.isArray(model.evidence_refs || model.evidenceRefs) ? (model.evidence_refs || model.evidenceRefs) : [];
    const queues = Array.isArray(model.queues) ? model.queues : [];
    const rows = [
      ['Status', model.status || 'unknown'],
      ['Queues/items', `${model.queue_count ?? model.queueCount ?? queues.length}/${model.item_count ?? model.itemCount ?? 0}`],
      ['Passed/failed', `${model.passed_count ?? model.passedCount ?? 0}/${model.failed_count ?? model.failedCount ?? 0}`],
      ['Deferred/blocked', `${model.deferred_count ?? model.deferredCount ?? 0}/${model.blocked_count ?? model.blockedCount ?? 0}`],
      ['Flaky/needs-rerun', `${model.flaky_count ?? model.flakyCount ?? 0}/${model.needs_rerun_count ?? model.needsRerunCount ?? 0}`],
      ['Malformed', model.malformed_count ?? model.malformedCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br><span class="${label === 'Status' ? statusClass(value) : ''}">${escapeText(value)}</span></div>`).join('');
    const itemRows = queues.flatMap((queue) => Array.isArray(queue.items) ? queue.items.map((item) => ({ queue, item })) : []).slice(0, 12).map(({ queue, item }) => {
      const target = item.scenarioTarget || item.scenario_target || {};
      const risk = item.riskArea || item.risk_area || {};
      const command = item.runCommandContext || item.run_command_context || {};
      const expected = Array.isArray(item.expectedEvidence || item.expected_evidence) ? (item.expectedEvidence || item.expected_evidence) : [];
      const runRefs = Array.isArray(item.runEvidenceRefs || item.run_evidence_refs) ? (item.runEvidenceRefs || item.run_evidence_refs) : [];
      const evaluatorRefs = Array.isArray(item.evaluatorEvidenceRefs || item.evaluator_evidence_refs) ? (item.evaluatorEvidenceRefs || item.evaluator_evidence_refs) : [];
      const blocked = Array.isArray(item.blockedReasons || item.blocked_reasons) ? (item.blockedReasons || item.blocked_reasons) : [];
      const stale = Array.isArray(item.staleRunRefs || item.stale_run_refs) ? (item.staleRunRefs || item.stale_run_refs) : [];
      const review = item.reviewGateRef || item.review_gate_ref || {};
      return `<li><strong>${escapeText(item.queueItemId || item.queue_item_id || 'qa item')}</strong>: <span class="${statusClass(item.status)}">${escapeText(item.status || 'unknown')}</span> · ${escapeText(item.priority || 'priority')} · ${escapeText(item.assignedRole || item.assigned_role || 'role')}<br><small>queue ${escapeText(queue.queueId || queue.queue_id || 'queue')} · scenario ${escapeText(target.scenarioId || target.scenario_id || 'scenario')} · risk ${escapeText(risk.riskId || risk.risk_id || 'risk')} · expected ${escapeText(expected.length)} · run/evaluator refs ${escapeText(runRefs.length)}/${escapeText(evaluatorRefs.length)} · review ${escapeText(review.path || review.id || 'missing')}</small><br><small>Inert command text: ${escapeText(command.command || 'missing')}</small>${blocked.length || stale.length ? `<br><small>Blockers/stale: ${escapeText([...blocked, ...stale].join(' · '))}</small>` : ''}</li>`;
    }).join('') || '<li>No parseable QA queue items are available.</li>';
    return `<div class="field-grid">${rows}</div>
      <h4>QA queue items</h4><ul class="run-meta-list">${itemRows}</ul>
      ${renderRefLinks('QA queue linked refs', refs, run)}
      <p class="run-meta">${escapeText(model.boundary || 'Read-only QA agent work queues; dashboard surfaces do not execute queue commands, spawn agents, write trusted state, auto-fix, auto-apply, auto-merge, or self-approve.')}</p>`;
  }

  function renderPerformanceRegressionLanes(run = {}) {
    const model = run.performance_regression_lanes || run.performanceRegressionLanes || {};
    if (!model.present) {
      return `<p class="empty-state">${escapeText(model.empty_state || model.emptyState || 'No performance/regression lanes are available for this run.')}</p>`;
    }
    const refs = Array.isArray(model.evidence_refs || model.evidenceRefs) ? (model.evidence_refs || model.evidenceRefs) : [];
    const lanes = Array.isArray(model.lanes) ? model.lanes : [];
    const rows = [
      ['Status', model.status || 'unknown'],
      ['Lanes', model.lane_count ?? model.laneCount ?? lanes.length],
      ['Improved/unchanged', `${model.improved_count ?? model.improvedCount ?? 0}/${model.unchanged_count ?? model.unchangedCount ?? 0}`],
      ['Regressed/inconclusive', `${model.regressed_count ?? model.regressedCount ?? 0}/${model.inconclusive_count ?? model.inconclusiveCount ?? 0}`],
      ['Missing/unsupported/stale', `${model.missing_baseline_count ?? model.missingBaselineCount ?? 0}/${model.unsupported_count ?? model.unsupportedCount ?? 0}/${model.stale_count ?? model.staleCount ?? 0}`],
      ['Malformed', model.malformed_count ?? model.malformedCount ?? 0],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br><span class="${label === 'Status' ? statusClass(value) : ''}">${escapeText(value)}</span></div>`).join('');
    const laneRows = lanes.slice(0, 12).map((lane) => {
      const evidenceLinks = lane.evidenceLinks || lane.evidence_links || {};
      const metrics = Array.isArray(lane.metrics) ? lane.metrics : [];
      const thresholds = Array.isArray(lane.thresholds) ? lane.thresholds : [];
      const blocked = Array.isArray(lane.blockedReasons || lane.blocked_reasons) ? (lane.blockedReasons || lane.blocked_reasons) : [];
      const stale = Array.isArray(lane.staleRunRefs || lane.stale_run_refs) ? (lane.staleRunRefs || lane.stale_run_refs) : [];
      const browserWarnings = Array.isArray(evidenceLinks.browserMetricWarnings || evidenceLinks.browser_metric_warnings) ? (evidenceLinks.browserMetricWarnings || evidenceLinks.browser_metric_warnings) : [];
      const linkCount = ['runComparisonRefs', 'frameBudgetRefs', 'scenarioMatrixRefs', 'qaQueueRefs', 'reviewGateRefs'].reduce((count, key) => count + (Array.isArray(evidenceLinks[key]) ? evidenceLinks[key].length : 0), 0);
      return `<li><strong>${escapeText(lane.laneId || lane.lane_id || 'regression lane')}</strong>: <span class="${statusClass(lane.classification)}">${escapeText(lane.classification || 'unknown')}</span> · ${escapeText(lane.assignedRole || lane.assigned_role || 'role')}<br><small>metrics ${escapeText(metrics.length)} · thresholds ${escapeText(thresholds.length)} · linked refs ${escapeText(linkCount)} · browser warnings ${escapeText(browserWarnings.length)}</small>${blocked.length || stale.length ? `<br><small>Blockers/stale: ${escapeText([...blocked, ...stale].join(' · '))}</small>` : ''}</li>`;
    }).join('') || '<li>No parseable performance/regression lanes are available.</li>';
    return `<div class="field-grid">${rows}</div>
      <h4>Performance/regression lanes</h4><ul class="run-meta-list">${laneRows}</ul>
      ${renderRefLinks('Performance/regression linked refs', refs, run)}
      <p class="run-meta">${escapeText(model.boundary || 'Read-only performance/regression lanes; dashboard surfaces do not execute commands, spawn agents, write trusted state, promote regressions, auto-apply, auto-merge, or self-approve.')}</p>`;
  }

  function renderRuntimeInvariants(run = {}) {
    const invariants = run.runtime_invariants || run.runtimeInvariants || {};
    if (!invariants.present) {
      return `<p class="empty-state">${escapeText(invariants.empty_state || 'No runtime invariant evidence is available for this run.')}</p>`;
    }
    const refs = Array.isArray(invariants.evidence_refs || invariants.evidenceRefs) ? (invariants.evidence_refs || invariants.evidenceRefs) : [];
    const summaries = Array.isArray(invariants.summaries) ? invariants.summaries : [];
    const evidence = Array.isArray(invariants.evidence) ? invariants.evidence : [];
    const rows = [
      ['Status', invariants.status || 'unknown'],
      ['Checks', invariants.check_count ?? invariants.checkCount ?? 0],
      ['Passed', invariants.passed_count ?? invariants.passedCount ?? 0],
      ['Failed', invariants.failed_count ?? invariants.failedCount ?? 0],
      ['Unsupported/missing', `${invariants.unsupported_count ?? invariants.unsupportedCount ?? 0}/${invariants.missing_count ?? invariants.missingCount ?? 0}`],
      ['Malformed/stale', `${invariants.malformed_count ?? invariants.malformedCount ?? 0}/${invariants.stale_count ?? invariants.staleCount ?? 0}`],
    ].map(([label, value]) => `<div><strong>${escapeText(label)}</strong><br><span class="${label === 'Status' ? statusClass(value) : ''}">${escapeText(value)}</span></div>`).join('');
    const summaryRows = summaries.length
      ? summaries.slice(0, 8).map((summary) => `<li><strong>${escapeText(summary.modelId || summary.model_id || 'runtime invariant model')}</strong>: ${escapeText(summary.checkCount ?? summary.check_count ?? 0)} check(s), ${escapeText(summary.failedCount ?? summary.failed_count ?? 0)} failed, ${escapeText(summary.malformedCount ?? summary.malformed_count ?? 0)} malformed, ${escapeText(summary.staleCount ?? summary.stale_count ?? 0)} stale<br><small>${escapeText(summary.runId || summary.run_id || 'unknown run')}${summary.scenarioId || summary.scenario_id ? ` · ${escapeText(summary.scenarioId || summary.scenario_id)}` : ''}</small></li>`).join('')
      : '<li>No parseable runtime invariant summaries are available.</li>';
    const checkRows = evidence.flatMap((item) => Array.isArray(item.checks) ? item.checks : []).slice(0, 12).map((check) => {
      const refs = Array.isArray(check.evidenceRefs || check.evidence_refs) ? (check.evidenceRefs || check.evidence_refs) : [];
      return `<li><strong>${escapeText(check.invariantId || check.invariant_id || 'invariant')}</strong>: <span class="${statusClass(check.status)}">${escapeText(check.status || 'unknown')}</span> · ${escapeText(check.invariantType || check.invariant_type || 'unknown type')} · ${escapeText(check.targetPath || check.target_path || 'unknown target')}<br><small>${escapeText(check.message || refs.join(' · ') || 'linked to scenario/runtime evidence')}</small></li>`;
    }).join('') || '<li>No parsed runtime invariant checks are available.</li>';
    return `<div class="field-grid">${rows}</div>
      <h4>Invariant evidence summaries</h4><ul class="run-meta-list">${summaryRows}</ul>
      <h4>Checks</h4><ul class="run-meta-list">${checkRows}</ul>
      ${renderRefLinks('Runtime invariant evidence refs', refs, run)}
      <p class="run-meta">${escapeText(invariants.boundary || 'Read-only runtime invariant evidence; dashboard surfaces do not mutate source, execute commands, auto-fix, auto-apply, or auto-merge.')}</p>`;
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

  function comparisonGateDeltas(artifact) {
    const value = artifact?.value || {};
    const fourGate = value.fourGate || value.four_gate || artifact?.fourGate || artifact?.four_gate || null;
    if (Array.isArray(fourGate?.gates)) {
      return fourGate.gates.map((gate) => ({
        gate: gate.gate || 'unknown',
        before: gate.before || gate.beforeStatus || gate.before_status || 'unknown',
        after: gate.after || gate.afterStatus || gate.after_status || 'unknown',
        transition: gate.transition || 'unknown',
        evidenceRefs: gate.evidenceRefs || gate.evidence_refs || [],
        comparability: gate.comparability || gate.comparabilityState || gate.comparability_state || 'unknown',
      }));
    }
    const deltas = value.fourGateDeltas || value.four_gate_deltas || artifact?.fourGateDeltas || artifact?.four_gate_deltas || [];
    return Array.isArray(deltas) ? deltas.map((delta) => ({
      gate: delta.gate || 'unknown',
      before: delta.beforeStatus || delta.before_status || delta.before || 'unknown',
      after: delta.afterStatus || delta.after_status || delta.after || 'unknown',
      transition: delta.transition || 'unknown',
      evidenceRefs: [...(delta.beforeEvidenceRefs || delta.before_evidence_refs || []), ...(delta.afterEvidenceRefs || delta.after_evidence_refs || [])],
      comparability: delta.comparabilityState || delta.comparability_state || 'unknown',
    })) : [];
  }

  function renderFourGateDeltaSummary(artifact, run) {
    const gates = comparisonGateDeltas(artifact);
    if (!gates.length) {
      return '<section class="panel"><h5>Four-gate before/after delta</h5><p class="empty-state compact">No four-gate rerun delta was exported for this comparison.</p></section>';
    }
    const rows = gates.map((gate) => {
      const refs = Array.isArray(gate.evidenceRefs) ? gate.evidenceRefs.filter(Boolean) : [];
      return `<li><strong>${escapeText(gate.gate)}</strong>: <span class="${statusClass(gate.transition)}">${escapeText(gate.before)} → ${escapeText(gate.after)} (${escapeText(gate.transition)})</span><br><small>comparability ${escapeText(gate.comparability)}</small>${renderComparisonRefLinks('Gate evidence refs', refs, run)}</li>`;
    }).join('');
    return `<section class="panel"><h5>Four-gate before/after delta</h5><p class="run-meta">Read-only exported JSON only; no accept/apply/write/merge controls or browser-side trusted comparison logic.</p><ul class="run-meta-list">${rows}</ul></section>`;
  }

  function renderSceneTreeInspector(run = {}) {
    const model = run.scene_tree_inspector || run.sceneTreeInspector || null;
    if (!model || model.present === false) {
      const emptyState = model?.empty_state || model?.emptyState || 'No scene tree read model is exported for this run.';
      return `<p class="empty-state">${escapeText(emptyState)}</p><p class="run-meta">Read-only inspection only; the dashboard does not write trusted state, edit scenes, expose selection controls, apply, or merge.</p>`;
    }
    const scenes = Array.isArray(model.scenes) ? model.scenes : [];
    const selectedScene = model.selected_scene || model.selectedScene || '';
    const nodes = Array.isArray(model.nodes) ? model.nodes : [];
    const sceneRows = scenes.length
      ? scenes.map((scene) => {
          const isSelected = scene.selected === true || scene.id === selectedScene;
          return `<li><strong>${escapeText(scene.name || scene.id || 'unknown scene')}</strong>: <span class="${statusClass(isSelected ? 'pass' : 'neutral')}">${isSelected ? 'selected' : 'not selected'}</span><br><small>${escapeText(scene.id || 'no id')}</small></li>`;
        }).join('')
      : '<li>No scenes exported.</li>';
    const nodeRows = nodes.length
      ? nodes.map((node) => {
          const parentId = node.parent_id != null ? node.parent_id : node.parentId;
          const parentText = parentId != null ? `parent ${escapeText(String(parentId))}` : 'root node';
          const stateLabel = node.authored === true ? 'authored state' : 'runtime state';
          const summary = node.component_summary || node.componentSummary || 'no components';
          const warnings = Array.isArray(node.warnings) ? node.warnings.filter(Boolean) : [];
          const warningRows = warnings.length
            ? `<ul class="run-meta-list">${warnings.map((warning) => `<li><span class="${statusClass('fail')}">${escapeText(warning)}</span></li>`).join('')}</ul>`
            : '';
          return `<li><strong>${escapeText(node.name || node.id || 'unknown node')}</strong>: <span class="${statusClass(node.authored === true ? 'pass' : 'neutral')}">${escapeText(stateLabel)}</span><br><small>${escapeText(node.type || 'unknown type')} · ${escapeText(String(node.id ?? 'no id'))} · ${parentText} · ${escapeText(summary)}</small>${warningRows}</li>`;
        }).join('')
      : '<li>No scene nodes exported.</li>';
    return `<p class="run-meta">Read-only scene tree inspection from exported JSON. Selection is presentation-only; the dashboard does not edit, apply, merge, or write trusted scene state.</p><h5>Scenes (selection is read-only)</h5><ul class="run-meta-list">${sceneRows}</ul><h5>Node hierarchy (authored vs runtime state)</h5><ul class="run-meta-list">${nodeRows}</ul>`;
  }

  function studioCommandRegistry() {
    return [
      { id: 'open-scene', label: 'Open scene', kind: 'navigation', description: 'Navigate the dashboard to a scene view.' },
      { id: 'focus-entity', label: 'Focus entity', kind: 'navigation', description: 'Scroll the inspector to a selected entity.' },
      { id: 'show-evidence', label: 'Show evidence', kind: 'navigation', description: 'Jump to the read-only evidence pane.' },
      { id: 'show-export-status', label: 'Show export status', kind: 'navigation', description: 'Reveal the exported run summary section.' },
      { id: 'show-plugin-status', label: 'Show plugin status', kind: 'navigation', description: 'Reveal the read-only plugin registry surface.' },
      { id: 'create-draft-edit', label: 'Create draft edit', kind: 'draft', description: 'Stage a local draft edit for offline review (no trusted write).' },
      { id: 'discard-draft', label: 'Discard draft', kind: 'draft', description: 'Clear the local draft edit without persisting.' },
      { id: 'copy-evidence-path', label: 'Copy evidence path', kind: 'copy', description: 'Copy a read-only evidence reference to the clipboard.' },
      { id: 'reset-layout', label: 'Reset layout', kind: 'navigation', description: 'Restore the default dashboard panel layout (view only).' },
    ];
  }

  const STUDIO_COMMAND_BLOCKLIST = [
    'run-shell', 'apply-source', 'auto-merge', 'publish', 'deploy', 'sign', 'upload',
    'install-plugin', 'mutate-dependency', 'edit-ci', 'execute-plugin', 'network-install', 'network-update',
  ];

  function isBlockedStudioCommand(id) {
    return STUDIO_COMMAND_BLOCKLIST.includes(String(id || ''));
  }

  function filterStudioCommands(query) {
    const needle = String(query || '').trim().toLowerCase();
    const commands = studioCommandRegistry();
    if (!needle) return commands;
    return commands.filter((command) => `${command.id} ${command.label} ${command.description}`.toLowerCase().includes(needle));
  }

  function resolveStudioCommand(id) {
    const key = String(id || '');
    if (isBlockedStudioCommand(key)) {
      return { allowed: false, reason: `Command "${key}" is privileged and is refused by the read-only command palette.` };
    }
    const command = studioCommandRegistry().find((item) => item.id === key);
    if (!command) {
      return { allowed: false, reason: `Command "${key}" is unknown and is refused by the read-only command palette.` };
    }
    return { allowed: true, kind: command.kind, command };
  }

  function renderStudioCommandPaletteSurface(run) {
    const commands = filterStudioCommands('');
    const options = commands.map((command) => `<li role="option" aria-label="${escapeText(command.label)}" data-command-id="${escapeText(command.id)}"><strong>${escapeText(command.label)}</strong> <span class="run-meta">(${escapeText(command.kind)})</span><br><small>${escapeText(command.description)}</small></li>`).join('');
    return `<section class="panel"><h3>Command Palette</h3><p class="run-meta">Read-only. The palette only offers navigation, local draft, and copy commands. It does not run shell, apply source, merge, publish, deploy, install, or execute plugins, mutate dependencies/CI, or write trusted state.</p><ul class="run-meta-list" role="listbox" aria-label="Studio command palette">${options}</ul></section>`;
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
        ${renderFourGateDeltaSummary(artifact, run)}
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

  function normalizeProductionEvidenceBundles(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
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

  function renderProductionEvidenceBundles(value = null) {
    const bundles = normalizeProductionEvidenceBundles(value);
    if (!bundles.length) {
      return '<section class="panel production-evidence-bundles"><h3>Production evidence bundle</h3><p class="empty-state">No production evidence bundle is attached to this dashboard data.</p><p class="run-meta">Read-only dashboard surface. The browser cannot spawn agents, execute commands, apply changes, auto-merge, self-approve, or write trusted state.</p></section>';
    }
    const cards = bundles.map((bundle) => {
      const laneOutputs = Array.isArray(bundle.laneOutputs) ? bundle.laneOutputs : [];
      const missing = Array.isArray(bundle.missingRefs) ? bundle.missingRefs : [];
      const stale = Array.isArray(bundle.staleRefs) ? bundle.staleRefs : [];
      const blocked = Array.isArray(bundle.blockedReasons) ? bundle.blockedReasons : [];
      const malformed = Array.isArray(bundle.malformedReasons) ? bundle.malformedReasons : [];
      const conflicts = Array.isArray(bundle.unresolvedConflicts) ? bundle.unresolvedConflicts : [];
      const missingReviews = Array.isArray(bundle.missingReviews) ? bundle.missingReviews : [];
      const forbidden = Array.isArray(bundle.forbiddenActions) ? bundle.forbiddenActions : [];
      const generatedRoots = Array.isArray(bundle.generatedState?.roots) ? bundle.generatedState.roots : [];
      const laneRows = laneOutputs.length
        ? laneOutputs.map((lane) => `<li>${escapeText(lane.lane || lane.id || 'lane')} · <span class="${statusClass(lane.status || 'unknown')}">${escapeText(lane.status || 'unknown')}</span>${Array.isArray(lane.blockedReasons) && lane.blockedReasons.length ? ` · ${escapeText(lane.blockedReasons.join(' · '))}` : ''}</li>`).join('')
        : '<li>No lane outputs recorded.</li>';
      const blockerText = [
        ...blocked,
        ...conflicts.map((conflict) => `${conflict.id || 'conflict'}:${conflict.summary || 'unresolved conflict'}`),
        ...missingReviews.map((review) => `${review.id || 'missing-review'}:${review.requiredReviewerRole || 'reviewer'}`),
      ];
      return `<article class="artifact production-evidence-bundle">
        <h4>${escapeText(bundle.bundleId || 'unknown-production-bundle')}</h4>
        <div class="run-meta"><span class="${statusClass(bundle.status || 'unknown')}">${escapeText(bundle.status || 'unknown')}</span> · refs:${productionBundleRefCount(bundle)} · lanes:${laneOutputs.length}</div>
        <div class="run-meta">Milestone: ${escapeText(bundle.milestone || 'unrecorded')}</div>
        ${missing.length ? `<div class="artifact-warning">Missing refs: ${escapeText(missing.join(' · '))}</div>` : '<div class="run-meta">No missing refs reported.</div>'}
        ${stale.length ? `<div class="artifact-warning">Stale refs: ${escapeText(stale.join(' · '))}</div>` : '<div class="run-meta">No stale refs reported.</div>'}
        ${blockerText.length ? `<div class="artifact-warning">Blockers: ${escapeText(blockerText.join(' · '))}</div>` : '<div class="run-meta">No blockers reported.</div>'}
        ${malformed.length ? `<div class="artifact-warning">Malformed: ${escapeText(malformed.join(' · '))}</div>` : ''}
        <ul>${laneRows}</ul>
        <div class="run-meta">Generated roots: ${escapeText(generatedRoots.join(' · ') || 'none')}</div>
        <div class="run-meta">Forbidden actions: ${escapeText(forbidden.join(' · ') || 'none')}</div>
        <p class="run-meta">${escapeText(bundle.boundary || 'Inert local audit artifact; dashboard is read-only.')}</p>
      </article>`;
    }).join('');
    return `<section class="panel production-evidence-bundles"><h3>Production evidence bundle</h3>
      <p class="run-meta">Read-only production evidence bundle. This dashboard displays state, blockers, missing/stale refs, lane outputs, and generated-state boundaries only; it does not spawn agents, execute commands, apply changes, auto-merge, self-approve, or write trusted state.</p>
      <div class="artifact-grid">${cards}</div>
    </section>`;
  }


  function pipelineInspectionModel(run = {}) {
    return run?.studio_multi_agent_pipeline_inspection || run?.studioMultiAgentPipelineInspection || run?.multi_agent_pipeline_inspection || run?.multiAgentPipelineInspection || null;
  }

  function renderStudioMultiAgentPipelineInspection(value = null) {
    const model = value && typeof value === 'object' ? value : null;
    if (!model) {
      return '<section class="panel studio-multi-agent-pipeline"><h3>Studio multi-agent pipeline inspection</h3><p class="empty-state">No Studio multi-agent pipeline inspection read model is attached to this dashboard data.</p></section>';
    }
    const sections = Array.isArray(model.sections) ? model.sections : [];
    const malformed = Array.isArray(model.malformedReasons) ? model.malformedReasons : [];
    const sectionRows = sections.length ? sections.map((section) => {
      const blockers = Array.isArray(section.blockers) ? section.blockers : [];
      const reasons = Array.isArray(section.malformedReasons) ? section.malformedReasons : [];
      return `<li><strong>${escapeText(section.label || section.id || 'pipeline section')}</strong> · <span class="${statusClass(section.status || 'unknown')}">${escapeText(section.status || 'unknown')}</span> · items ${escapeText(section.itemCount ?? 0)}<br><span class="run-meta">ID: ${escapeText(section.id || 'unknown')}</span>${blockers.length ? `<br><span class="artifact-warning">Blockers: ${escapeText(blockers.join(' · '))}</span>` : ''}${reasons.length ? `<br><span class="artifact-warning">Malformed: ${escapeText(reasons.join(' · '))}</span>` : ''}</li>`;
    }).join('') : '<li class="artifact-warning">Missing or malformed pipeline inspection sections.</li>';
    return `<section class="panel studio-multi-agent-pipeline"><h3>Studio multi-agent pipeline inspection</h3>
      <p class="run-meta">Read-only pipeline inspection. The dashboard displays section status, blockers, and malformed reasons only; it does not execute commands, spawn agents, write trusted browser state, bridge to local commands, use cloud orchestration, auto-apply, auto-merge, or self-approve.</p>
      <div class="run-meta">Schema: ${escapeText(model.schemaVersion || 'unknown')} · status <span class="${statusClass(model.status || 'unknown')}">${escapeText(model.status || 'unknown')}</span> · sections ${escapeText(sections.length)}</div>
      ${malformed.length ? `<div class="artifact-warning">Malformed input: ${escapeText(malformed.join(' · '))}</div>` : '<div class="run-meta">No malformed input reported.</div>'}
      <ul>${sectionRows}</ul>
      <p class="run-meta">${escapeText(model.boundary || 'Pipeline inspection display is read-only and command-inert.')}</p>
    </section>`;
  }


  function normalizeReviewCriticGates(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderReviewCriticGates(value = null) {
    const gates = normalizeReviewCriticGates(value);
    if (!gates.length) {
      return '<section class="panel review-critic-gates"><h3>Review/critic gate</h3><p class="empty-state">No review/critic gate is attached to this dashboard data.</p><p class="run-meta">Read-only dashboard surface. The browser cannot promote outputs, execute commands, spawn agents, apply changes, auto-merge, self-approve, or write trusted state.</p></section>';
    }
    const cards = gates.map((gate) => {
      const stateSnapshots = Array.isArray(gate.stateSnapshotRefs) ? gate.stateSnapshotRefs : Array.isArray(gate.stateSnapshotRefPaths) ? gate.stateSnapshotRefPaths : [];
      const qaRefs = Array.isArray(gate.qaEvidenceRefs) ? gate.qaEvidenceRefs : Array.isArray(gate.qaEvidenceRefPaths) ? gate.qaEvidenceRefPaths : [];
      const regressionRefs = Array.isArray(gate.regressionEvidenceRefs) ? gate.regressionEvidenceRefs : Array.isArray(gate.regressionEvidenceRefPaths) ? gate.regressionEvidenceRefPaths : [];
      const reviewed = Array.isArray(gate.evidenceReviewed) ? gate.evidenceReviewed : Array.isArray(gate.evidenceReviewedRefPaths) ? gate.evidenceReviewedRefPaths : [];
      const blockers = Array.isArray(gate.blockedReasons) ? gate.blockedReasons : Array.isArray(gate.blockers) ? gate.blockers : [];
      const stale = Array.isArray(gate.staleStateIndicators) ? gate.staleStateIndicators : [];
      const fixes = Array.isArray(gate.requiredFixes) ? gate.requiredFixes : [];
      const malformed = Array.isArray(gate.malformedReasons) ? gate.malformedReasons : [];
      const forbidden = Array.isArray(gate.forbiddenActions) ? gate.forbiddenActions : [];
      const refText = (refs) => refs.map((ref) => typeof ref === 'string' ? ref : ref.path || ref.id || 'ref').join(' · ') || 'missing';
      const implementer = gate.implementer?.actorId || gate.implementerActorId || 'unknown-implementer';
      const reviewer = gate.reviewer?.actorId || gate.reviewerActorId || 'unknown-reviewer';
      const critic = gate.critic?.actorId || gate.criticActorId || 'unknown-critic';
      const workPackage = gate.workPackageRef?.path || gate.workPackageRefPath || 'missing';
      const handoff = gate.handoffRef?.path || gate.handoffRefPath || 'missing';
      const ledger = gate.decisionLedgerRef?.path || gate.decisionLedgerRefPath || 'missing';
      return `<article class="artifact review-critic-gate">
        <h4>${escapeText(gate.gateId || 'unknown-review-critic-gate')}</h4>
        <p class="run-meta">Read-only review/critic gate. The dashboard displays linkage and blocker evidence only; it does not execute commands, spawn agents, apply changes, promote outputs, write trusted browser state, auto-merge, or self-approve.</p>
        <div class="run-meta">Schema: ${escapeText(gate.schemaVersion || 'unknown')} · task ${escapeText(gate.taskId || 'unknown')} · decision <span class="${statusClass(gate.decision || 'unknown')}">${escapeText(gate.decision || 'unknown')}</span> · recommendation ${escapeText(gate.promotionRecommendation || 'unknown')}</div>
        <div class="run-meta">Actors: implementer ${escapeText(implementer)} · reviewer ${escapeText(reviewer)} · critic ${escapeText(critic)}</div>
        ${blockers.length ? `<div class="artifact-warning">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="run-meta">No blockers reported.</div>'}
        ${stale.length ? `<div class="artifact-warning">Stale state: ${escapeText(stale.join(' · '))}</div>` : '<div class="run-meta">No stale state indicators reported.</div>'}
        ${fixes.length ? `<div class="artifact-warning">Required fixes: ${escapeText(fixes.join(' · '))}</div>` : ''}
        ${malformed.length ? `<div class="artifact-warning">Malformed: ${escapeText(malformed.join(' · '))}</div>` : ''}
        <div class="run-meta">Work package: ${escapeText(workPackage)}</div>
        <div class="run-meta">Handoff: ${escapeText(handoff)}</div>
        <div class="run-meta">State snapshots: ${escapeText(refText(stateSnapshots))}</div>
        <div class="run-meta">QA evidence: ${escapeText(refText(qaRefs))}</div>
        <div class="run-meta">Regression evidence: ${escapeText(refText(regressionRefs))}</div>
        <div class="run-meta">Decision ledger: ${escapeText(ledger)}</div>
        <div class="run-meta">Evidence reviewed: ${escapeText(refText(reviewed))}</div>
        <div class="run-meta">Forbidden actions: ${escapeText(forbidden.join(' · ') || 'missing')}</div>
        <p class="run-meta">${escapeText(gate.boundary || 'Review/critic gate display is read-only and command-inert.')}</p>
      </article>`;
    }).join('');
    return `<section class="panel review-critic-gates"><h3>Review/critic gate</h3><div class="artifact-grid">${cards}</div></section>`;
  }

  function normalizeProductionTaskBoards(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderProductionTaskBoards(value = null) {
    const boards = normalizeProductionTaskBoards(value);
    if (!boards.length) {
      return '<section class="panel production-task-boards"><h3>Production task board</h3><p class="empty-state">No production task board is attached to this dashboard data.</p></section>';
    }
    const cards = boards.map((board) => {
      const tasks = Array.isArray(board.tasks) ? board.tasks : [];
      const forbidden = Array.isArray(board.forbiddenActions) ? board.forbiddenActions : [];
      const guardrails = Array.isArray(board.guardrails) ? board.guardrails : [];
      const statusCounts = tasks.reduce((counts, task) => {
        const status = task?.status || 'unknown';
        counts[status] = (counts[status] || 0) + 1;
        return counts;
      }, {});
      const statusText = Object.keys(statusCounts).sort().map((status) => `${status}:${statusCounts[status]}`).join(' · ') || 'none';
      const blockers = tasks.flatMap((task) => Array.isArray(task?.blockedReasons) ? task.blockedReasons.map((reason) => `${task.id || 'task'}: ${reason}`) : []);
      const taskRows = tasks.length ? tasks.map((task) => {
        const targets = Array.isArray(task.targetArtifacts) ? task.targetArtifacts : [];
        const evidence = Array.isArray(task.requiredEvidence) ? task.requiredEvidence : [];
        return `<li><strong>${escapeText(task.id || 'task')}</strong> · ${escapeText(task.role || 'unknown-role')} · ${escapeText(task.ownerAgent || 'unknown-owner')} · <span class="${statusClass(task.status || 'unknown')}">${escapeText(task.status || 'unknown')}</span><br><span class="run-meta">Targets: ${escapeText(targets.map((target) => target.path || target.id || 'target').join(' · ') || 'missing')} · Evidence: ${escapeText(evidence.join(' · ') || 'missing')}</span></li>`;
      }).join('') : '<li class="artifact-warning">Missing or malformed tasks list.</li>';
      return `<article class="artifact production-task-board">
        <h4>${escapeText(board.boardId || 'unknown-board')}</h4>
        <p class="run-meta">Read-only production task board. The dashboard does not spawn agents, execute commands, apply changes, write trusted browser state, auto-merge, or self-approve.</p>
        <div class="run-meta">Schema: ${escapeText(board.schemaVersion || 'unknown')} · tasks ${escapeText(tasks.length)} · statuses ${escapeText(statusText)}</div>
        ${blockers.length ? `<div class="artifact-warning">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="run-meta">No blockers reported.</div>'}
        <div class="run-meta">Forbidden actions: ${escapeText(forbidden.join(' · ') || 'missing')}</div>
        <div class="run-meta">Guardrails: ${escapeText(guardrails.join(' · ') || 'missing')}</div>
        <ul>${taskRows}</ul>
        <p class="run-meta">${escapeText(board.boundary || 'Production task board display is read-only.')}</p>
      </article>`;
    }).join('');
    return `<section class="panel production-task-boards"><h3>Production task board</h3><div class="artifact-grid">${cards}</div></section>`;
  }

  function normalizeOwnershipPolicies(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderOwnershipPolicies(value = null) {
    const policies = normalizeOwnershipPolicies(value);
    if (!policies.length) {
      return '<section class="panel ownership-policies"><h3>Ownership policy</h3><p class="empty-state">No file/artifact ownership policy is attached to this dashboard data.</p></section>';
    }
    const cards = policies.map((policy) => {
      const entries = Array.isArray(policy.entries) ? policy.entries : [];
      const forbidden = Array.isArray(policy.forbiddenActions) ? policy.forbiddenActions : [];
      const guardrails = Array.isArray(policy.guardrails) ? policy.guardrails : [];
      const blockers = entries.flatMap((entry) => Array.isArray(entry?.blockedReasons) ? entry.blockedReasons.map((reason) => `${entry.id || 'entry'}: ${reason}`) : []);
      const escalations = entries.filter((entry) => entry?.escalation).map((entry) => `${entry.id || 'entry'}: ${entry.escalation.requiredDecision || entry.escalation.required_decision || 'decision required'}`);
      const entryRows = entries.length ? entries.map((entry) => {
        const target = entry.target || {};
        const evidence = Array.isArray(entry.evidenceRefs) ? entry.evidenceRefs : [];
        const workPackages = Array.isArray(entry.workPackageRefs) ? entry.workPackageRefs : [];
        return `<li><strong>${escapeText(entry.id || 'entry')}</strong> · ${escapeText(entry.role || 'unknown-role')} · ${escapeText(entry.ownerAgent || 'unknown-owner')} · <span class="${statusClass(entry.state || 'unknown')}">${escapeText(entry.state || 'unknown')}</span><br><span class="run-meta">Mode: ${escapeText(entry.mode || 'unknown')} · Target: ${escapeText(target.kind || 'unknown')}:${escapeText(target.path || target.id || 'missing')} · Work packages: ${escapeText(workPackages.join(' · ') || 'missing')} · Evidence: ${escapeText(evidence.map((ref) => ref.path || ref.id || 'ref').join(' · ') || 'missing')}</span></li>`;
      }).join('') : '<li class="artifact-warning">Missing or malformed ownership entries list.</li>';
      return `<article class="artifact ownership-policy">
        <h4>${escapeText(policy.policyId || 'unknown-policy')}</h4>
        <p class="run-meta">Read-only ownership policy. The dashboard reports blockers, deferred states, and escalation requirements only; it does not lock files, spawn agents, execute commands, apply changes, write trusted browser state, auto-merge, or self-approve.</p>
        <div class="run-meta">Schema: ${escapeText(policy.schemaVersion || 'unknown')} · entries ${escapeText(entries.length)} · milestone ${escapeText(policy.milestone || 'unknown')}</div>
        ${blockers.length ? `<div class="artifact-warning">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="run-meta">No blockers reported.</div>'}
        ${escalations.length ? `<div class="artifact-warning">Escalations: ${escapeText(escalations.join(' · '))}</div>` : '<div class="run-meta">No escalations reported.</div>'}
        <div class="run-meta">Forbidden actions: ${escapeText(forbidden.join(' · ') || 'missing')}</div>
        <div class="run-meta">Guardrails: ${escapeText(guardrails.join(' · ') || 'missing')}</div>
        <ul>${entryRows}</ul>
        <p class="run-meta">${escapeText(policy.boundary || 'Ownership policy display is read-only.')}</p>
      </article>`;
    }).join('');
    return `<section class="panel ownership-policies"><h3>Ownership policy</h3><div class="artifact-grid">${cards}</div></section>`;
  }

  function normalizeAgentWorkPackages(value = null) {
    if (Array.isArray(value)) return value;
    if (value && typeof value === 'object') return [value];
    return [];
  }

  function renderAgentWorkPackages(value = null) {
    const packages = normalizeAgentWorkPackages(value);
    if (!packages.length) {
      return '<section class="panel agent-work-packages"><h3>Agent work package</h3><p class="empty-state">No agent work package is attached to this dashboard data.</p></section>';
    }
    const cards = packages.map((pkg) => {
      const status = pkg.status || 'unknown';
      const allowed = Array.isArray(pkg.allowedArtifacts) ? pkg.allowedArtifacts : [];
      const criteria = Array.isArray(pkg.acceptanceCriteria) ? pkg.acceptanceCriteria : [];
      const commands = Array.isArray(pkg.verificationCommands) ? pkg.verificationCommands : [];
      const expected = Array.isArray(pkg.expectedEvidence) ? pkg.expectedEvidence : [];
      const ownership = Array.isArray(pkg.ownershipRefs) ? pkg.ownershipRefs : [];
      const forbidden = Array.isArray(pkg.forbiddenActions) ? pkg.forbiddenActions : [];
      const blockers = Array.isArray(pkg.blockedReasons) ? pkg.blockedReasons : Array.isArray(pkg.blockers) ? pkg.blockers : [];
      const malformed = Array.isArray(pkg.malformedReasons) ? pkg.malformedReasons : [];
      const criterionRows = criteria.length ? criteria.map((criterion) => `<li><strong>${escapeText(criterion.id || 'criterion')}</strong>: ${escapeText(criterion.description || 'missing description')}<br><span class="run-meta">Evidence: ${escapeText((Array.isArray(criterion.evidenceRefs) ? criterion.evidenceRefs : []).map((ref) => ref.path || ref.id || 'ref').join(' · ') || 'missing')}</span></li>`).join('') : '<li class="artifact-warning">Missing or malformed acceptance criteria.</li>';
      const commandText = commands.map((command) => command.command || '').filter(Boolean).join(' · ') || 'missing';
      const ownershipText = ownership.map((ref) => typeof ref === 'string' ? ref : ref.path || ref.id || 'ref').join(' · ') || 'missing';
      const expectedText = expected.map((ref) => typeof ref === 'string' ? ref : ref.path || ref.id || 'ref').join(' · ') || 'missing';
      const allowedText = allowed.map((ref) => ref.path || ref.id || 'artifact').join(' · ') || 'missing';
      const handoff = pkg.handoffTarget?.path || pkg.handoffTargetPath || pkg.handoffTarget?.id || 'missing';
      return `<article class="artifact agent-work-package">
        <h4>${escapeText(pkg.workPackageId || 'unknown-work-package')}</h4>
        <p class="run-meta">Read-only agent work package. The dashboard displays status, blockers, expected evidence, ownership refs, and handoff target only; it does not execute commands, spawn hidden agents, apply changes, write trusted browser state, auto-merge, or self-approve.</p>
        <div class="run-meta">Schema: ${escapeText(pkg.schemaVersion || 'unknown')} · task ${escapeText(pkg.taskId || 'unknown')} · role ${escapeText(pkg.role || 'unknown-role')} · <span class="${statusClass(status)}">${escapeText(status)}</span></div>
        ${blockers.length ? `<div class="artifact-warning">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="run-meta">No blockers reported.</div>'}
        ${malformed.length ? `<div class="artifact-warning">Malformed: ${escapeText(malformed.join(' · '))}</div>` : ''}
        <div class="run-meta">Allowed artifacts: ${escapeText(allowedText)}</div>
        <div class="run-meta">Expected evidence: ${escapeText(expectedText)}</div>
        <div class="run-meta">Ownership refs: ${escapeText(ownershipText)}</div>
        <div class="run-meta">Handoff target: ${escapeText(handoff)}</div>
        <div class="run-meta">Inert verification command text: ${escapeText(commandText)}</div>
        <div class="run-meta">Forbidden actions: ${escapeText(forbidden.join(' · ') || 'missing')}</div>
        <h5>Acceptance criteria</h5><ul>${criterionRows}</ul>
        <p class="run-meta">${escapeText(pkg.boundary || 'Agent work package display is read-only and command-inert.')}</p>
      </article>`;
    }).join('');
    return `<section class="panel agent-work-packages"><h3>Agent work package</h3><div class="artifact-grid">${cards}</div></section>`;
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
      const title = handoff.loopId || handoff.handoffId || handoff.taskId || 'unknown-handoff';
      const nextAction = handoff.nextSafeAction || handoff.nextRecommendedAction || 'unrecorded';
      return `<article class="artifact agent-handoff">
        <h4>${escapeText(title)}</h4>
        <div class="run-meta"><span class="${statusClass(handoff.status || 'unknown')}">${escapeText(handoff.status || 'unknown')}</span> · ${isV2 ? `task ${escapeText(handoff.taskId || 'unknown-task')}` : `step ${escapeText(handoff.currentStep?.stepId || 'none')}`}</div>
        ${isV2 ? `<div class="run-meta">Roles: ${escapeText(handoff.fromRole || 'unknown')} → ${escapeText(handoff.toRole || 'unknown')}</div>` : ''}
        <div class="run-meta">Next safe action: ${escapeText(nextAction)}</div>
        ${blockers.length ? `<div class="artifact-warning">Blockers: ${escapeText(blockers.join(' · '))}</div>` : '<div class="run-meta">No blockers reported.</div>'}
        ${risks.length ? `<div class="artifact-warning">Open risks: ${escapeText(risks.map((risk) => `${risk.id || 'risk'}:${risk.severity || 'unknown'}:${risk.description || 'missing'}`).join(' · '))}</div>` : '<div class="run-meta">No open risks reported.</div>'}
        ${stale.length ? `<div class="artifact-warning">Stale state: ${escapeText(stale.map((item) => `${item.id || 'stale'}:${item.reason || 'missing'}:${item.nextAction || 'inspect'}`).join(' · '))}</div>` : '<div class="run-meta">No stale state indicators reported.</div>'}
        ${decisions.length ? `<div class="run-meta">Required decisions: ${escapeText(decisions.map((decision) => `${decision.id || 'decision'}:${decision.kind || 'unknown'}`).join(' · '))}</div>` : '<div class="run-meta">No required decisions reported.</div>'}
        <div class="run-meta">Acceptance checklist: ${escapeText(checklist.map((item) => `${item.id || 'item'}:${item.checked ? 'checked' : 'unchecked'}`).join(' · ') || 'none')}</div>
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

  function behaviorEvidenceModel(run) {
    return run?.behavior_evidence || run?.behaviorEvidence || {};
  }

  function behaviorEvidenceBundles(model) {
    return Array.isArray(model?.bundles) ? model.bundles : [];
  }

  function renderBehaviorEvidenceLifecycle(run) {
    const model = behaviorEvidenceModel(run);
    const bundles = behaviorEvidenceBundles(model);
    if (!model.present && !bundles.length) {
      return '<section class="panel behavior-evidence-lifecycle"><h3>Behavior evidence lifecycle</h3><p class="empty-state">No behavior evidence bundle is indexed for this run.</p><p class="run-meta">Read-only dashboard surface. The browser cannot execute scripts, run command bridges, auto-apply changes, or write trusted files.</p></section>';
    }
    const cards = bundles.map((bundle) => {
      const failures = Array.isArray(bundle.observed_failures || bundle.observedFailures) ? (bundle.observed_failures || bundle.observedFailures) : [];
      const hypotheses = Array.isArray(bundle.next_step_hypotheses || bundle.nextStepHypotheses) ? (bundle.next_step_hypotheses || bundle.nextStepHypotheses) : [];
      const blockers = Array.isArray(bundle.blocked_reasons || bundle.blockedReasons) ? (bundle.blocked_reasons || bundle.blockedReasons) : [];
      const refs = Array.isArray(bundle.evidence_refs || bundle.evidenceRefs) ? (bundle.evidence_refs || bundle.evidenceRefs) : [];
      const guardrails = Array.isArray(bundle.guardrails) ? bundle.guardrails : [];
      const failureRows = failures.length ? failures.map((failure) => `<li><strong>${escapeText(failure.scenario_id || failure.scenarioId || 'scenario')}</strong>: ${escapeText(failure.summary || 'missing summary')}<br><span class="run-meta">Evidence: ${escapeText(failure.evidence_ref || failure.evidenceRef || 'missing')}</span></li>`).join('') : '<li>No observed failures recorded.</li>';
      const hypothesisRows = hypotheses.length ? hypotheses.map((hypothesis) => `<li><strong>${escapeText(hypothesis.id || 'hypothesis')}</strong>: ${escapeText(hypothesis.summary || 'missing summary')}</li>`).join('') : '<li>No next-step hypotheses recorded.</li>';
      return `<article class="artifact behavior-evidence-bundle">
        <h4>${escapeText(bundle.bundle_id || bundle.bundleId || 'behavior evidence bundle')}</h4>
        <div class="run-meta"><span class="${statusClass(bundle.status || 'unknown')}">${escapeText(bundle.status || 'unknown')}</span> · refs ${escapeText(bundle.lifecycle_ref_count ?? bundle.lifecycleRefCount ?? 0)} · ${escapeText(bundle.path || 'no path')}</div>
        ${bundle.read_error || bundle.readError ? `<div class="artifact-warning">Malformed: ${escapeText(bundle.read_error || bundle.readError)}</div>` : ''}
        ${blockers.length ? `<div class="artifact-warning">Blocked: ${escapeText(blockers.join(' · '))}</div>` : ''}
        <h4>Observed failures</h4><ul class="run-meta-list">${failureRows}</ul>
        <h4>Next-step hypotheses</h4><ul class="run-meta-list">${hypothesisRows}</ul>
        <div class="run-meta">Evidence refs: ${escapeText(refs.join(' · ') || 'none')}</div>
        <p class="run-meta">${escapeText(guardrails.join(' · ') || 'read-only rust/local untracked behavior evidence.')}</p>
      </article>`;
    }).join('');
    return `<section class="panel behavior-evidence-lifecycle"><h3>Behavior evidence lifecycle</h3>
      <p class="run-meta">Status ${escapeText(model.status || 'unknown')} · bundles ${escapeText(model.bundle_count ?? model.bundleCount ?? bundles.length)} · malformed ${escapeText(model.malformed_count ?? model.malformedCount ?? 0)} · lifecycle refs ${escapeText(model.lifecycle_ref_count ?? model.lifecycleRefCount ?? 0)} · failures ${escapeText(model.observed_failure_count ?? model.observedFailureCount ?? 0)} · hypotheses ${escapeText(model.next_step_hypothesis_count ?? model.nextStepHypothesisCount ?? 0)}</p>
      <p class="run-meta">${escapeText(model.boundary || 'read-only structured behavior lifecycle evidence; no command bridge, auto-apply, or trusted writes.')}</p>
      <div class="artifact-grid">${cards || '<p class="empty-state compact">No readable behavior evidence bundles.</p>'}</div>
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

  function renderSourceApplyHandoff(run) {
    const FORBIDDEN = ['apply_patch', 'merge_branch', 'self_approve_review', 'bypass_review_gate', 'execute_command', 'write_trusted_file', 'browser_command_bridge'];
    const exported = run?.source_apply_handoff || run?.sourceApplyHandoff || null;
    const guard = sourcePatchStaleTargetGuards(run)[0]?.value || {};
    const transaction = sourcePatchApplyTransactions(run)[0]?.value || {};
    const guardTarget = (Array.isArray(guard.targets) ? guard.targets[0] : null) || {};
    const txTarget = (Array.isArray(transaction.targets) ? transaction.targets[0] : null) || {};
    const exportedTarget = (exported && typeof exported.target === 'object' && exported.target) ? exported.target : {};
    if (!exported && !Object.keys(guard).length && !Object.keys(transaction).length) {
      return '<section class="panel source-apply-handoff"><h3>Source apply handoff</h3><p class="empty-state">No source apply handoff inputs are exported for this run.</p><p class="run-meta">Read-only dashboard surface. Studio produces preview inputs only; the browser cannot apply patches, merge branches, self-approve reviews, bypass review gates, execute commands, or write trusted files.</p></section>';
    }
    const path = exportedTarget.path || txTarget.path || guardTarget.path || 'unknown target';
    const expectedHash = exportedTarget.expected_hash || exportedTarget.expectedHash || guardTarget.expectedHash || guardTarget.expected_hash || 'unknown';
    const guardStatus = guard.readModel?.status || guard.read_model?.status || guard.status || '';
    const stale = typeof exportedTarget.stale === 'boolean'
      ? exportedTarget.stale
      : (guardStatus ? !/fresh|current|matches/i.test(String(guardStatus)) : null);
    const staleLabel = stale === true ? 'stale (re-preview required)' : stale === false ? 'fresh (matches expected before hash)' : 'unknown';
    const diffSummary = exported?.draft_diff_summary || exported?.draftDiffSummary || transaction.readModel?.readinessLabel || transaction.read_model?.readiness_label || 'No draft diff summary recorded.';
    const hints = Array.isArray(exported?.verification_hints || exported?.verificationHints)
      ? (exported.verification_hints || exported.verificationHints)
      : ['Apply is handled only by the Safe Source Apply review gates outside the browser.', 'A non-self accepted review decision plus rollback and evidence refs are required before any trusted apply.'];
    return `<section class="panel source-apply-handoff"><h3>Source apply handoff</h3>
      <p class="run-meta">Studio produces Safe Source Apply preview inputs only. Direct apply: <span class="${statusClass('blocked')}">disabled · no direct apply</span>. Apply, merge, and self-approve are handled by the Safe Source Apply review gates, not the browser.</p>
      <article class="artifact source-apply-handoff-target">
        <h4>${escapeText(path)}</h4>
        <div class="run-meta"><span class="${statusClass(stale === false ? 'passed' : 'unknown')}">${escapeText(staleLabel)}</span> · expected hash ${escapeText(expectedHash)}</div>
        <div class="run-meta">Draft diff: ${escapeText(diffSummary)}</div>
        <div class="run-meta">Review required: true · Rollback required: true · Evidence required: true</div>
        <div class="run-meta">Forbidden actions: ${escapeText(FORBIDDEN.join(' · '))}</div>
        <ul class="run-meta-list">${hints.map((hint) => `<li>${escapeText(hint)}</li>`).join('')}</ul>
      </article>`;
  }

  function renderEntityComponentInspector(run) {
    const SUPPORTED = ['string', 'number', 'boolean', 'enum', 'vector'];
    const source = run?.entity_component_inspector || run?.entityComponentInspector || null;
    if (!source || typeof source !== 'object' || Array.isArray(source) || source.present === false) {
      const empty = source?.empty_state || source?.emptyState || 'No entity/component inspector inputs are exported for this run.';
      return `<section class="panel entity-component-inspector"><h3>Entity / component inspector</h3><p class="empty-state">${escapeText(empty)}</p><p class="run-meta">Read-only dashboard surface. Studio inspects component fields and produces draft operations only; the browser cannot write trusted files, execute commands, or apply edits.</p></section>`;
    }
    const selectedEntity = source.selected_entity || source.selectedEntity || '';
    const entities = Array.isArray(source.entities) ? source.entities : [];
    const components = Array.isArray(source.components) ? source.components : [];
    const entityRows = entities.length
      ? entities.map((entity) => {
          const id = entity?.id || entity?.entity_id || 'unknown';
          const isSelected = entity?.selected === true || id === selectedEntity;
          return `<div class="run-meta"><span class="${statusClass(isSelected ? 'passed' : 'unknown')}">${escapeText(isSelected ? 'selected' : 'not selected')}</span> ${escapeText(entity?.name || id)} · ${escapeText(id)}</div>`;
        }).join('')
      : '<div class="run-meta">No entities recorded.</div>';
    const componentRows = components.length
      ? components.map((component) => {
          const fields = Array.isArray(component?.fields) ? component.fields : [];
          const fieldRows = fields.length
            ? fields.map((field) => {
                const type = String(field?.type || 'unknown');
                const supported = SUPPORTED.includes(type);
                const editable = field?.editable !== false && field?.unsafe !== true && supported;
                const valueText = field?.value === undefined ? '(no value)' : JSON.stringify(field.value);
                const label = editable ? 'editable (draft only)' : (field?.unsafe ? 'unsafe · blocked' : (supported ? 'read-only' : 'unsupported type · blocked'));
                return `<li><span class="${statusClass(editable ? 'passed' : 'unknown')}">${escapeText(label)}</span> ${escapeText(field?.name || 'field')} · type ${escapeText(type)} · value ${escapeText(valueText)}</li>`;
              }).join('')
            : '<li>No fields recorded.</li>';
          return `<article class="artifact entity-component"><h4>${escapeText(component?.component || component?.name || 'component')}</h4><div class="run-meta">entity ${escapeText(component?.entity_id || component?.entityId || selectedEntity || 'unknown')} · ${escapeText(component?.path || component?.source_path || 'unknown')}</div><ul class="run-meta-list">${fieldRows}</ul></article>`;
        }).join('')
      : '<div class="run-meta">No components recorded.</div>';
    return `<section class="panel entity-component-inspector"><h3>Entity / component inspector</h3>
      <p class="run-meta">Read-only inspection of exported entity/component fields. Supported primitive types: ${escapeText(SUPPORTED.join(', '))}. Valid edits produce draft operations only; trusted apply requires Safe Source Apply handoff. The browser cannot write trusted files, execute commands, or apply edits.</p>
      <div class="run-meta">Entities (selection read-only):</div>${entityRows}
      ${componentRows}
    </section>`;
  }
  function renderDraftOperationModel(run) {
    const KINDS = ['set_component_field', 'add_component', 'remove_component', 'rename_entity', 'reorder_child'];
    const FORBIDDEN = ['apply', 'apply_patch', 'merge', 'merge_branch', 'write_file', 'write_trusted_file', 'self_approve', 'bypass_review_gate', 'execute_command', 'run_command', 'publish', 'deploy', 'sign', 'upload', 'install', 'network_install'];
    const SCHEMAS = ['visual-edit-draft-v1'];
    const source = run?.studio_draft_authoring || run?.studioDraftAuthoring || null;
    const drafts = source && Array.isArray(source.drafts) ? source.drafts : (source && Array.isArray(source.records) ? source.records : []);
    if (!source || !drafts.length) {
      return '<section class="panel draft-operation-model"><h3>Draft operation model</h3><p class="empty-state">No draft operation model is exported for this run.</p><p class="run-meta">Read-only. Draft operations are review-only; the browser cannot write trusted files, execute commands, merge, self-approve, publish/deploy, or apply edits.</p></section>';
    }
    const pathUnsafe = (value) => {
      const text = String(value == null ? '' : value);
      return /(^|[\\/])\.\.([\\/]|$)/.test(text) || text.startsWith('/') || /^[A-Za-z]:[\\/]/.test(text) || /[;&|`$<>]/.test(text);
    };
    const validateOp = (operation) => {
      const reasons = [];
      if (!operation || typeof operation !== 'object' || Array.isArray(operation)) return { kind: 'unknown', valid: false, reasons: ['Malformed draft operation entry.'] };
      const kind = String(operation.kind || operation.type || 'unknown');
      if (FORBIDDEN.includes(kind)) return { kind, valid: false, reasons: [`Operation kind "${kind}" is a source-mutation/publish bypass and is refused.`] };
      if (!KINDS.includes(kind)) return { kind, valid: false, reasons: [`Unsupported draft operation kind "${kind}".`] };
      for (const [key, value] of Object.entries(operation)) {
        if (/^(command|cmd|exec|shell|hook|script|url|href)$/i.test(key)) reasons.push(`Operation field "${key}" implies command/network execution and is refused.`);
        if (typeof value === 'string' && pathUnsafe(value)) reasons.push(`Operation field "${key}" contains an unsafe path or shell metacharacter.`);
      }
      return { kind, valid: reasons.length === 0, reasons };
    };
    const rows = drafts.map((draft, index) => {
      const draftId = (draft && (draft.draftId || draft.draft_id)) || `draft-${index + 1}`;
      const schemaVersion = (draft && (draft.schemaVersion || draft.schema_version)) || 'visual-edit-draft-v1';
      const schemaSupported = SCHEMAS.includes(String(schemaVersion));
      const ops = (draft && (Array.isArray(draft.proposedOperations) ? draft.proposedOperations : (Array.isArray(draft.proposed_operations) ? draft.proposed_operations : []))) || [];
      const results = ops.map(validateOp);
      const valid = schemaSupported && results.length > 0 && results.every((r) => r.valid);
      const opItems = results.map((r) => `<li><span class="${statusClass(r.valid ? 'passed' : 'blocked')}">${escapeText(r.kind)}</span>${r.reasons.length ? ` — ${escapeText(r.reasons.join('; '))}` : ''}</li>`).join('') || '<li>No proposed operations recorded.</li>';
      const diff = valid ? `<pre>${escapeText(`# Draft preview diff (review-only)\n${results.map((r, i) => `~ op#${i + 1} ${r.kind}`).join('\n')}`)}</pre>` : '<p class="run-meta">Draft is blocked; no preview diff until all operations validate.</p>';
      return `<article class="artifact draft-operation"><h4>${escapeText(draftId)}</h4><div class="run-meta"><span class="${statusClass(valid ? 'passed' : 'blocked')}">${valid ? 'validated' : 'blocked'}</span> · schema ${escapeText(String(schemaVersion))} ${schemaSupported ? '(supported)' : '(unsupported)'}</div><ul class="run-meta-list">${opItems}</ul>${diff}</article>`;
    }).join('');
    return `<section class="panel draft-operation-model"><h3>Draft operation model</h3>
      <p class="run-meta">Read-only preview of neutral draft operations (${escapeText(KINDS.join(', '))}). Drafts are suitable for Safe Source Apply preview handoff; the browser cannot write trusted files, execute commands, merge, self-approve, publish/deploy, or apply edits.</p>
      ${rows}
    </section>`;
  }

  function studioKeyboardNavModel(run = null) {
    const hasRun = Boolean(run);
    const regions = [
      { id: 'run-list', label: 'Run list', role: 'navigation', shortcut: 'Alt+1', detail: hasRun ? 'select an exported run to inspect' : 'no runs loaded' },
      { id: 'run-detail', label: 'Run detail panel', role: 'region', shortcut: 'Alt+2', detail: hasRun ? 'read-only run evidence' : 'no run selected' },
      { id: 'evidence-categories', label: 'Evidence categories', role: 'region', shortcut: 'Alt+3', detail: 'category breakdown' },
      { id: 'run-comparison', label: 'Run comparison', role: 'region', shortcut: 'Alt+4', detail: 'before/after deltas' },
      { id: 'replay-controls', label: 'Replay controls', role: 'region', shortcut: 'Alt+5', detail: 'read-only replay stepping' },
    ];
    // Documented contrast/readability smoke pairs (intent only; no color computation).
    const contrastPairs = [
      { region: 'run-list', label: 'Run list', intent: 'high-contrast list labels readable at default zoom' },
      { region: 'run-detail', label: 'Run detail panel', intent: 'card labels readable against panel background' },
      { region: 'run-comparison', label: 'Run comparison', intent: 'delta status text readable for color-blind users' },
    ];
    return { schemaVersion: 'studio-keyboard-nav-v1', regions, contrastPairs };
  }

  function nextStudioFocus(regions, currentId, dir = 'next') {
    const list = Array.isArray(regions) ? regions : [];
    if (!list.length) return null;
    const index = list.findIndex((region) => region && region.id === currentId);
    if (index === -1) return list[0].id;
    if (dir === 'prev') {
      return index === 0 ? list[list.length - 1].id : list[index - 1].id;
    }
    return index === list.length - 1 ? list[0].id : list[index + 1].id;
  }

  function restoreStudioFocus(regions, lastId) {
    const list = Array.isArray(regions) ? regions : [];
    if (!list.length) return null;
    const match = list.find((region) => region && region.id === lastId);
    return match ? match.id : list[0].id;
  }

  function renderStudioAccessibilityNavSurface(run = null) {
    const model = studioKeyboardNavModel(run);
    const items = model.regions.map((region) => `<li role="listitem" tabindex="0" data-region-id="${escapeText(region.id)}"><a href="#${escapeText(region.id)}"><strong>${escapeText(region.label)}</strong></a> <kbd>${escapeText(region.shortcut)}</kbd><br><small>role ${escapeText(region.role)} · ${escapeText(region.detail)}</small></li>`).join('');
    const contrast = model.contrastPairs.map((pair) => `<li role="listitem"><strong>${escapeText(pair.label)}</strong>: ${escapeText(pair.intent)}</li>`).join('');
    return `<section class="panel" id="studio-accessibility-nav"><h3>Keyboard navigation and landmarks</h3>
      <nav role="navigation" aria-label="Studio keyboard navigation guide">
        <p class="run-meta">Read-only keyboard-shortcut and landmark guide. No apply, merge, command execution, or trusted-write controls are exposed.</p>
        <ol role="list" aria-label="Ordered focusable regions">${items}</ol>
      </nav>
      <h4>Contrast and readability smoke</h4>
      <ul role="list" aria-label="Contrast readability intent">${contrast}</ul>
    </section>`;
  }

  function renderRunDetail(run) {
    return renderRunDetailWithState(run, createReplayState(run), run?.regression_matrix || run?.regressionMatrix || null, run?.loop_evidence_bundles || run?.loopEvidenceBundles || run?.loop_evidence_bundle || run?.loopEvidenceBundle || null, run?.agent_handoffs || run?.agentHandoffs || run?.agent_handoff || run?.agentHandoff || null);
  }

  function renderEvaluatorDepthInspection(run = {}) {
    const verdict = run.verdict && typeof run.verdict === 'object' ? run.verdict : {};
    const categories = verdict.gateCategories || verdict.gate_categories || {};
    const summary = verdict.gateSummary || verdict.gate_summary || [];
    const visual = Array.isArray(verdict.visual || verdict.visualGateVerdicts || verdict.visual_gate_verdicts) ? (verdict.visual || verdict.visualGateVerdicts || verdict.visual_gate_verdicts) : [];
    const semantic = Array.isArray(verdict.semantic || verdict.semanticInvariantVerdicts || verdict.semantic_invariant_verdicts) ? (verdict.semantic || verdict.semanticInvariantVerdicts || verdict.semantic_invariant_verdicts) : [];
    const categoryKeys = ['mechanical', 'runtime', 'visual', 'semantic'];
    if (!Object.keys(categories).length && !(Array.isArray(summary) && summary.length) && !visual.length && !semantic.length) {
      return '<p class="empty-state">No evaluator-depth verdict categories are exported for this run.</p><p class="run-meta">Read-only inspection only; the dashboard does not write trusted state, expose mutation controls, bridge commands, auto-fix, auto-merge, or score subjective quality.</p>';
    }
    const categoryCards = categoryKeys.map((key) => {
      const category = categories[key] || {};
      const declared = category.declared === true ? 'declared' : 'undeclared / neutral';
      return `<article class="category-card">
        <div class="card-label">${escapeText(key)} gate</div>
        <div class="card-value"><span class="${statusClass(category.status || (category.declared === false ? 'neutral' : 'unknown'))}">${escapeText(category.status || (category.declared === false ? 'neutral' : 'unknown'))}</span></div>
        <div class="run-meta">${escapeText(declared)} · results ${escapeText(category.resultCount ?? category.result_count ?? 0)} · failures ${escapeText(category.failureCount ?? category.failure_count ?? 0)}</div>
      </article>`;
    }).join('');
    const visualRows = visual.length
      ? visual.map((gate) => {
          const refs = [gate.comparisonRef || gate.comparison_ref, ...(Array.isArray(gate.evidenceRefs || gate.evidence_refs) ? (gate.evidenceRefs || gate.evidence_refs) : [])].filter(Boolean);
          const threshold = Array.isArray(gate.thresholdSummary || gate.threshold_summary) ? (gate.thresholdSummary || gate.threshold_summary).join(' · ') : 'no threshold summary';
          return `<li><strong>${escapeText(gate.scenarioId || gate.scenario_id || 'scenario')} / ${escapeText(gate.checkpointId || gate.checkpoint_id || 'checkpoint')}</strong>: <span class="${statusClass(gate.state)}">${escapeText(gate.state || 'unknown')}</span> · changed ${escapeText(gate.changedPixels ?? gate.changed_pixels ?? 0)} px · regions ${escapeText(gate.changedRegionCount ?? gate.changed_region_count ?? 0)} · ${escapeText(threshold)}${renderRefLinks('Visual evidence', refs, run)}</li>`;
        }).join('')
      : '<li>No visual gate verdicts exported.</li>';
    const semanticRows = semantic.length
      ? semantic.map((gate) => {
          const refs = [gate.modelRef || gate.model_ref, gate.worldStateRef || gate.world_state_ref, ...(Array.isArray(gate.evidenceRefs || gate.evidence_refs) ? (gate.evidenceRefs || gate.evidence_refs) : [])].filter(Boolean);
          return `<li><strong>${escapeText(gate.invariantId || gate.invariant_id || 'invariant')}</strong>: <span class="${statusClass(gate.state)}">${escapeText(gate.state || 'unknown')}</span> · target ${escapeText(gate.targetPath || gate.target_path || 'n/a')} · ${escapeText(gate.reason || 'no reason')}${renderRefLinks('Semantic evidence', refs, run)}</li>`;
        }).join('')
      : '<li>No semantic invariant verdicts exported.</li>';
    return `<div class="category-grid">${categoryCards}</div>
      ${Array.isArray(summary) && summary.length ? `<h4>Gate summary</h4><ul class="run-meta-list">${summary.slice(0, 8).map((item) => `<li>${escapeText(item)}</li>`).join('')}</ul>` : ''}
      <h4>Visual gate verdicts</h4><ul class="run-meta-list">${visualRows}</ul>
      <h4>Semantic invariant verdicts</h4><ul class="run-meta-list">${semanticRows}</ul>
      <p class="run-meta">Read-only evaluator-depth inspection from exported JSON only; no trusted writes, mutation controls, command bridge, hidden execution, auto-fix, auto-merge, or subjective quality scoring.</p>`;
  }

  function renderRunDetailWithState(run, replayState, regressionMatrix = null, loopEvidenceBundles = null, agentHandoffs = null) {
    if (!run) return '<div class="empty-state">Select a run to inspect its evidence.</div>';
    const verdict = run.verdict || {};
    const summary = summarizeRun(run);
    const evidence = Array.isArray(run.evidence) ? run.evidence : [];
    const mutations = Array.isArray(run.mutations) ? run.mutations : [];
    return `<article>
      <h2>${escapeText(summary.id)}</h2>
      ${renderStudioAccessibilityNavSurface(run)}
      ${renderStudioDiagnosticsSurface(run)}
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
      <section class="panel"><h3>Camera/layer read model</h3>${renderCameraLayerSummary(run.engine_summaries || {})}</section>
      <section class="panel"><h3>Scene render breakdown</h3>${renderRenderBreakdownSummary(run.engine_summaries || {})}</section>
      <section class="panel"><h3>Gameplay trigger/flags</h3>${renderGameplaySummary(run.engine_summaries || {})}</section>
      <section class="panel"><h3>Animation and VFX evidence</h3>${renderAnimationVfxSummary(run.engine_summaries || {})}</section>
      <section class="panel"><h3>Audio intent evidence</h3>${renderAudioEvidenceSummary(run.engine_summaries || {})}</section>
      <section class="panel"><h3>Runtime profiler evidence</h3>${renderRuntimeProfilerSummary(run.engine_summaries || {})}</section>
      <section class="panel"><h3>Input action mapping</h3>${renderInputActionSummary(run.engine_summaries || {})}</section>
      <section class="panel"><h3>Asset reference integrity</h3>${renderAssetIntegrity(run)}</section>
      <section class="panel"><h3>Runtime asset loading</h3>${renderAssetLoading(run)}</section>
      <section class="panel"><h3>Asset preview evidence</h3>${renderAssetPreview(run)}</section>
      <section class="panel"><h3>Plugin registry browser</h3>${renderPluginRegistry(run)}</section>
      ${renderBehaviorEvidenceLifecycle(run)}
      <section class="panel"><h3>Source apply worktree context</h3>${renderSourceApplyWorktreeContext(run)}</section>
      <section class="panel"><h3>Runtime invariant evidence</h3>${renderRuntimeInvariants(run)}</section>
      <section class="panel"><h3>Route attempt evidence</h3>${renderRouteAttempts(run)}</section>
      <section class="panel"><h3>Visual comparison evidence</h3>${renderVisualComparisons(run)}</section>
      <section class="panel"><h3>Evaluator depth inspection</h3>${renderEvaluatorDepthInspection(run)}</section>
      <section class="panel"><h3>Studio scene tree inspector</h3>${renderSceneTreeInspector(run)}</section>
      <section class="panel"><h3>QA scenario candidates</h3>${renderQaScenarioCandidates(run)}</section>
      <section class="panel"><h3>Adversarial input fuzzing plans</h3>${renderFuzzingPlans(run)}</section>
      <section class="panel"><h3>QA worker assignments</h3>${renderQaWorkerAssignments(run)}</section>
      <section class="panel"><h3>QA agent work queues</h3>${renderQaAgentWorkQueues(run)}</section>
      <section class="panel"><h3>Performance/regression lanes</h3>${renderPerformanceRegressionLanes(run)}</section>
      ${renderSourcePatchEvidenceBundles(run)}
      ${renderSourcePatchApplyTransactions(run)}
      ${renderSourcePatchStaleTargetGuards(run)}
      ${renderSourceApplyHandoff(run)}
      ${renderAssetBrowser(run)}
      ${renderSceneCanvas(run)}
      ${renderDraftOperationModel(run)}${renderEntityComponentInspector(run)}
      <section class="panel"><h3>Verdict summary</h3><pre>${escapeText(JSON.stringify(verdict, null, 2))}</pre></section>
      ${renderCommandContext(run)}
      ${renderLoopDryRunSummary(run.loop_dry_run || run.loopDryRun || null)}
      ${renderLoopExecutionSummary(run.loop_execution || run.loopExecution || null)}
      ${renderLoopRecoveryStatus(run.loop_recovery || run.loopRecovery || run.loop_status || run.loopStatus || null)}
      ${renderStudioMultiAgentPipelineInspection(pipelineInspectionModel(run))}
      ${renderProductionTaskBoards(run.production_task_boards || run.productionTaskBoards || run.production_task_board || run.productionTaskBoard || null)}
      ${renderOwnershipPolicies(run.ownership_policies || run.ownershipPolicies || run.ownership_policy || run.ownershipPolicy || null)}
      ${renderAgentRoleModels(run.agent_role_models || run.agentRoleModels || run.agent_role_model || run.agentRoleModel || null)}
      ${renderAgentWorkPackages(run.agent_work_packages || run.agentWorkPackages || run.agent_work_package || run.agentWorkPackage || null)}
      ${renderAgentHandoffs(agentHandoffs || [
        ...normalizeAgentHandoffs(run.agent_handoffs || run.agentHandoffs || run.agent_handoff || run.agentHandoff || null),
        ...normalizeAgentHandoffs(run.agent_handoff_v2s || run.agentHandoffV2s || null),
      ])}
      ${renderReviewCriticGates(run.review_critic_gates || run.reviewCriticGates || run.review_critic_gate || run.reviewCriticGate || run.review_gate || run.reviewGate || null)}
      ${renderProductionEvidenceBundles(run.production_evidence_bundles || run.productionEvidenceBundles || run.production_evidence_bundle || run.productionEvidenceBundle || null)}
      ${renderLoopEvidenceBundles(loopEvidenceBundles || run.loop_evidence_bundles || run.loopEvidenceBundles || run.loop_evidence_bundle || run.loopEvidenceBundle || null)}
      ${renderJournalViewer(run)}
      ${renderMutationLifecycle(run)}
      ${renderRegressionPromotions(run)}
      ${renderRegressionMatrix(regressionMatrix)}
      ${renderProjectContext(run)}
      ${renderTransactionProvenance(run)}
      ${renderReplayControls(run, replayState)}
      ${renderStudioCommandPaletteSurface(run)}
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
        const selectedWithTopLevelBundles = selected ? {
          ...selected,
          production_evidence_bundles: selected?.production_evidence_bundles || selected?.productionEvidenceBundles || data.production_evidence_bundles || data.productionEvidenceBundles || data.production_evidence_bundle || data.productionEvidenceBundle || null,
        } : selected;
        detailEl.innerHTML = renderRunDetailWithState(selectedWithTopLevelBundles, replayStateFor(selected), data.regression_matrix || data.regressionMatrix || null, data.loop_evidence_bundles || data.loopEvidenceBundles || null, [
          ...normalizeAgentHandoffs(data.agent_handoffs || data.agentHandoffs || null),
          ...normalizeAgentHandoffs(data.agent_handoff_v2s || data.agentHandoffV2s || null),
        ]);
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

  function arrayField(value, ...keys) {
    if (!value || typeof value !== 'object') return [];
    for (const key of keys) {
      if (Array.isArray(value[key])) return value[key].filter(Boolean);
    }
    return [];
  }

  function diagnosticObjectCandidates(item) {
    const candidates = [];
    const seen = new Set();
    const visit = (value) => {
      if (!value || typeof value !== 'object' || Array.isArray(value) || seen.has(value)) return;
      seen.add(value);
      candidates.push(value);
      visit(value.value);
      visit(value.readModel);
      visit(value.read_model);
      visit(value.validation);
    };
    visit(item);
    return candidates;
  }

  function mutationDiagnosticArtifacts(run, id, path) {
    const artifacts = Array.isArray(run?.mutation_artifacts || run?.mutationArtifacts)
      ? (run.mutation_artifacts || run.mutationArtifacts)
      : [];
    return artifacts.filter((artifact) => artifact && typeof artifact === 'object' && (artifact.id === id || artifact.path === path));
  }

  function firstDiagnosticStatus(candidates) {
    const statusSource = candidates.find((candidate) => candidate.status !== undefined && candidate.status !== null && String(candidate.status));
    return statusSource ? String(statusSource.status) : '';
  }

  function diagnosticStatuses(candidates) {
    return candidates
      .map((candidate) => candidate.status)
      .filter((status) => status !== undefined && status !== null && String(status))
      .map((status) => String(status));
  }

  function blockedDiagnosticStatus(status) {
    return /blocked|forbidden|denied|rejected|failed|error|incompatible/.test(String(status || '').toLowerCase());
  }

  function staleDiagnosticStatus(status) {
    return /stale|expired|outdated|diverged|mismatch|dirty/.test(String(status || '').toLowerCase());
  }

  function diagnosticTarget(item, candidates) {
    const source = candidates.find((candidate) => candidate.id || candidate.transactionId || candidate.transaction_id || candidate.guardId || candidate.guard_id || candidate.targetPath || candidate.target_path || candidate.path) || item || {};
    return source.id || source.transactionId || source.transaction_id || source.guardId || source.guard_id || source.targetPath || source.target_path || source.path || 'operation';
  }

  function studioDiagnosticsModel(run) {
    const diagnostics = [];
    const record = (kind, severity, summary, governingGuardrail) => {
      const entry = { kind: String(kind || 'unknown'), severity: String(severity || 'info'), summary: String(summary || '') };
      if (governingGuardrail) entry.governing_guardrail = String(governingGuardrail);
      diagnostics.push(entry);
    };
    const safeCheck = (kind, fn) => {
      try {
        fn();
      } catch (error) {
        record(kind, 'error', `Diagnostic check "${kind}" failed closed: ${error && error.message ? error.message : 'unknown error'}.`, 'fail-closed-on-malformed-input');
      }
    };

    if (run === null || run === undefined || typeof run !== 'object') {
      record('missing-data', 'error', 'No run data is available; the run object is missing or is not an object.', 'read-only-evidence-only');
      return { diagnostics, counts: countBySeverity(diagnostics) };
    }

    safeCheck('missing-data', () => {
      const summary = run.summary;
      if (!summary || typeof summary !== 'object') {
        record('missing-data', 'error', 'Run summary is missing or malformed; core run metadata cannot be displayed.', 'read-only-evidence-only');
      } else if (!summary.id) {
        record('missing-data', 'warning', 'Run summary is missing an id; the run cannot be reliably identified.', 'read-only-evidence-only');
      }
    });

    safeCheck('invalid-schema', () => {
      if ('evidence' in run && run.evidence !== undefined && !Array.isArray(run.evidence)) {
        record('invalid-schema', 'error', 'Evidence index has an invalid schema; expected an array of evidence entries.', 'evidence-schema-contract');
      }
      if ('mutations' in run && run.mutations !== undefined && !Array.isArray(run.mutations)) {
        record('invalid-schema', 'error', 'Mutations have an invalid schema; expected an array of mutation entries.', 'evidence-schema-contract');
      }
    });

    safeCheck('blocked-operation', () => {
      const sources = [
        run.source_patch_apply_transactions, run.sourcePatchApplyTransactions,
        run.source_apply, run.sourceApply,
        run.mutations,
        mutationDiagnosticArtifacts(run, 'source-patch-apply-transaction', 'mutation/source-patch-apply-transaction.json'),
      ];
      sources.forEach((source) => {
        const list = Array.isArray(source) ? source : (source && typeof source === 'object' ? [source] : []);
        list.forEach((item) => {
          if (!item || typeof item !== 'object') return;
          const candidates = diagnosticObjectCandidates(item);
          const status = firstDiagnosticStatus(candidates);
          const forbidden = candidates.flatMap((candidate) => arrayField(candidate, 'forbiddenActions', 'forbidden_actions'));
          const blocked = candidates.flatMap((candidate) => arrayField(candidate, 'blockedReasons', 'blocked_reasons'));
          const reasons = blocked.length
            ? blocked.concat(blockedDiagnosticStatus(status) ? forbidden.map((action) => `forbidden action: ${action}`) : [])
            : (item.blocked === true ? ['blocked operation'] : (blockedDiagnosticStatus(status) ? [`status ${status}`].concat(forbidden.map((action) => `forbidden action: ${action}`)) : []));
          if (reasons.length) {
            record('blocked-operation', 'warning', `Operation "${diagnosticTarget(item, candidates)}" is blocked: ${reasons.join('; ')}.`, 'trusted-persistence-boundary');
          }
        });
      });
    });

    safeCheck('stale-source-apply-target', () => {
      const sources = [
        run.source_patch_stale_target_guards, run.sourcePatchStaleTargetGuards,
        run.stale_target_guards, run.staleTargetGuards,
        mutationDiagnosticArtifacts(run, 'source-patch-stale-target-guard', 'mutation/source-patch-stale-target-guard.json'),
      ];
      sources.forEach((source) => {
        const list = Array.isArray(source) ? source : (source && typeof source === 'object' ? [source] : []);
        list.forEach((guard) => {
          if (!guard || typeof guard !== 'object') return;
          const candidates = diagnosticObjectCandidates(guard);
          const statuses = diagnosticStatuses(candidates);
          const staleStatus = statuses.find(staleDiagnosticStatus);
          const status = staleStatus || statuses.find(blockedDiagnosticStatus);
          if (status) {
            const descriptor = staleStatus ? 'stale' : 'blocked';
            record('stale-source-apply-target', 'warning', `Source-apply target is ${descriptor} (${status}); apply must be re-validated against the current base before trusting it.`, 'stale-target-guard');
          }
        });
      });
    });

    safeCheck('broken-evidence-bundle', () => {
      const evidence = Array.isArray(run.evidence) ? run.evidence : [];
      evidence.forEach((item) => {
        if (!item || typeof item !== 'object') return;
        if (item.read_error || item.readError) {
          record('broken-evidence-bundle', 'error', `Evidence "${item.id || item.path || 'entry'}" is broken: ${item.read_error || item.readError}.`, 'evidence-integrity');
        } else if (item.exists === false) {
          record('broken-evidence-bundle', 'warning', `Evidence "${item.id || item.path || 'entry'}" is missing on disk.`, 'evidence-integrity');
        }
      });
    });

    safeCheck('malformed-plugin-descriptor', () => {
      const registry = run.plugin_registry || run.pluginRegistry || null;
      const registries = Array.isArray(registry?.registries) ? registry.registries : [];
      registries.forEach((reg) => {
        const plugins = Array.isArray(reg?.plugins) ? reg.plugins : [];
        plugins.forEach((plugin) => {
          if (!plugin || typeof plugin !== 'object') return;
          const validation = String(plugin.validationStatus || plugin.validation_status || '').toLowerCase();
          if (validation && validation !== 'valid' && validation !== 'ok') {
            record('malformed-plugin-descriptor', 'warning', `Plugin descriptor "${plugin.pluginId || plugin.plugin_id || 'plugin'}" is ${plugin.validationStatus || plugin.validation_status}.`, 'plugin-descriptor-contract');
          }
        });
      });
    });

    safeCheck('export-package-issue', () => {
      const bundles = run.production_evidence_bundles || run.productionEvidenceBundles || run.production_evidence_bundle || run.productionEvidenceBundle || null;
      const list = Array.isArray(bundles) ? bundles : (bundles && typeof bundles === 'object' ? [bundles] : []);
      list.forEach((bundle) => {
        if (!bundle || typeof bundle !== 'object') return;
        const status = String(bundle.status || '').toLowerCase();
        if (status && status !== 'ready' && status !== 'ok' && status !== 'complete') {
          record('export-package-issue', 'warning', `Export/package bundle "${bundle.id || bundle.path || 'bundle'}" is not ready (${bundle.status}).`, 'export-package-contract');
        }
      });
    });

    return { diagnostics, counts: countBySeverity(diagnostics) };
  }

  function countBySeverity(diagnostics) {
    const counts = { total: diagnostics.length, error: 0, warning: 0, info: 0 };
    diagnostics.forEach((diagnostic) => {
      const severity = diagnostic.severity === 'error' || diagnostic.severity === 'warning' || diagnostic.severity === 'info' ? diagnostic.severity : 'info';
      counts[severity] += 1;
    });
    return counts;
  }

  function studioErrorBoundary(label, renderFn) {
    try {
      const result = renderFn();
      return typeof result === 'string' ? result : String(result ?? '');
    } catch (error) {
      const message = error && error.message ? error.message : 'unknown error';
      return `<section class="panel error-boundary"><h3>${escapeText(label || 'Panel')} could not render</h3><p class="artifact-warning">This panel failed to render and was caught by the Studio error boundary: ${escapeText(message)}.</p><p class="run-meta">The failure is surfaced as an error, not as a successful, applied, or privileged operation. Studio remains read-only; no apply, merge, write, or command execution occurred.</p></section>`;
    }
  }

  function renderStudioDiagnosticsSurface(run) {
    return studioErrorBoundary('Studio diagnostics', () => {
      const model = studioDiagnosticsModel(run);
      const diagnostics = Array.isArray(model.diagnostics) ? model.diagnostics : [];
      const counts = model.counts || countBySeverity(diagnostics);
      if (!diagnostics.length) {
        return '<section class="panel"><h3>Studio diagnostics</h3><p class="empty-state compact">No diagnostics detected in the exported run. This surface is read-only and renders no controls.</p></section>';
      }
      const blocked = diagnostics.filter((diagnostic) => diagnostic.kind === 'blocked-operation');
      const stale = diagnostics.filter((diagnostic) => diagnostic.kind === 'stale-source-apply-target');
      const rows = diagnostics.map((diagnostic) => `<li><span class="${statusClass(diagnostic.severity)}">${escapeText(diagnostic.severity)}</span> <strong>${escapeText(diagnostic.kind)}</strong>: ${escapeText(diagnostic.summary)}${diagnostic.governing_guardrail ? `<br><small>Governing guardrail: ${escapeText(diagnostic.governing_guardrail)}</small>` : ''}</li>`).join('');
      const blockedRows = blocked.length
        ? `<h4>Blocked operations</h4><ul class="run-meta-list">${blocked.map((diagnostic) => `<li>${escapeText(diagnostic.summary)} <small>(blocked by ${escapeText(diagnostic.governing_guardrail || 'unknown guardrail')})</small></li>`).join('')}</ul>`
        : '';
      const staleRows = stale.length
        ? `<h4>Stale-data warnings</h4><ul class="run-meta-list">${stale.map((diagnostic) => `<li class="artifact-warning">${escapeText(diagnostic.summary)}</li>`).join('')}</ul>`
        : '';
      return `<section class="panel"><h3>Studio diagnostics</h3>
        <p class="run-meta">Read-only diagnostics: ${escapeText(counts.total)} issue(s) — ${escapeText(counts.error)} error, ${escapeText(counts.warning)} warning, ${escapeText(counts.info)} info. No controls; Studio cannot apply, merge, write trusted state, or execute commands.</p>
        <ul class="run-meta-list">${rows}</ul>${blockedRows}${staleRows}</section>`;
    });
  }


  const WORKSPACE_LAYOUT_STORAGE_KEY = 'ouroforge.studio.workspace';
  const WORKSPACE_LAYOUT_VERSION = 1;

  function defaultWorkspaceLayout() {
    return {
      version: WORKSPACE_LAYOUT_VERSION,
      selectedProjectId: null,
      selectedSceneId: null,
      selectedEntityId: null,
      visiblePanels: { runList: true, runDetail: true, evidence: true, replay: true },
      panelSizes: { runList: 280, runDetail: 480, evidence: 320 },
      filters: { runSearch: '', evidenceSearch: '' },
    };
  }

  function normalizeWorkspaceLayout(raw) {
    const defaults = defaultWorkspaceLayout();
    if (!raw || typeof raw !== 'object' || Array.isArray(raw)) {
      return defaults;
    }
    if (raw.version !== WORKSPACE_LAYOUT_VERSION) {
      // Missing, older, or future layout versions fall back to defaults (draft-only browser state).
      return defaults;
    }
    const normalizeString = (value) => (typeof value === 'string' ? value : null);
    const visiblePanels = raw.visiblePanels && typeof raw.visiblePanels === 'object' && !Array.isArray(raw.visiblePanels) ? raw.visiblePanels : {};
    const panelSizes = raw.panelSizes && typeof raw.panelSizes === 'object' && !Array.isArray(raw.panelSizes) ? raw.panelSizes : {};
    const filters = raw.filters && typeof raw.filters === 'object' && !Array.isArray(raw.filters) ? raw.filters : {};
    const normalizeSize = (value, fallback) => (typeof value === 'number' && Number.isFinite(value) && value >= 0 ? value : fallback);
    return {
      version: WORKSPACE_LAYOUT_VERSION,
      selectedProjectId: normalizeString(raw.selectedProjectId),
      selectedSceneId: normalizeString(raw.selectedSceneId),
      selectedEntityId: normalizeString(raw.selectedEntityId),
      visiblePanels: {
        runList: typeof visiblePanels.runList === 'boolean' ? visiblePanels.runList : defaults.visiblePanels.runList,
        runDetail: typeof visiblePanels.runDetail === 'boolean' ? visiblePanels.runDetail : defaults.visiblePanels.runDetail,
        evidence: typeof visiblePanels.evidence === 'boolean' ? visiblePanels.evidence : defaults.visiblePanels.evidence,
        replay: typeof visiblePanels.replay === 'boolean' ? visiblePanels.replay : defaults.visiblePanels.replay,
      },
      panelSizes: {
        runList: normalizeSize(panelSizes.runList, defaults.panelSizes.runList),
        runDetail: normalizeSize(panelSizes.runDetail, defaults.panelSizes.runDetail),
        evidence: normalizeSize(panelSizes.evidence, defaults.panelSizes.evidence),
      },
      filters: {
        runSearch: typeof filters.runSearch === 'string' ? filters.runSearch : defaults.filters.runSearch,
        evidenceSearch: typeof filters.evidenceSearch === 'string' ? filters.evidenceSearch : defaults.filters.evidenceSearch,
      },
    };
  }

  function loadWorkspaceLayout(storage) {
    if (!storage || typeof storage.getItem !== 'function') {
      return defaultWorkspaceLayout();
    }
    let rawText = null;
    try {
      rawText = storage.getItem(WORKSPACE_LAYOUT_STORAGE_KEY);
    } catch (error) {
      return defaultWorkspaceLayout();
    }
    if (typeof rawText !== 'string' || !rawText.length) {
      return defaultWorkspaceLayout();
    }
    let parsed = null;
    try {
      parsed = JSON.parse(rawText);
    } catch (error) {
      // Malformed JSON fails closed to defaults; never throw.
      return defaultWorkspaceLayout();
    }
    return normalizeWorkspaceLayout(parsed);
  }

  function saveWorkspaceLayout(storage, state) {
    const normalized = normalizeWorkspaceLayout(state);
    if (storage && typeof storage.setItem === 'function') {
      try {
        storage.setItem(WORKSPACE_LAYOUT_STORAGE_KEY, JSON.stringify(normalized));
      } catch (error) {
        // Browser storage may be unavailable or full; draft-only state is best effort.
      }
    }
    return normalized;
  }

  function resetWorkspaceLayout(storage) {
    if (storage && typeof storage.removeItem === 'function') {
      try {
        storage.removeItem(WORKSPACE_LAYOUT_STORAGE_KEY);
      } catch (error) {
        // Ignore storage removal failures; defaults are returned regardless.
      }
    }
    return defaultWorkspaceLayout();
  }

  function studioPerformanceBudget() {
    return {
      schemaVersion: 'studio-performance-budget-v1',
      thresholds: {
        sceneCount: 64,
        entityNodeCount: 4000,
        assetCount: 1200,
        evidenceRunCount: 256,
        pluginCount: 48,
        panelRenderMs: 250,
        panelUpdateMs: 120,
      },
    };
  }

  function evaluateStudioPerformanceBudget(metrics, budget) {
    const safeMetrics = metrics && typeof metrics === 'object' ? metrics : {};
    const activeBudget = budget && typeof budget === 'object' ? budget : studioPerformanceBudget();
    const thresholds = activeBudget.thresholds && typeof activeBudget.thresholds === 'object' ? activeBudget.thresholds : {};
    const dimensions = [
      'sceneCount',
      'entityNodeCount',
      'assetCount',
      'evidenceRunCount',
      'pluginCount',
      'panelRenderMs',
      'panelUpdateMs',
    ];
    const results = {};
    const gaps = [];
    dimensions.forEach((dimension) => {
      const rawValue = safeMetrics[dimension];
      const rawLimit = thresholds[dimension];
      const hasValue = typeof rawValue === 'number' && Number.isFinite(rawValue);
      const hasLimit = typeof rawLimit === 'number' && Number.isFinite(rawLimit);
      const value = hasValue ? rawValue : null;
      const limit = hasLimit ? rawLimit : null;
      // Fail open to recording gaps: missing metric or limit counts as an explicit gap, never a throw.
      const within = hasValue && hasLimit ? value <= limit : false;
      results[dimension] = { within, value, limit };
      if (!within) {
        gaps.push({
          dimension,
          value,
          limit,
          reason: !hasValue ? 'missing-metric' : !hasLimit ? 'missing-limit' : 'over-budget',
        });
      }
    });
    return {
      schemaVersion: 'studio-performance-budget-evaluation-v1',
      pass: gaps.length === 0,
      dimensions: results,
      gaps,
    };
  }

  function renderStudioPerformanceBudgetSurface(run) {
    const metrics = run && typeof run === 'object' && run.performanceMetrics && typeof run.performanceMetrics === 'object'
      ? run.performanceMetrics
      : {};
    const budget = studioPerformanceBudget();
    const evaluation = evaluateStudioPerformanceBudget(metrics, budget);
    const labels = {
      sceneCount: 'Fixture scenes',
      entityNodeCount: 'Entity / node count',
      assetCount: 'Asset count',
      evidenceRunCount: 'Evidence run count',
      pluginCount: 'Plugin count',
      panelRenderMs: 'Panel render (ms)',
      panelUpdateMs: 'Panel update (ms)',
    };
    const rows = Object.keys(labels).map((dimension) => {
      const result = evaluation.dimensions[dimension] || { within: false, value: null, limit: null };
      const status = result.within ? 'pass' : 'gap';
      const measured = result.value === null ? 'not measured' : result.value;
      const limit = result.limit === null ? 'no limit' : result.limit;
      return `<li><strong>${escapeText(labels[dimension])}</strong>: <span class="${statusClass(status)}">${escapeText(status.toUpperCase())}</span> · measured ${escapeText(measured)} · budget ${escapeText(limit)}</li>`;
    }).join('');
    const overall = evaluation.pass ? 'PASS' : 'GAP';
    const gapSummary = evaluation.gaps.length
      ? `<ul class="run-meta-list">${evaluation.gaps.map((gap) => `<li>${escapeText(labels[gap.dimension] || gap.dimension)}: ${escapeText(gap.reason)} (measured ${escapeText(gap.value === null ? 'n/a' : gap.value)} / budget ${escapeText(gap.limit === null ? 'n/a' : gap.limit)})</li>`).join('')}</ul>`
      : '<p class="run-meta">No over-budget dimensions recorded.</p>';
    return `<section class="panel"><h3>Studio Performance Budget</h3>
      <p class="run-meta">Read-only exported metrics vs declared budget; no trusted writes, mutation controls, command bridge, or browser-side enforcement.</p>
      <p class="card-value"><span class="${statusClass(evaluation.pass ? 'pass' : 'gap')}">Overall ${escapeText(overall)}</span></p>
      <ul class="run-meta-list">${rows}</ul>
      <h4>Recorded gaps</h4>${gapSummary}</section>`;
  }

  function renderSceneCanvas(run) {
    const source = run?.scene_canvas || run?.sceneCanvas || null;
    if (!source || typeof source !== 'object' || Array.isArray(source) || source.present === false) {
      const empty = source?.empty_state || source?.emptyState || 'No scene canvas inputs are exported for this run.';
      return `<section class="panel scene-canvas-panel"><h3>Visual scene canvas</h3><p class="empty-state">${escapeText(empty)}</p><p class="run-meta">Read-only visual preview. Studio produces draft transform operations only; the browser cannot write trusted scene files, execute commands, or apply edits.</p></section>`;
    }
    const num = (value, fallback) => (typeof value === 'number' && Number.isFinite(value) ? value : fallback);
    const width = num(source.width, 640);
    const height = num(source.height, 360);
    const grid = source.grid && typeof source.grid === 'object' ? source.grid : {};
    const gridSize = num(grid.size, 32);
    const snap = grid.snap === true;
    const selectedEntity = source.selected_entity || source.selectedEntity || '';
    const nodes = Array.isArray(source.nodes) ? source.nodes : [];
    const gridLines = [];
    if (gridSize > 0) {
      for (let x = 0; x <= width; x += gridSize) gridLines.push(`<line x1="${x}" y1="0" x2="${x}" y2="${height}" class="canvas-grid-line" stroke="#e0e0e0" stroke-width="1"/>`);
      for (let y = 0; y <= height; y += gridSize) gridLines.push(`<line x1="0" y1="${y}" x2="${width}" y2="${y}" class="canvas-grid-line" stroke="#e0e0e0" stroke-width="1"/>`);
    }
    const shapes = nodes.map((node, index) => {
      const id = node?.id || node?.entity_id || `node-${index + 1}`;
      const isSelected = node?.selected === true || id === selectedEntity;
      const authored = node?.authored !== false;
      const x = num(node?.x, 0);
      const y = num(node?.y, 0);
      const w = num(node?.w ?? node?.width, 24);
      const h = num(node?.h ?? node?.height, 24);
      const rotation = num(node?.rotation, 0);
      const cx = x + (w / 2);
      const cy = y + (h / 2);
      const fill = authored ? '#5b8def' : '#bbbbbb';
      const stroke = isSelected ? '#ff8c00' : '#333333';
      return `<g class="${authored ? 'canvas-node-authored' : 'canvas-node-runtime'}" data-entity="${escapeText(id)}" data-selected="${isSelected ? 'true' : 'false'}" transform="rotate(${rotation} ${cx} ${cy})"><rect x="${x}" y="${y}" width="${w}" height="${h}" fill="${fill}" stroke="${stroke}" stroke-width="${isSelected ? 3 : 1}"/></g>`;
    }).join('');
    const legend = nodes.map((node, index) => {
      const id = node?.id || node?.entity_id || `node-${index + 1}`;
      const isSelected = node?.selected === true || id === selectedEntity;
      const authored = node?.authored !== false;
      return `<li><span class="${statusClass(isSelected ? 'passed' : 'unknown')}">${escapeText(isSelected ? 'selected' : 'not selected')}</span> ${escapeText(node?.name || id)} · ${escapeText(authored ? 'authored state' : 'runtime state')}</li>`;
    }).join('') || '<li>No scene nodes recorded.</li>';
    return `<section class="panel scene-canvas-panel"><h3>Visual scene canvas</h3>
      <p class="run-meta">Read-only visual preview of exported scene/runtime data (${escapeText(String(width))}×${escapeText(String(height))} · grid ${escapeText(String(gridSize))}px · snap ${snap ? 'on' : 'off'}). Studio produces draft transform operations only; trusted apply requires Safe Source Apply handoff. The browser cannot write trusted files, execute commands, or apply edits.</p>
      <svg class="scene-canvas" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}" role="img" aria-label="Read-only scene preview">${gridLines.join('')}${shapes}</svg>
      <ul class="run-meta-list">${legend}</ul>
    </section>`;
  }

  function renderAssetBrowser(run) {
    const ALLOWED = ['asset-metadata', 'asset_metadata', 'texture-metadata', 'audio-metadata', 'model-metadata', 'generic-metadata'];
    const source = run?.asset_browser || run?.assetBrowser || null;
    if (!source || typeof source !== 'object' || Array.isArray(source) || source.present === false) {
      const empty = source?.empty_state || source?.emptyState || 'No asset browser inputs are exported for this run.';
      return `<section class="panel asset-browser"><h3>Asset browser</h3><p class="empty-state">${escapeText(empty)}</p><p class="run-meta">Read-only. Studio inspects asset manifests and plugin metadata; the browser does not import, generate, upload, fetch, or mutate assets.</p></section>`;
    }
    const pathUnsafe = (value) => {
      const text = String(value == null ? '' : value);
      if (!text) return false;
      return /(^|[\\/])\.\.([\\/]|$)/.test(text) || text.startsWith('/') || /^[A-Za-z]:[\\/]/.test(text) || /[;&|`$<>]/.test(text);
    };
    const rawAssets = Array.isArray(source.assets) ? source.assets : [];
    const hashCounts = {};
    rawAssets.forEach((asset) => { if (asset?.hash) hashCounts[asset.hash] = (hashCounts[asset.hash] || 0) + 1; });
    const cards = rawAssets.map((asset, index) => {
      const id = asset?.id || asset?.asset_id || `asset-${index + 1}`;
      const sourcePath = asset?.source_path || asset?.sourcePath || 'unknown';
      const outputPath = asset?.output_path || asset?.outputPath || 'unknown';
      const status = asset?.status || 'unknown';
      const warnings = [];
      if (status === 'missing' || (outputPath === 'unknown' && status !== 'source-only')) warnings.push('Missing output artifact');
      if (asset?.hash && hashCounts[asset.hash] > 1) warnings.push('Duplicate asset hash');
      if (pathUnsafe(asset?.source_path || asset?.sourcePath)) warnings.push('Unsafe source path');
      if (pathUnsafe(asset?.output_path || asset?.outputPath)) warnings.push('Unsafe output path');
      const metadataRaw = Array.isArray(asset?.metadata) ? asset.metadata : (asset?.metadata ? [asset.metadata] : []);
      const metaRows = metadataRaw.length
        ? metadataRaw.map((descriptor) => {
            const type = String(descriptor?.descriptor_type || descriptor?.descriptorType || descriptor?.type || 'unknown');
            const allowed = ALLOWED.includes(type);
            const plugin = descriptor?.plugin || descriptor?.plugin_id || 'unknown';
            return `<li><span class="${statusClass(allowed ? 'passed' : 'blocked')}">${escapeText(type)}</span> ${escapeText(plugin)}${allowed ? '' : ' — descriptor type not allowlisted'}</li>`;
          }).join('')
        : '<li>No metadata descriptors.</li>';
      const warnRows = warnings.length ? warnings.map((w) => `<li><span class="${statusClass('blocked')}">${escapeText(w)}</span></li>`).join('') : '<li>No diagnostics.</li>';
      return `<article class="artifact asset-card" data-asset-type="${escapeText(asset?.type || 'unknown')}"><h4>${escapeText(id)}</h4>
        <div class="run-meta">${escapeText(asset?.type || 'unknown')} · ${escapeText(status)} · hash ${escapeText(asset?.hash || 'unknown')}</div>
        <div class="run-meta">source ${escapeText(sourcePath)} → output ${escapeText(outputPath)}</div>
        <ul class="run-meta-list">${warnRows}</ul>
        <div class="run-meta">Plugin metadata (read-only):</div><ul class="run-meta-list">${metaRows}</ul>
      </article>`;
    }).join('') || '<div class="run-meta">No assets recorded.</div>';
    return `<section class="panel asset-browser"><h3>Asset browser</h3>
      <p class="run-meta">Read-only inspection of asset manifests and plugin-provided metadata. The browser does not import, generate, upload, fetch remote assets, write manifests, run commands, or apply edits.</p>
      ${cards}
    </section>`;
  }

  return { WORKSPACE_LAYOUT_STORAGE_KEY, WORKSPACE_LAYOUT_VERSION, defaultWorkspaceLayout, normalizeWorkspaceLayout, loadWorkspaceLayout, saveWorkspaceLayout, resetWorkspaceLayout, artifactHref, commandContext, comparisonRefHref, createReplayState, currentReplayView, init, jumpReplayToCheckpoint, renderStudioDiagnosticsSurface, studioDiagnosticsModel, studioErrorBoundary, countBySeverity, studioPerformanceBudget, evaluateStudioPerformanceBudget, renderStudioPerformanceBudgetSurface, renderStudioMultiAgentPipelineInspection, renderAgentRoleModels, renderAgentWorkPackages, renderAgentHandoffs, renderOwnershipPolicies, renderProductionTaskBoards, renderProductionEvidenceBundles, renderReviewCriticGates, renderAnimationVfxSummary, renderAudioEvidenceSummary, renderAssetIntegrity, renderAssetLoading, renderAssetPreview, renderBehaviorEvidenceLifecycle, renderPluginRegistry, renderEvaluatorDepthInspection, renderRuntimeInvariants, renderRuntimeProfilerSummary, renderRouteAttempts, renderVisualComparisons, renderFuzzingPlans, renderQaAgentWorkQueues, renderPerformanceRegressionLanes, renderSourceApplyWorktreeContext, renderSourceApplyHandoff, renderAssetBrowser, renderSceneCanvas, renderDraftOperationModel, renderEntityComponentInspector, renderSourcePatchEvidenceBundles, renderSourcePatchApplyTransactions, renderSourcePatchStaleTargetGuards, renderCameraLayerSummary, renderCategorySummary, renderCommandContext, renderGameplaySummary, renderInputActionSummary, renderRenderBreakdownSummary, renderTilemapSummary, renderJournalViewer, renderLoopDryRunSummary, renderLoopExecutionSummary, renderLoopEvidenceBundles, renderLoopRecoveryStatus, renderMutationLifecycle, renderProposalRationaleList, renderProbeContractStatus, renderProjectContext, renderQaScenarioCandidates, renderQaWorkerAssignments, renderRegressionMatrix, renderRegressionPromotions, renderReplayControls, renderRunComparison, renderSceneTreeInspector, studioCommandRegistry, filterStudioCommands, isBlockedStudioCommand, resolveStudioCommand, renderStudioCommandPaletteSurface, renderRunDetail, renderRunDetailWithState, renderRunList, renderSemanticDiffSummary, renderStudioAccessibilityNavSurface, renderTransactionProvenance, resetReplay, runRelativeHref, statusClass, stepReplayForward, studioKeyboardNavModel, nextStudioFocus, restoreStudioFocus, summarizeRun };
})();

if (typeof window !== 'undefined') {
  window.OuroforgeDashboard = OuroforgeDashboard;
  window.addEventListener('DOMContentLoaded', () => OuroforgeDashboard.init?.());
}

if (typeof module !== 'undefined') {
  module.exports = OuroforgeDashboard;
}
