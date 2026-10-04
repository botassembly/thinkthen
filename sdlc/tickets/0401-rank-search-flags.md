# 0401: Search flags on rank: files, windows, line numbers, neighbors, scores and several questions

Status: in progress. Slice A ticket review corrections are accepted at `2afd99d65364ebc4dd49258ed31f9990d6b460c4`. No product code exists. Implementation waits for ticket 0405 to answer filter, rank and grep and amend this scope as needed. The audit proposes separately reviewed shared intake, filter/rank display and find display slices in [0405-search-scope](../records/0405-search-scope.md); the rank-only outcome below is the prior design, not permission to omit those surfaces. No slice starts until the audit and amended slice scope receive fresh acceptance.

Milestone: 0.2

Lane: 3, first item. The docs story ticket and the DuckDB 1.5.4 extension ticket follow it in this lane.

Depends on: main at 0.2.0, as Ian ruled on 2026-10-04. Each slice lands on main and ships in 0.2.

Closes: `sdlc/issues/2026-10-03-search-features-positions-several-questions-and-grep-style-output.md` and `sdlc/issues/2026-10-03-rank-details-prints-value-null.md`. It answers item 3 of `sdlc/issues/2026-10-03-help-gaps-from-the-transcript-how-to.md` and leaves that issue open.

## Outcome

A user runs experiment 422's best transcript search as one command, with no awk, paste, sed or shell loop:

```sh
thinkthen rank @questions.json --window 10 --top 8 -n --around 5 --scores transcripts/*.txt
```

When all three slices land, these are true:

1. `rank` reads input files named after the question, in order. Line numbers restart at 1 in each file. `--input FILE` still works.
2. `rank --window N` joins every N lines of one file into one record. A window never crosses from one file into the next.
3. `rank -n` (long form `--line-number`) prints where each record starts, as `grep -n` and `grep -H` do.
4. `rank --around N` prints the N lines before and after each printed record, as `grep -C` does.
5. `rank --scores` prints the number that ordered each record before the record.
6. `rank @SET.json` reads a question set of `decide` questions, ranks the records once per question, and merges the lists by turns. The merge rule lives in `crates/thinkthen/src/core/order.rs`, so every surface that adds it later holds the same cases.
7. Every `rank --details` row carries `position`: the file, the first line and the last line of the record.
8. The Rust crate ranks over a question set by the same merge rule. Every binding already returns each ranked record's input index and probability, and that stays as it is.
9. A command line that uses none of the new flags sends the same requests, prints the same bytes and writes the same question digests as 0.1. The one exception is the new optional `position` member on `--details` rows. `specification/result.md`'s compatibility rule lets a `thinkthen.result/1` release add a member to a row.

## Evidence

- Starts from: the 0.1 `rank` command and library, three measurements, and one issue.
  - The library already returns positions. `Ranked<T>` in `crates/thinkthen/src/public/results/ranked.rs` carries `index()` and `probability()`, and `Engine::rank_with` in `crates/thinkthen/src/public/bulk.rs` fills them. The C door's `RankedRow` writes `{"index","record","probability"}`, and `libraries/BINDING-AUTHOR.md` makes every binding return those three members. The Python binding (`libraries/python/thinkthen/__init__.py`, `rank`) and the TypeScript binding (`libraries/typescript/index.js`, `rank`) both map `index` back to the caller's record. So the issue's library item 1 is already true on every binding. The library takes a list, and an index is a position in it. A file and line exist only on the command line.
  - The command does not use the library's `rank`. `judge::rank` in `crates/thinkthen/src/cli/judge.rs` runs the shared record pipeline, and `Output::ordered` in `crates/thinkthen/src/cli/schedule.rs` holds each `Judged` row and sorts with `core::ranking` when the input ends. Each row's printed line is rendered in `Judging::row_of` (`crates/thinkthen/src/cli/asking/row.rs`) before the order is known.
  - The command already knows each record's line. `edge::numbered` in `crates/thinkthen/src/cli/edge.rs` numbers every physical line from 1, blank lines included, and skips blank records. `Held.at` in `crates/thinkthen/src/cli/asking/judged.rs` carries that number. Nothing carries it to the printed row. `Held.at` also supplies ordered pipeline labels and request spans; file-local positions must not replace those global identities.
  - A trailing argument is refused today. `RankArguments.extra` in `crates/thinkthen/src/cli/args.rs` catches it, and `Command::stray` makes `cli::run` refuse it with `hint::ONE_QUESTION`. `crates/thinkthen/tests/backend/hints.rs::a_second_argument_says_where_the_evidence_goes` pins that sentence for `rank`.
  - Several questions per record already pack. `Planner::plan` in `judged.rs` passes a list of questions to `quoted_plan_of`. `pack::asks` in `crates/thinkthen/src/core/pack.rs` makes one `Ask` per question, keyed by `QuestionKey::of(url, model, state, question)`. `annotate` uses this path.
  - Experiment 422 measured search approaches against 15 quoted lines in three talk transcripts, 5,829 caption lines, in 36 live runs for $0.134. One broad question on 10-line windows found 5 of 15 in 300 lines read. Four narrow questions merged by turns found 11 of 15 in 300 lines and 13 in 600. Merging by highest probability found 9 of 15, because one question's best window scored 0.93 and another's scored 0.31. 10-line windows beat 5, 20 and 40. Half-step overlapping windows cost twice as much and added at most one quote. Five neighbor lines on each side of the top 8 windows per question raised recall from 12 of 15 in 310 lines to 14 of 15 in 570. A cut at 0.3 on any question found 11 of 15 in 420 lines. Keeping caption times in every line cost 56% more tokens and found one fewer quote. One question that names all four ideas found 9 of 15 in 300 lines for a quarter of the cost.
  - Experiment 0010 in `botassembly/thinkthen-exp` (`experiments/0010-thinkthen-navigate`) reproduced experiment 422's search step for step: 14 of 15 at 600 lines. On 50 QASPER papers, flat ranking of every paragraph answered 25 of 50 questions within 40 paragraphs, and a walk that picked documents first answered 19. Its answer 5 says navigate is a how-to over these features and not a command. Its answer 6 says all six features sit under the prototype: several questions by turns is the biggest recall lever, then neighbors, then 10-line windows, and line numbers and file arguments make the manifest possible.
  - Experiment 434 ranked 10-line windows of 8 cancer treatment papers for 32 labeled questions. It reached every answer range for 19 questions at 50 lines read and 29 at 200, where grep on question words reached 22 and 26. Paragraph windows reached 14 and 29. Adding 5 neighbor lines on each side cut full recall to 9 to 12 at every budget. So neighbors stay off by default.
  - Experiment 433 ranked 50 article rows for each of 112 TREC Precision Medicine topics with one question. It scored below the search tool's own order at P@10 (0.314 against 0.379). The topics name a disease, a gene and a treatment, and its owner asked for several questions per run.
- Keeps: every 0.1 behavior of a `rank` command line that names none of the new flags. The one exception is the new optional `position` member on `--details` rows, standard input included, which the add-a-member rule in `specification/result.md`'s Compatibility section allows.
  - Request bytes, batching, `--top` bounds, exact-tie input order (`core::ranking`), `--context`, `--plan`, the cache key and `meta.question_sha256`. The tests in `crates/thinkthen/tests/backend/keeping/` (`rank_top.rs`, `graded_rank.rs`, `default_framing.rs`, `permutation.rs`) pass unchanged.
  - A ranked record prints as it arrived, with a line feed. A run that stops prints nothing on standard output.
  - `rank` orders and never selects. `--threshold` stays refused by name. A saved `score` question still ranks alone.
  - `value` stays `null` on a yes/no `rank --details` row. `result.md`'s compatibility rule says a `thinkthen.result/1` release never changes a member's type or meaning. Choices 1 and 2 of the value-null issue would change both for `rank` rows, so this ticket takes choice 3.
  - `filter`, `decide`, `find` and every other verb keep refusing a trailing argument with `hint::ONE_QUESTION`.
  - The library's `Engine::rank`, `Ranked<T>`, the C door's `RankedRow`, every binding's `rank`, and SQL `thinkthen_rank` keep their shapes.
- Changes: three slices, each with its own proof. Slice A lands first. Slice B needs A's positions. Slice C needs A and adds the question name to B's output if B has landed.
  - Slice A, input files, windows, line numbers and positions.
    - `RankArguments.extra` becomes a visible `[FILE]...` positional. `Command::stray` stops matching `Rank`. Files read in the order given, and line numbers restart at 1 in each file. Files beside `--input` are a usage error. A trailing argument that names no file is a usage error that names it and keeps the quote hint: ``thinkthen: `rank` reads each argument after the question as an input file, and `this` is not one; quote a question of several words``. A directory keeps the existing `Failure::InputDirectory` sentence and exit 5. Validate every named file before scheduling any record, so a later missing file or directory sends no request. `edge::waiting` stays silent when files are named. `-` has no special meaning.
    - `--window N` takes a whole number of 1 or more and needs lines framing, which is the default with no pointer. Validate the resolved framing and pointer after loading a saved question, including its `on`. Windowing needs lines with no pointer; explicit `--jsonl`, `--csv`, `--tsv`, `--field` and a saved question whose resolved `on` selects a field are refused before sending. Keep the existing pointer precedence and refusal behavior, including saved `on` beside explicit `--lines`. A window is N consecutive physical lines of one file. Their texts join with one space, after each line loses its line ending and any carriage return before it. A blank line counts toward N and adds no text. A window of only blank lines is skipped, as a blank line is today. The last window of a file may be shorter. `--window 1` sends the same requests as no `--window`. Windowing lives at the command edge beside `edge::numbered`, because Ian's 2026-10-02 split keeps windowing out of the library.
    - Carry the typed file path, physical first line and physical last line separately from cumulative pipeline indexes, question labels and request spans. File-local line numbers restart without resetting pipeline identities. Positions belong to output and stop diagnostics, never evidence, request bytes, cache keys or question digests.
    - `-n` and `--line-number` print `LINE:` before each record, or `FILE:LINE:` when two or more files are named. `LINE` is a window's first line. `FILE` is the path as typed. `-n` works under lines and JSONL framing. It is a usage error beside `--csv`, `--tsv` or `--details`, because a table row has no line and `--details` carries `position`.
    - `--details` rows from `rank` under lines or JSONL framing gain `position`: `{"file":F,"line":A,"end_line":B}`. `file` is present when the records came from a named file, through an argument or `--input`. `end_line` is present under `--window`. The member is added to `DecisionResult` in `crates/thinkthen/src/core/result.rs`, so `schema_tests.rs` regenerates `specification/result.schema.json`.
    - A stop diagnostic names the file when files are named. Today `crates/thinkthen/src/cli/failure/stopped.rs` writes `thinkthen: stopped at record {at}; ` and then the reason and the finished counts. With named files it writes `thinkthen: stopped at FILE line N; ` and then today's text unchanged. Standard input, and `--input` without named files, keep `stopped at record N`.
    - `Judged` in `schedule.rs` carries the record's position. The ordered `Output` applies the prefix when it prints.
    - `rank --help` says that `value` is null on a yes/no row and that the ordering number is `answer.probability`, or `value` for a saved `score` question. It adds one line on choosing between the two record verbs: `rank` suits exploring, `filter` suits gating, and a `filter --threshold` comes from a few labeled cases. That is item 3 of the help-gaps issue.
  - Slice B, scores and neighbors.
    - `--scores` prints the ordering number and a tab before each printed record: the yes probability, or the weighted value of a saved `score` question. The number is written by the same JSON number writer as `--details`, so it matches `answer.probability` or `value` byte for byte. With `-n`, the score comes first: `0.75<TAB>FILE:LINE:record`. `--scores` works under lines and JSONL framing. It is a usage error beside `--csv`, `--tsv` or `--details`, because a table row prints as a JSON object and `--details` already carries the number.
    - `--around N` takes a whole number of 1 or more and needs lines framing. It is a usage error beside `--jsonl`, `--csv`, `--tsv`, `--field` or `--details`. Each printed record becomes a group. A group starts with a `--` line, or `-- SCORE` under `--scores`. It then prints the N lines before the record, the record's own lines and the N lines after, each line as it arrived. Under `--window` a hit prints its original lines, not the joined text. Under `-n` a record line reads `FILE:LINE:text` and a neighbor reads `FILE-LINE-text`, as `grep` writes them. A group stops at the edges of its file. Groups print in rank order and never merge, so a lower hit that overlaps a higher one still prints whole.
    - `--around` holds every input line for the run, so it can print neighbors after the order is known. `rank.md` says that `--top` then bounds the printed rows but not the held lines.
  - Slice C, several questions taking turns.
    - `core/order.rs` gains `turns(orders, top)`, beside `ranking`. It takes one ranked list per question, in set order. Round 1 takes each question's first record in set order, round 2 each second record, and so on. A record already placed is skipped and not replaced, so four lists of 8 with one shared record print 31 records. `top` cuts each list before the merge. With one question, `turns` equals `ranking`.
    - `rank @FILE` reads a question set when the file holds `questions`, through `core::QuestionSet::parse`. Every member must be a `decide` question with no threshold and no `on`. A member of another verb is exit 2, because the command names the wrong verb for the file. A member threshold is exit 5, as a rank question file's threshold is today. A member `on` is exit 5 with a sentence that points at `--field`. `--true` and `--false` beside a set are exit 2. `--field` applies to every member. `--top N` keeps N records per question.
    - Each member asks its question exactly as a plain `rank` with that text would. The builder may pack every member into one plan per record, as `annotate` does, or run one pass per member over held records. Either way the proof below holds the requests to the single-question bytes.
    - A `--details` row from a set adds `question_name`, the member's name. Its `meta.question_sha256` is the member's single-question digest. `--scores` writes `NAME<TAB>SCORE<TAB>`, and the `--around` group line reads `-- NAME SCORE`. `--facts` counts each record once.
    - The Rust crate gains `Engine::rank_set` and `Engine::rank_set_with` over a `QuestionSet` with a per-question top. They refuse a non-`decide` member, a member threshold and a member `on` with `Error::Usage` before any send. They return `Ranked<T>` rows, and `Ranked<T>` gains `question()`, which names the member that placed the row and returns `None` from `Engine::rank`. Facts cover every pass.
  - Specification and records: `specification/rank.md` (synopsis, what it reads, what it prints, options, examples), `specification/result.md` (the `rank` row's `position?` and `question_name?`), `specification/records.md` (windows and named files), `specification/question-file.md` (a question set under `rank`), `spec/result.md`'s member table, `specification/settings.md` rows for input files, window, line numbers, around, scores and the rank question set, and `CHANGELOG.md`. Each slice updates the pages it changes.
- Proof: outside-in command tests against the loopback listener, an edge-case table for the merge rule, and one library test. Every case replays canned answers, and no case needs a live call. New command tests go in `crates/thinkthen/tests/backend/keeping/` under the 500-line cap.
  - Slice A:
    - A table over two files. The first has 5 lines with a blank third line. The second has 3 lines. Under `--window 2` it pins each request body's joined text, the windows (first file lines 1 to 2, 3 to 4 and 5; second file lines 1 to 2 and 3), and each `--details` row's `position`. No request joins lines from both files.
    - `--window 1` against no `--window`: a run with `--cache DIR` and then the other form against the same folder sends 0 requests, counted at the listener.
    - `-n` output pinned for standard input (`LINE:`), one named file (`LINE:`), two files (`FILE:LINE:`), JSONL framing, and a window (its first line).
    - Refusals, each exit 2 with its exact sentence and 0 requests at the listener: `--window 0`; `--window` with `--jsonl`, `--csv`, `--tsv` and `--field`; `-n` with `--csv`, `--tsv` and `--details`; files with `--input`; and `rank Is this urgent`, which names the missing file `this` and keeps the quote hint. `hints.rs::a_second_argument_says_where_the_evidence_goes` drops `rank` from its verb loop and keeps `decide`, `filter` and `find`.
    - A directory keeps exit 5 and its existing sentence. A missing file or directory named after a valid file sends zero requests. Saved `decide` and `score` questions with `on`, under default framing and explicit `--lines`, preserve existing pointer refusals and send zero requests beside `--window`.
    - A stop on a bad record in the second of two files names that file and physical line, prints nothing on standard output, and exits as today. A batch crossing two files keeps globally ordered question labels while details and stop diagnostics report file-local physical positions.
    - `--plan --window 3` prints the joined first record in its request and sends nothing, counted at the listener.
    - A yes/no `rank --details` row still prints `"value":null` beside `position`. `schema_tests.rs` regenerates `result.schema.json` with the new optional member.
  - Slice B:
    - `--scores` output pinned for a yes/no question, a saved `score` question, and beside `-n`. For each row the printed number equals the bytes of `answer.probability` or `value` from the same run under `--details`.
    - An `--around` table: a hit on a file's first line, a hit on its last line, a hit on the last line of the first of two files (no neighbor from the second file), a window hit (its lines with `:`, neighbors with `-`), two overlapping hits (both groups whole, in rank order), a blank neighbor line, and `--top 1` (one group). Each case pins standard output byte for byte.
    - Output-only flags change no request: a run with `-n --scores --around 5` against a cache recorded without them sends 0 requests, counted at the listener.
    - Refusals, each exit 2 with 0 requests: `--around 0`; `--around` with `--jsonl`, `--csv`, `--tsv`, `--field` and `--details`; `--scores` with `--csv`, `--tsv` and `--details`.
    - `--scores` under JSONL framing prints the score, a tab and the record as it arrived.
    - A closed reader (`| head -n 1`) stops an `--around` run as `crates/thinkthen/tests/backend/closed_pipe.rs` proves for plain `rank`.
  - Slice C:
    - An edge-case table in `core/order.rs` for `turns`: two lists sharing a record (skipped, not replaced), lists of unequal length, `top` per list, empty lists, and one list (equal to `ranking`). Two property tests: no place appears twice, and one list gives the same order as `ranking`.
    - A command case with two members on different probability scales. Merging by highest probability would put three records of one member first. The printed order is the turns order.
    - Cache identity: two plain `rank` runs, one per member text, record into `--cache DIR`. `rank @set.json --cache DIR` then sends 0 requests at the listener and prints the turns order. Each `--details` row's `meta.question_sha256` equals the single run's digest and carries `question_name`.
    - A one-member set prints the same bytes as the plain `rank` of that member.
    - Refusals with 0 requests: a `choose` member (exit 2), a member threshold (exit 5), a member `on` (exit 5), `--true` beside a set (exit 2).
    - `--scores` and the `--around` group line name the member, and `--facts` reports one record per input record.
    - A library test in `crates/thinkthen/tests/public_batches/`: `Engine::rank_set` returns the turns order with each row's `index`, `probability` and `question()`, its facts count every pass, and a set holding a `choose` member is refused with 0 requests at the listener.
  - Every slice: `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`, `sdlc/scripts/lint`, the focused test files, and the spec gate at the checkpoint the coordinator names. `sdlc/ratchet.json` grows by the measured source total, and the commit says why.
- Defers: these wait for a later ticket or another surface. Each is named so it is not lost.
  - Several-question rank on the C door, the bindings and the SQL extensions. The issue planned it for every binding, but each library under `libraries/` needs its own declarations, tests and package checks. The merge rule sits in `core/order.rs`, so a later ticket adds it to each surface against the same cases. The builder files a new issue for it when this ticket closes the search issue.
  - The same flags on `filter`. The docs team asked for files, windows and `-n` on `filter` too. Ian's 0.2 scope names `rank`. The reading code is shared, so a later ticket costs little.
  - `--window N:S` with a step. Overlap doubled the cost in experiment 422 and added at most one quote.
  - `--strip REGEX`. Windows work without it. Experiment 422 measured its gain as cost (56% more tokens with times kept), and it would add a pattern language to the command line.
  - Paragraph windows split on blank lines and headings. In experiment 434 they reached 14 and 29 of 32 questions where 10-line windows reached 19 and 29.
  - `--merge max`. Turns beat it in experiment 422, 11 of 15 against 9.
  - `-A`, `-B` and `-C`. No measurement asked for uneven sides.
  - Joining touching `--around` groups. Navigate's manifest wants it, and that is the docs recipe's job.
  - A `score` member in a rank question set, and `position` for CSV and TSV rows.
  - `rank --threshold`. `sdlc/issues/2026-10-01-rank-keeps-only-records-over-a-threshold.md` stays open. `--scores` covers the stopping rule experiment 422 measured, and the approved 0.2 scope does not name a rank cut.
  - Items 1, 2 and 4 of the help-gaps issue: the `--record` help line, the repeated charge warning, and `requests_sent` on batched rows. They stay open in `sdlc/issues/2026-10-03-help-gaps-from-the-transcript-how-to.md` for a later Quick Fix or the docs story ticket 0402.
  - The navigate how-to and the transcript how-to. Both are docs recipes over these flags and go in the docs story ticket, which follows in lane 3. The navigate draft issue stays open for that recipe.

## Notes for the builder

- Pin every new sentence and every printed byte in the tests. Count requests at the listener to prove "sends nothing".
- Keep windowing, neighbors and prefixes at the command edge. Only `turns` goes in `core`.
- Grep conventions are the model: `:` after a record line's number, `-` after a neighbor's, and the file name only when two or more files are named. The one difference is that every `--around` group starts with a `--` line, so a score can ride on it.

## What Ian can overturn

- Several-question rank in the Rust crate only for 0.2, with the C door, the bindings and SQL later.
- The flags on `rank` only, with `filter` later.
- `value` staying null on yes/no `rank` rows, with `position` added beside it.
- The names `--window`, `-n`/`--line-number`, `--around`, `--scores` and `question_name`, and turns as the only merge.
- `--around` groups that never merge.

## Slice A ticket review corrections

The fresh ticket review required resolved framing checks for saved `on`, separate file-local positions and cumulative pipeline labels, directory exit 5, and complete file validation before scheduling. The coordinator recorded those corrections before implementation. The baseline checkpoint passed all 21 surfaces and the packed-release smoke on `e760432c80dc22501fa51694964f2eeec5b8e63e`, tagged `checkpoint/surfaces/2026-10-04-1`.
