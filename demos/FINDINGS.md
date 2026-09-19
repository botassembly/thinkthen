# What the demos found

Every design finding from the fifteen demo pages, strongest argument first. Pages 01 to 12 are written to ADR 0007 and its clarifications. Pages 13, 14, and 15 also drive ADR 0008 and ADR 0009, both Proposed. Strength is how hard the demos push: **strong** means a demo could not be written without the change, **medium** means a demo worked but read badly or hid something, **weak** means a demo noticed it.

## Against ADR 0008 and ADR 0009

| Finding | Demos | Section | Proposed change | Strength |
| --- | --- | --- | --- | --- |
| An exact check has no name. `--truth /gold_code=/output_code` makes a check out of two pointers and ADR 0008 never says what to call it. Demo 14 keys it by the whole expression, and that key changes whenever a pointer changes | 14 | ADR 0008 item 4 | `--truth NAME=/A=/B`, or a name for the exact form. A metric a script reads needs a key a script can write | strong |
| A run made by a bare verb is one check named after the verb, so `--threshold decide=0.82` names a word the user never typed. Two runs of two different questions concatenate into one check called `decide`, and the only warning is a list of question texts | 13 | ADR 0007 clarifications; ADR 0008 item 4 | Name a bare-verb check after its question text, or make `report` refuse a file whose rows carry two texts | strong |
| No report shape is fixed anywhere. ADR 0008 names the metrics and ADR 0009 names the headline, and neither names a key. Every `jq` path in demos 13 and 14 is a proposal, `run`, `checks`, and `baseline` included | 13, 14 | ADR 0008 item 4; ADR 0009 item 7 | Fix the object in `report.md` before any code. A report is read by scripts and nothing else | strong |
| How `find` says that nothing fits is open. Demo 15 argues for a `none` option: it costs no second request, and the tool already spells the outcome as an empty standard output and exit 3, exactly as `choose` does. A second yes/no question doubles the request and can disagree with the pick it guards. A cut on `confidence` rests on a formula ADR 0009 item 2 calls unpublished | 15 | ADR 0009 item 3 | `find` inherits `--threshold`, `null`, and exit 3 from `choose`, with a `none` option carried in the one request | strong |
| ADR 0009 never says what `find` prints. Demo 15 prints the line byte for byte as `filter` prints a record, and `--details` has no answer kind to carry a pick over units | 15 | ADR 0009 item 3 | Fix the output in `find.md`: the unit byte for byte, the record under `--jsonl`, and an answer kind that names the chosen unit and its probability | strong |
| `find` sends the whole input in one request and nothing narrows it. `--field` narrows a record, and no option narrows a page | 15 | ADR 0009 item 3 | The `find` help leads with the fact. It is the one verb where all of the input leaves at once | medium |
| The 255-unit ceiling has no stated edge. ADR 0009 does not say whether a larger file is refused or cut | 15 | ADR 0009 item 3 | Refuse before any request and name the count, in the words `segment` uses for a document too large | medium |
| A case missing from a baseline counts as a change and nothing says why. Demo 14's `E-04` could have been refused, dropped by an exporter, or failed | 14 | ADR 0008 item 4 | `--baseline` says only that the case is missing, and the help warns against reading it as a result | medium |
| `answer` carries the vendor's `confidence` when one exists, and ADR 0009 calls its formula unpublished. A demo cannot assert on a field that may be absent and cannot explain one whose meaning nobody knows | 02, 05 | ADR 0009 item 2 | Keep `confidence` out of every example and every default rule until a measurement says what it is for | medium |
| A calibration table cannot be read from a small file. Eight rows cannot fill probability bands, and ADR 0008 fixes neither the bands nor the minimum | 13 | ADR 0008 item 4 | Name the bands, and say what the table prints when a band holds no rows | medium |
| The demo could not show a resumed run resuming. Every entry exists in a committed recording, so a rerun replays everything and the saving is asserted in prose. ADR 0008 item 5 leans on the resume for its claim that a completed run holds a judgment for every case | 12, 14 | ADR 0008 item 5 | No change. The claim belongs to a live check and not to a gate | medium |
| The demo could not assert the question `segment` sends. The tool adds two unit ids to the user's question and no document says how | 11 | ADR 0009 item 1 | Fix the text in `segment.md`. A user who changes a question's wording is running a new measurement and has to see the whole of it | medium |
| The demo could not reach the refusal for a document too large. A fixture big enough to trip a vendor limit would dwarf the demos, and the limit will move | 11 | ADR 0009 item 1 | The refusal names its exit code and the limit it measured against | weak |
| `--top N` is free on `find` and costs a request per record on `rank`. The two verbs say opposite things about one option name | 15, 06 | ADR 0009 item 3 | The `--top` row in both helps says what the option does not limit | weak |
| The two-file procedure needs no option, and demos 13 and 14 confirm it. Sweep one file, take a number, report the other file at that number | 13 | ADR 0009 item 7 | No change. The demos back the refusal of a held-out split flag | weak |

## Against ADR 0007

| Finding | Demos | Section | Proposed change | Strength |
| --- | --- | --- | --- | --- |
| Nothing counts what a finished record run did. A stopped run prints a line on standard error naming the record, the records finished, and the records replayed. A run that finishes prints nothing, so a `filter` that kept two of five is silent and a clean run over ten thousand records leaves no number to size the next run with | 03, 06, 12 | Records; the clarifications | Print the same line at the end of every record run, with the records dropped added for `filter` | strong |
| A top-level `threshold` in an `annotate` file reaches `decide` questions only. A file of twenty `choose` questions at one cut repeats that cut twenty times | 08 | The `annotate` file | The top-level key applies to every question that names none, whatever its verb | strong |
| `config show` reports the file and a run reports the run. `--profile` and `THINKTHEN_PROFILE` change what a run selects and never reach `show` | 10 | Configuration | `config show` takes the same selection flags a run takes, or its help says it reports the file | medium |
| A record run's `--dry-run` reads the first record and stops. A pointer missing on record two is invisible, and `input` names the pointer asked for and not the record that answered | 09, 14 | The clarifications | The plan names the record it came from | medium |
| One code for two repairs is still a cost. Exit 2 covers a mistyped flag and a bad record alike, and a `case` branch has to read the message to tell them apart | 03, 07, 12 | Exit codes | No change. The stopped-run line leads with the record, because that word is what a reader greps for | medium |
| A `choose --raw` run prints an empty line for an unresolved record. A pipeline that strips blank lines drops the record and shifts every later row | 02 | The clarifications | The `--raw` help says the blank line is a record | medium |
| `decide --jsonl` without `--details` prints `true`, `false`, and `null` with nothing tying a line to a record. Zipping with `paste` is a trap, because a stopped run prints a short file and the zip shifts | 04 | Records | The `--jsonl` help says a record-mode script that needs the record uses `--details` | medium |
| A careful `choose` script reads the exit code and then ignores it, because exit 0 and exit 3 both carry a usable `--details` object. The `case` exists only to let a real failure through | 05 | Commands, `choose` | The `choose` help shows the shape. Anybody who wants the distribution will hit it | medium |
| No request budget exists anywhere. A per-file loop is many runs and `jobs` bounds only the inside of one run. A ranked run over a million-line file makes a million requests with no lever | 05, 06 | Records; Configuration | No new flag. The record-flag help says a per-file loop is outside every budget, and shows `find -print0 \| xargs -0 -n 1 -P 4` next to the `jobs` setting | medium |
| `meta.usage` is the sum over a record's requests, so a file with one pointer and a file with two print the same shape. No page can prove the saving without asserting a token count | 07, 14 | The clarifications | No change. The help names the request count per record instead | medium |
| Parallelism left the command line. A user with a rate limit edits a JSON file to change one number for one run, and nothing on the command line points at the file | 03 | Configuration | No new flag. The record-flag help names the `jobs` key and the configuration path | medium |
| An unresolved boundary on `segment` silently joins, and no field on a segment records that anything was close | 11 | The threshold; Output | Each segment carries the probability of the gap that opened it | medium |
| `rank` buffers the whole stream, because an order needs every record. `filter` streams. The two read identically on the command line | 06 | Output, `rank` | One line in the `rank` help | medium |
| `annotate --dry-run` checks the question file and not the records, so it passes a file that fails on the first record because of a name collision | 07 | The `annotate` file | One line in the help saying what `--dry-run` does and does not check | weak |
| An unresolved answer is `null` everywhere. That is right in JSON and empty in CSV. A spreadsheet cell reading nothing and a listing that says nothing become the same cell | 07 | The `annotate` file | One line in the `annotate` help about the `jq -r '@csv'` step | weak |
| `--details` carries `input`, the whole record including what was never sent. A user with megabyte records pays for it on every row to read one probability | 04 | Output, `--details` | No change. The help names the cost | weak |
| A threshold is measured for one model and nothing on the command line says so | 10, 13 | The threshold | The `--threshold` help says the mark belongs to a model | weak |
| Under the default cut of 0.5 nothing is ever unresolved, so a two-way `if` routes a message the model was unsure about with no sign that it was close | 01 | The threshold | The `decide` help says a three-way gate needs a band, next to the warning about `set -e` | weak |
| A truth pointer names a field that `--field` and `on` are what keep off the wire. A user who forgets them sends the answer with the question and poisons the measurement | 13, 14 | `report`; Records | One line in the `report` help | weak |

## Settled by ADR 0007 and its clarifications

Earlier pages raised these, and the ADR answered them. They are listed once and not repeated above.

- `--replay DIR` is in the specification, with a miss as a local failure at exit 5.
- The result has one shape with a bare value in front of it, and `--status`, `unassessed`, and the symmetric `--min-prob` are gone.
- `answer` carries a probability for every option on `choose` and every level on `score`, and `question` carries `options` and `levels`. A margin test is one `jq` line, so `--min-gap` is not missed.
- Under `--lines` or `--jsonl`, `choose --raw` prints an empty line for an unresolved answer, so one line still stands for one record.
- Exit 2 covers a usage error and an input error alike. A name collision in `annotate` exits 2. `config check` exits 5 on a file it refuses.
- A record run that stops early prints one line on standard error naming the record, the records finished, and the records replayed.
- An `annotate` file may carry one top-level `threshold`.
- A record-mode `--dry-run` plan carries an `input` object naming the framing and the pointers.
- `config path` prints the file `--config` or `THINKTHEN_CONFIG` names. `config show` prints one JSON object with the file's own key names and every default filled in.
- `--quiet` exists on `decide` and `choose` only, and `--quiet` beside `--details` is a usage error.
- `segment` reads one document, `--lines` and `--jsonl` are usage errors on it, and `--window` is dropped. The whole document goes in one request.
- Every segment carries `start_line` and `end_line` beside the unit indices.
- Judged columns are flat top-level fields on the record, so a chain of judgments never nests. `--as`, `--emit`, and `--id` are gone with the problem.
- `decide how` is gone. `rank` orders by the probability of yes and takes no rubric.
- The saved question file is JSON, so the proposed Markdown grammar is dead.
- `key_env` is always present in the plan and `null` when no key applies. `meta` carries `url` and `meta.profile` is `null` for an ad-hoc backend.
- `--plan` is `--dry-run`, `--backend` is `--profile`, and the five backend environment variables are gone.
- `--none`, `--invert`, `--abstain-on`, `--from`, `--order`, `--on-error`, `--max-requests`, and `--min-gap` are all gone.

## Draft features no demo needed

Fifteen jobs were written before the code. These parts of the three ADRs were never reached for.

- `score`, its two-to-ten levels, and a `score` question inside an `annotate` file. No job wanted a level, and the measurement says the tool is weakest here.
- `--lines` outside `find` and `segment`. Every record job had a file of JSON lines.
- `--units paragraphs`. Demo 11 has no blank line in its fixture.
- `--key-env`. The one demo with a second backend gave it a profile whose `key_env` is `null`.
- `--` as the end of option parsing. No demo had a question or a label that begins with a dash.
- `THINKTHEN_CONFIG` and `THINKTHEN_PROFILE`. Demo 10 used flags, because a demo page has to show what it selects.
- `timeout_seconds` and `max_retries` from the configuration file. Demos 10 and 12 set them on the command line.
- A file profile named `jev` replacing the built-in one.
- `options` as a plain list of labels in an `annotate` file. Demo 08 used the map form.
- A structured question or option description, from ADR 0009 item 5. Every question on these pages is one sentence.
- `--options POINTER` on `choose`, from ADR 0009 item 4. No demo had a candidate list that changes per record.
- `--id POINTER` on `report`. Demo 14's cases carry `/id`, and that is the default.
- Exit codes 5 and 70. Demos 12 and 15 name codes in `case` branches and no page produces either.
- The `chat-logprobs` adapter.
