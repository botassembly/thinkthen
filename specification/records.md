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
| `filter`, `rank` | One record flag is required. One document is not a stream |
| `annotate` | One document by default. All four record flags are accepted |

### CSV and TSV

CSV uses a comma and TSV uses a tab. Both require a header. Empty input is exit 2. A header without data rows is a successful empty dataset and makes no request. One UTF-8 byte-order mark is ignored only at the start of the first header name.

After that removal, a header name may preserve whitespace but may not be blank, contain a control character, or exactly duplicate another name. Comparison is case-sensitive. Header order becomes object key order. Every cell becomes a JSON string, including empty cells and text that looks like a number, boolean, null, array, or object. Every result remains JSONL.

The maintained `csv-core` grammar owns quoted delimiters, doubled quotes, quoted line feeds, irregular quote placement, LF and CRLF records, trailing empty cells, and blank physical lines outside quoted fields. Every data row must have the header's field count. `choose --options` remains JSONL-only because table cells are strings.

A header or logical data row may hold at most 16 MiB of encoded bytes before its record terminator. Quoted line feeds and doubled quotes count. The edge parses incrementally within that bound and stops after an oversized row; its unread tail never becomes another record. Header diagnostics say `the CSV header` or `the TSV header`. Data diagnostics say `the CSV record` or `the TSV record` and the stopped-run line counts data rows from one. A diagnostic never repeats a header or cell value. Invalid UTF-8 remains a local failure at exit 5; other table grammar failures exit 2.

## `--field POINTER`

`--field` is a JSON Pointer as RFC 6901 defines it. It names the part of each record the model sees. `/body` reads the `body` member, and `/a/text` reads `text` inside `a`.

**The pointer is the disclosure boundary.** Only the pointed value leaves the machine. `--details` still carries the whole record in `input`.

- `--field` with `--jsonl`, `--csv`, or `--tsv` reads the pointer in each record.
- `--field` without a record framing reads the whole input as one JSON value and takes the pointer inside it. No separate JSON framing flag exists.
- `--field` with `--lines` is a usage error. A text line has no members.
- A pointer that finds nothing is an input error for that record at exit 2, before any request for it.
- `$.body`, `#/id`, a wildcard, and a negative index are refused with a message that names RFC 6901, because the tool never guesses a pointer language.
- A pointed value that is not a string is serialized as compact JSON and sent as text.
- Without `--field`, a JSONL, CSV, or TSV record is serialized as compact JSON and the whole record becomes the evidence.

Invalid JSON under `--field` without a record framing is an input error at exit 2. It says `the input is not valid JSON: the JSON at line LINE column COLUMN is not one` for both standard input and `--input FILE`. The line and column come from the JSON parser. The message carries no parser text and repeats no input byte.

### Several pointers

Settled by ADR 0008 item 2, accepted in ADR 0010. `--field` takes one pointer or several. Several pointers build an evidence object, each member keyed by the last part of its pointer. Two members that would share one key are a usage error.

```sh
thinkthen decide 'The output answers the input correctly.' --jsonl --field /input --field /output < cases.jsonl
```

The evidence is then `{"input":"...","output":"..."}`. A check that must not see the gold answer names only the pointers it needs.

The evidence object is not a string, so it goes out as compact JSON by the rule above. [backends.md](backends.md) carries it in one field, the way a single pointer's value travels.

The vendor also accepts that field as a real JSON object rather than as text holding one. `probes/09-evidence-shape/` measured both on forty made-up labeled cases on 2026-09-19. Both shapes answered 40 of 40 correctly, no answer differed, the probability moved by 0.0045 on average and by 0.05 at most, and the object shape cost 13,954 input tokens against the string shape's 13,714. The check separated nothing, so the string stays. A measurement on cases the model finds hard could overturn it.

## What a record may not hold

- A JSON record that holds two members under one name is refused, because no reader can say which of the two a pointer means.
- A JSON record holding `NaN`, `Infinity`, or a number too large to be finite is refused, because none of the three is a JSON number.
- A malformed JSONL record is refused at exit 2 with `the record is not valid JSON`. Its stopped-record line identifies its place, so this message carries no JSON location and repeats no record byte.
- A record whose bytes are not valid UTF-8 is refused at exit 5, because bytes that are not text are a local failure rather than a record the tool read.
- A record over 16 MiB is refused at exit 2 for that record, before any request. The line that ended the record is not part of it. The number is fixed and no option sets it, because the vendor's token budget refuses evidence far smaller than that. The reader stops a little past the limit, so a stream whose line runs longer than that ends there. What follows the cut is the middle of the refused record and never a record of its own.

## Order and requests

Records never share model context, and no answer reaches another record's question. One value prints per record, and output keeps input order everywhere but `rank`. No record is dropped for being unresolved, except that `filter` prints only what it keeps. `filter` prints kept line and JSONL records as they arrived. CSV and TSV rows print as compact JSON objects in header order.

### How many requests each command makes

Settled by ADR 0008, accepted in ADR 0010. One request carries one piece of evidence and every question asked of it. Two pieces of evidence never share a request, because records must not see each other and a check must not see a field outside its pointers.

| Command | Requests |
| --- | --- |
| `decide`, `choose`, `tag`, `score` on one document | 1 |
| `annotate` on one document | 1 for each distinct `on` |
| `decide`, `choose`, `tag`, `score`, `filter`, `rank` over N records | N |
| `annotate` over N records | N times the number of distinct `on` sets |
| `find` | 1 |
| `--dry-run`, `--replay` | 0 |

`rank` sorts locally and makes no pairwise calls. Every request inside one command is independent of every other. A command is therefore one round, and the round runs in parallel with output order kept.

## Empty input

An empty record stream succeeds with no output and no request. An empty document is a usage error, because a judgment about nothing is a mistake in the pipeline.

## Failure

Settled by ADR 0008 item 5, accepted in ADR 0010. A run stops at the first failed record. Rows already printed stay printed, and the run ends with the code the failure earns: 4 for a backend failure, 5 for a local failure, 2 for a record the tool refused before sending it. No failure ever becomes `false`, `null`, a label, or a zero.

A run that stops early prints one line on standard error: the record it stopped at, how many records it finished, and how many of those came from a recording. A run that finishes prints nothing there. Printed output after a failure is a prefix of the input. It is not a finished dataset.

An unresolved answer is never retried. In record mode the exit code reports the run, and no record's answer sets it. Codes 6, 7, and 8 stay reserved and no command uses them.

## Resume

A rerun with `--record DIR --replay DIR` on one folder answers the finished records from disk and pays only for the rest. [recording.md](recording.md) gives the folder and the entry.

`--cache DIR` means `--record DIR --replay DIR`, by ADR 0010. It is Settled. `--cache` beside either of the two options it stands for is a usage error.

```sh
thinkthen decide 'This reports a payment failure.' --jsonl --field /body --record runs/tickets --replay runs/tickets < tickets.jsonl
```

A first run that stops at record 400 leaves 399 entries. The same command run again replays those 399 and pays for the rest.

Each digest keeps the first complete response installed in the folder. A cache miss can race with another worker or process, but only one response can win the entry name. A loser with the same stored JSON response succeeds. Whitespace outside the backend's JSON value does not distinguish responses. A loser holding another stored response stops at exit 5. The winner stays intact, so every successful run can replay the answers it printed.

## `jobs`

`jobs` bounds how many requests are in flight at once. The configuration file that held it left version one with ADR 0010, and [roadmap.md](roadmap.md) says so. ADR 0010 gives it the advanced option `--jobs N`, which takes a whole number from 1 to 32 and defaults to 4. The vendor's own example code uses 4 to 12 workers and says the public endpoint limits concurrency above about eight, so 4 is safe everywhere and a measured run can raise it. `annotate` also accepts `--jobs` for one document because distinct evidence groups make distinct requests. Another command refuses it outside record mode.

Output order never depends on `jobs`. A run with any number prints the bytes that `--jobs 1` prints, on standard output and on standard error, whether it finished or stopped. The tool holds finished rows in a bounded buffer until the rows before them are written, and the buffer holds at most `jobs` rows, so the memory of a long run stays flat.

One process opens one pool of connections and every worker posts through it, so a run over many records pays for one handshake rather than one for each record.

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
