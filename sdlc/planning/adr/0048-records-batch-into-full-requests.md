# ADR 0048: Records batch into full requests

> Amended by [ADR 0111](0111-question-cache-and-one-batching-path.md): item 5 is replaced. The cache and replay key is one backend question, not one batch request. Items 1, 2, 6 and 9 change as ADR 0111 lists.

- Status: Accepted 2026-09-26 on Ian's batching rulings of that day, through ticket 0139. The batching tickets named below build it. Ian can overturn each item
- Date: 2026-09-26

This ADR records the rulings in `sdlc/issues/closed/2026-09-26-batching-design.md`. That issue holds the argument. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` holds the measurements, and "evidence section N" below means its section N. It amends ADRs 0007, 0008, 0010, 0032 and 0040, and supersedes ADR 0009's one-request-per-record point. Changing an item below takes a new ADR.

## Context

Today each record is its own request. Each request pays about 250 input tokens of fixed overhead, and a one-title request bills about 290 in all (experiment 271, evidence section 7). A round trip takes a median of 135 to 151 ms, of which Jev's own time is 57 to 69 ms (experiment 268, evidence section 6). A list of short records therefore spends its money and its time on overhead.

Jev takes one evidence text and many questions in one request. Experiment 271 sent 7,000 questions in one request. Jev refused only requests over about 65,536 input tokens (evidence section 7).

Over the 306 Beatles titles, today's filter sent 306 requests in 13.0 to 14.2 s and scored 285 right. One full request answered in 0.30 to 0.39 s and scored 276 to 278 right. With the catalog as a shared context, one request scored 299 to 302 right in 0.36 s (evidence section 7). Ian ruled that speed wins.

## Decision

1. **The batch shape.** A batch is a run of consecutive records in input order, and it becomes one request. Each record gets its own question, which quotes the record: `The text is ` then the record's evidence as a JSON value, then `. `, then the user's question unchanged. A string record appears in JSON quotes with JSON escapes. An object or list appears as compact JSON. `--true` and `--false` stay on each question as `criteria`. Without a context the evidence is `{"records":[R1,…,RN]}`. With a context the evidence is the context text, and records appear only in their questions. Equal evidence inside one batch is asked once, and each copy gets that answer. Without a context, a batch of one record sends today's request byte for byte, so `--batch 1` keeps every recording, cache entry and fixture valid. The evidence sits inside an object because a plain list got one silent answer in the 2026-09-22 wire probe (evidence section 5).
2. **Where a batch closes.** Records join the open batch in input order. A batch closes after a record when the first of these holds:
   - Content cut: the record's content hash is 0 mod 4,096. The hash is the SHA-256 of the record's evidence in compact JSON, the bytes its question quotes. Its first 8 bytes, read as a big-endian unsigned integer, give the value taken mod 4,096. A context does not change the rule.
   - Size: the setting is a number `N` and the batch holds `N` records.
   - Profile limit: the next record would put the request over `max_questions`, `max_evidence_bytes` or `max_request_bytes`. At the built-in address the 96,000-byte ceiling stands in for `max_request_bytes`. A record whose batch of one already passes the ceiling goes alone as today's request.
   - Pause: a live run with no recording folder has waited 50 ms with no new record. The pause is off whenever `--cache`, `--record` or `--replay` names a folder. It has no option. (Amended by ADR 0053, below.)
   - End of input.

   By default a batch fills to the limit. A cut depends only on its own record, so an insert moves no existing cut. An inserted, removed or changed record that is not a cut changes its batch and the later batches of its stretch. A record that is or becomes a cut splits a stretch, and a removed cut merges two. The batches from that point re-form up to the next cut. Other addresses have no byte ceiling, and a backend that refuses a batch as too large fails it at exit 4.
3. **The setting.** The batch setting is `max`, the default, or a whole number of at least 1. A number larger than fits fills to the limit. `--batch 0`, a fraction, and any text but `max` are a usage error at exit 2. `max` exists as a spelling so a caller can undo a question file's number.
4. **Precedence.** Four tiers: the typed value, then the environment, then the question file, then the default. Only a per-call value counts as typed: `--batch` or a per-call `batch=`. The environment tier holds `THINKTHEN_BATCH`, the engine setting and the SQL `SET`. The read-only configuration file of ADR 0033 holds no `batch` key and adds no tier. A question file or an `annotate` question set carries at most one top-level `batch`. `specification/settings.md` explains the setting on every surface.
5. **Order, jobs, cache and replay.** Output keeps input order. A batch's rows print when its reply arrives and every earlier row has printed. `--jobs N` means N batches in flight, and a batch is one request, so it still means N requests in flight. It takes 1 to 32 and defaults to 4. The output buffer holds at most N batches of rows. The cache key is the batch's request digest, by today's digest rule. The same records under the same settings form the same batches, so `--replay` answers every batch from disk and stops at the first missing batch at exit 5. `--batch`, `--context`, the question, the pointers, a profile's limits and the records change batches. `--jobs` changes none.
6. **Failure.** One failed request fails every record of its batch. The run stops at the batch's first record, and earlier rows stay printed. The stop line names the range and echoes no record: `thinkthen: stopped at record 41; the request for records 41 to 50 failed: the backend answered with status 503; 40 records finished`. A reply that answers some questions fails only the records with missing or bad answers, and the run stops at the first. A retried status resends the whole batch as one request, and it may be billed twice. No batch is split and resent.
7. **Which functions batch.** Every function that can batch does so by default. `decide`, `filter` and `rank` ask one yes/no question per record. `choose` asks one pick-one question per record. `tag` asks one yes/no question per record and label. `score` asks one levels question per record. `annotate` puts the records of one `on` group in a shared request. `recognize` over many short texts follows `2026-09-26-recognize-design.md` section 7. `find` already sends its whole set in one request. `relate` does not batch, because each relation's request differs. A verb on one document has nothing to batch. A question file whose question text is a JSON object or list sends one record a request, because the quote prefix needs a text question.
8. **Calibration identity.** A threshold tuned at one batch setting may not fit another. The question file's `batch` names the setting its threshold was tuned at, as its `profile` names the backend. A run whose setting differs from the file's carries `meta.batch_warning`, such as `{"tuned_for":1,"running":"max"}`, and prints `threshold tuned at batch 1 is running at batch max` once on standard error. A file with no `batch` warns nobody. `audit --write` writes the graded runs' setting into the file. `audit` and `diff` print one warning line when the runs they compare differ in batch setting. The batch part of calibration identity stays out of the question digest, unlike ADR 0032's `profile`. (Amended by ADRs 0053 and 0085.)
9. **Row metadata.** Each per-record count is an even share of its batch's input tokens, output tokens and requests sent, with any remainder to the earliest records. Shares sum exactly to the batch. A usage field the backend did not report stays absent, never 0. Under `--details` a batched row's `meta.batch` holds `setting`, `records`, `position`, `closed` (`content`, `size`, `limit`, `pause` or `end`), and the batch's own `usage` and `requests_sent`. `meta.batch` is absent when the batch holds one record and no context, so `--batch 1` rows keep today's bytes. `meta.requests` holds the batch digest. A run with a context adds `meta.context_sha256`, the SHA-256 of the context file's bytes.
10. **Run facts.** `--facts` writes one `thinkthen.run/1` line on standard error at the end of a finished or stopped run. Without it a finished run stays silent there. The fields are `records`, `requests_sent`, `cache_answers`, `input_tokens`, `output_tokens`, `seconds` and `model`. A token field is present only when every live reply reported it. `seconds` never enters a byte-identity check. Library results carry `facts` on every call, with no setting and no second call. The run-facts ADR, written by batching ticket B12a, picks how a bare-value call carries them. SQL per-call facts stay deferred. (Amended by ADR 0083 for the command's stop member, retry count, presence rules and batch metadata.)
11. **Shared context.** `--context FILE` sends a reference text once a request, as the evidence. It leaves the machine. It enters the request digest and not the question digest. A context whose request with one record passes the ceiling or a profile limit is a usage error at exit 2 before any request. The message names the limit and the size and echoes no text. (Amended by ADR 0087.)
12. **Speed ahead of accuracy.** The default fills to the limit although it costs accuracy: 276 to 278 right against 285 over the 306 titles, and false yeses from 16 or 17 up to 27 to 29 (evidence section 7). Experiment 208 found a larger cost on longer records (evidence section 1). Design test 9 measures and reports the tool's own cost and gates nothing. The target is `filter` over the 306 titles in under half a second at the default throttle of 4, on a named build, measured live by ticket S1. (Amended by ADR 0053, below.)
13. **Dry run.** Record-mode `--dry-run` prints the plan for the first batch. It reads until the first batch closes by content, size, limit or end of input, and never waits on a pause.

## Amendment, 2026-09-26: ADR 0053 after the batching review

ADR 0053 amends items 2, 8 and 12. The pause fires in every mode, not only when no folder is named. A batch also closes at 4,096 members, repeats included. A question file with a `threshold` and no `batch` counts as tuned at batch 1 for item 8's warning. Item 12's cost becomes the tool's own form: 272 to 274 right and 32 to 34 false yeses over the 306 titles, against 285 and 16 or 17 one a request. Records of one batch are evidence for each other, and `--batch 1` keeps them apart. Ian can overturn each change.

## Amendment, 2026-09-26: ADR 0051 sets a request size everywhere and halves a refused batch once

ADR 0051, through ticket 0154, amends items 2, 5, 6, 9 and 11. The request size is a setting, `--max-request-bytes` or `THINKTHEN_MAX_REQUEST_BYTES`, with a default of 96,000 bytes at every address, not only the built-in one. A profile's `max_request_bytes` lowers it and no longer raises it. A batch of two or more records that the backend refuses as too large, by status 413 or status 400 naming `max_tokens_exceeded`, is split into two halves and each half is sent once. A replay asks those halves when the whole batch has no entry. Once B5 builds `meta.batch`, a row a half answers carries `"split":true` there. Until then the rows' per-request counts and digests record a split. Every other status keeps item 6's rule. Item 11's refusal of a context too large for one record now applies at every address, at the request size. Ian can overturn each change.

## What this amends

| Where | What changes |
| --- | --- |
| ADR 0007 line 103 | Records of one batch share one request and see each other |
| ADR 0007 line 153 | The clarification of one request per record counts batches |
| ADR 0009 line 12 | One request per record is superseded |
| ADR 0008 lines 41 to 52 | Two records share a request when they share a batch. The request table counts batches |
| ADR 0010 line 34 | `--jobs N` counts batches in flight, each one request |
| ADR 0032 line 18 | The batch part of calibration identity stays out of the digest |
| ADR 0040 lines 20 and 24 | Records combine into batches. Batched record plans at the built-in address close at the ceiling. This ADR is the packing ADR line 24 names |
| `specification/roadmap.md` | `--context FILE` and packing leave the held table |
| `records.md`, `result.md`, `channels.md`, `question-file.md`, `backends.md`, `annotate.md` | Each new rule stands beside today's sentence as `Not built yet, by ADR 0048 item N: …`. The ticket that builds item N deletes the old sentence and the marker. `grep -rn "Not built yet, by ADR 0048" specification` lists the leftovers, and the last batching ticket requires it to come back empty |

## Which ticket builds each item

The labels are the batching design's.

| Item | Built by |
| --- | --- |
| 1, 2 | B3 plans the batches. B4 puts them on the command |
| 3, 4, 5, 6, 13 | B4 for `decide`, `filter` and `rank`. B4 adds `batch` to the question file's `decide` entry and schema, B8 to `choose`, B9 to `tag` and `score`, B10 to the `annotate` question set |
| 7 | B4, B8, B9, B10, and recognize R7, one row each |
| 8 | B16 |
| 9, 10 | B5 on the command. B12a to B12f in the libraries |
| 11 | B7, and B8 for `choose` |
| 12 | S1 for the speed target, B6 for the accuracy cost |
| Libraries and SQL | B12a to B13e |

## What recognize R0 may rely on

- This ADR by number, with items 1 to 13 at these numbers.
- The batch shape and the close rule, items 1 and 2. R0's recognize batch shape extends them for many short texts.
- The setting, its spelling `max`, and its four tiers, items 3 and 4.
- `meta.batch` and its fields, item 9. R0 adds recognize's use of it.
- `jobs` as batches in flight, item 5. R0 amends `records.md` `jobs` for `recognize` on one document.
- ADR 0040's amendment reaching batched record plans. R0 amends ADR 0040 again for word and `confirm` questions.
- The canonical question form unchanged. `batch` and `context` stay out of the question digest, so `question-file.md`'s key order is R0's to amend.

This ADR leaves to R0 `backends.md` line 17, `recognize.md`, the recognition keys in `question-file.md`, and `answer.confirm`.

## What Ian can overturn

Ian's rulings:

1. Fill to the limit by default, with `--batch N` as an upper bound.
2. Content cuts inside fill-to-limit batches.
3. Speed ahead of accuracy. Test 9 reports and gates nothing.
4. `--facts` as the command's one option for run facts, silent by default. Library results carry `facts` on every call.
5. The batch part of calibration identity staying out of the question digest, unlike `profile`.
6. The ticket order: B0, C1, B1, B2, J1, B3, S1, B4, then the rest.

The design author's calls:

7. `max` as the spelling of the default.
8. The cut at SHA-256 mod 4,096.
9. `tag`, `score`, `annotate` and `recognize` batching by default, with their cost measured and reported.
10. The 50 ms pause, its lack of an option, and its absence under a recording folder. ADR 0053 item 1 withdrew that absence, so the pause fires in every mode.
11. `--batch 1` sending today's unquoted request.
12. Even shares of tokens across a batch.
13. The evidence of plain batches in an object.
14. DuckDB's vector edges closing batches where the command does not.
15. Recognize's 600,000-byte hard cap and 200-word window default, from the recognize design.

Ticket 0139's calls:

16. Pages keep today's sentence beside the marked rule until it is built.
17. The schema gains `batch` with the parser that reads it, not before.
18. `--batch`, `--context` and `--facts` in the long help only.
19. An `annotate` question set taking one top-level `batch`, built by B10.

## Amendment, 2026-09-26: ADR 0055 sends each record once

ADR 0055 amends item 1 for `decide`, `filter` and `rank`. Without a context, a batch of two or more distinct records sends the fixed sentence `Each question quotes the text it asks about.` as its evidence in place of `{"records":[…]}`. Each question still quotes its record as item 1 says. A batch of one still sends today's request byte for byte. `choose` and `tag` keep item 1's form until B8 and B9 measure them on long records. Ian can overturn it.

Amendment, ticket 0400 slice C (2026-10-04): omitted throttle now uses 8. Explicit settings, range 1 through 32 and the first explicit process selection remain unchanged. Ian can overturn this default.
