# What the demos found

Every design finding from the demo pages, strongest argument first. Pages 01 to 09 and 12 are written to ADR 0007 and its clarifications. Pages 13, 14, and 15 also drive ADR 0008 and ADR 0009, which ADR 0010 accepted in part. Strength is how hard the demos push: **strong** means a demo could not be written without the change, **medium** means a demo worked but read badly or hid something, **weak** means a demo noticed it.

ADR 0010 took `segment`, `report`, and the configuration file out of version one. The findings that belonged to them left this page with demo 11, demo 10, and `report.md`, and the git history keeps them. Two of those questions carry to the `jq` recipes of the plan's recipes slice: how a check made by a bare verb is named, and how a truth label stays off the wire.

## Against ADR 0008 and ADR 0009

| Finding | Demos | Section | Proposed change | Strength |
| --- | --- | --- | --- | --- |
| How `find` says that nothing fits is open. Demo 15 argues for a `none` option: it costs no second request, and the tool already spells the outcome as an empty standard output and exit 3, exactly as `choose` does. A second yes/no question doubles the request and can disagree with the pick it guards. A cut on `confidence` rests on a formula ADR 0009 item 2 calls unpublished | 15 | ADR 0009 item 3 | `find` inherits `--threshold`, `null`, and exit 3 from `choose`, with a `none` option carried in the one request | strong |
| `find` has no answer kind. `yes_no` carries one probability and `choice` carries one per option, and neither names a unit that arrived on standard input. `find.md` now fixes what `find` prints and leaves `--details` open | 15 | ADR 0009 item 3 | A fourth answer kind that names the chosen unit and carries a probability per unit | strong |
| `find` sends the whole input in one request and nothing narrows it. `--field` narrows a record, and no option narrows a page | 15 | ADR 0009 item 3 | The `find` help leads with the fact. It is the one verb where all of the input leaves at once | medium |
| `answer` carries the vendor's `confidence` when one exists, and ADR 0009 calls its formula unpublished. A demo cannot assert on a field that may be absent and cannot explain one whose meaning nobody knows | 02, 05 | ADR 0009 item 2 | Keep `confidence` out of every example and every default rule until a measurement says what it is for | medium |
| The demo could not show a resumed run resuming. Every entry exists in a committed recording, so a rerun replays everything and the saving is asserted in prose. ADR 0008 item 5 leans on the resume for its claim that a completed run holds a judgment for every case | 12, 14 | ADR 0008 item 5 | No change. The claim belongs to a live check and not to a gate | medium |
| `find` prints one unit and cannot be asked for three. One request already answered the whole page, so more units would cost nothing. No ADR names the option, so `find.md` holds it open rather than inventing a flag | 15 | ADR 0009 item 3 | A demo that needs a short list from one request brings the option in | weak |

## Against ADR 0007

| Finding | Demos | Section | Proposed change | Strength |
| --- | --- | --- | --- | --- |
| Nothing counts what a finished record run did. A stopped run prints a line on standard error naming the record, the records finished, and the records replayed. A run that finishes prints nothing, so a `filter` that kept two of five is silent and a clean run over ten thousand records leaves no number to size the next run with | 03, 06, 12 | Records; the clarifications | Print the same line at the end of every record run, with the records dropped added for `filter` | strong |
| A top-level `threshold` in an `annotate` file reaches `decide` questions only. A file of twenty `choose` questions at one cut repeats that cut twenty times | 08 | The `annotate` file | The top-level key applies to every question that names none, whatever its verb | strong |
| A record run's `--dry-run` reads the first record and stops. A pointer missing on record two is invisible, and `input` names the pointer asked for and not the record that answered | 09, 14 | The clarifications | The plan names the record it came from | medium |
| One code for two repairs is still a cost. Exit 2 covers a mistyped flag and a bad record alike, and a `case` branch has to read the message to tell them apart | 03, 07, 12 | Exit codes | No change. The stopped-run line leads with the record, because that word is what a reader greps for | medium |
| A `choose --raw` run prints an empty line for an unresolved record. A pipeline that strips blank lines drops the record and shifts every later row | 02 | The clarifications | The `--raw` help says the blank line is a record | medium |
| `decide --jsonl` without `--details` prints `true`, `false`, and `null` with nothing tying a line to a record. Zipping with `paste` is a trap, because a stopped run prints a short file and the zip shifts | 04 | Records | The `--jsonl` help says a record-mode script that needs the record uses `--details` | medium |
| A careful `choose` script reads the exit code and then ignores it, because exit 0 and exit 3 both carry a usable `--details` object. The `case` exists only to let a real failure through | 05 | Commands, `choose` | The `choose` help shows the shape. Anybody who wants the distribution will hit it | medium |
| No request budget exists anywhere. A per-file loop is many runs and `jobs` bounds only the inside of one run. A ranked run over a million-line file makes a million requests with no lever | 05, 06 | Records | No new flag. The record-flag help says a per-file loop is outside every budget, and shows `find -print0 \| xargs -0 -n 1 -P 4` next to the `jobs` setting | medium |
| `meta.usage` is the sum over a record's requests, so a file with one pointer and a file with two print the same shape. No page can prove the saving without asserting a token count | 07, 14 | The clarifications | No change. The help names the request count per record instead | medium |
| Parallelism left the command line. A user with a rate limit edits a JSON file to change one number for one run, and nothing on the command line points at the file | 03 | Records | No new flag. The record-flag help names the `jobs` setting | medium |
| `rank` buffers the whole stream, because an order needs every record. `filter` streams. The two read identically on the command line | 06 | Output, `rank` | One line in the `rank` help | medium |
| `annotate --dry-run` checks the question file and not the records, so it passes a file that fails on the first record because of a name collision | 07 | The `annotate` file | One line in the help saying what `--dry-run` does and does not check | weak |
| An unresolved answer is `null` everywhere. That is right in JSON and empty in CSV. A spreadsheet cell reading nothing and a listing that says nothing become the same cell | 07 | The `annotate` file | One line in the `annotate` help about the `jq -r '@csv'` step | weak |
| `--details` carries `input`, the whole record including what was never sent. A user with megabyte records pays for it on every row to read one probability | 04 | Output, `--details` | No change. The help names the cost | weak |
| A threshold is measured for one model and nothing on the command line says so | 10, 13 | The threshold | The `--threshold` help says the mark belongs to a model | weak |
| Under the default cut of 0.5 nothing is ever unresolved, so a two-way `if` routes a borderline message with no sign that it was close | 01 | The threshold | The `decide` help says a three-way gate needs a band, next to the warning about `set -e` | weak |

## Settled by the specification pages

Earlier drafts of these pages raised the following, and `specification/` now answers each one. They are listed once and not repeated above.

- `find.md` fixes what `find` prints: the chosen unit byte for byte, the record under `--jsonl`, and one unit rather than a list. More than 255 units is a usage error before any request.
- `annotate --dry-run` prints the plan as well as checking the file. The plan shows the first record's first `on` set, and its `input` object names every question's pointers.
- A base that is not an `http` or `https` address is a usage error before any request goes out, and so is a base carrying user information. ADR 0010 later left one wire shape, so nothing names one and nothing can be pointed at a server that does not speak it.
- An evidence object built from several pointers travels as compact JSON in one field, like any pointed value that is not a string.

## Settled by ADR 0007 and its clarifications

Earlier pages raised these, and the ADR answered them. They are listed once and not repeated above.

- `--replay DIR` is in the specification, with a miss as a local failure at exit 5.
- The result has one shape with a bare value in front of it, and `--status`, `unassessed`, and the symmetric `--min-prob` are gone.
- `answer` carries a probability for every option on `choose` and every level on `score`, and `question` carries `options` and `levels`. A margin test is one `jq` line, so `--min-gap` is not missed.
- Under `--lines` or `--jsonl`, `choose --raw` prints an empty line for an unresolved answer, so one line still stands for one record.
- Exit 2 covers a usage error and an input error alike. A name collision in `annotate` exits 2.
- A record run that stops early prints one line on standard error naming the record, the records finished, and the records replayed.
- An `annotate` file may carry one top-level `threshold`.
- A record-mode `--dry-run` plan carries an `input` object naming the framing and the pointers.
- `--quiet` exists on `decide` and `choose` only, and `--quiet` beside `--details` is a usage error.
- Judged columns are flat top-level fields on the record, so a chain of judgments never nests. `--as`, `--emit`, and `--id` are gone with the problem.
- `decide how` is gone. `rank` orders by the probability of yes and takes no rubric.
- The saved question file is JSON, so the proposed Markdown grammar is dead.
- `key_env` is always present in the plan, and `meta` carries the `url` that answered. Under ADR 0010 there is one key variable and one address, so neither field is ever `null`.
- `--plan` is `--dry-run`, and the five backend environment variables are gone. `--backend` was renamed once and then left with ADR 0010, and `roadmap.md` says so.
- `--none`, `--invert`, `--abstain-on`, `--from`, `--order`, `--on-error`, `--max-requests`, and `--min-gap` are all gone.

## Draft features no demo needed

Fifteen jobs were written before the code, and thirteen pages hold what is left of them. These parts of the three ADRs were never reached for.

- `score`, its two-to-ten levels, and a `score` question inside an `annotate` file. No job wanted a level, and the measurement says the tool is weakest here.
- `--lines` outside `find`. Every record job had a file of JSON lines.
- `--` as the end of option parsing. No demo had a question or a label that begins with a dash.
- `timeout_seconds` and `max_retries` from the configuration file. Demo 12 set them on the command line.
- `options` as a plain list of labels in an `annotate` file. Demo 08 used the map form.
- A structured question or option description. ADR 0010 struck it from version one, and every question on these pages is one sentence.
- `--options POINTER` on `choose`, from ADR 0009 item 4. No demo had a candidate list that changes per record.
- Exit codes 5 and 70. Demos 12 and 15 name codes in `case` branches and no page produces either.
