# Project Scaffold v1

Project Scaffold v1 adds a bounded Rust CLI command for creating a tiny local
Ouroforge project workspace from deterministic built-in source files. It exists
to make the Project Workspace Loop v1 authoring path reproducible without adding
a production editor, browser persistence, external template packages, native
export, plugins, or hosted services.

## Command

```bash
cargo run -p ouroforge-cli -- project init .omx/tmp/project-scaffold-smoke --template minimal-2d
```

The destination may be a new directory or an existing empty directory. The
command refuses:

- destinations containing `..` traversal;
- destinations that already exist as files;
- non-empty destination directories;
- unsupported template names.

Only `minimal-2d` is supported.

## Generated file tree

`project init <destination> --template minimal-2d` writes:

```text
<destination>/
  ouroforge.project.json
  scenes/main.scene.json
  seeds/platformer.yaml
  scenarios/smoke.scenario-pack.json
  assets/README.md
  README.md
  .gitignore
```

These files are starter project source files. They are different from local run
output and may be committed by the user in their own project repository. Smoke
directories generated inside this repository, such as `.omx/tmp/project-scaffold-smoke`,
are verification output and must remain untracked/cleaned.

## Immediate validation

After writing files, the CLI validates:

- the generated `ouroforge.project.json` through Rust Project Manifest v1;
- the generated Seed at `seeds/platformer.yaml`;
- the generated scene at `scenes/main.scene.json`.

Manual validation commands:

```bash
cargo run -p ouroforge-cli -- project validate .omx/tmp/project-scaffold-smoke/ouroforge.project.json
cargo run -p ouroforge-cli -- seed validate .omx/tmp/project-scaffold-smoke/seeds/platformer.yaml
```

The manifest summary should report project id `minimal_2d`, three source refs,
one asset root, runs root `runs`, and generated roots `runs,target,dashboard-data`.

## Generated-state policy

The scaffolded `.gitignore` marks local/generated state as untracked:

```gitignore
runs/
target/
dashboard-data/
.openchrome/
.omc/
.omx/
.claude/
```

In the Ouroforge repository itself, generated scaffold smoke output under
`.omx/tmp/` must be removed after verification and must not be committed.

## Relation to existing commands

The scaffolded Seed is compatible with existing non-project validation:

```bash
cargo run -p ouroforge-cli -- seed validate <project>/seeds/platformer.yaml
```

Project-bound run metadata is intentionally not implemented by scaffold. Until
#249 adds project run binding, regular run commands remain seed-based:

```bash
cargo run -p ouroforge-cli -- run <project>/seeds/platformer.yaml
```

Those runs still write to the current repository-level `runs/` default unless a
later project-run command changes that behavior.

## Non-goals

Project Scaffold v1 does not add:

- general template engines;
- external package managers or dependency installation;
- native export or packaging;
- plugin runtime or marketplace behavior;
- hosted/cloud/server/database/auth infrastructure;
- distributed QA/Elixir behavior;
- browser-side trusted writes or command bridges;
- project-bound run metadata;
- project comparison;
- mutation application;
- Studio v3 UI;
- production editor or Godot replacement claims.
