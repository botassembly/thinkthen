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
