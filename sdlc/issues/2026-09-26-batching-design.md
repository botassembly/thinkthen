# Batching: many records in few requests

Status: Sent by Ian to the main builder on 2026-09-26. Review findings and Ian's rulings of 2026-09-26 applied.

Filed 2026-09-26. This design replaces `2026-09-25-packing-and-batching-what-they-buy-what-they-cost-and-the-setting.md`, now in `closed/`. Its evidence comes from workspace experiments 208, 260, 261, 262, 268 and 271, and from the 2026-09-22 wire probe. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` copies every table this design cites. `2026-09-26-recognize-design.md` uses the same batch rules for many short texts.

## Ian's rulings, 2026-09-26

Ian ruled on these points after he sent the design. Ian can overturn each one.

1. **Batching is automatic and maximal.** By default each request carries as many records as fit under the backend profile's limits. The goal is the fewest requests. No small fixed default such as 10 exists. `--batch N` asks for at most N records a request. `--batch 1` gives today's requests back. Section 2 states the rule.
2. **Content cuts stay.** A cut depends only on its own record, so an insert moves no cut. The cuts now fall inside fill-to-limit batches. Section 2 states the rule and what an insert re-sends.
3. **Speed wins over accuracy.** The default fills to the limit, although a full request in the tool's own form scored 272 to 274 right of 306 with 32 to 34 false yeses (evidence section 14). Today's filter scored 285 with 16 or 17, and 10 records a request scored 283 to 287. ADR 0053 item 4 records the correction. Ticket B4 lands the default. Test 9 measures and reports the accuracy cost and gates nothing. Ticket D1's page states that cost with its record.
4. **The speed target.** `filter` over the 306 songs finishes in under half a second at the default throttle, on a named build, measured live. The throttle keeps its default of 4 requests in flight, set by `--jobs`, 1 to 32. Batching is the speed lever, and the throttle is not.
5. **A speed test, ticket S1.** It measures requests, wall time and tokens for every function and every Beatles Bench job, batched and unbatched. It fails any function that sends one request per item where a fuller request fits.
6. **A documentation page, ticket D1.** One page explains batching and each backend's limits. Every number names its record and build.
7. **Run facts.** Library results carry `facts` on every call, with no setting and no second call. `--facts` controls only what the command line prints.
8. **Recognize on long texts uses a window.** `2026-09-26-recognize-design.md` section 6 gives the rule.
9. **Ticket order.** B0 comes first, then C1. The two speed defects, B1 and B2, and relate's `--jobs`, J1, follow. The engine B3, then S1, then B4 and the rest come after them.
10. **Recognize's steps, stated plainly.** One request finds and labels the names, with two questions per word. `confirm` adds a request only when touching names need separating. Relations add requests. The recognize design states this where it explains requests.
11. **Every setting is explained in one place.** Ticket C1 writes `specification/settings.md` right after B0. Any ticket that adds or changes a setting updates its row in the same commit.

## What batching is for

A user points ThinkThen at a list and wants answers now. Today each record is its own request to Jev. Each request pays about 250 input tokens of fixed overhead, by experiment 271. A one-title request bills about 290 in all. Experiment 268 measured a median round trip of 135 to 151 ms. Jev's own time was 57 to 69 ms of it, and record size barely changed it. So a list of short records spends its money and its time on overhead.

Jev takes one evidence text and a `questions` map with many questions in one request. Experiment 271 sent 7,000 questions in one request. Jev set no count limit. It refused only requests over about 65,536 input tokens. One request can therefore carry many records, one question for each.

Ian ruled on 2026-09-25 that packing is a setting every surface reads the same way. The closed packing issue records that ruling. Ian then ruled that batching is on by default. Ian's rulings of 2026-09-26 make it maximal and put speed ahead of accuracy. Batching changes answers, because a record's neighbours are in view. The design states the measured cost plainly and gives one flag to turn batching off.

## What the user sees

### Command line

Ticket 0137, in progress, makes `filter` and `rank` read one record a line with no flag. Until it lands, every `filter` and `rank` example needs `--lines`, and the examples here carry it.

The simplest call:

```sh
$ thinkthen filter 'The text is the title of a song by the Beatles. It appears on the album Abbey Road.' --lines --threshold 0.7 < songs.txt
Because
Carry That Weight
Come Together
…
```

The tool sends all 306 titles in one request. The request body holds 55,490 bytes, under the 96,000-byte ceiling, by section 9 of the evidence record.

A shared reference text, sent once:

```sh
$ thinkthen filter 'It appears on the album Abbey Road.' --lines --threshold 0.7 --context catalog.txt < songs.txt
Because
Carry That Weight
Come Together
…
```

`catalog.txt` holds one line per song with its facts. The request fills the same way, so the catalog and all 306 titles go in one request of 62,748 bytes, by section 9 of the evidence record.

Sizing batching or turning it off:

```sh
$ thinkthen filter '…' --lines --batch 1 < songs.txt     # one record a request, today's exact requests
$ thinkthen filter '…' --lines --batch 10 < songs.txt    # at most 10 records a request
```

Run facts with `--facts`: one summary line goes to standard error at the end. A finished run without `--facts` prints nothing there, as today.

```sh
$ thinkthen filter '…' --lines --threshold 0.7 --facts < songs.txt > kept.txt
{"schema":"thinkthen.run/1","records":306,"requests_sent":1,"cache_answers":0,"input_tokens":11900,"output_tokens":5500,"seconds":0.4,"model":"jev-1.13.0"}
```

The line's numbers are illustrative.

### Python

```python
import thinkthen as tt

kept = tt.filter("The text is the title of a song by the Beatles. It appears on the album Abbey Road.", songs)
kept[:3]
# ['Because', 'Carry That Weight', 'Come Together']
kept.facts
# Facts(records=306, requests_sent=1, cache_answers=0, input_tokens=11900, output_tokens=5500, seconds=0.4, model='jev-1.13.0')

kept = tt.filter("It appears on the album Abbey Road.", songs, context=catalog)   # a reference text, sent once
kept = tt.filter("It appears on the album Abbey Road.", songs, batch=1)          # one record a request

on_abbey_road = tt.question(decide="It appears on the album Abbey Road.", threshold=0.7)   # a stricter bar
kept = tt.filter(on_abbey_road, songs, context=catalog)
engine = tt.Engine(batch=1)
```

A plain string is the question, as today. It uses the default cut of 0.5, so it keeps more titles than the 0.7 walkthrough below. Its output lines and facts are illustrative. A verb over many records returns a list, as today. Every library result carries `.facts` on every call, with no setting and no second call. `2026-09-26-every-surface-should-give-back-run-facts.md` asks for these run facts. `recognize` carries the same `Facts` fields. The other libraries take `batch` and `context` under their own spelling and return the same shape.

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
| `--batch 1`, today's requests | 306 | 13.0 to 14.2 s | 88,933 | $0.0037 | 285 | 29 to 31 | 16 to 17 |
| `--batch 10` | 31 | 1.3 to 1.7 s | 19,656 | $0.0008 | 283 to 287 | 35 to 39 | 18 to 22 |
| Default, all 306 in one request | 1 | 0.30 to 0.39 s | 11,913 | $0.0005 | 276 to 278 | 44 to 46 | 27 to 29 |
| Default, the tool's own form, measured by a review | 1 | 0.25 to 0.27 s | 13,185 | $0.0006 | 272 to 274 | 50 to 52 | 32 to 34 |
| `--context catalog.txt` | 1 | 0.36 s | 22,091 | $0.0009 | 299 to 302 | 17 to 19 | 2 to 3 |

Experiment 271 measured every row, three runs each, at filter's cut of 0.7 and 4 requests in flight. The first row ran through today's `thinkthen filter`. The other rows used a Python client with experiment 271's own question wording. Its tens ran in alphabetical order. On this list the section 2 rule forms the same 31 batches at `--batch 10`, because no title's hash is a content cut. Cost uses the recorded input price of $0.042 a million tokens. Output tokens are free under the vendor's price list. Ticket S1 measures the tool's own wording and time again, on a named build.

The tool's own form row comes from evidence section 14. It sent the planner's exact body through `annotate`, three runs, because the command could not batch yet. It is the figure to quote for the default until test 9 runs.

The user types the first command and gets 50 to 52 titles in about a quarter of a second, by the tool's own form in evidence section 14. Today they get 29 to 31 in 14 seconds. The full request made the filter more generous. It missed no Abbey Road song where today misses 4 or 5, and it kept 32 to 34 songs from other albums where today keeps 16 or 17. Ian ruled that speed wins, so the default accepts that cost, and the page states it. A user who wants today's answers types `--batch 1`. The same user adds `--context catalog.txt` and gets 17 to 19 titles in a third of a second. Only 2 or 3 of them are wrong.

### Friction list

| What a user must know | What this design does |
| --- | --- |
| That batching exists | Designed away. It is on by default for every stream |
| How many records to batch | Designed away. Each request fills to the backend's limits |
| That a live stream could wait for a full batch | Designed away. A live batch sends after a 50 ms pause in input, in every mode |
| How to share a reference text | One option, `--context FILE` |
| How to get today's exact answers | One flag, `--batch 1` |
| That old recordings and caches answer only under `--batch 1` | Kept. A batch is a new request. Resending costs cents, and `--batch 1` replays old folders |
| That a record's answer can change with its neighbours, and that one record can steer them | Kept, as Ian ruled. `--batch 1` keeps records apart. Test 9 measures the change and a planted claim, and ticket D1's page states both |
| That a threshold tuned at one batch setting may not fit another | Kept. The run warns when the settings differ, and so do `audit` and `diff` |
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

The batch setting is `max` by default, or the whole number `N` that `--batch N` sets. The tool adds records to the open batch in input order. A batch closes after a record when the first of these holds:

| Rule | When it closes |
| --- | --- |
| Content cut | The record's content hash is 0 mod 4,096 |
| Size | The setting is a number `N`, and the batch holds `N` records |
| Member cap | The batch holds 4,096 members, repeats included (ADR 0053 item 2) |
| Profile limit | The next record would put the request over `max_questions`, `max_evidence_bytes` or `max_request_bytes`. At the built-in address the 96,000-byte ceiling stands in for `max_request_bytes` |
| Pause | A live run has waited 50 ms with no new record, in every mode |
| End of input | Always |

The content hash is the SHA-256 of the record's evidence in compact JSON, the same bytes its question quotes. Its first 8 bytes, read as a big-endian unsigned integer, give the value taken mod 4,096. A context does not change the rule. At `N` of 1 every record closes its batch, so `--batch 1` sends today's requests.

**What an insert moves.** A content cut depends only on its own record. Cuts split the input into stretches, and batches fill from the start of each stretch. An insert moves no existing cut. The cases:
- An inserted, removed or changed record that is not a cut changes the batch that holds it and every later batch of its stretch. Batches before it keep their bytes. Every batch after the next cut keeps its bytes.
- Cut positions are memoryless. The next cut after any record lies on average 4,096 records ahead. So an insert re-sends about 4,096 records on average, a full stretch.
- A list shorter than a few thousand records usually holds no cut. There an insert re-sends every batch after it.
- An inserted record that is a cut splits its stretch in two. The batches after it re-form up to the next cut.
- A removed cut merges two stretches. A changed record that becomes or stops being a cut does the same. The batches from the merged or split point re-form up to the next cut.

**Why fill to the limit.** Ian ruled that speed wins over accuracy and that batches fill to the limit. On 306 songs one full request in the tool's own form answered in 0.25 to 0.27 s for 13,185 input tokens (evidence section 14). Today's filter took 13.0 to 14.2 s for 88,933. The same request scored 272 to 274 right against today's 285, and its false yeses rose from 16 or 17 to 32 to 34, by ADR 0053 item 4. Experiment 208 found a larger cost on longer records: 1,000 SMS messages at 40 a request scored 0.882 accuracy and 0.434 recall, against 0.968 and 0.941 one a request. Test 9 measures the tool's own cost, and ticket D1's page states it.

**Why 4,096.** A content cut costs at most one partly filled batch. The plain body over the 306 titles holds about 181 bytes a title, by section 9 of the evidence record. A request at the ceiling then holds about 529 short titles. One cut in 4,096 records therefore adds at most one request to about 8 full ones. A table of 100,000 short titles takes about 189 full requests and about 24 partial ones. At 4 in flight and an estimated half a second a request, that run takes an estimated half a minute. No title of the 306 falls on a cut at 4,096, so the list goes in one request. At 1,024 the 72nd title would cut it into two requests.

**Why 96,000 bytes.** The ceiling already exists in `specification/backends.md`. It holds under the token limit at relate's measured 0.516 tokens a byte. Dense text runs higher. A hex record measured 0.895 tokens a byte and an identifier list 0.908, and a batch of two dense records under the ceiling was refused (evidence section 14). ADR 0051's one halving rescues that case. It needs no tokenizer. Record text measured 0.40 tokens a byte in experiment 268. Ticket B6 measures the rate on batched record text again. A record whose batch of one already passes the ceiling goes alone as today's request.

**Other addresses.** ADR 0051 amends this rule. The request-size setting, 96,000 bytes by default, closes batches at every address. A profile can lower that size. A too-large refusal of a batch of at least two records halves once; a half that still fails exits 4. The caller can lower `--max-request-bytes` or use `--batch 1`.

**Replies.** Ticket 0132's reply limit, 1 MiB plus 8 bytes per request byte, fits a batch. A reply runs about 105 bytes a question, and each quoted question adds more than that to the request.

**Why 50 ms.** Experiment 268 measured Jev's own time near 60 ms and a median round trip near 140 ms. A 50 ms wait adds about a third of one round trip to a live record. The pause rule fires only when the reader is waiting for input. It fires in every mode, by ADR 0053 item 1, so a loop that writes one record and waits gets its answer under a named folder too. A file or a fast pipe never pauses. A live pipe recorded under a folder forms batches by its timing, and a replay with other timing can miss a batch at exit 5. The value is fixed and has no option.

### 3. Order, jobs, cache and replay

- **Order.** `decide` and `filter` keep input order. A batch's output rows print when its reply arrives and every earlier output row has printed. `rank` waits for the complete run, then sorts by probability with input order breaking ties.
- **Jobs.** `--jobs N` means N batches in flight. It takes 1 to 32, and the default stays 4, as `specification/records.md` fixes. The in-order completion buffer holds at most N batches; `rank` also holds judged records until it can sort the complete run. The throttle is not the speed lever. A full batch is.
- **Cache key.** The cache key is the batch's request digest, the same digest rule as today. A record's answer depends on its neighbours, so a per-record key would serve an answer made beside other records.
- **Replay.** The same records under the same settings form the same batches and the same digests, because a file never pauses. `--replay` then answers every batch from disk. An inserted, removed or changed record changes the batches section 2 names. `--cache` pays only for those batches. `--replay` stops at the first missing batch at exit 5.
- **Settings that change batches.** `--batch`, `--context`, the question, the pointers, a profile's limits and the records themselves. `--jobs` changes no batch.

### 4. Failure

One request carries one batch, so one failed request fails every record in it. The run stops at the batch's first record, as `specification/records.md` stops at the first failed record. `decide` and `filter` keep rows already printed; `rank` withholds rows on a stop. The stop line names the range and echoes no record:

```text
thinkthen: stopped at record 41; the request for records 41 to 50 failed: the backend answered with status 503; 40 records finished
```

A reply that answers some questions and not others fails only the records with missing or bad answers. The run stops at the first such record, and `decide` and `filter` print the earlier records of that batch, while `rank` withholds them on a stop. `annotate` keeps its exit 6 rule for a failed question beside good answers.

A retried status (429, 500, 502, 503, 504 or 529) resends the whole batch as one request, as `specification/backends.md` fixes. Ticket 0089's premise holds here: a retried status means the backend answered, and it may have billed the first attempt. A batch can therefore be paid twice, as a single record can today. ADR 0051 amends this: a batch of at least two records refused as too large halves once. Other retried statuses still resend the whole batch.

### 5. Which functions batch

Every function that can batch does so by default. Its measured accuracy cost is reported and stated, and it gates nothing.

| Function | How it batches | Default |
| --- | --- | --- |
| `decide`, `filter`, `rank` over records | One yes/no question per record | Fill to the limit, from B4. Measured in 208, 260 and 271 |
| `choose` over records | One pick-one question per distinct selected-record and complete-question pair; ordered options stay with their row | Fill to the limit, from B8. Experiment 262 measured an earlier form at 10; test 11 measures this build |
| `tag` over records | One yes/no question per record and label | Fill to the limit, from B9. B9 measures it |
| `score` over records | One levels question per record | Fill to the limit, from B9. B9 measures it |
| `annotate` over records | The records of one `on` group share a request, each question quoting its record | Fill to the limit, from B10 |
| `find` | Already sends its whole set in one request | Unchanged |
| `recognize` over many short texts | As `2026-09-26-recognize-design.md` section 7 gives it | Fill to the limit, from recognize R7 |
| `relate` | Does not batch. Each relation's request state differs, so relations cannot share a request. Ticket J1 runs its relations and split requests at once | Unchanged |
| Any verb on one document | Nothing to batch | Unchanged |
| A question file whose question text is a JSON object or list | The quote prefix needs a text question. Records themselves may be objects | One record a request |

### 6. Where the setting lives

| Surface | Batch setting | Context |
| --- | --- | --- |
| Command | `--batch N` or `--batch max` | `--context FILE` |
| Environment | `THINKTHEN_BATCH` | none |
| Question file | `"batch": N` or `"batch": "max"` | none |
| Python, TypeScript, Ruby, R, C, Rust | Engine setting and a per-call `batch` | Per-call `context` |
| Polars and pandas columns | As the library | As the library |
| DuckDB | `SET thinkthen_batch = N` | A final `context` argument on each scalar |
| PostgreSQL | `SET thinkthen.batch = N` | A final `context` argument |
| SQLite | The settings call the extension already has | A final `context` argument |

`N` is a whole number of at least 1. A number larger than fits fills to the limit. `max` is the default. It exists as a spelling so a caller can undo a question file's number.

Ian ruled the order: the typed value, then the environment, then the question file, then the default. It matches ADR 0007's profile order: the flag, `THINKTHEN_PROFILE`, the file's `profile`, then the built-in one. The typed value is `--batch` or a per-call `batch=`. The environment tier holds `THINKTHEN_BATCH`, the engine setting and the SQL `SET`. `specification/question-file.md` line 98 rules the command line, then the file, then the default. B0 amends that section for `batch` and adds `--batch` to line 100's list. B0 also adds `batch`, a whole number of at least 1 or `"max"`, to `question-file.schema.json`. Ticket C1's `specification/settings.md` carries the row.

`batch` stays out of the question digest. A request cap such as the SQL extensions' request total counts a batch as one request. The context is evidence. It leaves the machine, and it enters the request digest but not the question digest.

The SQL and frame surfaces batch the rows one call receives: a DuckDB vector of up to 2,048 rows, a PostgreSQL array, a data-frame column, or SQLite's `thinkthen_warm`. Rows with equal evidence are asked once, as DuckDB does today. Batches form by the same rule within the call. A vector's edge also closes a batch, so DuckDB can cut a table differently from the command. A plain SQLite scalar receives one row at a time and cannot batch. DuckDB's parallel scan can hand rows over in another grouping. Exact replay then needs a fixed row order, such as `SET threads = 1`.

### 7. The batch setting is calibration identity

A threshold tuned at one batch setting may not fit another, because batching shifts probabilities. In the tool's own form, false yeses rose from 16 or 17 to 32 to 34 with all 306 titles in one request (evidence section 14). The batch setting therefore joins the threshold's calibration identity under ADR 0032.

- The question file's `batch` names the setting its threshold was tuned at, as its `profile` names the backend.
- A run whose batch setting differs from the file's `batch` carries `meta.batch_warning` as `{"tuned_for":1,"running":"max"}`. It prints `threshold tuned at batch 1 is running at batch max` once on standard error, as ADR 0032's profile warning does. A file with a `threshold` and no `batch` counts as tuned at batch 1, by ADR 0053 item 3. A file with neither warns nobody.
- `audit --write` writes the graded runs' batch setting into the file's `batch` beside the threshold.
- `audit` and `diff` print one warning line on standard error when the runs they compare carry different batch settings. `diff` then prints at most three warning lines.

The calibration identity's batch part stays out of the question digest, by Ian's ruling. ADR 0032 puts `profile` in the digest. B0 records that difference.

## Output and run facts

Bare output does not change.

Every per-record count is a share of its batch. The tool splits the batch's input tokens, output tokens and requests sent evenly across its records and gives any remainder to the earliest records. Shares therefore sum exactly to the batch across all judged records. Printed rows can omit records filtered out or dropped by `rank --top`; `--facts` retains whole-run totals. Under `--details` a batched record's `meta` carries:

```json
"meta":{"usage":{"input_tokens":63,"output_tokens":18},"requests_sent":1,"cached":false,"requests":["6b1f…c4"],"batch":{"setting":10,"records":10,"position":1,"closed":"size","usage":{"input_tokens":624,"output_tokens":175},"requests_sent":1}}
```

The row above is the first of a 10-record batch under `--batch 10` that billed 624 input and 175 output tokens. Rows 1 to 4 carry 63 input tokens and rows 5 to 10 carry 62. Rows 1 to 5 carry 18 output tokens and rows 6 to 10 carry 17. The other nine rows carry `requests_sent` 0. `meta.batch.setting` is the run's batch setting, a number or `"max"`. `records` counts the records in this batch. `position` is the record's place in its batch. `closed` says why the batch closed: `content`, `size`, `limit`, `pause` or `end`. `meta.batch` is absent when the batch holds one record and no context. So `--batch 1` rows keep today's bytes. `requests` holds the batch digest. A run with a context adds `meta.context_sha256`, the SHA-256 of the context file's bytes. When per-request time lands through `2026-09-23-record-the-backends-own-time-for-each-call.md`, a batched row reports its batch's time under `meta.batch`.

A usage field the backend did not report stays absent in a row's share. It is never written as 0.

### Run facts

Library results carry `facts` on every call, with no setting and no second call. `--facts` controls only what the command line prints.

Ian ruled that a finished command run stays silent on standard error by default, as `specification/records.md` line 109 says. The one name is `facts` on every surface:
- The command takes `--facts`. At the end of the run it writes one `thinkthen.run/1` line to standard error, finished or stopped. `--facts` works with or without `--details`. Without it the command prints no facts.
- Every library result carries `.facts`, or the surface's own spelling of it, on every call. A single call's facts have the same fields with `records` of 1. The run-facts ADR picks how a bare-value call carries them. It does not make them optional.
- SQL per-call facts are deferred by name in `2026-09-26-every-surface-should-give-back-run-facts.md`. `thinkthen_usage()` keeps its process totals.

The fields are `records`, `requests_sent`, `cache_answers`, `input_tokens`, `output_tokens`, `seconds` and `model`. The names match `thinkthen status` and the library counters. `requests_sent` counts every HTTP attempt, as `meta.requests_sent` does. `cache_answers` counts requests answered from a cache, as `status` counts them. A row answered that way carries `meta.cached` true. `input_tokens` and `output_tokens` sum the usage of live replies, as `status` does. Each is present only when every live reply in the run reported it. With no live reply, or a reply that reported none, the field is absent. It is never 0 for a count nobody reported. `seconds` is wall time. It never enters a byte-identity check.

`filter --details` prints only kept records, so its rows undercount the run. `--facts` counts every record and request.

`--dry-run` prints the plan for the first batch in place of the first record. It reads until the first batch closes by content, size, limit or end of input, and never waits on a pause.

## Edge cases

| Case | Expected behaviour |
| --- | --- |
| Empty stream | No output, no request |
| One record | Today's exact request |
| 306 titles, default | One request of 306 records, 55,490 bytes |
| 306 titles, `--batch 10` | 31 requests: 30 of 10 and one of 6 |
| A line inserted into a recorded list | Its batch and the later batches of its stretch miss the cache. Batches before it and after the next content cut replay |
| A record whose batch of one passes 96,000 bytes | Sent alone as today's request |
| A record that passes Jev's evidence limit | The backend refuses it at exit 4 with today's `max_tokens_exceeded` message |
| A context whose request with the question and no record passes a request or profile limit | Exit 2 before any request. The message names the binding limit and size, and echoes no text |
| A later record whose batch of one with the context passes a request or profile limit | Exit 2 at that record. Earlier batches are sent; `decide` and `filter` print completed rows, while `rank` withholds them on a stop. The refused record sends nothing. ADR 0087 |
| Two equal records in one batch | One question. Both get its answer. Both print where kept |
| A record holding quote marks or a newline | JSON escapes inside the quote |
| A CSV row | Quoted as its compact JSON object |
| A question file whose question text is a JSON object or list | One record a request |
| `--batch 1 --context catalog.txt` | Each request carries the context and one quoted record |
| `--batch 50 --context catalog.txt` | At most 50 records a request |
| `--batch 0` or `--batch fill` | Usage error at exit 2 |
| Question file `"batch": 1` with `--batch 10` | At most 10 a request. The typed flag wins. The tuned-for warning prints |
| Question file `"batch": 1` with `THINKTHEN_BATCH=max` | Fill to the limit. The environment beats the file. The warning prints |
| Question file `"batch": 1` and no other setting | One record a request |
| Question file `"batch": 1` with `tt.Engine(batch=10)` | At most 10 a request. The engine setting sits in the environment tier. The warning shows in `meta.batch_warning` |
| `--batch 10` with `--jobs 1` | Same output bytes as `--jobs 8` |
| A live stream that pauses, with or without a folder | The open batch sends after 50 ms |
| A stream that pauses under `--cache DIR`, `--record DIR` or `--replay DIR` | The open batch sends after 50 ms, as in a live run |
| 10,000 records of 5 distinct values, none a content cut | Three batches of 4,096, 4,096 and 1,808 members |
| `--replay` with another `--batch` than the recording | Exit 5 at the first missing batch |
| An old folder recorded one record a request | Replays under `--batch 1`. Under the default, `--replay` stops at exit 5 and `--cache` pays again |
| One batch fails | The run stops at its first record. `decide` and `filter` keep earlier printed rows; `rank` prints none. The stop line names the range |
| One question in a reply is missing | That record fails. `decide` and `filter` print earlier records of the batch; `rank` withholds them |
| A recorded reply missing one answer | Every replay fails at that record the same way. Replay never resends to fill it |
| A 429 or 5xx | The whole batch is resent as one request. After a 5xx the backend may bill both attempts |
| An interrupt mid-batch | No new batch starts. Batches in flight finish and are billed. `decide` and `filter` print finished rows in order; `rank` withholds a partial order, as `specification/channels.md` fixes for SIGINT |
| The reader downstream closes the pipe | No new batch starts. Batches in flight may be billed |
| `rank --top 5` | Every record is judged in batches. Five print |
| DuckDB parallel scan | Batches may differ between runs. The cache misses those batches |

## Acceptance tests

Tests 1 to 8 and 13 need no network. They use recordings and a loopback backend.

1. **Batch of one.** Every existing fixture under `specification/fixtures/systemone/` encodes byte for byte under `--batch 1` and under a stream of one record.
2. **Batch body.** New fixtures pin one plain batch of three records, one context batch, one pick-one batch, one CSV batch and one batch with a duplicate record.
3. **Grouping.** A 25-line fixture holds two lines whose content hash is 0 mod 4,096. Its README names them and works out every batch by hand with `sha256sum`, at the default under a profile whose `max_questions` is 8, and at `--batch 5`. Records of 40,000 bytes close batches at the ceiling. The same fixture with one line inserted changes only the batches the README names: the insert's batch and the later batches of its stretch. A second insert adds a line whose hash is itself 0 mod 4,096. It splits its stretch, and the README names the batches that re-form up to the next cut.
4. **Order and jobs.** A recorded 306-line `filter` at `--batch 10` prints identical bytes at `--jobs 1` and `--jobs 8`, on standard output and standard error, without `--facts`. With `--facts`, the two standard error lines match once `seconds` is removed.
5. **Replay.** A recorded run replays with no network under the same settings. Under another `--batch` it exits 5 and names the first missing batch.
6. **Shares.** For every batch in a recorded run, the rows' shares of input tokens, output tokens and requests sent sum to the batch. The `--facts` totals equal the sum over live replies. A replayed run's facts carry no token fields.
7. **Pause.** A real pipe carries 3 records from a writer that then stops and holds the pipe open. The loopback backend receives a first request of 3 records within 5 seconds. The same pipe under `--cache DIR` and under `--record DIR` sends the same request within 5 seconds.
8. **Failure.** A loopback backend answers 503 to the second batch's request after the retries. The run stops at that batch's first record with exit 4. `decide` prints every earlier completed row, `filter` prints earlier kept rows, and `rank` prints none because it cannot sort a partial run.
9. **Accuracy cost, recorded live.** Run `decide --lines --details` with experiment 271's key and filter's question at a cut of 0.7 over the 306 songs. `decide` prints every record, so `audit` sees every answer. Make three runs at `--batch 1`, three at `--batch 10` and three at the default. Add one run at `--batch 10` and one at the default over each of three shuffled orders with fixed seeds, as local experiment 275 ran them. The default and `--batch 10` arms send ADR 0055's fixed evidence sentence, with each record quoted inside its own question. The ticket reports each batched setting's three same-bytes repeats and its three shuffled orders side by side. For each, it reports right answers, false yeses, misses, the mean calibration error from `thinkthen audit`, and the answers that cross the cut. It gates nothing. Ticket D1's page states the cost with this record. For comparison, ADR 0055's table for the title task "It appears on the album Abbey Road" gives 286 right at one title a request, 266 to 277 in the records-list form, and 283 to 287 in the quoted form. Its one measured loss is the release-year title task "It was released before 1965". There the records-list form scored 276 to 283 in the table's own order and 258 to 270 over shuffled orders. The quoted form scored 255 to 259, and one title a request scored 253. One more arm adds one planted false claim at the end of the 306 titles at the default, and reports how many other answers crossed the cut. Under ADR 0055 item 5 the planted record sits only in its own question, so the arm checks that claim and gates nothing.
10. **Accuracy, context.** The same songs with the catalog as `--context`: one request, and at least 297 right in each of three repeats. Experiment 271 scored 299 to 302.
11. **Pick-one.** Experiment 262's 200 `choose` items at the default and at `--batch 10`. The ticket reports both scores against experiment 262's bar of 135 of 200. It gates nothing.
12. **Speed.** Ticket S1 owns the speed target: `filter` over the 306 songs in under half a second at the default throttle, on a named build, measured live.
13. **Calibration identity.** A question file with `"batch": 10`, run from a recording at `--batch 1`, prints the tuned-for warning once and carries `meta.batch_warning`. `audit` and `diff` over two runs at different batch settings each print their warning. `audit --write` writes the runs' batch setting into the file.

Tests 9 to 11 are paid. Together they cost under $0.03 at the recorded price.

## Tickets

In order. "Needs ADR" marks a ticket that changes a Settled specification page.

Any ticket that adds or changes a setting updates that setting's row in `specification/settings.md` in the same commit.

| # | Outcome | Scope | Proof | Depends on | ADR |
| --- | --- | --- | --- | --- | --- |
| B0 | Ian's rulings recorded | One ADR: the batch shape, fill to the limit with content cuts at 0 mod 4,096, `--batch N` and `max`, shares, `--facts`, library `facts` on every call, `--context`, `meta.context_sha256`, the question file's `batch` and its precedence, the batch setting as calibration identity, and speed ahead of accuracy. It lands before recognize R0. It amends ADR 0007 line 103, "each record is its own request, and records never share model context". It amends ADR 0008's request table and its sentence "Two pieces of evidence never share a request", as ADR 0010 accepted them, and ADR 0010's `--jobs` as requests in flight. It amends ADR 0040's "Records … are never combined" and ADR 0032's rule that calibration identity enters the question digest. It amends the `roadmap.md` rows that keep `--context FILE` and packing out for isolation, `records.md` "Order and requests" and `jobs`, `result.md` `meta`, `channels.md`, `question-file.md` precedence and its schema, and `backends.md` | ADR accepted; pages updated | none | This is the ADR |
| C1 | Every setting is explained in one place | A new `specification/settings.md`, described below. It needs no batching code | Its lint check with all three planted failures | B0 | no |
| B1 | Usage writes stop serializing requests | Per `2026-09-26-usage-file-writes-serialize-requests-in-flight.md` | That issue's timing at `--jobs 16` | none | no |
| B2 | The pool keeps up to `--jobs` connections | Per `2026-09-26-connection-pool-reopens-connections-above-three-jobs.md` | New connections at `--jobs 16` stay near the job count | none | no |
| J1 | `relate` and split requests run at once | One text's split requests and one relate's relations run under the engine's throttle. `relate` takes `--jobs`. Closes `2026-09-25-relate-sends-one-chunk-at-a-time.md`. It comes early because relate is the furthest from fast today | Ticket 0118's loopback count reaches the throttle | none | Needs an amendment. `relate.md` says the command sends its requests in order and refuses `--jobs`. Amend it |
| B3 | The engine plans batches | Fill to the limit, content cuts, size, profile limits, ceiling, quote prefix, evidence object, duplicates, batch of one, digests. No surface change | Tests 1, 2 and 3 | B0 | covered by B0 |
| S1 | The speed test | Described below | Its gate part passes with its list of functions still to batch; its live part reports on a named build | B1, B2, J1, B3 | no |
| B4 | The command batches `decide`, `filter` and `rank`, filling to the limit by default | `--batch`, `THINKTHEN_BATCH`, the question file's `batch`, precedence, jobs over batches, order, pause, failure line, record, replay, cache, dry run. It removes `decide`, `filter` and `rank` from S1's list | Tests 4, 5, 7 and 8; S1's gate part | B3, S1 | covered by B0 |
| B5 | Run facts stay true under batches | `meta.batch`, `--facts` and the `thinkthen.run/1` line. Ticket 0146 (B4) builds the shares | Test 6 | B4 | covered by B0 |
| B16 | The batch setting is calibration identity | `meta.batch_warning` and its line, the `audit` and `diff` warnings, `audit --write` writing `batch` | Test 13 | B5 | covered by B0 |
| B6 | The accuracy cost is measured and reported | One authorized live run with the tool's own wording, and the token rate on batched record text. The ticket reports the cost. It does not change the default | Test 9 | B1, B2, B5, B16 | no |
| B7 | Shared context | `--context FILE` for `decide`, `filter` and `rank`; exit 2 for a context over the ceiling; `meta.context_sha256`. B7 picks the late-overflow policy of ticket 0144's deferred gap 5 and rewrites ADR 0048 item 11 to match | Tests 2 and 10 | B4 | covered by B0 |
| B8 | `choose` batches | The pick-one question per record, and `--context` for `choose`. It removes `choose` from S1's list | Test 11, plus one recorded context run reported in the ticket | B4, B7 | covered by B0 |
| B9 | `tag` and `score` batch | One authorized run per type on a labeled set of at least 200 items, at `--batch 1` and the default. The ticket reports the cost. It removes `tag` and `score` from S1's list | Recorded runs and the report in the ticket | B4 | covered by B0 |
| B10 | `annotate` batches per `on` group | Every group, filling to the limit. It removes `annotate` over records from S1's list | A recorded question set of yes/no and pick-one questions | B8, B9 | covered by B0 |

| B11 | Moved out of batching | It is ticket J1 | none | none | none |
| B12a | The Rust library batches and gives facts | `batch`, `context`, `.facts` on every call of the public API | Shared conformance cases for batch count, shares and `--batch 1` bytes | B5, B7 | Needs an ADR amending ADR 0017 for `.facts`, unless the run-facts ADR lands first |
| B12b | The C door batches and gives facts | The same, on the JSON door | The shared conformance cases | B12a | covered by B12a's ADR |
| B12c | Python batches and gives facts | The same | The shared conformance cases | B12a | covered by B12a's ADR |
| B12d | TypeScript batches and gives facts | The same | The shared conformance cases | B12a | covered by B12a's ADR |
| B12e | Ruby batches and gives facts | The same | The shared conformance cases | B12a | covered by B12a's ADR |
| B12f | R batches and gives facts | The same | The shared conformance cases | B12a | covered by B12a's ADR |
| B13a | Polars columns batch | A column is one call | The shared conformance cases | B12a | covered by B0 |
| B13b | pandas columns batch | A column is one call | The shared conformance cases | B12c | covered by B0 |
| B13c | DuckDB batches | Vectors, `SET thinkthen_batch`, the `context` argument | A DuckDB query over the 306 rows sends the command's 1 request at `SET threads = 1`; conformance cases | B12a | covered by B0 |
| B13d | PostgreSQL batches | Arrays, `SET thinkthen.batch`, the `context` argument | The shared conformance cases | B12a | covered by B0 |
| B13e | SQLite batches | `thinkthen_warm`, the settings call, the `context` argument | The shared conformance cases | B12a | covered by B0 |
| B14 | Dropped | Ticket 0137 makes `filter` and `rank` read lines by default | none | none | none |
| B15 | Replaced | It is ticket D1 | none | none | none |
| D1 | The documentation page | Described below | Docs review; every number on the page names its record and build | B6, B7, S1, ticket 0137 | no |

Ticket 0216 lands independently reviewed offline B10 code at accepted source `efa2cd9e`, with request-aligned `meta.batches` under accepted ADR 0092. The separate recorded yes/no and pick-one comparison remains. S1 keeps its annotate function row and B10 exception until that comparison supplies reviewed evidence; a code build alone does not close them.


### C1: the settings reference

Ian ruled on 2026-09-26 that every setting is explained in one place. C1 builds `specification/settings.md`. It needs no batching code, so it can land before batching.

- One precedence rule at the top, in prose above the table: the typed value, then the environment, then the question file, then the default. The profile's place in that order comes from ADR 0007.
- One table with fixed columns, in this order: setting, what it does, default, allowed values, then one column per surface. The surface columns are the command flag, the environment variable, the question-file key, Rust, Python, TypeScript, Ruby, R, C, DuckDB, PostgreSQL and SQLite. Prose stays outside the table. The website generates its Settings page from the table at build time.
- One row per setting. The rows cover today's settings: the threshold, the throttle with its default of 4, set by `--jobs`, 1 to 32, the timeout, retries, the cache and recording folders, the profile and the backend address, and the key's environment variable. They cover every setting these designs add: `batch`, `context` and `facts`, and recognize's `keep`, `infixes`, `boundary`, `window` and hard cap. Worked examples under the table show `'s` possessives with `keep` and hyphens with `infixes`.
- A cell reads "not on this surface" where the setting does not apply. A row for a setting that a design adds names the ticket that brings it.
- A lint check keeps the page complete, and runs from a rung. It checks the column headers and their order. Every command flag in the help needs a row. Every `THINKTHEN_` environment name the product code reads needs a row. Names only tests read, such as `THINKTHEN_TEST_*`, sit on one named list in the check. A row for a setting that no longer exists fails. The check plants three failures: a missing row, a stale row and a moved column.

### S1: the speed test

Ian ruled on 2026-09-26 that speed wins, and set the target. S1 measures it.

- For every function and every Beatles Bench job, it measures requests sent, wall time, and input and output tokens, batched at the default and unbatched at `--batch 1`.
- It reports `recognize` by step: finding and labelling names, `confirm`, and relations. It reports `annotate` by its requests for each field.
- Its gate part needs no network. A loopback backend counts requests. It fails any function that sends one request per item where a fuller request fits. Functions not yet batched sit on one named list, each with its ticket. Each batching ticket removes its entry. An entry whose ticket has landed fails.
- Its live part runs by hand from `sdlc/scripts/live` under a token cap, with Ian's authorization. It records the target: `filter` over the 306 songs finishes in under half a second at the default throttle of 4, on a named build. The record names the build.
- It measures whether a large shared context adds time, at the sizes a filled request really carries, up to the byte ceiling. It runs the 306 songs with and without the catalog as `--context`, and with contexts that fill the request near the ceiling. Experiment 268 measured the service's own time at 57 to 61.5 ms up to about 1,000 context tokens and 69 ms at 2,585, with a fitted slope of 0.4 ms a thousand tokens for the service and 0.7 ms for the whole request. Nothing above 2,585 context tokens was measured one request at a time, so a catalog of tens of thousands of tokens is unmeasured. Experiment 271 adds one data point, not a slope: the whole catalog as context in one request, 22,091 input tokens, answered in 0.36 s. Without the catalog the same 306 titles took 0.30 to 0.39 s. Sections 6 and 7 of the evidence record hold the tables. No page or slide claims a context-time figure until S1 has measured it.
- Ticket 0145 builds S1. S1 lands on its gate part and its review. Its baseline live run, "S1 live run 1", runs on a main commit after S1 lands and before B4 builds, and B4 takes it as a precondition. B4 owns the authorized run that measures the target after B4. The coordinator ruled this on 2026-09-26, and Ian can overturn it.

### D1: the documentation page

Ian ruled on 2026-09-26 that one page explains batching. D1 writes it in `specification/` or as a how-to under ADR 0011's form.

- It says batching is automatic, and how to size it with `--batch N` or turn it off with `--batch 1`.
- It gives each backend's limits from its profile: `max_questions`, `max_evidence_bytes` and `max_request_bytes`, and the 96,000-byte ceiling at the built-in address.
- It says how a batch splits when it would pass them, and what an insert moves.
- It states the accuracy cost from test 9 and the speed from S1. Every number names its record and build.
- It states the trust model of ADR 0053 item 5: records of one batch are evidence for each other, and `--batch 1` keeps them apart.
- It says dense text can pass the token limit under 96,000 bytes, and that one halving follows.
- Its examples leave out `--lines` only after ticket 0137 lands.
- The website agent owns `site/`. D1 files an issue for the website owner to add the site page. D1 does not edit `site/`.

## Open items Ian can overturn

Ian's rulings. Ian can overturn each.

1. Fill to the limit by default, with `--batch N` as an upper bound.
2. Content cuts inside fill-to-limit batches.
3. Speed ahead of accuracy: test 9 reports and gates nothing.
4. `--facts` as the command's one option for run facts, with silence by default. Library results carry `facts` on every call.
5. The batch part of calibration identity staying out of the question digest, unlike `profile`.
6. The ticket order: B0, C1, B1, B2, J1, B3, S1, B4, then the rest.

The author's calls. Ian can overturn each.

7. `max` as the spelling of the default.
8. The cut at SHA-256 mod 4,096.
9. `tag`, `score`, `annotate` and `recognize` batching by default, with their cost measured and reported.
10. The 50 ms pause and its lack of an option. ADR 0053 item 1 withdrew its absence under a recording folder.
11. `--batch 1` sending today's unquoted request. Experiment 271 found the quote alone gained 5 to 7 right. A later change could quote at one record too, at the cost of every old recording.
12. Even shares of tokens across a batch, in place of shares by record size.
13. Leaving the evidence of plain batches in an object, where experiment 271 sent a plain list of lines.
14. DuckDB's vector edges still close batches where the command does not. Recordings therefore match across the command and DuckDB only at `--batch 1` until that is solved.
15. Recognize's 600,000-byte hard cap and its 200-word window default, from `2026-09-26-recognize-design.md`.
