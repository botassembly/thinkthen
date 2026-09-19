# Records

Status: **Settled** for version one, by ADR 0007. **Draft** for several pointers on `--field`, for the request table, and for the default of `jobs`.

The default input is one text document. A record stream turns a command into a map over records. [channels.md](channels.md) governs the five channels, and [result.md](result.md) governs the shape of one result.

## Where the input comes from

`--input FILE` reads a file. Without it the tool reads standard input. `--input` names a path and never a framing.

| Flag | What one record is |
| --- | --- |
| none | The whole input is one text document and one record |
| `--lines` | Each line is one text record. A trailing newline ends the last record |
| `--jsonl` | Each line is one JSON value and one record. No blank lines |

The tool never guesses the framing. It never repairs invalid JSON, never truncates a record, and never opens a file because a string looks like a path.

| Command | Framing |
| --- | --- |
| `decide`, `choose`, `score` | One document by default. `--lines` and `--jsonl` are accepted |
| `filter`, `rank` | One of `--lines` or `--jsonl` is required. One document is not a stream |
| `annotate` | One document by default. Both flags are accepted |
| `report`, `config` | Neither flag applies |

## `--field POINTER`

`--field` is a JSON Pointer as RFC 6901 defines it. It names the part of each record the model sees. `/body` reads the `body` member, and `/a/text` reads `text` inside `a`.

**The pointer is the disclosure boundary.** Only the pointed value leaves the machine. `--details` still carries the whole record in `input`.

- `--field` with `--jsonl` reads the pointer in each line's record.
- `--field` without `--jsonl` reads the whole input as one JSON value and takes the pointer inside it. No separate JSON framing flag exists.
- `--field` with `--lines` is a usage error. A text line has no members.
- A pointer that finds nothing is an input error for that record at exit 2, before any request for it.
- A pointed value that is not a string is serialized as compact JSON and sent as text.
- Without `--field`, a `--jsonl` record is serialized as compact JSON and the whole record becomes the evidence.

### Several pointers

Draft, from Proposed ADR 0008. `--field` takes one pointer or several. Several pointers build an evidence object, each member keyed by the last part of its pointer. Two members that would share one key are a usage error.

```sh
thinkthen decide 'The output answers the input correctly.' --jsonl --field /input --field /output < cases.jsonl
```

The evidence is then `{"input":"...","output":"..."}`. A check that must not see the gold answer names only the pointers it needs.

The evidence object is not a string, so it goes out as compact JSON by the rule above. [backends.md](backends.md) carries it in one field, the way a single pointer's value travels.

## Order and requests

Records never share model context, and no answer reaches another record's question. One value prints per record, and output keeps input order everywhere but `rank`. No record is dropped for being unresolved, except that `filter` prints only what it keeps. `filter` prints a kept record byte for byte as it arrived.

### How many requests each command makes

One request carries one piece of evidence and every question asked of it. Two pieces of evidence never share a request, because records must not see each other and a check must not see a field outside its pointers.

| Command | Requests |
| --- | --- |
| `decide`, `choose`, `score` on one document | 1 |
| `annotate` on one document | 1 for each distinct `on` |
| `decide`, `choose`, `score`, `filter`, `rank` over N records | N |
| `annotate` over N records | N times the number of distinct `on` sets |
| `find` | 1 |
| `report`, `config`, `--dry-run`, `--replay` | 0 |

`rank` sorts locally and makes no pairwise calls. Every request inside one command is independent of every other. A command is therefore one round, and the round runs in parallel with output order kept.

## Empty input

An empty record stream succeeds with no output and no request. An empty document is a usage error, because a judgment about nothing is a mistake in the pipeline.

## Failure

A run stops at the first failed record. Rows already printed stay printed, and the run ends with the code the failure earns: 4 for a backend failure, 5 for a local failure, 2 for a record the tool refused before sending it. No failure ever becomes `false`, `null`, a label, or a zero.

A run that stops early prints one line on standard error: the record it stopped at, how many records it finished, and how many of those came from a recording. A run that finishes prints nothing there. Printed output after a failure is a prefix of the input. It is not a finished dataset.

An unresolved answer is never retried. In record mode the exit code reports the run, and no record's answer sets it. Codes 6, 7, and 8 stay reserved and no command uses them.

## Resume

A rerun with `--record DIR --replay DIR` on one folder answers the finished records from disk and pays only for the rest. [recording.md](recording.md) gives the folder and the entry.

```sh
thinkthen decide 'This reports a payment failure.' --jsonl --field /body --record runs/tickets --replay runs/tickets < tickets.jsonl
```

A first run that stops at record 400 leaves 399 entries. The same command run again replays those 399 and pays for the rest.

## `jobs`

`jobs` is a configuration setting and never a flag. It bounds how many requests are in flight at once. [config.md](config.md) holds it.

Draft: the default is 4. The vendor's own example code uses 4 to 12 workers and says the public endpoint limits concurrency above about eight, so 4 is safe everywhere and a measured run can raise it.

Output order never depends on `jobs`. The tool holds finished rows in a bounded buffer until the rows before them are written.

When the program downstream closes the pipe, the tool stops reading and stops scheduling. Requests already sent may still be billed.

## The built-in profile's published limits

The vendor published these on 2026-09-19 for the `jev` profile. Another profile has its own.

| Limit | Value |
| --- | --- |
| Requests a minute | 1,200 |
| Evidence in one request | about 32,000 tokens |
| One whole request | about 64,000 tokens |

A run over the rate limit gets a 429, and [backends.md](backends.md) fixes the retry. Evidence past the token limit is refused, and the exit code is 4.

## A loop over files has no budget

A per-record judgment is a paid request, and a run has no request cap. A loop over a folder of files sits outside even that, because each file is its own run. The help shows `find -print0 | xargs -0 -n 1 -P 4` beside the cost warning.
