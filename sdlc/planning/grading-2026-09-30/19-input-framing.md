# Area 19: Input framing

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

Input framing turns the bytes a command reads into records (one document, lines, JSON Lines, CSV or TSV), refuses what a record may not hold, and selects the part of each record that leaves the machine through `--field` pointers.

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/`, `sdlc/` or `conformance/`. Line counts are nonblank lines. `cli/file_size.rs` and `cli/normalize.rs` are not framing. `cli/file_size.rs` (36) installs the `SIGXFSZ` policy, and `cli/normalize.rs` (78) joins a negative `--threshold` before Clap. The `@FILE` cap lives in `public/question_file.rs` (46) and `cli/question_text.rs` (21), and this report counts those two.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code, core | `core/records.rs` (467), `core/pointer.rs` (229), `core/json.rs` (303) |
| Code, edge | `cli/table.rs` (408), `cli/edge.rs` reader part (`Chunks`, `numbered`, `source`: about 90 of 486), `cli/asking/reading.rs` (21), `cli/question_text.rs` (21), `public/question_file.rs` (46) |
| Code, per-command readers | `cli/asking/judged.rs:36-70`, `cli/annotate.rs:108-121`, `cli/relate/input.rs` (91), `cli/find.rs:152-180` |
| Code, shared text types | `core/text.rs` (411; `Evidence` only is framing) |
| Tests | About 75 tests. Unit: `core/records/tests.rs` (21, one a proptest at `:402-424`), `core/pointer.rs` (4), `core/json.rs`, `cli/edge.rs` (7), `cli/table.rs` (5). Integration under `crates/thinkthen/tests/backend/`: `blank_lines.rs` (2), `json_syntax.rs` (6), `pointer_echo.rs` (1), `table.rs` (13), `from_record.rs` (7), `record_values.rs` (3), `streaming.rs` (17) and `streaming/record_rows.rs` (1) |
| Contract | `specification/records.md` "Where the input comes from", "CSV and TSV", "`--field POINTER`", "Several pointers", "What a record may not hold", "Portable spelling of a selected batch record" (`:87`); `specification/question-file.md`; `conformance/record-values.json`; `specification/fixtures/batching/portable-*`; ADR 0007, 0008, 0010, 0026, 0028, 0048 |

## Complexity: 3 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 3 | About 1,700 nonblank lines in the framing code (records 467, pointer 229, json 303, table 408, reader about 90, four per-command readers about 200) |
| States and concurrency | 2 | Sequential. A bounded line reader (`cli/edge.rs:61`, `:290-330`) and an incremental table parser with a 16 MiB row bound (`cli/table.rs:121-200`). No thread of its own |
| Rules and refusals | 4 | About 38: 15 `RecordError` kinds (`core/records.rs:61-111`), 3 `ReadingError`, 6 `PointerError` (`core/pointer.rs:18-37`), 3 `JsonError` (`core/json.rs:26-43`), 9 table errors (`cli/table.rs:28-101`), plus the 16 MiB cut, the 1 MiB `@FILE` cap, the directory input and the blank-line rule |
| Surfaces touched | 5 | The command frames input. The selected-record spelling is a shared byte contract: 18 of the 21 bindings read `specification/fixtures/batching/portable-*` (15 of the 18 libraries (all but Polars, C# and Rust) and all three `databases/*`). Counts the command plus 18 |
| Settings | 3 | Five rows: Input file, Framing, Evidence pointer, Details, Kind pointer |
| Contract weight | 4 | Six spec pages with sections (`records.md`, `settings.md`, `question-file.md`, `annotate.md`, `relate.md`, `find.md`) and six ADRs (0007, 0008, 0010, 0026, 0028, 0048): 12 |
| Churn and debt | 3 | 15 commits since 2026-09-23 on the framing paths, two of them fixes (`7ecf7a8bd`, `ddbb2257c`) and one edge cleanup (`e1fc25543`, "one 16 MiB edge per reader"). Not rewritten. No open issue names the area |

Mean 3.4, rounded to 3.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | Behavior matches the contract on the primary paths. A line ends at LF with a CR before it removed (`core/records.rs:344-347`), a whole document keeps every byte (`:345`, "nothing ends a whole input"), invalid UTF-8 is a local failure (`:364`), an over-16 MiB record is refused before any request (`:361`), and the reader stops 2 bytes past the limit and never frames the tail as a record (`cli/edge.rs:310-322`). The BOM is stripped from the first CSV header (`cli/table.rs:207-214`) and refused as a JSON syntax error elsewhere (`core/json.rs:311`; `tests/backend/json_syntax.rs:39`, `:68`). Drift and gaps. (1) `records.md` says nothing about a BOM under `--lines` or one document, where the code keeps it in the evidence sent to the backend (`core/records.rs:374`, no strip). No test pins it. (2) The cleanup page still says the `@FILE` reader "has four copies, filed as debt" (`sdlc/planning/cleanup-2026-09-30.md:168`). Ticket 0345 closed that (`sdlc/issues/closed/2026-09-30-question-file-reader-copies-and-uncapped-loaders.md`), and the SQL extensions keep their own 1 MiB readers. (3) `records.md:37` lists "LF and CRLF records", and csv-core also ends a record at a lone CR (`cli/table.rs:185-189` counts a lone CR as a terminator). Unconfirmed: I did not run a lone-CR table |
| Reliability | B | Failure paths are tested with sends counted or exit codes pinned: syntax errors name their input and send nothing (`tests/backend/json_syntax.rs:36-134`), an invalid UTF-8 record and a whole document (`:205`), blank lines keep their numbers (`tests/backend/blank_lines.rs:19`, `:50`), a final line with no newline still judges (`tests/backend/streaming.rs:291`), table rule failures are exact (`tests/backend/table.rs:140`, `:212`), and a closed pipe stops reading (`tests/backend/table.rs:469`). A proptest pins that a line comes back as it arrived (`core/records/tests.rs:402-424`). Two fix commits in the last 7 days, each followed by tests. The 16 MiB table edge test is the slowest unit test in the area (about 4.5 s in ticket 0317's record). `relate` collects every JSONL or table record before its entity cap applies (`cli/relate/input.rs:62-68`, `:77`), so a very long file is held in memory. Read from the code; I did not measure it |
| Maintainability | C | The table-or-chunks framing dispatch is written out in four places: `cli/asking/judged.rs:41-70`, `cli/annotate.rs:110-121`, `cli/relate/input.rs:10-68` and `cli/find.rs:152-180` (find re-checks its own 16 MiB total), and `cli/recognize.rs` and its dry run repeat the same shape (unconfirmed, not read in full). A new framing needs edits in each. `Json::parse` decides its error kind by matching serde_json's message text (`core/json.rs:92-100`: a marker prefix and `"number out of range"`), so a serde_json upgrade can change the refusal with no compile error. `core/records.rs` holds 467 of 500 lines and `core/text.rs` holds 411, with name types unrelated to framing beside `Evidence`. `tests/backend/table.rs` is 493 and `tests/backend/streaming.rs` 489. Pure code is kept in `core` and `Debug` withholds evidence (`core/records.rs:119-133`). No lint suppression in the area |

## Strengths

- Framing is a pure function over bytes in `core`, and the edge owns reading. `Reading::new` refuses `--field` with `--lines` and clashing pointer keys at construction (`core/records.rs:280-298`).
- The pointer type refuses other pointer languages by name and never echoes a control character (`core/pointer.rs:53-69`, `tests/backend/pointer_echo.rs:12`). Index versus member follows RFC 6901, so `01` is a name (`core/pointer.rs:124-133`).
- Both readers bound what they read: the line reader takes `MAX + 2` bytes (`cli/edge.rs:61`, `:310`) and the table parser stops after an oversized row (`cli/table.rs:163-165`).
- A JSON record keeps member order, refuses duplicate names and non-finite numbers where they arrive (`core/json.rs:88-103`), and the contract states which number spellings need not survive (`specification/records.md:87`).
- `Record` and `Json` errors carry no evidence byte, and a test pins it (`core/records/tests.rs:205`, `tests/backend/json_syntax.rs:134`).

## Cleanup

1. **Write one framing dispatch.** Where: `cli/asking/judged.rs:41-70`, `cli/annotate.rs:110-121`, `cli/relate/input.rs:10-68`, `cli/find.rs:152-180`. Why: four copies of "table rows, or numbered chunks, each mapped to a placed failure". One `edge::frame(source, reading, kind)` that returns numbered records would keep new framings and the 16 MiB rule in one place. Size: M. Blocks 0.1: no.
2. **Pin and state the BOM rule for text framings.** Where: `specification/records.md:19-31` and `core/records.rs:364-374`. Why: the page covers the CSV BOM only, and a BOM at the start of a `--lines` file or a document is sent to the model as text. Choose strip or keep, write one sentence, add a row to `tests/backend/streaming.rs`. Size: S. Blocks 0.1: no.
3. **Stop reading error kinds from serde_json's message.** Where: `core/json.rs:92-100`. Why: the duplicate-name and non-finite kinds depend on message text. A serde_json bump can turn them into syntax errors with no failing build, though the tests at `core/json.rs:297-305` would catch it. Carry the kind in a custom error type, or keep the test as the guard and say so in a comment. Size: S. Blocks 0.1: no.
4. **Bound `relate`'s input by count as it reads.** Where: `cli/relate/input.rs:62-68`, `:77`. Why: `find` stops at its count (`cli/find.rs:160-162`), and `relate` reads the whole stream before its 400-entity check. Unconfirmed until measured. Size: S. Blocks 0.1: no.
5. **Correct the cleanup page's `@FILE` line.** Where: `sdlc/planning/cleanup-2026-09-30.md:168`. Why: it says four copies, filed as debt. Ticket 0345 paid it. Size: S. Blocks 0.1: no.
6. **Lift `core/text.rs` name types out of the framing file.** Where: `core/text.rs:123-215` (`ModelName`, `Url`). Why: 411 lines with four unrelated types; the next framing change touches the file. Size: S. Blocks 0.1: no.
7. **Test a lone CR in a table and a blank `--lines` line in `relate`.** Where: `tests/backend/table.rs`, `cli/relate/input.rs:73-80`. Why: both behaviors were read from code only. Size: S. Blocks 0.1: no.

## Confidence: medium-high

Read: all of `core/records.rs`, `core/pointer.rs`, the first 140 lines of `core/json.rs`, `cli/table.rs` lines 100 to 260, the reader in `cli/edge.rs`, `cli/asking/reading.rs`, `cli/question_text.rs`, the per-command readers in `judged.rs`, `annotate.rs`, `relate/input.rs` and `find.rs`, `specification/records.md` lines 1 to 90, and the names of every framing test. Checked commits and closed issues for churn.

Not checked: no test or build was run. `core/text.rs`, `cli/recognize.rs` and its dry run were not read in full. `conformance/record-values.json` and the 18 bindings corpus checks were counted by name, not read, so surface parity on the selected-record spelling is taken from the corpus README. Items 4 and 7 and the lone-CR claim are unconfirmed. The batching fixtures README still describes content cuts that ADR 0111 removed (area 1's item 5 owns that).
