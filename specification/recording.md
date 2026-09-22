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

With none of these options, normal commands use the platform answer cache: `$XDG_CACHE_HOME/thinkthen`, then `$HOME/.cache/thinkthen` on Linux, and `$HOME/Library/Caches/thinkthen` on macOS. `THINKTHEN_CACHE` selects another folder. `--no-cache` disables answer-cache lookup and writing for one run. Explicit `--record` or `--replay` suppresses the default cache. Dry runs, help, version, and `cache prune` create no default cache.

An XDG home or `HOME` participates in these paths only when it is absolute. A relative or blank home is unusable and never resolves below the working directory.

The default cache folder is created only when the first valid prepared request reaches recording setup. A rejected record or invalid `find` set creates nothing. Replay of a missing explicit directory reports the request's normal replay miss and creates no directory. Replay of an existing read-only directory locks its already-open directory handle and changes no file or directory metadata.

The optional read-only configuration file is `$XDG_CONFIG_HOME/thinkthen/config.json`, then `$HOME/.config/thinkthen/config.json` on Linux, and `$HOME/Library/Application Support/thinkthen/config.json` on macOS. It has schema `thinkthen.config/1` and optional `url`, `model`, `cache`, and positive `cache_bytes` fields. Unknown fields are refused. The default is cache enabled with a target of 100,000,000 allocated bytes. The tool never creates or edits this file.

A recording directory argument that names a regular file is a local failure at exit 5. The diagnostic says to choose another path or remove the file. It repeats neither the path nor an operating-system error.

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
- A writing mode creates its private temporary entry before it reads the key or sends a request. An unusable recording folder therefore fails locally before a request. After a reply succeeds and decodes, the tool writes and syncs the complete entry. It makes a missing final name as a hard link without replacing anything already there. It replaces a damaged final name atomically while it owns that digest's lock. It syncs the recording folder before success, so a reader never sees half a file. Every returned path attempts to remove its temporary name. A process crash can leave a private dot-prefixed temporary file.
- A `DIR` the tool creates is readable by its owner alone, and so is every entry the tool writes, because a recording holds the evidence. On Unix that is mode `0700` for the folder and `0600` for each file, whatever the umask says. A `DIR` that already exists keeps the mode it has.
- The first valid complete entry installed for a digest stays there until the user removes it. Recording the same stored JSON response again succeeds without changing the entry. Whitespace outside the backend's JSON value and formatting around a valid version-one envelope do not distinguish responses. A different stored response is a local failure at exit 5. The message names the entry and repeats neither response. Replay alone refuses a damaged entry locally. A successful answer through `--record` or `--cache` replaces a damaged entry atomically, and a later replay reads the repair.
- New writes require hard-link support in `DIR`. A filesystem that refuses hard links returns a local recording failure at exit 5 and leaves an existing entry untouched.
- A missing or damaged entry takes an exclusive operating-system lock for its digest, then checks the entry again. Concurrent callers that share a cache send one backend request when the owner installs a complete entry. Waiters replay that entry and report `meta.replayed: true`. A record-only caller also uses this lock while an entry is missing or damaged. It releases the old lock before sending when the recheck finds a valid entry, so concurrent record-only callers against a valid entry each send once.
- Every recording or cache operation also holds a shared lock on the already-open directory handle from its first lookup through replay or installation. Replay creates no gate file and remains read-only. `cache prune` holds the exclusive side of the same gate, so it cannot split the digest-lock namespace while a request uses the folder.
- A completed lock file is removed while its owner still holds the original inode and after a valid final entry exists. An existing waiter remains on that inode, rechecks the valid entry, and does not send a duplicate cache request. A failed owner with no valid final entry leaves one empty lock file. Lock files carry only the lowercase digest as their name and contain no data. On Unix the `.locks` directory has mode `0700` and files created by the tool have mode `0600`. Closing a file releases its lock after success, failure, or process death. Lock files left by an older version remain until a later prune command. No owner record or recovery step exists.

Every recording storage failure exits 5 and prints `thinkthen: the recording folder could not be read or written; check its permissions and free space`. The message carries no path, entry bytes, evidence, credential, or operating-system error. On Unix the process safely handles `SIGXFSZ`, so a file-size limit reaches this failure and the normal temporary cleanup path instead of terminating the process. A later disk-full or sync failure can still discard an answer the backend already returned.

A successful recorded run can replay the answers it printed. A run that receives different responses for one digest stops at the conflict instead of saving a history that would replay differently. A repeated trial that wants another backend answer uses a fresh folder.

If a cache owner dies after the backend accepts the request and before it installs the entry, a waiter sends again because no durable answer exists. Closing that interruption window would require pending state and a recovery protocol.

The first live answers on 2026-09-19 returned the same probability for two identical requests. ADR 0010 holds the measurement.

## What replay changes in a result

`meta.replayed` is `true` when the answer came from a recording and `false` when a backend answered. It is always present. `meta.requests` carries the same digest that names the requested recording entry. Replay and live execution therefore report one request identity. Everything else in the result is what the recorded response yields, the usage included. A later token ledger counts only answers that a backend gave.

A recorded partial reply replays the same good answers, failed markers, failure count, and exit 6. Recording keeps the raw response and request bytes unchanged.

## A recording holds the evidence

The request body carries the evidence. A recording is as private as the input it was made from. Record only what may be kept, and keep a recording of private input out of version control.

## Pruning a cache

`thinkthen cache prune DIR` is the only cache-entry removal surface, and `DIR` is always explicit. `--older-than Nd|Nh|Nm|Ns` and `--answered-by-other-than MODEL` select a union. `--max-size BYTES` sets a separate target; without it, the configuration target applies. Selected entries leave first, then the oldest modification time leaves until recognized allocated bytes reach the target. Every chosen entry is deleted in modification-time order, with equal times sorted by digest name. An active digest lock is skipped without waiting. Its entry and allocated bytes remain in the final counts and may leave the cache over target. The command prints `removed N entries and B bytes; N entries and B bytes remain`.

Prune validates every digest-named regular file before it deletes one. The name must match the digest recomputed from the fixed adapter, stored URL, and exact request bytes, and the response must name a nonblank model. A digest-shaped symlink or other non-regular object is refused without following it. Unknown names, directories, locks, and dot-prefixed temporary files are ignored. A scan refusal deletes nothing. A later filesystem failure can leave an oldest prefix deleted and prints no success line.
