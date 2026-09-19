# Recording and replay

Status: **Settled** for version one, by an agent under ADR 0005. Ian has not read it, and he can overturn it cheaply until demos depend on it. Every token count and probability in an example here is illustrative.

A recording is a folder of backend exchanges. It lets a command run again with no network, no key, and the same answer. Demos and tests replay recordings, so no gate ever reaches a backend.

## The options

| Option | Meaning |
| --- | --- |
| `--record DIR` | Call the backend, then write the exchange into `DIR`. `DIR` is created when absent |
| `--replay DIR` | Answer from `DIR` alone. Open no connection and read no key. A request that `DIR` lacks is a local failure, exit code 5, and the message names the missing entry |
| Both, with the same `DIR` | A cache. An entry that exists is replayed. A request that is absent goes to the backend and is recorded |
| `--cache DIR` | The row above, written once. It stands beside neither `--record` nor `--replay`, because one run keeps one folder |

Giving both options with two different folders is a usage error. So is giving either option beside `--dry-run`, because a plan sends nothing and reads nothing.

Both options on one folder are also the resume for a record run. [records.md](records.md) shows it.

## An entry

One exchange is one file named `DIGEST.json`. `DIGEST` is the SHA-256, in lowercase hex, of the adapter name, a newline, the URL, a newline, and the request body exactly as the adapter encoded it. An adapter encodes the same plan to the same bytes every time, so the same command finds the same entry.

```json
{
  "schema": "thinkthen.recording/1",
  "adapter": "systemone",
  "url": "https://api.typesafe.ai/v1/systemone",
  "request": {"state":"Help! My payouts have been failing for 3 days.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"Does this convey urgency?"}}},
  "response": {"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":312,"output_tokens":48}}
}
```

- The five fields each sit on their own line, in the order above. `request` and `response` are JSON values rather than strings, and each one is copied exactly as the bytes that crossed the wire, so an entry is a faithful copy of the exchange. One exchange therefore writes the same file every time, and a diff shows which field changed.
- Only an exchange that succeeded and decoded is recorded. A failure is never recorded.
- An entry holds bodies and never headers. No key can reach a recording.
- A file under `DIR` that is not an entry is a local failure, exit code 5. The message names the file and the line and column the reading stopped at, and never the text it stopped on. An entry is written around the evidence, so quoting that text would print the evidence into a diagnostic.
- No refusal repeats any field of an entry. An entry that parses and names another schema is refused with a sentence that names no schema. Every field of a file under `DIR` is untrusted text: it is unbounded, it can carry a control byte, and it can quote the evidence back.
- An entry stores the address the request went to, its path included, so a token must never sit in the path of a base. A base carrying user information, a query, or a fragment is refused for the same reason.
- An entry is written to a temporary name in `DIR` and then renamed, so a reader never sees half a file. A write that fails takes its temporary file with it.
- A `DIR` the tool creates is readable by its owner alone, and so is every entry the tool writes, because a recording holds the evidence. On Unix that is mode `0700` for the folder and `0600` for each file, whatever the umask says. A `DIR` that already exists keeps the mode it has.
- Recording the same request again replaces the entry. A decider model can answer differently on another day, and the newest answer wins.

The first live answers on 2026-09-19 returned the same probability for two identical requests. ADR 0010 holds the measurement. A repeated trial therefore means something only when the candidate's output changes.

## What replay changes in a result

`meta.replayed` is `true` when the answer came from a recording and `false` when a backend answered. It is always present. Everything else in the result is what the recorded response yields, the usage included. A later token ledger counts only answers that a backend gave.

## A recording holds the evidence

The request body carries the evidence. A recording is as private as the input it was made from. Record only what may be kept, and keep a recording of private input out of version control.
