# Plugin Registry Evidence v1

Fixture-scoped plugin registry evidence for #744. The schema records manifest
hashes, validation/compatibility status, declared allowlisted capabilities,
extension points, evidence refs, and blocked reasons.

This is declarative evidence only: it does not execute plugins, install or update
packages, load native extensions, run shell commands, mutate trusted source,
publish/deploy, or grant dashboard/Studio write authority.

PES10.7.2 writes validated registry evidence only under a run evidence tree
(`evidence/plugins/<registry-id>.json`) and appends an `evidence/index.json` entry
for dashboard/Studio inspection; duplicate evidence ids/paths are blocked before
rewriting.
