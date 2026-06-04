# Source File Class v1 fixtures

Tracked source-like fixtures for #356 SMP1.2.1 through SMP1.2.3. These examples exercise the
Source Mutation Preview v1 classifier only; they do not create patch previews,
run sandbox evaluation, apply source patches, merge branches, execute commands,
or write generated artifacts.

`classification-cases.json` records deterministic path examples for:

- potentially allowed source-like data (`allowed`);
- restricted classes that need explicit review approval (`needs-approval`);
- blocked dependency, CI/workflow, script, generated/local, hidden-root, unsafe,
  and opaque classes (`blocked`).

Generated preview, sandbox, report, dashboard, and run outputs remain ignored
local state unless a later issue explicitly scopes a tiny source-like fixture.
#1 and #23 remain open as governance/context anchors.

See `../../docs/source-file-class-validator-v1.md` for the report shape and review expectations.
