# Era M Human-Steerability Assessment

Era M — Active Human Intervention (Agent-First, Human-Steerable) — is recorded as complete on merged evidence for Milestones 74-80 plus the M81 governance refresh.

## Completed evidence chain

| Milestone | Capability | Evidence |
| --- | --- | --- |
| M74 | Design gate: active intervention posture, local Phoenix LiveView, and two-plane boundary | #2052 / PR #2101 |
| M75 | Proposal amendment before approval | #2053-#2056 / PRs #2146, #2149, #2151, #2152; Scenario Coverage v66 |
| M76 | Human-authored artifact intake | #2057-#2060 / PRs #2155, #2158, #2160, #2161; Scenario Coverage v67 |
| M77 | Live campaign steering directives | #2064 / PR #2153; Scenario Coverage v68 |
| M78 | Human constraints as first-class gates | #2065-#2068 / PRs #2162, #2163, #2164, #2165; Scenario Coverage v69 |
| M79 | Diagnosis correction and intervention feedback loop | #2069-#2072 / PRs #2166, #2242, #2243, #2244; Scenario Coverage v70 |
| M80 | Stage takeover and handback | #2076 / PR #2159; Scenario Coverage v71 |
| M81 | Era M governance refresh and steerability assessment | #2077 |

## Steerability assessment

Era M makes human intervention active but still evidence-gated. Covered intervention classes are:

1. proposal amendment;
2. human-authored artifact intake;
3. live campaign steering directive;
4. human constraint gate;
5. diagnosis correction / attribution feedback;
6. stage takeover and handback.

For the covered classes, intervention coverage is `6 / 6 = 100%`: each has a contract, implementation or demo evidence, and scenario coverage where scoped. No class grants raw writes. Every human write is represented as a validated, recorded proposal, constraint, or directive through the existing review/apply, scene/source-apply, evaluator, evidence, and provenance gates.

## Reaffirmed invariants

- Agent-first default remains the default. The autonomous loop completes with zero human input.
- Human intervention is opt-in at defined points and never mandatory.
- Studio surfaces are read + gated-write only.
- Rust remains the data plane for artifact truth, validation, determinism, evidence, provenance, diagnosis semantics, scene/source-apply, and gated writes.
- Elixir/OTP + Phoenix LiveView remain local control and presentation only; they capture, route, and render human intent but never own artifact semantics or perform trusted writes.
- CLI fallback remains sufficient for a fresh checkout without Studio.
- Hosted, multi-user, collaborative, or real-time remote Studio remains Layer-3 DEFER.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- No raw bypass, new write path, new data store, opaque ML authority, browser command bridge, auto-merge, or production-readiness claim is introduced.
- #1 and #23 remain open governance anchors.

## Era N handoff

Era N may improve human-grade Studio and adoption UX, but it inherits Era M's read + gated-write boundary. Usability work may lower friction; it may not bypass Rust-owned gates or make human intervention required.
