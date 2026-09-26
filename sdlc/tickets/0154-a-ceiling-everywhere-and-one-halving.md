---
flow: build
priority: 154
opens: sdlc/planning/adr/0051-every-address-has-a-ceiling-and-a-refused-batch-halves-once.md sdlc/planning/adr/0048-records-batch-into-full-requests.md sdlc/planning/adr/0040-split-requests-under-backend-limits.md crates/thinkthen/src/core/backend.rs crates/thinkthen/src/core/batch.rs crates/thinkthen/src/core/batch/tests.rs crates/thinkthen/src/engine/prepared_request.rs crates/thinkthen/src/engine/error.rs crates/thinkthen/src/cli/args.rs crates/thinkthen/src/cli/edge.rs crates/thinkthen/src/cli/asking.rs crates/thinkthen/src/cli/asking/batched.rs crates/thinkthen/src/cli/relate.rs crates/thinkthen/src/cli/recognize.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/cli/failure/status.rs crates/thinkthen/src/core/result.rs crates/thinkthen/src/result_json.rs crates/thinkthen/tests/backend/batching crates/thinkthen/tests/backend/batching.rs crates/thinkthen/tests/backend/relate/ceiling.rs crates/thinkthen/tests/backend/status_reason.rs specification/backends.md specification/records.md specification/result.md specification/settings.md specification/decide.md specification/filter.md specification/rank.md specification/relate.md specification/recognize.md sdlc/issues/2026-09-26-batching-design.md sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0154: The request size is a setting with a default everywhere, and a refused batch halves once

Status: ready for review. Owner: Claude. It carries ADR 0051. It builds only after ticket 0146 (B4) and the batching design's ticket B5 land on main.

Review route: a fresh read-only Claude session reviews this design, ADR 0051, and later the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user points `thinkthen filter` at a local server with a smaller model and runs it over a long list. Today the default batch fills to thousands of records, the server refuses it, and the run stops at exit 4. After this ticket each request holds at most 96,000 bytes at every address. A user with a larger model raises that with `--max-request-bytes N` or `THINKTHEN_MAX_REQUEST_BYTES`, with no profile file. When a backend still refuses a batch as too large, the command splits it in half once and sends each half, and the run finishes with the split marked.

The rulings of 2026-09-26 set the scope. ADR 0048 says that changing one of its items takes a new ADR. ADR 0051 is that ADR, drafted in this ticket's branch. Ian can overturn each ruling.

1. The 96,000-byte default applies at every address. The coordinator ruled it.
2. A batch of two or more records refused as too large is split in half, and each half is sent once. A half still refused fails at exit 4. The split shows in `meta.batch`. Replay and recording stay deterministic. The coordinator ruled it.
3. The request size in bytes becomes a normal setting with a flag, an environment variable and a documented default, and follows the settings precedence. Ian ruled it.

## What happens today

Read from `origin/main` `6fbebfdf` and the branch of ticket 0146 at `6cdbe075`.

- `core/backend.rs::Backend::ceiling` returns `Some(96_000)` only when the posting URL, less `/systemone` and one trailing slash, equals the built-in base. Two callers read it. `core/batch.rs::Batcher::over` uses it as the request limit when no profile sets one. `engine/prepared_request.rs::SettledRelation::settle` splits relation plans under it when no profile limits request bytes.
- `specification/backends.md`, "Explicit profiles and local preflight", says: "A profile's `max_request_bytes` replaces it" and "Every other plan and every other address has no ceiling."
- A profile's `max_request_bytes` is a limit the backend enforces. A plan splits under it, and a request that cannot split under it, one question or one record, exits 2. The ceiling splits and never refuses: one question or one record alone goes as one request.
- No flag or environment variable sets a request size. `specification/settings.md` has a row for the backend profile and none for a request size.
- The recording folder's identity hashes the URL alone (`core/recording.rs:115`), so a value carried beside the URL does not change it.
- `engine/http.rs` reads a status 400 body up to 4 KiB. When `detail.error_type` is exactly `max_tokens_exceeded`, the error is `Error::TokenLimit`. Every other failed status is `Error::Status(code)`. Neither 400 nor 413 is retried.
- `cli/failure/status.rs` holds a phrase for 400, 401, 402, 403, 404, 422, 429 and the retried statuses. Status 413 prints the bare `the backend answered with status 413`. The `max_tokens_exceeded` sentence ends `shorten the text or set a lower max_request_bytes with --profile`.
- A failed live request writes no recording entry: `engine/request.rs::ask_prepared` cancels its write permit.
- Ticket 0146 puts batches on the command. Its worker in the new `cli/asking/batched.rs` holds each item's batch, first record number and parsed records, asks `Engine::ask_batch`, and turns a failure into one stop line naming the batch's range. Its `Completed` carries rows and an optional stop after them. Its usage shares split a batch's tokens and attempts evenly across its rows.
- B5 adds `meta.batch` under `--details`, by ADR 0048 item 9. It is absent for a batch of one record with no context.
- `crates/thinkthen/tests/backend/relate/ceiling.rs` pins that relation plans split at the built-in address and nowhere else, and that a profile of 200,000 bytes replaces the ceiling. `core/batch/tests.rs::the_ceiling_closes_batches_at_the_built_in_address_only` pins the same for batches.
- No committed recording under `demos/`, `probes/`, `transforms/` or `crates/thinkthen/tests/fixtures/` holds a file over 90,000 bytes, so no committed recording holds a request the new default would split.

## Design

### The setting

The request size is a whole number of bytes, at least 1. It is read in this order, by the settings page's precedence:

1. `--max-request-bytes N`, typed.
2. `THINKTHEN_MAX_REQUEST_BYTES`, read once in `cli/edge.rs` beside `THINKTHEN_BASE_URL`.
3. The default, 96,000.

It has no question-file key and no configuration-file key, because it describes the backend and not a question. The flag sits in the long help only, as `--batch` does. It goes on the verbs where it acts: `decide`, `filter` and `rank`, whose batches close at it, and `relate` and `recognize`, whose relation plans split under it. On `choose`, `tag`, `score`, `annotate` and `find` it would do nothing, so clap refuses it there as an unknown option, and the variable is ignored there. The batching tickets B8, B9 and B10 add the flag when those verbs batch.

**Refusals.** Each sentence is pinned in a test.

| Where | Value | Exit | Message |
| --- | --- | --- | --- |
| `--max-request-bytes` | `0`, `-1`, `1.5`, `lots`, empty | 2 | `--max-request-bytes takes a whole number of at least 1` |
| `THINKTHEN_MAX_REQUEST_BYTES` | the same | 2 | `THINKTHEN_MAX_REQUEST_BYTES takes a whole number of at least 1` |

On one document the flag is accepted. One record or one question always goes alone, so the value changes nothing there and needs no refusal.

**How it reaches the planners.** `Backend` gains the resolved request size, set by a new `Backend::with_request_size(n)` after `Backend::resolve`. `Backend::ceiling` returns it for every address. Both callers keep reading `backend.ceiling()`. The URL match moves into a new `Backend::is_built_in`, which only the warning reads. The library builder does not call `with_request_size`, so `Backend::resolve` defaults to 96,000 there.

**How it combines with a profile.** A request holds at most the smaller of the request size and the profile's `max_request_bytes`. The profile keeps its meaning: a request that cannot split under the profile's limit is refused. One record or one question over the request size alone still goes alone. `Batcher::over` and `SettledRelation::settle` take the smaller of the two in place of `profile.or(ceiling)`, and keep the profile's refusal separate. A profile can therefore lower the size and no longer raise it.

**Sizing it for a larger model.** ADR 0051 item 3 gives the rule, and `settings.md` repeats it with a worked example. Divide the input-token limit by 0.516 tokens a byte and keep a quarter back. A 500,000-token context gives `--max-request-bytes 726000`. A 1,000,000-token context gives about 1,453,000.

### The warning above 96,000 at the built-in address

When the backend is the built-in one and the request size is above 96,000, the command prints one line on standard error:

```text
thinkthen: warning: max_request_bytes 200000 is above the built-in address's ceiling of 96000; that backend refuses a request over 65536 input tokens
```

It prints once a run, after the backend resolves and before any request or plan prints. So it prints under `--dry-run` too and never reads a key. The run goes on.

### The settings page

`settings.md` gains one row, "Request size":

- What it does: the most bytes one request holds before the tool splits a plan or closes a batch.
- Default: 96,000. The reason sits in the cell: the hosted backend refuses over 65,536 input tokens, and at the worst measured 0.516 tokens a byte 96,000 bytes is about 49,500 tokens, a quarter under the limit (ticket 0123, ADR 0051).
- Allowed values: a whole number of at least 1.
- Command: `--max-request-bytes`. Environment: `THINKTHEN_MAX_REQUEST_BYTES`. Question file: `not on this surface`.
- Library and SQL cells: `not on this surface`, naming the request-size follow-up below. The engine applies 96,000 there.

The precedence section's list gains the order `--max-request-bytes`, then `THINKTHEN_MAX_REQUEST_BYTES`, then 96,000. The backend profile row gains one sentence: a profile's `max_request_bytes` lowers the request size and never raises it. Below the table, one short paragraph gives the sizing rule and the 500,000 and 1,000,000-token examples.

### The libraries: a follow-up, not ticket 0148

The libraries get the same setting in a follow-up ticket after 0148 and 0149 land. The coordinator numbers it. It adds `EngineBuilder::max_request_bytes`, reads `THINKTHEN_MAX_REQUEST_BYTES` in `EngineBuilder::from_env`, adds one row to 0148's binding table on every surface, one case to `conformance/settings.json`, and the SQL spellings by 0149's rules. Extending 0148 instead would break its stop rule 3, which forbids engine changes, because the builder can pass the size only once this ticket's `Backend::with_request_size` exists. 0148 is also accepted, and extending it would need a second review. Until the follow-up lands, every library applies the default of 96,000 at every address, which is this ticket's change to them.

### What counts as too large

`engine/error.rs` gains `Error::too_large()`, true for `Error::TokenLimit` and `Error::Status(413)`, at every address. Nothing else counts. `cli/failure/status.rs` gains the 413 phrase `the backend refused the request as too large; set a lower --max-request-bytes`, and the `max_tokens_exceeded` sentence ends `shorten the text or set a lower --max-request-bytes`.

### One halving

`core/batch.rs` gains a pure function `halves`. It takes the batcher's backend, profile, question and context, and the batch's records as `BatchRecord`s. It feeds the first ⌈n/2⌉ records to a fresh `Batcher` with the setting `max`, then the rest to another, and calls `finish()` on each. Each half therefore forms by ADR 0048 item 1. A half is a subset of a batch that fit, so it fits and closes once. A second batch from either half is a `BatchError::Defect`.

The worker in `cli/asking/batched.rs` asks the whole batch as ticket 0146 builds it. When the ask fails with `too_large()` and the batch holds two or more records, the worker rebuilds the records' `BatchRecord`s through `Reading::batch_record`, calls `halves`, and asks the first half, then the second. The halves run inside the item's place, so `--jobs N` still bounds requests in flight at N.

- Both halves answer: the item carries every row of the batch.
- The first half fails: the item stops at the batch's first record with the half's cause, and the second half is not sent.
- The second half fails: the item carries the first half's rows and a stop at the second half's first record.
- A half that fails as too large is never split again.

The stop line is ticket 0146's one-line form with the half's range. A half of one record keeps today's two lines, by 0146's rule for a batch of one.

### Replay

Under `--replay`, the worker treats a whole batch with no entry the way it treats a refusal: it asks the halves, from disk. When the first half has no entry, the stop names the whole batch's range, exactly as ticket 0146 prints a replay miss. When the first half answers and the second has none, the first half's rows print and the stop names the second half. A split live run then replays byte for byte, with no recording format change and no new entry kind.

Under `--cache` and the default cache, a missing whole batch goes live as today. On a refusal the halves are asked through the cache like any request.

### The mark and the counts

B5's `meta.batch` gains `"split":true` on every row a half answers, including a half of one record with no context, where ADR 0048 item 9 otherwise leaves `meta.batch` out. The half's own `records`, `position`, `usage` and `requests_sent` fill the other fields, and `closed` keeps the whole batch's reason. The refused request's attempts join the first half's `requests_sent` before ticket 0146's even shares divide it. `meta.requests` holds the half's digest.

### Pages

The build edits each page and deletes the sentence each rule replaces, in the same commit. No page gets a marker for ADR 0051.

- `backends.md`, "Explicit profiles and local preflight": the ceiling paragraph states ADR 0051 items 1 to 5, and the batch paragraph states items 6, 7 and 9. "A profile's `max_request_bytes` replaces it" becomes "the smaller of the two applies". "Every other plan and every other address has no ceiling" becomes "Every other plan has no ceiling." The status table gains the 413 row, and the 400 `max_tokens_exceeded` row takes the new ending.
- `records.md`: the failure rule for a batch names the halving, and the replay rule names the halves.
- `result.md`: the `meta.batch` row gains `split`.
- `settings.md`: the "Request size" row, the precedence line, the profile row's sentence and the sizing paragraph, as above.
- `decide.md`, `filter.md`, `rank.md`, `relate.md` and `recognize.md`: one sentence each naming `--max-request-bytes`.
- ADR 0040's ticket 0123 amendment gains the marker `(Amended by ADR 0051.)`, after ticket 0147 lands, because 0147 edits the same line.
- `sdlc/issues/2026-09-26-batching-design.md` gains one sentence in section 2's "Other addresses" and one in section 4, each pointing at ADR 0051.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **The 96,000-byte default applies at every address.** The coordinator ruled it.
2. **The request size is a setting: flag, then variable, then 96,000.** Ian ruled it. The author chose the spelling `--max-request-bytes` and `THINKTHEN_MAX_REQUEST_BYTES`, as suggested, because it matches the profile key a reader already knows.
3. **No question-file or configuration-file tier.** The size describes the backend. A question file travels between backends, and the read-only configuration file adds no tier for `batch` either.
4. **A request holds the smaller of the setting and a profile's `max_request_bytes`.** Two limits that each "replace" the other need a tie rule. The smaller one is always safe, and the profile already means "the backend enforces this". A profile of 200,000 no longer raises the size above 96,000. A caller moves that number to the setting. No release has shipped, and the rename costs one flag.
5. **Above 96,000 at the built-in address, the command warns.** A refusal would block a sound use. 96,000 bytes sits near 49,500 tokens at relate's measured 0.516 tokens a byte, and record text measured 0.40, so the hosted limit lies near 127,000 to 164,000 bytes. A caller who measured their own text may use that room. The warning keeps the risk visible.
6. **The flag goes only on the verbs where it acts.** Ian's ruling 5 of 2026-09-25 says no setting may do nothing. `choose`, `tag`, `score`, `annotate` and `find` never split under the size today.
7. **The libraries get the setting in a follow-up, not in ticket 0148.** 0148 forbids engine changes, and the size needs this ticket's `Backend::with_request_size`. The follow-up is one row per binding in 0148's table and one shared case.
8. **Status 413, and status 400 naming `max_tokens_exceeded`, count as too large at every address.** The 400 body check already runs at every address, and a server presenting the System One shape means the same by it. RFC 9110 defines 413 as a body too large for the server. Plain 400 and 422 stay out, because each can mean a malformed request, and a malformed batch fails both halves too.
9. **One halving, then exit 4.** The coordinator ruled it. A batch sends at most three requests.
10. **The first half takes the extra record of an odd batch, and the halves go one after the other.** Order stays simple, and the throttle keeps its meaning.
11. **A replay asks the halves when the whole batch has no entry.** This keeps replay deterministic with no new recording entry. A refusal is never written, so a replay cannot tell a refused batch from a missing one. Asking the halves in both cases gives the live run's rows.
12. **The refused request's attempts count in the first half.** Rows then still sum to the run, as ADR 0048 item 9 requires.
13. **`split` marks every row a half answers, even a half of one record.** Without that, a batch of two split into two single records would lose its mark.
14. **The too-large phrases name `--max-request-bytes`.** The setting is now the direct fix, and a profile can no longer raise the size.
15. **Build after ticket 0146 and B5.** The section "Order against ticket 0146" gives the costs. The coordinator can overturn it.

## Order against ticket 0146

Recommended: build 0154 after 0146 and B5 land, in that order.

- **After 0146 and B5.** The halving lives in 0146's new `cli/asking/batched.rs` and uses its `ask_batch`, stop line and shares. The mark lives in B5's `meta.batch`. The flag joins `cli/args.rs` beside 0146's `--batch`. Each file changes once 0146 is on main, so nothing collides. The cost is time. Until 0154 lands, a default batch at another address still has no ceiling, and B5 has no ticket number yet. B5 is next after B4 in the design's order, so the wait is one ticket.
- **Fold into 0146 before it starts.** 0146 and its pages would be written once, with no second pass over `batched.rs`, `args.rs` or `backends.md`. The cost is larger. 0146 is accepted and would need a second review. Its product budget of 545 lines would grow by about 225, and its test budget of 420 by about 440, on a level-3 ticket. Tickets 0148, 0150 and 0153 wait on 0146 and would wait longer. The mark still waits for B5, because 0146 excludes `meta.batch`.
- **Fallback when B5 lags.** The coordinator may start 0154 right after 0146. Then B5's ticket builds the `split` member and its `--details` row in test 3. The setting, the halving and the stop lines still land here.

Overlap with 0146's files: `core/batch.rs`, `cli/args.rs`, `cli/edge.rs`, `cli/asking.rs`, `cli/asking/batched.rs`, `cli/failure.rs`, `result_json.rs`, `core/result.rs`, `crates/thinkthen/tests`, `backends.md`, `records.md`, `result.md`, `settings.md`, `decide.md`, `filter.md`, `rank.md`, `sdlc/issues`. The overlap is safe only because 0154 builds after 0146 lands. This ticket's Phase 1 commit touches none of them. It adds ADR 0051 and markers in ADR 0048, which no in-flight ticket opens.

Overlap with other tickets:

- Ticket 0147 opens ADR 0040 and `backends.md`, and its ADR 0050 names one recognize piece "at an address with no ceiling and no profile". After 0154 that case no longer exists. 0154 edits ADR 0040 only after 0147 lands, and leaves `backends.md` line 17 to 0147. 0147 also opens `recognize.md` and `relate.md`. 0154 adds one sentence to each after 0147 lands.
- Ticket 0148 opens `settings.md`. 0154 adds a new row and touches the profile row after 0148 lands.
- Ticket 0153 opens `cli/args.rs`. Both build after 0146. Whichever builds second merges the other's `args.rs` lines.
- Ticket 0155, retries and backoff, opens `cli/args.rs`, `backends.md`, `settings.md` and ADR 0048 item 10. The two tickets touch different lines of each. 0155 builds after 0146 and 0148 and before B5, so it lands before 0154, and 0154 merges it.

## Edge cases

| Input | Expected |
| --- | --- |
| Three 20,000-byte lines, `decide`, loopback, nothing set | 2 requests: 2 records, then 1. Each body at most 96,000 bytes |
| The same with `--max-request-bytes 200000` | 1 request of 3 records. No warning |
| The same with `THINKTHEN_MAX_REQUEST_BYTES=200000` | 1 request of 3 records |
| The same with `THINKTHEN_MAX_REQUEST_BYTES=200000 --max-request-bytes 50000` | 3 requests of 1 record. The flag wins |
| The same with a profile of `max_request_bytes` 200,000 and nothing else | 2 requests. A profile no longer raises the size |
| The same with a profile of 50,000 and `--max-request-bytes 200000` | 3 requests of 1 record. The smaller wins |
| The same at `--batch 1` | Today's three requests, byte for byte |
| One 100,000-byte line, loopback, nothing set | 1 request, sent alone. Not refused locally |
| The same with a profile of 50,000 | Today's profile refusal at exit 2. The profile still refuses |
| `--max-request-bytes 0`, `-1`, `1.5`, `lots` | Exit 2, the pinned flag sentence. Nothing sent |
| `THINKTHEN_MAX_REQUEST_BYTES=0` on `decide` over records | Exit 2, the pinned variable sentence |
| `THINKTHEN_MAX_REQUEST_BYTES=0` on `choose` | Ignored. Today's requests |
| `choose --max-request-bytes 5` | Exit 2, clap's unknown-option refusal |
| Relate over the Beatles set at loopback, nothing set | Relation request counts 1 and 2, sizes 81,943, 95,779 and 76,411 bytes, as at the built-in address |
| The same with `--max-request-bytes 200000` | Counts 1 and 1, sizes 81,943 and 161,252 |
| Relate at the built-in address, nothing set | Today's plan, byte for byte |
| `--max-request-bytes 200000` at the built-in address, `--dry-run` | The pinned warning line on standard error, once. Exit 0 |
| `--max-request-bytes 96000` at the built-in address | No warning |
| `--max-request-bytes 200000` at `https://api.typesafe.ai/v1/` | The warning |
| `--max-request-bytes 200000` at loopback | No warning |
| A profile of 200,000 at the built-in address, nothing else | No warning. The size stays 96,000 |
| Batch of 5 refused with 413, halves fit | 3 requests of 5, 3 and 2 records. Rows 1 to 5. Exit 0. Standard error empty |
| The same with 400 naming `max_tokens_exceeded` | The same |
| The same with 422 | 1 request. Exit 4. 0146's one-line stop naming records 1 to 5 with the 422 phrase |
| The same with plain 400 | 1 request. Exit 4. The plain 400 phrase |
| First half still refused | 2 requests. No rows. `thinkthen: stopped at record 1; the request for records 1 to 3 failed: the backend answered with status 413: the backend refused the request as too large; set a lower --max-request-bytes; 0 records finished`. Exit 4 |
| Second half still refused | 3 requests. Rows 1 to 3. The same line naming records 4 to 5, `3 records finished`. Exit 4 |
| Batch of 2 refused | 3 requests. Each half's body equals today's single-record request, byte for byte |
| Batch of 1 refused, `--batch 1` | 1 request. Today's two lines. Exit 4 |
| 413 with `--max-retries 2` | Not retried. The counts above hold |
| A 503 on a half | Retried as today. Exhausted retries stop at the half's first record |
| An interrupt after the refusal | No half starts. Today's cancellation lines |
| `--details` on a split batch of 5 | Every row's `meta.batch.split` is `true`. Rows 1 to 3 carry `requests_sent` 1, 1, 0, and rows 4 and 5 carry 1, 0 |
| `--details` on an unsplit batch | No `split` member |
| `--dry-run` on a batch the backend would refuse | The whole batch's plan |
| `--replay` of a split run's recording | The recorded run's standard output, standard error and exit code, byte for byte, with no network |
| `--replay` of a run whose second half failed | Rows 1 to 3. Exit 5 at record 4, naming records 4 to 5 |
| `--replay` of the split recording at `--batch 4` | Exit 5 at record 1, naming records 1 to 4 |
| `--replay` of a `--batch 10` recording at `--batch 20` | Answered from the recorded halves, marked `split`. Intended: every answer comes from a recorded request with the same bytes |
| `--cache DIR` over a refusing backend, run twice | The second run sends 1 request, the refused whole, and answers both halves from the cache |

## Proof

Every test drives the compiled binary against the in-process loopback (`tests/backend/harness`, `Listener::answering`) or through `--dry-run`, which reads no key. The refusing loopback is an ordinary server: its closure answers a body longer than a set size with the refusal, and every other body with ticket 0146's per-record answer. No test-only export, flag or hook is added.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `the_request_size_closes_batches_at_every_address`, new in `tests/backend/batching/ceiling.rs` | The first thirteen edge rows through the loopback or `--dry-run`. Each row pins the request count, the records a request, each body's size against its limit, or the whole refusal sentence and exit code. The `--batch 1` row compares bodies by digest with the digests main prints before the change | (a) `ceiling()` keeps the built-in match: the nothing-set row sends 1 request. (b) The profile still replaces the size: the profile-of-200,000 row sends 1. (c) The larger of the two wins: the 50,000-and-200,000 row sends 1. (d) The variable beats the flag: the both-set row sends 1. (e) `0` parses: the refusal row sends. (f) The profile's refusal is dropped with the merge: the 100,000-byte line under a 50,000 profile goes alone |
| `a_request_size_over_the_ceiling_warns_at_the_built_in_address`, new in the same file | The five warning rows, through `decide --dry-run` and `relate --dry-run`. Each pins the whole standard error | (a) Warn at every address: the loopback row is red. (b) Warn at 96,000 or more: the 96,000 row is red. (c) Compare the URL as text: the trailing-slash row is red. (d) Warn only on `decide`: the `relate` row is red |
| `a_batch_refused_as_too_large_goes_again_in_halves`, new in `tests/backend/batching/too_large.rs` | The refusal rows of the edge table, at `--jobs 1`. Each row pins the loopback's request count and records a request, whole standard output, whole standard error and exit code. The `--details` row pins `split` and `requests_sent` | (a) No halving: the 413 row exits 4. (b) Halve again: the first-half row sends 4 requests. (c) Halve on 422: the 422 row sends 3. (d) Send the second half after the first fails: the first-half row sends 3. (e) The first half takes ⌊n/2⌋: the 413 row sees 5, 2 and 3. (f) Drop the refused attempt from the shares: rows 1 to 3 carry 1, 0, 0. (g) Quote a lone record in a half: the batch-of-2 row's bodies differ from today's |
| `a_split_run_replays_and_caches`, new in the same file | The replay and cache rows of the edge table. It records a split run into a private temporary folder, stops the loopback, and replays | (a) Replay does not ask the halves: the split replay exits 5 at record 1. (b) A miss on both names the first half: the `--batch 4` row names records 1 to 2. (c) The halves skip the cache: the second cached run sends 3 requests |
| `the_beatles_set_splits_at_every_address`, rewritten from `the_beatles_set_splits_at_the_hosted_address_and_nowhere_else` in `tests/backend/relate/ceiling.rs` | The relate rows of the edge table, by `--dry-run`. `a_profile_byte_limit_replaces_the_ceiling_and_other_limits_join_it` becomes `a_profile_byte_limit_lowers_the_size_and_other_limits_join_it`: the 200,000 profile row moves to `--max-request-bytes 200000`, and a 50,000 profile row shows the profile lowering it. `one_question_over_the_ceiling_goes_alone_and_is_not_refused` gains the same result at loopback. `a_full_line_set_plans_inside_the_child_deadline` keeps its one-request speed case under `--max-request-bytes 6000000`. The trailing-slash test moves into the warning test | (a) `SettledRelation::settle` keeps a built-in-only ceiling: loopback counts read 1 and 1. (b) `settle` lets a profile replace the size: the 200,000 profile row in the rewritten profile test gives 1 and 1 |
| `a_status_names_its_fixed_action_and_only_the_known_reason`, one new row and one changed row in `tests/backend/status_reason.rs` | Status 413 and the `max_tokens_exceeded` body print their pinned phrases | (a) Drop the 413 phrase: the bare status prints. (b) Keep the old `max_tokens_exceeded` ending: that row is red |

`core/batch/tests.rs::the_ceiling_closes_batches_at_the_built_in_address_only` loses its loopback row and its profile-raises row, and is renamed `the_ceiling_closes_batches_and_a_profile_lowers_it`. Its other rows stay. The command-level test above owns the every-address rule and the precedence, so one contract is tested at one layer.

The four questions:

- **`the_request_size_closes_batches_at_every_address`.** It protects ADR 0051 items 1 and 2 for batches: no default request passes 96,000 bytes anywhere, the flag beats the variable, the smaller of the size and a profile wins, a profile still refuses what cannot split, and bad values are refused. Each plant above is a credible slip in the precedence or the merge. No test checks a batch's size at another address through the command, and nothing reads the new flag or variable today. It needs no hook: the loopback is another address, and `--dry-run` shows what a run would send.
- **`a_request_size_over_the_ceiling_warns_at_the_built_in_address`.** It protects ADR 0051 item 4's sentence and its reach. A warning at every address, an off-by-one limit, a text match on the URL, or a missed verb fails it. No test reads this warning today. It needs no hook: `--dry-run` resolves the address and reads no key.
- **`a_batch_refused_as_too_large_goes_again_in_halves`.** It protects ADR 0051 items 6, 7, 8 and 10: which refusals halve, how many requests go, which rows print, the stop line, and the counts. Each plant above is a credible slip in the worker. Ticket 0146's failure test sends no refusal of this kind, and nothing halves today. It needs no hook: the loopback's refusal is an ordinary reply to an ordinary body.
- **`a_split_run_replays_and_caches`.** It protects ADR 0051 item 9, the determinism of a split run under `--record`, `--replay` and `--cache`. A replay that skips the halves, a miss that names the wrong range, or halves that bypass the cache fails it. Ticket 0146's replay test never splits. It needs no hook.
- **The rewritten relate tests and the status rows** change existing tests to the new contract. Each fails on main after the change and passes after it.

## Budgets

Nonblank lines, measured with `grep -c .`. Net lines against main after tickets 0146 and B5 land.

- `crates/thinkthen/src/core/backend.rs`: at most 15 net, for the carried size, `with_request_size` and `is_built_in`.
- `crates/thinkthen/src/core/batch.rs`: at most 40 net, for `halves` and the smaller-of merge.
- `crates/thinkthen/src/engine/prepared_request.rs`: at most 5 net.
- `crates/thinkthen/src/engine/error.rs`: at most 8 net, for `too_large()`.
- `crates/thinkthen/src/cli/args.rs`: at most 20 net, for the flag on five verbs.
- `crates/thinkthen/src/cli/edge.rs`, `cli/asking.rs`, `cli/relate.rs`, `cli/recognize.rs` and `cli/failure.rs`: at most 40 net together, for the variable, the refusals, `with_request_size` and the warning.
- `crates/thinkthen/src/cli/failure/status.rs`: at most 5 net.
- `crates/thinkthen/src/cli/asking/batched.rs`: at most 70 net, for halving and the replay halves.
- `crates/thinkthen/src/core/result.rs` and `result_json.rs`: at most 20 net together, for `split` and the refused attempts.
- Product code total: at most 223 net.
- `crates/thinkthen/tests/backend/batching/ceiling.rs`: at most 170, new.
- `crates/thinkthen/tests/backend/batching/too_large.rs`: at most 260, new.
- `tests/backend/relate/ceiling.rs`, `tests/backend/status_reason.rs`, `core/batch/tests.rs` and the `mod` lines: at most 35 net together.
- Pages under `specification/`: at most 45 net together.
- ADR 0040 and the batching design issue: at most 4 net together.
- `sdlc/ratchet.json` moves to the measured total, at most 688 above main after 0146 and B5 land: 223, 170, 260 and 35. The commit says what grew.
- No dependency.
- The `surfaces` rung runs, because the default size changes the libraries' relation plans at other addresses.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if ticket 0146 or B5 has not landed on main, unless the coordinator has chosen the fallback in "Order against ticket 0146".
3. Stop if `--batch 1` changes one byte of any request, row, standard error line or exit code. A batch of one never splits.
4. Stop if a committed recording, demo, probe replay or spec page changes under the new default. None is expected. Hand back which one.
5. Stop if the halving needs a recording format change, a new entry kind, a second scheduler, or a change to what `--jobs` counts.
6. Stop if the request size changes the recording folder's identity.
7. Stop if any plant stays green.
8. Stop if the change needs a file that ticket 0147, 0148 or 0155 still opens and has not landed. ADR 0040, `backends.md` line 17, `relate.md` and `recognize.md` wait for 0147. `settings.md` waits for 0148.
9. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.

## Scope and exclusions

Excluded: the setting on the libraries and SQL, which the request-size follow-up carries. Halving a relation chunk that a backend refuses. Halving more than once. A recorded refusal entry. Any change to the retry rule, which ticket 0155 owns. The flag on `choose`, `tag`, `score` and `annotate`, which their batching tickets add. A live measurement of whether a refused request is billed. `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code. The change raises the ceiling and adds a setting, so the code review names what it checked.

## Complexity

Contract 2; state and timing 1; reach 2; proof 2; cost of error 1; total 8. Final level: 2. The risks are a replay that stops matching its recording, a halving that sends more requests than it should, a precedence slip between the setting and a profile, and relation digests that move at other addresses. Tests 1, 3 and 4 guard the first three. The rewritten relate test pins the fourth.

## Deferred gaps

1. The library settings follow-up: `EngineBuilder::max_request_bytes`, `THINKTHEN_MAX_REQUEST_BYTES` in `from_env`, one row in ticket 0148's binding table per surface, one case in `conformance/settings.json`, and the SQL spellings by ticket 0149's rules. It also carries ticket 0155's `Counters::retries()` to Rust and every binding. It builds after 0148 and 0149. Until then the libraries apply 96,000 at every address.
2. Whether the hosted backend bills a refused request is unmeasured. ADR 0051 counts the refused attempt and claims no cost.
3. A cached rerun resends the refused whole batch once before its halves answer from the cache. A recorded refusal could skip that request. It waits until a run shows that the cost matters.
4. A relation chunk refused as too large still fails at exit 4. The default size makes that rare.
5. The libraries batch from B12a. B12a reuses `halves` and ADR 0051's rules, and its ticket names that.
6. B6 measures the token rate on batched record text. If it shows 96,000 bytes too loose for the hosted backend, a new ADR changes the default.
7. ADR 0040's marker, and the `relate.md` and `recognize.md` sentences, land after ticket 0147.
8. The flag joins `choose`, `tag`, `score` and `annotate` with B8, B9 and B10.

## What Ian can overturn

- Ian's ruling: the request size as a setting with a flag, a variable and a documented default.
- The coordinator's rulings: the default at every address, and one halving on a too-large refusal.
- Decision 2: the spelling `--max-request-bytes` and `THINKTHEN_MAX_REQUEST_BYTES`.
- Decision 3: no question-file or configuration-file tier.
- Decision 4: the smaller of the setting and a profile's limit, so a profile no longer raises the size.
- Decision 5: a warning, not a refusal, above 96,000 at the built-in address.
- Decision 6: the flag only on the verbs where it acts.
- Decision 7: a follow-up for the libraries, not an extension of 0148.
- Decision 8: status 413 and the named 400 body as too large at every address, with 422 left out.
- Decision 10: the first half takes the extra record, and the halves go one after the other.
- Decision 11: a replay asks the halves when the whole batch has no entry, including the intended `--batch 20` over `--batch 10` case.
- Decision 12: the refused attempts count in the first half.
- Decision 13: `split` on every row a half answers.
- Decision 14: the too-large phrases name `--max-request-bytes`.
- Decision 15: build after 0146 and B5, not folded into 0146.

## Closes

No issue. The build adds pointers to ADR 0051 in `sdlc/issues/2026-09-26-batching-design.md`, which stays open as the batching plan.

## Evidence

- Starts from: The rulings of 2026-09-26: the coordinator's on the default everywhere and the halving, and Ian's on the setting. ADR 0048 items 2, 5, 6 and 9, and sections 2 and 4 of `sdlc/issues/2026-09-26-batching-design.md`, which argue for failing a refused batch and for never splitting one. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` section 7, where the hosted backend refused 2,448 and 8,000 questions with 400 `max_tokens_exceeded` and took 7,000 short ones, and section 9's 181 bytes a title. Ticket 0123's measurement of 0.516 input tokens a byte for relate's JSON, and experiment 268's 0.40 for record text. The code at `origin/main` `6fbebfdf`: `core/backend.rs`, `core/batch.rs`, `core/recording.rs`, `engine/prepared_request.rs`, `engine/http.rs`, `engine/request.rs`, `cli/profile.rs`, `cli/edge.rs`, `cli/failure/status.rs`. Ticket 0146 at `6cdbe075`, and ticket 0148 at `d1afe98a` for the library settings table and its stop rule 3. A size scan of every committed recording file found none over 90,000 bytes.
- Keeps: Every request at the built-in address with no size set. Every `--batch 1` request, row, stop line and exit code. A profile's refusals. The retry rule and ADR 0048 item 6 for every status but a too-large refusal. The recording format and the folder identity. The throttle's meaning. Ticket 0146's stop lines and replay-miss range.
- Changes: The request size as a setting, `--max-request-bytes` then `THINKTHEN_MAX_REQUEST_BYTES` then 96,000, applied at every address. A profile's `max_request_bytes` lowers it and no longer raises it. A warning above 96,000 at the built-in address. One halving of a batch refused by 413 or 400 `max_tokens_exceeded`, with replay asking the halves. A 413 phrase and a new `max_tokens_exceeded` ending. `meta.batch.split`. ADR 0051, markers in ADR 0048, and ten specification pages.
- Proof: Four new loopback and dry-run tests and three amended ones, each with its plants, under "Proof".
- Defers: The setting on the libraries and SQL, billing of a refused request, the cached rerun's repeated refusal, relation chunk halving, library halving at B12a, B6's token rate, ADR 0040's marker after ticket 0147, and the flag on the verbs that batch later.
