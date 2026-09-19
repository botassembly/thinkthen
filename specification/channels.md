# Channels

Status: **Settled** for version one.

`thinkthen` is an ordinary Unix program. Every command obeys these rules.

## The five channels

| Channel | Carries |
| --- | --- |
| Arguments | The command, its options, and the question. Never data |
| Standard input | The evidence. Never instructions |
| Standard output | Results, and nothing else. One JSON document, or one record per line |
| Standard error | Diagnostics for a person. Never parsed by a script |
| Exit code | The outcome class, from the table below |

## What the tool never does

- It never runs a command, and it never writes a file the user did not name.
- It never reads free text as a command. An unknown word is a usage error.
- Code parses the command line. The decider model reads only the question text, the options, and the evidence.
- It never prints a key, and no key appears in a plan, a result, a recording, or an error.

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | The command ran. Under `--status`, the accepted answer is yes |
| 1 | Under `--status` only: the accepted answer is no |
| 2 | Bad usage: an unknown option, a missing argument, or options that conflict |
| 3 | Under `--status` only: the answer is unsure |
| 4 | The backend failed: no key, a refused key, a timeout, an error status, or a reply the adapter cannot read |
| 5 | A local failure: input that cannot be read or is not valid UTF-8, or output that cannot be written |
| 70 | A defect inside `thinkthen` |

Codes 6, 7, and 8 are reserved for stream commands. Without `--status`, a no and an unsure both exit 0, because the command did its job and the result says what it found.

One function maps every error to its exit code.

A reader that closes the pipe early is no error. `thinkthen ... | head -1` ends quietly, and the command keeps the exit code it had earned.

## `--plan`

`--plan` prints what the command would send and then stops. It calls no backend and needs no key. The plan is one compact JSON document on standard output with six fields that are always present: `backend`, `url`, `adapter`, `model`, `key_env`, and `request`. `request` is the body the adapter would send, as a JSON value and never as a string. `backend` is `null` for an ad-hoc backend. `key_env` is the name of the key variable, or `null` when no key would be sent, so a script can prove that a key stays home. The request body carries the evidence, because the evidence is what leaves the machine. The plan never holds a key. `--plan` sends nothing, so it takes neither `--record` nor `--replay`, and either one beside it is a usage error.

## `--status`

`--status` turns a judgment into a shell condition. It needs a pass mark. The result still prints on standard output.

```sh
if thinkthen decide if 'asks for a refund' --min-prob 0.9 --status < message.txt > /dev/null; then
  echo refund
fi
```

A script that must tell a no from an unsure reads `$?` with `case`.
