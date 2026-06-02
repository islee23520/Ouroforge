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
    return {
      id: summary.id,
      seed: summary.seed_id,
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
      return `<button class="run-button${active}" data-run-id="${escapeText(summary.id)}">
        <div class="run-id">${escapeText(summary.id)}</div>
        <div class="run-meta">${escapeText(summary.seed)} · ${summary.evidenceCount} evidence · ${summary.mutationCount} mutations · ${summary.workerCount} workers</div>
        <div class="run-status-row">
          <span class="${statusClass(summary.runStatus)}">run ${escapeText(summary.runStatus)}</span>
          <span class="${statusClass(summary.verdict)}">verdict ${escapeText(summary.verdict)}</span>
          <span class="${statusClass(summary.scenario)}">scenario ${escapeText(summary.scenario)}</span>
        </div>
      </button>`;
    }).join('');
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
      ${missing}${readError}
    </article>`;
  }

  function renderScreenshot(artifact, run) {
    return `<article class="artifact">
      <a href="${escapeText(artifactHref(artifact, run))}" target="_blank" rel="noreferrer">${escapeText(artifact.id)}</a>
      <img class="screenshot" src="${escapeText(artifactHref(artifact, run))}" alt="${escapeText(artifact.id)}" />
    </article>`;
  }

  function renderJsonArtifact(artifact, run) {
    const preview = artifact.value === undefined || artifact.value === null
      ? '<p class="empty-state compact">No JSON preview available.</p>'
      : `<pre>${escapeText(JSON.stringify(artifact.value, null, 2))}</pre>`;
    return `<article class="artifact">
      <a href="${escapeText(artifactHref(artifact, run))}" target="_blank" rel="noreferrer">${escapeText(artifact.id)}</a>
      <div class="run-meta">${escapeText(artifact.path)}</div>
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

  function renderRefLinks(title, refs, run, kind = 'artifact') {
    if (!Array.isArray(refs) || !refs.length) return '';
    const links = refs.map((ref) => {
      const href = kind === 'mutation' ? null : runRelativeHref(ref, run);
      return href
        ? `<a class="ref-chip" href="${escapeText(href)}" target="_blank" rel="noreferrer">${escapeText(ref)}</a>`
        : `<span class="ref-chip">${escapeText(ref)}</span>`;
    }).join('');
    return `<div class="ref-group"><div class="card-label">${escapeText(title)}</div><div class="ref-list">${links}</div></div>`;
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

  function renderMutationLifecycle(run) {
    const lifecycle = run?.mutation_lifecycle;
    if (!lifecycle) {
      return `<section class="panel"><h3>Mutation Review</h3><p class="empty-state">No mutation lifecycle read model is available. Export dashboard data with the latest Rust CLI.</p></section>`;
    }
    const stages = Array.isArray(lifecycle.stages) ? lifecycle.stages : [];
    const stageCards = stages.length ? stages.map((stage) => `<article class="lifecycle-card">
      <div class="journal-entry-header">
        <h4>${escapeText(stage.label || stage.id)}</h4>
        <span class="${statusClass(stage.state)}">${escapeText(stage.state || 'missing')}</span>
      </div>
      <div class="run-meta">${escapeText(stage.artifact_path || 'No artifact path')}</div>
      <div class="run-meta">${escapeText(stage.record_count ?? 0)} record(s)</div>
      ${stage.read_error ? `<div class="artifact-warning">${escapeText(stage.read_error)}</div>` : ''}
      ${renderRefLinks('Evidence refs', stage.evidence_refs, run)}
      ${Array.isArray(stage.records) && stage.records.length ? `<pre>${escapeText(JSON.stringify(stage.records, null, 2))}</pre>` : '<p class="empty-state compact">No lifecycle records for this stage.</p>'}
    </article>`).join('') : '<p class="empty-state">No mutation lifecycle stages are available.</p>';
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
      <div class="lifecycle-grid">${stageCards}</div>
    </section>`;
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
    const checkpoint = checkpoints.find((item) => Number(item.frame ?? item.tick) === frame) || checkpoints[0] || null;
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
    return renderRunDetailWithState(run, createReplayState(run));
  }

  function renderRunDetailWithState(run, replayState) {
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
      <section class="panel"><h3>Verdict summary</h3><pre>${escapeText(JSON.stringify(verdict, null, 2))}</pre></section>
      ${renderJournalViewer(run)}
      ${renderMutationLifecycle(run)}
      ${renderReplayControls(run, replayState)}
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
        detailEl.innerHTML = renderRunDetailWithState(selected, replayStateFor(selected));
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

  return { artifactHref, createReplayState, currentReplayView, init, jumpReplayToCheckpoint, renderCategorySummary, renderJournalViewer, renderMutationLifecycle, renderReplayControls, renderRunDetail, renderRunDetailWithState, renderRunList, resetReplay, runRelativeHref, statusClass, stepReplayForward, summarizeRun };
})();

if (typeof window !== 'undefined') {
  window.OuroforgeDashboard = OuroforgeDashboard;
  window.addEventListener('DOMContentLoaded', () => OuroforgeDashboard.init?.());
}

if (typeof module !== 'undefined') {
  module.exports = OuroforgeDashboard;
}
