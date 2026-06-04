# Source Patch Test Command Allowlist v1 fixture

This fixture is inert policy data for source patch preview evaluation. It records
known local verification commands as `argv` arrays so sandbox dry-run work can
match exact or prefix-safe commands without shell parsing.

The fixture itself does not execute commands, apply patches, create sandboxes,
mutate dependencies, use the network, read credentials, write trusted source
files, or bridge browser/UI actions to the local shell. Execution is limited to
the Rust sandbox evaluator, which runs matched argv vectors from the generated
sandbox worktree and records generated evidence only.

SMP1.6 sandbox execution uses this policy only after preview validation and
sandbox isolation. Matching a command here is necessary but not sufficient for
trusted authority: the evaluator still requires sandbox worktree containment,
policy-id metadata, forbidden-command rejection, bounded evidence capture, and
no generated artifacts committed as source.
