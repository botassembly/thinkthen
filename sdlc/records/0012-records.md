# 0012: Records

Branch `ticket/0012-records`. Built 2026-09-19.

## What landed

`decide`, `choose`, and `score` run over many records. `--input FILE` reads a path, `--lines` makes each line a text record, `--jsonl` makes each line a JSON record, and `--field POINTER` is RFC 6901 and may be given more than once. One request goes out per record, in input order, one at a time. One value prints per record. A failed record stops the run, the rows already printed stay printed, and one line on standard error names the record by its number, how many records finished, and how many of those a recording answered. `meta.tool` now names the binary in every result. How-to 04 is green over five live exchanges.

The sequential form alone is built. `--jobs`, `--cache DIR`, `--options POINTER`, `filter`, `rank`, and `annotate` are out of scope and none of them is started.

## The three modules the core gained

`json.rs` reads one record into the tool's own JSON tree. The clippy table bans `serde_json::Value` and `serde_json::Map`, because a wire body decodes into a typed struct. A user's record has whatever shape the user gave it, so it needs a dynamic value, and this is the one in the crate. The tree refuses a duplicate member name where it arrives and a number that is not finite with it. No dependency was added: `serde_json` already parses and `serde` gives the visitor that counts the names.

`pointer.rs` reads RFC 6901 and nothing else. `$.body`, `#/id`, a wildcard, and a negative index are each refused by their own name, and every message says "RFC 6901", so a user who typed another pointer language is told which one this tool reads.

`records.rs` holds `Reading`, which is the framing and the pointers together. It frames one record from bytes, strips a carriage return before the line feed, and builds the evidence one or several pointers send. Everything in all three modules is a pure function over bytes and values. The binary opens the file, reads standard input, and writes the rows.

## Red then green

- **A duplicate member name.** `{"id":1,"id":2}` parsed. Left `Ok(Object([("id", Number(1)), ("id", Number(2))]))`, right `Err(DuplicateName)`.
- **The fragment form.** `#/id` was refused as `NotAPointer` rather than as the URI fragment form. Left `Err(NotAPointer)`, right `Err(Fragment)`.
- **The carriage return.** `one line\r\n` reached the evidence as `one line\r`. Left `Ok("one line\r")`, right `Ok("one line")`.
- **The key clash.** `/a/text` and `/b/text` were accepted together. Left a `Reading` holding both, right `Err(KeyClash("text"))`.
- **`meta.tool`.** `Meta::new` took four arguments and the pinned line held no `tool`: `error[E0061]: this function takes 4 arguments but 5 arguments were supplied`.
- **`input` on a row.** `error[E0599]: no method named with_input found for struct DecisionResult`.
- **The whole record surface.** The seventeen cases of `tests/backend/streaming.rs` were run against the binary with the record path stashed. Sixteen failed with `error: unexpected argument '--jsonl' found` or `'--lines'`. The one that passed was the plan over one document, which the four-field plan already made.

Going green found one real defect. A 500 from the backend was retried twice before the run stopped, so the listener ran out of canned responses and the message read "the backend could not be reached" rather than "status 500". The case pins `--max-retries 0`.

Two refusals turned out to be `serde_json`'s own. `NaN`, `Infinity`, and `1e999` were already refused before any code of this ticket ran, the first two as a syntax error and the third as a number out of range. The rule is stated and held by a test, and the code that maps it is a belt on a parser that already buckles.

## Acceptance, bullet by bullet

| Bullet | The test that proves it |
| --- | --- |
| Each framing | `streaming::each_framing_prints_one_value_per_record_in_input_order`, `records::tests::each_framing_says_what_one_record_is` |
| One and several pointers | `streaming::one_pointer_sends_the_value_and_several_send_an_object_keyed_by_the_last_part`, `records::tests::one_pointer_sends_the_value_it_names_and_several_send_an_object` |
| The key clash | `streaming::a_record_whose_framing_and_pointers_cannot_act_together_sends_nothing`, `records::tests::the_framing_and_the_pointers_are_refused_when_they_cannot_act_together` |
| Each refused input, with zero requests for that record | `streaming::a_record_the_tool_refuses_stops_the_run_and_sends_nothing_for_itself`, `streaming::a_record_that_is_not_text_stops_the_run_at_exit_five`, `streaming::a_pointer_in_another_language_is_refused_by_name_before_any_request` |
| Order kept | `streaming::each_framing_prints_one_value_per_record_in_input_order`, which pins the three request bodies in order |
| The stop at the first failure, with the standard-error line | `streaming::a_backend_failure_stops_the_run_and_the_rows_before_it_stay_printed`, and the "What can go wrong" block of how-to 04 |
| The empty input | `streaming::an_empty_input_succeeds_with_no_output_and_no_request` |
| The record-mode plan | `streaming::the_record_mode_plan_shows_the_first_record_and_names_the_framing`, `streaming::the_plan_reads_no_further_than_the_first_record` |
| `input` under `--details` | `streaming::a_record_row_carries_the_whole_record_under_input`, `result::tests::a_record_row_carries_the_whole_record_under_input`, `streaming::a_document_row_carries_no_input_because_one_document_is_no_stream` |
| One output line per record | `streaming::the_row_count_equals_the_record_count_for_every_run_that_finishes` |
| The pinned `decide` digest and every recording still replay | The `spec` rung: `spec/decide.md` and thirteen green demos, all against committed recordings |
| The rows under `recipes/rows/` still replay | The `spec` rung: how-tos 13, 24, 25, 28, and 38 read them through the recipes |
| No key and no evidence in an error or a `Debug` line | `streaming::a_record_the_tool_refuses_stops_the_run_and_sends_nothing_for_itself` asserts the diagnostics name no word of the record, `records::tests::a_record_the_tool_refuses_names_no_part_of_itself` asserts the same of every `RecordError`, and the existing secrecy cases still hold |
| The spec rung is green with the key unset and touches no network | `sdlc/scripts/spec` unsets both variables and every demo replays |
| The ratchet equals the measured total | `sdlc/scripts/lint` |

The property that one row prints per record is a loop over zero to eight records in two views rather than a `proptest` case. `proptest` is a development dependency of the core alone, and `sdlc/scripts/policy.py` holds that list. Adding it to the binary would be a new dependency and would need a second review of its own, and the loop holds the same claim over the counts that matter. The parsers themselves do carry `proptest` cases: `pointer::tests::a_pointer_over_written_names_finds_the_value_they_nest` and `records::tests::framing_a_line_keeps_the_line`.

## The sentences added to `specification/records.md`

Six, and no other specification page changed.

1. "Under `--lines` a carriage return before the line feed is stripped with it, and a carriage return anywhere else in the line is kept."
2. "`$.body`, `#/id`, a wildcard, and a negative index are refused with a message that names RFC 6901, because the tool never guesses a pointer language."
3. "A JSON record that holds two members under one name is refused, because no reader can say which of the two a pointer means."
4. "A JSON record holding `NaN`, `Infinity`, or a number too large to be finite is refused, because none of the three is a JSON number."
5. "A record whose bytes are not valid UTF-8 is refused at exit 5, because bytes that are not text are a local failure rather than a record the tool read."
6. The `jobs` section said the setting has no home on the command line yet, which ADR 0010 had already contradicted. It now reads: "ADR 0010 gives it the advanced option `--jobs N` with a default of 4, and the sequential form landed first."

Sentence 5 is the one place where the page's own failure table and the landed code disagree. The table gives exit 2 to a record the tool refused before sending it. Bytes that are not text were a local failure at exit 5 from ticket 0005, and `spec/decide.md` pins that code in an executable block. Changing it would break a pinned page to gain nothing, so the sentence states the exception. Ian can overturn this cheaply.

## Choices made where the pages were silent

Each one is cheap to overturn.

- **Record mode is `--lines` or `--jsonl`.** `--field` without either one reads the whole input as one JSON value, and that run stays a single document: its answer sets the exit code, its row carries no `input`, and its plan carries four fields. The pages call that framing "one document" and the exit table calls it single input.
- **`input` appears on a row only in record mode.** A single-document `--details` row is unchanged, so every pinned page and every recorded digest holds.
- **The plan's `field` is always a list, even when it is empty.** `channels.md` shows the field with one pointer and never with none. An always-present list names "no pointers" honestly and gives a reader one shape.
- **The empty pointer's key is the empty name.** `--field ''` alone means the whole record. Two of them would end in the same empty name and are refused by the clash rule, so no extra rule was needed.
- **A `--dry-run` over an empty record stream prints nothing and exits 0,** the way an empty record stream does anywhere else.
- **`--input` that names no readable file is exit 5,** because `channels.md` gives 5 to a local failure over a file.
- **`--quiet` in record mode is allowed and prints nothing.** It suppresses output, which it can still do. It tells the caller nothing, because no record's answer reaches the exit code, and that is the caller's choice to make.
- **The stop line is a second line on standard error.** The first line is the cause and its wording, and the second is the count. `records.md` asks for one line about the stop, and a run that said only where it stopped would not say why.
- **The diagnostic names the pointer that found nothing.** A pointer is what the user typed on the command line, so it is not a record's content.

## The review

A second agent reviewed the widened public surface and the raised ceiling. Its findings and what was done with each are recorded in the commits that follow this file.

## The ladder

| Rung | Script | Result |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | exit 0 |
| 1 | `sdlc/scripts/lint` | exit 0 |
| 2 | `sdlc/scripts/test` | exit 0, 101 tests |
| 3 | `sdlc/scripts/spec` | exit 0, 13 green demos and 8 red |

The largest file is `crates/thinkthen/tests/backend/streaming.rs` at 474 lines, under the 500-line ceiling.

## The ratchet

5862 to 7406, in three commits that each say what grew. The core's three new modules and their tests are 800 lines, `meta.tool` and `input` are 46, and the binary's record path with its seventeen cases is 698. Duplication was looked for in `judge.rs`, where the single-document run now calls the same `Judging::one` the stream calls, so the request, the recording, the answer, and all three views are written once; in `edge.rs`, where the chunk reader replaced `evidence` rather than joining it; in `text.rs` and `render.rs`, whose blank-text values and compact writer the new code reuses; in `question.rs`, whose `Labels::checked` holds a repeat check over a list of owned strings that shares no type with the check over pointer keys; and in `tests/backend/harness`, whose `spawn` and `Listener` the new page reuses.

## Dependencies

None added, none removed.

## The live spend

One job, `demos/04-review-queue/record.sh`, through `sdlc/scripts/live`. Five exchanges for 1,453 input tokens. The ledger went from 32,842 to 34,295 of 476,000,000.

The live answers disagreed with the red page. MSG-04 reads "Cancel the second seat, keep mine." and answered 0.07 where the page had assumed a yes, and MSG-05 asks about pausing and answered 0.08. The page now says what the model said: one record cleared the high mark, three fell under the low one, and one landed in the band. The finding survives its point, which is that a partial cancellation is a different question from ending a subscription, so the page states it rather than rewording the question until the old numbers return.

`apikey_`, `authorization`, and `bearer` each appear zero times in every committed recording.

## Left for another ticket

- Ticket 0013 carries `--jobs N`, `--cache DIR`, and `choose --options POINTER`. Nothing here blocks bounded parallel requests: `Judging::one` takes one record's bytes and gives back one row, and the stop rule is the only state the loop keeps.
- The committed rows under `recipes/rows/` were written before `meta.tool` existed and carry no such field. They stay as they are, because no recipe reads it.
