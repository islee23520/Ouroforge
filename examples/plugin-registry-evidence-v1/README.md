# Plugin Registry Evidence v1

Fixture-scoped plugin registry evidence for plugin/extension milestones. The
schema records manifest hashes, validation/compatibility status, declared
allowlisted capabilities, extension points, evidence refs, blocked reasons, and
v1 declarative read-only dashboard panel descriptors.

Dashboard panel descriptors are data only: panel id, title, allowlisted data
source key, allowlisted template/component reference, and layout/display hints.
They do not execute plugins, arbitrary JavaScript, event handlers, remote assets,
commands, native extensions, or trusted writes.

This is declarative evidence only: it does not execute plugins, install or update
packages, load native extensions, run shell commands, mutate trusted source,
publish/deploy, or grant dashboard/Studio write authority.

PES10.7.2 writes validated registry evidence only under a run evidence tree
(`evidence/plugins/<registry-id>.json`) and appends an `evidence/index.json` entry
for dashboard/Studio inspection; duplicate evidence ids/paths are blocked before
rewriting.

PES10.9.1 adds the dashboard panel descriptor contract and fixture examples while
keeping generated runtime registry/evidence outputs ignored unless explicitly
fixture-scoped.
