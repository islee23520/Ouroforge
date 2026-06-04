# Source Patch Test Command Allowlist v1 fixture

This fixture is inert policy data for source patch preview evaluation. It records
known local verification commands as `argv` arrays so later sandbox dry-run work
can match exact or prefix-safe commands without shell parsing.

It does not execute commands, apply patches, create sandboxes, mutate
dependencies, use the network, read credentials, write trusted source files, or
bridge browser/UI actions to the local shell.
