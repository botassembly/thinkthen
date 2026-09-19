# Record 0002: Translate a plan to and from systemone

- Ticket: `sdlc/tickets/0002-translate-a-plan-to-and-from-systemone.md`
- Branch: `ticket/0002-systemone-adapter`
- Landed: 2026-09-19

## What was built

`thinkthen-core` gains the plan, the reply, and the `systemone` adapter of `specification/backends.md`, plus the blank-text rule `specification/decide.md` now states.

- `text.rs`: `Evidence` joins `Condition`, `BackendName`, and `ModelName` in the one macro. All four refuse text that is empty or holds only white space. `EmptyTextError` is now `BlankTextError`, because the enum refuses more than an empty string.
- `adapter.rs`: the `Adapter` enum moves out of `result.rs` and gains `FromStr`, with `UnknownAdapterError` naming what was asked for.
- `plan.rs`: `Plan` holds the evidence, the model name, and an ordered list of questions. `EmptyPlanError` refuses a plan that asks nothing.
- `reply.rs`: `Reply` holds the model that answered, one answer per planned question, and the usage the backend reported.
- `systemone.rs`: `encode`, `decode`, `EncodeError`, and `DecodeError`, over five private wire structs.

The ticket names an `AdapterKind` enum. `result.rs` already held `Adapter` for the `meta.adapter` field, so the enum kept the name `Adapter` and gained the parse the ticket asked for. There is one enum, in one module, and no second name for the same idea.

`systemone` is a public module rather than four flat re-exports. `encode` and `decode` are names a second adapter will also want, so they stay behind the name of the format they speak.

The request structs derive `Serialize` always and `Deserialize` and `PartialEq` only under `cfg(test)`. Production writes the request and never reads one back. Tests read both the fixture and the encoded bytes into the same struct, which is how the comparison happens by value with no dynamic JSON.

`encode` returns a result. Serializing a struct of strings and a map with string keys cannot fail today, and the core denies `unwrap` and `expect`, so the error from `serde_json::to_vec` travels up rather than being swallowed. `EncodeError` is the one error variant here that no test can provoke.

## Red then green

Each test below failed for the reason given before the code that satisfies it existed.

- `new_refuses_text_that_is_empty_or_only_white_space`: failed at `" "` with `left: Ok(Condition(" "))`, `right: Err(Condition)`. The rule was still `is_empty`.
- `an_adapter_parses_from_its_lowercase_name`: failed to compile with `the trait bound Adapter: FromStr is not satisfied`.
- `a_plan_that_asks_nothing_is_refused`: failed with `left: Ok(Plan { ..., questions: [] })`, `right: Err(EmptyPlanError)`.
- `encode_writes_the_request_the_fixture_shows`: failed with `left: Request { state: "", model: "", questions: {} }` against the fixture's state, model, and `q1`.
- `each_refused_response_names_its_own_cause`: failed with `left: Err(Malformed("nothing is read yet"))`, `right: Err(MissingAnswer("q1"))`.
- `decode_reads_the_answer_the_fixture_shows` and `a_response_decodes_without_usage_and_past_unknown_fields`: both failed at `a systemone response: Malformed("nothing is read yet")`.

## The ladder

| Rung | Script | Exit |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | 0 |
| 1 | `sdlc/scripts/lint` | 0 |
| 2 | `sdlc/scripts/test` | 0 |
| 3 | `sdlc/scripts/spec` | 0 |

Twenty-nine unit tests, one integration test, one documentation test, and four spec examples pass. The largest file is `systemone.rs` at 277 non-blank lines.

## The ratchet

The ceiling was 799 and is now 1264, which is the measured total. It moved in three steps, each in the commit that needed it: 827 for the blank-text rule and the fourth text value, 868 for the adapter module and its parse, and 1264 for the plan, the reply, and the adapter with its tests.

## Dependencies

None were added. `serde`, `serde_json`, `thiserror`, and `proptest` cover the work, and `policy.py` still reports the same accepted sets over 52 resolved packages.

## What the specification left unsaid

- `backends.md` does not say what an adapter does with a response whose `model` is blank. Decode refuses it as malformed, because `ModelName` cannot hold blank text and the result must report the model that answered.
- `backends.md` names four refusals for decode and never says whether unknown question names in `answers` are an error. They are ignored. The plan decides which names are read, so a backend that answers more than it was asked does not fail the judgment.
- The fixture README says to compare JSON by value and never by bytes, and the core cannot hold dynamic JSON. The comparison therefore runs through the adapter's own request structs, which is the only typed way to do it.
- `backends.md` says question names are `q1` onward in plan order. It does not say what orders the `questions` map on the wire. The map is a `BTreeMap`, so the bytes are the same every run, and a plan of ten or more questions would write `q10` before `q2`. A JSON object carries no order, so the name is what matters.
- The `refused-wrong-kind` fixture is a `choice` answer carrying `confidence`, a field that appears nowhere in `backends.md`. Nothing reads it. The four Draft rows of the mapping table still owe their field names.
- `decide.md` settles that a condition or evidence that holds only white space is a usage error. It says nothing about a backend name or a model name. One rule now covers all four text values, because a name of pure white space names nothing either.

## Review

A second reviewing agent read the branch after it was rebased onto `afc5ee5`, the commit that told `specification/backends.md` what decode ignores, what it refuses, and that a blank profile value is a usage error. The figures above describe the branch as the build agent left it. The figures in this section describe the branch as it now stands.

### What the review checked

It read `AGENTS.md`, `sdlc/planning/rust-standards.md`, Ian's Rust ideal state, the settled parts of `specification/backends.md`, the five fixture files and their README, the ticket, the record, the Review section of record 0001, the whole diff against main, and every file under `crates/thinkthen-core/src`.

Against the ticket it walked each Scope item and each Acceptance bullet to the code and the test that satisfies it. All four text values refuse white space. `Evidence`, `ModelName`, and `Plan` hold what the ticket names. `Adapter` parses from `systemone` and refuses five other spellings. `encode` of the urgency plan equals the request fixture by value through the adapter's own structs. `decode` of the response fixture yields 0.92, the model name, and the usage. Each `refused-` fixture yields its own variant. A response with no `usage` and a response carrying `request_id` and `rationale` both decode. The property test draws any character and round-trips the evidence and the condition. Wire bodies are typed structs, and the core's dynamic JSON bans still hold.

It broke three behaviors and watched the matching tests fail, then restored the code. Making `wire_name` zero-based failed `encode_writes_the_request_the_fixture_shows` and failed `each_refused_response_names_its_own_cause` with `left: Err(MissingAnswer("q0"))` against `q1`. Clamping the decoded number into range failed `each_refused_response_names_its_own_cause` with a decoded `Probability(1.0)` where `ProbabilityOutOfRange("q1")` was owed. Narrowing the blank-text rule back to `is_empty` failed `new_refuses_text_that_is_empty_or_only_white_space` at `" "`.

Against the rebased specification it checked the four distinct decode causes, the refusal of an absent or blank model, the names `q1` onward in plan order, the indifference of decode to key order in `answers`, the ignoring of an answer name the plan lacks, the ignoring of the vendor's `confidence` and any other unread field, and acceptance of a response with no `usage`. Two of those rules had no test, and the review added them. The blank-value rule for the URL and the key variable belongs to the binary, which this ticket excludes.

On size it counted every file, measured the whole crate rather than the diff, and looked for wire structs to merge, error variants nobody can act on apart, plan machinery ticket 0003 will not call, and copied test bodies a table would fold. The five wire structs each carry one JSON object the format names. The four text newtypes already share one macro, so no private helper would remove copying there.

On the public surface it listed every public item against what tickets 0003 and 0004 need, confirmed every struct field is private, every fallible constructor returns a result, every fixed set is an enum, every public item carries a doc comment that `cargo doc` with warnings denied proves, and no module's first documentation line joins two jobs with "and".

On errors it confirmed five `thiserror` types, no boxed error, no ignored result binding, and no key, evidence, or condition text in any message. Decode errors name a question only by its wire name. The one serde message that reaches a user is the parse failure, which serde_json writes as a position and a field name and never as the input.

On ownership it found no clone outside test setup, no owned parameter that is not kept, and no reference counting or interior mutability. `decode` sizes its answer vector from the plan before it fills it.

On gates it confirmed the workspace lint table, both `clippy.toml` files, all four ladder scripts, both manifests, and `Cargo.lock` are byte for byte what main carries. No dependency was added. No `#[allow]` or `#[expect]` appears anywhere. No `unwrap`, `expect`, or `panic` sits outside a test module. The largest file is `systemone.rs` at 305 non-blank lines, against a ceiling of 500.

### What the review changed

- Added the two tests the rebased specification earned. One encodes a two-question plan, checks `q1` and `q2` against the plan order, and decodes a body that writes `q2` before `q1`. The other refuses a response whose model is absent, empty, or pure white space. Both were watched failing first: the order test reported `"asks for a refund"` where `"is urgent"` was owed, and the model test accepted a body with no model at all.
- Deleted `text_that_json_escapes_reaches_the_wire_unchanged`. Its six strings are a subset of what `any::<char>()` generates, and the assertion restated that serde_json escapes a string.
- Deleted `Answer::kind`, which no caller reads, and narrowed `AnswerKind`, `Verb`, `Question::verb`, and `Question::condition` to the crate. Record 0001 kept them public against a guess about this ticket. This ticket dispatches on the verb from inside the crate and never reads the kind.

### What the review left, and why

- `DecodeError::Malformed` carries two refusals: a body that is not a `systemone` response, and a response whose model is absent or blank. The specification states the model rule on its own line, and a caller that wants to tell a broken backend from a broken reply cannot. A fifth variant would separate them. The steering agent decides.
- `MissingAnswer`, `WrongKind`, and `ProbabilityOutOfRange` each carry a `String` holding `q1`. A caller that wants to know which question failed has to parse the place back out of the name. A `usize` place, rendered as the wire name in the message, would carry the same text and no parsing.
- `EncodeError` cannot fire. `serde_json` fails only on a non-string map key, a non-finite float, or a writer error, and a request holds none of the three. The alternative is a suppression of the `unwrap` ban, which is worse than an unreachable variant, so the result type stays.
- `Reply` is a named triple with three accessors. A tuple would save about thirty lines and cost the names, and every adapter returns this shape, so it stays.
- Seven public items have no caller outside the crate today: `SCHEMA`, `Usage::new`, `Probability::new`, `ProbabilityError`, `Answer::new_yes_no`, `Answer::probability`, and `Assessment::min_prob`. Narrowing them turns on what tickets 0003 and 0004 will print and record, so the review names them rather than guessing.
- `encode` copies the evidence and every condition into owned strings. A borrowing request struct would remove three allocations per request and would tie the test structs to the lifetime of the bytes they are read from. One request precedes one HTTP POST, so the allocation is named here and kept.
- `EmptyPlanError` guards a state no Acceptance bullet names, and ticket 0003 always builds a plan of one question. It stays because `encode` of an empty plan would send a request no backend can answer, and a fallible constructor is what the standards ask for.
- `Adapter` implements `FromStr` rather than carrying an inherent parse. The trait costs three lines of boilerplate and lets the binary hand the name straight to a `value_parser`.

### The ratchet

The ceiling was 1264 when the review started and is 1283 now. It rose to 1309 for the two conformance tests, fell to 1292 when the escape cases went, and fell to 1283 when the dead accessor and the four narrowed items went. Every step landed in the commit that moved it.

### The ladder after the review

| Rung | Script | Exit |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | 0 |
| 1 | `sdlc/scripts/lint` | 0 |
| 2 | `sdlc/scripts/test` | 0 |
| 3 | `sdlc/scripts/spec` | 0 |

Thirty unit tests, one integration test, one documentation test, and four spec examples pass.

### The verdict

The work is good enough to land.
