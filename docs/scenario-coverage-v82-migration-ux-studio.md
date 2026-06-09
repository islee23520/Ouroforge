# Scenario Coverage v82 — Migration UX Studio Regression Suite

Scenario Coverage v82 locks the Era O M94 Migration UX behavior introduced by
#2186, #2187, and #2188. It covers the Studio import wizard, fidelity report
view, fix-forward routing, and demo evidence. The suite is deliberately
boundary-focused: Studio is a Phoenix/LiveView-facing control and presentation
plane, while Rust owns adapter IR, mapping, fidelity/oracle records,
deterministic state hashes, and gated writes.

## Boundary

- one-way on-ramp only: source-project facts become Ouroforge-native skeleton
  evidence. There is no live Godot/Unity bridge and no embedded engine runtime.
- Source-project/open-text only: Godot `.tscn`/`.tres` and Unity Force-Text +
  `.meta` projects are accepted through Rust adapters; shipped builds,
  decompiled source, binary ripping, player data, and runtime payloads are
  rejected before Studio routing.
- Fidelity remains honest: 🟢 means clean declarative skeleton import, 🟡 means
  best-effort or caveated import, and 🔴 means unsupported or clean-room Era R
  re-derivation. A lossy import or behavior-bearing unit cannot be graded clean.
- Oracle-gated: no row can be claimed `ported`, behavior-equivalent, or done
  without captured passing oracle evidence. M94 Studio rows keep
  `ported_claim_allowed=false`.
- Determinism is visible: 2D verification uses bit-exact `sha256:` state hashes;
  2.5D/3D would use deterministic state-hash primary plus perceptual secondary.
- Studio does not write artifacts, own artifact semantics, create a new data
  store, or self-certify fixes. Every write-affecting action routes through the
  existing `ouroforge` CLI/gates.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v82.studio-import-wizard-source-only` | The wizard rejects shipped-build/decompiled-style paths and routes only Godot/Unity source projects through allowed Rust migration CLI families. |
| `v82.studio-fidelity-report-lossy-not-clean` | The report view preserves Green/Yellow/Red rows, rejects Red rows without Era R tasks, and blocks laundering lossy/behavioral gaps into clean status. |
| `v82.studio-no-auto-port-without-oracle` | The demo and validators reject `claimed_ported_units` and any port-claim permission without later oracle evidence. |
| `v82.studio-deterministic-hash-evidence-required` | Demo/report evidence includes deterministic `sha256:` hashes and validators fail closed when all hash evidence is missing. |
| `v82.studio-no-trusted-elixir-write` | Studio Migration UX lib code has no trusted write primitive and retains Rust data-plane ownership. |
| `v82.coverage-ledger-and-demo-script` | The v82 matrix, docs, demo script, and tests are recorded as the coverage ledger for #2189. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```

The lane implementation also runs `cargo build --workspace --jobs 2`, Studio
`mix compile --warnings-as-errors`, the trusted-write grep, and #1/#23 open
state checks before merge.
