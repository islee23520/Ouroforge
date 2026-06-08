'use strict';

function createSteamworksBridge(steamworks) {
  const available = Boolean(steamworks && steamworks.available);
  return {
    available,
    openOverlay(target) {
      if (!available) return { status: 'disabled', reason: 'no-steam', target };
      return { status: 'requested', target };
    },
    unlockAchievement(achievementId, trustedEvidenceRef) {
      if (!available) return { status: 'local-only', achievementId, trustedEvidenceRef };
      return { status: 'requested', achievementId, trustedEvidenceRef };
    },
    syncCloudSave(saveRef) {
      if (!available) return { status: 'local-only', saveRef };
      return { status: 'requested', saveRef };
    },
    submitDailySeedScore(payload) {
      if (!available) return { status: 'disabled', reason: 'no-steam', payload };
      return { status: 'requested', payload };
    }
  };
}

module.exports = { createSteamworksBridge };
