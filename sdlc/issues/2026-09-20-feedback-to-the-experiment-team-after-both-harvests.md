# Feedback to the experiment team after both harvests

Status: Open

Ian asked on 2026-09-20 whether the team behind `experiments/205-thinkthen-libs/` and `experiments/207-thinkthen-db/` should wait for the build team. Mostly yes. Three pieces of work do not wait, and the first is worth more than the rest. This page sits beside the plan in `2026-09-20-what-the-two-experiments-ask-of-the-engine-and-the-order-to-build-it.md`.

The work was good. Two experiments found the same missing layer from two directions, checked their own results by command, and named what they faked. The fork hang, the width finding, and the cancel numbers changed the build order.

## Do now, in this order

**1. Run the same benches on a blocking engine.** The stand-in uses an async runtime. The real engine is a blocking client with scoped threads and no runtime, and the plan recommends it stay that way. That recommendation rests on reasoning. No run has tested it. Build a third stand-in over the real `thinkthen-core`: a blocking `ureq` client, scoped threads, one width gate for the process, the cancel token, and a process-ID check that rebuilds the pool and the gate. Bind it to Python and PostgreSQL. Both fork. Four questions:

- Do 1,000 records at 300 ms and `jobs` 32 still land near 9.65 s, and the null backend near 1.3 µs a record?
- Does the process-ID check end the fork hang on the wire, under `multiprocessing` and in a PostgreSQL backend?
- Do 100 calls at once stay at 32 in flight?
- How does the calling thread hear an interrupt while it blocks? Python, Ruby, R, SQLite, and PostgreSQL each check for an interrupt on the calling thread. With that thread blocked, nobody sets the token. Settle the shape. Either the engine runs a poll callback on the calling thread on a short tick, or each binding waits in its own loop.

The ADR 0017 rewrite waits for these four answers and for nothing else.

**2. Prove the DuckDB interrupt inside Python.** The extension takes SIGINT at `LOAD` and chains to the previous handler. The proof ran under the DuckDB command line. Most DuckDB use happens inside a Python process, and Python owns SIGINT there. Show three things: Ctrl-C still raises `KeyboardInterrupt`, the query stops, and a host that installs its handler after `LOAD` still works. If one fails, the handler is for the command line alone, and the DuckDB page says so.

**3. Fix the cases before they become the shared suite.** The plan takes the twenty cases as the start of the conformance suite.

- `cases2/03-score-levels.json` expects a probability mapped to a band with an `under` bound. `specification/score.md` defines a number from 0 to K−1: the position on the named levels, weighted by probability. Rewrite the case to the specification. `thinkthen_score` returns that number in all three databases, and the nearest level's name goes in `thinkthen_details`. Object if the evidence argues against it.
- The error kinds follow the command's classes: usage, backend with the network included, local file or recording, cancelled, defect.
- Questions use the grammar in `specification/question-file.md`, for `choose` and `score` as well as `decide`.

## Added 2026-09-21, while job 1 runs

Job 1 runs in `experiments/211-thinkthen-blocking-engine/`. Its engine lane reported 9.666 s on the bench, 32 in flight on 33 connections for 100 calls at once, and a forked child that answers. Two finds already belong to the real engine: the connection pool is sized to the width gate, because the default of ten idle connections churned 488 connections where 33 serve, and a wait loop must end after a cancel. Five more checks fit this run, and none widens it past the two hosts:

1. **Fork during a batch, as well as after a call.** A child forked while 32 threads run inherits the gate and the pool in whatever state they held, and none of those threads. The process-ID check must replace that state without taking an inherited lock. Test it on the wire, in Python.
2. **What the engine holds between calls.** After a batch of 100,000 records: the thread count, the open connections and how long they stay open, and the resident memory against the start. A host that sleeps for an hour then calls again must not meet a dead pooled connection as an error.
3. **A deadline for one call.** A host that answers a person gives a call five seconds. Show the shape beside the cancel token. `2026-09-21-what-a-tool-search-feature-asks-of-find-as-a-function.md` has the case.
4. **An error that says whether a second try could help.** The same page. A busy backend and a refused reply are different to a caller with a fallback.
5. **A million cache entries.** Time to find one entry, time to fill, and disk used, for the flat folder, a folder split by the first two characters of the digest, and one SQLite file. `2026-09-21-the-disk-cache-is-never-on-unless-the-user-names-a-folder.md` has the reasons. This one can follow job 1 and does not hold the ADR.

The ADR 0017 rewrite also reads three pages filed on 2026-09-21: the cache page, the tool-search page, and `2026-09-21-triage-of-the-open-issues-by-layer.md`.

## Added 2026-09-21, after experiment 211 reported

The run answered its four questions and the five added checks, and the orchestrator reran every suite by hand. Staying blocking is now a measured choice: 9.66 s in Rust, 9.658 s through Python, and 9.673 s inside PostgreSQL, at 32 in flight. The fork fix holds by construction, and the fork-during-batch test caught that the first design passed for the wrong reason. The order from here changes in one way: **the ADR 0017 draft goes first.** It gates the engine, and the build team reaches that gate when the transforms land. Job 3 follows, then job 2, then the hand-off list.

Four questions on the findings, each small:

1. **A spent deadline is the caller's own limit, and the findings file it as a retryable backend error.** A caller with a fallback treats "my five seconds ran out" and "the backend is busy" differently: the first says nothing about the backend's health. Give it its own kind, or a field beside the retryable flag.
2. **Eighty-three threads at a width of 32.** Name the other 51. A host that embeds the engine sees that spike, and the embedding note needs the cause and whether it scales with the width.
3. **A retry after a dead connection can bill twice** when the request left before the connection died. A judgment is safe to repeat, so the behavior is right. The usage counters should count both sends, and the page should say so.
4. **The cache measurement and the lock files.** The command keeps one lock file per entry under `.locks/`, seen on 2026-09-21. Say whether the million-entry numbers counted them. If the sharded folder is the pick, the locks shard the same way or leave after the entry lands.

Two inputs the ADR draft reads beside the ones already listed: `2026-09-21-one-shape-for-nine-surfaces-as-the-slides-show-it.md`, with ten picks on names, the empty value for "not sure", bands, `score`, and `decide_many`, and `2026-09-20-a-status-command-for-configuration-and-usage.md`, for the request limit as an engine setting. The draft objects wherever the evidence disagrees with a pick. Job 3 writes the shared cases to those picks once the draft settles them. Every page says "0.1" or "the first release".

## Hand off

- **The changes for each surface go to the planning pages.** Each experiment README lists the changes it would make to its `GOALS.md`. Those files are copies of `sdlc/planning/libraries/*.md` and `sdlc/planning/databases/*.md`. Apply the changes to the real pages, one commit for each folder, each change with its evidence. Main is clear of the design holder's edits now.
- **Each dependency problem gets its own issue here, with the smallest reproduction.** Two so far: the scalar-bind surface in DuckDB's stable C API, and the `rusqlite` headers that stop at SQLite 3.34. Nobody opens an issue upstream. Ian decides.
- **Record the literal lines.** The bulk call in each language and the example query in each database, exactly as they ran. The findings table has no literal line for Python or R. The talk and the site copy these lines.
- **Check the five answers.** The plan page answers five parked questions. Check each against the evidence and object where it disagrees.

## Wait

Everything that needs the real engine waits: a third round of bindings, packaging in CI, registry uploads, and any speed number for a slide. The real libraries land in this repository through tickets after the ADR. No package built in the experiments folder ships. Builds before the release carry 0.0.N, per `2026-09-20-the-first-release-is-0-1-on-every-surface.md`.

## What Ian can overturn

All of it. Skipping item 1 is the cheap overturn. The cost is that the build team learns at step 3 of the plan whether blocking holds.

2026-09-21: job 1 has landed. The four answers and the five checks added today are in `experiments/211-thinkthen-blocking-engine/FINDINGS.md`, with the commands and output in that folder's lane notes. Blocking holds; the ADR 0017 rewrite can be drafted from it.

## Added 2026-09-21, after the ADR 0017 rewrite and the foyer verdict

The marketing side checked the rewritten ADR 0017 (`9b52ea9`) against the ten picks and against the slide code in the marketing repository. All ten picks are adopted, and the slide code matches the ADR on names, argument order, the empty value, `Engine::from_env`, `.band(0.2, 0.8)`, and `'@refund.json'`. The foyer verdict is accepted: a forked child that hangs and a 430 ms cost on every open each rule it out for a command that starts fresh on every call.

Five requests before or during job 3:

1. **Which databases get `thinkthen_warm`.** The ADR says experiment 207 answered it and names the DuckDB page. The slides show it on SQLite, where a query judges row by row and needs the warm pass most. The ADR names each engine that ships it and says why.
2. **The bulk spelling in C.** The ADR admits `decide_many` by name on Python, TypeScript, and Ruby only. The C slide shows `thinkthen_decide_many`, because every binding that loads the C library needs one bulk entry point. The ADR either admits it for C or names C's bulk spelling.
3. **The cache location against the written rule.** The repository rule says the tool "never writes a file the user did not name". The ADR recommends the platform cache home when the cache is on with no folder named, and it never cites that rule. The ADR states which exact setting turns the cache on without a folder, and it says plainly that accepting the recommendation amends the rule. Ian rules on it.
4. **A retried send reaches the user.** The counters count two sends for one judgment. The ADR says where a user sees the second send: `details`, `usage`, or both. A bill that shows two requests and a tool that shows one is a trust problem.
5. **The deadline message.** The error names the limit that ran out and its value, so a reader knows which setting to raise.

One request for job 3: the twenty shared cases live in one data file that every surface reads. The ADR is Proposed, and a changed pick is then one edit.

One sentence wanted for the manual, from the thread finding: "ThinkThen holds no threads between calls, and during a call it uses about one per request in flight." The experiment team confirms or corrects it.

Ian can overturn any of these.

## Added 2026-09-21: Ian ruled XDG

Ian ruled that XDG is the default home for the cache and the configuration. Request 3 above is answered: ADR 0017 states the ruling, cites the amended rule, and drops "Recommended". The full ruling is in `2026-09-21-the-disk-cache-is-never-on-unless-the-user-names-a-folder.md`. A forked child and nine surfaces now share one default folder, so the many-processes test on the sharded store matters more than before.

## Answered by the experiment team, 2026-09-21

The five requests and the manual sentence, answered in ADR 0017 and here:

1. `thinkthen_warm` ships in all three databases; the ADR names them, and SQLite leads in need because a query there judges row by row.
2. The C door carries `thinkthen_decide_many`, admitted by name in the surface check.
3. The cache ruling is written into the ADR with the amended rule and its one stated exception; "Recommended" is gone. The on-by-default reading is marked Ian's alone to overturn, with the three guards.
4. The second send shows in both places: `usage` carries the process's send count, and a judgment's `details` carries the sends that produced it.
5. The deadline error names the limit and its value.

The manual sentence, confirmed with one correction from the measurements: "ThinkThen holds no threads between calls, and during a call it uses about one thread per request in flight, briefly twice that while new connections open." The brief doubling is the HTTP client's resolver, one short-lived thread per simultaneous dial during the ramp; it settles to one per request once the pool is warm and vanishes when the call ends. The real engine halves the ramp with a synchronous numeric resolver.

Job 3 runs now with Ian's condition: the twenty cases land in one data file every surface reads.

2026-09-21, closing: the brief's whole arc has landed. Job 1's report is above and in `experiments/211-thinkthen-blocking-engine/FINDINGS.md`, with the five added checks answered, the deadline ruling implemented as the sixth error kind, the 83 threads named, the retried send counted, and the cache compared across four stores including foyer. The ADR 0017 rewrite carries the ruling and the ten picks (`49fb7ad`). Job 3 delivered the twenty cases as one conformance file, `experiments/207-thinkthen-db/engine/cases2/conformance.json`, validated offline, with the divergence table recorded (`2dbdd2f2`'s lane; the file is the record). Job 2 proved the DuckDB interrupt inside Python and the page carries the three sentences (`2c9b831`). The hand-off landed as five commits: `019af68` and `462a36e` apply the goals-page changes with their evidence, `635034a` files the two dependency problems with smallest reproductions, `264b503` records the literal lines for all six languages and three databases, and `38cdbd4` checks the five parked answers, all confirmed. Nothing is left open on this brief. The build team's gate is the ADR review; Ian's word accepts it.
