# The quality plan

Status: **Proposal** from experiment 218, wave 1 (2026-09-21). Waves 2 through 4 extend the checklist and the tester brief as the libraries, databases, and the real engine land. Ian can overturn the form and any rung placement.

One rule carries the whole plan: a finding that can be a test becomes one, and a test runs from a rung or it rots. What cannot be a test becomes a line on the release checklist, and what cannot be checked becomes the exploring tester's rotation.

## Part 1: automated tests, by rung

The ladder stays `install`, `lint`, `test`, `spec`. Wave 1 found nineteen issues; thirteen become the tests below. Each names the issue it guards and what it costs to run. The loopback-listener technique is already the repository's own ("a test that claims 'sends nothing' counts the requests on the loopback listener").

### `test` (Rust tests; today ~4 minutes)

| Test | Guards | Cost |
| --- | --- | --- |
| Send-count visibility: a 500-then-ok listener; `--details` must carry the number of sends and the summed usage, and the stand-in count must match | issue 3 (retried send invisible) | seconds |
| Address-change warning: cache three records under one loopback URL, resume under another; the run must warn and the second listener must see exactly the re-sent requests | issue 2 (silent re-bill) | seconds |
| Lock cleanup: after a completed `--cache` run, `.locks` holds nothing (or shards with entries); file count equals entry count | issue 10 (locks stay) | seconds |
| Write-before-request: a read-only record folder must refuse before the first send (listener count zero), or the help must say the request happens first | issue 12 (paid answer discarded) | seconds |
| Early close: a listener that closes without a response fails in milliseconds, not at `--timeout` | issue 8 (closed connection costs the deadline) | seconds |
| Sentence pins, one assertion per message: empty evidence, question-file refusals (single file names the offending key), tolerance printed as `0.01`, transport failures name the event and the `--timeout`/`--max-retries` levers, no `timeout: global` string reaches a user | issues 7, 9, 14, 16 | seconds |
| Result-object parity: every function's `--details` carries the same keys; a key one function has and another lacks fails the test (wave 1's caller found `annotate`'s request digest) | the caller-review area, README addition 2026-09-21 | seconds |
| Dry-run parity: `--dry-run` accepts and refuses exactly what the live run does, and prints the request count | same | seconds |
| Sizes battery: 40 bytes, 3 KB, 16 KB, 100 KB, 1.4 MB through each function against a loopback stand-in; nothing is sent that could never be answered | same | under a minute |
| SIGINT: a spawned record run stopped by SIGINT exits 130, prints the stopped-at line, and leaves every entry complete (needs a process spawn; skip on hosts that cannot signal) | issue 11 (silent stop) | ~2 s |

### `lint` (policy, pages, size, format; today ~1 minute)

| Check | Guards | Cost |
| --- | --- | --- |
| Vocabulary lint over the built help: run `thinkthen --help` and all eight function helps, grep for the banned words (`document`, `row` in the record sense, `label` for `choose`'s options, `judgment`, `failed` alone, `rating`, `unresolved` once the outcome word is ruled); zero hits or the check names each hit | issue 17, issue 4 (outcome names) | seconds, needs a built binary |
| README and demo pages join the same grep, source-side | issue 17 | seconds |

The marketing repository's pages get the same script on its own side, run before a page or deck ships; thinkthen's gate never reads another repository.

### `spec` (executable pages, transforms, green how-tos; today ~3 minutes)

| Test | Guards | Cost |
| --- | --- | --- |
| A how-to that joins two functions with a pipe and drives the empty-pipe path (`find` prints nothing, the next stage must not go red) | the two-function-flows issue, issue 18's family | ~10 s |
| A demo page for resume: interrupt a `--cache` run, resume it, assert the request count for the unfinished records only (the resume arithmetic wave 1 verified by hand) | issues 2 and 11 | ~5 s |
| A refused question inside a good request: one question of a set refused; the good answers survive or the failure names the one that did not | the caller-review area | ~5 s |

### Pre-release only (too slow for every commit)

| Script | What | Cost |
| --- | --- | --- |
| `sdlc/scripts/load` (proposed) | 100k records through `filter --jsonl`, `decide --lines`, `rank`, `tag`, `annotate` against a bundled stand-in; asserts peak RSS of the streaming functions stays inside a fixed bound (wave 1's numbers become the first bounds) and prints the table | ~10 minutes |
| Speed-claim rerun | the command-local numbers from the public pages (process start, cache hit versus miss, records per second against the stand-in) | ~2 minutes |

## Part 2: the release checklist

The full pass runs at the exact release commit, on Linux and macOS, from the installed package, not the checkout. It blocks on any Open blocker or major in `sdlc/issues/` that is not consciously waived in the release record.

1. The gate ladder is green on both machines.
2. Every printed example runs: README, all help pages, all green how-tos, the marketing pages and the deck, from the installed binary. The marketing repository's example check runs on its side in the same day.
3. Install on a clean Linux account and a clean macOS account with the one line each slide shows; the first example on each slide runs (area 11).
4. The conformance cases pass on all nine library and database surfaces; the same input gives the same answer on every surface (area 10, wave 2).
5. Load: 100k and 1M records against the stand-in; streaming memory holds; the table is filed in the release record.
6. Every speed and cost number on a public page is rerun or re-confirmed against its recorded probe; the page and the number move together.
7. Secrecy sweep: canary key through every command and failure path, every output, recording, cache entry, and error; zero hits.
8. The misbehaving-backend battery (area 7's 24 cases) passes against the stand-in.
9. The cache rows that waves 1 could not run (default XDG folder, 100 MB cap, prune, `status`, one-command clear, full disk) pass, or the release record says why not.
10. The vocabulary lint is zero-hit on help, README, demos, and the marketing pages.
11. Every wave-1 issue file is closed or waived with a name beside the waiver.

## Part 3: the exploring tester

A standing brief for one agent that pokes around after each release and on a schedule Ian sets. Budget: one working session per rung of the schedule, no paid calls, load only against the stand-in.

- **What it does.** Behaves like a careful new user, a careless new user, and a busy production system, and names its seat on every finding: a person typing, a script in a loop, or a program that keeps the result. Picks two areas from the rotation below, writes down everything that surprised, confused, slowed, or failed. Fixes nothing. Each finding becomes one issue file with the smallest reproduction.
- **Rotation.** (1) printed examples on whatever pages changed since last time; (2) unusual input, fresh cases only; (3) shell behaviour and pipes; (4) the misbehaving backend, one failure family per run; (5) the cache under damage, contention, and disk pressure; (6) load and memory at the current record ceiling; (7) the words, help against vocabulary; (8) install on one clean account; (9) the caller's seat, one parity or sizes check per run.
- **Where findings go.** `sdlc/issues/`, one file per finding, committed on main and pushed. A finding in a dependency stays in the notes.
- **What it hands back.** One page: what was run, what passed, what failed, what was not run, the one thing most likely to embarrass us next, and a "what would not embarrass us" paragraph. The embarrassment order is money first, by Ian's ruling of 2026-09-21: silent spend leads, and the release checklist separately blocks on any open blocker.
- **The standing rules.** No paid call without Ian's authorization for that test through `sdlc/scripts/live`; the run's log states which address each command will reach before anything runs; no key is set unless the step needs one, and then it is a canary value; an explicit `--url` on every command that reads a key; test only this workspace's software on this workspace's machines; temporary files die with the test.

## The two reviews, both gates

A review by someone who types commands and a review by someone who calls the tool from a program find different classes of fault, and the plan names both as gates. The typing review is the how-to and demo ladder above. The caller review is the parity, dry-run, sizes, and refused-question tests, and it joins the exploring tester's rotation. (Standing rule added 2026-09-21 from the caller's review that found four things wave 1 missed.)

## The first bounds, from wave 1

The load table in the experiment's wave-1 FINDINGS.md sets the first memory bounds for `sdlc/scripts/load`: the streaming functions (`filter`, `decide --lines`, `tag`) held 7.5 to 8.2 MB peak at both 100,000 and 1,000,000 records, so the streaming bound starts at **16 MB at 1M records**, twice the measured peak. `rank` buffers by design and reached 175 MB at 1M, so its bound starts at **220 MB at 1M**. `find` refuses anything past 255 units in 0.24 s; its guard is the refusal itself. The first run of the script confirms or tightens these numbers, and every later run holds them.
