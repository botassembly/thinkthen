---
flow: build
priority: 153
opens: crates/thinkthen/tests/backend/threshold_args.rs crates/thinkthen/src/core/mod.rs crates/thinkthen/src/cli/hint.rs crates/thinkthen/src/cli/mod.rs crates/thinkthen/src/cli/args.rs crates/thinkthen/src/cli/args/command.rs crates/thinkthen/src/cli/args/find.rs crates/thinkthen/tests/hints.rs specification/channels.md specification/records.md sdlc/issues/2026-09-20-new-user-stumble-register.md sdlc/issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0153: Hints for a guessed verb, a table fed to `--jsonl`, and a second argument

Status: COMPLETE.

Opened as: 2026-10-11. on 2026-09-27. Fresh independent final review accepted `3e8ad635`, and the checked change landed with its issue-row closures. Owner: Codex.

Review route: Ian routed this work to Codex. A fresh read-only reviewer accepted code `9304b75c`, and a separate fresh reviewer accepted the bounded landing amendment below. A fresh final reviewer accepted the code corrections, ceiling and evidence at `3e8ad635`.

## Outcome and authority

A new user who types the verb the headline sells, feeds a CSV file to `--jsonl`, or puts a file path after the question gets one line that names the right function or option. The tool refuses at exit 2, as it does today. It never runs the function it names, never reads the file it was handed, and never guesses which one the user meant.

This is row H5 of `sdlc/planning/backlog-0-1-2026-09-26.md`, "hints for a guessed verb", in lane 4. It settles item 5 of `sdlc/issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md` and rows 3, 5 and 6 of `sdlc/issues/2026-09-20-new-user-stumble-register.md`. The backlog row says it touches the command's argument handling and merges after ticket 0146. Goal 4 of that backlog covers it. The coordinator authorized the ticket on 2026-09-26.

## What happens today

Observed on 2026-09-26 with the debug binary built at `origin/main` `a2fe448d`, no key, the base address on a closed local port, and `--no-cache`.

| Command line | Standard error | Exit |
| --- | --- | --- |
| `thinkthen grep x` | `error: unrecognized subcommand 'grep'`, then the usage line | 2 |
| `thinkthen if x` | The same, plus `tip: a similar subcommand exists: 'diff'` | 2 |
| `thinkthen sort x` | The same, plus `tip: a similar subcommand exists: 'score'` | 2 |
| `thinkthen classify x`, `summarize` | `unrecognized subcommand` and the usage line | 2 |
| `thinkthen filter 'Q?' --jsonl < table.csv` | `thinkthen: the record is not valid JSON`, then `thinkthen: stopped at record 1; 0 records finished` | 2 |
| `rank`, `find`, `decide`, `choose`, `tag` and `score` with `--jsonl < table.csv` | The same two lines. `rank` adds its held clause to the second | 2 |
| `thinkthen decide 'Q?' README.md` | `error: unexpected argument 'README.md' found`, then the usage line | 2 |
| `thinkthen filter 'Q?' README.md` | The same | 2 |
| `thinkthen annotate checks.json README.md` | `thinkthen: the second path is input; write it as `--input FILE`` | 2 |

Two of clap's tips point the wrong way. `if` suggests `diff`, and `sort` suggests `score`. A run with no key still parses record 1 before it looks for a key, so the JSONL case reaches the stop line with nothing sent.

## Design

### A guessed verb

`cli/mod.rs::entry` calls `Cli::try_parse_from` in place of `Cli::parse_from`. On an error it calls `hint::refused(error)`, in a new file `cli/hint.rs`. When the error kind is `clap::error::ErrorKind::InvalidSubcommand` and its `ContextKind::InvalidSubcommand` value is a word in the table below, `hint::refused` writes one line to standard error, ``thinkthen: `WORD` is not a command; TAIL``, and returns exit code 2. WORD comes from the table, never from the command line, so the line echoes no typed byte. The same error arrives for `thinkthen help grep`, so that command gets the same hint. Every other error goes to `error.exit()`. So `--help`, `--version` and every other clap message keep their bytes and exit codes.

| Word | Line on standard error |
| --- | --- |
| `grep` | `` thinkthen: `grep` is not a command; `filter` keeps the records where the answer is yes `` |
| `if` | `` thinkthen: `if` is not a command; `decide` answers one yes or no question in its exit code `` |
| `classify` | `` thinkthen: `classify` is not a command; `choose` picks one option, and `tag` names every label that fits `` |
| `switch` | `` thinkthen: `switch` is not a command; `choose` picks one option, and `tag` names every label that fits `` |
| `sort` | `` thinkthen: `sort` is not a command; `rank` sorts records by how likely the answer is yes `` |
| `summarize` | `` thinkthen: `summarize` is not a command; thinkthen judges text and writes none `` |
| `rewrite` | `` thinkthen: `rewrite` is not a command; thinkthen judges text and writes none `` |

Each tail reuses the words of the named function's own help line. The word is matched exactly. `cli/hint.rs` holds every hint text: `GUESSED`, a `const` array of (word, tail) pairs; `CHOOSE_OR_TAG`, the one tail `classify` and `switch` share; `WRITES_NONE`, the one tail `summarize` and `rewrite` share; and the sentences `NOT_JSON_LINES` and `ONE_QUESTION` below.

### A table fed to `--jsonl`

`cli/mod.rs::told` already restates a failure in the words the command line calls for. It gains one arm. The failure is `Failure::Stopped` at record 1, and its cause is `Failure::Record(RecordError::Json(JsonError::Syntax { .. }))`. The command line typed `--jsonl` and no `--field`. Then the cause becomes `Failure::Usage(NOT_JSON_LINES)`, and the stop line after it is unchanged. The sentence is:

`the record is not valid JSON; read a table with `--csv` or `--tsv`, and plain text with `--lines``

The accepted scope was corrected on 2026-09-27 after the builder found that `core::json` is private. `core/mod.rs` re-exports `JsonError` within the crate under `#[cfg(feature = "cli")]`, so `cli/mod.rs` can match `JsonError::Syntax` directly. This adds no public library surface and changes no parser or error behavior. Duplicate-name and nonfinite-number errors keep their existing messages. The command never classifies an error by its displayed sentence.

`Command::typed_jsonl(&self) -> bool` in `cli/args/command.rs` reads the two typed options. It sits beside `input()` and `timeout()`, which read one option of every verb the same way. `find` reads its own `FindCommon`.

### A second argument

`DecideArguments`, `FilterArguments` and `RankArguments` in `cli/args.rs`, and `FindArguments` in `cli/args/find.rs`, each gain one hidden positional:

```rust
/// Taken so the command can say where the evidence goes.
#[arg(value_name = "EVIDENCE", hide = true)]
pub(crate) extra: Vec<OsString>,
```

The field takes every loose word: a second path, the rest of an unquoted question, or a second value after an option that takes one, such as `--field /a /b` or `--input a.txt b.txt`. It is `Vec<OsString>`, so a word that is not UTF-8 still reaches the refusal rather than clap's UTF-8 error. `annotate`'s `extra_input` differs: it is `Option<PathBuf>`, holds one path, and a third word there still draws clap's refusal. This ticket leaves `annotate` as it is. `Command::stray(&self) -> bool` says whether one of the four holds any word. `cli/mod.rs::run` checks it right after the timeout check, before it reads input, and returns `Failure::Usage(ONE_QUESTION)`:

`the question is one argument and each option takes one value; quote a question of several words, and send evidence on standard input or as `--input FILE``

The sentence names no typed word, so it echoes nothing from the command line.

### `choose --help` names `tag`

The `choose` doc comment in `cli/args/command.rs` gains one sentence after its first line, as its own paragraph: "Use `tag` when more than one answer can apply." It shows in the long help.

### Pages

- `specification/channels.md`, "What the tool never does": after "An unknown word is a usage error", a sentence says seven guessed words name the function that does the job, and the table above in short form.
- `specification/channels.md`, "Arguments": a sentence says `decide`, `filter`, `rank` and `find` take one question and each option one value, and a loose word is refused with `ONE_QUESTION` at exit 2.
- `specification/records.md:75`: the malformed JSONL bullet says that under a typed `--jsonl` with no `--field`, a failure that stops at record 1 is refused with the longer sentence, and the stop line follows as before.

## Decisions

Each is the agent's decision within the backlog row. Ian can overturn any of them.

1. **A hint is a refusal with a pointer.** Each case exits 2 before any request, as today. The tool never runs the named function and never takes the typed word as that function. `channels.md` says an unknown word is a usage error, and it stays one.
2. **The verb table matches exact words only.** No case folding, prefix or distance match. `Grep` and `gerp` keep clap's refusal and clap's tip. A fuzzy match would be the tool guessing.
3. **The table holds the seven words the issue names.** `grep`, `if`, `classify`, `switch`, `sort`, `summarize` and `rewrite`. Row 3 of the register also saw `split`. No one function does a split: `choose` with `awk` splits a file by label, and `find` picks one line. So `split` keeps clap's refusal. Row 3 also saw `tag`, which is now a verb.
4. **`summarize` and `rewrite` name no neighbor tool.** Naming a product would be an outward claim in a public repository. Item 6 of the docs issue owns the refusals page that names a kind of neighbor for each refused job.
5. **The hint rides on clap's own error.** `try_parse_from` and `ErrorKind::InvalidSubcommand` mean the hint fires only where clap would refuse anyway. A scan of raw arguments before clap would repeat clap's grammar. The error context comes with clap's default features, so no dependency or feature changes.
6. **The JSONL hint fires on record 1 only, with `--jsonl` typed and no `--field`.** It fires wherever that failure arrives as a stop at record 1. `find --dry-run` parses every unit before it plans, so it stops that way and gets the hint. The other verbs' dry runs report the bare cause with no place, so they keep the plain sentence. `relate` reads one entity document and reports the bare cause with no stop line, so it keeps the plain sentence too. A table whose header row is itself valid JSON, such as one column named `123`, passes record 1 and fails later, so it gets no hint. A failure at record 1 says the framing is likely wrong. A failure at record 7 says one record is bad, and `--csv` would mislead. `--lines` refuses a pointer (ticket 0137), so the sentence would be wrong beside `--field`. The tool never inspects the bytes to see whether they look like CSV. The sentence repeats no input byte, as `records.md` requires.
7. **The JSONL hint lives in `told`.** `told` is the one place that restates a failure from the command line. After ticket 0146 the reader parses each record, so the parse site moves. `Stopped` and its cause stay, by 0146's own design. So the arm in `told` survives the move, and `cli/failure.rs`, which 0146 changes, stays untouched.
8. **The second-argument hint covers the four verbs that take one question.** `decide`, `filter`, `rank` and `find`. `choose`, `tag`, `score` and `recognize` read every later word as an option, label, level or kind, so an extra path there is a legal list entry. The tool cannot tell a path from an option without guessing. `annotate` keeps its own sentence, because its first argument is itself a path. `relate` takes no question positional.
9. **The loose-word hint never checks the filesystem.** `decide Is this urgent`, `decide 'Q?' README.md` and `decide 'Q?' --field /a /b` get the same sentence. It names each likely fix, so it never has to choose between them.
10. **The refusal comes before input is read.** `run` checks `stray` before `edge::waiting` and before any verb reads standard input. A pipe that never ends cannot hold the refusal back.

## Edge cases

| Command line | After | Kind |
| --- | --- | --- |
| `thinkthen grep x` | `thinkthen: `grep` is not a command; `filter` keeps the records where the answer is yes`, exit 2, empty standard output | Changed |
| `thinkthen if x`, `classify`, `switch`, `sort`, `summarize`, `rewrite` | Each word's line from the table, exit 2 | Changed |
| `thinkthen grep` with no other argument | The `grep` line, exit 2 | Changed |
| `thinkthen help grep` | The `grep` line, exit 2. Clap reports the same unknown subcommand | Changed |
| `thinkthen Grep x` | Clap's `unrecognized subcommand 'Grep'`, exit 2 | Kept |
| `thinkthen gerp x` | Clap's refusal, exit 2 | Kept |
| `thinkthen split x` | Clap's refusal, exit 2. Decision 3 | Kept |
| `thinkthen think about it` | Clap's refusal, exit 2 (`decide_edge.rs`) | Kept |
| `thinkthen --help`, `thinkthen decide --help`, `thinkthen --version` | Clap's bytes and exit 0 | Kept |
| `thinkthen decide --bogus` | Clap's `unexpected argument '--bogus'`, exit 2 | Kept |
| `thinkthen decide 'Q?' -- --threshold -.5` | The exact `ONE_QUESTION` sentence, exit 2, empty standard output; the words after `--` are positional | Changed |
| `thinkthen decide 'Q?' README.md` | `thinkthen: ` and `ONE_QUESTION`, exit 2 | Changed |
| `thinkthen decide Is this urgent` | The same sentence, exit 2 | Changed |
| `filter`, `rank` or `find` with a second argument | The same sentence, exit 2 | Changed |
| `thinkthen decide 'Q?' --jsonl --field /a /b` | The same sentence, exit 2. `/b` is a loose word | Changed |
| `thinkthen decide 'Q?' --input a.txt b.txt` | The same sentence, exit 2. `b.txt` is a loose word | Changed |
| `thinkthen decide 'Q?'` and a second word that is not UTF-8 | The same sentence, exit 2 | Changed |
| `thinkthen annotate checks.json README.md` | Its own sentence, exit 2 | Kept |
| `thinkthen choose 'Q?' a b README.md` | `README.md` is a third option | Kept |
| `filter 'Q?' --jsonl < table.csv` | The `NOT_JSON_LINES` sentence, then `thinkthen: stopped at record 1; 0 records finished`, exit 2 | Changed |
| `rank 'Q?' --jsonl < table.csv` | The sentence, then the stop line with its held clause, exit 2 | Changed |
| `find`, `decide`, `choose`, `tag`, `score`, `annotate` or `recognize` with `--jsonl < table.csv` | The sentence, then the stop line, exit 2 | Changed |
| `find 'Q?' --jsonl --dry-run < table.csv` | The sentence, then `thinkthen: stopped at record 1; 0 records finished`, exit 2. Decision 6 | Changed |
| `relate 'works_for=PER:ORG' --jsonl < table.csv` | The plain sentence alone, no stop line, exit 2. Decision 6 | Kept |
| `find 'Q?' --jsonl --dry-run` over a one-column table whose header is `123` | The plain sentence, then `thinkthen: stopped at record 2; 0 records finished`, exit 2. Decision 6 | Kept |
| `decide 'Q?' --jsonl --field /a` over a bad record 1 | The plain sentence and the stop line (`json_syntax.rs`) | Kept |
| `--jsonl` over a good record 1 and a bad record 3 | The plain sentence, stopped at record 3 | Kept |
| `decide`, `filter`, `rank`, `choose`, `tag`, `score`, `annotate` or `recognize` with `--jsonl --dry-run < table.csv` | The plain sentence, no stop line. Deferred gap | Kept |
| `filter @file --jsonl` whose file holds `on`, over a table | The sentence. Its `--lines` clause then draws the text-line refusal, which says why. Deferred gap | Changed |
| A record that is not UTF-8, or holds a duplicate name, at record 1 | Its own sentence | Kept |

## Proof

The four new tests live in a new file, `crates/thinkthen/tests/hints.rs`. Each runs the compiled binary with a cleared environment, a private `HOME`, no key, `--no-cache` where the verb reads input, and `THINKTHEN_BASE_URL` on a closed local port, the way `decide_edge.rs` does. `decide_edge.rs` holds 487 nonblank lines of its 500, so the tests take a new file. Each test is an edge-case table that pins the exact standard error, the exit code and an empty standard output.

| Test | Rows | Planted fault that turns it red |
| --- | --- | --- |
| `a_guessed_verb_names_the_function_that_does_the_job` | The seven table words, each pinned to its whole sentence. `Grep`, `gerp` and `split`, each pinned to clap's exact first line `error: unrecognized subcommand '…'` | (a) Restore `Cli::parse_from`: every hinted row prints clap's text. (b) Point `sort` at `score`: the `sort` row differs. (c) Match words without case: the `Grep` row prints a hint |
| `a_second_argument_says_where_the_evidence_goes` | `decide 'Q?' README.md`, `filter 'Q?' README.md`, `rank 'Q?' README.md`, `find 'Q?' README.md`, `decide Is this urgent`, `decide 'Q?' --jsonl --field /a /b`, `decide 'Q?' --input a.txt b.txt`, and `decide 'Q?'` with a second word of the bytes `0xFF 0xFE` | (d) Drop the field from `FindArguments`: the `find` row prints clap's text. (e) Refuse only when the word names an existing file: `this` and `urgent` name no file, so the run goes on with the question `Is` and stops for the missing key. The `Is this urgent` row prints ``thinkthen: the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent`` and exits 4 |
| `a_table_fed_to_jsonl_names_csv_and_lines` | A two-line CSV with a quoted comma into `filter`, `rank` and `find` with `--jsonl`, each pinned to both lines. `find --jsonl --dry-run` over the same table, pinned to both lines. `find --jsonl --dry-run` over a one-column table whose header is `123`, pinned to the plain sentence and `stopped at record 2; 0 records finished` | (f) Drop the arm in `told`: the four table rows print the plain sentence. (g) Drop the record-1 condition: the `123` row prints the hint. (h) Drop the no-`--field` condition: the existing `json_syntax.rs::jsonl_syntax_keeps_the_record_sentence_and_sends_nothing` turns red |

The fourth, a one-row test, `choose_help_names_tag`, runs `choose --help` and requires the exact line "Use `tag` when more than one answer can apply." on standard output. Plant (i) deletes the sentence.

The four questions for each new test:

- **`a_guessed_verb_names_the_function_that_does_the_job`.** It protects the exact hint for each headline word, and clap's refusal for every other word. A lost `try_parse` branch, a wrong pair, or a fuzzy match fails it. No test runs an unknown subcommand except `think` in `decide_edge.rs`, which checks the exit code only. It needs no hook: the binary is the boundary.
- **`a_second_argument_says_where_the_evidence_goes`.** It protects where the refusal fires and its sentence, for each of the four verbs. A missing field on one verb, or a refusal that checks the filesystem, fails it. No test passes a second word to these verbs. It needs no hook.
- **`a_table_fed_to_jsonl_names_csv_and_lines`.** It protects the hint's three conditions: record 1, typed `--jsonl`, and no `--field`. A missing arm, a hint at a later record, or a hint beside a pointer fails it or `json_syntax.rs`. Existing tests pin the plain sentence with `--field` only. It needs no hook: `find`'s dry run parses every unit before it plans, which is the real boundary, and no key is set.
- **`choose_help_names_tag`.** It protects the one route from `choose` to `tag` a reader of the help sees. Deleting the line fails it. `version.rs` pins only the first help line. It needs no hook.

No unit test is added. No existing test changes its expected output.

## Budgets

Nonblank lines, measured with `grep -c .` on the diff.

- `crates/thinkthen/src/core/mod.rs`: at most 2 net, for the command-only crate-private error re-export.
- `crates/thinkthen/src/cli/hint.rs`: at most 55, for `GUESSED`, `CHOOSE_OR_TAG`, `WRITES_NONE`, `NOT_JSON_LINES`, `ONE_QUESTION` and `refused`.
- `crates/thinkthen/src/cli/mod.rs`: at most 20 net.
- `crates/thinkthen/src/cli/args/command.rs`: at most 40 net, for `stray`, `typed_jsonl` and the `choose` help sentence.
- `crates/thinkthen/src/cli/args.rs` and `cli/args/find.rs`: at most 16 net together, four fields.
- `crates/thinkthen/tests/hints.rs`: at most 150.
- `specification/channels.md` and `records.md`: at most 12 lines changed together.
- `sdlc/ratchet.json` moves to the measured total in the commit that adds the code. The commit says what grew. The builder looks for duplication to delete first in `cli/mod.rs` and in the `Command` accessors, which match every verb one by one.
- No dependency and no clap feature change. No public library type, method or message changes, so the `surfaces` rung is not required.

## Stop rules

1. Stop before starting if ticket 0146 has not landed on main.
2. Stop before crossing a budget, adding a dependency, or changing clap's features.
3. Stop if any plant stays green.
4. Stop if a record-1 JSON syntax failure after 0146 no longer reaches `told` as `Failure::Stopped` at 1 with a `Failure::Record` cause. The design of decision 7 then needs a new review.
5. Stop if a hidden field changes any byte of any help or usage output, or any `spec/` page or green demo turns red.
6. Stop if any existing test's expected output has to change.
7. Stop if the change needs `cli/failure.rs`, `cli/judge.rs`, `cli/asking.rs` or `cli/schedule.rs`, which ticket 0146 owns.

## Scope and exclusions

Excluded: `site/`, which marketing owns. A hint on `--dry-run` for a table fed to `--jsonl`. A second-argument hint on verbs that take a list. Any change to what a verb reads or sends. Guessing a framing from the bytes, which `records.md` rules out.

## Build order with ticket 0146

Ticket 0146, on `ticket/0146-command-batches-decide-filter-rank`, is accepted and not yet built. Its `opens` line names every file both tickets touch:

| File | 0146 | 0153 |
| --- | --- | --- |
| `crates/thinkthen/src/cli/args.rs` | Flattens a new `Batching` struct into `DecideArguments`, `FilterArguments` and `RankArguments` | Adds one hidden `extra` field to the same three structs |
| `crates/thinkthen/tests` | Opens the whole folder | Adds one new file, `tests/hints.rs`. No existing file changes |
| `specification/records.md` | Batching, order and failure lines | One bullet at line 75 |
| `specification/channels.md` | Batching rules | Two sentences, in "What the tool never does" and "Arguments" |
| `sdlc/ratchet.json`, `sdlc/records`, `sdlc/tickets`, `sdlc/issues` | Its own record, ticket and ceiling | Its own record, ticket and ceiling |

0153 touches no other file 0146 opens. `cli/mod.rs`, `cli/args/command.rs`, `cli/args/find.rs` and the new `cli/hint.rs` are outside 0146's list. The semantic tie is decision 7: 0146 moves record parsing into the reader, and 0153's `told` arm reads the stop that parsing makes.

Order: 0146 builds and lands first. 0153 builds after it, from a merge of that main. The 0153 builder merges `origin/main` before the final run, and rechecks stop rule 4 against 0146's reader. Tickets 0147 and 0152 also open `channels.md` and `records.md`. Whichever lands later merges the pages by hand.

## Routing

Builder: Claude (Opus subagent), in the lane the coordinator names. Reviewer: a fresh read-only Claude session for design and for code. The change raises the ceiling, so the code review names what it checked.

## Complexity

Contract 2; state and timing 0; reach 1; proof 1; cost of error 1; total 5. Final level: 1. The risk is a hint that fires where it misleads, which plants (c), (e), (g) and (h) guard.

## Deferred gaps

- `--dry-run` over a table fed to `--jsonl`, on every verb but `find`, prints the plain sentence with no stop line, because the dry run reports no record place. After 0146 a dry run reads up to the first closed batch, so the place is not always record 1.
- `filter @file --jsonl` whose question file holds `on` gets the hint, and its `--lines` clause then meets the text-line refusal. `told` sees the command line, not the settled pointer.
- `choose`, `tag`, `score` and `recognize` take a path after the question as an option, label, level or kind. The tool cannot tell which the user meant.
- `split` gets clap's refusal. A split how-to page would give it a place to point.
- A pointer typed with `--field` beside `--jsonl` over a table gets the plain sentence.

## What Ian can overturn

- Decision 3: the seven words, and no hint for `split`.
- Decision 4: no neighbor tool named for `summarize` and `rewrite`.
- Decision 6: the JSONL hint only at record 1, only with `--jsonl` typed and no `--field`.
- Decision 8: no second-argument hint on verbs that take a list.
- The exact wording of each sentence.

## Closes

- Item 5 of `sdlc/issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md`. The lander marks it fixed by ticket 0153 with the landing commit. The issue stays open for its other items.
- Rows 3, 5 and 6 of `sdlc/issues/2026-09-20-new-user-stumble-register.md`. The lander closes each row with the landing commit. The register stays open until launch, as its status line says.

## Evidence

- Starts from: Item 5 of the docs issue, which checked the debug build on 2026-09-25 and named the verb table, the `--csv` and `--lines` hint, and the second-path hint. Rows 3, 5 and 6 of the stumble register, observed by command on 2026-09-20 with an empty environment and a closed port. The table under "What happens today", observed on 2026-09-26 at `origin/main` `a2fe448d` with no key. `annotate`'s `extra_input` refusal at `cli/annotate.rs:144` and its pinned test at `tests/backend/annotate.rs:228`. `told` in `cli/mod.rs`. Ticket 0137's rule that `--lines` refuses a pointer. Ticket 0146's design, read from `ticket/0146-command-batches-decide-filter-rank` at `6cdbe075`: parsing moves to the reader, and input refusals keep today's cause and stop line.
- Keeps: Exit 2 for every case. Clap's messages for every word outside the table, every unknown option, `--help` and `--version`. `annotate`'s second-path sentence. The plain JSONL sentence at a later record, beside `--field`, under `--dry-run` on every verb but `find`, and on `relate`. Every existing test, `spec/` page and demo.
- Changes: Seven guessed verbs print one sentence that names the function. `decide`, `filter`, `rank` and `find` refuse a loose word with one sentence naming one question, one value an option, quoting and `--input FILE`. A typed `--jsonl` with no `--field` that fails at record 1 names `--csv`, `--tsv` and `--lines`. `choose --help` names `tag`. `channels.md` and `records.md` say so.
- Proof: The four outside-in tests under "Proof", each an exact-sentence table over the compiled binary with no key and no network. Plants (a) to (i) each turn a test red, and (h) turns the existing `json_syntax.rs` row red.
- Defers: A hint under `--dry-run`, a hint that knows the settled pointer, second-argument hints on list verbs, and a place for `split`.


## Bounded landing amendment, 2026-09-27

The final checks found two conflicts between this ticket and retained checks. The `choose` help sentence now says exactly “Use `tag` when more than one answer can apply.” This keeps the pointer to `tag` without calling a choose option a label. The approved vocabulary and its plants stay unchanged. Update the new exact-sentence hint test with the sentence; this is the only product wording correction.

After the explicit `--` terminator, `--threshold` and `-.5` are literal positional words. For `decide QUESTION -- --threshold -.5`, the accepted extra-word hint correctly gives `ONE_QUESTION`, exit 2 and no output. Update only the final case in `threshold_args::non_numeric_option_tokens_keep_claps_existing_meaning` to pin that exact sentence instead of the broad `error:` prefix. Keep its other five rows, all actual threshold parsing, and the assertion that no threshold-value diagnostic appears. No product parser change is needed for this correction. The original stop rule 6 has this one reviewed exception; every other retained expected output remains unchanged. Add the boundary case to the table above when building. The original gate failures provide the red evidence.

The new exact file is `crates/thinkthen/tests/backend/threshold_args.rs`; claim it before editing. No dependency or feature changes. The file may grow by at most five nonblank lines; existing ticket budgets remain. Run the hints and threshold-argument tests, approved-help vocabulary via the normal demos check, formatting, relevant Clippy and ratchets. Reuse passing unaffected checks from `3dadbb76`; record what ran and what was skipped. The full test rung stopped inside the backend test binary, so finish every not-yet-run root test binary, doctest, external consumer and script step from `sdlc/scripts/test`. No full surfaces rerun is required by the accepted budget. A fresh code review must also cover the two post-review lint corrections (in-place boxed value and scoped expect annotation), this amendment, and exact verification evidence before landing.
