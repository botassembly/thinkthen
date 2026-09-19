# 0018: The how-to standard and the smaller list

Branch `ticket/0018-how-to-standard`. Built 2026-09-19. No behavior of the binary changed, and nothing under `crates/` was touched.

## What landed

`sdlc/scripts/demos` measures rules 1, 2, 3, 4, 5, 7, and 8 of ADR 0016 on every green page, after the form checks of ADR 0011 and before it runs a block. One `awk` program reads the page and names the page, the rule, and the number it measured:

```
demos: 04-review-queue/README.md breaks rule 1 of ADR 0016: 177 lines, and the limit is 120
```

`sdlc/scripts/demos-self-test` is new and the `spec` rung runs it before `demos`. It builds sixteen fixture pages under `target/demos-standard/`, fifteen that each break one rule and one that breaks none, runs the real runner over each, and pins the exit code and the whole sentence. A check that runs from no rung rots, and a fixture that pins "it failed somehow" rots the same way.

Fifteen green pages became thirteen. Every one is inside the limits.

## Each page, before and after

| Page | Lines before | Lines after | Words before | Words after | What changed |
| --- | --- | --- | --- | --- | --- |
| 01 refund gate | 109 | 65 | 756 | 521 | Shed the band and the audit block to 19 |
| 02 route a ticket | 149 | 111 | 1146 | 887 | Shed two blocks that tested the fixture and the margin block that 20 owns. Absorbed 05 as its closing section |
| 04 review queue | 177 | 97 | 1328 | 825 | Shed the counting to 25, the re-cut to 13, and the refused-record block to 12 |
| 05 sort a folder | 162 | gone | 1192 | gone | Merged into 02 |
| 12 keep going | 110 | 111 | 764 | 778 | One link |
| 13 pick a threshold | 127 | 98 | 1094 | 897 | The split moved into the first block through process substitution, so nothing is set up before the first result. Five steps became four and five links became four |
| 17 rate and sort | 149 | 87 | 1183 | 899 | New scenario: routing a request by how hard it is. The sorting section became a sentence, because 06 owns ordering |
| 19 no or could not ask | 145 | 108 | 1053 | 898 | New scenario: gating a proposed shell command and failing closed. Took 01's band and audit row. Seven blocks became four |
| 20 not stated or false | 174 | 107 | 1253 | 893 | Title names both verbs. The branching block went to 02 and the exit table to 02 and 19. It took 05's margin rule |
| 21 options from the record | 95 | 97 | 801 | 899 | New scenario: picking the next action from a list that changes at every step |
| 24 compare two runs | 93 | 89 | 748 | 727 | The first block lost its `mktemp` |
| 25 check the judge | 103 | 104 | 841 | 899 | Absorbed 38's calibration table |
| 27 test with no network | 118 | 108 | 867 | 787 | Shed the cache bullet to 12. The listing of the script under test moved below the first asserting block |
| 28 what a run cost | 110 | 110 | 817 | 817 | One link |
| 38 what a probability means | 96 | gone | 785 | gone | Merged into 25 |

## The recordings deleted, and the scenarios changed

Three pages changed scenario, and each was recorded again through `sdlc/scripts/live` by a `record.sh` beside it.

| Page | Old scenario | New scenario | Recordings deleted | Recordings written |
| --- | --- | --- | --- | --- |
| 19 | A change note tested in staging | Three proposed shell commands, judged safe, unclear, and unsafe | 2 | 3 |
| 17 | Four bug reports rated for disruption | Four customer requests routed by how hard they are | 4 | 4 |
| 21 | A returns desk filing notes under its own codes | Three steps of a support job, each with its own actions | 3 | 3 |

05's five recorded exchanges moved into `02-route-a-ticket/recording/` with its `inbox/`, because a recording folder is content-addressed and two questions share one folder without collision. `record.sh` in 02 now records six exchanges. 38 held no recording of its own, because it reads the committed rows under `transforms/`.

## The choices made where ADR 0016 was silent

Each is overturnable.

1. **A "runnable block" is a fenced block that holds `mustmatch`, and a `bash` block that holds none fails rule 3.** A block in another language, such as the `sh` blocks on 27 that show a command a reader types once by hand, asserts nothing and is not counted. ADR 0016 says "every block asserts something" without saying which blocks count, and a page has to be able to show a command nobody runs in a gate.
2. **A "step" is a `##` heading that is not `Input`, `What can go wrong`, or `Related how-tos`.** ADR 0016 caps steps at four and never says how to count one. This is the only mechanical reading, and it is why 13 and 25 lost a section each.
3. **Rule 2's "nothing is set up before the first result" is measured on the first block alone.** The first block may hold no `mktemp`, no `trap`, and no heredoc. Later blocks may, because the reader has already seen a result by then.
4. **Rule 2 is measured on the first block that asserts, not on the first fence, and it has two clauses.** The first `thinkthen` line has to be within five lines of the block's start, and so does the first working line, counted as the first line that is neither blank, nor a `set` builtin, nor a bare assignment. ADR 0016 names only the `thinkthen` line, and four eval pages run no `thinkthen` at all, so the second clause is what holds a `jq` page to the same promise. Measuring from the first asserting block is what lets 27 show the script under test as a listing nobody runs, while the reader still meets a result first.
5. **Rule 4 counts `thinkthen VERB` over the whole page, and the title must name every verb used.** 20 uses both `decide` and `choose` and its title now reads "with `decide` and `choose`". The contrast is the whole page, so naming it costs four words.
6. **Rule 8 also refuses two closing links to one target.** ADR 0016 says each link goes to a different idea, and one target twice is the only form of that a script can see.
7. **The folder slugs of 19, 17, and 21 stay as they are.** `crates/thinkthen/tests/demo_runner.rs` names `19-no-or-could-not-ask` and `21-options-from-the-record`, and the ticket forbids touching `crates/`. A later ticket can rename all three together.
8. **How-to 42 is the serve-a-loop page.** ADR 0016 names one page with no number, "serve a loop from one long-lived process". Ticket 0017 takes 40 and 41, so 42 is the next free number. Every other page in ADR 0016's list of 27 already had one.
9. **04 lost its refused-record block to 12 rather than trimming it.** 12 already asserts the same two sentences on standard error. Two pages teaching one thing is what the standard exists to stop.
10. **02 keeps two scenarios.** The ticket orders 05 into 02 as its closing section, and 05's scenario is filing meeting notes while 02's is routing a ticket. Re-recording the notes as tickets would have spent live tokens to teach nothing new, so the closing section says plainly that it is the same shape over a folder.
11. **17 holds its three levels in a `levels` array.** The bytes sent are the same, so the digest is the same, and the page fits inside 900 words with the levels written out once where a reader meets them.

## The live calls

Three, all through `sdlc/scripts/live` by a `record.sh` beside each page. How-to 19 spent 990 input tokens over three exchanges, how-to 17 spent 1,485 over four, and how-to 21 spent 1,090 over three. The ledger went from 251,466 to 255,031 of 476,000,000. Every committed recording was searched for `apikey_`, `authorization`, and `bearer` in any case, and all three counts are zero.

The judge answered 0.89 for a `grep`, 0.22 for a `git fetch`, and 0.01 for an `rm -rf`, which is why how-to 19 uses the band `0.1:0.8` and gets one of each answer from three commands. `git fetch` writes inside `.git` and touches no working file, and 0.22 is the model saying so.

## The review

A second agent with fresh context read three pages as a newcomer, checked the diff against the ticket and against ADR 0016, and read "What reviewers keep finding" line by line.

The verdict on the first pass was **not ready**, with six must-fix findings, three adjacent notes, and five opinions. Every one is answered below.

| Finding | What happened |
| --- | --- |
| The record and `demos/README.md` were uncommitted and the record still held a placeholder | Both committed with this record |
| Choice 4 described a check the script did not have: the `thinkthen` clause was silently unenforced on a `jq`-only page | The check was rewritten. Rule 2 now measures the first asserting block and applies a first-working-line clause that a `jq`-only page cannot dodge. Choice 4 above says what the code does |
| 05's margin rule and its parallel `xargs` loop were dropped rather than moved, and the record wrongly gave the margin to 20 | The margin rule is now Step 3 of 20, written in `jq` and asserted at 0.81. The parallel loop stands dropped: `--jobs` on 12 is the lever for concurrency, and `demos/FINDINGS.md` already records that a per-file loop sits outside every budget |
| `demos/FINDINGS.md` still cited the deleted page 05 on three rows | Repointed to 02 on two rows and to 06 alone on the budget row |
| `documentation-plan.md` left the State cell blank for 18, 30, and 23, and said only "The flagship" for 16 | Every State cell now starts with Green or Red, and 16 reads "Red. The flagship" |
| The rule-3 carve-out for blocks that are not `bash` was untested | A `rule3-other-language` case was added: an `sh` block that asserts nothing passes |
| Rule 7 is a heading grep and catches only the banned heading | Stands. A design argument under another heading is a thing a reader catches and a script cannot. Recorded as a standing limit |
| Rule 2 measured the first fence, and 27's first fence was a listing nobody runs | Fixed, and 27 restructured so its first block asserts |
| Rule 4 counts a verb anywhere on the page, including prose | Stands. A verb named in prose is a verb the page teaches, and the title should say so |
| 02 sat at exactly 900 words | Trimmed to 887 |
| The README front window gives no link for the red pages 03, 06, 15, and 16 | Stands. A red page is not a how-to yet, and the row says "coming" with its number. The link goes in when the page turns green |
| `sdlc/scripts/README.md` said "thirteen fixture pages, one per rule" | Corrected to sixteen |
| "every block asserts" should read "every `bash` block" | Corrected in `demos/README.md` and in `documentation-plan.md` |
| The empty `demos/05-sort-a-folder/` directory survived on disk | Removed |

Three self-test cases were added for the rewritten rule 2 and the rule 3 carve-out, taking the suite from thirteen cases to sixteen. Three rules carry a standing limit a script cannot close: rule 6's "ordinary office scenario" is not measured at all, rule 7 sees only the banned heading, and rule 4 cannot tell a verb in a command from a verb in a sentence.

## The gates

`install`, `lint`, `test`, and `spec` all exit 0. `demos: 13 green, 7 red`. The ratchet is unchanged, because nothing under `crates/` moved.

The ticket stays at `in progress`.
