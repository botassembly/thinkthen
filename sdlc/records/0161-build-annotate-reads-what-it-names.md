# 0161: Build `annotate` reading what its set names

Status: built 2026-09-26, awaiting code review. Owner: Claude.

Branch `ticket/0161-annotate-reads-what-it-names`, in lane 1. The ticket is `sdlc/tickets/0161-annotate-reads-what-it-names.md`, accepted at `93634805`. The build merged `origin/main` at the start, again after ticket 0158 landed, and again after ticket 0164 landed, before the last ladder run. Ian can overturn decisions 1, 3, 5 and 7 of the ticket. No live call ran. Every rung and plant ran with `THINKTHEN_API_KEY` unset.

## Result

- `QuestionSet::group_evidence` takes a `BatchRecord` and never parses text. A root group gets the record's evidence. A group that reads a part selects inside the record's object or array through the new `Reading::part`. Any other value is text, and the group is refused with `RecordError::TextPart`, which names the group's first question. `PartError::Render` is gone.
- `Reading::part` and `Reading::selected` share one private helper, `chosen`, so the `state` rule lives once. `batch_record` lost its `expect(dead_code)`. The module mark on `core/batch.rs` stays.
- `ReadingError::LinesPart` and `RecordError::TextPart` carry the ticket's two sentences. `QuestionSet::first_part` names the first question that reads a part. `cli/annotate.rs::run` refuses such a set under `--lines` right after it builds the reading, before any input is read.
- The command's `plan_for` passes `base.batch_record(record)?`. The libraries' `annotated` parses the record once, as the command reads a whole document, only when `first_part()` is some. A root-only library set builds its `BatchRecord` from the text and never parses it.
- `--dry-run` prints the first chunk of the first group, and the plan gains `request_count` and `group_requests` after `on`.
- `specification/annotate.md` carries the ticket's line 34, 81, 85, 89, 94 and 96 edits. How-to 16 prints no plan, so it did not change.
- `spec/annotate.md` runs the page's three examples against four hand-built `local-1` entries under `spec/fixtures/annotate/`. The request bodies came from a throwaway loopback in the session scratchpad. The digests follow `Exchange::digest` for `https://api.typesafe.ai/v1/systemone`, and the replies were written by hand.
- The three issues the ticket closes moved to `sdlc/issues/closed/`.

## Edge rows

| Row | Result |
| --- | --- |
| 1. JSON text in a `/body` string, `on: /x` | Exit 2, both stream lines, no request |
| 2. Plain words in a `/body` string | Exit 2, both stream lines, no request |
| 3. `/body` object, `on: /x` | Sends `"1"`, prints the record with `unresolved: true` |
| 4. `--lines` with a JSON-looking line | Exit 2, the one `--lines` line, no stop line, no request |
| 5. `--lines` with empty input | Exit 2, the one `--lines` line, no request |
| 6. `--lines` with a root-only set | Runs as today and sends the line as text |
| 7. Document `{"body":"text"}`, `on: /body` | Sends `"text"`, the object gains the answer |
| 8. Plain-text document, `on: /body` | Exit 2, the one document line, no request |
| 9. 250 questions, profile of 100, `--dry-run` | The request holds 100 questions, `request_count` 3, `group_requests` `[3]` |
| 10. Two `on` groups, no profile, `--dry-run` | The first group's request, `request_count` 2, `group_requests` `[1,1]` |
| 11. Library, two groups over `a private note` | `Usage` with the named `summary` sentence, no request |
| 12. Library, root-only set over `{"a":1,"a":2}` | Answers, and its one request sends that text byte for byte |

## Tests

- `on_reads_the_selected_value` in `crates/thinkthen/tests/backend/annotate_on.rs` covers rows 1 to 8. Each row pins standard output, the whole standard error, the exit code, and the `state` of every request the loopback saw.
- `the_plan_shows_each_request` in `crates/thinkthen/tests/annotate_plan.rs` covers rows 9 and 10. Row 10 pins the whole plan line.
- `each_group_sees_its_part_and_a_bad_part_sends_nothing` in the Rust consumer's `parts.rs` replaces the parser-sentence row with row 11 and adds row 12. Its other rows and the secrecy check stay.
- `spec/annotate.md` runs in the `spec` rung.

The four questions are the ticket's. No test needs a test-only hook: the loopback counts requests, `--dry-run` is the real plan, and the page replays ordinary entries.

## Plants

Each plant edited one source file, ran the named test under the heavy lock, and restored the file with `git checkout`. The tree was clean after each.

| Plant | Result |
| --- | --- |
| (a) Re-parse the evidence text in `group_evidence` | Red. Row 1 exits 0 and sends `1` |
| (b) Refuse every non-root `on` | Red. Row 3 exits 2 with the stream lines |
| (c) Drop the early check and refuse `--lines` in `Judging::record` | Red. Row 4 gains the stop line |
| (d) Print `plans.first()`, the unsplit group | Red. Row 9's request holds 250 questions |
| (e) Count one request per group | Red. Row 9's counts read `request_count` 1 |
| (f) Put `--field /body` back in the page's JSONL example | Red. `thinkthen` exits 2 with the named sentence and the page block fails |
| (g) Build the record from `Reading::evidence` and parse its text | Red. Row 1 exits 0 and sends `1` |
| (h) Run the old document parse in `bulk.rs` before the groups | Red. Row 11 reads the parser's sentence |
| (i) Parse every library record, root-only sets included | Red. Row 12 is refused |

The table loop stops at its first failing row, so plants (a), (b) and (c) name that row. The ticket's other rows for those plants were not observed on their own.

## Budgets

Nonblank lines, net against `origin/main`.

| Item | Budget | Measured |
| --- | --- | --- |
| `cli/annotate.rs` and `cli/annotate/` | at most 40 | 8 (3, 5) |
| `core/question_set.rs` | at most 15 | 12 |
| `core/records.rs` | at most 25 | 13 |
| `core/mod.rs` | at most 2 | 1 |
| `core/plan_document.rs` | at most 15 | 15 |
| `public/bulk.rs` | at most 15 | 15 |
| `cli/conformance_tests.rs` and `annotate_order.rs` | at most 8 | 6 (2, 4) |
| `tests/backend/annotate_on.rs` | at most 150, and one `mod` line | 147, and one `mod` line |
| `tests/annotate_plan.rs` | at most 80 | 78 |
| The consumer's `parts.rs` | at most 30 | 2 |
| `spec/annotate.md` | at most 90 | 37 |
| Pages | at most 20 | 0 net, six lines rewritten |
| `sdlc/ratchet.json` | the measured total | 298 above main |

No dependency was added.

## Ladder

| Rung | Result |
| --- | --- |
| `install` | exit 0 |
| `lint`, with `THINKTHEN_PRIVATE_NAMES` set | exit 0 |
| `test` | exit 0 |
| `spec` | exit 0, demos 21 green |
| `surfaces` | exit 0 |

The first `lint` run failed on `expect` in the new plan test's helpers. The helpers now return `io::Result`, and that fix cost five lines.

## Deviations

- The ticket's line numbers for `core/records.rs` had drifted by a few lines. The code matched its description.
- Plants (a), (b) and (c) name the first failing row, as the plant table says.
- Plant (f) makes `thinkthen` exit 2 inside the page's pipeline, and the page block fails with exit 1 under `pipefail`.

## Left for landing and later

- A fresh read-only Claude session reviews the diff. The ceiling raise and the public surface change need a second reviewer, and the coordinator arranges that. The public change is the library's sentence for a record that is not JSON text when the set has a part group.
- Ticket 0146 calls `batch_record` from its reader and finds no mark to remove in `core/records.rs`.
- Ticket 0160 merges the one `mod` line in `tests/backend/main.rs` if it lands second.
- A binding test of the named text sentence stays deferred, as the ticket says.
