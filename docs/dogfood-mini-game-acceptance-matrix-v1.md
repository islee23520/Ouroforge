# Dogfood Mini-Game Acceptance Matrix v1

Issue: #2384
Companion GDD: `docs/dogfood-mini-game-gdd-v1.md`
Status: acceptance matrix contract only; no runtime, Studio, source, asset, scenario, or release implementation is added here.

## Capability status key

- **Available contract**: referenced issue is CLOSED and can be used for GDD feasibility planning.
- **Dependency**: referenced issue is OPEN or otherwise not implemented; the GDD may require it, but later issues must not silently assume it exists.
- **Human-owned**: deterministic evidence may inform the decision, but a human must make the fun/feel/release judgment.

## M118-M124 capability inventory

| Milestone | Capability | Source issue(s) | Status for #2384 |
| --- | --- | --- | --- |
| M118 | Runtime shell layout, HUD/status, canonical states | #2352, #2353, #2354 | Available contract for layout/HUD/status; #2355 remains dependency for pause/restart/win/loss UX. |
| M119 | Runtime API inventory, deterministic replay, diagnostics | #2356, #2357, #2358 | Available contract. |
| M120 | Visual rubric, source-like visual refresh, screenshot regression | #2359, #2360, #2361 | Dependency. Do not claim visual quality/product-observed pass until these land. |
| M121 | Studio workspace, project open/run/evidence, source/generated browser | #2362, #2363, #2364 | Available contract. |
| M122 | Scene editor spec, selection/transform drafts, component inspector; review/apply handoff | #2365, #2366, #2367, #2368 | Available for draft/spec/inspection through #2367; dependency for review/apply handoff (#2368). |
| M123 | Tilemap spec, tile painting draft, reachability/objective validation | #2369, #2370, #2371 | Dependency. |
| M124 | Behavior authoring model, behavior parameter draft/preview, scenario assertion generation | #2372, #2373, #2374 | Dependency. |

## Requirement-to-capability matrix

| Requirement id | Requirement | Evidence expected in later implementation | M118 | M119 | M120 | M121 | M122 | M123 | M124 | Status / dependency note |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| R1 | 5-10 minute four-room mini-game scope | GDD scope checklist, playtest script duration notes | states/HUD available | replay timing available | dependency for readability | workspace available | drafts partly available | dependency for rooms | dependency for hazards | Contract now; product-observed proof later. |
| R2 | Product-facing launch, title, controls, objective, scene status | Live screenshot `state-start.png`; shell text visible | #2352-#2354 available | diagnostics available | dependency for visual pass | run/evidence link available | n/a | n/a | n/a | Pause/restart still depends on #2355. |
| R3 | Human-discoverable movement and controls with replay parity | Input replay labels plus visible controls help | controls shell available | #2357 available | dependency for visual clarity | run evidence available | n/a | n/a | n/a | Must not rely only on harness API as in #2351. |
| R4 | HUD objective, relay count, key, gate, win/loss status | HUD/status samples and screenshots | HUD/status available; win/loss dependency #2355 | state API available | dependency for contrast/readability | Studio evidence panel available | HUD component drafts partly available | n/a | scenario assertions dependency | Win/loss message depends on #2355. |
| R5 | Key and relay collection update flags and gate state | Replay checkpoints `relay-1`, `key-collected`, `gate-open` | canonical states available | replay/digest/diagnostics available | dependency for readable feedback | workspace evidence available | component drafts available | objective validation dependency | assertion generation dependency | Later implementation must flag M123/M124 gaps if open. |
| R6 | Blocked gate before prerequisites is visible and testable | `fail/blocked` screenshot/replay and diagnostics | state name specified | diagnostics/replay available | dependency for visual fail state | evidence link available | trigger drafts available | reachability/objective dependency | assertion dependency | Do not mark green without blocked-state evidence. |
| R7 | Deterministic hazard creates fair fail/restart pressure | Hazard route/timing draft, fail replay, restart evidence | fail/restart dependency #2355 | replay/diagnostics available | dependency for hazard readability | workspace available | transform/component draft available | tile collision dependency | behavior authoring dependency | Explicit M118.4/M123/M124 dependency. |
| R8 | Pause/resume/restart/checkpoint recovery works | `paused`, `restarted` screenshots and replay labels | dependency #2355 | replay labels available | dependency for UI readability | run evidence available | n/a | n/a | behavior reset assertions dependency | Cannot be product-observed complete until #2355 lands or gap is recorded. |
| R9 | Visual style is intentional, readable, and free of hidden missing-asset diagnostics | M120 rubric verdict, state screenshots, diagnostics clean | shell viewport available | diagnostics available | dependency #2359-#2361 | source/generated labels available | n/a | tile visuals dependency | hazard visuals dependency | #2351 missing-asset gap must not be assumed fixed. |
| R10 | Studio opens project, runs/copyable safe command, links evidence | Studio/cockpit workspace screenshots/read model | runtime URL/state available | run metadata available | n/a | #2362-#2364 available | n/a | n/a | n/a | Available contract; no browser trusted command bridge. |
| R11 | Scene editor supports entity selection and safe draft edits | Draft JSON, preview summary, validation errors | state targets available | diagnostics available | n/a | workspace available | #2365-#2367 available; #2368 dependency | n/a | n/a | Review/apply handoff remains dependency. |
| R12 | Tilemap authoring supports rooms, collision, markers, reachability | Tile draft, collision/marker validation, reachability report | runtime states available | diagnostics available | visual dependency | workspace available | scene draft context available | dependency #2369-#2371 | n/a | All tilemap authoring is dependency. |
| R13 | Behavior authoring supports hazard timing/route and scenario assertion drafts | Behavior draft/preview and assertion draft | fail/win states available | replay/diagnostics available | visual dependency | workspace available | component context available | tile context dependency | dependency #2372-#2374 | All behavior authoring is dependency. |
| R14 | Live observability records screenshots, replay, diagnostics, frame/event samples | M116-style generated bundle under ignored `runs/` | canonical screenshot states available | replay/diagnostics available | visual capture dependency | evidence browser available | n/a | n/a | scenario assertions dependency | Later product-observed issues own live evidence. |
| R15 | Generated-state audit keeps run/screenshots/browser/package outputs out of trusted source | `git status --short --ignored` audit | n/a | n/a | generated screenshot policy dependency | source/generated browser available | review/apply dependency | draft output dependency | draft output dependency | Required for closure; no generated artifacts tracked in #2384. |
| R16 | Contract-complete vs product-observed classification is explicit | Closure/PR evidence states classification | runtime evidence split available | diagnostics available | visual pass dependency | evidence browser available | n/a | n/a | n/a | #2384 is contract-complete only. |
| R17 | Human-owned fun/feel verdict is recorded separately from deterministic pass/fail | Human playtest note/verdict form | supports visible states | supports telemetry | visual clarity dependency | evidence display available | authoring context available | level edits dependency | behavior edits dependency | Human-owned; never automated. |
| R18 | No browser trusted writes, command bridge, hidden execution, auto-apply/merge, or self-approval | Boundary checklist and generated-state audit | no trusted writes | local API only | n/a | source/generated labels | review/apply dependency preserves gate | draft-only dependency | draft-only dependency | Invariant across all later work. |

## Acceptance rows for #2384 itself

| Check | Required evidence in this PR unit | Result expectation |
| --- | --- | --- |
| GDD exists | `docs/dogfood-mini-game-gdd-v1.md` | Defines 5-10 minute mini-game, mechanics, encounters, assets/audio/HUD, save/progression, authoring/edit goals, fun/feel boundary. |
| Matrix exists | `docs/dogfood-mini-game-acceptance-matrix-v1.md` | Maps every requirement to M118-M124 and marks unimplemented capability dependencies. |
| #2351 gaps honored | GDD negative-baseline section | Missing asset/debug-probe/fun-quality gaps are not treated as solved. |
| Contract classification | GDD governance and this matrix | #2384 can close only as `contract-complete` unless later live evidence is attached. |
| Generated-state clean | Git diff/status audit | Only GDD/matrix docs are tracked; no run/screenshot/package/browser outputs. |
| Human fun/feel ownership | GDD and R17 | Human verdict remains separate from deterministic evidence. |

## Later product-observed acceptance rows

These rows are not satisfied by #2384. They are the target for #2385-#2387 and the M130 usability gate.

| Row | Product-observed evidence required | Dependency risk |
| --- | --- | --- |
| PO1 | Live playthrough reaches `win/exit` from a local URL/command with replay and final digest. | Needs M118.4 if restart/win/loss UX is included. |
| PO2 | Screenshots exist for `start`, `key-collected`, `gate-open`, `fail/blocked`, `paused`, `restarted`, `win/exit`. | Needs M120.3 capture and M120.1 rubric. |
| PO3 | Runtime diagnostics are clean or explicitly classified as product-observed blockers. | M119 available; visual missing-asset gap from #2351 must be red until fixed. |
| PO4 | Studio opens project, distinguishes source/generated state, and links evidence without trusted browser writes. | M121 available; later UI proof still required. |
| PO5 | Scene/tile/behavior edits flow through drafts and review/apply handoff, not direct writes. | Depends on #2368-#2374. |
| PO6 | Human playtest verdict records clarity, fairness, pacing, and fun/feel notes. | Human-owned; cannot be automated by the engine. |

## Non-goal and wording guard

This matrix must not be cited as proof of a shipped game, public release, store readiness, production editor, full Studio, arbitrary scripting, secure sandbox, Godot parity/replacement, automated fun/quality verdict, trusted browser write path, command bridge, auto-apply, auto-merge, self-approval, dependency install, package publish, deploy, upload, signing, or CI/workflow mutation.

#1 and #23 remain open.
