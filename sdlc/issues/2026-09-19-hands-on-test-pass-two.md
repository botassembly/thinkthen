# thinkthen QA pass 2

Status: Closed on 2026-09-22. Merged into 2026-09-22-command-wording-and-help-fixes-for-0-1.md.

Worktree `the QA worktree`, level with main at `fcbe7fa`. Built with `cargo build --locked --release`. No tracked file changed (`git status --porcelain` empty at the end). Every command ran with `THINKTHEN_API_KEY` unset or set to the marker `MARKERKEY456`, and no live call was made. Scratch, the fake server, and output live under the session scratchpad.

The backend was `--replay` over the committed recordings plus one throwaway Python server on `127.0.0.1`. The server reads a `P=<number>` marker out of the evidence text and answers with that probability, so exact ties, exact cuts, and any order were built to order. It counts requests at `/count` and zeroes them at `/reset`. Four instances ran, on 18181 (answers) and on 18402, 18422, and 18500 (a 402, a 422, and a 500 that quote the key and the evidence back in the body). All four were stopped; `ps -eo pid,args | grep server.py` returns nothing and all four ports are down.

`filter` and `rank` are built. Ticket 0022 already holds pass one's findings, and nothing below repeats them.

**Headline: no wrong answer, no leak, no crash, and no hang was found in `filter` or `rank`.** The order, the ties, the cuts, the byte fidelity, the request counts, and the secrecy are all exactly what the pages promise. Every finding below is a message or a page.

---

## Findings

### 1. `filter` and `rank` say "the command is `decide`" when they refuse a question file of the wrong verb

Severity: **wrong message**.

```
$ thinkthen filter @choose-question.json --jsonl --input records.jsonl
thinkthen: the command is `decide` and the question file holds a `choose` question
rc=2

$ thinkthen rank @score-question.json --jsonl --input records.jsonl
thinkthen: the command is `decide` and the question file holds a `score` question
rc=2
```

The command is `filter`. The command is `rank`. Neither is `decide`, and neither word appears on the line the user typed.

`specification/question-file.md` line 19: "A command that names the wrong verb for the file is exit 2, because the line to fix is the one the user typed." The whole reason the page gives for exit 2 is that the message should point at the typed line. This message points at a verb the user never typed. `decide`, `choose`, and `score` each name themselves correctly, so only the two new verbs are wrong:

```
$ thinkthen choose @decide-question.json a b --dry-run
thinkthen: the command is `choose` and the question file holds a `decide` question
```

A reader of how-to 43, which runs `thinkthen filter @convention.json`, who edits `convention.json` from `decide` to `choose` gets told about a command that is not in the pipeline.

The exit code is right, the refusal is right, and the sibling refusals on the same two verbs name themselves correctly, which makes the gap look like one shared string:

```
thinkthen: --threshold: `filter` takes a single cut, so ask `decide --details` and split the three piles with `jq`
thinkthen: the question file's `threshold`: `rank` orders and never selects, so put a cut in `filter --threshold`
```

Smallest fix: pass the invoked command's own name into the wrong-verb refusal, the way the threshold refusal already receives it.

### 2. `--input` naming a directory blames standard input, and adds a record line for a failure that is not a record

Severity: **wrong message**.

```
$ thinkthen filter 'Q' --jsonl --field /b --input .
thinkthen: standard input could not be read: Is a directory (os error 21)
thinkthen: stopped at record 1; 0 records finished, 0 from a recording
rc=5
```

Standard input was a pipe and was never read. `--input` named the thing that failed. The same run with a path that does not exist gets the right noun, so the reader is there and only this branch misses it:

```
$ thinkthen filter 'Q' --jsonl --field /b --input nosuch.jsonl
thinkthen: --input could not be opened: No such file or directory (os error 2)
rc=5

$ thinkthen filter 'Q' --jsonl --field /b --input unreadable.jsonl
thinkthen: --input could not be opened: Permission denied (os error 13)
rc=5
```

A directory opens and then fails on the first read, so it falls past the `--input` branch into the generic reader. It is not a `filter` fault: `decide`, `rank`, and every verb behave the same way.

```
$ thinkthen decide 'Q' --input .
thinkthen: standard input could not be read: Is a directory (os error 21)
rc=5
```

The second line is wrong too. `specification/records.md` line 89: "A run that stops early prints one line on standard error: the record it stopped at, how many records it finished, and how many of those came from a recording." No record was read, the stream never opened, and "stopped at record 1" invites the reader to look at the first line of a file the tool never got into.

Exit 5 is right, and `specification/channels.md` line 57 calls a file a local failure.

Smallest fix: carry the `--input` path through to the read as well as the open, so the failing read names `--input` and skips the record line when no record was framed.

### 3. `rank --help` promises input order three times, and `rank` sorts

Severity: **page disagrees with the binary**.

```
$ thinkthen rank --help
      --lines
          Take each line as one text record.

          One value prints per record, in input order. ...

      --jsonl
          Take each line as one JSON record.

          One value prints per record, in input order. ...

      --jobs <N>
          ...
          Output never depends on it: a run with any number prints the bytes one job prints, in input order.
```

`specification/rank.md` line 19: "Each record as it arrived, most likely yes first. Ties keep input order." The same help's own opening paragraph says so. Measured, the binary sorts:

```
$ thinkthen rank 'Q' --jsonl --field /b --input bound.jsonl
{"b":"P=1 one"}
{"b":"P=0.90001 over"}
{"b":"P=0.9 exact"}
...
```

The three strings are shared with `decide`, `choose`, `score`, and `filter`, where they are true. Only `rank` reorders, so only `rank` reads falsely. The `--jobs` line is the one that costs a reader most, because its real claim (a run at any `jobs` prints the bytes that one job prints) is true and worth keeping, and the four words after it are what break it. I confirmed the real claim separately: `--jobs 1`, `4`, and `32` over 200 records gave byte-identical output on `filter` and on `rank`, and 50 records at one probability came back in input order at `--jobs 1` and at `--jobs 32`.

Smallest fix: give `rank` its own wording for those three help strings, ending "in the order `rank` prints" rather than "in input order".

### 4. `demos/README.md` says `filter` and `rank` are not built, three rows above three green pages that use them

Severity: **page disagrees with the binary**.

`demos/README.md`, in "Red and green": "A demo starts **red**, and this list marks it **coming** with the ticket or the slice that writes it. Only `decide`, `choose`, and `score` are built, so every page that needs another verb is still a plan."

The same file's "Many records" table lists 03 (`filter`, green), 43 (`filter`, green), and 06 (`rank`, green). All three run. I copied each folder to scratch beside a copy of `transforms/` and ran every fenced block by hand with the key unset: 11 blocks across the three pages, 0 failures.

`sdlc/planning/plan.md` line 31 records slice 8 as Done. Ticket 0021 rewrote this file and the sentence survived.

Smallest fix: the sentence becomes a list of the verbs still unbuilt, `annotate` and `find`, which is what the remaining "coming" rows need.

### 5. The README tells a newcomer the landed code speaks an earlier grammar, and it does not

Severity: **page disagrees with the binary**.

`README.md` line 13: "Those commands are the design. `specification/` is the contract, and code follows it. The code that has landed still speaks an earlier grammar, and `sdlc/planning/plan.md` says where the change stands."

The three commands directly above that sentence are the first thing a newcomer reads, and all three run as written:

```
$ thinkthen decide 'Does the customer ask for a refund?' --dry-run < message.txt
{"url":"https://api.typesafe.ai/v1/systemone",...,"questions":{"q1":{"type":"noul","instructions":"Does the customer ask for a refund?"}}}
rc=0

$ thinkthen choose 'Which kind of request is this?' bug feature question other --dry-run < issue.txt
rc=0

$ thinkthen filter 'Does this describe a bug that can be reproduced?' --jsonl --field /body --dry-run < issues.jsonl
rc=0
```

`sdlc/planning/plan.md` marks slices 3, 4, 5, 7, 7b, and 8 Done, which is every verb the README shows. The sentence tells a newcomer to distrust the one thing on the page they can check in ten seconds.

Smallest fix: drop the clause, or narrow it to `annotate` and `find`, which the specification holds and the binary does not offer.

### 6. The `--field` long help on `filter` and `rank` describes a mode neither verb has

Severity: **page disagrees with the binary** (small).

```
$ thinkthen filter --help
      --field <POINTER>
          ...
          Give it more than once to send an object of the named parts, keyed by the last part of each pointer. Without --jsonl it reads the whole input as one JSON value. ...
```

`specification/filter.md` line 15 and `rank.md` line 13: one of `--lines` and `--jsonl` is required. "Without `--jsonl`" on these two verbs means `--lines`, and `--lines` refuses `--field`:

```
$ thinkthen filter 'Q' --lines --field /b --input lines.txt
thinkthen: --field: a text line has no members, so --lines takes no pointer
rc=2

$ thinkthen filter 'Q' --field /b --input one.jsonl
thinkthen: `filter` maps over a stream, so it takes --lines or --jsonl
rc=2
```

Both refusals are correct and well worded. The sentence in the help describes a third case that cannot be reached on either verb. The string is shared with `decide`, `choose`, and `score`, where it is true. The same paragraph's first two sentences are true everywhere and were confirmed: `--field /q --field /p` sends `{"q":...,"p":...}` and `path` stayed out of the request.

Smallest fix: drop that one sentence from the `filter` and `rank` copies of the string.

### 7. `filter --details` prints `value` as a boolean, and `result.md` says `value` is the record

Severity: **papercut**.

```
$ thinkthen filter 'Q' --jsonl --field /b --details --input bound.jsonl
{"value":true,...,"answer":{"kind":"yes_no","probability":0.9},"threshold":0.5,...}
{"value":false,...,"answer":{"kind":"yes_no","probability":0.4999},...}
```

`specification/result.md` line 28: "`value` is the bare value the command would have printed." Line 14 of the same page says `filter`'s bare value is "each kept record, byte for byte as it arrived, in input order". Read together, `value` on a `filter` row should be the record, which would be useless and would duplicate `input`.

The boolean is the right thing to print, and it is the only field that says whether the row was kept: `value: true` matched the kept set exactly at the default cut and at `--threshold 0.9` (3 of 7 both ways). `rank` prints `value: null`, which `rank.md` line 43 states outright. `filter.md` says nothing about `value` at all.

Smallest fix: one row on `result.md` saying that `value` on a `filter` row is the boolean the cut produced, and that `filter` keeps the row when it is `true`.

### 8. A `rank` plan names a `threshold` source for a setting `rank` does not have

Severity: **papercut**.

```
$ thinkthen rank @question.json --jsonl --dry-run --input records.jsonl | jq -c '.from'
{"question":"file","true":"file","false":"default","threshold":"default","on":"file","model":"default"}
```

`specification/rank.md` line 37: "`rank` takes no `--threshold`, no `--quiet`, and no `--raw`." Line 43: "Every ranked row carries `threshold: null` and `value: null`, because `rank` reads no rule and makes no selection." `specification/channels.md` line 100 says `from` "names each setting's source ... so a confused user can see what won". On `rank` this key is always `default` and can never be anything else, so it answers a question no `rank` user can ask, and it suggests a rule is in play that `rank` then reports as `null`.

The refusals around it are all correct: a `threshold` in the file is exit 5 with a message naming the file's key, and a `--threshold` on the command line is exit 2.

Smallest fix: leave `threshold` out of `from` on `rank`, and say on `channels.md` line 100 that `from` names only the settings the verb takes.

### 9. Every `filter` and `rank` row says its verb is `decide`

Severity: **papercut**.

```
$ thinkthen filter 'Q' --jsonl --field /b --details --input one.jsonl | jq -c .question
{"verb":"decide","text":"Q"}

$ thinkthen rank 'Q' --jsonl --field /b --details --input one.jsonl | jq -c .question
{"verb":"decide","text":"Q"}
```

`specification/result.md` line 22: "`question` names the verb and the text the model received." A user who saves rows from a `filter` run and a `rank` run into one file cannot tell them apart from `question.verb`, and `verb` is the only field that would have said. A `rank` row is distinguishable by `threshold: null`, a `filter` row is not distinguishable from a `decide` row at all.

Arguably right, because `filter` and `rank` ask a `decide` question and replay a `decide` recording, which I confirmed: a recording made by `decide` over 200 records answered `filter` and `rank` with zero requests on the counter, and `filter` kept exactly the 90 records `decide` called `true`. The page does not say which reading it means, so the field is ambiguous rather than wrong.

Smallest fix: one sentence on `result.md` saying that `question.verb` names the question's kind and not the command, so a `filter` and a `rank` row both read `decide`.

### 10. A record that is not UTF-8 is reported as "the evidence"

Severity: **papercut**.

```
$ thinkthen filter 'Q' --jsonl --field /b --input bad-bytes.jsonl
{"b":"P=0.9"}
thinkthen: the evidence is not valid UTF-8
thinkthen: stopped at record 2; 1 records finished, 0 from a recording
rc=5
```

`specification/records.md` line 59: "A record whose bytes are not valid UTF-8 is refused at exit 5, because bytes that are not text are a local failure rather than a record the tool read." The page's noun is "a record". The message's noun is "the evidence", which under `--field` is a strictly smaller thing than the record, so a reader with `--field /b` looks at the `body` member when the bad byte may sit anywhere in the line. The exit code and the prefix are right, and the run stopped where it should.

This is the same shape as pass one's finding 5 in a place that finding did not reach.

Smallest fix: "the record is not valid UTF-8".

---

## What worked

- **Byte fidelity.** Under `--jsonl`: odd spacing inside a record, spaces around the colons and braces, `\uXXXX` escapes left as escapes, an emoji, and a final record with no line feed all came back unchanged, with one line feed added at the end. Under `--lines`: trailing spaces kept, a CRLF stripped to a bare line feed, a carriage return in the middle of a line kept, UTF-8 kept, and a final line with no line feed given one. `--details` re-encodes `input` as a JSON value, which is what `result.md` line 79 describes. `rank` does the same on both framings.
- **Order and ties.** `filter` and `rank` gave byte-identical output at `--jobs 1`, `4`, and `32` over 200 records, on standard output and on standard error, finished and stopped. `filter` kept input order exactly. `rank` sorted descending over 4,000 records with no inversion. Ties: four probabilities in three interleaved groups came back grouped and in input order inside each group at all three job counts, and 50 records at one probability came back 0 to 49 at `--jobs 1` and `--jobs 32`.
- **The cut.** Exactly at the cut is kept, just under is dropped, at every form. `--threshold 0.9` over p of 1, 0.90001, 0.9, 0.8999, 0.5, 0.4999, and 0 kept the first three. The default kept p of 0.5 and dropped 0.4999. `--threshold 1` kept only p of 1. `--threshold 0` is refused: `a single cut is above zero and at most one`. A band is refused on the command line at exit 2 and in a question file at exit 5, each with its own sentence naming `decide --details`.
- **Exit codes.** `filter` over records that were all dropped exits 0, which is `filter.md` line 49. Empty input exits 0 with no output and **0 requests on the counter**. One record kept, one record dropped, every record kept, and no record kept all exit 0.
- **A failed record.** A missing pointer at record 4 of 5: `filter` exits 2 with the first three records already printed and `thinkthen: the record holds nothing at '/b'` plus the stopped line. `rank` exits 2, prints **zero bytes** on standard output, and says so: `stopped at record 4; 3 records finished, 0 from a recording, and nothing was printed because an order needs every record`. That sentence is `rank.md` line 41 word for word and it is the best message in the tool. The same holds under `--details` and at `--jobs 4`, byte for byte.
- **`--top`.** 1 gives 1 row, 7 over 7 records gives 7, 99 over 7 gives 7, and 0 is `thinkthen: --top prints the first N of the order, and N is 1 or more` at exit 2. Every non-number falls out at exit 2. `--top 3 --details` prints 3 rows, the top 3, which is `rank.md` line 30. **`--top` saves no request**: 7 records with `--top 1` and 7 records with no `--top` both put 7 on the counter, and 4,000 records cost 4,000 either way.
- **Refusals by name.** `filter --quiet`, `filter --raw`, `rank --quiet`, `rank --raw`, and `rank --threshold` each get a `thinkthen:` sentence naming the option and the command that carries it, at exit 2. A missing framing is `thinkthen: 'filter' maps over a stream, so it takes --lines or --jsonl`. `--lines` beside `--jsonl`, a repeated `--threshold`, and `--jobs 0` or `33` are each exit 2.
- **Question files.** A `decide` file drives both verbs, and `on` acts as `--field`. `--threshold`, `--true`, and `--field` each replace the file's value and flip that one key of `from` to `command line`. A `choose` file and a `score` file are each refused at exit 2 (finding 1 is the wording alone), and a band in the file is exit 5.
- **Recording and replay.** `--cache` over 200 records: 200 requests and 200 entries on the first run, **0 requests** on the second, byte-identical output, folder `0700` and each entry `0600`. `--record` then `--replay`: 200 then 0. A resume after a stop at record 101 paid for exactly the 101 that were left. `--record` or `--replay` beside `--dry-run`, and `--cache` beside either, are each refused.
- **A `decide` recording under `filter` and `rank`.** A folder written by `decide 'Shared question.' --jsonl --field /body --record` over 200 records answered `filter` and `rank` on the same question and the same records with **0 requests** on the counter, `meta.replayed` true on every row, and the 90 kept records matching the 90 `true` rows of the `decide` run. `decide` replayed the same folder back to a byte-identical run. This is `filter.md` line 45 and `rank.md` line 43.
- **A closed reader.** `filter ... | head -1` over 200 records: 6 requests at `--jobs 1`, 15 at `--jobs 8`, nothing on standard error, no error. `rank ... | head -1` made all 200, which is right, because an order needs every record.
- **Scale.** 4,000 records of about 100 bytes: `rank` through the fake server took 20.3 s at `--jobs 8` with a peak resident set of 9.5 MB, and the replay of the same run took 0.02 s at 4.3 MB. `filter` over the same replay took 0.01 s at 3.6 MB. Nothing grew with the record count that a whole-stream hold does not explain.
- **Secrecy.** 20 runs across `filter` and `rank` covering success, `--details`, `--dry-run`, `--record`, a missing key, a missing pointer, a replay miss, and servers answering 402, 422, and 500 that quoted the key and the evidence back in the body. The key `MARKERKEY456` appears **0 times** in any standard output, any standard error, and any of the two recordings written. The marker evidence appears **0 times** in any standard error. It appears on standard output only in a kept record, in `--details` `input`, in the `--dry-run` plan, and in a recording, all four by design. The error lines carry the status and the fixed phrase and nothing else: `thinkthen: the backend answered with status 402: the account has no credit`.
- **Help against the binary.** Every option the short help of `filter` and `rank` lists is accepted, and `--url`, `--model`, `--record`, `--replay`, `--cache`, `--jobs`, `--timeout`, and `--max-retries` appear in the long help alone on both, which is `channels.md` line 32. `--top` is refused on `filter` and `--threshold` on `rank`, as their pages say. The top-level help lists the five built commands and nothing else. `filter.md` line 37 and `rank.md` line 35 name `--url` and `--model` as the backend options and leave `--timeout` and `--max-retries` out of both tables, though both verbs accept them; `channels.md` line 32 covers them, so the two tables are incomplete rather than wrong.
- **Record rules on the new verbs.** A duplicate member name, `NaN`, a blank line under `--jsonl`, a non-JSON record, a pointer that finds nothing, `$.body`, and `--field` beside `--lines` each behave on `filter` and `rank` exactly as `records.md` gives them, at the right code, with the prefix rule honored on each.
- **The terminal notice.** Through a real pseudo-terminal on standard input, `filter` and `rank` each print `thinkthen: reading evidence from the terminal; end it with Ctrl-D on a line of its own` on standard error with the output unchanged. Through a pipe, standard error is empty.
- **How-tos 03, 06, and 43.** Copied to scratch beside a copy of `transforms/` and every `bash` block run in order with the key unset: 4, 3, and 4 blocks, 0 failures. Every shown output matched the page. None of the three reaches `../../transforms/`, so all three run from a copy of their own folder, unlike the six pages pass one found.
- **How-tos 40 and 41.** 4 and 5 blocks, 0 failures, after tickets 0020 and 0021. 41 still reaches `../../transforms/` in four places, which is pass one's finding 10.
- **The pinned plan.** The `decide --dry-run` digest that `spec/decide.md` pins reproduces byte for byte after 0020 and 0021, and the short-and-long help split that the same page pins holds for `decide` and for both new verbs.
- **Old recordings.** `probes/replay-check.sh` on `02-confidence` and `07-true-and-false-texts`: 60, 40, and 40 rows reproduced from the committed recordings alone, with no network and no key.
- **The README and `demos/README.md`.** Every link in both resolves. All three README commands run. Every status line in `demos/README.md` matches the status line of the folder it names, and every "coming" row names a verb that is genuinely unbuilt.
- **Pass one's clap findings, in new places.** `filter --top 2`, `rank --top -1`, `rank --top abc`, and `rank --top ''` each exit 2 but fall out of clap, and `--top -1` still gets the "to pass '-1' as a value, use '-- -1'" tip. Same root as pass one's findings 6 and 7, which ticket 0022 holds. `1 records finished` is pass one's finding 9 and appears on both new verbs.

## What I could not test

- `annotate` and `find`, and `specification/annotate.md` and `find.md`. The binary offers five subcommands and neither of those two.
- Exit code 70 on `filter` or `rank`. No input reached a defect path.
- A record over 16 MiB on `filter` or `rank`. Pass one measured it on `decide` and the reader is shared, so I spent the time on the new order and tie rules instead.
- `rank` against a stream large enough to prove the whole-stream hold is a problem. 4,000 records cost 9.5 MB, which says the hold is real and cheap at that size and says nothing about a file of ten million.
- Retry timing on `filter` and `rank`. Pass one measured the backoff on `decide` and the client is shared.
- Whether `--top` could save a request. `rank.md` line 27 says it cannot, the binary agrees, and no measurement can show that a saving is impossible rather than merely absent.
