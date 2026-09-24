# Surfaces branch: the fifth review and the wave-6 fixes

Date: 2026-09-23. Reviewed tip: 398d7bb (the dev team's wave 5). Fixed tip: 610a133 on branch `surfaces-wave6`. Companion page: `2026-09-23-surfaces-branch-error-index.md`, one row per defect across all rounds.

## Verdict

Wave 6 fixed most of what the fifth review found, and four independent verifiers confirmed those fixes. The branch is not ready to merge. Three things block it:

1. The error index is traced from documents and is not yet probe-verified. Main's quality plan (item 17) requires every row closed or waived by a named independent probe. After wave 6, 56 of 184 rows are Open, Partial or Unknown. Some of those carry a stale round-4 status and are in fact fixed. A triage wave must prove each one.
2. The branch has not taken main for two days. Main is 305 commits ahead. A trial merge conflicts only in two issue documents, but main landed request splitting, SIGINT cancellation and the relate method ruling, and the stand-in engine carries none of them.
3. A rare crash at the C door, about one run in 83, is still unexplained.

The full gate at 610a133 passed, offline, with every local test server up: `all landed checks green: green=1001 skipped=91 diverged=0 failed=0 (wire: 10 of 10 wire suites ran in a passing surface, 0 in a failed surface, 0 lost)`, exit 0. The lint rung also passed at 610a133, with the surfaces ratchet at 26848/26848. An earlier run at the same tip failed honestly: the worktree lacked the DuckDB command-line binary, and the gate printed and counted `FAIL surface-duckdb`.

## How this wave ran

The dev team's wave 5 ended at 398d7bb with a verification table claiming 20 rows closed and 12 open. Six reviewers re-checked those claims with the fourth review's own probes. Ian then asked the steering session to fix the findings directly instead of handing them back. Each reviewer became a fixer in its own worktree and branch (`w6/standin`, `w6/python`, `w6/ruby-r`, `w6/duckdb`, `w6/sqlite-pg`, `w6/gate`). The steering session merged the lanes into `surfaces-wave6`. Four fresh verifiers, who wrote none of the fixes, then checked every claim. Their findings went back to the lanes for a second round of fixes.

Every fix followed the fourth review's rules. The reviewer's probe failed on the old code and passed on the new code. A regression test landed, and the verifier broke the fix in a scratch copy to show the test goes red. The gate ran at the exact tip before this note was written.

## Wave-5 claims the fifth review did not accept

- The stand-in use-after-free still crashed the C door (2 of 54 runs), and ThreadSanitizer found 9 data races in the new slot code. The team's 614,924 clean cycles did not close it.
- The Python Arrow reader still read past the end of a buffer. The verifier had called two crafted shapes "void".
- A signal still made Ruby send a paid request twice. The team's stub counted a request only after its delay, so it never saw the first one.
- Wave 5 made Ctrl-C stop working in a Ruby bulk call. It ran 10 seconds and sent all 40 requests.
- The DuckDB Ctrl-C handler took a lock and allocated memory inside the signal handler, so it could freeze the process.
- The gate did not reproduce offline. Four surfaces needed network setup, the summary said "stub up" after every wire suite was lost, and 18 "diverged" cases hid mismatches. One PostgreSQL case passed on any answer.
- `CARGO_NET_OFFLINE` does not imply `--locked`. The PostgreSQL release library carried 163 home paths. The R install still failed offline. The CI ratchet failed on a shallow clone. ADR 0031 collided with main's 0031.
- SQLite's forked child lost its interrupt or hung. The SQLite question cache could keep a stale parse. PostgreSQL checked a file's size before its confinement and leaked it.

## What wave 6 fixed, confirmed by an independent verifier

Stand-in, contract, C, Rust:
- The slot table now uses one short lock and counted references. ThreadSanitizer found 5 to 8 races on the old code and 0 on the new.
- A request whose bytes left is never sent again after a reset, a timeout or a signal. The server got 3 requests before and 1 after. A harmless signal on the worker thread no longer fails the call as cancelled.
- The width ceiling holds on every backend. Width 100,000 used 334 MB and crashed before, and it is refused now.
- The C door reads its flags by value. A thread's failure entry leaves the table when the thread exits. Too-large deadlines are refused with one rule, which ADR 0041 records.
- The churn and scale tests run in the gate and fail when the fix is broken.

Python:
- The Arrow reader bounds every read by what the array declares. Seven of nine crafted shapes that crashed now refuse, and the four start-offset shapes found by the verifier refuse too. Normal columns over 4 MiB work again.
- Deadlines refuse bools and text. Error messages name the verb called. Number and dict questions carry the usage kind.

Ruby and R:
- Ctrl-C stops a bulk call in about 1 second after 4 of 40 requests. A signal sends one request. A cancelled token sends nothing across all 11 verbs. Internal names are hidden at runtime.
- R sends UTF-8 under the C locale, keeps one-item lists as lists, raises its refusals with the usage kind, and its interrupt fallback works.

DuckDB:
- The Ctrl-C handler only writes to a pipe. A forked child's Ctrl-C no longer cancels the parent. A Ctrl-C with a chained host handler stops the query 20 of 20 times, where it stopped 10 of 20 before.
- Relate refuses a large grouped or windowed input early, with a setting `thinkthen_relate_holding_rows` to raise the limit.
- Logical types are freed. Identity names come from system randomness. The deny allowance covers three named crates.

SQLite and PostgreSQL:
- A forked SQLite child hears its own interrupt, 60 of 60. The cache stamp comes from the parsed file's own descriptor, and stale answers fell from 121 in 1,500 to 0.
- PostgreSQL confines a path before it opens it, and on Linux it opens beneath the directory. Size, hard links and timing no longer leak.
- The PostgreSQL answer table has a byte budget. Saved answers are keyed on the model in both databases.

Gate:
- Every surface that fails or is not set up prints a counted FAIL. A wire suite whose stub died fails. Node's skips count as skips. "Diverged" is gone, and every mismatch prints FAIL.
- Every build call carries `--locked`. Every package-manager call is offline, and a check enforces it. Built files carry no home path. The shallow-clone ratchet names its real problem.
- ADR 0031 is now 0041. MERGE-NOTE's citations are true, and it names both conflicting files and the ratchet cost of option (a).

## Still open, with owners

- The C door churn crash at new-thread start. It is not reproduced in 32 verifier runs, and ASan, TSan and a thread-start tracer were clean. Owner: stand-in. Issue on the branch.
- Main's own CLI retries a request after a transport failure. Owner: core. This needs its own issue on main.
- DuckDB `con.interrupt()` cannot cancel a running call, because the C API exposes no way to read that flag. The relate row cap trusts the planner's estimate, and a badly estimated join still reaches about 1 GB. A Ctrl-C after a query's last engine call is not caught. Owner: DuckDB. Each needs a decision: accept it as a documented limit, or change the design.
- Python Arrow: a column claiming about 4 billion rows over a tiny buffer can read far before it faults, and lowering MAX_ROWS is the lever. Offsets that agree with each other but pass a short real buffer cannot be detected through the C data interface. Owner: Python.
- The SQLite answers table has no size bound. A same-size rewrite with a restored modified time serves stale, which is documented. The warm cap is a constant. Owner: SQLite and PostgreSQL.
- Over HTTPS, a signal that keeps repeating can hold a read past its deadline. It never resends. The connection pool uses an unstable ureq API, pinned at 3.4.2. Owner: stand-in.
- The other surfaces' wire suites have not been audited for self-skip. The gate's "not set up" check misses the DuckDB command-line binary. Owner: gate.
- Nothing ran on a Mac. Owner: Ian, through a todo.
- Three wave-5 ceiling raises (78315ba, d9cc3db, 252f530) name no second-agent review. History was not rewritten.
- The product deck changed on 2026-09-23, after DuckDB's frozen fixture copy was taken.

## Process findings about this wave

- The steering session's first gate run downloaded about 120 MB of public Python packages, which breaks the no-network rule. No credential or paid service was involved. The Python check's `uv` call had no offline switch. Every package-manager call is now offline, and a gate check enforces it.
- Two gate runs hid surfaces that never started. DuckDB and SQLite exited nonzero without a counted line. This is fixed.
- One lane's broad kill command stopped other reviewers' stubs. Lanes now stop processes by their own ports.
- One lane's SQLite build folder was a link to an older build, so some of its runs tested old code. The independent verifier built fresh and confirmed every SQLite claim.
- The steering session started a merge in the integration worktree while a gate was running there, and aborted it within seconds. That gate is not cited.
- The box ran at load averages of 300 to 480 for hours, partly from other sessions. Several timing tests failed under that load on untouched code and were made load-tolerant without loosening their limits.

## Decisions Ian can overturn

- The fixes landed on `surfaces-wave6`, not `surfaces`. `surfaces` stays at 398d7bb until Ian approves a fast-forward.
- DuckDB kept an estimate guard with a setting, and did not use a global memory limit, because DuckDB 1.5.5's memory limit is global only and would cap the host's other queries.
- The DuckDB drawn-calls fixture is a frozen copy with a hash pin, not a live copy of the deck.
- PostgreSQL refuses an outside symlink that points into the confined directory.
- The ratchet check accepts "second-agent" with a hyphen as well as "second agent".
- Python refuses a bool as a deadline, as Node does. ADR 0041 carries the amendment.

## Next

1. Triage the 56 non-closed rows of the error index. Each row gets its proving probe run at the tip, and is then closed, left open with an owner, or waived by a decision. This is main's release gate.
2. Merge main into the branch in a new worktree, not into main. Bring the stand-in up to main's request splitting, SIGINT cancellation and retry fix.
3. File main's CLI retry bug as its own issue on main.
4. Bring the DuckDB limit decisions to Ian with options.
