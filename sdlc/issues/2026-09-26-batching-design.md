# Batching: many records in few requests

Status: Held by Ian. Do not start until Ian sends it.

Filed 2026-09-26. This design replaces `2026-09-25-packing-and-batching-what-they-buy-what-they-cost-and-the-setting.md`, now in `closed/`. Its evidence comes from workspace experiments 208, 260, 261, 262, 268 and 271, and from the 2026-09-22 wire probe. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` copies every table this design cites. `2026-09-26-recognize-design.md` uses the same batch rules for many short texts.

## What batching is for

A user points ThinkThen at a list and wants answers now. Today each record is its own request to Jev. Each request pays about 250 input tokens of fixed overhead, by experiment 271. A one-title request bills about 290 in all. Experiment 268 measured a median round trip of 135 to 151 ms. Jev's own time was 57 to 69 ms of it, and record size barely changed it. So a list of short records spends its money and its time on overhead.

Jev takes one evidence text and a `questions` map with many questions in one request. Experiment 271 sent 7,000 questions in one request. Jev set no count limit. It refused only requests over about 65,536 input tokens. One request can therefore carry many records, one question for each.

Ian ruled on 2026-09-25 that packing is a setting every surface reads the same way. The closed packing issue records that ruling. Ian then ruled that batching is on by default. No dated record of that second ruling exists, so this design records it, dated 2026-09-26. Batching changes answers, because a record's neighbours are in view. The default therefore stays at one record a request until ticket B6 shows the batched answers pass the calibration bars in acceptance test 9. The design states the measured trade plainly and gives one flag to turn batching off.

## What the user sees

### Command line

Ticket 0137, in progress, makes `filter` and `rank` read one record a line with no flag. Until it lands, every `filter` and `rank` example needs `--lines`, and the examples here carry it. The examples show the default after B6 turns it on.

The simplest call:

```sh
$ thinkthen filter 'The text is the title of a song by the Beatles. It appears on the album Abbey Road.' --lines --threshold 0.7 < songs.txt
Because
Carry That Weight
Come Together
…
```

The tool sends the 306 titles about 10 to a request, 34 requests, 4 at a time.

A shared reference text, sent once:

```sh
$ thinkthen filter 'It appears on the album Abbey Road.' --lines --threshold 0.7 --context catalog.txt < songs.txt
Because
Carry That Weight
Come Together
…
```

`catalog.txt` holds one line per song with its facts. With a context, a request carries as many records as fit, so all 306 titles go in one request.

Turning batching off or sizing it:

```sh
$ thinkthen filter '…' --lines --batch 1 < songs.txt     # one record a request, today's exact requests
$ thinkthen filter '…' --lines --batch 100 < songs.txt   # about 100 records a request, at most 200
```

Run facts with `--facts`: one summary line goes to standard error at the end. A finished run without `--facts` prints nothing there, as today.

```sh
$ thinkthen filter '…' --lines --threshold 0.7 --facts < songs.txt > kept.txt
{"schema":"thinkthen.run/1","records":306,"requests_sent":34,"cache_answers":0,"input_tokens":20400,"output_tokens":5500,"seconds":1.5,"model":"jev-1.13.0"}
```

The line's numbers are illustrative.

### Python

```python
import thinkthen as tt

kept = tt.filter("The text is the title of a song by the Beatles. It appears on the album Abbey Road.", songs)
kept[:3]
# ['Because', 'Carry That Weight', 'Come Together']
kept.facts
# Facts(records=306, requests_sent=34, cache_answers=0, input_tokens=20400, output_tokens=5500, seconds=1.5, model='jev-1.13.0')

kept = tt.filter("It appears on the album Abbey Road.", songs, context=catalog)   # a reference text, sent once
kept = tt.filter("It appears on the album Abbey Road.", songs, batch=1)          # one record a request

on_abbey_road = tt.question(decide="It appears on the album Abbey Road.", threshold=0.7)   # a stricter bar
kept = tt.filter(on_abbey_road, songs, context=catalog)
engine = tt.Engine(batch=1)
```

A plain string is the question, as today. It uses the default cut of 0.5, so it keeps more titles than the 0.7 walkthrough below. Its output lines and facts are illustrative. A verb over many records returns a list, as today. The list carries `.facts`, the run facts that `2026-09-26-every-surface-should-give-back-run-facts.md` asks for. `recognize` carries the same `Facts` fields. The other libraries take `batch` and `context` under their own spelling and return the same shape.

### DuckDB

```sql
SELECT title FROM songs
WHERE thinkthen_decide('The text is the title of a song by the Beatles. It appears on the album Abbey Road.', title);

SELECT title FROM songs
WHERE thinkthen_decide('It appears on the album Abbey Road.', title, (SELECT string_agg(entry, chr(10)) FROM catalog));

SET thinkthen_batch = 1;
```

The extension batches the rows DuckDB hands it in one call. A final `context` argument sends a reference text once per request.

### Walkthrough: a first user and 306 songs

A user has `songs.txt`, 306 Beatles titles, and wants the ones on Abbey Road. The catalog key marks 18.

| What they type | Requests | Time | Input tokens | Cost | Right of 306 | Kept | False yeses |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Today, one title a request | 306 | 13.0 to 14.2 s | 88,933 | $0.0037 | 285 | 29 to 31 | 16 to 17 |
| Default, about 10 titles a request | 31 | 1.3 to 1.7 s | 19,656 | $0.0008 | 283 to 287 | 35 to 39 | 18 to 22 |
| `--context catalog.txt` | 1 | 0.36 s | 22,091 | $0.0009 | 299 to 302 | 17 to 19 | 2 to 3 |

Experiment 271 measured every row, three runs each, at filter's cut of 0.7 and 4 requests in flight. Its batched rows used fixed tens in alphabetical order, 31 requests. The content cuts of section 2 form 34 requests on this list. Cost uses the recorded input price of $0.042 a million tokens. Output tokens are free under the vendor's price list. The batched rows used a Python client with experiment 271's own question wording. Ticket B6 measures the tool's own wording and time again.

The user types the first command and gets 35 to 39 titles in under 2 seconds. Today they would get 29 to 31 in 14 seconds. Batching made the filter more generous. It missed 1 Abbey Road song where today misses 4 or 5, and it kept a few more songs from late albums. That shift is why the default waits for test 9. The same user adds `--context catalog.txt` and gets 17 to 19 titles in a third of a second. Only 2 or 3 of them are wrong.

### Friction list

| What a user must know | What this design does |
| --- | --- |
| That batching exists | Designed away once B6 passes. It is then on by default for every stream |
| How many records to batch | Designed away. The default follows the measurements |
| That a live stream could wait for a full batch | Designed away. A live, unrecorded batch sends after a 50 ms pause in input |
| How to share a reference text | One option, `--context FILE` |
| How to get today's exact answers | One flag, `--batch 1` |
| That old recordings and caches answer only under `--batch 1` | Kept. A batch is a new request. Resending costs cents, and `--batch 1` replays old folders |
| That a record's answer can change with its neighbours | Kept, as Ian ruled. Test 9 bounds the change, and the page states it |
| That a threshold tuned at one batch size may not fit another | Kept. The run warns when the sizes differ, and so do `audit` and `diff` |
| That `filter` and `rank` need `--lines` | Designed away by ticket 0137, from `2026-09-26-filter-and-rank-could-read-lines-by-default.md` |
| That speed stalls above 3 jobs | Designed away by tickets B1 and B2, the two filed speed defects |

## The design

### 1. What one batch sends

A batch is a run of consecutive records in input order. It becomes one request. Each record gets its own question, and the question quotes the record:

```text
The text is "Come Together". The text is the title of a song by the Beatles. It appears on the album Abbey Road.
```

The prefix is `The text is ` followed by the record's evidence as a JSON value and `. `. A string record appears in JSON quotes with JSON escapes. An object or list from `--field`, JSONL, CSV or TSV appears as compact JSON. The user's own question follows unchanged. `--true` and `--false` stay on each question as `criteria`.

The evidence depends on the form:
- Without a context, the evidence is `{"records":[R1,…,RN]}`. Each record appears as the part `--field` selected, or whole.
- With a context, the evidence is the context text. Records appear only in their questions.

Without a context, a batch of one record sends today's request, byte for byte. `--batch 1` therefore keeps every existing recording, cache entry and fixture valid. With a context, a batch of one sends the context and one quoted record.

Why this shape:
- Quoting works. Experiment 271 gained 5 to 7 right from the quote alone, at one title a request. Experiment 261 found the same for pick-one.
- A question alone does not point Jev at a record. Experiment 261 scored 30 of 77 with the question alone and no quote.
- The evidence shape barely matters. Experiment 260 found no accuracy gap between a list of objects, named columns and one table string.
- A plain JSON list as evidence gets one silent answer. The 2026-09-22 wire probe sent a `state` holding a list of two strings and got one answer at 0.50 (`closed/2026-09-22-wire-probe-can-one-request-carry-many-states.md`, shape 4). The records sit inside an object for that reason.

Equal evidence inside one batch is asked once, and each copy gets that answer.

### 2. Where a batch closes

`N` is the batch size: 10 by default once B6 passes, 1 until then, or the value `--batch N` sets. A batch closes after a record when the first of these holds:

| Rule | When it closes |
| --- | --- |
| Content cut | The record's content hash is 0 mod `N` |
| Cap | The batch holds `2N` records |
| Profile limit | The next record would put the request over `max_questions`, `max_evidence_bytes` or `max_request_bytes`. At the built-in address the 96,000-byte ceiling stands in for `max_request_bytes` |
| Pause | A live, unrecorded run has waited 50 ms with no new record |
| End of input | Always |

The content hash is the SHA-256 of the record's evidence in compact JSON, the same bytes its question quotes. Its first 8 bytes, read as a big-endian unsigned integer, give the value taken mod `N`. At `N` of 1 every record closes its batch, so `--batch 1` sends today's requests.

With a context and no `--batch`, `N` is unbounded. A context batch then closes only at a profile limit, the ceiling, a pause or the end of input. `--batch N` with a context brings back the content cut and the cap.

**Why content cuts.** Ian ruled that batches follow content, so an inserted line moves only nearby batches. Fixed runs of 10 move every batch after an insert, and each moved batch misses the cache. Over the 306 titles at `N` of 10, content cuts form 34 batches of 1 to 20 records, and a title inserted after line 100 changed 1 batch. A batch closed by a profile limit or the cap starts the next batch early. The two layouts rejoin at the next content cut.

**Why 10 without a context.** On 306 songs, 10 a request scored 283 to 287 right against today's 285. Three runs of one setting disagree on 6 to 8 songs, so that gap is noise. It cut time about nine times and tokens 4.5 times. Going to 100 saved about another second on 306 songs and scored 281 to 284. All 306 in one request scored 276 to 278. Pick-one was measured only at 10, in experiment 262. It met the single-record bar there. Experiment 208 saw accuracy slip past 20 records, and the cap of `2N` keeps the default at 20 or fewer. A table of 100,000 rows at about 10 a request takes about 8 minutes at the vendor's 1,200 requests a minute. A caller who wants more speed sets `--batch 100`.

**Why no record limit with a context.** The reference text is the expensive part, so it should go once. On 306 songs the catalog sent once scored 299 to 302 right. Sending one catalog entry with each song scored 301 to 303. The single request used about a quarter of the tokens.

**Why 96,000 bytes.** The ceiling already exists in `specification/backends.md`. It holds under the token limit at the worst measured rate of 0.516 tokens a byte. It needs no tokenizer. Record text measured 0.40 tokens a byte in experiment 268. Ticket B6 measures the rate on batched record text again. A record whose batch of one already passes the ceiling goes alone as today's request.

**Other addresses.** An address other than the built-in one has no byte ceiling, as `specification/backends.md` fixes. There a batch closes at a content cut, the cap, a pause, the end of input, or a limit from `--profile FILE`. A backend that refuses a batch as too large fails it at exit 4. The caller then sets `max_request_bytes` in a profile, or `--batch 1`.

**Replies.** Ticket 0132's reply limit, 1 MiB plus 8 bytes per request byte, fits a batch. A reply runs about 105 bytes a question, and each quoted question adds more than that to the request.

**Why 50 ms.** Experiment 268 measured Jev's own time near 60 ms and a median round trip near 140 ms. A 50 ms wait adds about a third of one round trip to a live record. The pause rule fires only when the reader is waiting for input. It is off whenever `--cache`, `--record` or `--replay` names a folder, so a recorded run's batches depend only on the records and settings. A file or a fast pipe never pauses either. The value is fixed and has no option.

### 3. Order, jobs, cache and replay

- **Order.** Output keeps input order, as today. A batch's rows print when its reply arrives and every earlier row has printed.
- **Jobs.** `--jobs N` means N batches in flight. The default stays 4. The output buffer holds at most N batches of rows.
- **Cache key.** The cache key is the batch's request digest, the same digest rule as today. A record's answer depends on its neighbours, so a per-record key would serve an answer made beside other records.
- **Replay.** The same records under the same settings form the same batches and the same digests, because a recorded run never pauses. `--replay` then answers every batch from disk. An inserted, removed or changed record changes the batch that holds it, and at most the batches up to the next content cut. `--cache` pays only for those batches. `--replay` stops at the first missing batch at exit 5.
- **Settings that change batches.** `--batch`, `--context`, the question, the pointers, a profile's limits and the records themselves. `--jobs` changes no batch.

### 4. Failure

One request carries one batch, so one failed request fails every record in it. The run stops at the batch's first record, as `specification/records.md` stops at the first failed record. Rows before it stay printed. The stop line names the range and echoes no record:

```text
thinkthen: stopped at record 41; the request for records 41 to 50 failed: the backend answered with status 503; 40 records finished
```

A reply that answers some questions and not others fails only the records with missing or bad answers. The run stops at the first such record, and the earlier records of that batch print. `annotate` keeps its exit 6 rule for a failed question beside good answers.

A retried status (429, 500, 502, 503, 504 or 529) resends the whole batch as one request, as `specification/backends.md` fixes. Ticket 0089's premise holds here: a retried status means the backend answered, and it may have billed the first attempt. A batch can therefore be paid twice, as a single record can today. No batch is split and resent.

### 5. Which functions batch

| Function | How it batches | Default |
| --- | --- | --- |
| `decide`, `filter`, `rank` over records | One yes/no question per record | On after B6. Measured in 208, 260 and 271 |
| `choose` over records | One pick-one question per record, options as today | On after B8. Measured at 10 in 262 |
| `tag` over records | One yes/no question per record and label | Off until ticket B9 measures it |
| `score` over records | One levels question per record | Off until ticket B9 measures it |
| `annotate` over records | The records of one `on` group share a request, each question quoting its record | On for groups of yes/no and pick-one questions after ticket B10 |
| `find` | Already sends its whole set in one request | Unchanged |
| `recognize` over many short texts | As `2026-09-26-recognize-design.md` section 7 gives it | Off until that design's test 9 passes |
| `relate` | Does not batch. Each relation's request state differs, so relations cannot share a request. Ticket J1 runs its relations and split requests at once | Unchanged |
| Any verb on one document | Nothing to batch | Unchanged |
| A question file whose question text is a JSON object or list | The quote prefix needs a text question. Records themselves may be objects | One record a request |

### 6. Where the setting lives

| Surface | Batch size | Context |
| --- | --- | --- |
| Command | `--batch N`, 1 to 1,000 | `--context FILE` |
| Environment | `THINKTHEN_BATCH` | none |
| Question file | `"batch": N` | none |
| Python, TypeScript, Ruby, R, C, Rust | Engine setting and a per-call `batch` | Per-call `context` |
| Polars and pandas columns | As the library | As the library |
| DuckDB | `SET thinkthen_batch = N` | A final `context` argument on each scalar |
| PostgreSQL | `SET thinkthen.batch = N` | A final `context` argument |
| SQLite | The settings call the extension already has | A final `context` argument |

Ian ruled the order: the typed value, then the environment, then the question file, then the default. It matches ADR 0007's profile order: the flag, `THINKTHEN_PROFILE`, the file's `profile`, then the built-in one. The typed value is `--batch` or a per-call `batch=`. The environment tier holds `THINKTHEN_BATCH`, the engine setting and the SQL `SET`. `specification/question-file.md` line 98 rules the command line, then the file, then the default. B0 amends that section for `batch` and adds `--batch` to line 100's list. B0 also adds `batch`, a whole number from 1 to 1,000, to `question-file.schema.json`.

`batch` stays out of the question digest. A request cap such as the SQL extensions' request total counts a batch as one request. The context is evidence. It leaves the machine, and it enters the request digest but not the question digest.

The SQL and frame surfaces batch the rows one call receives: a DuckDB vector of up to 2,048 rows, a PostgreSQL array, a data-frame column, or SQLite's `thinkthen_warm`. Rows with equal evidence are asked once, as DuckDB does today. Batches form by the same content cuts within the call. A vector's edge also closes a batch, so DuckDB can cut a table differently from the command. A plain SQLite scalar receives one row at a time and cannot batch. DuckDB's parallel scan can hand rows over in another grouping. Exact replay then needs a fixed row order, such as `SET threads = 1`.

### 7. The batch size is calibration identity

A threshold tuned at one batch size may not fit another, because batching shifts probabilities. Experiment 271's false yeses rose from 16 or 17 to 18 to 22 at 10 a request. The batch size therefore joins the threshold's calibration identity under ADR 0032.

- The question file's `batch` names the size its threshold was tuned at, as its `profile` names the backend.
- A run whose batch size differs from the file's `batch` carries `meta.batch_warning` as `{"tuned_for":10,"running":1}`. It prints `threshold tuned at batch 10 is running at batch 1` once on standard error, as ADR 0032's profile warning does. A file with no `batch` warns nobody.
- `audit --write` writes the graded runs' batch size into the file's `batch` beside the threshold.
- `audit` and `diff` print one warning line on standard error when the runs they compare carry different batch sizes. `diff` then prints at most three warning lines.

The calibration identity's batch part stays out of the question digest, by Ian's ruling. ADR 0032 puts `profile` in the digest. B0 records that difference.

## Output and run facts

Bare output does not change.

Every per-record count is a share of its batch. The tool splits the batch's input tokens, output tokens and requests sent evenly across its records and gives any remainder to the earliest records. Shares therefore sum exactly to the batch, and the rows of a run sum exactly to the run. Under `--details` a batched record's `meta` carries:

```json
"meta":{"usage":{"input_tokens":63,"output_tokens":18},"requests_sent":1,"cached":false,"requests":["6b1f…c4"],"batch":{"size":10,"position":1,"closed":"content","usage":{"input_tokens":624,"output_tokens":175},"requests_sent":1}}
```

The row above is the first of a 10-record batch that billed 624 input and 175 output tokens. Rows 1 to 4 carry 63 input tokens and rows 5 to 10 carry 62. Rows 1 to 5 carry 18 output tokens and rows 6 to 10 carry 17. The other nine rows carry `requests_sent` 0. `meta.batch.size` is the run's batch size `N`. `position` is the record's place in its batch. `closed` says why the batch closed: `content`, `cap`, `limit`, `pause` or `end`. `meta.batch` is absent at a batch size of 1, and a reader takes its absence as size 1. So `--batch 1` rows keep today's bytes. `requests` holds the batch digest. A run with a context adds `meta.context_sha256`, the SHA-256 of the context file's bytes. When per-request time lands through `2026-09-23-record-the-backends-own-time-for-each-call.md`, a batched row reports its batch's time under `meta.batch`.

A usage field the backend did not report stays absent in a row's share. It is never written as 0.

### Run facts

Ian ruled that a finished run stays silent on standard error by default, as `specification/records.md` line 109 says. Facts print only when the caller asks. The one name is `facts` on every surface:
- The command takes `--facts`. At the end of the run it writes one `thinkthen.run/1` line to standard error, finished or stopped. `--facts` works with or without `--details`.
- A library result over many records carries `.facts`, or the surface's own spelling of it. A single call's facts have the same fields with `records` of 1. The run-facts ADR picks how a bare-value call returns them.
- SQL per-call facts are deferred by name in `2026-09-26-every-surface-should-give-back-run-facts.md`. `thinkthen_usage()` keeps its process totals.

The fields are `records`, `requests_sent`, `cache_answers`, `input_tokens`, `output_tokens`, `seconds` and `model`. The names match `thinkthen status` and the library counters. `requests_sent` counts every HTTP attempt, as `meta.requests_sent` does. `cache_answers` counts requests answered from a cache, as `status` counts them. A row answered that way carries `meta.cached` true. `input_tokens` and `output_tokens` sum the usage of live replies, as `status` does. Each is present only when every live reply in the run reported it. With no live reply, or a reply that reported none, the field is absent. It is never 0 for a count nobody reported. `seconds` is wall time. It never enters a byte-identity check.

`filter --details` prints only kept records, so its rows undercount the run. `--facts` counts every record and request.

`--dry-run` prints the plan for the first batch in place of the first record. It reads until the first batch closes by content, cap, limit or end of input, and never waits on a pause.

## Edge cases

| Case | Expected behaviour |
| --- | --- |
| Empty stream | No output, no request |
| One record | Today's exact request |
| 306 titles, default | 34 requests of 1 to 20 records |
| A line inserted into a recorded list | Its batch and at most the batches up to the next content cut miss the cache. The rest replay |
| A record whose batch of one passes 96,000 bytes | Sent alone as today's request |
| A record that passes Jev's evidence limit | The backend refuses it at exit 4 with today's `max_tokens_exceeded` message |
| A context whose request with one record passes the ceiling or a profile limit | Exit 2 before any request. The message names the limit and the context's size, and echoes no text |
| Two equal records in one batch | One question. Both get its answer. Both print where kept |
| A record holding quote marks or a newline | JSON escapes inside the quote |
| A CSV row | Quoted as its compact JSON object |
| A question file whose question text is a JSON object or list | One record a request |
| `--batch 1 --context catalog.txt` | Each request carries the context and one quoted record |
| `--batch 50 --context catalog.txt` | Content cuts at 50, at most 100 records a request |
| `--batch 0` or `--batch 1001` | Usage error at exit 2 |
| Question file `"batch": 1` with `--batch 10` | Batches of about 10. The typed flag wins. The tuned-for warning prints |
| Question file `"batch": 1` with `THINKTHEN_BATCH=10` | Batches of about 10. The environment beats the file. The warning prints |
| Question file `"batch": 1` and no other setting | One record a request |
| Question file `"batch": 1` with `tt.Engine(batch=10)` | Batches of about 10. The engine setting sits in the environment tier. The warning shows in `meta.batch_warning` |
| `--batch 10` with `--jobs 1` | Same output bytes as `--jobs 8` |
| A live, unrecorded stream that pauses | The open batch sends after 50 ms |
| A stream that pauses under `--cache DIR` | No pause cut. The batch waits for a content cut, the cap, a limit or the end |
| `--replay` with another `--batch` than the recording | Exit 5 at the first missing batch |
| An old folder recorded one record a request | Replays under `--batch 1`. Under the default, `--replay` stops at exit 5 and `--cache` pays again |
| One batch fails | The run stops at its first record. Earlier rows stay printed. The stop line names the range |
| One question in a reply is missing | That record fails. Earlier records of the batch print |
| A recorded reply missing one answer | Every replay fails at that record the same way. Replay never resends to fill it |
| A 429 or 5xx | The whole batch is resent as one request. After a 5xx the backend may bill both attempts |
| An interrupt mid-batch | No new batch starts. Batches in flight finish and print in order, as `specification/channels.md` fixes for SIGINT. They are billed |
| The reader downstream closes the pipe | No new batch starts. Batches in flight may be billed |
| `rank --top 5` | Every record is judged in batches. Five print |
| DuckDB parallel scan | Batches may differ between runs. The cache misses those batches |

## Acceptance tests

Tests 1 to 8 and 13 need no network. They use recordings and a loopback backend.

1. **Batch of one.** Every existing fixture under `specification/fixtures/systemone/` encodes byte for byte under `--batch 1` and under a stream of one record.
2. **Batch body.** New fixtures pin one plain batch of three records, one context batch, one pick-one batch, one CSV batch and one batch with a duplicate record.
3. **Grouping.** A 25-line fixture forms the batches its README works out by hand with `sha256sum` at `--batch 10`. No batch passes 20 records. Records of 40,000 bytes close batches at the ceiling. The same fixture with one line inserted changes only the batches the README names.
4. **Order and jobs.** A recorded 306-line `filter` prints identical bytes at `--jobs 1` and `--jobs 8`, on standard output and standard error, without `--facts`. With `--facts`, the two standard error lines match once `seconds` is removed.
5. **Replay.** A recorded run replays with no network under the same settings. Under another `--batch` it exits 5 and names the first missing batch.
6. **Shares.** For every batch in a recorded run, the rows' shares of input tokens, output tokens and requests sent sum to the batch. The `--facts` totals equal the sum over live replies. A replayed run's facts carry no token fields.
7. **Pause.** A real pipe carries 3 records from a writer that then stops and holds the pipe open. The loopback backend receives a first request of 3 records within 5 seconds. The same pipe under `--cache DIR` sends nothing until the writer closes the pipe.
8. **Failure.** A loopback backend answers 503 to the second batch's request after the retries. The run stops at that batch's first record, with every earlier row printed and exit 4.
9. **Calibration safety, recorded live.** Run `decide --lines --details` with experiment 271's key and filter's question at a cut of 0.7 over the 306 songs. `decide` prints every record, so `audit` sees every answer. Make three runs at `--batch 1`, three at the default, and one at the default over each of two shuffled orders with fixed seeds. The default turns on only if every bar holds:
   - **Shuffled orders.** Between the two shuffled runs, at most 15 of 306 answers cross the cut, and the mean absolute change in probability is at most 0.06. Experiment 271 saw 2 to 6 crossings and a mean of 0.02 between repeats of the same bytes. Experiment 208's regrouped tens crossed 3.4% of answers with a mean move of 0.047.
   - **False yeses.** The default runs' mean false yeses exceed the `--batch 1` runs' mean by at most 3. Experiment 271's Python client gave 20.7 against 16.7, a gap of 4, so its wording would fail this bar.
   - **Calibration error.** The default runs' mean calibration error from `thinkthen audit` exceeds the `--batch 1` mean by at most 0.02. Over 271's saved replies it fell from about 0.19 to about 0.12.
   - **Right answers.** Each default run scores at least 280 right. Today scores 285.
10. **Accuracy, context.** The same songs with the catalog as `--context`: one request, and at least 297 right in each of three repeats. Experiment 271 scored 299 to 302.
11. **Pick-one.** Experiment 262's 200 `choose` items at the default batch score at least 135 of 200, experiment 262's bar.
12. **Speed.** Test 9's default runs finish in under 3 seconds each. Experiment 271 measured 1.3 to 1.7 s.
13. **Calibration identity.** A question file with `"batch": 10`, run from a recording at `--batch 1`, prints the tuned-for warning once and carries `meta.batch_warning`. `audit` and `diff` over two runs at different batch sizes each print their warning. `audit --write` writes the runs' batch size into the file.

Tests 9 to 12 are paid. Together they cost under $0.03 at the recorded price.

## Tickets

In order. "Needs ADR" marks a ticket that changes a Settled specification page.

| # | Outcome | Scope | Proof | Depends on | ADR |
| --- | --- | --- | --- | --- | --- |
| B0 | Ian's ruling recorded | One ADR: the batch shape, content cuts, sizes, shares, `--facts`, `--context`, `meta.context_sha256`, the question file's `batch` and its precedence, the batch size as calibration identity, and a default of 1 until B6 passes. It lands before recognize R0. It amends ADR 0007 line 103, "each record is its own request, and records never share model context". It amends ADR 0008's request table and its sentence "Two pieces of evidence never share a request", as ADR 0010 accepted them, and ADR 0010's `--jobs` as requests in flight. It amends ADR 0040's "Records … are never combined" and ADR 0032's rule that calibration identity enters the question digest. It amends the `roadmap.md` rows that keep `--context FILE` and packing out for isolation, `records.md` "Order and requests" and `jobs`, `result.md` `meta`, `channels.md`, `question-file.md` precedence and its schema, and `backends.md` | ADR accepted; pages updated | none | This is the ADR |
| B1 | Usage writes stop serializing requests | Per `2026-09-26-usage-file-writes-serialize-requests-in-flight.md` | That issue's timing at `--jobs 16` | none | no |
| B2 | The pool keeps up to `--jobs` connections | Per `2026-09-26-connection-pool-reopens-connections-above-three-jobs.md` | New connections at `--jobs 16` stay near the job count | none | no |
| B3 | The engine plans batches | Content cuts, cap, profile limits, ceiling, quote prefix, evidence object, duplicates, batch of one, digests. No surface change | Tests 1, 2 and 3 | B0 | covered by B0 |
| B4 | The command batches `decide`, `filter` and `rank`, with the default batch size at 1 | `--batch`, `THINKTHEN_BATCH`, the question file's `batch`, precedence, jobs over batches, order, pause, failure line, record, replay, cache, dry run | Tests 4, 5, 7 and 8 | B3 | covered by B0 |
| B5 | Run facts stay true under batches | Shares, `meta.batch`, `--facts` and the `thinkthen.run/1` line | Test 6 | B4 | covered by B0 |
| B16 | The batch size is calibration identity | `meta.batch_warning` and its line, the `audit` and `diff` warnings, `audit --write` writing `batch` | Test 13 | B5 | covered by B0 |
| B6 | The default is measured and turned on | One authorized live run with the tool's own wording, and the token rate on batched record text. The default moves to 10 only if every bar of tests 9 and 12 holds. Otherwise it stays at 1 and the ticket reports to Ian | Tests 9 and 12 | B1, B2, B5, B16 | no |
| B7 | Shared context | `--context FILE` for `decide`, `filter` and `rank`; no record limit with a context; exit 2 for a context over the ceiling; `meta.context_sha256` | Tests 2 and 10 | B4 | covered by B0 |
| B8 | `choose` batches | The pick-one question per record, and `--context` for `choose` | Test 11, plus one recorded context run reported in the ticket | B4, B7 | covered by B0 |
| B9 | `tag` and `score` measured | One authorized run per type on a labeled set of at least 200 items, at `--batch 1` and the default. Turn a type on only if it meets test 11's rule for its set | Recorded runs and the decision in the ticket | B4 | no |
| B10 | `annotate` batches per `on` group | Groups whose question types are on | A recorded question set of yes/no and pick-one questions | B8, B9 | covered by B0 |
| B11 | Moved out of batching | It is ticket J1 below | none | none | none |
| B12a | The Rust library batches and gives facts | `batch`, `context`, `.facts` on the public API | Shared conformance cases for batch count, shares and `--batch 1` bytes | B5, B7 | Needs an ADR amending ADR 0017 for `.facts`, unless the run-facts ADR lands first |
| B12b | The C door batches and gives facts | The same, on the JSON door | The shared conformance cases | B12a | covered by B12a's ADR |
| B12c | Python batches and gives facts | The same | The shared conformance cases | B12a | covered by B12a's ADR |
| B12d | TypeScript batches and gives facts | The same | The shared conformance cases | B12a | covered by B12a's ADR |
| B12e | Ruby batches and gives facts | The same | The shared conformance cases | B12a | covered by B12a's ADR |
| B12f | R batches and gives facts | The same | The shared conformance cases | B12a | covered by B12a's ADR |
| B13a | Polars columns batch | A column is one call | The shared conformance cases | B12a | covered by B0 |
| B13b | pandas columns batch | A column is one call | The shared conformance cases | B12c | covered by B0 |
| B13c | DuckDB batches | Vectors, `SET thinkthen_batch`, the `context` argument | A DuckDB query over the 306 rows sends the command's 34 requests at `SET threads = 1`; conformance cases | B12a | covered by B0 |
| B13d | PostgreSQL batches | Arrays, `SET thinkthen.batch`, the `context` argument | The shared conformance cases | B12a | covered by B0 |
| B13e | SQLite batches | `thinkthen_warm`, the settings call, the `context` argument | The shared conformance cases | B12a | covered by B0 |
| B14 | Dropped | Ticket 0137 makes `filter` and `rank` read lines by default | none | none | none |
| B15 | The site explains batching | One how-to page with the walkthrough table, the flag, the context, the calibration warning and the replay rule. Its examples leave out `--lines` only after ticket 0137 lands | Docs review | B6, B7, ticket 0137 | no |

One ticket sits outside batching:

| # | Outcome | Scope | Proof | Depends on | ADR |
| --- | --- | --- | --- | --- | --- |
| J1 | `relate` and split requests run at once | One text's split requests and one relate's relations run under the engine's throttle. `relate` takes `--jobs`. Closes `2026-09-25-relate-sends-one-chunk-at-a-time.md` | Ticket 0118's loopback count reaches the throttle | none | Needs an amendment to `relate.md`, which says the command sends its requests in order and refuses `--jobs` |

## Open items Ian can overturn

1. The default of 10 records without a context, the cap of `2N`, and no record limit with a context.
2. The content cut on SHA-256 mod `N`, in place of fixed runs.
3. The bars of test 9, set by the queue owner from experiments 208 and 271.
4. The 50 ms pause, its lack of an option, and its absence under a recording folder.
5. `--batch 1` sending today's unquoted request. Experiment 271 found the quote alone gained 5 to 7 right. A later change could quote at one record too, at the cost of every old recording.
6. Even shares of tokens across a batch, in place of shares by record size.
7. `--facts` as the one option for run facts, with silence by default.
8. `tag`, `score` and `recognize` waiting for their measurements before they batch.
9. Leaving the evidence of plain batches in an object, where experiment 271 sent a plain list of lines.
10. DuckDB's vector edges still close batches where the command does not. Recordings therefore match across the command and DuckDB only at `--batch 1` until that is solved.
11. The batch part of calibration identity staying out of the question digest, unlike `profile`.
