---
flow: build
priority: 137
opens: crates/thinkthen/src/cli/asking.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/cli/failure/tests.rs crates/thinkthen/src/cli/args.rs crates/thinkthen/src/cli/args/command.rs crates/thinkthen/src/core/records.rs crates/thinkthen/tests/backend/keeping.rs crates/thinkthen/tests/backend/keeping crates/thinkthen/tests/backend/refused.rs specification/filter.md specification/rank.md specification/records.md specification/channels.md sdlc/planning/adr/0007-flat-verbs-bare-values-and-one-threshold.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0137: filter and rank read lines by default

Status: ready. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user types `thinkthen filter 'This mentions a refund.' < notes.txt` and gets the kept lines. Today that command stops with a usage error, so nearly every `filter` and `rank` example carries `--lines`.

The ask is `sdlc/issues/2026-09-26-filter-and-rank-could-read-lines-by-default.md`. The queue owner ruled on 2026-09-26 for its Option 1, with two guards. Ian can overturn the ruling.

- `filter` and `rank` read `--lines` when no framing flag is given.
- A pointer with no framing flag means `--jsonl` on these two verbs.
- The `--dry-run` plan names the framing, and says when the default chose it.
- `--lines`, `--jsonl`, `--csv`, and `--tsv` keep working unchanged.

## What happens today

`Common::framing` in `crates/thinkthen/src/cli/args.rs:175` returns `Framing::Document` when no flag is given. `read_by` in `crates/thinkthen/src/cli/asking.rs:254` builds a `Reading` from it and from the settled pointers. The pointers come from `--field`, or from a question file's `on` when no `--field` is typed. `run` then refuses the document at `asking.rs:152`:

    thinkthen: `filter` maps over a stream, so it takes --lines, --jsonl, --csv, or --tsv

It exits 2 and sends nothing. The same sentence prints when a pointer is given with no flag. `sdlc/issues/closed/2026-09-19-hands-on-test-pass-two.md` line 177 saw it for `filter 'Q' --field /b --input one.jsonl`. `crates/thinkthen/tests/backend/refused.rs:206-213` pins both sentences.

The plan's `input` object is `ReadingPlan` in `crates/thinkthen/src/core/records.rs:253`. It holds `framing` and `field`, as `{"framing":"jsonl","field":["/body"]}`. It says nothing about where the framing came from. The top-level `from` object names the source of each question setting as `"file"`, `"command line"`, or `"default"`. It appears only when a question file was read.

## Design

`read_by` takes the verb's `Keeping`. When the verb is `filter` or `rank` and no framing flag was given, it picks the framing:

- JSON Lines when any pointer is settled, from `--field` or from the question file's `on`.
- Lines otherwise.

It marks the `Reading` as defaulted. Every other verb and every explicit flag goes through as today. The refusal at `asking.rs:152` and `Failure::NoFraming` go away, because nothing else raises it.

`Reading` gains one flag and one method, `by_default()`. `ReadingPlan` gains one member, `from`, written only when the flag is set. A defaulted plan's `input` reads:

    {"framing":"lines","field":[],"from":"default"}
    {"framing":"jsonl","field":["/body"],"from":"default"}

An explicit flag prints `input` exactly as today, with no `from`. So no plan a script parses today changes.

## Decisions

Each is the agent's decision under the queue owner's ruling. Ian can overturn any of them.

1. **The default is lines when no pointer is settled.** This is the ruling. `find` already reads lines by default, so three verbs of the four that map over a stream now share it. A line is the common shape of plain text in a pipe. The explicit flags stay for scripts that want to say what they read.
2. **A settled pointer with no framing flag means JSON Lines.** The rule is: on `filter` and `rank`, a pointer from `--field` or from the question file's `on`, with no framing flag, reads JSON Lines. ADR 0007 already ties `--field` to JSON. On `decide`, `--field` with no flag reads the whole input as one JSON value. `filter` and `rank` cannot read one document, and a pointer beside `--lines` is a usage error. So JSON Lines is the only JSON framing left for them. The file's `on` counts, because the pass-two record says `on` acts as `--field` on both verbs, and one rule for both homes is what `question-file.md` promises. `decide`, `choose`, `tag`, `score`, and `annotate` keep reading one document.
3. **The plan says `"from":"default"` inside `input`, only when the default chose.** The word is the one the top-level `from` object already uses for a default. An explicit flag adds nothing, because the user is looking at the command line. So every existing plan, page, and test stays byte for byte. The default is worth naming, because a pointer silently changes the framing from lines to JSON Lines.
4. **The default is picked in `read_by`, at the command edge.** `Common::framing` serves `decide`, `annotate`, `relate`, and `recognize` too, and a clap default would give them the same framing. `read_by` already holds the settled pointers and the verb. It is one place, and `annotate.rs:184` keeps its own call.
5. **ADR 0007 is amended in place.** Its Records section says "`filter` and `rank` require one of the two." That line stays, marked as amended, and a dated "Amendment, 2026-09-26" section states the new rule and its guards. The ADR's own header says a changed line takes a new ADR, and `AGENTS.md` says to amend an accepted ADR where the history matters. The queue owner chose the amendment. Ian can overturn it for a new ADR.
6. **`find` keeps its rule.** `find --field /body` with no flag is a usage error today, because `find` defaults to lines. Bringing it in line is a separate change to a verb this ticket does not open. It is a deferred gap.

## Edge cases

`V` is `filter` or `rank`. "Pointer" means `--field`, or a question file's `on` with no `--field`.

| Command line | Today | After |
| --- | --- | --- |
| `V Q` over text lines | Exit 2, the stream sentence, no request | Each line is one text record, one request each. Changed |
| `V Q --field /body` over JSON Lines | Exit 2, the stream sentence | Each line is one JSON record, `/body` is sent. Changed |
| `V @file` whose file holds `on`, no flag | Exit 2, the stream sentence | JSON Lines, the file's pointer is sent. Changed |
| `V @file` with `on`, and `--field /x`, no flag | Exit 2 | JSON Lines, `/x` is sent, as `--field` outranks `on` today. Changed |
| `V Q` over empty input | Exit 2 | Exit 0, no output, no request, as `--lines` does. Changed |
| `V Q` over JSON Lines, no pointer | Exit 2 | Each line is a text record sent whole and printed as it arrived, as `--lines` does. Changed |
| `V Q --dry-run` | Exit 2 | The plan, with `"input":{"framing":"lines","field":[],"from":"default"}`. Changed |
| `V Q --field /body --dry-run` | Exit 2 | The plan, with `"input":{"framing":"jsonl","field":["/body"],"from":"default"}`. Changed |
| `V Q --jobs 8`, no flag | Exit 2 | Accepted, as under `--lines`. Changed |
| `V Q --lines`, `--jsonl`, `--csv`, or `--tsv` | Works | Kept, and the plan has no `from` |
| `V Q --lines --field /body` | Exit 2, `--field: a text line has no members, so --lines takes no pointer` | Kept |
| `V Q --quiet`, `--raw`, `rank --threshold`, `filter --top` | Refused by name | Kept |
| `decide`, `choose`, `tag`, `score`, `annotate` with no flag | One document | Kept |
| `decide Q --field /body` with no flag | One JSON document | Kept |
| `find --field /body` with no flag | Exit 2, the text-line sentence | Kept. Decision 6 |

## Proof

Every test runs the compiled command against a loopback listener and counts the requests it read. The three new tests live in a new file, `crates/thinkthen/tests/backend/keeping/default_framing.rs`, a child of `keeping.rs`. It reuses that file's records, listener builders, and output readers. `keeping.rs` holds 495 nonblank lines of its 500, so it gains only the `mod` line.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `filter_and_rank_read_lines_when_no_framing_is_named` | For each verb, three text lines with no framing flag at `--jobs 1`. Exit 0, empty standard error, and the exact standard output: the kept lines in order for `filter`, all three most likely yes first for `rank`. The listener read exactly 3 requests, and request N's `state` is line N | (a) Restore the `NoFraming` refusal: exit 2 with the stream sentence. (b) Default to one document and drop the refusal: 1 request, whose `state` is the whole input |
| `a_pointer_with_no_framing_reads_json_lines` | Two rows over the four JSON records `keeping.rs` already holds, with no framing flag: `filter Q --field /body`, and `filter @file` whose file holds `"on":"/body"`. Exit 0, the kept records byte for byte, 4 requests, and each request's `state` is that record's body alone, never its id | (c) Default to lines whatever the pointers: exit 2 with the text-line sentence on both rows. (d) Read the typed `--field` in place of the settled pointers: the file row reads lines and exits 2 |
| `the_plan_names_a_framing_the_default_chose` | `filter Q --dry-run` over text lines and `rank Q --field /body --dry-run` over JSON records. Each prints one exact plan line with its `input` object as in the design, exits 0, and the listener sees no connection | (e) Drop the `from` marker: both lines differ. (f) Mark every framing as defaulted: the existing `the_plan_under_dry_run_shows_the_first_record_and_opens_no_connection` row, which pins `"input":{"framing":"jsonl","field":["/body"]}` for an explicit `--jsonl`, goes red |

The two stream-sentence rows in `refused.rs:206-213` go, and the `NoFraming` row in `cli/failure/tests.rs` goes with its variant. The first test's exit 0 with no flag is the proof that the old usage error is gone. Plant (a) restores it and turns that test red.

The four questions for each new test:

- **`filter_and_rank_read_lines_when_no_framing_is_named`.** It protects the ruling: no flag frames by lines, for both verbs. A restored refusal, or a default of one document, fails it. No test runs either verb without a flag, except the refusal rows this ticket removes. It needs no hook.
- **`a_pointer_with_no_framing_reads_json_lines`.** It protects the pointer guard in both homes of the pointer. A default that ignores the pointer, or one that reads only the typed option, fails it. Both are the likeliest ways to write the rule wrong. No test runs a pointer with no flag on these verbs, except the refusal row removed here. It needs no hook.
- **`the_plan_names_a_framing_the_default_chose`.** It protects the visible default in the plan, the one place a user sees which framing a pointer chose. Dropping the marker fails it. Marking every framing fails the existing explicit-flag row. No test runs a plan without a flag. It needs no hook.

No unit test is added. `Reading::new` keeps its unit tests, and the new flag adds no branch they could reach.

## Pages and help

- `specification/filter.md` and `rank.md`: the synopsis shows the framing flags as optional. "What it reads" and the options table state the default and the pointer rule. The "missing framing is a usage error" sentences go. The `--lines` example in `filter.md` drops the flag. The `--jsonl --field` examples stay.
- `specification/records.md`: the framing table row for `filter` and `rank`, and the `--field` bullets, state the rule.
- `specification/channels.md`: the record-mode plan paragraph says `input` carries `"from":"default"` when `filter` or `rank` took the default.
- ADR 0007: the dated amendment of decision 5.
- Help: the `filter` and `rank` long help say what they read with no flag. The `--field` help says that on `filter` and `rank`, no framing flag reads JSON Lines.
- No `spec/` page runs `filter` or `rank`, and no demo runs either without a flag. The build checks both again with `grep`. Every demo keeps its explicit flag, so none changes.

## Budgets

Nonblank lines, measured with `grep -c .` on the diff.

- `crates/thinkthen/src`: at most 25 added, and at most 15 net of lines removed.
- Tests: at most 110 added in `keeping/default_framing.rs` and `keeping.rs`. The removed refusal rows come off.
- Pages and the ADR: at most 40 lines changed across the four specification pages and ADR 0007.
- `sdlc/ratchet.json` moves to the measured total in the commit that adds the code. The commit says what grew. The builder looks for duplication to delete first in `asking.rs` and `records.rs`.
- No dependency. No public library type, method, or message changes, so the `surfaces` rung is not required.

## Stop rules

1. Stop before crossing a budget or adding a dependency.
2. Stop if any plant stays green.
3. Stop if the change needs a file that tickets 0133 to 0136 own: `sdlc/scripts` rungs, the children check, the interrupt test switch, the public library entity `Debug` and API gaps, `audit` and `core/measure`, or the Polars surfaces.
4. Stop if another in-flight branch changes `cli/asking.rs`, `core/records.rs`, or `cli/failure.rs` before this one lands. The coordinator orders the two.
5. Stop if any `spec/` page or green demo turns red. None should, because every one uses an explicit flag or another verb.

## Scope and exclusions

Excluded: `site/`, which the website agent owns. The Beatles Bench repository, which its owner keeps. `find`'s pointer rule (decision 6). Guessing a framing from the input, which the issue's Option 3 names and `records.md` rules out. Any change to `decide`, `choose`, `tag`, `score`, `annotate`, `relate`, or `recognize`.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The change raises the ceiling by a few lines, so the code review names what it checked.

## Complexity

Contract 2; state and timing 0; reach 1; proof 1; cost of error 1; total 5. Final level: 1. The risk is a pointer read as a text line, which the second test and plant (c) guard.

## Deferred gaps

- `site/` carries `--lines` on `filter` and `rank` examples. They keep working. The website agent can drop the flag where the default covers it.
- The Beatles Bench scripts carry `--lines` on `filter` and `rank`. They keep working. The bench owner can drop the flag.
- `find --field` with no flag stays a usage error. Reading JSON Lines there, as `filter` and `rank` now do, would make the three stream verbs agree.
- The plan does not say where an explicit framing came from. No reader needs it, because the flag is on the command line.

## What Ian can overturn

- The queue owner's ruling: Option 1 over keeping the required flag (Option 2).
- Decision 2: a pointer means JSON Lines on these two verbs. The other choice keeps it a usage error.
- Decision 3: `"from":"default"` inside `input`, only when defaulted.
- Decision 5: an amendment to ADR 0007 in place of a new ADR.
- Decision 6: `find` keeps its rule for now.

## Closes

- `sdlc/issues/2026-09-26-filter-and-rank-could-read-lines-by-default.md`. The lander moves it to `closed/` in the landing commit.

## Evidence

- Starts from: The issue above, filed 2026-09-26 by the marketing session after Ian asked why `--lines` is in almost every example. `sdlc/issues/closed/2026-09-19-hands-on-test-pass-two.md`, which saw the stream sentence for `filter --field` with no flag and recorded that a file's `on` acts as `--field` on both verbs. ADR 0007's Records section. The code paths named in "What happens today" at `origin/main` `be549e05`. `find`'s lines default in `cli/find.rs`.
- Keeps: Every explicit framing flag and its output, its plan, and its refusals. `--lines` beside a pointer stays a usage error. `decide`, `choose`, `tag`, `score`, and `annotate` still read one document by default. The refusals of `--quiet`, `--raw`, `--threshold`, and `--top` by name. Every existing plan byte for byte. Every demo and `spec/` page.
- Changes: `filter` and `rank` with no framing flag read lines, or JSON Lines when a pointer is settled. A defaulted plan's `input` carries `"from":"default"`. The stream sentence and `Failure::NoFraming` go. `filter.md`, `rank.md`, `records.md`, `channels.md`, the help, and ADR 0007 say so.
- Proof: The three outside-in tests under "Proof": lines by default counted at one request per line, a pointer from either home read as JSON Lines, and the plan naming the default. Plants (a) to (f) each turn a test red, and (a) restores the old usage error.
- Defers: The `site/` examples and the Beatles Bench scripts, which keep working with their flag. `find`'s pointer rule. A plan source for an explicit framing.
