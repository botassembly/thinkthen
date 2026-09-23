# Stand-in parity with main, wave 7

Date: 2026-09-23. Branch `w7/sync`, pushed as `surfaces-wave7`. The wave merged main at `1b894d9`. Main had landed four tickets that change how an engine sends, stops, and relates. The stand-in answers the surfaces until the production engine replaces it, so each ruling was checked against it here. Each item names main's source, the decision, the proof, and what Ian can overturn.

## 1. Retry waits stop at the attempt timeout (main's ticket 0064)

Main caps every retry wait at `--timeout`. A `Retry-After` header wait is the least of the header, 60 seconds, and the timeout. A doubled backoff is capped at the timeout too.

The stand-in held the 60-second ceiling only. A 30-second header on a 250 ms timeout waited 30 seconds, and a doubled backoff waited 1 s then 2 s. The stand-in now caps the wait at its attempt timeout, in `post` in `standin/src/lib.rs`.

Proof: `standin/tests/retry_wait_bound.rs`. Before the change both tests failed. The header test waited 30.05 s, and the backoff test passed its 2.5 s limit. After it, the two tests pass in 0.57 s. Each listener counts the requests it read, so the tests also pin one first attempt plus every retry. The gate runs every stand-in test file except `wire.rs`, so the file runs in the gate with no new wiring.

Commit `1c09f86` carries the change. Main's per-result send count needed no change. `Details.sends` already counts every wire send that produced a judgment.

Ian can overturn the cap. The cap follows main's ruling, so overturning it would reopen ticket 0064 on main too.

## 2. Cooperative cancellation and SIGINT (main's tickets 0073 and 0074)

Main's engine checks the token once, right before it counts and sends each attempt. It polls the token during retry waits. It lets an attempt already sent finish within its timeout. The command then flushes finished output and exits 130.

The stand-in already met every rule a library can observe. `post` checks the token and the deadline at the top of each attempt, before it counts or sends. `sleep_checked` polls the token during a retry wait every 100 ms. A retryable failure turns into the cancelled kind only when the token fires before or during the next wait, and its sends stay counted. `standin/tests/backoff.rs` and the wave-6 no-resend tests prove these rules. No test could go red, so this wave adds no code for them.

One difference stays by design. Main's command blocks SIGINT on its engine threads, so a sent attempt always finishes. A library cannot own a host's signals. ADR 0017 section 2 rules the library shape instead: the call returns within one poll tick, and nothing is served after the return. The stand-in resumes a read that a harmless signal interrupted. It abandons the read only when the call's own token fired. The request was sent and is counted either way. The paid answer is lost only in that case, and the stand-in keeps no cache that could hold it.

SIGINT ownership, exit 130, and default-signal emulation belong to the command, so they have no stand-in counterpart. Each surface keeps its own channel: Python's `KeyboardInterrupt`, R's tick, Ruby's and Node's interrupts, and the databases' interrupt checks.

Ian can overturn the abandoned-read rule. The other choice is for the stand-in to block SIGINT on its own worker threads, as main's command does. Single calls would still run on the host's thread.

## 3. Request splitting under backend limits (main's ticket 0079, ADR 0040)

Main splits a multi-question plan into the fewest requests that fit an explicit backend profile's byte and question limits. It refuses a `choose` with more options than the profile's `max_options`. A plan with no profile is never split, and the 255-option ceiling stays.

The contract has no profile setting, so a surface cannot name a limit, and the stand-in has none to split under. The stand-in and the contract already refuse more than 255 options and more than 255 relate records. No branch file carries the withdrawn 100-option claim. This wave adds no code. The contract gains a profile setting only when a surface needs one, and that widens a public surface, which needs a second-agent review.

Ian can overturn this by asking for a profile setting on every surface now.

## 4. Relate uses method H (main's ruling of 2026-09-23)

Main ruled that every relation uses method H: one yes/no question per pair per relation, and each direction of a one-way relation is its own question. The ruling is in `sdlc/planning/relate-design.md`, from the bake-off in `sdlc/issues/2026-09-23-relate-methods-bake-off.md`.

The stand-in's four relate rows come from the earlier harvest package. They hold the pick-one method, which returns at most one relation per pair. The stand-in had no method-H recording at all.

`conformance/tools/build_relate_yes_no.py` now reads the bake-off's recorded method-H answers, with no network and no key. It adds two rows of form `yes-no` to `standin/data/recognize-replay.json`: the eleven travel rules (`contradicts`, both ways, 55 questions) and the seven-step cause chain (`causes`, one way, 42 questions). The replay returns every recorded question whose yes reaches the bar, so one pair may carry several relations. `build_recognize_cases.py` keeps these rows when it rebuilds the table. Commit `cc2c036` carries the change.

Proof: `standin/tests/relate_yes_no.rs`. Before the replay change both tests returned no edges. After it, the chain returns 1 to 2, 2 to 3, 3 to 4, and 4 to 5, each the right way round. The travel rules return the four true contradicting pairs. These are the bake-off's own answers at the 0.5 bar.

What stays open for Ian: the deck's relate sample, the alerts in `R01-demo`, and the other three older rows still answer by the withdrawn pick-one method. No method-H recording of those records exists. The conformance file's relate cases derive from them, so every surface still replays pick-one numbers for them. The options:

- (a) Record the four older relate texts under method H with a live run. This is a paid call under `sdlc/scripts/live`, and it needs Ian's authorization. Then regenerate the table and the conformance cases. This is the recommendation. The bake-off spent under 6 cents for 92 requests, and these four texts need four requests.
- (b) Replace the four rows with bake-off sets, and move the deck's relate sample to one of those sets. This needs no paid call. It changes a public slide sample.
- (c) Keep the pick-one rows as marked recordings of the withdrawn method until the production engine replaces the stand-in. This costs nothing now. The surfaces' relate conformance then proves a method main no longer uses.

### Option (a): the method-H recording (2026-09-23)

Ian approved option (a) on 2026-09-23, with a budget of at most $2 of paid calls for wave 7. This recording spent part of it.

`conformance/relate-h/` holds the work. `sets/` names three record sets and the package rows each one replaces. `alerts` replaces `R01-demo`. `founders-24` replaces `R02-pickone`. `staff-10` replaces `R03-persubject-10` and `R04-pairs-10`, because those two rows hold the same ten records. `arms.py` writes the questions in the bake-off's lean method-H wording. The records carry kind `*`, so every ordered pair is legal for a one-way rule. That gives 18 questions for the alerts, 1,104 for the founders, and 90 for the staff records. `job.sh` asks them through `annotate` and records each request under `runs/<set>/H/cache/`.

The job first ran against a local stub on port 8451 with a placeholder key. The stub counted 3 requests of 18, 1,104, and 90 questions. A second run sent nothing, because the cache answered it.

The paid job then ran once: `sdlc/scripts/live --max-tokens 100000 conformance/relate-h/job.sh`, with the key passed only by the live door. It sent 3 requests. `live --status` read 422,954,418 charged tokens before and 423,054,418 after, so the ledger charged 100,000 tokens. That is about $0.004 at the bake-off's rate. The backend reported 26,645 input tokens (768, 23,600, and 2,277) and 22,911 output tokens. Ian can overturn the wording choices in `sets/`. Changing them needs another paid run.

### The replay switch

`conformance/tools/build_relate_yes_no.py` now adds the three sets to `standin/data/recognize-replay.json` as `yes-no` rows. It removes the four pick-one rows they replace, and it rewrites conformance cases 69, 70, and 72 from the recording under their old ids. Each row keeps the package's numbered text ("Alert 1: ...") as its second match key. `build_recognize_cases.py` keeps these rows and cases when it rebuilds from the package. Case 71 stays as the pinned per-subject arm, and every runner still skips it by form.

The stand-in's relate replay now reads only `yes-no` rows, and the pick-one path is gone from `standin/src/replay.rs`. It also refuses a rule asked in the other direction from the recording. The alerts recorded `caused_by` one way and `same_as` both ways. A one-way answer served for a both-ways ask would invent the lower-first order, so the ask is a usage error that names the recorded direction. The validator checks the same rule on every case.

Proof, red then green:

- `standin/tests/relate_yes_no.rs` gained four tests: the alerts, a founder pair that holds both relations, the staff records, and the direction refusal. Before the table change the first three returned the pick-one edges, for example `founded` 1 to 2 alone at 0.87. Before the refusal the both-ways ask returned `caused_by` 2 to 1. All pass after.
- `validate_conformance.py` from before this change fails cases 69, 70, and 72 on the new file ("edges replay exactly"). The new validator passes all 84 cases.

What changed for every surface. At the 0.5 bar the alerts now answer `caused_by` 1 to 4 (0.71), 2 to 1 (0.65), 2 to 4 (0.73), and 3 to 4 (0.55), plus `same_as` 1 and 2 (0.61) when asked. No edge reaches 0.9. The slide's bar moves from 0.9 to 0.7, which keeps the same two sound edges, 1 to 4 and 2 to 4. Each surface's test, example, and docstring moved with it. The slide itself lives outside this repository and still draws 0.9 and 0.94. Ian can overturn the 0.7 bar. The other choice is to keep 0.9 and show an empty answer.

The direction refusal found three runners that dropped a rule's `either` flag and asked `same_as` one way. The pick-one replay had hidden it, because it ignored the flag. The SQLite driver now passes `either:NAME`. The DuckDB runner and its review tools now share `relate_rules` in `databases/duckdb/tools/review4_lib.py`, which sends the JSON spec when a case asks a both-ways rule. The PostgreSQL call takes bare names only, which its NOTES.md already records as a gap. Case 69 is skipped there with that reason in the skip table. Ian can overturn the skip by asking for an `either` spelling in the PostgreSQL call. That widens a public surface and needs a second-agent review.

Checks run at this commit, with no stub: the stand-in suite, the conformance validator and its tests, the skip table, and the surface checks for Python, TypeScript, Rust, C, R, SQLite, DuckDB, and PostgreSQL all pass. The Ruby check could not run here, because the `thinkthen-ruby-builder:local` image is missing. Its two edits, `tests/test_surface.rb` and `examples.json`, are unproven until the gate runs. Under four parallel builds, timing tests failed once in Python, TypeScript, and DuckDB. Each passed when run alone.
