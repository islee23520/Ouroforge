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

  function cloneScene(scene) {
    return JSON.parse(JSON.stringify(scene));
  }

  function getValue(entity, path) {
    return path.split('.').reduce((value, part) => value && value[part], entity);
  }

  function coerceValue(raw, kind) {
    if (kind === 'number') return Number(raw);
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

  function renderTree(scene, selectedId) {
    return scene.entities.map((entity) => `<button class="tree-button ${entity.id === selectedId ? 'active' : ''}" data-entity-id="${entity.id}">${entity.id}<br><small>${entity.components.controllable ? 'controllable' : 'static'}</small></button>`).join('');
  }

  function renderInspector(scene, entityId, scenePath = 'examples/game-runtime/scene.json') {
    const entity = scene.entities.find((candidate) => candidate.id === entityId);
    if (!entity) return '<div class="empty">Select an entity to inspect supported properties.</div>';
    const fields = EDITABLE_FIELDS.map(([path, kind]) => {
      const value = getValue(entity, path);
      const input = kind === 'boolean'
        ? `<select data-edit-path="${path}"><option value="true" ${value ? 'selected' : ''}>true</option><option value="false" ${!value ? 'selected' : ''}>false</option></select>`
        : `<input data-edit-path="${path}" type="${kind === 'number' ? 'number' : 'text'}" value="${value}" />`;
      return `<label>${path}${input}</label>`;
    }).join('');
    return `<div class="inspector"><div class="panel"><h2>${entity.id}</h2><div class="field-grid">${fields}</div></div><div class="panel"><h3>Current component JSON</h3><pre>${JSON.stringify(entity, null, 2)}</pre></div><div class="panel"><h3>Validated write path</h3><pre id="edit-command">${cliCommand(scenePath, entity.id, 'components.transform.x', entity.components.transform.x)}</pre></div></div>`;
  }

  async function init() {
    const treeEl = document.getElementById('scene-tree');
    const inspectorEl = document.getElementById('inspector');
    let scene = await fetch('../game-runtime/scene.json').then((response) => response.json());
    let selectedId = scene.entities[0]?.id;
    const paint = () => {
      treeEl.innerHTML = renderTree(scene, selectedId);
      inspectorEl.innerHTML = renderInspector(scene, selectedId);
      treeEl.querySelectorAll('[data-entity-id]').forEach((button) => button.addEventListener('click', () => { selectedId = button.dataset.entityId; paint(); }));
      inspectorEl.querySelectorAll('[data-edit-path]').forEach((input) => input.addEventListener('change', () => {
        const path = input.dataset.editPath;
        scene = applyEdit(scene, selectedId, path, input.value);
        const entity = scene.entities.find((candidate) => candidate.id === selectedId);
        document.getElementById('edit-command').textContent = cliCommand('examples/game-runtime/scene.json', selectedId, path, getValue(entity, path));
      }));
    };
    paint();
  }

  return { EDITABLE_FIELDS, applyEdit, cliCommand, getValue, init, renderInspector, renderTree, validateEdit };
})();

if (typeof window !== 'undefined') {
  window.OuroforgeCockpit = OuroforgeCockpit;
  window.addEventListener('DOMContentLoaded', () => OuroforgeCockpit.init());
}

if (typeof module !== 'undefined') {
  module.exports = OuroforgeCockpit;
}
