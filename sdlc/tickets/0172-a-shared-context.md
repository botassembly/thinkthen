---
flow: build
priority: 172
opens: sdlc/planning/adr sdlc/planning/adr/0048-records-batch-into-full-requests.md crates/thinkthen/src/cli/args.rs crates/thinkthen/src/cli/edge.rs crates/thinkthen/src/cli/asking.rs crates/thinkthen/src/cli/asking/batched.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/cli/failure crates/thinkthen/src/core/batch.rs crates/thinkthen/src/core/batch/tests.rs crates/thinkthen/src/core/result.rs crates/thinkthen/src/result_json.rs crates/thinkthen/tests/backend/batching.rs crates/thinkthen/tests/backend/batching probes/context probes/README.md specification/records.md specification/result.md specification/channels.md specification/backends.md specification/decide.md specification/filter.md specification/rank.md specification/settings.md specification/roadmap.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0172: A shared context

Status: product code, measured growth and helper/prose corrections accepted by fresh High review; the named 0170–0172 local checkpoint and authorized paid proof passed. Awaiting coordinator landing. The coordinator accepted the design on 2026-09-27 after a fresh read-only review. Owner: Codex.

Review route: the builder follows the work plan: Claude now, or Codex after the handover. A fresh read-only session from the builder's vendor reviews the final diff.

## Outcome and authority

A user has `songs.txt`, 306 titles, and `catalog.txt`, one line of facts per song. They run:

```sh
thinkthen filter 'It appears on the album Abbey Road.' --threshold 0.7 --context catalog.txt < songs.txt
```

The tool sends one request. The catalog is its evidence, sent once, and each title rides in its own quoted question. In experiment 271 a similar form scored 299 to 302 right of 306 with 2 or 3 false yeses. The default without a context scored 283 to 287 right, by ADR 0055. A context too large to send is refused before any request when the tool can tell, and at the record that makes it too large when it cannot.

This is batching row B7 of `sdlc/issues/2026-09-26-batching-design.md`: "`--context FILE` for `decide`, `filter` and `rank`; exit 2 for a context over the ceiling; `meta.context_sha256`. B7 picks the late-overflow policy of ticket 0144's deferred gap 5 and rewrites ADR 0048 item 11 to match." Proof: design tests 2 and 10. It is Batch D item 6 of `sdlc/planning/work-plan-2026-09-27.md`.

Ian's rulings set the frame. Ian can overturn each.

- Speed wins over accuracy. The context form keeps one request for the 306 titles.
- Simple beats clever. The tool does not read ahead of a stream to check sizes.
- Paid runs are authorized with a token cap and a cost estimate. This ticket carries one.

## Prior experiment evidence

- Local experiment 284, file 18: ADR 0048 item 11 promises exit 2 "before any request" for a context too large. That cannot hold once a later record, or a run-time halving, finds the overflow after earlier requests went out. It asks B7 to pick the policy and rewrite item 11.
- Ticket 0144, deferred gap 5: `Batcher::push` returns `BatchError::ContextOverLimit` for a record whose batch of one with the context passes a limit. B7 chooses: check the first record before any request, or fail at that record.
- Experiment 271, copied in `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` sections 7 and 9: the catalog as context scored 299 to 302 of 306 in one request of 62,748 bytes and 22,091 input tokens, in 0.36 s.
- ADR 0055 item 2: a context changes nothing in the batch form. Item 5: a context stays shared evidence for every record of its batch.
- Ticket 0154 makes the request size a setting at every address. Its deferred gap 10 asks B7's refusal to name `--max-request-bytes`.

## Which findings hold

Checked on `origin/main` `40783433`.

| Finding | Holds? | Evidence | This ticket |
| --- | --- | --- | --- |
| File 18, the promise cannot hold | Yes | `core/batch.rs:137-148` checks the context with no record in `Batcher::new`. `context_fits` at line 308 checks each record only when `push` reaches it. After ticket 0154, a batch the backend refuses as too large is halved once after it was sent. Both happen after earlier requests may have gone | A new ADR rewrites item 11 to the promise the tool can keep. The command fails at the record |

## What happens today

Read from `origin/main` `40783433`, and tickets 0146 and 0154 on their branches.

- `core/batch.rs::Batcher::new` takes an optional context as `Evidence`. It refuses a structured question beside a context and a context whose request with no record passes a limit. `BatchError::ContextOverLimit` carries the limit's kind, the limit and the size, and no text. The fixture `batch-context` pins a context batch's bytes.
- Ticket 0146 passes no context. Its reader turns a refusal from `push` into `Input::Failed` after it queues every batch that closed first. `decide` prints earlier completed rows, `filter` prints earlier kept rows, and `rank` withholds rows on a stop. The run stops with today's two lines.
- In `Batcher::push`, `context_fits` checks the new record before the open batch closes as `Limit`. So a record that cannot fit beside the context is refused while the records before it still sit in the open batch. The refusal then empties that batch, so those rows would never be sent; `decide` and `filter` would also lose their printed prefix. This ticket fixes the order in the core: when the record does not fit, `push` first closes the open batch into `closed`, then calls `context_fits` on the record alone.
- Ticket 0154 applies the request size at every address, so a context refusal reaches every address.
- After tickets 0170 and 0171, `meta` carries `batch` and `batch_warning`. `result.md` line 110 marks `context_sha256` as not built.

## Retained behavior

- Every run without `--context` is unchanged.
- A context never enters the question digest. `meta.question_sha256` stays the same with or without it.
- The batch form of ADR 0055 is unchanged. With a context, the evidence is the context, and each record appears once in its quoted question.
- `choose`, `tag`, `score` and `annotate` take no `--context` until B8 and later.

## The change

### The option

`--context FILE` joins ticket 0146's `Batching` struct, so `decide`, `filter` and `rank` take it. It sits in the long help only. The edge reads the file's bytes once, before the batcher starts, and passes them as `Evidence`. No environment variable and no question-file key sets it, by design section 6.

- With `--batch 1`, each request carries the context and one quoted record.
- Every row under `--details` carries `meta.context_sha256`, the SHA-256 of the file's bytes, after `batch_warning`. Every row of a context run carries `meta.batch`, even a batch of one, by ADR 0048 item 9.
- Record-mode `--dry-run` plans the first batch with the context as its evidence.
- The context is evidence. It leaves the machine. It enters the request digest, so a changed context misses the cache and a replay.

### Refusals

Each sentence is pinned in a test. None echoes the context or a record.

| When | Exit | Message after `thinkthen: ` |
| --- | --- | --- |
| On one document | 2 | `--context shares one text across the records of a stream, and a single text is one record` |
| The file cannot be opened | 5 | `--context could not be opened: ERROR` |
| The file is not UTF-8 | 5 | `--context is not UTF-8 text` |
| The file is empty | 2 | `--context names an empty file` |
| A question written as JSON | 2 | `--context needs a question written as text; a question written as JSON cannot quote a record` |
| The context and the question pass the request size, before any record | 2 | `--context: the context and the question make a request of A bytes, over the request size of L bytes; raise --max-request-bytes or shorten the context` |
| The same, over a profile limit, including a profile `max_request_bytes` below the request size | 2 | `--context: profile NAME allows at most L WORDS; the context's request has A` |
| A record and the context pass the request size | 2 | `--context: this record and the context make a request of A bytes, over the request size of L bytes; raise --max-request-bytes, or shorten the context or the record` |
| The same, over a profile limit | 2 | `--context: profile NAME allows at most L WORDS; this record and the context make A` |

The first five refusals and the two "before any record" rows happen before any request. A record's refusal takes 0146's refusal path: every batch that closed before it is sent. `decide` prints earlier completed rows, `filter` prints earlier kept rows, and `rank` withholds them on a stop. The run then stops at that record with the cause line and today's stop line, at exit 2. That record sends nothing.

### The ADR

ADR 0048 says a change to one of its items takes a new ADR. The build writes one, numbered with the next free number in the builder's range (up to 0079 for Claude, 0080 to 0099 for Codex), that rewrites item 11:

1. Before any request, the run refuses at exit 2 a context whose request with the question and no record passes the request size or a profile limit.
2. A record whose batch of one with the context passes a limit stops the run at that record, at exit 2. That record sends nothing. Batches before it may already have been sent. `decide` and `filter` print completed rows; `rank` withholds them on a stop.
3. A batch the backend refuses as too large follows ADR 0051's one halving. A half still refused fails at exit 4, as any batch does.
4. The tool does not read ahead of a stream to check sizes. A stream can be endless, a live pipe waits on the 50 ms pause, and ADR 0053 bounds the memory a run holds.

ADR 0048 item 11 gains `(Amended by ADR NNNN.)`. The batching design issue's edge row for an oversized context changes to match in the same commit.

### Pages

- `records.md`: `--context` under "Order and requests", the late-record rule, and the trust sentence: a context is evidence for every record of its batch.
- `result.md`: the `context_sha256` row loses its marker.
- `channels.md` line 32: `--context FILE` joins the advanced options.
- `backends.md`: the context refusal at every address, by the request size.
- `decide.md`, `filter.md` and `rank.md`: one example each, and the measured numbers from this ticket's live record.
- `settings.md`: `context` moves from "Settings on the way" into the table. Its library and SQL cells name B12a and B13a to B13e.
- `roadmap.md` line 43: `--context FILE` is built.

After this ticket, `grep -rn "Not built yet, by ADR 0048 item 11" specification` returns nothing.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **A record that cannot fit beside the context fails at that record.** Checking only the first record would leave the promise false for the second. Reading the whole stream first breaks live pipes and the memory bound. Failing at the record matches how 0146 already treats a record over a profile limit.
2. **The late refusal exits 2.** The exit table says 2 means the failing record sent nothing, and that holds.
3. **The promise names what the tool can count.** Bytes are known before a send. The backend's tokens are not, so a token refusal after sending stays with ADR 0051's halving.
4. **An empty context is refused.** An empty document is a usage error today, because a judgment about nothing is a mistake.
5. **`--context` sits on the three batching verbs only.** B8 adds it to `choose`. Putting it on `Common` would need refusals for verbs that cannot use it.

## Edge cases

| Input | Expected |
| --- | --- |
| The `batch-context` fixture's records and context, default, loopback | One request. Its body equals `specification/fixtures/systemone/batch-context.request.json` without its trailing newline |
| The same at `--batch 1` | One request per record. Each body carries the context and one quoted question |
| `--details` on a context run | Every row carries `meta.context_sha256` and `meta.batch`, even at `--batch 1` |
| Two runs with two different contexts | The same `meta.question_sha256`. Different `meta.requests` |
| `--replay` of a context run with another context | Exit 5 at the first batch |
| `--context` on one document | Exit 2, the pinned sentence. No request |
| A missing file, a non-UTF-8 file, an empty file | Exit 5, 5 and 2, the pinned sentences. No request |
| A JSON question with `--context` | Exit 2, the pinned sentence. No request |
| A context over the request size with the question alone | Exit 2 before any request. The sentence names `--max-request-bytes` |
| A context over a profile's `max_evidence_bytes` | Exit 2 before any request. The sentence names the profile |
| A profile whose `max_request_bytes` is below the request size, and a context over the profile's limit with the question alone | Exit 2 before any request. The profile sentence, not the `--max-request-bytes` one |
| `decide` records 1 and 2 fit, record 3 overflows beside the context, default batch | One request of 2 records. Rows 1 and 2 print. The cause line, then `stopped at record 3; 2 records finished`. Exit 2 |
| Record 1 overflows beside the context | No request. Exit 2 at record 1 |
| `--dry-run --context` over three lines | The plan of one batch with the context as its evidence |
| A 413 on a `decide` context batch of 4, halves answer | Three requests, each with the context. Rows 1 to 4 print |

## Tests and proof

Every command test drives the compiled binary against the in-process loopback. New tests live in ticket 0146's `tests/backend/batching` folder.

| Test | What it proves | Deliberate break that turns it red |
| --- | --- | --- |
| `a_context_rides_every_batch`, design test 2 at the command | The fixture, `--batch 1`, `--details`, `--dry-run` and 413 halving rows. The loopback's body equals the fixture. `meta.context_sha256` equals the pinned hex of the fixture context's bytes. The dry-run plan's evidence is the context. Each of the three requests of the halving row carries the context | (a) List the records in the evidence beside the context: the body differs. (b) Send the context in the first request only: the `--batch 1` row. (c) Hash the text after a trim: the digest differs. (d) Omit `meta.batch` on a context batch of one. (e) Build the halves without the context: the halves' bodies differ. (f) Build the plan without the context: the dry-run row differs |
| `a_context_stays_out_of_the_question_digest` | The two-context and replay rows | (a) Add the context to the question digest: `meta.question_sha256` differs. (b) Leave the context out of the request body: the replay under another context answers |
| `a_context_that_cannot_fit_is_refused` | Every refusal row of the edge table. Each pins the loopback's request count, standard output, the whole standard error and the exit code. The context and every record hold a marker string, and no standard error line contains it | (a) Check only the first record: the record-3 row sends a request with record 3. (b) Exit 4 for the late refusal. (c) Drop the batches before the refused record, as `push` does today: the record-3 row prints nothing. (d) Echo the context in a message: the marker appears. (e) Name `--max-request-bytes` when a profile's limit bound: the profile row |
| Design test 10, live, recorded in `sdlc/records/` | Three runs of `decide --lines --details --threshold 0.7 --context CATALOG` over the 306 titles, graded by `thinkthen audit` against the 18-row Abbey Road key. Each run sends one request. It reports right answers, false yeses, misses, input and output tokens for each run | Not a gate. See "The paid run" |

The four questions:

- **What behavior does each protect?** The context's wire bytes, its identity in the digests, and every refusal with its promise.
- **What credible regression fails each?** The breaks above: a context sent once per run, a context in the question digest, a late overflow that sends the record anyway, or a message that leaks the context.
- **Why does no existing test catch them?** The planner's fixture test checks the planner's bytes. Nothing on the command reads `--context` today.
- **Does any need a test-only hook?** No. The loopback counts requests and bodies.

### The paid run

The builder runs it after the loopback tests pass and before landing, through `sdlc/scripts/live`.

- **Where.** A new unnumbered folder, `probes/context/`, like `probes/speed/`. Its `job.sh` starts with `#!/bin/sh` and calls a short Python helper. The helper reuses `probes/speed/measure.py`'s `check` for the titles and `catalog_text` for the catalog. It reuses `plan()`'s build step, or refuses unless the binary is newer than the last commit to the command's sources, and it refuses a dirty checkout, as S1's job does.
- **Question.** `It appears on the album Abbey Road.`, at `--threshold 0.7`, through `decide --lines --details --no-cache --context CATALOG`.
- **Key.** The rows of the bench's `data/songs.tsv` whose `first_album` is Abbey Road. There are 18. The helper refuses any other count.
- **What it keeps.** The detailed rows go only to a scratch file, which the helper grades with `thinkthen audit` and then deletes. The record keeps counts only: right answers, false yeses, misses, input and output tokens, and requests for each run. No output text, no standard error, no key.
- **Command.** `sdlc/scripts/live --max-tokens 150000 probes/context/job.sh BENCH NAME`.
- **Cost.** Three requests of about 22,100 input tokens each, by experiment 271: about 66,300 input tokens. At the recorded price of $0.042 a million input tokens, that is about $0.003. Output tokens are free under the vendor's price list. The 150,000-token value is an authorization reservation for input and output, not a hard limit inside a request.
- **Stop rule.** Before each new request, the helper stops if earlier requests reported at least 120,000 input and output tokens combined. It refuses missing usage instead of estimating it. A single request can cross that threshold, so the reservation must not be described as a runtime cap.
- **Use.** The record names the build. The three pages quote its numbers. If any run scores under 297 right, landing stops. The builder hands the scores back to the queue owner, because this wording differs from experiment 271's and the pages would claim a number the tool does not reach.

## Budgets and ratchet estimate

Nonblank lines, measured with `grep -c .`, net against main after ticket 0171 lands.

- `core/batch.rs`: at most 6 net, to close the open batch before the context check. `core/batch/tests.rs`: at most 12 net, one planner row for it.
- `cli/args.rs`: at most 10 net.
- `cli/edge.rs`: at most 20 net, for reading the file.
- `cli/asking.rs` and `cli/asking/batched.rs`: at most 25 net together.
- `cli/failure.rs` and its folder: at most 40 net, for the refusals.
- `core/result.rs` and `result_json.rs`: at most 15 net together.
- Product code: at most 116 net.
- Tests: at most 232 net.
- `sdlc/ratchet.json` moves to the measured total, at most 348 above main after 0171 lands. The commit says what grew.
- `probes/context/`: a job of at most 10 lines and a helper of at most 90. They sit outside the ratchet.
- Pages: at most 45 net lines. One ADR of at most 45 lines.
- No dependency. One paid job under a 150,000-token reservation, with its next-request stop rule above.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if a run without `--context` changes one byte.
3. Stop if the context reaches the question digest, a message, a `Debug` line or a probe row.
4. Stop if any break stays green.
5. Stop if tickets 0154, 0170 or 0171 have not landed on main.
6. Stop if a paid run scores under 297 right, and hand the scores to the queue owner. Do not start another run once reported input plus output tokens reach 120,000; refuse missing usage.
7. Stop if the late-overflow rule needs read-ahead, a change to 0146's reader protocol, or more than the reordering in `Batcher::push`.

## Build order

1. Ticket 0154 lands first. It makes the request size a setting at every address, which the refusals name.
2. Tickets 0170 (B5) and 0171 (B16) land first, in that order. All three edit `meta`, `result_json.rs` and `cli/asking/batched.rs`.
3. B8 (`choose` batches) builds after this ticket and adds `--context` to `choose`.

## Scope and exclusions

Excluded: `--context` on `choose` (B8), `tag`, `score` and `annotate`. Library and SQL context (B12a to B13e). A context from an environment variable or a question file. Measuring context time at large sizes, which S1 owns. The batching page (D1). `site/`.

## Routing

Builder: the agent the work plan names, Claude now or Codex after the handover. Reviewer: a fresh read-only session from the builder's vendor for the code. The change raises the ceiling and widens a public surface, so the code review names what it checked.

## Complexity

Contract 2; state and timing 1; reach 1; proof 2; cost of error 2; total 8. Final level: 2. The risks are a context that leaks into a message, a late overflow that sends anyway, and a context left out of the request digest. The refusal and digest tests guard them.

## Deferred gaps

1. `choose` takes `--context` in B8. The other verbs follow their batching tickets.
2. Context time above 2,585 tokens is measured by S1 once `--context` exists. No page claims a context-time figure before that.
3. A context the backend refuses on tokens, under the byte size, is found only after sending. ADR 0051's halving handles it.
4. The libraries and SQL surfaces take a per-call `context` from B12a and B13a to B13e.

## What Ian can overturn

- Decision 1: fail at the record, not read ahead or check the first record only.
- Decision 2: exit 2 for the late refusal.
- Decision 4: an empty context is refused.
- Decision 5: `--context` on the three batching verbs only.
- The ADR's rewrite of item 11.

## Closes

Local experiment 284 file 18, recorded as settled in the landing commit. Ticket 0144's deferred gap 5. The batching design issue stays open.

## Evidence

- Starts from: Local experiment 284 file 18, checked on `origin/main` `40783433`: `core/batch.rs` `Batcher::new`, `context_fits` and `BatchError::ContextOverLimit`, and `result.md` line 110. Ticket 0144's deferred gap 5. Experiment 271's context arm in evidence sections 7 and 9: 299 to 302 of 306 in one request of 22,091 input tokens. ADR 0048 items 9 and 11, ADR 0051, ADR 0055 items 2 and 5. Tickets 0146 and 0154 on their branches, and tickets 0170 and 0171.
- Keeps: Every run without `--context`. The question digest. ADR 0055's batch form. The verbs that take no context.
- Changes: `decide`, `filter` and `rank` take `--context FILE`. The context is each batch's evidence, and every row carries `meta.context_sha256`. A context too large with the question alone is refused before any request. A record too large beside the context stops the run at that record at exit 2, after earlier batches send. `decide` and `filter` print finished rows; `rank` withholds rows on a stop to preserve ordering. `Batcher::push` closes the open batch before its context check. ADR 0087 rewrites ADR 0048 item 11. Nine specification pages change.
- Proof: Focused outside-in loopback cases pin the wire body, other verb plans, batch-one and 413 branches, digest identity, and refusal boundaries. The named 0170–0172 local checkpoint passed. Authorized design test 10 sent three requests under a 150,000-token reservation; each scored 306 of 306 against the 297 bar and reported complete input and output usage. The [count record](../records/2026-09-27-0172-shared-context-build.md) and [artifact](../../probes/context/runs/context-0172-43edb3dc.jsonl) pin the result.
- Defers: `--context` on the other verbs, context time at large sizes, a token refusal found after sending, and the libraries and SQL surfaces.

## Build preflight, 2026-09-27

The [shared preflight](../records/2026-09-27-batching-ticket-preflight.md) compares executable main `b7efdcc9` with pending 0154 `d10f5d22`. It preserves the accepted context scope and refusal order.

- `Batching` reaches `cli/judge.rs::Asked` and `cli/asking.rs::run` before `cli/asking/batched.rs::run` passes `None` to `Batcher::new`. Carry the file bytes across that handoff before the dry-run/live split. `cli/edge.rs` should read the file without trimming it; preserve the exact bytes for `context_sha256`. This is the 0154 missing-adapter and 0152 digest-fixture lesson.
- `core/batch.rs::Batcher::new` checks context alone; `push` currently calls `context_fits` before closing an open batch. The late-refusal proof must show an earlier batch's rows and send count survive before record 3 fails, while an early refusal sends zero. Reuse the existing `batch-context.request.json` exact-body fixture and the core three-40,000-byte, two-plus-one arithmetic from `core/batch/tests.rs:388-425`, rather than the superseded 0154 twenty-thousand-byte premise. This addresses the 0154 fixture and 0155 request-count incidents.
- `Batcher::close` hashes the final body with the address via `Exchange::digest`; `result_json.rs::decision` computes `question_sha256_with_profile` separately. The loopback proof should hold the latter constant across two contexts, change the request digest, and check each split request body. The 0154 resolved ceiling and profile limit must be compared before naming the refusal; its new command adapter is pending, so verify it on landed main before implementing this ticket. The accepted paid run remains a later builder step, under its ticket cap.

## What the build taught us

- The preflight correctly named the dry-run `Batcher::new` and 413 `halves` context handoffs, the late-close ordering, the exact-body fixture, and the second `Run` initializer in the Rust library. Those checks shaped the focused proofs before code review.
- Refusals need a typed, evidence-free cause before the existing stop path prints a message. That path and raw-byte context validation added more code than the original estimate. The coordinator authorized a measured amendment for independent growth and test-value review; the build record gives each group and retained boundary.
- A context on one record still needs `meta.batch`; a no-context batch of one keeps its old bytes. The command tests pin both paths through the shared metadata adapter. The helper that captures listener requests drains them, so a later send-count assertion uses its persistent counter.
- Ian authorized the prepared live proof through the delegated paid budget. On clean build `43edb3dc`, all three one-request runs scored 306 of 306 with zero false yeses or misses. Each reported 19,634 input and 5,710 output tokens; all 76,032 combined tokens across the three runs stayed under the helper's between-request stop threshold. The count-only artifact contains no evidence text or key. The named local 0170–0172 command checkpoint passed; the next cross-surface checkpoint belongs to the integrated database work.
- Fresh review caught a preparation transfer miss: `sdlc/scripts/live` reserves tokens before a job but cannot stop one request at that number. The helper must count reported input and output, refuse missing usage, and stop before another run once the previous total reaches its threshold. The same review found that `rank` withholds late-stop rows and that the shared result compatibility table needed the new optional digest member.
- Follow-up review found three more copies of the same broad row-output promise in the ticket and records page. The builder searched the claimed documents and qualified refusal, split, replay and interrupt wording for `rank`. Repeated prose copies caused an incomplete documentation correction, not a runtime defect. The exact synthetic helper input and output are retained under ignored `target/codex-builds/0172/proof/` for replay.
- A further read of the whole batching design issue found that its order sentence and acceptance test 8 still treated `rank` like an input-order streaming verb. The final correction also distinguished the in-order buffer from `rank`'s held records and the sum of judged shares from printed subsets. This shows why a keyword search alone did not finish the prose correction; it does not reopen the accepted batching outcome.
- Walk the full path: `Batching::context` on decide/filter/rank -> the `cli/judge.rs::Asked` adapter -> `cli/asking.rs::run` after stream-only validation -> `cli/edge.rs` exact file read -> `cli/asking/batched.rs::run` -> `Batcher::new` before the branch. The dry-run branch prints `item.batch.plan` from `planned`; the live branch uses `Former`, `engine.records`, `answered` and `result_json.rs` metadata. `core/batch.rs::close` is where every ordinary and split batch acquires its body and request digest. This catches a flag wired into parsing but lost at the plan or live handoff.
