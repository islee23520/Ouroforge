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

  function renderRunDetail(run) {
    return renderRunDetailWithState(run, createReplayState(run), run?.regression_matrix || run?.regressionMatrix || null);
  }

  function renderRunDetailWithState(run, replayState, regressionMatrix = null) {
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
      <section class="panel"><h3>Verdict summary</h3><pre>${escapeText(JSON.stringify(verdict, null, 2))}</pre></section>
      ${renderCommandContext(run)}
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
        detailEl.innerHTML = renderRunDetailWithState(selected, replayStateFor(selected), data.regression_matrix || data.regressionMatrix || null);
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

  return { artifactHref, commandContext, comparisonRefHref, createReplayState, currentReplayView, init, jumpReplayToCheckpoint, renderCategorySummary, renderCommandContext, renderJournalViewer, renderMutationLifecycle, renderProposalRationaleList, renderProbeContractStatus, renderProjectContext, renderRegressionMatrix, renderRegressionPromotions, renderReplayControls, renderRunComparison, renderRunDetail, renderRunDetailWithState, renderRunList, renderSemanticDiffSummary, renderTransactionProvenance, resetReplay, runRelativeHref, statusClass, stepReplayForward, summarizeRun };
})();

if (typeof window !== 'undefined') {
  window.OuroforgeDashboard = OuroforgeDashboard;
  window.addEventListener('DOMContentLoaded', () => OuroforgeDashboard.init?.());
}

if (typeof module !== 'undefined') {
  module.exports = OuroforgeDashboard;
}
