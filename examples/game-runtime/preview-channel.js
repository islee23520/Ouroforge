(function attachPreviewChannel(root) {
  // Preview channel delta application for Live Preview Loop v1 (M131.2,
  // Era X #2519). Deltas arrive PRE-VALIDATED and fully normalized from the
  // Rust preview session (`ouroforge preview serve`, see
  // docs/preview-session-v1.md). This module applies them MECHANICALLY to the
  // running world: a 1:1 mapping from each allowlisted scene-edit path onto
  // the live entity shape, with no interpretation, defaulting, clamping, or
  // semantic decisions. Anything unexpected is a diagnostic, never a guess.
  // The channel is receive-only: it originates no writes and no intents.

  const PREVIEW_DELTA_SCHEMA_VERSION = 'ouroforge.preview-delta.v1';

  function failure(error, diagnostic) {
    return { ok: false, error, diagnostic };
  }

  function isPlainObject(value) {
    return Boolean(value) && typeof value === 'object' && !Array.isArray(value);
  }

  function requireInteger(value, path) {
    if (!Number.isInteger(value)) {
      return `preview delta value for ${path} must be an integer`;
    }
    return null;
  }

  // Each entry mirrors apply_scene_edit in ouroforge-core-types exactly:
  // same allowlist, same required-component failures, same value types.
  const EDIT_APPLIERS = {
    'sprite.color': (entity, value) => {
      if (typeof value !== 'string') return 'preview delta value for sprite.color must be a string';
      return () => {
        entity.sprite.color = value;
      };
    },
    'components.transform.x': integerApplier((entity) => entity.components.transform, 'x'),
    'components.transform.y': integerApplier((entity) => entity.components.transform, 'y'),
    'components.velocity.x': integerApplier((entity) => entity.components.velocity, 'x'),
    'components.velocity.y': integerApplier((entity) => entity.components.velocity, 'y'),
    'components.size.width': integerApplier((entity) => entity.components.size, 'width'),
    'components.size.height': integerApplier((entity) => entity.components.size, 'height'),
    'components.controllable': (entity, value) => {
      if (typeof value !== 'boolean') {
        return 'preview delta value for components.controllable must be a boolean';
      }
      return () => {
        entity.components.controllable = value;
      };
    },
    'components.status.hitPoints': optionalIntegerApplier(
      (entity) => entity.components.status,
      'hitPoints',
      'status'
    ),
    'components.status.maxHitPoints': optionalIntegerApplier(
      (entity) => entity.components.status,
      'maxHitPoints',
      'status'
    ),
    'components.input.moveSpeed': optionalIntegerApplier(
      (entity) => entity.components.input,
      'moveSpeed',
      'input'
    ),
    'components.input.jumpImpulse': optionalIntegerApplier(
      (entity) => entity.components.input,
      'jumpImpulse',
      'input'
    ),
    'components.cameraTarget.weight': optionalIntegerApplier(
      (entity) => entity.components.cameraTarget,
      'weight',
      'cameraTarget'
    ),
    'components.uiText.text': (entity, value) => {
      if (typeof value !== 'string') {
        return 'preview delta value for components.uiText.text must be a string';
      }
      if (!entity.components.uiText) {
        return 'preview delta path components.uiText.text requires uiText component';
      }
      return () => {
        entity.components.uiText.text = value;
      };
    },
  };

  function integerApplier(target, field) {
    return (entity, value, path) => {
      const error = requireInteger(value, path);
      if (error) return error;
      return () => {
        target(entity)[field] = value;
      };
    };
  }

  function optionalIntegerApplier(target, field, componentName) {
    return (entity, value, path) => {
      const error = requireInteger(value, path);
      if (error) return error;
      if (!target(entity)) {
        return `preview delta path ${path} requires ${componentName} component`;
      }
      return () => {
        target(entity)[field] = value;
      };
    };
  }

  function stageEdit(world, edit) {
    if (!isPlainObject(edit) || typeof edit.entityId !== 'string' || typeof edit.path !== 'string') {
      return { error: 'preview delta edit is malformed' };
    }
    const entity = world.entities.find((candidate) => candidate.id === edit.entityId);
    if (!entity) {
      return { error: `preview delta entity not found: ${edit.entityId}` };
    }
    const applier = EDIT_APPLIERS[edit.path];
    if (!applier) {
      return { error: `preview delta path is not allowed: ${edit.path}` };
    }
    const staged = applier(entity, edit.value, edit.path);
    if (typeof staged === 'string') return { error: staged };
    return { commit: staged, entityId: edit.entityId };
  }

  // Apply one normalized delta to the live world. All-or-nothing: every edit
  // is staged before any commit, so a malformed edit leaves the world
  // untouched. Returns:
  //   { ok: true, appliedEdits, entityIds }            - edits committed
  //   { ok: true, requiresSceneReload: true }          - caller must reload
  //   { ok: true, skipped: 'rejected' }                - rejected deltas are
  //     informational; the channel never applies them
  //   { ok: false, error, diagnostic }                 - surface as runtime
  //     diagnostic, never silently
  function applyPreviewDelta(world, delta) {
    if (!isPlainObject(world) || !Array.isArray(world.entities)) {
      return failure('preview delta target world is malformed', 'preview_delta_apply_failed');
    }
    if (!isPlainObject(delta)) {
      return failure('preview delta is not an object', 'preview_delta_schema_unsupported');
    }
    if (delta.schemaVersion !== PREVIEW_DELTA_SCHEMA_VERSION) {
      return failure(
        `preview delta schemaVersion must be ${PREVIEW_DELTA_SCHEMA_VERSION}`,
        'preview_delta_schema_unsupported'
      );
    }
    if (delta.status === 'rejected') {
      return { ok: true, skipped: 'rejected', appliedEdits: 0 };
    }
    if (delta.status !== 'applied') {
      return failure(
        `preview delta status must be applied or rejected, found ${String(delta.status)}`,
        'preview_delta_schema_unsupported'
      );
    }
    if (delta.kind === 'sceneReload') {
      return { ok: true, requiresSceneReload: true, appliedEdits: 0 };
    }
    if (!Array.isArray(delta.edits) || delta.edits.length === 0) {
      return failure('preview delta has no edits to apply', 'preview_delta_apply_failed');
    }
    const staged = [];
    for (const edit of delta.edits) {
      const result = stageEdit(world, edit);
      if (result.error) return failure(result.error, 'preview_delta_apply_failed');
      staged.push(result);
    }
    for (const entry of staged) entry.commit();
    return {
      ok: true,
      appliedEdits: staged.length,
      entityIds: staged.map((entry) => entry.entityId),
    };
  }

  const api = Object.freeze({
    PREVIEW_DELTA_SCHEMA_VERSION,
    applyPreviewDelta,
    supportedDeltaPaths: Object.freeze(Object.keys(EDIT_APPLIERS)),
  });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgePreviewChannel = api;
})(typeof window !== 'undefined' ? window : globalThis);
