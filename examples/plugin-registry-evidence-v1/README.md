# Plugin Registry Evidence v1

Fixture-scoped plugin registry evidence for #744. The schema records manifest
hashes, validation/compatibility status, declared allowlisted capabilities,
extension points, evidence refs, and blocked reasons.

This is declarative evidence only: it does not execute plugins, install or update
packages, load native extensions, run shell commands, mutate trusted source,
publish/deploy, or grant dashboard/Studio write authority.
