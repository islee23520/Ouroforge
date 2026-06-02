# Runtime Probe Contract v2

Runtime Probe Contract v2 defines the browser observation API exposed as
`window.__OUROFORGE__`. The contract makes runtime pages testable by scenarios,
agents, dashboard exports, and future read-only cockpit surfaces without guessing
method names or response shapes.

The contract is browser-read-only for trusted state. Runtime pages may expose
in-memory observation and deterministic stepping controls. Rust remains the only
trusted persistence, evidence, validation, and evaluation boundary.

## Contract Object

A conforming runtime page exposes `window.__OUROFORGE__` as an object by the time
Rust scenario execution starts. The object should be stable for the page lifetime
and should expose these methods:

- `getWorldState()`;
- `getFrameStats()`;
- `getEvents()`;
- `step(count = 1)`;
- `pause()`;
- `resume()`;
- `setInput(input)`;
- `snapshot()`;
- `restore(snapshotOrId)`.

Runtime pages may expose additional methods, but scenarios and dashboard read
models must not require undocumented methods.

## Method Expectations

| Method | Required shape | Side effects | Failure behavior |
| --- | --- | --- | --- |
| `getWorldState()` | JSON object describing deterministic observable game state; should include a monotonic or comparable tick/frame value when available | observation only | malformed/non-object state is probe contract failure evidence |
| `getFrameStats()` | JSON object with bounded frame/performance counters such as `frame`, `tick`, `fps`, `deltaMs`, or runtime-specific equivalents | observation only | malformed/non-object stats are probe contract failure evidence |
| `getEvents()` | JSON array of bounded event objects, or JSON object with an `events` array | observation only | malformed events are explicit evidence, not silent empty defaults |
| `step(count = 1)` | returns world state or a step result object after advancing deterministic simulation by a non-negative integer count | in-memory runtime state only | invalid count should be clamped or fail explicitly |
| `pause()` | returns frame stats or status object | in-memory runtime state only | missing/malformed status is explicit evidence |
| `resume()` | returns frame stats or status object | in-memory runtime state only | missing/malformed status is explicit evidence |
| `setInput(input)` | accepts bounded directional/control input object and returns world state or input status | in-memory runtime state only | unsupported input keys may be ignored or reported explicitly |
| `snapshot()` | returns a JSON object snapshot or a stable snapshot id plus enough data for restore evidence | in-memory runtime state only | missing snapshot id/data is explicit evidence |
| `restore(snapshotOrId)` | returns restored world state or status object | in-memory runtime state only | missing snapshot should fail explicitly |

## Evidence Shape Expectations

Runtime probe evidence should identify the contract when practical:

```json
{
  "probeContract": {
    "name": "ouroforge-runtime-probe",
    "version": "v2"
  }
}
```

Existing v1 evidence remains readable. V2 evidence should prefer explicit
contract status over inferred defaults:

- `present`: every required method exists and observed response shapes are valid;
- `missing`: `window.__OUROFORGE__` or a required method is unavailable;
- `malformed`: a method returned a shape that scenarios/evaluators cannot trust;
- `legacy`: evidence predates v2 and did not record contract status.

## Scenario/Evaluator Expectations

Scenario and evaluator code must treat missing or malformed probe responses as
clear evidence failures. They should not silently substitute empty world state,
empty frame stats, or empty event lists unless the scenario contract explicitly
allows optional data.

Failure evidence should include:

- method name;
- expected shape;
- observed value type or bounded observed JSON;
- scenario id and step/assertion context when available;
- generated evidence artifact path when an artifact is written.

## Current Runtime Example Compatibility

`examples/runtime-probe/index.html` already exposes the v2 method set as an
in-memory static page probe. Follow-up PRs must add conformance tests and, where
low-risk, record v2 contract metadata in runtime probe evidence.

Other runtime examples may be treated as legacy until they either conform or
produce explicit missing/malformed probe evidence.

## Browser and Trust Boundary

The probe API must not:

- write trusted files;
- access local filesystem state;
- execute shell commands;
- start a command bridge;
- auto-run Rust checks;
- auto-apply mutations;
- merge source changes;
- claim a stable public engine/plugin API.

Probe methods are observation and in-memory runtime controls only. Rust commands
own evidence writes, scenario verdicts, run metadata, dashboard exports, and
mutation lifecycle persistence.

## Compatibility Notes

- V2 is an internal evidence contract, not a public engine compatibility promise.
- Existing seeds/runs remain compatible unless a follow-up PR explicitly records
  a contract upgrade and evidence migration behavior.
- Browser/Studio surfaces may display probe contract status from exported read
  models, but they remain read-only.
- No Playwright, hosted QA, browser farm, plugin runtime, native export, or
  distributed QA/Elixir implementation is authorized by this contract.

## Follow-up PR Responsibilities

- EF1.4.2 adds runtime example conformance tests and low-risk evidence metadata.
- EF1.4.3 hardens missing/malformed scenario/evaluator evidence.
- EF1.4.4 surfaces contract status in exported/read-only dashboard or cockpit
  read models where already supported.
