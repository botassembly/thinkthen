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

The tool never guesses the framing. It never repairs invalid JSON, never truncates a record, and never opens a file because a string looks like a path.

Under `--lines` and under `--jsonl` a carriage return before the line feed is stripped with it, and a carriage return anywhere else in the line is kept.

| Command | Framing |
| --- | --- |
| `decide`, `choose`, `score` | One document by default. `--lines` and `--jsonl` are accepted |
| `filter`, `rank` | One of `--lines` or `--jsonl` is required. One document is not a stream |
| `annotate` | One document by default. Both flags are accepted |

## `--field POINTER`

`--field` is a JSON Pointer as RFC 6901 defines it. It names the part of each record the model sees. `/body` reads the `body` member, and `/a/text` reads `text` inside `a`.

**The pointer is the disclosure boundary.** Only the pointed value leaves the machine. `--details` still carries the whole record in `input`.

- `--field` with `--jsonl` reads the pointer in each line's record.
- `--field` without `--jsonl` reads the whole input as one JSON value and takes the pointer inside it. No separate JSON framing flag exists.
- `--field` with `--lines` is a usage error. A text line has no members.
- A pointer that finds nothing is an input error for that record at exit 2, before any request for it.
- `$.body`, `#/id`, a wildcard, and a negative index are refused with a message that names RFC 6901, because the tool never guesses a pointer language.
- A pointed value that is not a string is serialized as compact JSON and sent as text.
- Without `--field`, a `--jsonl` record is serialized as compact JSON and the whole record becomes the evidence.

### Several pointers

Settled by ADR 0008 item 2, accepted in ADR 0010. `--field` takes one pointer or several. Several pointers build an evidence object, each member keyed by the last part of its pointer. Two members that would share one key are a usage error.

```sh
thinkthen decide 'The output answers the input correctly.' --jsonl --field /input --field /output < cases.jsonl
```

The evidence is then `{"input":"...","output":"..."}`. A check that must not see the gold answer names only the pointers it needs.

The evidence object is not a string, so it goes out as compact JSON by the rule above. [backends.md](backends.md) carries it in one field, the way a single pointer's value travels.

## What a record may not hold

- A JSON record that holds two members under one name is refused, because no reader can say which of the two a pointer means.
- A JSON record holding `NaN`, `Infinity`, or a number too large to be finite is refused, because none of the three is a JSON number.
- A record whose bytes are not valid UTF-8 is refused at exit 5, because bytes that are not text are a local failure rather than a record the tool read.
- A record over 16 MiB is refused at exit 2 for that record, before any request. The line that ended the record is not part of it. The number is fixed and no option sets it, because the vendor's token budget refuses evidence far smaller than that. The reader stops a little past the limit, so a stream whose line runs longer than that ends there. What follows the cut is the middle of the refused record and never a record of its own.

## Order and requests

Records never share model context, and no answer reaches another record's question. One value prints per record, and output keeps input order everywhere but `rank`. No record is dropped for being unresolved, except that `filter` prints only what it keeps. `filter` prints a kept record byte for byte as it arrived.

### How many requests each command makes

Settled by ADR 0008, accepted in ADR 0010. One request carries one piece of evidence and every question asked of it. Two pieces of evidence never share a request, because records must not see each other and a check must not see a field outside its pointers.

| Command | Requests |
| --- | --- |
| `decide`, `choose`, `score` on one document | 1 |
| `annotate` on one document | 1 for each distinct `on` |
| `decide`, `choose`, `score`, `filter`, `rank` over N records | N |
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

## `jobs`

`jobs` bounds how many requests are in flight at once. The configuration file that held it left version one with ADR 0010, and [roadmap.md](roadmap.md) says so. ADR 0010 gives it the advanced option `--jobs N`, which takes a whole number from 1 to 32 and defaults to 4. The vendor's own example code uses 4 to 12 workers and says the public endpoint limits concurrency above about eight, so 4 is safe everywhere and a measured run can raise it. `--jobs` outside record mode is a usage error, because one document sends one request.

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

A run over the rate limit gets a 429, and [backends.md](backends.md) fixes the retry. Evidence past the token limit is refused, and the exit code is 4.

## A loop over files has no budget

A per-record judgment is a paid request, and a run has no request cap. A loop over a folder of files sits outside even that, because each file is its own run. The help shows `find -print0 | xargs -0 -n 1 -P 4` beside the cost warning.
