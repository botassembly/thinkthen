# Records

Status: **Draft**.

Some verbs judge one document. Others judge a stream of records. `where`, `rank`, `match`, and `run` read a stream, and this document fixes what they share. [channels.md](channels.md) still governs the five channels, and [result.md](result.md) still governs the shape of one result.

A record is one unit of evidence with its own result. A row is one line that a stream verb prints.

## Framing

`--input KIND` says how standard input divides into records.

| Kind | What it means |
| --- | --- |
| `text` | The whole input is one document and one record. This is the default for `if`, `which`, `how`, `segment`, and `run` |
| `json` | The whole input is one JSON value and one record. An array is one record. It is never a batch |
| `jsonl` | Each line is one JSON value and one record. No blank lines |
| `lines` | Each line is one text record. A trailing newline ends the last record |

`where`, `rank`, and `match` reject `--input text`, because one document is not a stream.

The tool never guesses the framing. It never repairs invalid JSON, never truncates an oversized record, and never opens a file because a string looks like a path.

CSV and TSV are later additions. They arrive with `--as NAME`, which appends the judgment to each row as a new column beside a status column. That work needs a column-collision rule, a quoting dialect, and a null convention, so it lands in its own document.

## Pointers

`--on POINTER` names the value inside a JSON record that becomes the evidence. `--id POINTER` names the value that identifies the record in the result.

Both take a JSON Pointer as RFC 6901 defines it. `/body` reads the `body` member. `/a/text` reads `text` inside `a`. A pointer that hits nothing is an error for that record.

**The pointer is the disclosure boundary.** Only the pointed value leaves the machine. Every other field of the record stays local and is still available for output.

Without `--on`, a `jsonl` record is serialized as compact JSON and the whole record becomes the evidence. Without `--id`, the result identifies the record by its zero-based input position.

## What prints

`--emit MODE` picks what each row holds.

| Mode | What one row holds |
| --- | --- |
| `result` | The judgment for that record |
| `input` | The original record, unchanged |
| `annotated` | `{"input": ..., "result": ...}` |

Defaults: `result` for `match` and `run`, `input` for `where` and `rank`.

Preserving a record means preserving its values and its field names. It does not promise the same whitespace.

## The row guarantee

Under `--emit result` and `--emit annotated`, these hold for every stream verb.

- One output row per input record.
- Input order is kept, except that `rank` prints its own order.
- No row is dropped for being unsure.
- No failure becomes `false`, `other`, zero, or an empty success.

`where --emit input` is the one place where records leave the output, because selecting is what a filter does. The count of records that the filter left out prints on standard error. A pipeline that must keep every record uses `--emit annotated`.

## Unsure records

`where` takes `--unknown drop|keep|error`. The verb's own section gives the default.

| Value | What an unsure record does |
| --- | --- |
| `drop` | It is left out of `--emit input`, and the count prints on standard error |
| `keep` | It is selected |
| `error` | The run ends at that record with exit code 8 |

`--unknown keep` needs no result-bearing mode, because the record still prints under every mode.

Measurement of the first decider model showed answers inside the unsure band flipping between identical runs 5% to 14% of the time. An unsure record is a record to look at again. It is not a record the model called false.

## Order and parallelism

`--jobs N` bounds how many requests are in flight at once. The default is 4.

Output order never depends on `--jobs`. The tool holds finished rows in a bounded buffer until the rows before them are written. A faster later record never attaches itself to an earlier one.

The first decider model accepts about 1,200 requests per minute. A run over that rate gets a 429, and [backends.md](backends.md) fixes the retry. `--jobs` is the lever that keeps a run under the rate.

When the program downstream closes the pipe, the tool stops reading and stops scheduling. Requests already sent may still be processed and billed.

## Limits

| Option | Meaning | Default |
| --- | --- | --- |
| `--jobs N` | Requests in flight at once | 4 |
| `--max-requests N` | The cap on requests for the whole run, counting retries | None |
| `--max-record-bytes N` | The cap on one record's evidence | 262144 |

A record over `--max-record-bytes` fails. The tool never truncates it. Under `--on-error stop` the run ends with exit code 5, and under `continue` the record gets an error row.

Reaching `--max-requests` ends the run with exit code 7. Printed output is a prefix of the input, and it is not a finished dataset.

The backend has its own limits. The first decider model takes about 32k tokens per question and about 65k per request. Evidence past that is refused by the backend, and the exit code is 4.

## Failures

`--on-error stop|continue` says what one record's failure does. The default is `stop`.

`stop` ends the run at the first failure. The exit code is the one the failure earns from [channels.md](channels.md): 4 for a backend failure, 5 for a local failure.

`continue` carries every record to the end. A failed record gets a row with an error status and a stable code, and the run exits 6. `continue` needs `--emit result` or `--emit annotated`, because an error has nowhere to go in `--emit input`.

An unsure answer is never retried. Asking again until the answer is acceptable is not a policy.

## Exit codes 6, 7, and 8

[channels.md](channels.md) reserves these for stream commands. This document assigns them.

| Code | Meaning |
| --- | --- |
| 6 | The run reached the last record under `--on-error continue`, and at least one record failed. Every record has a row |
| 7 | The run ended before the last record because a declared limit was reached. Printed output is a prefix |
| 8 | The run ended because a declared requirement was not met. `where --unknown error` met an unsure record, and `segment --unknown error` met an unsure boundary |

When more than one applies, 8 wins over 7, and 7 wins over 6. The most specific cause is the one worth reporting.

A run with no pass mark still exits 0. Codes 1 and 3 belong to `--status`, and no stream verb offers `--status`.

**Questions for Ian**

- Should `--max-requests` carry a default? A default of 1,000 protects a wallet from a large file. No default lets a complete job finish instead of turning into exit 7 partway. Recommendation: no default, and the help names the flag next to the cost warning.
- Should `--jobs` default to 4? Four is safe under every rate limit and leaves a hosted backend idle. A higher default finishes a large file sooner and risks more 429s. Recommendation: 4, and revisit it once a run has been measured.
- Should `--on-error continue` be allowed to change `--emit` on its own? Changing it saves the user a flag. Refusing keeps the printed shape a thing the user chose. Recommendation: refuse, and say in the error message which mode to pass.
