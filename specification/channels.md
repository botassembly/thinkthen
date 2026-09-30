# Channels

Status: **Settled** for version one, by ADR 0007, amended by ADR 0048.

`thinkthen` is an ordinary Unix program. Every command obeys these rules. On SIGINT or SIGTERM, it stops starting work. Each request already sent finishes within its attempt timeout. The command writes the output it finished, prints the stop line, then ends by the same signal, so a shell reports 130 or 143. A backend failure that ends a sent request after the signal is reported as the signal's stop. A second SIGINT or SIGTERM ends the command at once by that signal. It prints no stop line, and output not yet written is lost. A supervisor that sends SIGTERM should wait longer than `--timeout`, 30 seconds by default, before it kills the command, so sent requests can finish.

## The five channels

| Channel | Carries |
| --- | --- |
| Arguments | The command, its options, and the question. Never data |
| Standard input | The evidence. Never instructions |
| Standard output | Results, and nothing else. One bare JSON value, or one value per record |
| Standard error | Diagnostics for a person, except the final `thinkthen.run/1` line requested by `--facts`, which a script may parse |
| Exit code | The outcome class, from the table below |

## What the tool never does

- It never runs a command, and it never writes a file the user did not name.
- It never reads free text as a command. An unknown word is a usage error.
  Seven exact guessed words, `grep`, `if`, `classify`, `switch`, `sort`, `summarize` and `rewrite`, get a hint and still exit 2. The first five name `filter`, `decide`, `choose` or `tag`, and `rank`; the last two say the tool writes no text.
- Code parses the command line. The model reads only the question text, the options or levels, and the evidence.
- It never prints a key. No key appears in a plan, a result, a recording, or an error.

## Arguments

The grammar is the verb, then the question, then what the verb needs. No verb has a hidden default question. Options may sit before or after the operands, and `--` ends option parsing. A question that begins with a dash follows `--`.

`decide`, `filter`, `rank` and `find` take one question, and each option takes one value. A loose word is refused at exit 2: `the question is one argument and each option takes one value; quote a question of several words, and send evidence on standard input or as `--input FILE``.

An unknown option is a usage error. So is a repeated single-value option, and so is an option that cannot act in the chosen mode. Every one of them exits 2 before any request goes out.

Everyday options are `--threshold`, `--details`, `--quiet`, `--raw`, `--input FILE`, `--lines`, `--jsonl`, `--csv`, `--tsv`, `--field POINTER`, `--options POINTER`, `--top N`, `--none`, and `--plan`. The list names every one. Each verb's page says which of them it takes.

Advanced options appear in the long help alone: `--model`, `--timeout`, `--max-retries`, `--record DIR`, `--replay DIR`, `--cache DIR`, `--jobs N`, `--facts`, and `--batch N` and `--context FILE` on `decide`, `filter`, `rank`, `choose`, `tag` and `score`. `--url` and `--profile FILE` also appear in short help. They decide where evidence goes and whether a request is locally refused before it goes there.

## Standard input

Without `--input FILE` the evidence comes from standard input. When standard input is a terminal, the tool writes one line on standard error that says it is reading evidence from the terminal and how to end it, so a person does not read a waiting command as a hung one. The line never appears in a pipe, in a redirection, or under `--input`, and standard output is the same either way.

## Standard output

Standard output holds a bare JSON value. `true`, `"bug"`, and `1.6` are whole outputs. [result.md](result.md) gives the value for each command and the object that `--details` prints in its place.
The six ordinary judgment verbs may add row-scoped live `meta.attempts` under `--details`; `--facts` remains one terminal `thinkthen.run/1` line on standard error without an attempt list. A terminal failure with no printed detail row does not print a separate attempt array.

`--quiet` suppresses standard output on `decide` and `choose` over one document. Record mode refuses it because no record's answer sets the exit code. No other command takes it, because no other command carries its answer in the exit code. The exit code still reports the answer, and standard error still reports a failure. `--quiet` beside `--details` is a usage error. `--plan` may be added to any command line that is valid without it, and it prints the plan whatever view option stands beside it.

`--raw` prints a `choose` label without its quotation marks and prints nothing for `null`. No other command takes it.

A reader that closes the pipe early is no error. `thinkthen ... | head -1` ends quietly, and the command keeps the exit code it had earned.

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | The command finished. On single-input `decide`, the answer is yes |
| 1 | Single-input `decide` only: the answer is no |
| 2 | A usage error or an input error. The failing record sent nothing |
| 3 | Single-input `decide` and `choose`: the answer is not sure. `find --none`: nothing fits |
| 4 | The backend failed or sent a reply the adapter refused. For `check`, the report holds a critical line |
| 5 | A local failure: a file or a recording |
| 6 | `annotate` or `relate` completed with at least one valid and one failed logical question |
| 7 | `annotate --jsonl --details --batch 1 --on-error continue` completed with at least one missing-pointer error row |
| 70 | A defect in the tool |
| 130, 143 | SIGINT or SIGTERM stopped the command. It ends by that signal, and a shell reports 128 plus the signal's number |

Code 8 stays reserved. One function maps every error to its exit code. Exit 6 prints no diagnostic because the result marks each failed question. When a completed annotate run has both failed logical questions and missing-pointer error rows, exit 7 wins; a later terminal failure keeps its own code.

`recognize` exits 0 for every complete result, including no names. It never uses 1 or 3. A failed step-1, step-2, or relation request exits 4 and prints no partial value for that input. A text over `--max-text-bytes`, 600,000 bytes by default, exits 2 before any request.

`relate` exits 0 for a complete result, including no accepted edges. Recoverable mixed logical failure prints the buffered partial result and exits 6. If no valid logical answer remains, it exits 4 with no output. [relate.md](relate.md) fixes its aggregate behavior.

In record mode the exit code reports the run. A record run exits 0 when it completes without a partial or whole-run failure. The printed values carry the individual answers. `annotate` exits 6 when a completed run contains one or more failed questions; good answers and failed markers both print. Its explicit missing-pointer continuation exits 7 after one or more error rows. A valid answer on standard output can accompany exit 1, 3, 6, or 7, so a script that wants the value reads it and then reads `$?`.

```sh
asks_for_refund() {
  thinkthen decide 'the customer asks for a refund' --quiet
}
if asks_for_refund < message.txt; then
  echo refund
fi
```

A script that must tell a no from a not sure reads `$?` with `case`.

## A gate

A gate is a command whose exit code decides whether something happens. Word the question so that yes permits the action, and treat every exit code other than 0 as a refusal. A no, a not sure answer, a usage error, a backend failure, and a defect then all leave the action undone.

## `set -e` and `pipefail`

`decide` exits 1 on a no and 3 on a not sure answer. Under `set -e` a plain `thinkthen decide ...` ends the script on either one. Under `set -o pipefail` a `decide` inside a pipeline gives the whole pipeline a non-zero status for the same reason. Put the command in an `if`, a `case`, or a `||` list. The help says so.

## `--plan`

`--plan` prints what the command would send and then stops. It calls no backend and needs no key. It sends nothing, so `--record` or `--replay` beside it is a usage error.

For a one-document `decide`, `choose`, `tag` or `score` plan with standard output on a terminal, standard error says `thinkthen: plan: each question in request.questions quotes the evidence it asks about.` before the JSON appears. The hint contains no question or evidence text. With standard output piped or redirected, it does not appear, even if standard error is a terminal. Record-mode and other plans have no role hint; their evidence can sit in different parts of the request.

The first output line is one compact JSON document with four fields that are always present.

```json
{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY","request":{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"Help! My payouts have been failing for 3 days.\". Does this convey urgency?"}}}}
```

`request` is the body the adapter would send, as a JSON value and never as a string. `key_env` names the variable a key would be read from, so a script can prove that a key stays home. The plan never holds the value.

The second output line counts the entire validated input. `requests` counts prepared requests before cache answers, refusal splits and retries. `estimated_bytes` sums their exact UTF-8 body lengths; `estimated_input_tokens.lower` rounds down at 0.516 tokens per byte and `upper` rounds up at 0.908. These measured rates estimate input tokens, not a provider bill. `upper_bound` marks staged recognize and relate work whose later requests depend on answers.

```json
{"records":1,"requests":1,"estimated_bytes":219,"estimated_input_tokens":{"lower":113,"upper":199},"upper_bound":false}
```

The preview band rounds the sum of prepared bytes; live estimated-input admission rounds each actual attempt separately after cache, retry and split decisions. A plan sends nothing and does not reserve that admission total.

The plan carries the evidence, because the evidence is what leaves the machine. A plan deserves the same care as the request itself. The plan never holds a key.

In record mode `--plan` validates every record before printing. The first line discloses the first prepared request; the second counts the whole input. The first line carries a fifth field, `input`, naming the framing and pointers. When `filter` or `rank` took its framing by default, `input` also carries `"from":"default"`. Batched verbs use the same content, member and byte cuts as execution, without a pause boundary.

```json
{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY","input":{"framing":"jsonl","field":["/body"]},"request":{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"Payouts have failed for 3 days.\". Does this report a payment failure?"}}}}
```

A run that read a question file carries one more field, `from`, between `input` and `request`. It names only settings the verb takes, and gives each source as `file`, `command line`, or `default`, so a confused user can see what won. A run with no question file carries no `from`, and [question-file.md](question-file.md) gives the rest.

```json
{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY","from":{"question":"file","true":"file","false":"file","threshold":"command line","on":"default","model":"file"},"request":{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me please.\". Does this message ask for a refund?","criteria":{"true":"The writer asks for money back.","false":"The writer asks for anything else."}}}}}
```

`annotate --plan` also checks the saved file, and its `input` object names each question's pointers. [annotate.md](annotate.md) gives both.

`recognize --plan` reports the first record’s exact step-1 requests under `thinkthen.recognize-plan/2` and counts the complete input on the second line. [recognize.md](recognize.md) fixes that schema.

`relate --plan` reports the complete entity set, ordered rules, their fixed yes/no method and null fallback, and every exact shared request under `thinkthen.relate-plan/1`. Its second line marks the request count as an upper bound. It sends nothing. [relate.md](relate.md) fixes that schema.
