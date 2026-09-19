# Records

Status: **Settled** for version one, by ADR 0007. **Draft** for the default of `jobs`.

The default input is one text document. A record stream turns a command into a map over records. [channels.md](channels.md) governs the five channels, and [result.md](result.md) governs the shape of one result.

## Where the input comes from

`--input FILE` reads a file. Without it the tool reads standard input. `--input` names a path and never a framing.

| Flag | What one record is |
| --- | --- |
| none | The whole input is one text document and one record |
| `--lines` | Each line is one text record. A trailing newline ends the last record |
| `--jsonl` | Each line is one JSON value and one record. No blank lines |

The tool never guesses the framing. It never repairs invalid JSON, never truncates an oversized record, and never opens a file because a string looks like a path.

| Command | Framing |
| --- | --- |
| `decide`, `choose`, `score` | One document by default. `--lines` and `--jsonl` are accepted |
| `filter`, `rank` | One of `--lines` or `--jsonl` is required. One document is not a stream |
| `annotate` | One document by default. Both flags are accepted |
| `segment` | One document only. Either flag is a usage error |
| `report`, `config` | Neither flag applies |

## `--field POINTER`

`--field` is a JSON Pointer as RFC 6901 defines it. It names the part of each record the model sees. `/body` reads the `body` member. `/a/text` reads `text` inside `a`.

**The pointer is the disclosure boundary.** Only the pointed value leaves the machine. The rest of the record stays on the machine, and `--details` still carries the whole record in `input`.

- `--field` with `--jsonl` reads the pointer in each line's record.
- `--field` without `--jsonl` reads the whole input as one JSON value and takes the pointer inside it. No separate JSON framing flag exists.
- `--field` with `--lines` is a usage error. A text line has no members.
- A pointer that finds nothing is an input error for that record at exit 2, before any request for it.
- A pointed value that is not a string is serialized as compact JSON and sent as text.

Without `--field`, a `--jsonl` record is serialized as compact JSON and the whole record becomes the evidence.

## Order and requests

Each record is its own request. Records never share model context, and no answer reaches another record's question.

Output keeps input order on every command but `rank`, which prints its own order and holds every record until the input ends. One value prints per record. No record is dropped for being unresolved, except that `filter` prints only what it keeps.

Preserving a record means preserving its values and its field names. It does not promise the same whitespace. `filter` prints a kept record byte for byte as it arrived.

## Empty input

An empty record stream succeeds with no output and no request. An empty document is a usage error, because a judgment about nothing is a mistake in the pipeline.

## Failure

A run stops at the first failed record. Rows already printed stay printed, and the run ends with the code the failure earns: 4 for a backend failure, 5 for a local failure, 2 for a record the tool refused before sending it. No failure ever becomes `false`, `null`, a label, or a zero.

Printed output after a failure is a prefix of the input. It is not a finished dataset.

An unresolved answer is never retried. Asking again until the answer is acceptable is not a policy.

In record mode the exit code reports the run, and no record's answer sets it. Codes 6, 7, and 8 stay reserved and no command uses them.

## Resume

A rerun with `--record DIR --replay DIR` on one folder answers the finished records from disk and pays only for the rest. That pairing is the resume. [recording.md](recording.md) gives the folder and the entry.

```sh
thinkthen decide 'This reports a payment failure.' --jsonl --field /body --record runs/tickets --replay runs/tickets < tickets.jsonl
```

A first run that stops at record 400 leaves 399 entries in the folder. The same command run again replays those 399 and sends the rest.

## `jobs`

Draft. `jobs` is a configuration setting and never a flag. It bounds how many requests are in flight at once. [config.md](config.md) holds it.

Output order never depends on `jobs`. The tool holds finished rows in a bounded buffer until the rows before them are written. A faster later record never attaches itself to an earlier one.

The first decider model accepts about 1,200 requests per minute. A run over that rate gets a 429, and [backends.md](backends.md) fixes the retry. `jobs` is the lever that keeps a run under the rate.

When the program downstream closes the pipe, the tool stops reading and stops scheduling. Requests already sent may still be processed and billed.

## A loop over files has no budget

A per-record judgment is a paid request, and a run has no request cap. A loop over a folder of files sits outside even that, because each file is its own run. The help for every record-reading command shows `find -print0 | xargs -0 -n 1 -P 4` next to the cost warning.

## Open points

- What does `jobs` default to? Recommendation: 4, the value ADR 0007's configuration example shows. Four is safe under the measured rate limit, and a measured run can raise it.
- Is a record whose pointer finds nothing really exit 2? ADR 0007 says so, and it also says that exit 2 means nothing was sent. The two disagree once earlier records have been judged. Recommendation: keep exit 2 for the pointer miss and soften the exit 2 row to say that the failing record sent nothing.
