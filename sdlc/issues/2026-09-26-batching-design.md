# Batching: many records in few requests

Status: Held by Ian. Do not start until Ian sends it.

Filed 2026-09-26. This design replaces `2026-09-25-packing-and-batching-what-they-buy-what-they-cost-and-the-setting.md`, now in `closed/`. Its evidence comes from workspace experiments 208, 260, 261, 262, 268 and 271. `2026-09-26-recognize-design.md` uses the same batch rules for many short texts.

## What batching is for

A user points ThinkThen at a list and wants answers now. Today each record is its own request to Jev. Each request pays about 250 input tokens of fixed overhead, by experiment 271, and a one-title request bills about 290 in all. A bare title adds only about 3 tokens. Experiment 268 measured a round trip of 140 to 195 ms. Jev's own time was about 60 ms of it, and context size barely changed it. So a list of short records spends its money and its time on overhead.

Jev takes one evidence text and a `questions` map with many questions in one request. Experiment 271 sent 7,000 questions in one request. Jev set no count limit. It refused only requests over about 65,536 input tokens. One request can therefore carry many records, one question for each.

Ian has ruled that batching is on by default. Batching changes answers, because a record's neighbours are in view. The design states the measured trade plainly and gives one flag to turn it off.

## What the user sees

### Command line

Every example shows what a user types after the tickets land. Ticket B14 makes `filter` and `rank` read one record a line by default.

The simplest call needs no flag:

```sh
$ thinkthen filter 'The text is the title of a song by the Beatles. It appears on the album Abbey Road.' --threshold 0.7 < songs.txt
Because
Carry That Weight
Come Together
…
```

The tool sends the 306 titles 10 to a request, 31 requests, 4 at a time.

A shared reference text, sent once:

```sh
$ thinkthen filter 'It appears on the album Abbey Road.' --threshold 0.7 --context catalog.txt < songs.txt
Because
Carry That Weight
Come Together
…
```

`catalog.txt` holds one line per song with its facts. With a context, a request carries as many records as fit, so all 306 titles go in one request.

Turning batching off or sizing it:

```sh
$ thinkthen filter '…' --batch 1 < songs.txt     # one record a request, today's exact requests
$ thinkthen filter '…' --batch 100 < songs.txt   # up to 100 records a request
```

Run facts with `--details`: each kept record prints its full result, and one summary line goes to standard error at the end.

```sh
$ thinkthen filter '…' --threshold 0.7 --details < songs.txt > kept.jsonl
{"schema":"thinkthen.run/1","records":306,"requests_sent":31,"cached_requests":0,"input_tokens":19656,"output_tokens":5356,"seconds":1.4,"model":"jev-1.13.0"}
```

### Python

```python
import thinkthen as tt

kept = tt.filter("The text is the title of a song by the Beatles. It appears on the album Abbey Road.", songs)
kept[:3]
# ['Because', 'Carry That Weight', 'Come Together']
kept.facts
# Facts(records=306, requests_sent=31, cached_requests=0, input_tokens=19656, output_tokens=5356, seconds=1.4, model='jev-1.13.0')

kept = tt.filter("It appears on the album Abbey Road.", songs, context=catalog)   # a reference text, sent once
kept = tt.filter("It appears on the album Abbey Road.", songs, batch=1)          # one record a request

on_abbey_road = tt.question(decide="It appears on the album Abbey Road.", threshold=0.7)   # a stricter bar
kept = tt.filter(on_abbey_road, songs, context=catalog)
engine = tt.Engine(batch=1)
```

A plain string is the question, as today. It uses the default cut of 0.5, so it keeps more titles than the 0.7 walkthrough below. Its output lines are illustrative. A verb over many records returns a list, as today. The list carries `.facts`, the run facts that `2026-09-26-every-surface-should-give-back-run-facts.md` asks for. `recognize` carries the same `Facts` fields. The other libraries take `batch` and `context` under their own spelling and return the same shape.

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
| Default, 10 titles a request | 31 | 1.3 to 1.7 s | 19,656 | $0.0008 | 283 to 287 | 35 to 39 | 18 to 22 |
| `--context catalog.txt` | 1 | 0.36 s | 22,091 | $0.0009 | 299 to 302 | 17 to 19 | 2 to 3 |

Experiment 271 measured every row, three runs each, at filter's cut of 0.7 and 4 requests in flight. Cost uses the recorded input price of $0.042 a million tokens. Output tokens are free under the vendor's price list. The batched rows used a Python client with experiment 271's own question wording. The tool's wording and time are measured again in ticket B6.

The user types the first command and gets 35 to 39 titles in under 2 seconds. Today they would get 29 to 31 in 14 seconds. Batching made the filter more generous. It missed 1 Abbey Road song where today misses 4 or 5, and it kept a few more songs from late albums. The same user adds `--context catalog.txt` and gets 17 to 19 titles in a third of a second. Only 2 or 3 of them are wrong.

### Friction list

| What a user must know | What this design does |
| --- | --- |
| That batching exists | Designed away. It is on by default for every stream |
| How many records to batch | Designed away. The default follows the measurements |
| That a live stream could wait for a full batch | Designed away. A batch sends after a 50 ms pause in input |
| How to share a reference text | One option, `--context FILE` |
| How to get today's exact answers | One flag, `--batch 1` |
| That old recordings and caches answer only under `--batch 1` | Kept. A batch is a new request. Resending costs cents, and `--batch 1` replays old folders |
| That a record's answer can change with its neighbours | Kept, as Ian ruled. The page states the measured trade |
| That `filter` and `rank` need `--lines` | Designed away by ticket B14, from `2026-09-26-filter-and-rank-could-read-lines-by-default.md`. The examples above already leave it out |
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
- A question name does not point Jev at a record. Experiment 261 scored 30 of 77 with the question alone and no quote.
- The evidence shape barely matters. Experiment 260 found no accuracy gap between a list of objects, named columns and one table string.
- A plain JSON list as evidence gets one silent answer. The records sit inside an object for that reason.

Equal evidence inside one batch is asked once, and each copy gets that answer.

### 2. How big a batch is

A batch closes at the first of these:

| Rule | Default |
| --- | --- |
| Records | 10 without a context. No record limit with a context. `--batch N` caps records in both forms |
| Request bytes | 96,000, the built-in ceiling `specification/backends.md` already sets for relation plans |
| A pause in input | 50 ms with no new record |
| End of input | Always |

**Why 10 without a context.** On 306 songs, 10 a request scored 283 to 287 right against today's 285. Three runs of one setting disagree on 6 to 8 songs, so that gap is noise. It cut time about nine times and tokens 4.5 times. Going to 100 saved about another second on 306 songs and scored 281 to 284. All 306 in one request scored 276 to 278. Pick-one was measured only at 10, in experiment 262. It met the single-record bar there. Experiment 208 saw accuracy slip past 20 records with position references. A table of 100,000 rows at 10 a request takes about 8 minutes at the vendor's 1,200 requests a minute. A caller who wants more speed sets `--batch 100`.

**Why no record limit with a context.** The reference text is the expensive part, so it should go once. On 306 songs the catalog sent once scored 299 to 302 right. Sending one catalog entry with each song scored 301 to 303. The single request used about a quarter of the tokens.

**Why 96,000 bytes.** The ceiling already exists and holds under the token limit at the worst measured rate of 0.516 tokens a byte. It needs no tokenizer. A record whose batch of one already passes the ceiling goes alone as today's request.

**Why 50 ms.** Experiment 268 measured Jev's own time near 60 ms and a round trip of 140 to 195 ms. A 50 ms wait adds a quarter to a third of one round trip to a live record. The pause rule fires only when the reader is waiting for input. A file or a fast pipe never pauses, so its batches depend only on order and settings. The value is fixed and has no option.

### 3. Order, jobs, cache and replay

- **Order.** Output keeps input order, as today. A batch's rows print when its reply arrives and every earlier row has printed.
- **Jobs.** `--jobs N` means N batches in flight. The default stays 4. The output buffer holds at most N batches of rows.
- **Cache key.** The cache key is the batch's request digest, the same digest rule as today. A record's answer depends on its neighbours, so a per-record key would serve an answer made beside other records.
- **Replay.** Given the same input order and the same batch settings, the tool forms the same batches and the same digests. `--replay` then answers every batch from disk. A run whose batches were cut by a pause is the exception, and a known gap. Its saved input, read from a file, forms full batches. `--replay` stops at the first missing batch at exit 5, and `--cache` pays only for the batches that differ.
- **Settings that change batches.** `--batch`, `--context`, the question, the pointers and the input order. `--jobs` changes no batch.

### 4. Failure

One request carries one batch, so one failed request fails every record in it. The run stops at the batch's first record, as `specification/records.md` stops at the first failed record. Rows before it stay printed. The stop line names the range:

```text
thinkthen: stopped at record 41; the request for records 41 to 50 failed: the backend answered with status 503; 40 records finished
```

A reply that answers some questions and not others fails only the records with missing or bad answers. The run stops at the first such record, and the earlier records of that batch print. `annotate` keeps its exit 6 rule for a failed question beside good answers. A 429 is retried as one request, as `specification/backends.md` fixes. No batch is split and resent, because a batch under the ceiling cannot pass the token limit.

### 5. Which functions batch

| Function | How it batches | Default |
| --- | --- | --- |
| `decide`, `filter`, `rank` over records | One yes/no question per record | On. Measured in 208, 260 and 271 |
| `choose` over records | One pick-one question per record, options as today | On. Measured at 10 in 262 |
| `tag` over records | One yes/no question per record and label | Off until ticket B9 measures it |
| `score` over records | One levels question per record | Off until ticket B9 measures it |
| `annotate` over records | The records of one `on` group share a request, each question quoting its record | On for groups of yes/no and pick-one questions after ticket B10 |
| `find` | Already sends its whole set in one request | Unchanged |
| `recognize` over many short texts | As `2026-09-26-recognize-design.md` section 7 gives it | Off until that design's test 9 passes |
| `relate` | One complete set. Each relation's request state differs, so relations cannot share a request. Its relations and split requests run at once under `--jobs` | Ticket B11 |
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

A typed value wins. `--batch` on the command and `batch=` on one call replace the question file's `batch`, as `specification/question-file.md` line 100 rules for typed single values. Ticket B0 adds `--batch` to that line's list. The file's `batch` in turn replaces `THINKTHEN_BATCH` and the engine setting. A question that batches badly carries `"batch": 1` and stays unbatched until a caller types another size. A request cap such as the SQL extensions' request total counts a batch as one request. The context is evidence. It leaves the machine, and it enters the request digest but not the question digest.

The SQL and frame surfaces batch the rows one call receives: a DuckDB vector of up to 2,048 rows, a PostgreSQL array, a data-frame column, or SQLite's `thinkthen_warm`. Rows with equal evidence are asked once, as DuckDB does today. Batches form in first-seen order within the call. A plain SQLite scalar receives one row at a time and cannot batch. DuckDB's parallel scan can hand rows over in another grouping. Exact replay then needs a fixed row order, such as `SET threads = 1`.

## Output and run facts

Bare output does not change.

Every per-record count is a share of its batch. The tool splits the batch's input tokens, output tokens and requests sent evenly across its records and gives any remainder to the earliest records. Shares therefore sum exactly to the batch, and the rows of a run sum exactly to the run. Under `--details` a batched record's `meta` carries:

```json
"meta":{"usage":{"input_tokens":63,"output_tokens":18},"requests_sent":1,"cached":false,"requests":["6b1f…c4"],"batch":{"size":10,"position":1,"closed":"full","usage":{"input_tokens":624,"output_tokens":175},"requests_sent":1}}
```

The row above is the first of a 10-record batch that billed 624 input and 175 output tokens. Rows 1 to 4 carry 63 input tokens and rows 5 to 10 carry 62. Rows 1 to 5 carry 18 output tokens and rows 6 to 10 carry 17. The other nine rows carry `requests_sent` 0. `closed` says why the batch closed: `full`, `bytes`, `pause` or `end`. `requests` holds the batch digest. When per-request time lands through `2026-09-23-record-the-backends-own-time-for-each-call.md`, a batched row reports its batch's time under `meta.batch`.

`filter --details` prints only kept records, so its rows undercount the run. The command therefore writes one `thinkthen.run/1` line to standard error at the end of every `--details` run. It carries records, requests sent, cached requests, input tokens, output tokens, seconds and model. The library's `.facts` carries the same fields.

`--dry-run` prints the plan for the first batch in place of the first record. It reads until the first batch closes by size, bytes or end of input, and never waits on a pause.

## Edge cases

| Case | Expected behaviour |
| --- | --- |
| Empty stream | No output, no request |
| One record | Today's exact request |
| 306 records, default | 31 requests: 30 of 10 and 1 of 6 |
| A record whose batch of one passes 96,000 bytes | Sent alone as today's request |
| A record that passes Jev's evidence limit | The backend refuses it at exit 4 with today's `max_tokens_exceeded` message |
| A context larger than the ceiling | Each request carries the context and one record. `--dry-run` shows the request count |
| Two equal records in one batch | One question. Both get its answer. Both print where kept |
| A record holding quote marks or a newline | JSON escapes inside the quote |
| A CSV row | Quoted as its compact JSON object |
| A question file whose question text is a JSON object or list | One record a request |
| `--batch 1 --context catalog.txt` | Each request carries the context and one quoted record |
| `--batch 50 --context catalog.txt` | At most 50 records a request |
| `--batch 0` or `--batch 1001` | Usage error at exit 2 |
| Question file `"batch": 1` with `--batch 10` | Up to 10 records a request. The typed flag wins |
| Question file `"batch": 1` and no flag | One record a request |
| Question file `"batch": 1` with `tt.Engine(batch=10)` | One record a request. The file beats the engine setting. A per-call `batch=10` beats the file |
| `--batch 10` with `--jobs 1` | Same output bytes as `--jobs 8` |
| A live stream that pauses | The open batch sends after 50 ms |
| `--replay` with another `--batch` than the recording | Exit 5 at the first missing batch |
| An old folder recorded one record a request | Replays under `--batch 1`. Under the default, `--replay` stops at exit 5 and `--cache` pays again |
| One batch fails | The run stops at its first record. Earlier rows stay printed. The stop line names the range |
| One question in a reply is missing | That record fails. Earlier records of the batch print |
| A 429 | Retried as one request |
| The reader downstream closes the pipe | No new batch starts. Batches in flight may be billed |
| `rank --top 5` | Every record is judged in batches. Five print |
| DuckDB parallel scan | Batches may differ between runs. The cache misses those batches |

## Acceptance tests

Tests 1 to 6 need no network. They use recordings and a loopback backend.

1. **Batch of one.** Every existing fixture under `specification/fixtures/systemone/` encodes byte for byte under `--batch 1` and under a stream of one record.
2. **Batch body.** New fixtures pin one plain batch of three records, one context batch, one pick-one batch, one CSV batch and one batch with a duplicate record.
3. **Grouping.** 306 lines form 31 requests. 25 lines at `--batch 10` form 10, 10 and 5. Records of 40,000 bytes close batches by bytes.
4. **Order and jobs.** A recorded 306-line `filter` prints identical bytes at `--jobs 1` and `--jobs 8`, on standard output and standard error.
5. **Replay.** A recorded run replays with no network under the same settings. Under another `--batch` it exits 5 and names the first missing batch.
6. **Shares.** For every batch in a recorded run, the rows' shares of input tokens, output tokens and requests sent sum to the batch. The `thinkthen.run/1` totals equal the sum of the replies' usage.
7. **Pause.** A test reader that stalls after 3 records makes the first batch send 3 records within 100 ms.
8. **Failure.** A loopback backend that answers 503 to the second batch stops the run at record 11 with rows 1 to 10 printed and exit 4.
9. **Accuracy, plain.** One recorded live run of 306 songs with experiment 271's key and question, three repeats, default batch: each repeat scores at least 280 right. Today scores 285.
10. **Accuracy, context.** The same songs with the catalog as `--context`: one request, and at least 297 right in each of three repeats. Experiment 271 scored 299 to 302.
11. **Pick-one.** Experiment 262's 200 `choose` items at the default batch score at least 135 of 200, experiment 262's bar.
12. **Speed.** Test 9's runs finish in under 3 seconds each. Experiment 271 measured 1.3 to 1.7 s.

Tests 9 to 12 are paid. Together they cost under $0.02 at the recorded price.

## Tickets

In order. "Needs ADR" marks a ticket that changes a Settled specification page.

| # | Outcome | Scope | Proof | Depends on | ADR |
| --- | --- | --- | --- | --- | --- |
| B0 | Ian's ruling recorded | One ADR: batching on by default, the batch shape, sizes, shares, the run line on standard error, `--context`, the question file's `batch`, `relate --jobs`, and `filter` and `rank` reading lines by default. It lands before recognize R0. It amends `records.md` "Order and requests" and `jobs`, `result.md` `meta`, `channels.md`, `question-file.md`, `backends.md` and `relate.md` | ADR accepted; pages updated | none | This is the ADR |
| B1 | Usage writes stop serializing requests | Per `2026-09-26-usage-file-writes-serialize-requests-in-flight.md` | That issue's timing at `--jobs 16` | none | no |
| B2 | The pool keeps up to `--jobs` connections | Per `2026-09-26-connection-pool-reopens-connections-above-three-jobs.md` | New connections at `--jobs 16` stay near the job count | none | no |
| B3 | The engine plans batches | Grouping, quote prefix, evidence object, duplicates, byte ceiling, batch of one, digests. No surface change | Tests 1, 2 and 3 | B0 | covered by B0 |
| B4 | The command batches `decide`, `filter` and `rank` | `--batch`, `THINKTHEN_BATCH`, the question file's `batch`, jobs over batches, order, pause, failure line, record, replay, cache, dry run | Tests 4, 5, 7 and 8 | B3 | covered by B0 |
| B5 | Run facts stay true under batches | Shares, `meta.batch`, the `thinkthen.run/1` line | Test 6 | B4 | covered by B0 |
| B6 | The default is measured with the tool's own wording | One authorized live run. If test 9 or 12 fails, the ticket stops and reports to Ian. The default stays until he rules | Tests 9 and 12 | B1, B2, B5 | no |
| B7 | Shared context | `--context FILE` for `decide`, `filter` and `rank`; no record limit with a context | Tests 2 and 10 | B4 | covered by B0 |
| B8 | `choose` batches | The pick-one question per record, and `--context` for `choose` | Test 11, plus one recorded context run reported in the ticket | B4, B7 | covered by B0 |
| B9 | `tag` and `score` measured | One authorized run per type on a labeled set of at least 200 items, at `--batch 1` and the default. Turn a type on only if it meets test 11's rule for its set | Recorded runs and the decision in the ticket | B4 | no |
| B10 | `annotate` batches per `on` group | Groups whose question types are on | A recorded question set of yes/no and pick-one questions | B8, B9 | covered by B0 |
| B11 | `relate` and split requests run at once | Relations and chunks under the throttle; `relate` takes `--jobs`. Closes `2026-09-25-relate-sends-one-chunk-at-a-time.md` | Ticket 0118's loopback count reaches the throttle | B0 | covered by B0 |
| B12 | Libraries batch and give facts | `batch`, `context`, `.facts` in Python, TypeScript, Ruby, R, C and Rust | Shared conformance cases for batch count, shares and `--batch 1` bytes | B5, B7 | Needs an ADR amending ADR 0017 for `.facts`, unless the run-facts ADR lands first |
| B13 | SQL and frames batch | DuckDB, PostgreSQL, SQLite `thinkthen_warm`, Polars and pandas; `context` argument | A DuckDB query over 306 rows sends 31 requests; conformance cases | B12 | covered by B0 |
| B14 | `filter` and `rank` read lines by default | Per `2026-09-26-filter-and-rank-could-read-lines-by-default.md` | The no-flag call reads lines; the explicit flags still work | B0 | covered by B0 |
| B15 | The site explains batching | One how-to page with the walkthrough table, the flag, the context and the replay rule | Docs review | B6, B7 | no |

## Open items Ian can overturn

1. The default of 10 records without a context, and no record limit with one.
2. The 50 ms pause and its lack of an option.
3. `--batch 1` sending today's unquoted request. Experiment 271 found the quote alone gained 5 to 7 right. A later change could quote at one record too, at the cost of every old recording.
4. Even shares of tokens across a batch, in place of shares by record size.
5. The run line on standard error under `--details`.
6. `tag`, `score` and `recognize` waiting for their measurements before they batch.
7. Leaving the evidence of plain batches in an object, where experiment 271 sent a plain list of lines.
8. Leaving replay after a pause as a known gap, in place of storing batch sizes in the recording folder.
