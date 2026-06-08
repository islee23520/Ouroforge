# Deckbuilder UI v1

Issue: #1826

Deckbuilder UI v1 adds a card/hand/pipeline read model to the existing JS game runtime. It reuses the current deck-roguelike substrate and `window.__OUROFORGE__` probe; it does not introduce a UI framework, a parallel engine, browser-side trusted writes, command bridge, auto-apply, or auto-merge.

Runtime scope:

- renders hand cards from the current deck-roguelike run into a deterministic `deckbuilderUi.renderModel.hand` list;
- renders draft-only pipeline slots into `deckbuilderUi.renderModel.pipeline`;
- exposes selection and queueing interaction state through the runtime probe;
- queueing creates a proposal with `trustedWrite: false` routed to the existing review/apply/trust-gradient path;
- generated runs/artifacts remain untracked unless explicitly fixture-scoped.

Boundary and governance:

- Browser/Studio surfaces remain read-only and draft-only.
- Rust/local remains the trusted owner for validation, persistence, and the review/apply/trust-gradient path.
- The feature makes no production-ready, Godot-replacement/parity, quality, fun, public-launch, or market-demand claim.
- Issues #1 and #23 remain open.
