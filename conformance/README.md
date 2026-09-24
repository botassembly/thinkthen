# Shared conformance cases

`cases.json` is the language-neutral behavior contract for the command, the Rust library, later language bindings, and database extensions. Every surface reads the same cases. The file fixes one canonical backend URL, question grammar, exact System One request bytes, decoded answers, bare values, detailed request identities, host-neutral bulk results, and the six public error kinds.

A successful exchange has one of two provenance values. `captured` names a committed public recording whose request and response match the embedded exchange exactly. `synthetic_contract` says the exchange was written against the accepted wire contract. Fault cases name deterministic injection points. They are schema contracts until the private engine runner lands with the one-crate merge.

The mixed `annotate` case keeps successful `decide`, `choose`, `score`, and `tag` answers together in one request. The separate partial case keeps three good answers, distinguishes one valid `null` answer from one failed answer, checks the exact failure marker and count, and keeps the logical request identity. The two-group case fixes aggregate request identity in question-set group order.

`rank` expectations name zero-based input indexes in output order beside their yes probabilities. `find` expectations name the selected zero-based input index, or `null`, and list every candidate probability in input order. Its `none` candidate appears last with a null index. A host maps these indexes back to its own container.

Cases 26 and above came from the retired surfaces branch through ticket 0091. Each names its branch case under `provenance.branch`. A case that waits on a ruling names it under `provenance.pending`, and it still runs and must pass. Case names say `unsure`, per ADR 0017 section 6 item 4. The specification grammar keeps unresolved.

A `decide_many` success is `decide` over several records, one exchange per record in input order. An `annotate` case over several records holds one exchange per question-set group per record, record by record, and each answer lists only its own record's requests. A `filter` case may hold no exchange, and then it expects no index. A `counters` expectation repeats one call through a cache folder and asserts the process counter differences around the calls, since the counters have no reset.

A fault with a `question_form` breaks a question rule instead of naming an injection. The form `text` gives the question as typed values and expects `usage`. The form `file` loads it from a named file and expects `local`.

`recognize` and `relate` cases carry `text` or `entities` beside a version-one question file. Their one answer, named `result`, holds the command's value and its detailed request identity. The command runner replays their exchanges through the command, because their assembly lives there. Their exchanges are synthetic and carry the production planner's request bytes. A captured re-record waits for a live run.

`19-find-none` keeps `bare: "none"` and `details.answer.kind: "choice"`, because `answers[]` holds the wire-level choice decode. Its `operation.selected` is `null`, as `specification/find.md` says.

The pure-core integration test validates this file offline through the production question grammar, request encoder, response decoder, digest, answer rules, ranking, and find selector. It does not call a command or a network service.

`backend-profiles.json` adds the shared limit and calibration cases. Its exact edges cross the production profile parser and request encoder. It covers evidence bytes, request bytes, expanded tags, grouped annotate, equal and differing names, and either absent name.

`record-values.json` fixes the host-neutral default bulk row. Typed records and bare values cross the production serializer. Command-line framing stays in compiled binary tests.

## The loopback backend

`conformance/backend` is an offline backend every surface's tests can start. Run `cargo run --package conformance-backend`. It binds 127.0.0.1 on a free port and prints the port on its first line. A `count` line on standard input prints the requests read so far, a `release` line lets held replies go, and closing standard input prints the final count and exits. The command's own tests use its library in process.

A caller picks an arm by the base it gives, because the engine appends `/systemone` to any base.

| Base path | Answer |
| --- | --- |
| `/case/ID/v1` | The named case's response for each of its exact request bodies. Request digests hash the served URL, so a runner recomputes each expected digest |
| `/generic/v1` | Any well-formed request. The first option, level, or yes gets 0.9, and the rest share the remainder in declared order |
| `/arm/reset/v1` | A reset after the whole request arrives |
| `/arm/429/v1`, `/arm/503/v1` | That status with `retry-after-ms: 10` |
| `/arm/refuse/v1` | Status 422, the refusal that case `21-backend-fault` injects |
| `/arm/held/v1` | The generic answer, held until a `release` line |
| `/arm/malformed/CAUSE/v1` | The generic answer with the last question broken for one of the six failure causes. A distribution cause needs a choice or score question |

An unknown body, arm, or request gets status 500 and a line on standard error, so drift fails loud.
