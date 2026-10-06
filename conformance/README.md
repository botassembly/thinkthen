# Shared conformance cases

`cases.json` is the language-neutral behavior contract for the command, the Rust library, later language bindings, and database extensions. Every surface reads the same cases. The file fixes one canonical backend URL, question grammar, exact System One request bytes, decoded answers, bare values, detailed request identities, host-neutral bulk results, and the six public error kinds.

`settings.json` holds nine engine-setting cases under `thinkthen.settings-cases/1`. The command, Rust, Python, TypeScript, Ruby, R, C, DuckDB, PostgreSQL and SQLite runners map each setting to their own public spelling, run each step against a loopback arm or a fresh folder, and compare the answer or error with the listener count. SQL adapter proof belongs to ticket 0149.

`routine-ids.txt` selects 32 IDs from `cases.json` for the routine surface checks in `sdlc/scripts/surfaces`. Each runner reads the canonical 55-case JSON and filters by the absolute path in `THINKTHEN_CONFORMANCE_IDS`; a direct runner with that name unset still checks all 55. A missing or duplicate ID fails before a case runs. Supported cases count as pass or fail, unsupported injections as not run with a reason, and unselected cases are reported separately. The nine `settings.json` cases keep their own counted replay, cache-miss, and request-size relation split proof. Stored loopback replies in `cases.json` are not cache hits. `sdlc/scripts/test-full-cases --run` chooses the full functional set at a batch checkpoint; `sdlc/scripts/test-stress --run` is the separate repeated load campaign.

A successful exchange has one of two provenance values. `captured` names a committed public recording folder, under `demos/` or `specification/`, whose `thinkthen.jsonl` holds each question of the embedded exchange with the answer and model its response carries. `synthetic_contract` says the exchange was written against the accepted wire contract. Fault cases name deterministic injection points. They are schema contracts until the private engine runner lands with the one-crate merge.

The mixed `annotate` case keeps successful `decide`, `choose`, `score`, and `tag` answers together in one request. The separate partial case keeps three good answers, distinguishes one valid `null` answer from one failed answer, checks the exact failure marker and count, and keeps the logical request identity. The two-group case fixes aggregate request identity in question-set group order.

`rank` expectations name zero-based input indexes in output order beside their yes probabilities. `find` expectations name the selected zero-based input index, or `null`, and list every candidate probability in input order. Its `none` candidate appears last with a null index. A host maps these indexes back to its own container.

Cases 26 and above came from the retired surfaces branch through ticket 0091. Each names its branch case under `provenance.branch`. A case that waits on a ruling names it under `provenance.pending`, and it still runs and must pass. Case names say `unsure`, per ADR 0017 section 6 item 4. The specification uses that machine word and says "not sure" in prose.

A `single` answer's `details` also carry the run facts: `usage`, the exchange reply's counts, absent when the reply has none; `requests_sent` of 1; and `cached` of `false`. Each surface runner makes its details call the text's first send, so these hold. The command's own runner is the exception. It replays in process, so it sees `requests_sent` of 0, and it leaves these run facts to the command's spec pages. A choice or score answer carries `confidence` when its reply does, and is absent otherwise. The file holds no `url`, because the loopback port changes each run. Each surface runner compares `meta.url` with the address it served, `BASE/systemone`.

An `annotate` case may name a `record`, the one JSON record every surface sends as text, and its set's `on` pointers read each group's part. A `decide_many` success is `decide` over several records, one exchange per record in input order. An `annotate` case over several records holds one exchange per question-set group per record, record by record, and each answer lists only its own record's requests. A `filter` case may hold no exchange, and then it expects no index. A `counters` expectation repeats one call through a cache folder and asserts the process counter differences around the calls, since the counters have no reset.

A fault with a `question_form` breaks a question rule instead of naming an injection. The form `text` gives the question as typed values and expects `usage`. The form `file` loads it from a named file and expects `local`.

`recognize` and `relate` cases carry `text` or `entities` beside a version-one question file. Their one answer, named `result`, holds the command's value and its detailed request identity. The command runner replays their exchanges through the command, because their assembly lives there. The `relate` exchanges are synthetic and carry the production planner's request bytes. Cases 41 to 50, the `recognize` cases, are captured. Ticket 0147 recorded them live, and each exchange names its entry under `specification/fixtures/recognize/recordings/conformance/`.

`19-find-none` keeps `bare: "none"` and `details.answer.kind: "choice"`, because `answers[]` holds the wire-level choice decode. Its `operation.selected` is `null`, as `specification/find.md` says.

The pure-core integration test validates this file offline through the production question grammar, request encoder, response decoder, digest, answer rules, ranking, and find selector. It does not call a command or a network service.

`backend-profiles.json` adds the shared limit and calibration cases. Its exact edges cross the production profile parser and request encoder. It covers evidence bytes, request bytes, expanded tags, grouped annotate, equal and differing names, and either absent name.

`calibration.json` pins one saved question's canonical bytes, full independently calculated digest and mismatch pair. It also pins a named set's canonical bytes and digest. Public library and SQL details checks use this one input at their real entry points; text-only routes assert refusal before a send.

`record-values.json` fixes the host-neutral default bulk row. Typed records and bare values cross the production serializer. Command-line framing stays in compiled binary tests.

## The loopback backend

`conformance/backend` is an offline backend every surface's tests can start. Run `cargo run --package conformance-backend`. It binds 127.0.0.1 on a free port and prints the port on its first line. A `count` line on standard input prints the requests read so far. A `release` line lets every held reply go, now and from here on. A `round` line lets go the replies held at that moment, and a reply that arrives later holds again. A `wait N` line later prints `wait K`, where K is the count once it reads at least N, or at 30 s. Closing standard input prints the final count and exits without waiting for a pending `wait`. The command's own tests use its library in process.

A caller picks an arm by the base it gives, because the engine appends `/systemone` to any base.

| Base path | Answer |
| --- | --- |
| `/case/ID/v1` | The named case's response for each of its exact request bodies. Request digests hash the served URL, so a runner recomputes each expected digest |
| `/generic/v1` | Any well-formed request. The first option, level, or yes gets 0.9, and the rest share the remainder in declared order |
| `/arm/reset/v1` | A reset after the whole request arrives |
| `/arm/429/v1`, `/arm/503/v1` | That status with `retry-after-ms: 10` |
| `/arm/refuse/v1` | Status 422, the refusal that case `21-backend-fault` injects |
| `/arm/held/v1` | The generic answer, held until a `release` line or the next `round` line |
| `/arm/delay/MS/v1` | The generic answer after MS milliseconds, at most 10000. Each connection sleeps on its own thread. A bad value gets status 500 and `the delay arm needs a whole number of milliseconds`. A value above 10000 gets status 500 and `the delay arm allows at most 10000 milliseconds` |
| `/arm/full/v1` | The generic answer, plus `"confidence":0.9` on every choice and score answer and `"usage":{"input_tokens":1,"output_tokens":1}` |
| `/arm/status/CODE/v1` | Status CODE with body `status arm`, for CODE 401, 402, 403, or 404. Any other value gets status 500 and `the status arm takes 401, 402, 403, or 404` |
| `/arm/malformed/CAUSE/v1` | The generic answer with the last question broken for one of the six failure causes. A distribution cause needs a choice or score question |

An unknown body, arm, or request gets status 500 and a line on standard error, so drift fails loud.

A whole number is ASCII digits only, in the delay arm and in `wait` alike. `+200`, `-1`, an empty value, and a value too large for 64 bits are not whole numbers. A `wait` with no whole number is an unknown line. It writes a line on standard error and nothing on standard output.

### One backend per test

Each test starts its own backend. The count, the held gate, the rounds, and the port live in that process, so no state needs a reset. Twenty backends start together within 5 s, and `sdlc/records/0117-design-review.md` measured 112 ms for twenty.

1. Start the binary and read the port line.
2. Drive `count`, `wait N`, `round`, and `release` from one writer.
3. Close standard input at the end, and read the final count.

Lines are served in order. A `wait` answers on its own line later and never holds up the lines behind it. Its `wait ` prefix tells it from a `count` line when the two interleave. A `wait` still pending at the end can print before or after the final count. Rust tests call `Backend::wait`, `Backend::round`, and `Backend::release` in process.

## Required 0.2 parity (0432 phase A)

The `parity` section of `cases.json` is the independent public inventory:
28 consumers, with Java/Kotlin/Scala, Dart/Flutter, TypeScript/JavaScript,
CLI/C, three SQL extensions and three dataframe surfaces kept separate.
It references the existing 55 behavior cases, nine settings cases and type
corpus rather than changing their landed schemas. `decide_many` is a decide
input form; generic JSON compatibility calls do not qualify named or typed
cells. Counts from source are not execution evidence.

Run `sdlc/scripts/surfaces --parity-baseline` for phase A: only actual CLI,
Rust and C consumers run; other rows remain missing/not checked. This bounded
run also exits nonzero and cannot qualify full parity.

Run `sdlc/scripts/surfaces --parity` explicitly for complete adoption. It runs the
existing full consumers with no selector, writes logs and the current matrix
to `target/parity/`, and exits nonzero for every failed or missing required
cell, including exit 77. Routine gates validate the inventory and retain
current green behavior; they do not require unfinished parity adoption.
After all owners adopt the inventory, 0432 must make this full run mandatory
and pass it before closing. The overall ticket remains open.

Each required declaration has `id`, one of the ten `verb` values, `kind`,
`input`, `expect` and `preconditions`. References resolve to existing fixtures;
expectations are independent of implementations. `preconditions` name owner
tickets, not exemptions. Result/2 IDs/provenance and image cases are target
contracts until their native owners land. The generated result schema and
type corpus remain result/1. PNG fixtures are deterministic one-pixel red and
blue images: ordered red/blue/red attachments retain a duplicate. They prove
transport and admission, not model accuracy. Native image owners supply the
saved response/wire oracle for their admitted route before support qualifies.

A family's existing consumer checks its named public method, the declaration's
expected fields/values/errors and counted loopback sends. Compiler checks must
access known probabilities, facts, spans, edges, locations and ID fields with
real public types. Runtime checks validate their values. On completion, print
one JSON line on stdout, for example:

```text
parity: {"consumer":"rust","case":"annotate-packed-groups","checks":["named","runtime"],"status":"pass"}
```

The `checks` list must equal the declaration's list (default `named,runtime`;
typed cells also require `compile`). Emit `fail` for a completed failed case;
never emit a pass from a generic JSON call, schema-only validation, selector,
ruling or missing toolchain. A shared folder must emit separate consumer IDs
at each actual door. Unknown/duplicate IDs and skipped statuses fail. Process
failure invalidates all its emitted cells. With no adopted output, named door
counts remain **not checked**, even when the baseline command passed.

`annotate-packed-groups` pins ADR 0111's single packed request and member
probabilities, through the public Rust consumer. The old two-group exchanges
remain the historical per-group grammar oracle; they are not the current
transport expectation. The partial-member case remains required, including
its valid null and failed marker. Existing runner skips remain visible missing
cells until owners replace them with real boundary executions. Text-only image
rulings, dropped-image routes and PostgreSQL's client-reader workaround require
executed boundary cases; a written ruling never passes a cell by itself.
