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

- The five fields each sit on their own line, in the order above. `request` and `response` are JSON values rather than strings. The adapter emits the request as one compact JSON value, so its stored value is the bytes sent. The stored response is the JSON value the backend returned; whitespace outside that value is not part of the entry. One exchange therefore writes the same file every time, and a diff shows which field changed.
- Only an exchange that succeeded and decoded is recorded. A failure is never recorded.
- An entry holds bodies and never headers. No key can reach a recording.
- Replay derives the digest-named entry for the requested exchange and reads only that path. It does not enumerate or validate other files in `DIR`, so an unrelated file is ignored. A requested entry that cannot be parsed is a local failure, exit code 5. The message names the entry and the line and column where reading stopped, and never the text it stopped on. An entry is written around the evidence, so quoting that text would print the evidence into a diagnostic.
- No refusal repeats any field of an entry. An entry that parses and names another schema is refused with a sentence that may name only the trusted fixed schema `thinkthen.recording/1` and never repeats the schema the entry named. Every field of a file under `DIR` is untrusted text: it is unbounded, it can carry a control byte, and it can quote the evidence back.
- An entry stores the address the request went to, its path included, so a token must never sit in the path of a base. A base carrying user information, a query, or a fragment is refused for the same reason.
- An entry is written completely to a private temporary name in `DIR` and closed. The tool then makes the final name as a hard link without replacing anything already there, so a reader never sees half a file. Every returned write path attempts to remove its temporary name. A process crash can leave a complete private dot-prefixed temporary file.
- A `DIR` the tool creates is readable by its owner alone, and so is every entry the tool writes, because a recording holds the evidence. On Unix that is mode `0700` for the folder and `0600` for each file, whatever the umask says. A `DIR` that already exists keeps the mode it has.
- The first complete entry installed for a digest stays there until the user removes it. Recording the same stored JSON response again succeeds without changing the entry. Whitespace outside the backend's JSON value and formatting around a valid version-one envelope do not distinguish responses. A different stored response is a local failure at exit 5. The message names the entry and repeats neither response. A damaged existing entry keeps its current safe refusal and is never replaced.
- New writes require hard-link support in `DIR`. A filesystem that refuses hard links returns a local recording failure at exit 5 and leaves an existing entry untouched.
- A cache miss takes an exclusive operating-system lock for its digest, then checks the entry again. Concurrent callers that share a cache send one backend request when the owner installs a complete entry. Waiters replay that entry and report `meta.replayed: true`. Record-only runs do not take this lock.
- Empty lock files stay under the private `.locks` directory. They carry only the lowercase digest as their name and contain no data. On Unix the directory has mode `0700` and files created by the tool have mode `0600`. Closing a file releases its lock after success, failure, or process death. No owner record or recovery step exists.

A successful recorded run can replay the answers it printed. A run that receives different responses for one digest stops at the conflict instead of saving a history that would replay differently. A repeated trial that wants another backend answer uses a fresh folder.

If a cache owner dies after the backend accepts the request and before it installs the entry, a waiter sends again because no durable answer exists. Closing that interruption window would require pending state and a recovery protocol.

The first live answers on 2026-09-19 returned the same probability for two identical requests. ADR 0010 holds the measurement.

## What replay changes in a result

`meta.replayed` is `true` when the answer came from a recording and `false` when a backend answered. It is always present. `meta.requests` carries the same digest that names the requested recording entry. Replay and live execution therefore report one request identity. Everything else in the result is what the recorded response yields, the usage included. A later token ledger counts only answers that a backend gave.

## A recording holds the evidence

The request body carries the evidence. A recording is as private as the input it was made from. Record only what may be kept, and keep a recording of private input out of version control.
