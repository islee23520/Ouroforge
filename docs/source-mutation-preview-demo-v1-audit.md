# Source Mutation Preview Demo v1 display audit

This audit closes the SMP1.10.3 documentation unit for #364. It describes how the demo preview, sandbox plan, and evidence summary may be displayed in Studio and dashboard surfaces without adding apply, merge, write, or command authority.

## Display contract

Studio and dashboard surfaces may show:

- preview id `smp1-10-demo-preview-001`;
- file-class, risk, stale-target, sandbox-plan, and evidence-summary refs;
- dry-run status summaries and allowlisted command text;
- generated sandbox report refs as local review evidence;
- forbidden action notices such as `apply_patch`, `merge_branch`, `execute_command`, `write_trusted_file`, and `browser_command_bridge`.

Studio and dashboard surfaces must not provide:

- source patch apply controls for the trusted worktree;
- merge, auto-merge, auto-accept, or branch automation;
- browser trusted writes, uploads, command bridges, local server bridges, or hidden command execution;
- arbitrary shell, network, install, dependency, credential, build-script, CI, or workflow mutation scope;
- public launch automation or production isolation claims.

## Demo artifact mapping

| Demo artifact | Display treatment | Guardrail |
| --- | --- | --- |
| `examples/source-mutation-preview-demo-v1/patch-preview-demo.sample.json` | Read-only patch preview and risk summary | `sourceMutationApplyStatus` remains `blocked`. |
| `examples/source-mutation-preview-demo-v1/sandbox-dry-run-plan.sample.json` | Read-only dry-run plan and cleanup policy | Plan metadata does not execute commands. |
| `examples/source-mutation-preview-demo-v1/sandbox-dry-run-evidence-summary.sample.json` | Read-only evidence ids and generated report refs | Generated reports remain untracked. |

## Wording guardrails

Use conservative wording:

- "preview-only" instead of "auto-fix";
- "sandbox dry-run evidence" instead of production-sandbox guarantee language;
- "read-only display" instead of "editor apply surface";
- "copyable/allowlisted command text" instead of "browser command execution";
- "generated local evidence" instead of "committed report output".

## Closure audit for #364

Before closing #364, verify that:

1. SMP1.10.1, SMP1.10.2, and SMP1.10.3 are merged in order.
2. Latest `main` has been pulled.
3. Rust preview/sandbox tests, Node dashboard/cockpit tests, and broad gates pass.
4. Generated `sandbox/`, `runs/`, `target/`, `.omx/`, `.omc/`, `.openchrome/`, and `.claude/` state remains untracked.
5. #1 and #23 remain open.
