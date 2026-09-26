# Records

Status: **Settled** for version one, by ADR 0007 and ADR 0010.

The default input is one text document. A record stream turns a command into a map over records. [channels.md](channels.md) governs the five channels, and [result.md](result.md) governs the shape of one result.

## Where the input comes from

`--input FILE` reads a file. Without it the tool reads standard input. `--input` names a path and never a framing.

| Flag | What one record is |
| --- | --- |
| none | The whole input is one text document and one record |
| `--lines` | Each line is one text record. A trailing newline ends the last record |
| `--jsonl` | Each line is one JSON value and one record. No blank lines |
| `--csv` | The first logical row is a header. Each later row becomes one JSON object of string cells |
| `--tsv` | The CSV rules with a tab delimiter |

The four flags are mutually exclusive. The tool never guesses the framing from a filename. It never repairs invalid JSON, never truncates a record, and never opens a file because a string looks like a path.

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
- A malformed JSONL record is refused at exit 2 with `the record is not valid JSON`. Its stopped-record line identifies its place, so this message carries no JSON location and repeats no record byte.
- A record whose bytes are not valid UTF-8 is refused at exit 5, because bytes that are not text are a local failure rather than a record the tool read.
- A record over 16 MiB is refused at exit 2 for that record, before any request. The line that ended the record is not part of it. The number is fixed and no option sets it, because the vendor's token budget refuses evidence far smaller than that. The reader stops a little past the limit, so a stream whose line runs longer than that ends there. What follows the cut is the middle of the refused record and never a record of its own.

## Order and requests

Records never share model context, except that `find` deliberately sends its complete bounded set as one aggregate request. Default `decide`, `choose`, `tag`, and `score` output keeps each parsed record under `input` and its answer under `value`. Output keeps input order everywhere but `rank` and `find`. No record is dropped for being unresolved, except that `filter` prints only what it keeps and `find --none` prints nothing. `choose --raw` keeps printing plain labels for lines and JSONL. `filter` prints kept line and JSONL records as they arrived. CSV and TSV rows print as compact JSON objects in header order. Every output from CSV and TSV input is JSONL.

### How many requests each command makes

Settled by ADR 0008, accepted in ADR 0010, with the `find` exception settled by ADR 0030. One request normally carries one piece of evidence and every question asked of it. `find` sends all of its units together because choosing the best unit requires comparison across the set.

| Command | Requests |
| --- | --- |
| `decide`, `choose`, `tag`, `score` on one document | 1 |
| `annotate` on one document | 1 for each distinct `on` |
| `decide`, `choose`, `tag`, `score`, `filter`, `rank` over N records | N |
| `annotate` over N records | N times the number of distinct `on` sets |
| `find` | 1 |
| `relate` | The shared relation planner's exact request count for the complete set |
| `--dry-run`, `--replay` | 0 |

`rank` sorts locally and makes no pairwise calls. Without a cache, every request inside one command is independent of every other. A command is therefore one round, and the round runs in parallel with output order kept. With a cache, equal request digests share one backend call and each record still receives its own logical judgment in input order.

## Empty input

An empty line or JSONL stream succeeds with no output and no request. CSV and TSV require a header, so an empty CSV or TSV input exits 2. An empty document is a usage error, because a judgment about nothing is a mistake in the pipeline.

For `relate`, an empty line or JSONL stream and a header-only table are successful empty sets. A blank line in either stream is invalid. The default document must be a nonempty JSON array. The command validates the whole set before any request. [relate.md](relate.md) gives its independent name and kind pointers and complete-set refusals.

## Failure

Settled by ADR 0008 item 5, accepted in ADR 0010. A run stops at the first failed record. Rows already printed stay printed, and the run ends with the code the failure earns: 4 for a backend failure, 5 for a local failure, 2 for a record the tool refused before sending it. No failure ever becomes `false`, `null`, a label, or a zero.

A run that stops early prints one line on standard error with the record where it stopped and how many records it finished. When `--record`, `--replay`, or `--cache` made recordings relevant, the line also says how many finished records came from one. A run that finishes prints nothing there. Printed output after a failure is a prefix of the input. It is not a finished dataset.

An unresolved answer is never retried. In record mode the exit code reports the run, and no record's answer sets it. A completed run exits 0 unless `annotate` preserves one or more failed questions beside good answers and exits 6. Codes 7 and 8 stay reserved.

## Resume

A rerun with `--record DIR --replay DIR` on one folder answers the finished records from disk and pays only for the rest. [recording.md](recording.md) gives the folder and the entry.

`--cache DIR` means `--record DIR --replay DIR`, by ADR 0010. It is Settled. `--cache` beside either of the two options it stands for is a usage error.

```sh
thinkthen decide 'This reports a payment failure.' --jsonl --field /body --record runs/tickets --replay runs/tickets < tickets.jsonl
```

A first run that stops at record 400 leaves 399 entries. The same command run again replays those 399 and pays for the rest.

Each digest keeps the first complete response installed in the folder. Concurrent cache misses for that digest wait on one operating-system file lock. The owner checks again, sends only if the entry remains absent, and installs the complete response. Waiters replay it. A failed or stopped owner releases the lock automatically; the next waiter sends if no entry was installed. Record-only writers remain independent, and a later writer holding another stored response stops at exit 5. The winner stays intact, so every successful run can replay the answers it printed.

## `jobs`

`jobs` sets the throttle. The throttle is the most requests in flight at once, per loaded copy of the library. The configuration file that held it left version one with ADR 0010, and [roadmap.md](roadmap.md) says so. ADR 0010 gives it the advanced option `--jobs N`, which takes a whole number from 1 to 32 and defaults to 4. The vendor's own example code uses 4 to 12 workers and says the public endpoint limits concurrency above about eight. A measured run can raise it. `annotate` also accepts `--jobs` for one document because distinct evidence groups make distinct requests. Another command refuses it outside record mode.

The default can run past the vendor's documented 1,200 requests a minute on short records. Experiment 206 measured it from one machine, and `sdlc/issues/closed/2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun.md` records it. On three 200-record checks of short lines, a throttle of 4 sent 1,267, 1,319, and 1,272 requests a minute. On five checks, a throttle of 3 sent between 972 and 1,017. At a throttle of 3, 3,000 short lines took 183.6 seconds, or 980 a minute. The accuracy record reports that longer records answer more slowly and stay under the limit at 4. It gives no rate for them. The service refused nothing in those runs, or at about 4,300 a minute in an earlier run that `sdlc/issues/closed/2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md` records. A run that must stay inside the documented limit on short records sets `--jobs 3`. `sdlc/issues/closed/2026-09-24-the-default-jobs-width-runs-past-the-documented-limit.md` records why the default stays at 4.

Output order never depends on `jobs`. A run with any number prints the bytes that `--jobs 1` prints, on standard output and on standard error, whether it finished or stopped. The tool holds finished rows in a bounded buffer until the rows before them are written, and the buffer holds at most `jobs` rows, so the memory of a long run stays flat.

One process opens one pool of connections and every worker posts through it, so a run over many records pays for one handshake rather than one for each record. A run opens up to one connection for each request in flight, so `--jobs N` opens up to N connections. Experiment 218 saw 30 to 35 open file descriptors at `--jobs 32` and 7 at `--jobs 4`. A backend or proxy that caps connections per client needs a lower `--jobs`.

A run still stops at the first failed record. No new request starts once a failure is seen, and a request that finished after the failed record is still written under `--record`, because it was billed and a resume should not pay for it twice.

When the program downstream closes the pipe, the tool stops reading and stops scheduling. Requests already sent may still be billed.

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
