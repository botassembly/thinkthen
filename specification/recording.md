# Recording and replay

Status: **Settled** for version one, by an agent under ADR 0005. Ian has not read it, and he can overturn it cheaply until demos depend on it.

A recording is a folder of backend exchanges. It lets a command run again with no network, no key, and the same answer. Demos and tests replay recordings, so no gate ever reaches a backend.

## The options

| Option | Meaning |
| --- | --- |
| `--record DIR` | Call the backend, then write the exchange into `DIR`. `DIR` is created when absent |
| `--replay DIR` | Answer from `DIR` alone. Open no connection and read no key. A request that `DIR` lacks is a local failure, exit code 5, and the message names the missing entry |
| Both, with the same `DIR` | A cache. An entry that exists is replayed. A request that is absent goes to the backend and is recorded |

Giving both options with two different folders is a usage error.

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
- An entry is written to a temporary name in `DIR` and then renamed, so a reader never sees half a file.
- Recording the same request again replaces the entry. A decider model can answer differently on another day, and the newest answer wins.

## What replay changes in a result

`meta.replayed` is `true` when the answer came from a recording and `false` when a backend answered. It is always present. Everything else in the result is what the recorded response yields, the usage included. A later token ledger counts only answers that a backend gave.

## A recording holds the evidence

The request body carries the evidence. A recording is as private as the input it was made from. Record only what may be kept, and keep a recording of private input out of version control.
