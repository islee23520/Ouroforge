# GDD-to-Prototype Demo v1

Issue: #659

This demo composes the accepted GDD-to-prototype v1 contracts into one tiny deterministic fixture: `examples/gdd-to-prototype-demo-v1/tiny-collect-gdd.md` plus `demo.manifest.fixture.json`. It demonstrates a bounded local path from GDD fixture to requirement extraction, mechanics/core-loop mapping, feasibility, scaffold/scene/behavior/asset/scenario plans, task graph, draft bundle, review/apply status, run evidence, evidence/journal bundle, and Studio read-only inspection compatibility.

## Reproducible checks

```bash
cargo test -p ouroforge-core --test gdd_to_prototype_demo_contract -- --nocapture
node examples/authoring-cockpit/cockpit.test.cjs
```

The commands above are documentation and test evidence. Browser/Studio surfaces may display copyable command text, but it is inert text only and does not execute from the browser.

## Expected evidence

- GDD, extracted requirements, mechanics mapping, feasibility, plans, drafts, task graph, review/apply, run evidence, and journal artifacts remain separate.
- Requirement coverage links back to run evidence and journal summary.
- Dashboard/Studio compatibility is read-only/draft-only.
- Generated prototype drafts, runs, screenshots, dashboard exports, temp projects, and local tool state remain untracked unless explicitly fixture-scoped.
- Asset notes use placeholders/local fixtures/manifest references only; no generated copyrighted or proprietary assets are introduced.

## Known gaps and unsupported cases

- The demo is one deterministic bounded fixture, not arbitrary GDD support.
- Unsupported mechanics or missing run output remain explicit evidence gaps.
- The browser does not write files, run generation, apply changes, merge branches, execute commands, or persist trusted state.

## Boundaries

No autonomous unrestricted game creation, arbitrary source mutation, arbitrary script execution, browser trusted writes, command bridge, local server bridge, hidden command execution, auto-apply, auto-merge, self-approval, uncontrolled asset generation, production game, shipped-game, commercial readiness, production-ready engine, current Godot replacement, native export, plugin runtime, marketplace, hosted/cloud/server/auth/account behavior, public launch, or autonomous launch claim is added.

#1 remains open. #23 remains open.
