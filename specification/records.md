# Records

Status: **Settled** for version one, by ADR 0007 and ADR 0010, amended by ADR 0048.

The default input is one text document. A record stream turns a command into a map over records. [channels.md](channels.md) governs the five channels, and [result.md](result.md) governs the shape of one result.

## Where the input comes from

`--input FILE` reads a file. Without a named source the tool reads standard input. `--input` names a path and never a framing.

Version 0.2 lets `decide`, `filter`, `rank`, `choose`, `score`, `tag` and `annotate` repeat `--input FILE` in argument order. Decide, filter, rank and annotate also accept positional input files after the question or question set. Positional files cannot accompany `--input`. Choose, score and tag retain positional labels and levels; they never infer a file from an existing path. `-` names a literal file. All ten accept explicit files and folders under the additive [file reader contract](files.md). Existing single-file calls retain their framing and output defaults.

Named paths and folder manifests are validated before any request. Text contents open incrementally. CSV and TSV retain their existing required-header validation before any request. Content failures still stop when the failing item is read. Each file keeps its own physical line numbers, starting at one. Pipeline labels remain global across files and include skipped physical lines.

### Several documents

Without an explicit record mode, each named file is one document on decide, choose, score and tag. A run with several files prints one JSONL carrier per file: `{"input_file":FILE,"value":VALUE}`. With `--details`, `input_file` joins the existing result members. Such a completed run exits 0, including false and null answers. A later empty document or backend failure preserves completed output and returns the actual failure code. Empty documents remain input refusals. Multi-document runs refuse `--quiet` and `--raw` before sending. One file and stdin retain their scalar output and answer exits.

### Text-line windows

`--window N` joins up to N physical text lines into each item on all ten functions under the [file reader contract](files.md). N is an ASCII whole number of at least one. This option explicitly selects line framing on default-document functions. It refuses JSONL, CSV, TSV, `--field` and resolved saved `on` pointers before sending. Annotate admits windows only for a question set without `on`.

A window never crosses a file boundary. Internal physical line feeds remain; only the final record-ending LF and an immediately preceding CR are removed. A final short window is one item. Empty files make no windows and send nothing. All-whitespace windows are skipped after size validation, and their lines still advance positions. A joined item retains the existing 16 MiB record limit.

Detailed line and document results include `position: {"file":FILE,"first":FIRST,"last":LAST}`. Stdin has a null file. First and last are inclusive physical line numbers. CSV and TSV positions remain absent. JSON file names in `position` and `input_file` use human display strings. Non-UTF-8 names replace invalid bytes with the replacement character; file access retains the original operating-system path. Locations never enter question digests, request keys or provider requests.

| Flag | What one record is |
| --- | --- |
| none | The whole input is one text document and one record |
| `--lines` | Each nonblank line is one text record. Empty lines and lines holding only white space are skipped before any request or batch. They keep their input line numbers, while "records finished" counts only records sent. A trailing newline ends the last record |
| `--jsonl` | Each line is one JSON value and one record. No blank lines |
| `--csv` | The first logical row is a header. Each later row becomes one JSON object of string cells |
| `--tsv` | The CSV rules with a tab delimiter |

The four flags are mutually exclusive. The tool never guesses the framing from a filename. It never repairs invalid JSON, never truncates a record, and never opens a file because a string looks like a path. In its default one-document mode, `annotate` alone parses valid JSON content as a JSON value and treats a JSON syntax failure as text; [annotate.md](annotate.md) gives the rule and the limits of choosing record framing explicitly.

Under `--lines` and under `--jsonl` a carriage return before the line feed is stripped with it, and a carriage return anywhere else in the line is kept.

| Command | Framing |
| --- | --- |
| `decide`, `choose`, `tag`, `score` | One document by default. All four record flags are accepted |
| `filter`, `rank` | Lines by default, or JSONL when a pointer is given. All four record flags are accepted. One document is not a stream |
| `annotate` | One document by default. All four record flags are accepted |
| `find` | Lines by default or JSONL. CSV and TSV are not options |
| `relate` | One JSON array by default. Lines, JSONL, CSV, and TSV form one complete entity set |

### CSV and TSV

CSV uses a comma and TSV uses a tab. Both require a header. Empty input is exit 2. A header without data rows is a successful empty dataset and makes no request. One UTF-8 byte-order mark is ignored only at the start of the first header name.

After that removal, a header name may preserve whitespace but may not be blank, contain a control character, or exactly duplicate another name. Comparison is case-sensitive. Header order becomes object key order. Every cell becomes a JSON string, including empty cells and text that looks like a number, boolean, null, array, or object. Every result remains JSONL.

The maintained `csv-core` grammar owns quoted delimiters, doubled quotes, quoted line feeds, irregular quote placement, LF and CRLF records, trailing empty cells, and blank physical lines outside quoted fields. Every data row must have the header's field count. `choose --options` remains JSONL-only because table cells are strings.

A header or logical data row may hold at most 16 MiB of encoded bytes before its record terminator. Quoted line feeds and doubled quotes count. The edge parses incrementally within that bound and stops after an oversized row; its unread tail never becomes another record. Header diagnostics say `the CSV header` or `the TSV header`. Data diagnostics say `the CSV record` or `the TSV record` and the stopped-run line counts data rows from one. A diagnostic never repeats a header or cell value. Invalid UTF-8 remains a local failure at exit 5; other table grammar failures exit 2.

## `--field POINTER`

`--field` is a JSON Pointer as RFC 6901 defines it. It names the part of each record the model sees. `/body` reads the `body` member, and `/a/text` reads `text` inside `a`.

**The pointer is the disclosure boundary.** Only the pointed value leaves the machine. `--details` still carries the whole record in `input`.

- `--field` with `--jsonl`, `--csv`, or `--tsv` reads the pointer in each record.
- `--field` without a record framing reads the whole input as one JSON value and takes the pointer inside it. No separate JSON framing flag exists. On `filter` and `rank` it reads JSONL instead, and so does a question file's `on`.
- `--field` with `--lines` is a usage error. A text line has no members.
- A pointer that finds nothing is an input error for that record at exit 2, before any request for it.
- `$.body`, `#/id`, a wildcard, and a negative index are refused with a message that names RFC 6901, because the tool never guesses a pointer language.
- A pointer that names an object or a list sends that JSON value as `state`. A pointer that names a string sends the text it holds, and one that names a number, `true`, `false`, or `null` sends its compact spelling as text. An empty object and an empty list are values, and a selected string that is empty or holds only white space is refused.
- Without `--field`, a JSONL, CSV, or TSV record is serialized as compact JSON and the whole record becomes the evidence, sent as text.

Invalid JSON under `--field` without a record framing is an input error at exit 2. It says `the input is not valid JSON: the JSON at line LINE column COLUMN is not one` for both standard input and `--input FILE`. The line and column come from the JSON parser. The message carries no parser text and repeats no input byte.

### Several pointers

Settled by ADR 0008 item 2, accepted in ADR 0010. `--field` takes one pointer or several. Several pointers build an evidence object, each member keyed by the last part of its pointer. Two members that would share one key are a usage error.

```sh
thinkthen decide 'The output answers the input correctly.' --jsonl --field /input --field /output < cases.jsonl
```

The evidence is then `{"input":"...","output":"..."}`. A check that must not see the gold answer names only the pointers it needs.

The request carries the evidence object as the `state` value itself, in the order the pointers were given. [backends.md](backends.md) holds the field.

`probes/09-evidence-shape/` measured the object form against text holding it on forty made-up labeled cases on 2026-09-19. Both shapes answered 40 of 40 correctly, no answer differed, the probability moved by 0.0045 on average and by 0.05 at most, and the object shape cost 13,954 input tokens against the string shape's 13,714. The check separated nothing, so the string stayed until founder ruling 7 settled the object form. A request whose `state` is an object is a new request identity, and a recording made for the string form does not answer it.

## What a record may not hold

- A JSON record that holds two members under one name is refused, because no reader can say which of the two a pointer means.
- A JSON record holding `NaN`, `Infinity`, or a number too large to be finite is refused, because none of the three is a JSON number.
- A JSON record nests at most 127 levels of arrays and objects. A deeper record is refused at exit 2 with `the JSON nests more than 127 levels of arrays and objects, the most this tool reads`, and a stream stops at it with the stopped-record line. The limit is fixed and no option sets it, because it keeps a hostile or mistaken record from exhausting the parser's stack. The command, the libraries and the C door read records through one parser, so each refuses with this sentence.
- A malformed JSONL record is refused at exit 2 with `the record is not valid JSON`. Under a typed `--jsonl` with no `--field`, a syntax failure that stops at record 1, and no other refusal, instead says `the record is not valid JSON; read a table with `--csv` or `--tsv`, and plain text with `--lines``. The stopped-record line follows as before. Neither message carries a JSON location or repeats a record byte.
- A record whose bytes are not valid UTF-8 is refused at exit 5, because bytes that are not text are a local failure rather than a record the tool read.
- A record over 16 MiB is refused at exit 2 for that record, before any request. The line that ended the record is not part of it. The number is fixed and no option sets it, because the vendor's token budget refuses evidence far smaller than that. The reader stops a little past the limit, so a stream whose line runs longer than that ends there. What follows the cut is the middle of the refused record and never a record of its own.

## Order and requests

A record batch shares one request and its evidence among its questions. `find` deliberately sends its complete bounded set as one aggregate request. On `decide`, `filter`, `rank`, `choose`, `tag` and `score`, records of one batch share one request. On `annotate`, every group's questions share the fixed state, so the groups of many records pack together, by ADR 0111 section 5; each question quotes only its own group's selected part. Every request of `decide`, `filter`, `rank`, `choose`, `tag`, `score` and `annotate` takes one quoted form, by ADR 0111 section 1, a batch of one included. Without `--context FILE` the evidence is `Each question quotes the text it asks about.`, and each record appears once, inside its own question: `The text is `, the record's compact JSON, `. `, then the question. So one record always makes the same questions, whatever else shares its request, and a record is not evidence for any other record in its batch. With `--context FILE`, every batch sends that file's exact text once as shared evidence and quotes each record only in its question. The context leaves the machine and changes the request digest, but not the question digest. A question written as a JSON object or list cannot take the quote, so its record goes alone as the evidence. A batch asks one logical question for each distinct selected-record and complete-question pair. `tag` expands that question into one adjacent wire question per label. Equal selected records with different ordered options or descriptions receive separate questions. The questions stay the same, but records that share one request can still affect each other's answers. `--batch 1` keeps records apart and sends one record per request. Default `decide`, `choose`, `tag`, and `score` output keeps each parsed record under `input` and its answer under `value`. Output keeps input order everywhere but `rank` and `find`. `rank` sorts by yes probability for a plain or saved `decide` question, and by weighted value for a saved `score` question. Exact ties keep input order, including at a `--top` boundary; `score` itself keeps input order. No record is dropped for being not sure, except that `filter` prints only what it keeps and `find --none` prints nothing. `choose --raw` keeps printing plain labels for lines and JSONL. `filter` prints kept line and JSONL records as they arrived. CSV and TSV rows print as compact JSON objects in header order. Every output from CSV and TSV input is JSONL.

### Portable spelling of a selected batch record

A question quotes the **selected value**, after record framing and any `--field` pointer. Text becomes a JSON string. JSONL can select a JSON object, array, number, boolean or null; table cells remain strings. Write that value as one compact JSON value in UTF-8, with no whitespace between tokens and no line ending. The same spelling finds duplicate selected-record/question pairs. No surface cuts batches by content since ADR 0111.

Keep array order and every object's received member order, including nested objects. Write a quotation mark as `\"`, a backslash as `\\`, and control characters with JSON escapes. Write valid non-ASCII characters directly in UTF-8, including `é`; an input `\u00e9` that parses to that character has the same selected spelling. Do not replace it with a second optional escape form. A JSON integer remains an integer when the parser holds it as a signed or unsigned 64-bit integer. A finite decimal or exponent is held as a 64-bit float and written in a shortest round-tripping form; `1` and `1.0` remain distinct, and `-0.0` retains its sign. Numeric input lexemes beyond those distinctions need not survive parsing: for example, `1e20` is written `1e+20`. The [literal batching corpus](fixtures/batching/README.md#portable-cross-surface-corpus) pins these byte cases. Other hosts share this Rust pipeline, but they must pass the same accepted value into it; text-only bindings do not claim structured-object parity.

The cache identity of each question is its key, by ADR 0111 section 2: the SHA-256 of the adapter name, the resolved backend URL, the model, the shared state and the question as sent, joined by line feeds. A record's questions keep their keys whatever else shares their request. A batch of one takes the same quoted form, so an entry recorded under the earlier unquoted or record-list forms is never found again. A user's old cache is ignored until `thinkthen cache convert` reads it.

### How many requests each command makes

Settled by ADR 0008, accepted in ADR 0010, with the `find` exception settled by ADR 0030. One request normally carries one piece of evidence and every question asked of it. `find` sends all of its units together because choosing the best unit requires comparison across the set. On `decide`, `filter`, `rank`, `choose`, `tag` and `score`, each request fills with records up to the smaller of the request-size setting, 96,000 bytes by default, and a profile's limit, or until another batch limit closes it. N records make N requests only at `--batch 1`. `tag` counts each label as a wire question; `score` counts one wire question per record.

A live command run and a Rust library call both send the open request after 50 ms with no new record, in every mode, with or without a folder. The library runs its pipeline on a background thread, so the pause also applies while a caller's `Iterator::next()` blocks. `Max` otherwise waits for a size or profile limit, the 4,096-record cap, a full window, source exhaustion or a local refusal. An interactive Rust caller selects `BatchSetting::Records(1)` to receive each row before the library pulls the next input. A command file or a fast pipe never pauses, so it forms the same requests every run. A slow pipe forms requests by its timing. Each question keeps its own key, so moved request edges never cost a cache answer, and replay never misses for timing. A request also closes at 4,096 records, repeats included.

Adding a record asks only that record's questions, by ADR 0111: after 100 records, a run of 120 that includes them sends one request holding the 20 new records' questions. The [40-record local comparison](../sdlc/records/qf-incremental-batch-guidance.md) measured the request-level cache that ADR 0111 replaced, where one early insertion re-asked all 41 records at `max`.

A context and question that exceed the resolved request size or profile limit without any record fail at exit 2 before a request. If a later record cannot fit beside the context, the tool first sends any earlier open batch, then stops at that record with exit 2. `decide`, `filter`, `choose`, `tag`, and `score` print their completed rows; `rank` withholds them on a stop because it cannot order a partial set. The refused record sends nothing. The tool does not read ahead to check the rest of a stream. ADR 0087 records this boundary.

A threshold saved in a question file was tuned at the file's `batch`, or at 1 when the file has no `batch`. A `decide`, `filter`, `rank`, `choose`, or `tag` record run at another setting prints one warning on standard error and carries `meta.batch_warning` in each detailed row. The warning leaves the run's batch setting and the question digest unchanged.

| Command | Requests |
| --- | --- |
| `decide`, `choose`, `tag`, `score` on one document | 1 |
| `annotate` on one document | 1, because every `on` group shares one state; more when its questions pass a request or profile limit |
| `decide`, `filter`, `rank`, `choose`, `tag`, `score` over N records | One for each batch, and N at `--batch 1` |
| `annotate` over N records | One for each batch, where a batch carries every question of its records across all `on` groups; at `--batch 1`, one for each record. A record whose questions pass a request or profile limit splits across requests |
| `find` | 1 |
| `relate` | The shared relation planner's exact request count for the complete set |
| `--plan`, `--replay` | 0 |

`rank` sorts locally and makes no pairwise calls. Without a cache, every request inside one command is independent of every other. A command is therefore one round, and the round runs in parallel with output order kept. With a cache, the store keeps one answer for each question under its question key. A request carries only the questions that the store and the run's earlier requests lack, so equal questions share one backend answer. Each record still receives its own logical judgment in input order.

## Empty input

An empty line or JSONL stream succeeds with no output and no request. CSV and TSV require a header, so an empty CSV or TSV input exits 2. An empty document is a usage error, because a judgment about nothing is a mistake in the pipeline.

For `relate`, an empty line or JSONL stream and a header-only table are successful empty sets. A blank line in either stream is invalid. The default document must be a nonempty JSON array. The command validates the whole set before any request. [relate.md](relate.md) gives its independent name and kind pointers and complete-set refusals.

## Failure

Settled by ADR 0008 item 5, accepted in ADR 0010, with ADR 0104's narrow annotate exception. By default a run stops at the first failed record. Rows already printed stay printed, and the run ends with the code the failure earns: 4 for a backend failure, 5 for a local failure, 2 for a record the tool refused before sending it. `rank` prints no rows on a stopped run because its order needs the complete set. No failure ever becomes `false`, `null`, a label, or a zero. Only `annotate --jsonl --details --batch 1 --on-error continue` emits one separate versioned error row for a missing question-set `on` pointer and continues. Other failures still stop; [annotate.md](annotate.md) fixes that row and its exit.

A run that stops early prints one line on standard error with the record where it stopped and how many records it finished. A run stopped by SIGINT or SIGTERM names no record, because the next record may never have arrived. Its line reads `thinkthen: stopped by a signal; 2 records finished`. When `--record`, `--replay`, or `--cache` made recordings relevant, the line also says how many finished records came from one. A run that finishes prints nothing there. When the request for a batch of two or more records fails, the run stops at the batch's first record and the line names the range its request carried: `thinkthen: stopped at record 11; the request for records 11 to 20 failed: CAUSE; 10 records finished`. A batch refused as too large by 413 or named `max_tokens_exceeded` halves once. If its first half fails, the stop names that half's range and sends no second half. If its second half fails, `decide`, `filter`, `choose`, `tag`, and `score` print the first half's rows before the stop names the second half's range. `rank` prints no partial order. A request failure, a reply that fails as a whole, and a replay miss take the range form. When a reply answers some records of a batch and fails one, `decide`, `filter`, `choose`, `tag`, and `score` print the earlier rows and the line names that record; `rank` withholds rows: `thinkthen: stopped at record 13; the reply for records 11 to 20 gave record 13 no usable answer; 12 records finished`, at exit 4. A batch of one record and a refused record keep the cause line and the stop line. `--facts` adds one `thinkthen.run/1` line after the stop line and any usage warning. Printed output after a failure is a prefix of the input. It is not a finished dataset.

A not sure answer is never retried. In record mode the exit code reports the run, and no record's answer sets it. A completed run exits 0 unless `annotate` preserves one or more failed questions beside good answers and exits 6, or its explicit continuation emitted a missing-pointer row and exits 7. Code 8 stays reserved.

## Resume

A rerun with `--record DIR --replay DIR` on one folder answers the finished records from disk and pays only for the rest. A record whose reply was partial counts as unfinished, so the rerun asks for it again. [recording.md](recording.md) gives the folder and the entry.

`--cache DIR` means `--record DIR --replay DIR`, by ADR 0010. It is Settled. `--cache` beside either of the two options it stands for is a usage error.

```sh
thinkthen decide 'This reports a payment failure.' --jsonl --field /body --record runs/tickets --replay runs/tickets < tickets.jsonl
```

At `--batch 1`, a first run that stops at record 400 leaves 399 entries, and the same command run again replays those 399 and pays for the rest. Under batching, the store keeps one entry for each answered question, whatever batch carried it. A rerun under the same folder replays the stored answers and sends only the questions the store lacks, packed into new batches. A partial reply stores its good answers, and its failed question has no entry, so the resumed run asks only that question. A too-large batch that halves stores both halves' answers; the refused whole batch stores nothing. Under `--replay` alone, a question the folder lacks stops the run at exit 5, as [recording.md](recording.md) gives.

Each digest keeps the first complete response installed in the folder. A partial reply, one that failed a question beside a good answer, is not complete. Under ADR 0053 item 6 and its amendment, a cache does not install it, and reads an entry that holds one as a miss. Concurrent cache misses for that digest wait on one operating-system file lock. The owner checks again, sends unless a complete entry now exists, and installs the response only when it is complete. Waiters replay it. A failed or stopped owner releases the lock automatically; the next waiter sends if no entry was installed. Record-only writers remain independent, and a later writer holding another stored response stops at exit 5. The winner stays intact, so every successful run can replay the answers it printed.

## `jobs`

`jobs` sets the throttle. The throttle is the most requests in flight at once, per loaded copy of the library. The configuration file that held it left version one with ADR 0010, and [roadmap.md](roadmap.md) says so. ADR 0010 gives it the advanced option `--jobs N`, which takes a whole number from 1 to 32 and defaults to 8 (ticket 0400). The vendor's own example code uses 4 to 12 workers and says the public endpoint limits concurrency above about eight. A measured run can raise it. `annotate` also accepts `--jobs` for one document because distinct evidence groups make distinct requests. `relate` accepts it for its one entity set because its relations and split requests are distinct requests. Another command refuses it outside record mode. The split requests of one text also run under the throttle, and their answers keep request order.

The historical default of 4 could run past the vendor's documented 1,200 requests a minute on short records. Experiment 206 measured it from one machine, and `sdlc/issues/closed/2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun.md` records it. On three 200-record checks of short lines, a throttle of 4 sent 1,267, 1,319, and 1,272 requests a minute. On five checks, a throttle of 3 sent between 972 and 1,017. At a throttle of 3, 3,000 short lines took 183.6 seconds, or 980 a minute. That rate implies about 0.18 seconds per reply at three requests in flight; the reply time was derived, not measured. The accuracy record reports that longer records answer more slowly and stay under the limit at 4. It gives no rate for them. The service refused nothing in those runs, or at about 4,300 a minute in an earlier run that `sdlc/issues/closed/2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md` records. A 0.1-second reply can send `--jobs 3` past 1,200 requests a minute: local experiment 273, report 07 measured 1,285 a minute on loopback. Those rates came with no pacer, and by Ian's ruling 14 the default still paces nothing (ticket 0343). A user who needs to stay under the limit sets `requests_per_minute` on the backend's entry in the configuration file, or `THINKTHEN_REQUESTS_PER_MINUTE`; the [settings page](settings.md) has its row. The limit holds within one process, and separate processes on one account each get the full rate. [The proxy service](../sdlc/issues/2026-09-30-proxy-service-for-shared-limits-and-traces.md) and [the batch command](../sdlc/issues/2026-09-30-batch-command-runs-many-questions-in-one-process.md) are the future answers for a limit across processes. `sdlc/issues/closed/2026-09-24-the-default-jobs-width-runs-past-the-documented-limit.md` records why the historical default stayed at 4; ticket 0400 raises it to 8.

Experiment 0017 measured the 100,000-row AG News run at 60.29 s with throttle 4 and 52.00 s with throttle 8: identical 489 requests, 11,326,950 input tokens, $0.4757 and 0.890 accuracy, with no retry at 8. Its 25,000-row run took 16.18 s at 4 and 10.58 s at 8; mean requests in flight rose from 2.54 to 3.28. These are retained measurements, not a promise about another server or workload. The run has no server clock to separate network and server time. [The evidence record](../sdlc/records/0400-provider-setups-and-width.md) owns the observations.

A throttle of 8 can pass TypeSafe's retained 1,200-per-minute allowance on short records. A 429 is retried with a doubling wait. Optional `"typesafe": {"requests_per_minute": 1200}` in the configuration file spaces starts within one process. Throttle bounds simultaneous attempts; rate spaces request starts. Neither 8 in flight nor a process-local rate of 600 per minute guarantees Perplexity's organization-wide 10-per-second allowance across clients. Local-server queueing consumes each attempt's timeout; measured server slots do not establish an optimal client width.

For a completed finite input with stable batch boundaries, output order and bytes do not depend on `jobs`: the run prints what `--jobs 1` prints. Streaming output holds at most `jobs` un-emitted batches behind an earlier one, plus its bounded input and open-batch staging. A batch may contain many rows, and an individual row may be large. `rank` without `--top` retains every scored output row, so its memory can grow with input length. `rank --top N` keeps at most N winning output rows and at most `jobs` un-emitted batches, plus the bounded staging. This is a row-count bound, not a fixed byte or process-memory limit. A stopped run reports the work it actually finished; limiting the window can reduce speculative requests before the stop.

That order can make a live stream look idle: if its oldest unprinted record or batch is slow or retrying, finished later rows wait behind it. On commands that emit rows as they go, and on `rank --top N`, a full ordered `jobs` window stops further batch dispatch until that earlier work completes or stops. Uncut `rank` instead admits more work when a later request finishes, even while an earlier batch waits. Every rank form prints no order until every answer is ready. An attempt has its own `--timeout`; retried statuses can add waits, and another call's provider backoff can delay a send. The command has no whole-run deadline, so `--timeout` is not a promise that a whole run finishes within that time. A stop diagnostic names the failed record or request range after the failure; `--facts` gives final totals, not a live identity for the record currently blocking output.

On `decide`, `filter`, `rank`, and `choose`, `jobs` counts batches in flight. A batch is one request, so it still counts requests in flight. Streaming commands and `rank --top N` hold at most `jobs` un-emitted batches; uncut `rank` can retain later completions behind a slow earlier batch. For a completed finite input with stable batch boundaries, changing `--jobs` changes no batch or request body. On a timed pipe, changed backpressure may shift a pause-based batch cut.

One process opens one pool of connections and every worker posts through it, so a run over many records pays for one handshake rather than one for each record. A run opens up to one connection for each request in flight, so `--jobs N` opens up to N connections. The pool reuses no connection that sat idle for one second or more. A backend that keeps an idle connection open longer than one second plus the round trip therefore no longer races its keep-alive close with a send (ticket 0341). Experiment 218 saw 30 to 35 open file descriptors at `--jobs 32` and 7 at `--jobs 4`. A backend or proxy that caps connections per client needs a lower `--jobs`.

A run still stops at the first failed record. No new request starts once a failure is seen, and a request that finished after the failed record is still written under `--record`, because it was billed and a resume should not pay for it twice.

When the program downstream closes the pipe, the tool stops reading and stops scheduling. Where the platform reports the closed pipe, the tool notices between requests, so a command such as `filter`, which prints only some records, stops as soon as the reader is gone. Where it does not, the tool stops at its next write. Requests already sent may still be billed.

## The default backend's published limits

The vendor published these on 2026-09-19 for the hosted service at `https://api.typesafe.ai/v1`. Another address has its own.

| Limit | Value |
| --- | --- |
| Requests a minute | 1,200 |
| Evidence in one request | about 32,000 tokens |
| One whole request | about 64,000 tokens |

The vendor's pages disagreed on the last row, and one page gave 32,000 tokens for a whole request. A live check on 2026-09-19 sent one request of 33,663 input tokens, with about 20,000 tokens of evidence and three long questions, and the service accepted it. The larger number holds, which matters to `annotate`, because a question set rides in one request. `sdlc/records/0017-every-question-option-has-two-homes.md` holds the check.

A run over the rate limit gets a 429, and [backends.md](backends.md) fixes the retry. Evidence past the token limit is refused, and the exit code is 4.

## A loop over files has no budget

A per-record judgment is a paid request, and a run has no request cap. A loop over a folder of files sits outside even that, because each file is its own run. The help shows `find -print0 | xargs -0 -n 1 -P 4` beside the cost warning.

## Filter and rank neighbor snapshots

Filter, rank and find own `-n`/`--line-number`, `--scores` and `--around N`. Their text views refuse `--details`, CSV and TSV. [filter.md](filter.md) and [rank.md](rank.md) and [find.md](find.md) define exact prefixes, delimiters and score meanings. Locations are physical and file-local. Several named source arguments show a filename only with `-n`. A repeated pathname retains an independent source occurrence. This private identity adds no result JSON member.

Only `--around` consumes all source bytes before the first request. Its immutable snapshot accepts exactly 16 MiB across all source occurrences and refuses one more original byte before decoding. It counts CR, LF and blanks. Open/read failures retain their existing safe error classes. The same snapshot supplies framed items and byte neighbors; file mutation cannot change either. No spool is written. The renderer scans bounded bytes without a line-offset table and expands groups only at output. Top rank retains winner rows rather than expanded groups. The source cap does not bound the process's total memory. Stdin is consumed before sending in this view. Ordinary content failures retain filter's completed prefix and rank's empty failure output.

Find preserves its complete aggregate candidate set under every display flag. Every admitted candidate already enters the request; neighbor display adds nothing. Only around snapshots the source first. Find refuses blank or malformed candidates before sending and keeps its ordinary count and byte checks without around. Details appends the selected physical position and omits it for unresolved none.

Native complete fallible record calls use `Engine::try_decide_records_complete_with`, `try_choose_records_complete_with`, `try_choose_dynamic_records_complete_with`, `try_tag_records_complete_with`, `try_score_records_complete_with`, `try_filter_records_complete_with`, and `try_annotate_records_complete_with`. They pull the ordinary reader/composition iterator through the same scheduler, preserve original occurrences and locations, and yield the completed prefix followed by one terminal error with joined final facts. Dynamic choose takes a `RecordChooseQuestion` and requires a whole ordered candidate list on every record. Missing candidates stop at that original without inheriting an earlier list. `Batch::into_call` collects those native values and facts for a caller needing one complete call. Eager complete calls still validate the whole input set before any send. Per-record context caps apply before its row sends; explicit empty context suppresses the call fallback.

Whole-set find and rank admit fallible originals inside the native engine before any send. `try_find_records_complete_with` uses the existing ordered candidate set, count and byte limits, retaining every original and location through replay. `try_rank_records_complete_with` retains ordinals and described decide/saved score readings before stable ordering assigns positions. Find accepts one call-wide context and refuses per-record context/options; a reader failure cannot produce a partial whole-set selection.

`recognize_records_complete_with` and its fallible `try_` form admit selected native records through the existing recognition first-stage/profile boundaries before sending. One call owns the shared cancellation, deadline and budget. Each completed original has its own full staged result and source-aware owned observations; started failure facts retain the completed prefix. Recognition uses call-wide context and refuses per-record context/options. Offsets refer to the selected text, while the complete record retains the whole original and its physical location.

Native whole-set relate consumes composed records through `relate_records_complete_with` or its fallible `try_` form. `Relate::from_records_json`, `load_records` and `record_fields` retain the ordinary saved/custom RFC 6901 name/kind pointers; the released entity-only loaders retain their default-pointer contract. The existing native record reader handles recognized `text` fallback at `/name`. The complete aggregate retains the ordered original set as `CompleteRecord<Vec<T>, CompleteRelated>`, with the existing 255 distinct-entity limit and a 16 MiB retained-original boundary. Literal source text uses the ordinary wildcard-kind rules; mixed text/JSON modes and per-record context/options refuse before sending. Find and relate owned question details expose the full ordered source set through `inputs()`, including physical locations outside request/cache/result identity.

Native complete record calls take `RecordInput.context: Option<RecordContext>`. `RecordContext::Text` retains exact text; an empty text value suppresses shared fallback. `RecordContext::Object` contains `ObjectContext`, constructed from an already parsed `RawRecord` object and admitted only by the question's explicit object `context_schema`. Object member order and extra properties remain intact. `RecordReading::with_context_schema` enables that same typed projection from a context pointer; absence of the declaration retains the original text-only pointer rule. JSON-looking strings are never parsed as context objects. Null is always refused. Every explicit per-item context validates before lookup/send; absent per-item context may use the unchanged plain shared context without applying the declaration to it. The context stays separate from evidence and cannot move spans or source coordinates. No released scalar or convenience signature changes.

With input declarations, an incremental native or CLI reader validates a bounded admission stage before cache lookup or sends. The ordinary packer determines stage boundaries from the selected record cap and existing state/request/profile/question limits; the ordinary input pause may close a stage. A malformed item/context prevents that uncommitted stage from sending. Earlier admitted work may complete and keeps its ordered prefix and started facts. This does not change wire grouping, batch defaults/precedence, concurrency or per-question cache identity: only missing admitted questions reach the ordinary packer and send coordinator. A materialized native complete call validates its entire selected collection first. Annotation declaration failures are terminal; the existing explicit missing-pointer exception remains limited to missing pointers. Rank still withholds ordered output on a stopped call.

Complete-set native find admission stops before pulling an input suffix after cancellation or one excess candidate. The composed route admits at most the existing candidate maximum plus that one refused candidate; it never converts a too-large set into a truncated successful find. Released and complete plain find share the same cancellation checks. Relative deadlines are fixed once before preparation and reused through the actual call.
