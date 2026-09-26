---
flow: build
priority: 145
opens: crates/thinkthen/tests/speed.rs probes/speed probes/README.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0145: The speed test

Status: ready. The coordinator accepted it on 2026-09-26 after a fresh read-only review. Owner: Claude. Lane: `worktrees/thinkthen-lane-2`. It builds only after tickets 0143 and 0144 have landed on main.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A builder who lands a batching ticket sees at once whether each function still sends one request per record. Ian sees, on a named build, how long `filter` over the 306 songs takes, and what every function and every Beatles Bench job costs in requests, time and tokens.

This is ticket S1 in `sdlc/issues/2026-09-26-batching-design.md`. Its row reads: "The speed test. Described below. Its gate part passes with its list of functions still to batch; its live part reports on a named build. Depends on B1, B2, J1, B3. Needs no ADR." The design's "S1: the speed test" section describes it. Ian's ruling 4 of 2026-09-26 sets the target: `filter` over the 306 songs finishes in under half a second at the default throttle of 4, on a named build, measured live. Ruling 5 asks for this test. Ruling 9 orders S1 after B3 and before B4. ADR 0048 item 12 gives the target to S1.

The ticket has two parts.

- **The gate part** runs in the `test` rung. It needs no engine and no real key. A loopback backend answers every request. It fails any function that sends one request per item where a fuller request fits, unless that function sits on the named list of functions still to batch.
- **The live part** is a job for `sdlc/scripts/live`. It runs only as a separate, named run that Ian authorizes. The builder never runs it and never runs `sdlc/scripts/live`. S1 lands without it. The baseline run, "S1 live run 1", happens on a main commit after S1 lands and before B4 builds.

B1 (ticket 0141) and B2 (ticket 0142) have landed. J1 (ticket 0143) and B3 (ticket 0144) are in flight. The build starts only after both have landed on main. The coordinator set this order, and Ian can overturn it.

## What happens today

- Each record is its own request. `decide`, `filter`, `rank`, `choose`, `tag` and `score` send one request per record. `annotate` sends one request per record for each `on` group (`specification/annotate.md`, "Requests"). `recognize` over records sends one request per text, plus relation requests. `find` already sends its whole set in one request. `relate` sends one request per relation, and ticket 0143 sends them together under `--jobs`.
- Ticket 0144 builds the batch planner in the pure core. It changes no surface. Ticket B4 puts batches on the command for `decide`, `filter` and `rank`. B8, B9, B10 and recognize's R7 follow for the other functions.
- No test counts requests per function across the whole command. Each ticket pins its own function.
- `thinkthen status --json` reports `usage.total.requests_sent`, `input_tokens` and `output_tokens` from the usage file. ADR 0049 writes that file behind the requests, and the command waits for the writer before it exits (`specification/recording.md`). A private home per run therefore gives one run's own counts.
- The conformance backend (`conformance/backend`, a dev-dependency of the crate) serves a `/generic/v1` arm that answers any well-formed request and counts requests at the socket. Its default arm reports no usage.
- `sdlc/scripts/live` runs a job with the checkout's `target/debug/thinkthen` first on `PATH` and hands the job `THINKTHEN_API_KEY`. The job's first line must be `#!/bin/sh`.
- Beatles Bench (`github.com/botassembly/beatles-bench`, commit `7d246844` when this was written) runs each case as its own command of one record (`scripts/run/functions.py`, `scripts/run/ask.py`). So the bench as it runs today cannot batch. Every case file under `functions/` shares one set of arguments. The `audit` folder's case files ask `decide` over 70 records each, so they send requests. The `diff` folder asks nothing. The bench's 1,501 questions fall into 140 groups that share a function and a question. Its `data/songs.tsv` title column, in file order, is experiment 268's `songs.txt`, 306 lines with SHA-256 `3250fed3858ab12481d7e42b70d5eeeb18292db8a7580ec397a91bcd89bcdffd`.

## Design

Everything lives in a new folder, `probes/speed/`, beside the other live measurements. `probes/README.md` gains one row for it. The folder name has no leading number, so the `spec` rung's replay check passes it by, as it passes `token-budget/`.

### The folder

| File | What it holds |
| --- | --- |
| `functions.jsonl` | One row per function: the verb, its arguments, its input file, `items`, `floor`, and, for a function still to batch, its `list` entry |
| `workloads/` | The gate's inputs: 12 made-up one-line records, the same 12 as JSON Lines, a 12-line set of short made-up sentences, three made-up names, and one `annotate` question set |
| `measure.py` | The one runner. Modes `gate`, `plan` and `live` |
| `job.sh` | The live job. `#!/bin/sh`, then `exec python3 probes/speed/measure.py live "$@"` |
| `README.md` | What each mode measures, the list rule, how Ian authorizes a run, and each run's folder |
| `runs/NAME/` | Each authorized live run's rows and build facts. The first appears with the first live run |

### The function table

`functions.jsonl` holds ten rows, one per function. Each row names:

- `function`: the verb.
- `args`: its arguments after the verb, with no address, key or cache flag.
- `input`: a file under `workloads/`.
- `items`: the requests today's command sends for this workload, one per item, worked out by hand from the specification.
- `floor`: the fewest requests that can carry the workload once the function batches.
- `list`: present only while the function is still to batch. It holds `ticket`, the batching design's label, and `number`, the ticket number once that ticket has one, or `null`.

| Function | Workload | `items` | `floor` | List entry |
| --- | --- | --- | --- | --- |
| `decide` | 12 lines, `--lines` | 12 | 1 | B4 |
| `filter` | 12 lines, `--threshold 0.95` | 12 | 1 | B4 |
| `rank` | 12 lines | 12 | 1 | B4 |
| `choose` | 12 lines, three options | 12 | 1 | B8 |
| `tag` | 12 lines, three labels | 12 | 1 | B9 |
| `score` | 12 lines, three levels | 12 | 1 | B9 |
| `annotate` | 12 JSON Lines records, a question set with two `on` groups | 24 | 2 | B10 |
| `recognize` | 12 short sentences, `--lines`, no relation rule | 12 | 2 | R7 |
| `find` | 12 lines | 1 | 1 | none |
| `relate` | three names, `--lines`, three bare rules | 3 | 3 | none |

`recognize`'s floor is 2: one word request and at most one `confirm` request for the whole batch, by `2026-09-26-recognize-design.md` section 7. `relate` never batches, by ADR 0048 item 7, so its floor is its relation count. `find` already sends one request. `filter` runs at `--threshold 0.95`, and the generic arm answers yes at 0.9, so it prints no rows. A count taken from printed rows would then read 0.

The 12 lines are plain ASCII with no quote mark or backslash. None is a content cut by ticket 0144's rule, the SHA-256 of the record's compact JSON with its first 8 bytes read big-endian and taken mod 4,096. The README lists each line's first 16 hex digits from `printf '"%s"' LINE | sha256sum`, as 0144's grouping README does. The sentences and the JSON Lines records get the same check. At the loopback address no byte ceiling applies (ADR 0048 item 2), so each workload fits one request.

### The gate mode

`measure.py gate [LIST]` reads `functions.jsonl`, or a list file given as its argument. It takes the command from `THINKTHEN_BIN` and the address from `THINKTHEN_BASE_URL`. For each row it:

1. Makes a private home with `tempfile.mkdtemp` and deletes only that folder afterwards.
2. Runs the command once over the row's input with `--no-cache`. It builds the command's environment from scratch: `PATH`, `HOME` set to the private home, the address, and the fixed made-up key `THINKTHEN_API_KEY=loopback-not-a-key`. It never reads the caller's key.
3. Reads `usage.total.requests_sent` from `thinkthen status --json` in the same home.
4. Writes one row to standard output: the function, `items`, the requests sent and `floor`, tab-separated.

The rules, in this order:

1. A row with a `list` entry whose `number` names a ticket in `sdlc/tickets/` with a `Status: landed` line fails: `speed: decide is listed for ticket 0141, which has landed. Remove its entry.`
2. A row with a `list` entry fails when it sends at most `floor`: `speed: find is listed for ticket B99 but sent 1 request for 12 items. Remove its entry.` It fails when it sends more than `items`, with the same numbers and `Check its workload.` in place of the last sentence. A count between the two passes, so a function that batches only part of its workload stays listed.
3. A row with no `list` entry must send at most `floor`. More fails: `speed: filter sent 12 requests for 12 items where 1 fits. Batch it, or list it with its ticket.`

A failure goes to standard error, and the mode exits 1 after every row has run. A command that exits other than 0 fails with its exit code and no echoed text. The builder pins each sentence's exact bytes in the test.

Each batching ticket removes its function's entry in the commit that makes the function batch. If it does not, rule 2 fails the gate. A ticket that removes an entry without batching fails rule 3. When a batching ticket takes a number, its writer adds the number to the entry, and rule 1 then catches a landing that left the entry behind. A ticket that changes a listed function's per-item count updates that row's `items`.

### The plan mode

`measure.py plan BENCH` first runs `cargo build --locked --package thinkthen` with `THINKTHEN_API_KEY` unset, as `probes/replay-check.sh` builds. It then prints every command the live mode would run, one line each, with the number of records and an estimate of input tokens. The estimate counts 300 tokens a request, and 0.516 tokens a byte for the evidence-size arms. It checks the live mode's preconditions below. It sends nothing. It refuses at exit 2 when `THINKTHEN_API_KEY` is set. The builder commits before running `plan`, because the clean-tree check needs a clean tree. The coordinator reads its output before asking Ian to authorize a run.

### The live mode

`sdlc/scripts/live --max-tokens 2000000 probes/speed/job.sh BENCH NAME` runs it. `BENCH` is a Beatles Bench checkout. `NAME` names the run folder `probes/speed/runs/NAME/`. Before any request the mode refuses at exit 2 when the checkout's `git status --porcelain` is not empty, when `target/debug/thinkthen` is missing or older than the commit time of `HEAD`, when `THINKTHEN_BASE_URL` is set, when `BENCH` has no `data/songs.tsv`, or when its title column does not hash to `3250fed3…`. It refuses at exit 2 when the run folder already exists.

It writes `build.json`: the checkout commit, the binary's SHA-256, `thinkthen --version`, the debug profile, the bench commit and whether the bench tree was clean, the processor count, and `/proc/loadavg` at the start and the end. It never records the key, the address's credentials, the environment or the host name.

It runs every measurement one command at a time, so no two runs share the network. Each run gets its own private home and `--no-cache`. The mode builds each command's environment from scratch, with only `PATH`, the private `HOME` and the key the door gave it. So every request goes to the built-in address. It passes no proxy variable, so a run needs direct access to the service. The connection-count issue names this. For each run it writes one row to `rows.jsonl`: the measurement, the arm, the repeat, the records, `requests_sent`, `input_tokens` and `output_tokens` from `status --json`, the wall seconds from `time.monotonic()` around the command, the lines printed, and the exit code. It stores no output text. On a failed command it stores the exit code and nothing from standard error.

It stops before starting a command once the input tokens it has counted reach 1,800,000, and prints how far it got. When a command reports no tokens, the count adds that command's estimate from the plan. The door's charge of 2,000,000 covers the one command that may cross.

The arms:

| Measurement | Runs | Arms |
| --- | --- | --- |
| **The target.** `filter 'The text is the title of a song by the Beatles. It appears on the album Abbey Road.' --threshold 0.7` over the 306 titles, default throttle | 3 | The default. `--batch 1` when `filter --help` names `--batch` |
| **Throttle check.** The same filter at `--jobs 16` | 1 | Today's requests: the default before B4, `--batch 1` after |
| **Every function.** Each `functions.jsonl` row over its workload | 1 | The default, and `--batch 1` when the verb's help names `--batch` |
| **Recognize by step.** `recognize` over the 12 sentences with one relation rule, and with `--boundary run` when `recognize --help` names `--boundary` | 1 | As above. The report gives word requests from the plain row, relation requests as the difference with the relation row, and `confirm` requests as the difference with the `--boundary run` row |
| **Annotate by field.** The question set's two groups, each as its own one-group set, beside the two-group row | 1 | As above |
| **Beatles Bench jobs.** Each case file matching `functions/*/*-cold.jsonl`, `functions/*/*-context.jsonl` or `functions/decide/decide-love.jsonl` in the bench, as one command over all its records with the file's shared arguments, run in the case folder. `audit`'s two case files send requests and are measured. `diff` asks nothing and is listed with no row | 1 | As above |
| **Beatles Bench questions.** The 1,501 questions in the bench's `questions/*.jsonl`, as one `--jsonl --field /input` command per group of one function and one question, with `--options /options` for `choose`, as `ask.py` builds each call | 1 | As above |
| **Evidence size.** `decide` over one record at 18, 8,000, 24,000, 40,000 and 56,000 bytes of evidence. The evidence is the bench catalog's lines, repeated with a numbered header where it runs out. 56,000 bytes is the most that stays under the service's 32,000-token evidence limit at the worst measured 0.516 tokens a byte | 3 each | Today's single request |
| **Context.** The 306 titles with the bench catalog as `--context`, and with a context padded to 56,000 bytes, so the one request nears the 96,000-byte ceiling with its 306 quoted questions | 3 each | Only when `filter --help` names `--context` |

At the end the mode prints the report: one Markdown table per measurement with requests, the median and range of wall seconds, input and output tokens, and dollars at the recorded input price of $0.042 a million tokens. The target table says `met` when the median of the three default runs is under 0.5 s, and `not met` otherwise, and gives all three times. The builder copies nothing into a page. The run's record copies the tables.

Help probing decides the optional arms. So a later build that gains `--batch`, `--context` or `--boundary` measures them with no edit to this job. Each such run still needs Ian's authorization.

### The Rust test

`crates/thinkthen/tests/speed.rs` carries `#![cfg(feature = "cli")]`. It starts the in-process conformance backend and runs `python3 probes/speed/measure.py gate` against its `/generic/v1` base. It clears the environment and passes only `PATH`, `THINKTHEN_BIN` as the compiled binary, and `THINKTHEN_BASE_URL`. `crates/thinkthen/tests/demo_runner.rs` already runs a repository script from a test in this way.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **One runner serves the gate and the live part.** The same table, the same command lines and the same count feed both. A second runner would drift from the first.
2. **The count comes from `status --json` in a private home.** It counts every HTTP attempt on every function, including `filter`, whose `--details` rows cover only kept records. It needs no proxy and never touches the key. The gate proves the count against the socket on every run.
3. **The gate runs from the `test` rung as a Rust test.** Ticket 0128 opens `sdlc/scripts`, so no rung script changes. The test rung already runs every Rust test.
4. **Workloads of 12 records.** They fit one request at the loopback, so the floor is exact. They keep the gate fast.
5. **The list lives in the table as each row's `list` entry.** One file names each function, its workload, and whether it still waits, with its ticket. A batching ticket edits one line.
6. **A listed function fails at or below `floor` or above `items`.** This catches a ticket that batches and forgets its entry, and it lets partial batching stay listed.
7. **The target is met on the median of three runs under 0.5 s.** One run can hit a slow network moment. The report prints all three.
8. **The named build is the live door's debug binary at a clean commit.** The door runs `target/debug/thinkthen`. `build.json` names the commit and the binary's hash. A release build would be faster, so the debug figure is conservative.
9. **Optional arms follow the command's help.** The job measures `--batch 1`, `--context` and `--boundary run` as soon as the build has them. No later ticket edits the job for them.
10. **Before `--context` exists, S1 measures evidence size with today's single request.** The context-time gap is the time of a large request, up to the ceiling. One large record measures it on today's build. The batched context arms join once B7 lands.
11. **Beatles Bench jobs run as one command per case file, and its questions as one command per function and question.** Run as the bench runs them, one command a case, no job could batch. The grouping is the fewest commands the bench's own arguments allow.
12. **The live part reads Beatles Bench from a checkout the caller names.** This repository copies none of its data. The run records the bench commit.
13. **S1 lands when the gate passes and the code review accepts.** No live run is needed to land. The baseline run, "S1 live run 1", happens on a main commit after S1 lands and before B4 builds. B4's ticket takes that run as a precondition. The coordinator asks Ian to authorize it by name when it is due. The target reads `not met` on that run, because nothing batches yet. B4 owns the run that measures the target after B4. The coordinator ruled this on 2026-09-26, and Ian can overturn it.
14. **The gate uses a fixed made-up key.** A command may want a key at any address. The gate passes `loopback-not-a-key`, which only the loopback backend ever sees. It never reads the caller's key.

## Edge cases

| Case | Expected |
| --- | --- |
| The committed table on today's build | Gate exit 0. Eight listed rows send `items`. `find` sends 1, and `relate` sends 3 |
| A listed function that now batches | Rule 2 fails, naming the function, its ticket, and its counts |
| A function taken off the list that still sends one per item | Rule 3 fails, naming the function and the floor |
| A listed entry whose numbered ticket has landed | Rule 1 fails, whatever the function sends |
| A listed entry with `number` null | Rule 1 does not apply. Rules 2 and 3 still do |
| A command that exits 4 at the loopback | The gate fails that row with its exit code and no echoed text |
| The usage writer loses counts | The requests `status` reports fall short of the socket count, and the Rust test fails |
| A key in the caller's environment | The Rust test clears it. The gate mode never reads it and passes each command only `loopback-not-a-key` |
| `plan` with `THINKTHEN_API_KEY` set | Exit 2 before the build |
| `live` with `THINKTHEN_BASE_URL` set | Exit 2 before any request |
| `live` with a binary older than `HEAD`'s commit time | Exit 2 before any request |
| A listed function that batches part of its workload | It sends between `floor` and `items`, and it passes while it stays listed |
| `plan` or `live` with a dirty checkout | Exit 2 before any request |
| `live` with a bench whose titles differ | Exit 2 before any request, naming the file and the expected hash |
| `live` into a run folder that exists | Exit 2 before any request |
| `live` reaches 1,800,000 counted input tokens | It starts no further command and says how far it got |
| A build before B4 | No `--batch` in help. Only the default arm runs. The target reads `not met` |
| A build after B4 | `decide`, `filter` and `rank` gain the `--batch 1` arm. Other functions follow their tickets |
| A build without `--context` | The context arms are skipped and the report says so |
| The service refuses one evidence size | Its row keeps the exit code, and the mode goes on to the next command |
| The bench's `diff` folder | Listed with no request |

## Proof

One outside-in test in `crates/thinkthen/tests/speed.rs`. It drives the compiled binary through the real runner against the in-process loopback backend. Its rows run the committed table and three planted table copies written to a temporary folder.

| Row | Expected | Planted fault that turns it red |
| --- | --- | --- |
| The committed table | Exit 0 and empty standard error. The requests the runner reports sum to the backend's own count, 112 on today's build | (a) The runner counts `--details` rows, one per printed row, in place of `status`: `filter` at `--threshold 0.95` prints no row, so the sum falls short of the socket count by 12. (b) The runner runs every row in one shared home: `status` totals pile up across rows, the listed rows after the first report more than `items`, and the gate exits 1 |
| `filter`'s entry removed | Exit 1, and standard error is exactly the rule 3 sentence for `filter` | (c) Rule 3 compares with `items` in place of `floor`: the row passes |
| `find` listed under `B99`, with its `items` set to 12 in the planted copy | Exit 1, and standard error is exactly the rule 2 sentence for `find` | (d) Rule 2 fails only when a listed function sends more than `items`: the row passes |
| `decide`'s entry numbered `0141` | Exit 1, and standard error is exactly the rule 1 sentence for `decide` | (e) Rule 1 reads the ticket's first line in place of its `Status:` line: the row passes |

The four questions:

- **What behavior does it protect?** Ian's ruling 5: a function that can batch does not send one request per record, and the list of functions still to batch stays true as each batching ticket lands.
- **What credible regression fails it?** A batching ticket that lands without removing its entry. A change that sends one request per record again from a function that batched. An entry left for a ticket that landed. A usage count that falls short of the requests sent, which would make the live figures wrong.
- **Why does no existing test catch it?** Each batching ticket pins its own function's grouping, in the core for 0144 and on the command for B4. No test counts every function's requests, and none checks the list.
- **Does it need a test-only export, flag, or hook?** No. It runs the compiled binary and the committed runner, counts at the loopback socket and in `status`, and reads the committed table and tickets. The made-up key is an ordinary value of the real variable.

The live part is not a test. The builder checks the `plan` output with the key unset. The record of "S1 live run 1", made after S1 lands, names the build.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `crates/thinkthen/tests/speed.rs`: at most 90.
- `probes/speed/measure.py`: at most 260.
- `probes/speed/job.sh`: at most 6.
- `probes/speed/functions.jsonl`: 10 rows.
- `probes/speed/workloads/`: at most 50 lines across its files.
- `probes/speed/README.md`: at most 70.
- `probes/README.md`: one table row and at most two lines of prose.
- `sdlc/issues/`: one new issue of at most 25 lines, and one added line in the S1 row's section of the batching design.
- `sdlc/ratchet.json` moves to the measured total, at most 90 above today. The commit says what grew.
- No dependency. No product source, page, setting or public surface changes, so the `surfaces` rung is not required.
- "S1 live run 1", after landing: at most 2,000,000 tokens charged, about $0.08 at the recorded price. The plan's estimate is about 1,200,000.

## Stop rules

1. Stop before crossing a budget by more than a tenth, or adding a dependency.
2. Stop if any plant stays green.
3. Stop if today's build sends other than `items` for any listed row, or other than 1 for `find` or 3 for `relate`. The design's premise is then wrong for that function.
4. Stop if the requests `status` reports differ from the socket count on today's build. File the defect against ticket 0141's writer in `sdlc/issues/` and report it.
5. Stop if the change needs a file that ticket 0143, 0144 or 0128 opens, or any file in `sdlc/scripts/`.
6. Stop if the gate needs a network call, a real key or an engine.
7. Never run `job.sh`, the live mode or `sdlc/scripts/live`. Run `plan` only with `THINKTHEN_API_KEY` unset. Stop and hand back when the gate passes and the code review accepts. The live run happens only as a separate run that Ian authorizes by name, after S1 lands.
8. Stop if the live mode would record any header, key, environment line or output text.

## Scope and exclusions

Excluded: batching any function (B4, B8, B9, B10, R7). The `--batch`, `--context`, `--facts` and `--boundary` options. The accuracy cost, which B6 measures. Library and SQL surfaces (B12a to B13e). The documentation page D1. Changes to Beatles Bench. `site/`.

## Routing

Builder: Claude (Opus subagent) in lane 2. Reviewer: a fresh read-only Claude session for design and for code. The change raises the ceiling, so the code review names what it checked. The live run: after S1 lands and before B4 builds, the coordinator brings Ian the `plan` output and the charge, and asks him to authorize "S1 live run 1" by name.

## Complexity

Contract 1; state and timing 1; reach 1; proof 1; cost of error 1; total 5. Final level: 2. The risks are a gate that counts the wrong thing, which the socket check guards, and a live run that spends more than planned, which the plan, the job's own stop and the door's charge guard.

## Deferred gaps

1. The gate checks no `--batch 1` arm. B4's test 1 already pins that a batch of one is today's request. The live part measures the arm.
2. The target reads `not met` on "S1 live run 1", because nothing batches yet. B4 owns the authorized run that measures the target after B4. The S1 row's section of the batching design records this.
3. The context arms wait for B7, and the `confirm` step waits for recognize's R3. Each needs its own authorized run once its build exists.
4. Beatles Bench jobs run only in the live part. The repository holds no offline copy of them.
5. A new verb is not forced into the table. Nothing lists the command's verbs without copying them.
6. Wall time is the process's time, one handshake included. The service's own time per request waits for `2026-09-23-record-the-backends-own-time-for-each-call.md`.
7. Ticket 0142's deferred gap 1 asks S1's live part to count secure connections at `--jobs 16`. S1 does not measure it. S1 measures that run's time only. The new issue `sdlc/issues/2026-09-26-count-secure-connections-at-sixteen-jobs.md` asks for the count through a local CONNECT proxy.
8. List entries carry no ticket number until each batching ticket is numbered.
9. The bench's `questions/functions/*.jsonl`, the function suite that `functions.py` runs by default, is not measured. Its cases overlap the `functions/` folders. A later run can add it.

## What Ian can overturn

- The coordinator's order: S1 builds after 0143 and 0144 land.
- Decision 7: the target is met on the median of three runs.
- Decision 8: the debug binary at a clean commit as the named build.
- Decision 10: evidence size on today's single request stands in for context time until B7.
- Decision 11: the bench's jobs grouped into one command per case file and per question group.
- Decision 13, the coordinator's ruling: S1 lands on the gate and the review, and "S1 live run 1" follows on main before B4 builds, as B4's precondition.
- The live charge of 2,000,000 tokens and the job's own stop at 1,800,000.

## Closes

No issue. `sdlc/issues/2026-09-26-batching-design.md` stays open until its last ticket lands.

## Evidence

- Starts from: The S1 row and section, and rulings 4, 5 and 9, of `sdlc/issues/2026-09-26-batching-design.md`. ADR 0048 items 2, 7 and 12. ADR 0049, which makes the usage totals trustworthy once the command exits. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` sections 6, 7 and 9: experiment 268's round trips and its unmeasured context sizes above 2,585 tokens, experiment 271's 0.30 to 0.39 s for one request of 306 titles, and the 55,490-byte body with no content cut. Ticket 0142's deferred gap 1. Ticket 0143's relate under `--jobs` and ticket 0144's planner, content-cut rule and ceiling, read from their branches. Beatles Bench at `7d246844`: its runners send one command per case, its case files share one argument set each, and its 1,501 questions form 140 groups. The code at `origin/main` `7850db3f`.
- Keeps: Every product source file, page, setting and surface. Every existing test. The live door and its ledger, untouched. Beatles Bench, read only.
- Changes: A new `probes/speed/` folder with the function table, the gate workloads, the runner and the live job. A new Rust test that runs the gate against the loopback backend. One row in `probes/README.md`. One new issue for the connection count. One line in the batching design's S1 section naming B4 as owner of the target run after B4.
- Proof: The four-row test under "Proof" with plants (a) to (e). The `plan` output checked with the key unset. The record of "S1 live run 1", which names the build. That run follows landing on a main commit and is B4's precondition.
- Defers: The `--batch 1` arm in the gate. The target on a batching build, which B4 owns. The context and `confirm` arms. Offline bench jobs. The bench's `questions/functions/` suite. Verb completeness. Per-request service time. Connection counts at `--jobs 16`, filed as their own issue. Ticket numbers on list entries.
