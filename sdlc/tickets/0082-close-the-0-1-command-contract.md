---
flow: build
priority: 82
opens: README.md crates/thinkthen/Cargo.toml crates/thinkthen/src/cli crates/thinkthen/tests specification spec demos sdlc/scripts sdlc/ratchet.json sdlc/records
---

# 0082: Close the 0.1 command contract

Status: landed 2026-09-24. `sdlc/records/0082-build-close-command-contract.md` records the build. Owner: Claude.

## Design and decisions

The command's help and prose close 0.1's wording items. This ticket changes words and pins them with exact tests. It changes no result key, no behavior, and no exit code.

Decisions, each of which Ian can overturn:

- Item 44 (`calibrated` becomes `tuned_for`, Ian's 2026-09-23 ruling) moves to ticket 0090. It changes a result key and has its own public-surface review. This ticket stays pure wording, and its stop rule holds without an exception.
- Item 3 keeps the `recognize` and `relate` introductions that landed with 0080 and 0088. Both passed the 0088 reviews. This ticket pins them and writes no new sentences.
- `relate` gets this whole-set exit sentence in long help, drawn from `specification/relate.md`: `A run that answers some relation questions and fails others prints what it has and exits 6. A run whose relation questions all fail prints nothing and exits 4.` An empty line or JSONL stream still succeeds with no output, as `specification/relate.md` says.
- Public teaching copy says **not sure** and **broken**. `decide` long help replaces `the exit code is 0 for yes, 1 for no, and 3 for unresolved` with this teaching sentence: `The exit code is 0 for yes, 1 for no, 3 for not sure, and any other code when the run is broken or interrupted.` The README carries no four-outcome sentence on main, and this ticket adds none.
- Exactly one sentence, in `specification/decide.md` beside the `null` answer, defines the formal term: `` `unresolved` is the formal name for a not sure answer. `` Other specification pages keep `unresolved` as exact contract language and need no edit. `failed` stays valid for one question a backend could not answer inside an otherwise usable result. It never names the whole broken outcome. `sdlc/issues/2026-09-21-the-help-first-lines-and-the-public-words.md` records the approved vocabulary.
- Row 43 also fixes the crate description in `crates/thinkthen/Cargo.toml`, which says `Put a decider model in the shell`.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex (Astra) reviews as the other vendor under `sdlc/planning/one-line-plan-2026-09-24.md`.

## Outcome and authority

Re-audit the 45 items in `sdlc/issues/2026-09-22-command-wording-and-help-fixes-for-0-1.md` against main. Exercise root help and short and long help for all ten functions: `decide`, `filter`, `rank`, `choose`, `find`, `score`, `tag`, `annotate`, `recognize`, and `relate`. Change only mismatches that survive in command wording, help, specification, diagnostics, vocabulary, or result documentation. Do not repeat a fix main already carries.

## Required audit and disposition

The baseline is main containing the landing records for 0088 and 0089. 0089 rewrites `--max-retries` long help in `cli/args.rs` and `cli/args/find.rs`, so an earlier baseline would fail the byte diff below. Before editing product text or code, capture root help plus `-h` and `--help` for every function from that baseline. Inspect every source location the merged issue names, and write the observed disposition into the implementation record. A prior ticket or an accepted design is not proof. The built binary or owning file is proof.

The table below is the bounded design. If the audit contradicts one row, correct that row in the record and fix only a surviving in-scope mismatch. Stop for design review if a contradiction needs new behavior, a new surface, or another outward decision.

| Item | Disposition | Required evidence or work |
| --- | --- | --- |
| 1 | fixed here | Reconcile public **not sure** and **broken** with the formal `unresolved` term under the rule above. Pin the four-outcome teaching sentence. |
| 2 | already fixed | Ticket 0066 root introduction remains exact. |
| 3 | already fixed | Tickets 0080 and 0088 landed the introductions. Preserve the eight accepted introductions. Pin `recognize` as `Find every name in a text and assign one of the given kinds.` and `relate` as `Find named relations across one complete entity set.` Both passed the 0088 reviews. |
| 4 | fixed here (pin only) | Main already lists `status`; `decide`, `filter`, `rank`, `choose`, `find`, `score`, `tag`, `annotate`, `recognize`, `relate`; `cache`; generated `help`. Add the exact inventory assertion. |
| 5 | already fixed | Ticket 0066's description, usage, options, examples layout remains intact. |
| 6 | fixed here | Complete the approved fixed-word sweep in built help, README, and green how-tos, with contextual allowances stated below. |
| 7 | already fixed | Ticket 0057 names the `0.5` default wherever the option exists. |
| 8 | already fixed | Ticket 0057 names the default width of 4 in long help. |
| 9 | already fixed | Ticket 0057 and ADR 0031 put `--url` in short help and keep other advanced options in long help. |
| 10 | already fixed | Ticket 0057 gives `find --none` empty output and exit 3. |
| 11 | already fixed | Ticket 0057 refuses zero timeout locally. |
| 12 | fixed here | Add the exact record-run exit sentence to `recognize`. `relate` reads one whole entity set and carries no record-run sentence. Add the whole-set exit sentence below to its long help. |
| 13 | already fixed | Ticket 0058 names both no and not sure in the `set -e` warning. |
| 14 | already fixed | `rank` help describes its printed order rather than input order. |
| 15 | already fixed | `filter` and `rank` no longer describe a nonexistent unframed field mode. |
| 16 | already fixed | `--input` directory refusal names the option and emits no framed-record stop line. |
| 17 | already fixed | Invalid framed bytes say that the record is not valid UTF-8. |
| 18 | already fixed | Ticket 0058 omits recording counts when no recording feature was named. |
| 19 | already fixed | System One and recording JSON refusals name the object that is not valid. |
| 20 | already fixed | Ticket 0058 uses singular `field` for one CSV field. |
| 21 | already fixed | Ticket 0058 gives a fixed action for a recording path that is a regular file. |
| 22 | already fixed | Ticket 0058 documents the question-name rule and points the refusal at it. |
| 23 | already fixed | Ticket 0058 names empty or blank evidence directly. |
| 24 | already fixed | Ticket 0058 names the unknown single-question-file key. |
| 25 | already fixed | Ticket 0058 names the missing `questions` wrapper first. |
| 26 | already fixed | Ticket 0058 chose a fixed tool-owned status-400 phrase and suppresses hostile bodies. |
| 27 | stale | Ticket 0058's secrecy decision forbids exposing backend refusal bodies. Recording one would retain untrusted evidence-bearing text without a user-visible contract. |
| 28 | already fixed | Ticket 0058 maps structured transport failures to fixed tool-owned guidance. |
| 29 | already fixed | Ticket 0058 prints tolerance `0.01`. |
| 30 | already fixed | Ticket 0058 applies one-document width validation to dry-run before a plan or send. |
| 31 | already fixed | Ticket 0057 aligns exit 6 and partial question failure. |
| 32 | already fixed | Ticket 0057 distinguishes an empty document from an empty framed stream. |
| 33 | already fixed | Ticket 0057 distinguishes empty line/JSONL streams from headerless CSV/TSV. |
| 34 | already fixed | Ticket 0057 makes an exact `choose` tie unresolved with or without a cut. |
| 35 | already fixed | Ticket 0057 repairs the detailed `rank` and `tag` examples. |
| 36 | already fixed | `result.md` says `filter`'s `value` is the cut's boolean. |
| 37 | already fixed | A `rank` plan omits an inapplicable threshold source and `from` names only settings the function takes. |
| 38 | already fixed | `result.md` says `question.verb` names the question kind; `filter` and `rank` therefore report `decide`. |
| 39 | already fixed | The how-to index no longer carries the stale built-command list. |
| 40 | already fixed | The root README no longer claims that its examples use an unlanded grammar. |
| 41 | already fixed | The landed 0080 and 0088 work left the specification index and result-kind count at ten functions and the current result kinds. |
| 42 | already fixed | Live-call text promises robustness or reports measured variation; only replay promises the same recorded answer. |
| 43 | fixed here | Remove `decider model` and `decision model` from current public command help, the crate description in `Cargo.toml`, README, specification, and how-to prose. Use `System One model` when naming the product type and `the model` otherwise. |
| 44 | moved | Ticket 0090 renames `calibrated` to `tuned_for` in the tool and specification. Surfaces and site copy close the rest of the item later. |
| 45 | fixed here | Make `annotate` begin exactly `Answer a saved set of questions about every record.` everywhere its command introduction is owned in this repository. |

## Scope

Allowed work is limited to surviving rows 1, 4, 6, 12, 43, and 45, the pins for row 3, their exact tests, and the mechanical vocabulary check. Edit command help, root README text, the crate description, specification text, green how-tos, and result wording only where one of those rows requires it. A diagnostic may change only if the audit proves that a row marked already fixed still reproduces and the repair changes wording without changing behavior. Any contradiction outside these rows stops work for a reviewed ticket amendment or a separate bounded ticket. Update the source ratchet only for necessary Rust tests or help text. The implementation record carries the final 45-row disposition and cites the test or file that proves every `already fixed`, `stale`, and `moved` row.

The vocabulary check runs from the existing ladder. It reads the built root help and short and long help for all ten functions, plus README and green how-to prose. It skips fenced code and paths. It holds two lists, because a script can judge a fixed phrase but not the meaning of a context word.

- Fixed banned phrases apply everywhere it scans: `decider model`, `decision model`, `rating`, `the mark`, and `judgment` or `judgments`. Sanctioned uses stay listed in the check with their reason.
- Context words apply only to built help: `row`, `document`, `unit`, and `label` in `choose` help. The check sanctions `rows` for saved detailed result rows, CSV `header row`, `label` in `tag` help, and the one-document framing option names. Each sanction is a listed exact phrase.

It also rejects `unresolved` in built help and permits the one formal definition in `specification/decide.md`. It permits `failed` for an individual failed question and `probability` for a backend-reported probability.

Demo 02 keeps its title, `Branch on a label with choose and case`. Context words apply only to built help, so a how-to title does not break the rule. A rename would also move the title in `demos/README.md`, `sdlc/planning/documentation-plan.md`, and ADR 0018's list, which `sdlc/scripts/pages` cross-checks, for no gain in the command's own words. It carries no rule for `calibrated`; ticket 0090 owns that check. The check prints the file, line, word, and violated rule for every hit. Planted self-tests cover one banned and one sanctioned use.

Excluded: item 44, new commands, options, functions, dynamic option sources, threshold comparison, recognition or relation policy, planner changes, APIs, language or database surfaces, the `surfaces` branch, packages, installers, workflows, publication, site or deck edits, dependencies, request bytes, cache or recording formats, result keys or shapes, exit-code changes, live calls, and paid calls. Do not edit the merged issue to declare victory. The landing record supplies the closure evidence.

## Deterministic acceptance

- The baseline artifact contains 21 help captures from main after 0089 lands: root help and both help modes for each of the ten functions. A table names the command, capture, exit code, and exact first sentence. The post-change artifact has the same 21 members. A byte diff is empty except for lines authorized by rows 1, 6, 12, 43, and 45.
- One exact inventory assertion pins root order as `status`; the ten functions in the order above; `cache`; generated `help`. It finds each function once and no eleventh judgment function. Ticket 0083 later edits this assertion to insert `transform`. Exact short and long help assertions pin `recognize` and `relate` to the row 3 introductions and keep their beta warning, cut description, and cost disclosure.
- Every record-capable command carries the exact sentence `A record run exits 0 when it completes without a partial or whole-run failure. The printed values carry the individual answers.` exactly once in long help, `recognize` included. `relate` carries no record-run sentence. An exact assertion pins its long help to the whole-set exit sentence in the decisions above, exactly once.
- Exact assertions pin `annotate -h` and `annotate --help` to `Answer a saved set of questions about every record.` as their first sentence. The root row uses the same sentence. Its usage, options, result-shape paragraph, examples, record-exit sentence, and exit-6 sentence each remain exactly once. `tests/tag_edge.rs` and `tests/version.rs` change their pinned old sentence. `tests/find_edge.rs` line 89 pins `Every unit leaves together and sees every other unit` and changes with row 6.
- An exact assertion pins the `decide` long help teaching sentence given in the decisions above, exactly once. The `decide` long help no longer says `3 for unresolved`. A fixed-string test finds the sentence `` `unresolved` is the formal name for a not sure answer. `` exactly once across `specification/`, in `decide.md`. No public teaching sentence uses `error` as the fourth outcome or `failed` for the whole run.
- The vocabulary checker scans built help for all ten functions and README and every green how-to under the two lists above. Its unsanctioned-hit count is exactly zero. Its self-test exits nonzero for planted `decider model` in a how-to and planted `document` in built help. It accepts planted `tag label`, CSV `header row`, saved detailed `rows` in a how-to, and individual `failed question`.
- Fixed-string checks find zero uses of `decider model` or `decision model` in README, `crates/thinkthen/Cargo.toml`, specification, green how-tos, and built help. Historical tickets, records, issues, probes, recordings, fixtures, and measured quotations stay unchanged.
- A machine-readable 45-row audit in the implementation record contains every integer 1 through 45 exactly once, each marked one of `already fixed`, `stale`, `fixed here`, or `moved`. Row 44 is `moved` and names ticket 0090. Every row names its proving command, test, or file.
- Every changed local refusal is tested at its exact exit code and complete standard-error sentence. A counted loopback listener observes zero requests for each such case. A dry run alone is not proof of no send. If the audit finds no surviving diagnostic mismatch, no diagnostic source changes.
- Focused help, vocabulary, specification, and changed diagnostic tests fail for the stated old text before the fix and pass afterward. Run `sdlc/scripts/install`, `sdlc/scripts/lint`, `sdlc/scripts/test`, and `sdlc/scripts/spec` in sequence with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, then `git diff --check`. No live or paid command runs.

## Budgets

Production Rust changes may touch at most four files and add at most 60 nonblank lines. The likely set is three files. `cli/args/command.rs` holds rows 1, 12, 43, and 45. `cli/args.rs` holds row 6 text: `choose` `label` at lines 315 to 341, `document` at 79, 154, and 341, `row` at 38 to 40, and `judgments` at 446. `cli/args/find.rs` holds `unit` on six lines. Rust tests and helpers may touch at most five files, likely `decide_edge.rs`, `tag_edge.rs`, `version.rs`, `find_edge.rs`, and one help test, and add at most 220 nonblank lines. The vocabulary enforcement may add at most 140 nonblank lines across existing scripts and fixtures. Public prose, `Cargo.toml` included, may touch at most sixteen files and add no more than 120 net nonblank lines; prefer replacement and deletion. Add no dependency and keep every Rust file below the 500-line ceiling. The ratchet increase equals the measured Rust increase, and the record names why each added block earns its lines and where duplication was removed first.

Stop and re-score if the audit needs more than these budgets, changes a result key or shape, alters request or runtime behavior, reaches outside this repository, or finds another outward decision.

## Dependencies and order

Work starts from main containing the landing records for 0088 (public `relate`) and 0089 (no resend after a transport failure). 0089 shares `cli/args.rs`, `cli/args/find.rs`, `specification/result.md`, and `sdlc/ratchet.json` with this ticket. Tickets 0057, 0058, 0066, 0067, 0080, 0087, and 0088 supply the accepted prior fixes.

Queue order: 0089, 0082, 0083, 0090. None of them builds in parallel with another. 0083 shares `cli/args/command.rs`, the root inventory test, `sdlc/ratchet.json`, and `sdlc/scripts/lint` with this ticket.

## Complexity

Contract 2; state and timing 0; reach 1; proof 2; cost of error 1; total 6. Final level: 2. Claude owns the queue and builds this ticket. Re-score and stop if work crosses an exclusion or changes runtime behavior.
