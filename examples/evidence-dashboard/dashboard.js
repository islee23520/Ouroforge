const OuroforgeDashboard = (() => {
  function statusClass(status) {
    return `status status-${String(status || 'unknown').toLowerCase()}`;
  }

  function artifactHref(artifact, run) {
    const runDir = run?.summary?.run_dir || run?.summary?.runDir || '';
    return `../../${runDir}/${artifact.path}`;
  }

  function summarizeRun(run) {
    return {
      id: run.summary.id,
      seed: run.summary.seed_id,
      verdict: run.summary.verdict_status,
      evidenceCount: run.summary.evidence_count,
      mutationCount: run.summary.mutation_count,
    };
  }

  function escapeText(value) {
    return String(value ?? '');
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
        <div class="run-meta">${escapeText(summary.seed)} · ${summary.evidenceCount} evidence · ${summary.mutationCount} mutations</div>
        <span class="${statusClass(summary.verdict)}">${escapeText(summary.verdict)}</span>
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
    return `<article class="artifact">
      <a href="${artifactHref(artifact, run)}" target="_blank" rel="noreferrer">${escapeText(artifact.id)}</a>
      <div class="run-meta">${escapeText(artifact.kind)}</div>
      <div class="run-meta">${escapeText(artifact.path)}</div>
    </article>`;
  }

  function renderScreenshot(artifact, run) {
    return `<article class="artifact">
      <a href="${artifactHref(artifact, run)}" target="_blank" rel="noreferrer">${escapeText(artifact.id)}</a>
      <img class="screenshot" src="${artifactHref(artifact, run)}" alt="${escapeText(artifact.id)}" />
    </article>`;
  }

  function renderJsonArtifact(artifact, run) {
    return `<article class="artifact">
      <a href="${artifactHref(artifact, run)}" target="_blank" rel="noreferrer">${escapeText(artifact.id)}</a>
      <pre>${escapeText(JSON.stringify(artifact.value, null, 2))}</pre>
    </article>`;
  }

  function renderRunDetail(run) {
    if (!run) return '<div class="empty-state">Select a run to inspect its evidence.</div>';
    const verdict = run.verdict || {};
    return `<article>
      <h2>${escapeText(run.summary.id)}</h2>
      <div class="cards">
        <div class="card"><div class="card-label">Seed</div><div class="card-value">${escapeText(run.summary.seed_id)}</div></div>
        <div class="card"><div class="card-label">Verdict</div><div class="card-value"><span class="${statusClass(run.summary.verdict_status)}">${escapeText(run.summary.verdict_status)}</span></div></div>
        <div class="card"><div class="card-label">Evidence</div><div class="card-value">${run.evidence.length}</div></div>
        <div class="card"><div class="card-label">Mutations</div><div class="card-value">${run.mutations.length}</div></div>
      </div>
      <section class="panel"><h3>Verdict summary</h3><pre>${escapeText(JSON.stringify(verdict, null, 2))}</pre></section>
      <section class="panel"><h3>Journal</h3><pre>${escapeText(run.journal)}</pre></section>
      ${renderArtifacts('Screenshots', run.screenshots, run, renderScreenshot)}
      ${renderArtifacts('World state', run.world_states, run, renderJsonArtifact)}
      ${renderArtifacts('Console logs', run.console_logs, run, renderJsonArtifact)}
      ${renderArtifacts('Mutation proposals', run.mutations.map((mutation) => ({ id: mutation.id, kind: 'mutation/proposal', path: mutation.evidence_id, value: mutation })), run, renderJsonArtifact)}
      ${renderArtifacts('Evidence index', run.evidence, run, renderArtifactLink)}
    </article>`;
  }

  async function init() {
    const listEl = document.getElementById('run-list');
    const detailEl = document.getElementById('run-detail');
    try {
      const response = await fetch('dashboard-data.json', { cache: 'no-store' });
      if (!response.ok) throw new Error(`failed to load dashboard-data.json: ${response.status}`);
      const data = await response.json();
      const runs = data.runs || [];
      let selected = runs[0] || null;
      const paint = () => {
        listEl.innerHTML = renderRunList(runs, selected && selected.summary.id);
        detailEl.innerHTML = renderRunDetail(selected);
        listEl.querySelectorAll('[data-run-id]').forEach((button) => {
          button.addEventListener('click', () => {
            selected = runs.find((run) => run.summary.id === button.dataset.runId) || null;
            paint();
          });
        });
      };
      paint();
    } catch (error) {
      listEl.innerHTML = `<div class="empty-state">${escapeText(error.message)}</div>`;
    }
  }

  return { artifactHref, init, renderRunDetail, renderRunList, statusClass, summarizeRun };
})();

if (typeof window !== 'undefined') {
  window.OuroforgeDashboard = OuroforgeDashboard;
  window.addEventListener('DOMContentLoaded', () => OuroforgeDashboard.init?.());
}

if (typeof module !== 'undefined') {
  module.exports = OuroforgeDashboard;
}
