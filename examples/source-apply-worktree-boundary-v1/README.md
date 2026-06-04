# Source Apply Worktree Boundary v1 Fixture

`worktree-context-policy.sample.json` is tracked fixture metadata for #701
SA15.3.1. It describes policy-only worktree boundaries and blocked context
states. It does not execute commands, apply patches, write trusted files, create
locks, inspect Git state, or grant source apply authority.

Focused smoke:

```bash
node -e "const fs=require('fs'); const p='examples/source-apply-worktree-boundary-v1/worktree-context-policy.sample.json'; const v=JSON.parse(fs.readFileSync(p,'utf8')); if (v.schemaVersion !== 'source-apply-worktree-boundary-policy-v1') throw new Error('bad schema'); if (!v.blockedContextStates.includes('dirty-target')) throw new Error('missing dirty-target'); console.log('worktree boundary fixture smoke passed')"
```
