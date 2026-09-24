REJECT

# 0082 code review: close the 0.1 command contract

Reviewed `ticket/0082-close-command-contract` at 2dd4d6d1 against merge base 915119b9. This was a fresh, read-only session. Authority: the ticket, `0082-design-review.md`, and `0082-design-rereview.md`. Newer main (fdb7c535) touches only two planning files and merges cleanly.

## Blocking

B1. README.md:21 still teaches the old four outcomes: `- A yes, a no, an unresolved answer, and an error stay four different outcomes in the output and in the exit code.` This line is on main (it dates from 9b6a8295). The ticket's decision premise (ticket line 20: "The README carries no four-outcome sentence on main") is false. The row 1 audit missed the line, yet the record (build record line 45) marks row 1 `fixed here`. The acceptance bullet "No public teaching sentence uses `error` as the fourth outcome" fails. The vocabulary check cannot catch this: `unresolved` is a help-only rule and `error` has no rule. Smallest fix: make README.md:21 read `- A yes, a no, a not sure answer, and a broken run stay four different outcomes in the output and in the exit code.` This stays within row 1. README is already one of the 16 prose files, and the net line count stays 0. Then correct the premise in the record's deviations list and in the row 1 proof. Pin the new sentence by adding one fixed-string assertion on README to `version::the_specification_defines_unresolved_once_and_keeps_the_closed_wording`.

B2. The vocabulary check passes silently when help fails to print. At sdlc/scripts/demos:230 and :233 the script runs `thinkthen ... | vocabulary ...` under `sh -e` without pipefail, so the pipeline's status is awk's. I observed this: a stand-in binary made `relate -h` and `relate --help` exit 2 with no output, and `demos` still printed `demos: 21 green, 0 red`, rc=0. If a verb is renamed or dropped, its help check is skipped without notice. Smallest fix: capture first, then scan. For example `help=$(thinkthen "$verb" "$flag")` followed by `printf '%s\n' "$help" | vocabulary ...`, since `set -e` aborts when a command substitution in an assignment fails. Do the same for root help. Add one self-test case where the stand-in exits nonzero.

## Verified (by command, in a scratch copy with its own CARGO_TARGET_DIR, and THINKTHEN_API_KEY and THINKTHEN_BASE_URL unset)

- At HEAD, the five focused test files pass (version 6, decide_edge 20, tag_edge 2, find_edge 14, choose_and_score_edge 9). `demos-self-test: 19 cases pass`. `demos: 21 green, 0 red`.
- With the three help files reverted to 915119b9, seven tests go red across all five files. Against that binary the vocabulary check prints 77 hits and exits 1.
- Pins that fail on their own: changing only the decide teaching sentence fails `decide_edge.rs:474`, and changing only the relate "exits 4" fails `decide_edge.rs:472`.
- Awk probes. Red: case-folded `Decider Model`, `Ratings`, `the mark` (including inside inline code), `row-major units`, `unresolved` in help, `document`, and `label` in choose help. Green: `curating`, `arrow`, `community`, `unity`, `header rows`, `LABEL=DESCRIPTION`, `label` in tag help, `rows` in prose, and `/`-paths.
- No diagnostic changed. The source diff covers only doc comments in `cli/args.rs`, `cli/args/command.rs`, and `cli/args/find.rs`. Clap value names are unchanged.
- The decide exit sentence is true. Only `Outcome::No` maps to 1 and only `Outcome::Unresolved` maps to 3 (`cli/asking.rs:511-516`). No failure path maps to 1 or 3. Failures exit 2, 4, 5, 70, or 130.
- The relate sentence is true. `cli/relate.rs:83-85` returns `Relate(Logical)` when no answer succeeded and at least one failed, which exits 4 (`cli/failure/relate.rs:29`) with nothing printed. A partial run exits 6 (`relate.rs:98-99`). This matches `specification/relate.md:62`.
- The HEAD binary's 21 captures match `0082-help-after.txt` in every non-blank line. Every before-to-after line change belongs to row 1, 6, 12, 43, or 45.
- Budgets. Production: 3 files, +58 nonblank lines added, +8 net. Tests: 5 files, +102 net. Scripts: +84. Prose: 16 files, net 0. The nonblank `.rs` total under `crates` measures 43782 at the merge base and 43892 at HEAD. That is +110, which equals the ratchet raise.
- Prose accuracy. `result.md` has five answer kinds, and all ten functions are built. The "System One model" wording matches README:22. The swaps of threshold for mark, and questions or answers for judgments, are accurate.
- Departures. The seven listed departures are all acceptable: row 4 already fixed (ORDER pin at version.rs:107), rows 39 and 41 fixed and in scope, the one-document sanction unneeded, the LABEL=DESCRIPTION sanction (it keeps a diagnostic unchanged), the scope-based sanctions, and no cost sentence added. The departure list omits B1.

## Follow-ups (not blocking; file as issues)

- F1. `cli/failure.rs:432` still says "one document sends one request", while help now says "a single text". A diagnostic change needs its own ticket.
- F2. `relate --help` lists `--jobs`, and `relate` refuses it at exit 2 (observed). This predates 0082.
- F3. The check misses the British spelling `judgement`, and it misses a phrase split across two lines.
- F4. `specification/score.md:57` still says "rubric judgments". The specification sits outside the check's scope, but the wording now differs from help and demo 17.
- F5. `recognize` and `relate` help carry no cost disclosure. The ticket's acceptance line assumed they did.
