(function attachSnapshots(root) {
  function clone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function createSnapshotRegistry() {
    const snapshots = new Map();
    let nextId = 1;

    function capture(state, tick = 0) {
      const snapshotId = `snapshot-${String(nextId).padStart(4, '0')}`;
      nextId += 1;
      const snapshot = {
        snapshotId,
        tick,
        capturedAtTick: tick,
        state: clone(state),
      };
      snapshots.set(snapshotId, snapshot);
      return snapshotId;
    }

    function restore(snapshotId) {
      if (typeof snapshotId !== 'string' || snapshotId.trim() === '') {
        throw new Error('snapshotId is required');
      }
      const snapshot = snapshots.get(snapshotId);
      if (!snapshot) throw new Error(`snapshot not found: ${snapshotId}`);
      return clone(snapshot.state);
    }

    function metadata(snapshotId) {
      const snapshot = snapshots.get(snapshotId);
      if (!snapshot) return null;
      return {
        snapshotId: snapshot.snapshotId,
        tick: snapshot.tick,
        capturedAtTick: snapshot.capturedAtTick,
      };
    }

    function list() {
      return Array.from(snapshots.values()).map((snapshot) => metadata(snapshot.snapshotId));
    }

    return Object.freeze({ capture, restore, metadata, list });
  }

  const api = Object.freeze({ createSnapshotRegistry });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  root.OuroforgeSnapshots = api;
})(typeof window !== 'undefined' ? window : globalThis);
