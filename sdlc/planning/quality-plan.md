# The quality plan

Status: **Proposal under use** from experiment 218, fed by waves 1 and 1.5 (2026-09-21) and the ticket and issue catalog of 2026-09-22. Waves 2 through 4 extend it as the libraries, databases, and the real engine land. Ian can overturn the form and any rung placement.

One rule carries the whole plan: a finding that can be a test becomes one, and a test runs from a rung or it rots. What cannot be a test becomes a line on the release checklist, and what cannot be checked becomes the exploring tester's rotation.

## Part 1: automated tests, by rung

The ladder stays `install`, `lint`, `test`, `spec`. Wave 1 found nineteen issues; thirteen become the tests below. Each names the issue it guards and what it costs to run. The loopback-listener technique is already the repository's own ("a test that claims 'sends nothing' counts the requests on the loopback listener").

### `test` (Rust tests; today ~4 minutes)

| Test | Guards | Cost |
| --- | --- | --- |
| Send-count visibility: a 500-then-ok listener; `--details` must carry the number of sends and the summed usage, and the stand-in count must match | issue 3 (retried send invisible) | seconds |
| Address-change refusal: a recording folder is bound to one backend identity (ticket 0065); a resume under a different address exits 5 before the key is read, sends nothing, prints no result, and leaves the entries unchanged | issue 2 (silent re-bill), fixed by ticket 0065 | seconds |
| Scheduler waiting bound (ticket 0024): at most `jobs` dispatched rows wait while requests are in flight, and streaming output returns before more input is read; assert against a stand-in by counting requests in flight and rows consumed | ticket 0024 | seconds |
- Connection count: the number of simultaneous connections a run opens equals what the ruling on `--jobs` fixes (today, one per in-flight request), and the page says so | the jobs-connections issue | seconds |
| Engine reuse: two calls in one process, module-level and object API both, open one connection on a counting stub (no re-handshake on the second call); each database extension does the same after a warm call | the one-engine-per-process issue (2026-09-22) | seconds |
| Lock cleanup: after a completed `--cache` run, `.locks` holds nothing (or shards with entries); file count equals entry count | issue 10 (locks stay) | seconds |
| Write-before-request: a read-only record folder must refuse before the first send (listener count zero), or the help must say the request happens first | issue 12 (paid answer discarded) | seconds |
| Early close: a listener that closes without a response fails in milliseconds, not at `--timeout` | issue 8 (closed connection costs the deadline) | seconds |
| Sentence pins, one assertion per message: empty evidence, question-file refusals (single file names the offending key), tolerance printed as `0.01`, transport failures name the event and the `--timeout`/`--max-retries` levers, no `timeout: global` string reaches a user | issues 7, 9, 14, 16 | seconds |
| Result-object parity: every function's `--details` carries the same keys; a key one function has and another lacks fails the test (wave 1's caller found `annotate`'s request digest) | the caller-review area, README addition 2026-09-21 | seconds |
| Dry-run parity: `--dry-run` accepts and refuses exactly what the live run does, and prints the request count | same | seconds |
| Sizes battery: 40 bytes, 3 KB, 16 KB, 100 KB, 1.4 MB through each function against a loopback stand-in; nothing is sent that could never be answered | same | under a minute |
| Membership invariance: for every function and every output-shaping flag (`--details`, `--raw`, `--quiet`, `--top`), the set of output units is identical with and without the flag; only the shape of a unit may change. The check runs each function both ways over the same mixed input and compares which records appear. Guards the `filter --details` ruling of 2026-09-21, found by a marketing reader of the deck after wave 1 missed it | the filter ruling, `sdlc/issues/2026-09-21-filter-details-must-still-filter.md` | seconds |
| Row-mapping invariance, every surface: a table of repeated texts, NULLs, and 10,000 rows through every row-shaped function must give each row the same answer the single-row call gives it. One answer per distinct text is computed; the mapping back to rows is the bug the surfaces review found in DuckDB | the surfaces review, wrong-answers class 1 | seconds per surface |
| Chunked-input invariance: the same table arriving in pieces (Arrow record batches, query chunks) produces the same rows in the same order as the whole table | the surfaces review, Python multi-piece annotate | seconds |
| A gate that can fail: every conformance runner exits nonzero on any miss, compares answers exactly (never `want in got`), honors the file argument it is given, and runs from the repository root. The runner is run once against a deliberately broken case and must fail | the surfaces review, conformance-driver class | seconds |
| SIGINT: a spawned record run stopped by SIGINT exits 130, prints the stopped-at line, and leaves every entry complete (needs a process spawn; skip on hosts that cannot signal) | issue 11 (silent stop) | ~2 s |

### `lint` (policy, pages, size, format; today ~1 minute)

| Check | Guards | Cost |
| --- | --- | --- |
| Vocabulary lint over the built help: run `thinkthen --help` and all function helps, grep for the banned words (`document`, `row` in the record sense, `label` for `choose`'s options, `judgment`, `failed` alone for the broken outcome, `rating`, `unresolved`), and for the numbers words (`certainty`, `likelihood`, `cutoff`, `gray zone`, `score` as a name for a probability, `confidence` and `accuracy` and `calibrated` outside their sanctioned uses, a computed number called `probability`); zero hits or the check names each hit. The source is `products/thinkthen/vocabulary.md`, "The words for numbers", which overrides older lines | issue 17, issue 4 (outcome names) | seconds, needs a built binary |
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

A pinning row lands with its fix, red-green: a row that guards an open issue is written the day the fix lands, or it sits in the plan marked red-by-design. Red-by-design today: the SIGINT stopped-at line, and `--dry-run` printing the request count. Both guard open issues; neither is on the rung until its fix lands.

## Part 1a: pinned by the repository's own ladder

The ticket catalog of 2026-09-22 (`experiments/218-thinkthen-release-qa/wave2/ticket-coverage.md`) maps all 68 landed tickets to rows. Eight are pinned by committed repo tests the ladder already runs rather than by rows here (0017 option equivalence, 0022 and 0030 refusal sentences, 0026 entry immutability, 0029 lazy replay, 0031 digest case folding, 0033 threshold validation, 0039 one-send-per-digest): the ladder is the guard, and the exploring tester's rotation re-checks them by hand once a cycle.

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
12. The surfaces review's classes are closed or waived: wrong answers with no error, host crashes at a boundary, cancel per host, and the security rows (file access behind the host's own switch, functions marked direct-only, PUBLIC revoked, relate on the caller's connection).
13. The packages carry licenses and platforms, and no package ships recordings: a recording commits its evidence, and a published package publishes it.
14. A mechanical hygiene check runs zero-hit: no private repository names, no home-directory paths, in shipped files and comments.
15. The transform catalog surface exists (`thinkthen transform list` and `show`, ticket 0050's promise) or the release record names who deferred it; no open ticket owns it today.
16. Issue statuses are normalized to a two-value vocabulary (Open and everything-else) before the waiver sweep runs; the record currently carries nine status words and two files with none.

## Part 3: the exploring tester

A standing brief for one agent that pokes around after each release and on a schedule Ian sets. Budget: one working session per rung of the schedule, no paid calls, load only against the stand-in.

- **What it does.** Behaves like a careful new user, a careless new user, and a busy production system, and names its seat on every finding: a person typing, a script in a loop, or a program that keeps the result. Picks two areas from the rotation below, writes down everything that surprised, confused, slowed, or failed. Fixes nothing. Each finding becomes one issue file with the smallest reproduction.
- **Rotation.** (1) printed examples on whatever pages changed since last time; (2) unusual input, fresh cases only; (3) shell behaviour and pipes; (4) the misbehaving backend, one failure family per run; (5) the cache under damage, contention, and disk pressure; (6) load and memory at the current record ceiling; (7) the words, help against vocabulary; (8) install on one clean account; (9) the caller's seat, one parity or sizes check per run.
- **Where findings go.** `sdlc/issues/`, one file per finding, committed on main and pushed. A finding in a dependency stays in the notes.
- **What it hands back.** One page: what was run, what passed, what failed, what was not run, the one thing most likely to embarrass us next, and a "what would not embarrass us" paragraph. The embarrassment order is money first, by Ian's ruling of 2026-09-21: silent spend leads, and the release checklist separately blocks on any open blocker.
- **The standing rules.** No paid call without Ian's authorization for that test through `sdlc/scripts/live`; the run's log states which address each command will reach before anything runs; no key is set unless the step needs one, and then it is a canary value; an explicit `--url` on every command that reads a key; test only this workspace's software on this workspace's machines; temporary files die with the test.

## The two reviews, both gates

A review by someone who types commands and a review by someone who calls the tool from a program find different classes of fault, and the plan names both as gates. The typing review is the how-to and demo ladder above. The caller review is the parity, dry-run, sizes, and refused-question tests, and it joins the exploring tester's rotation. (Standing rule added 2026-09-21 from the caller's review that found four things wave 1 missed.)

## The rules that outrank every page

Added 2026-09-21 after the `filter --details` miss. A marketing reader of the deck caught what wave 1's 345 checks did not: `filter --details` printed the records the filter had refused, because one spec page said to and the testers graded against the page.

1. **A flag that adds information never changes membership.** A flag may change what is printed about a unit of output. It never changes which units print. Any flag that widens, narrows, or reorders the set of output units is a finding, whatever the page says.
2. **The page never outranks the stranger.** The standard is a careful new user. When behavior makes that user say "what the hell," it is filed against the page that permits it, not excused by it. A page is a claim, not a standard.
3. **Two pages that disagree are a finding on their own.** When two specification pages describe the same behavior differently (`filter.md` said kept-or-not while `result.md` said same records as the bare values), the contradiction is filed the day it is found, even if the binary matches one of them and every printed example runs.
4. **A check that cannot fail is not a check.** A runner that prints FAILED and exits 0, a comparison that tests `want in got`, a checker that ignores the file it was given: each is a green lie. Every check proves it can fail before it proves anything else.
5. **No test fixture lives in product code.** Failure injection is a mode of the harness or the stand-in, never a constant in the engine that fails one fixed text on the live path.
6. **Row answers are positional.** Repeats, NULLs, chunks, and packing must never move an answer to another row. And where packing legitimately changes an answer (neighbors matter, the wire probe measured 34 of 1,000 moving on regrouping), the page says so and the cache key covers the whole request.

## The first bounds, from wave 1

The load table in the experiment's wave-1 FINDINGS.md sets the first memory bounds for `sdlc/scripts/load`: the streaming functions (`filter`, `decide --lines`, `tag`) held 7.5 to 8.2 MB peak at both 100,000 and 1,000,000 records, so the streaming bound starts at **16 MB at 1M records**, twice the measured peak. `rank` buffers by design and reached 175 MB at 1M, so its bound starts at **220 MB at 1M**. `find` refuses anything past 255 units in 0.24 s; its guard is the refusal itself. The first run of the script confirms or tightens these numbers, and every later run holds them.
