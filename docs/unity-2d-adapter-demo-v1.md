# Unity 2D Adapter Demo v1

This fixture-scoped demo proves the Era O M93 Unity 2D adapter can import a small
Unity Force-Text source project into Rust-owned migration IR and produce an
honest fidelity report. It is an on-ramp demonstration, not engine absorption,
not a finished-game auto-port, and not a Unity runtime bridge and no Unity runtime bridge is introduced.

Run:

```bash
examples/unity-2d-adapter-v1/demo/run-demo.sh
```

The script writes `examples/unity-2d-adapter-v1/generated/fidelity-report.json`
via:

```bash
cargo run -p ouroforge-cli -- migration unity-demo --project examples/unity-2d-adapter-v1/sample-project --output examples/unity-2d-adapter-v1/generated/fidelity-report.json
cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers 2 || true
```

The report must show green, yellow, and red fidelity units; resolved `.meta`
GUID references; logic touchpoints for behavior-bearing MonoBehaviour data;
missing oracle records; `claimed_ported_units: []`; and a deterministic
`sha256:` IR state hash.

Clean-room boundary: source-project/open-text `.unity`, `.prefab`, `.asset`, and
`.meta` inputs only. Decompiled source, shipped-build ripping, Unity runtime
embedding, live bridging, and direct logic translation are out of scope. Logic is
not translated or claimed complete; it becomes an Era R re-derivation task until
Ouroforge-native oracle evidence passes.

Rust remains the data plane and owns artifact truth, validation, and state
hashing. Studio/Elixir is not touched here and has no trusted write or artifact
semantics authority. #1 and #23 remain open.

Invariant wording for automated coverage: no Elixir/Phoenix trusted write and no
Unity runtime bridge.
