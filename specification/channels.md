# Channels

Status: **Settled** for version one, by ADR 0007.

`thinkthen` is an ordinary Unix program. Every command obeys these rules.

## The five channels

| Channel | Carries |
| --- | --- |
| Arguments | The command, its options, and the question. Never data |
| Standard input | The evidence. Never instructions |
| Standard output | Results, and nothing else. One bare JSON value, or one value per record |
| Standard error | Diagnostics for a person. Never parsed by a script |
| Exit code | The outcome class, from the table below |

## What the tool never does

- It never runs a command, and it never writes a file the user did not name.
- It never reads free text as a command. An unknown word is a usage error.
- Code parses the command line. The decider model reads only the question text, the options or levels, and the evidence.
- It never prints a key. No key appears in a plan, a result, a recording, or an error.

## Arguments

The grammar is the verb, then the question, then what the verb needs. No verb has a hidden default question. Options may sit before or after the operands, and `--` ends option parsing. A question that begins with a dash follows `--`.

An unknown option is a usage error. So is a repeated single-value option, and so is an option that cannot act in the chosen mode. Every one of them exits 2 before any request goes out.

Everyday options are `--threshold`, `--details`, `--quiet`, `--raw`, `--input FILE`, `--lines`, `--jsonl`, `--field POINTER`, `--top N`, `--dry-run`, and `--profile NAME`. Each verb's page says which of them it takes.

Advanced options appear in the long help alone: `--url`, `--adapter`, `--model`, `--key-env`, `--timeout`, `--max-retries`, `--record DIR`, `--replay DIR`, and `--config FILE`.

## Standard output

Standard output holds a bare JSON value. `true`, `"bug"`, and `1.6` are whole outputs. [result.md](result.md) gives the value for each command and the object that `--details` prints in its place.

`--quiet` suppresses standard output on `decide` and `choose`. No other command takes it, because no other command carries its answer in the exit code. The exit code still reports the answer, and standard error still reports a failure. `--quiet` beside `--details` is a usage error.

`--raw` prints a `choose` label without its quotation marks and prints nothing for `null`. No other command takes it.

A reader that closes the pipe early is no error. `thinkthen ... | head -1` ends quietly, and the command keeps the exit code it had earned.

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | The command finished. On single-input `decide`, the answer is yes |
| 1 | Single-input `decide` only: the answer is no |
| 2 | A usage error or an input error. The failing record sent nothing |
| 3 | Single-input `decide` and `choose` only: the answer is unresolved |
| 4 | The backend failed or sent a reply the adapter refused |
| 5 | A local failure: a file, a recording, the configuration |
| 70 | A defect in the tool |

Codes 6, 7, and 8 stay reserved. One function maps every error to its exit code.

In record mode the exit code reports the run. No record's answer sets it. A valid answer on standard output can accompany exit 1 or 3, so a script that wants the value reads it and then reads `$?`.

```sh
if thinkthen decide 'the customer asks for a refund' --quiet < message.txt; then
  echo refund
fi
```

A script that must tell a no from an unresolved reads `$?` with `case`.

## `set -e` and `pipefail`

`decide` exits 1 on a no and 3 on an unresolved answer. Under `set -e` a plain `thinkthen decide ...` ends the script on any no. Under `set -o pipefail` a `decide` inside a pipeline gives the whole pipeline a non-zero status for the same reason. Put the command in an `if`, a `case`, or a `||` list. The help says so.

## `--dry-run`

`--dry-run` prints what the command would send and then stops. It calls no backend and needs no key. It sends nothing, so `--record` or `--replay` beside it is a usage error.

The plan is one compact JSON document on standard output with six fields that are always present.

```json
{"profile":"jev","url":"https://api.typesafe.ai/v1/systemone","adapter":"systemone","model":"jev-latest","key_env":"TYPESAFE_API_KEY","request":{"state":"Help! My payouts have been failing for 3 days.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"Does this convey urgency?"}}}}
```

`request` is the body the adapter would send, as a JSON value and never as a string. `profile` is `null` for an ad-hoc backend. `key_env` is the name of the key variable, or `null` when no key would be sent, so a script can prove that a key stays home.

The plan carries the evidence, because the evidence is what leaves the machine. A plan deserves the same care as the request itself. The plan never holds a key.

In record mode `--dry-run` prints the plan for the first record and stops. It reads no further than that record, and the plan carries a seventh field, `input`, naming the framing and the pointers.

```json
{"profile":"jev","url":"https://api.typesafe.ai/v1/systemone","adapter":"systemone","model":"jev-latest","key_env":"TYPESAFE_API_KEY","input":{"framing":"jsonl","field":["/body"]},"request":{"state":"Payouts have failed for 3 days.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"Does this report a payment failure?"}}}}
```

`annotate --dry-run` checks the saved file and sends nothing, as [annotate.md](annotate.md) describes. `report` and `config` send nothing at any time, so `--dry-run` beside either is a usage error.

