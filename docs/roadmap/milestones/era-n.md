### Era N: Human-Grade Studio and Adoption UX (Agent-First, Newcomer-Accessible)

Added 2026-06-09. Era M makes the human an active, gated participant; Era N makes the engine **accessible to newcomers and non-developers** and unifies observation + intervention + authoring into one human-grade Studio. The bet from the OSS-core + PLG indie strategy is that adoption needs a low barrier to entry and a pleasant, intuitive surface — without weakening the evidence-native, deterministic, gated core.

**Guiding principle for Era N.** The Studio is a **local-first, single-user Phoenix LiveView** surface that *renders Rust-owned evidence read-only and routes every write through the existing gates* (read + gated-write, per Era M). Accessibility raises usability, never bypasses verification: generation still flows through review/apply; the deterministic Rust kernel and the CLI fallback are untouched.

**Boundaries (all prior boundaries unchanged; Era N additions in bold):**
- **The Studio is local single-user Phoenix LiveView; hosted/multi-user/collaborative Studio is Layer-3 DEFER; the CLI fallback remains first-class.**
- **Accessibility/onboarding UX raises usability only; generation and all writes still flow through review/apply/the gates — no bypass of verification or determinism.**
- Rust = data plane; Elixir/Phoenix LiveView = control + presentation plane; fun/taste + release go/no-go remain human; conservative wording (no "no-code game maker" overclaim).

**Completion update (2026-06-09).** Era N is complete on merged evidence through #2099 / PR #2292. The evidence chain covers M82 non-developer generative front door (#2078-#2081 / Scenario Coverage v72), M83 onboarding/templates/docs (#2082-#2085 / Scenario Coverage v73), M84 accessibility/i18n/themes/keyboard (#2086-#2089 / Scenario Coverage v74), M85 Phoenix LiveView Human-Grade Studio (#2090-#2094 / Scenario Coverage v75), M86 local Studio packaging/delivery (#2095-#2098 / Scenario Coverage v76), and M87 governance/adoption assessment (#2099 / Scenario Coverage v77). Adoption UX improved the local path to a first verified game structurally through guided intake, templates, first-run docs, accessibility, unified local Studio surfaces, and local packaging smoke evidence. This is not a no-code, hosted, release, deploy, signing, or human-required claim: Studio remains read + gated-write and local-first; Rust remains the data plane; Elixir/Phoenix remains control + presentation; hosted/multi-user collaboration remains Layer-3 DEFER; fun/taste and release go/no-go remain human; #1 and #23 remain open.

#### Milestone 82: Non-Developer Generative Front-Door UX
Goal: let a non-developer go from a brief/conversation to a verified game proposal.
Target deliverables: a conversational/guided intake shell over the Milestone 30 generation path; a proposal preview; routing through review/apply (generation never performs a trusted write); a demo and Scenario Coverage v72.
Success criteria: a non-developer reaches a verified proposal via a guided flow; generation stays read-only/draft until gated apply; deterministic.

#### Milestone 83: Onboarding, Templates, In-Product Docs, and First-Run
Goal: get a new user from zero to a running game quickly.
Target deliverables: a template gallery; a step-by-step first-run flow; in-product docs/tutorials anchored to real surfaces; sample seeds; a demo and Scenario Coverage v73.
Success criteria: a new user can start from a template and reach a running, verified game; docs are in-product and accurate; no bypass of the gated path.

#### Milestone 84: Tool Accessibility (a11y), Internationalization, Themes, and Keyboard
Goal: make the tool itself accessible and international.
Target deliverables: a11y (keyboard navigation, screen-reader labels) for the Studio; tool-UI internationalization (Phoenix gettext); themes/contrast; keyboard shortcuts; a demo and Scenario Coverage v74.
Success criteria: the Studio is keyboard- and screen-reader-navigable and localizable; a11y checks pass; behavior unchanged.

#### Milestone 85: Human-Grade Studio UX Surface (Phoenix LiveView)
Goal: unify observation, intervention (Era M), and authoring into one local-first, real-time, read + gated-write Studio.
Target deliverables: a Phoenix LiveView Studio composing the live diagnosis console, steering console, amend/constraint/correction panels, takeover/handback controls, interactive authoring, and the evidence/journal/verdict views; real-time updates via PubSub; **all writes routed only through the existing gates**; a demo and Scenario Coverage v75.
Success criteria: the Studio renders Rust-owned evidence and performs no trusted writes itself (every write goes through review/apply/scene-or-source-apply); it is real-time and local-first; the CLI fallback still works.

#### Milestone 86: Studio Packaging and Local Delivery
Goal: deliver the Studio as a local desktop/web app.
Target deliverables: packaging of the local Phoenix Studio with the Rust kernel; an install/run UX; a built-artifact smoke test; a demo and Scenario Coverage v76.
Success criteria: a user can install and run the local Studio + kernel; hosted/multi-user is explicitly DEFER (Layer-3); local-first preserved.

#### Milestone 87: Era N Roadmap and #1 Governance Refresh, and Adoption-UX Assessment
Goal: record Era N completion and assess adoption usability without core erosion.
Target deliverables: an adoption-UX assessment (newcomer time-to-first-verified-game; that accessibility never bypassed the gates/determinism); reaffirmation of local-first, two-plane, and DEFER boundaries; a #1 completion comment; a Scenario Coverage v77 regression suite.
Success criteria: the roadmap and #1 reflect actual Era N completion with evidence; the no-bypass and local-first invariants are reaffirmed; #1 and #23 remain open.
