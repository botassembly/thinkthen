# thinkthen QA pass 1

Worktree `the test worktree`, branch `qa/2026-09-19`, at `8e2ea0e`. Built with `cargo build --locked --release`. No tracked file changed (`git status --porcelain` empty at the end). Every command ran with `THINKTHEN_API_KEY` unset or set to the marker `MARKERKEY123`, and no live call was made. Scratch, fake servers, and output live under `qa/`. All fake servers were stopped; `ps -eo pid,args | grep -E 'server\.py|raw\.py'` returns nothing and no port in 18080-18099 is held.

The binary reports `thinkthen 0.0.1`. `filter`, `rank`, `annotate`, and `find` are not built, which matches slices 8, 9, and 11 of `sdlc/planning/plan.md`. Everything in those four pages is untested.

**Headline: no wrong answer, no leak, no crash, and no hang was found.** Every finding below is a message, a page, or a papercut.

---

## Findings

### 1. A refused port, a query, a fragment, and a bad scheme all give one message, and it names the wrong rule

Status: Closed by ticket 0022.

Severity: **wrong message**.

```
$ thinkthen decide q --url 'http://localhost:99999' --dry-run
thinkthen: a base address begins with `http://` or `https://` and carries no user information
rc=2

$ thinkthen decide q --url 'http://localhost:' --dry-run
thinkthen: a base address begins with `http://` or `https://` and carries no user information

$ thinkthen decide q --url 'http://localhost?x=1' --dry-run
thinkthen: a base address begins with `http://` or `https://` and carries no user information

$ thinkthen decide q --url 'ftp://localhost' --dry-run
thinkthen: a base address begins with `http://` or `https://` and carries no user information
```

`specification/backends.md` line 31 states four separate rules: the port rule ("A port is digits naming a number from 0 to 65535... An empty port, a signed number, and a number past 65535 are each a usage error"), the user-information rule, the query and fragment rule, and the scheme rule. The same sentence says "The refusal message names the rule and never the base it refused." The binary refuses all four correctly at exit 2 and never echoes the base, but it names only two of the four rules, so a user with a port typo reads a message about a scheme and user information and has nothing to act on.

This matters because `sdlc/planning/plan.md` records "a port past 65535 that was silently dropped" as one of the four holes ticket 0019's reviewer found. The hole is closed; the message is not.

Smallest fix: split the one refusal into four, one sentence per rule, each naming its own rule and no base.

### 2. `recording.md` says a non-entry file under `DIR` is exit 5; the binary ignores it

Status: Closed by ticket 0029.

Severity: **page disagrees with the binary**.

```
$ mkdir c5 && cp good-entry.json c5/ && echo 'junk' > c5/notes.txt
$ printf 'Refund me please.' | thinkthen decide q --replay c5
true
rc=0
```

`specification/recording.md` line 37: "A file under `DIR` that is not an entry is a local failure, exit code 5." The binary reads only the one entry whose digest it needs and never looks at the rest of the folder, so a stray file passes unnoticed. When the binary does read a damaged entry it behaves exactly as the page says:

```
$ head -c 80 good-entry.json > c1/<digest>.json
$ printf 'Refund me please.' | thinkthen decide q --replay c1
thinkthen: the entry `2f62...d8e.json` was refused: the file is not a recording entry: the JSON at line 4 column 15 is not one
rc=5
```

Lazy reading is the better behavior; the page is what is wrong. Smallest fix: change line 37 to say that an entry the run reads and cannot parse is exit 5, and that the tool reads no file it does not need.

### 3. `recording.md` says a foreign schema is refused "with a sentence that names no schema"; the sentence names one

Status: Closed by ticket 0030.

Severity: **page disagrees with the binary**.

```
$ printf 'Refund me please.' | thinkthen decide q --replay c8
thinkthen: the entry `2f62...d8e.json` was refused: the entry names a schema this version does not read, and this version reads `thinkthen.recording/1`
rc=5
```

`specification/recording.md` line 38: "An entry that parses and names another schema is refused with a sentence that names no schema." The intent is clearly that the untrusted string from the file is never echoed, and the binary honors that: it prints its own schema, never the file's. But the page as written is false of the binary. Smallest fix: line 38 becomes "refused with a sentence that never repeats the schema the file named."

### 4. Host case changes the recording digest; scheme case does not

Severity: **papercut**.

```
$ thinkthen decide q --url http://localhost:1234 --replay hc   ->  rc=0, hit
$ thinkthen decide q --url HTTP://localhost:1234 --replay hc   ->  rc=0, hit
$ thinkthen decide q --url http://LOCALHOST:1234 --replay hc
thinkthen: the replay folder holds no entry named `b31439d4...b6b6e9.json`
rc=5
```

`specification/backends.md` line 31: "A scheme is read without regard to case and written back in lower case, so one exchange keeps one recording digest whatever case the caller typed." The scheme is normalized, the host is not, so the stated goal is met for half the address. A host name is case-insensitive, and `http://LOCALHOST` is accepted as loopback (the loopback check is case-insensitive) yet records under a second digest. Smallest fix: lower-case the host alongside the scheme before the address is formed, and say so on line 31.

### 5. A question file that is empty, that holds a BOM, or that holds a trailing comma all report "the record is not valid JSON"

Severity: **papercut**.

```
$ thinkthen decide @empty.json --dry-run
thinkthen: the question file is not JSON this tool reads: the record is not valid JSON
rc=5
```

The same inner clause appears for `@bom.json` and `@trailcomma.json`. The exit code is right (`specification/question-file.md` line 17). The word "record" belongs to record mode and means nothing here; it is the inner JSON reader's message leaking into the outer one, and it tells the user nothing about which of the three faults they have. The same clause also surfaces for a single non-JSON document under `--field`:

```
$ printf 'E.' | thinkthen decide q --field /a --dry-run
thinkthen: the record is not valid JSON
rc=2
```

Smallest fix: give the question-file reader and the whole-document reader their own nouns ("the question file", "the input"), and have the JSON reader report the line and column as it already does for a recording entry.

### 6. `--threshold -0.1` falls out of clap with a message about passing `-0` as a value

Severity: **papercut**.

```
$ thinkthen decide q --threshold -0.1
error: unexpected argument '-0' found
  tip: to pass '-0' as a value, use '-- -0'
rc=2
```

Exit 2 is right. `specification/threshold.md` line 23 lists "a number that is not finite" and a reversed band as usage errors and expects a `thinkthen:` message naming `--threshold`. A negative cut is the obvious typo for `0.1`, and the tip sends the user toward `--`, which would then make `-0.1` the question. `--threshold 0`, `1.5`, `90`, `nan`, `inf`, `0.9:0.1`, `:0.9`, `0.1:`, and `0.1:0.9:0.5` all give proper `thinkthen:` messages. Smallest fix: let `--threshold` accept a leading hyphen (clap's `allow_hyphen_values`) so the tool's own refusal runs.

### 7. `score --quiet` and `decide --raw` fall out of clap with a "pass as a value" tip

Severity: **papercut**.

```
$ thinkthen score q a b --quiet
error: unexpected argument '--quiet' found
  tip: to pass '--quiet' as a value, use '-- --quiet'
rc=2
```

`specification/channels.md` line 42 and 44 say plainly that no command other than `decide` and `choose` takes `--quiet`, and that only `choose` takes `--raw`. The exit code is right and the reason is discoverable, but the tip is actively wrong advice: following it makes `--quiet` a level. Compare the tool's own good refusal for the same class of mistake: `thinkthen: --jobs bounds the requests in flight, and one document sends one request`. Smallest fix: accept the option on every verb and refuse it in the tool's own words when the verb does not take it.

### 8. `--quiet` in record mode is refused, and no page says it is

Severity: **page disagrees with the binary** (small).

```
$ printf '{"n":1}\n' | thinkthen decide q --jsonl --field /n --quiet
thinkthen: --quiet carries the answer in the exit code, and no record's answer sets it
rc=2
```

The message is excellent and the reasoning is right: `specification/channels.md` line 62 says no record's answer sets the exit code, so `--quiet` would discard the whole run. `channels.md` line 28 covers it generically ("an option that cannot act in the chosen mode"). But `channels.md` line 42 and `decide.md` line 28 describe `--quiet` without this limit, so a reader of either page is surprised. Smallest fix: one clause on `channels.md` line 42.

### 9. "1 records finished"

Severity: **papercut**.

```
thinkthen: stopped at record 2; 1 records finished, 0 from a recording
```

`specification/records.md` line 89 fixes the content of this line but not its grammar. Smallest fix: singular form at one.

### 10. A green how-to cannot be run from a copy of its own folder

Severity: **papercut**.

Demos 12, 13, 24, 25, 28, and 41 reach `../../transforms/...` inside their fenced blocks. A reader who copies one folder, as `demos/README.md` invites by presenting each page as a self-contained shell job, gets:

```
jq: Could not open ../../transforms/score/score.jq: No such file or directory
```

With `transforms/` present beside `demos/` every one of them passes. Smallest fix: one sentence in the page's Input section naming the transform folder as a second input, or copies of the `.jq` files inside the demo folders.

---

## What worked

- **Exit codes.** Every code in `channels.md` was reached and was right: 0 on yes, 1 on no, 2 on 27 distinct usage and input errors, 3 on an unresolved band and on a `choose` cut and on an exact tie, 4 on a missing key and on each of 401/402/403/404/422/429/500 and on four shapes of refused reply, 5 on a question file, a recording entry, a missing `--input`, and non-UTF-8 bytes. Code 70 was never provoked and is untested.
- **The threshold.** The whole worked-boundary table of `threshold.md` lines 29-34 reproduces exactly across p of 0, 0.1, 0.5, 0.9, and 1 under no threshold, `0.9`, and `0.1:0.9`. Every refusal on line 23 fires at exit 2 before any request. A band on `choose` and any threshold on `score` are refused with their own sentences.
- **The question digest.** All three worked examples of `question-file.md` lines 99-114 reproduce byte for byte. The same question typed and read from a file gives one digest; `--threshold`, `--true`, `--model`, and an option description each change it.
- **The question file.** Every refusal on the page fires at the right code with a message naming the file's key: two verbs, no verb, an unknown key, another verb's key, a non-object, an unreadable file, a directory, a bad threshold, a bad pointer, a blank question, a blank model, a non-string verb value. A file naming the wrong verb for the command is exit 2 and every other fault is exit 5, as line 17 requires. A path with spaces, a symlink, a CRLF file, a 200,000-character question, and a question beginning with `@` all work.
- **Precedence.** `--threshold`, `--true`, `--false`, `--model`, and `--field` each replace the file's value and each flip that one key in the `--dry-run` `from` object to `command line`. Positional options and `--option` each replace the file's whole `options` map rather than merging, and levels do the same.
- **Records.** `--lines`, `--jsonl`, `--input`, one `--field` and several, a missed pointer, `$.body`, `#/id`, two pointers sharing a last name, an empty stream, an empty document, a non-JSON record, a blank line, a final line with no line feed, CRLF, a duplicate member name, `NaN`, non-UTF-8 bytes, a record of 17 MiB with a record after it, and a record of 15 MiB all behave as `records.md` states. The oversized record stops the run at exit 2 after exactly one request, and the tail is never sent as a record of its own.
- **`--jobs`.** 0 and 33 are refused, 1 through 32 work, `--jobs` outside record mode is refused, and 40 records come back in input order at jobs 1, 8, and 32.
- **Resume and cache.** A first `--cache` run over 40 records made 40 requests and wrote 40 entries; the second run made **zero** requests, counted on the fake server, and printed byte-identical output. `--record DIR --replay DIR` on one folder did the same. Two different folders are refused. The folder is mode 0700 and each entry 0600.
- **A closed reader.** `... --jsonl --jobs 1 | head -1` over 40 records made 3 requests and stopped, exit 0, nothing on standard error.
- **`choose --options POINTER`.** A list record and a map record both work; a record with one option, with duplicates, with a missing pointer, or with a non-list are each exit 2 for that record before any request. `--options` without `--jsonl`, with `--lines`, beside positional options, and beside `--option` are each refused.
- **Retries.** A 429 retries twice with a backoff doubling from one second (measured 3.0 s). `Retry-After: 1` gives 2.0 s, `Retry-After-Ms: 200` gives 0.41 s, and with both headers the milliseconds one wins (0.21 s). A 402 is not retried. A 429 followed by a 200 succeeds on the retry. `--timeout 1` against a 4-second reply fails at 1.06 s.
- **Transport.** A 302 is not followed (exit 4, status 302, one hit on the server). A closed connection is exit 4. A body over 1 MiB is exit 4 naming the 1048576 limit. No reply body is ever echoed.
- **The address rule.** `http://` reaches `localhost`, `127.0.0.1`, `[::1]`, `LocalHost`, and `HTTP://LOCALHOST`, and refuses `localhost.`, `LOCALHOST.`, `127.1`, `0.0.0.0`, `[::ffff:127.0.0.1]`, `[0:0:0:0:0:0:0:1]`, `2130706433`, `0177.0.0.1`, `127.0.0.2`, `localhost.example.com`, and `example.com`, each with the clear-text sentence. Every `https://` base is untouched. Surrounding space is dropped.
- **Proxy variables.** With the target at `http://127.0.0.1:18090` and a second server at 18099, each of `HTTP_PROXY`, `http_proxy`, `ALL_PROXY`, `all_proxy`, `HTTPS_PROXY`, and `https_proxy` pointed at 18099 left the request going direct: 1 hit on 18090 and 0 on 18099 every time.
- **The terminal notice.** Through a real pseudo-terminal on standard input: `thinkthen: reading evidence from the terminal; end it with Ctrl-D on a line of its own` on standard error, with the plan unchanged on standard output. Through a pipe, standard error is 0 bytes. With `--input` and a terminal still attached, standard error is empty.
- **Secrecy.** Across 21 runs covering success, each refusal, a failed request, a malformed answer, four rejected replies, `--dry-run`, and `--record`, with the key `MARKERKEY123` and the evidence `MARKEREVIDENCE-SECRET`: the key appears 0 times on standard output, 0 times on standard error, and 0 times in any of the three recordings written. The evidence appears 0 times on standard error. It appears in the `--dry-run` plan and in the recordings, both by design and both documented. Servers answering 402, 422, and 500 quoted both markers back in the response body, and none of it reached the user. On the wire the key travels as `Authorization: Bearer MARKERKEY123`, as `backends.md` line 41 says.
- **Replay misses.** Different evidence, a different question, a different model, a different address, and a different verb each miss cleanly at exit 5 with the missing entry named. A recording made by `decide` is not answered by `choose`. A tampered request, a renamed adapter, a truncated entry, a non-JSON entry, and a foreign schema are each exit 5. A tampered response is replayed faithfully, which is what a recording is for.
- **Help against the binary.** `decide` offers `--true --false --threshold --quiet --details` and the record and backend options, no `--raw`, no `--option`. `choose` adds `--option --options --threshold --raw`. `score` offers no `--threshold`, no `--quiet`, no `--raw`. Every option the help lists is accepted and every one it omits is refused. `channels.md` line 32's advanced list matches the long help exactly. With no command the help goes to standard error at exit 2, so standard output stays empty.
- **Options and levels.** 2 to 255 options accepted, 256 refused; 2 to 10 levels accepted, 11 refused. Duplicates, blanks, white space, control characters, a non-text label, `--option` with no `=`, and `--option` beside a positional list are each exit 2. Unicode labels including an emoji work. An `--option` with an empty description is accepted and counts as no description, and it gives the digest that the map-with-`null` form gives.
- **`--raw`.** Prints the bare label; prints nothing at all for an unresolved single document; prints one empty line per record under `--lines` and under `--jsonl`, so one line still stands for one record. `--raw` beside `--quiet` is refused.
- **All 15 green how-tos.** Copied to scratch and every `bash` block run in order with the key unset: 59 blocks, 0 failures, once `transforms/` sits beside `demos/`. The text reads clearly; no sentence struck me as opaque to a newcomer.
- **All 9 probes.** `probes/replay-check.sh` reproduces every committed row from the recordings alone: 20, 239, 60, 40, 40, 60, 60, 60, 20, 20, 40, 40, 60, 60, and 40 rows across the fourteen files.
- **Every transform.** `counts`, `score`, `sweep`, `band`, `calibration`, `cost`, and `compare` over one row, over an empty file, over unresolved rows, over a row with no `usage`, and over a row with no probability. Empty input gives zeros and nulls rather than a division error. A row with no probability is refused by name (`row C-01 carries no probability`) by all five transforms that need one; `counts` and `cost` do not need one and pass. A row with no `usage` lands in `cost`'s `no_usage` list.

## What I could not test

- `filter`, `rank`, `annotate`, and `find`, and `specification/filter.md`, `rank.md`, `annotate.md`, and `find.md`. The binary offers three subcommands.
- Exit code 70. No input I tried reached a defect path.
- Proxy behavior under `https://`. It needs a TLS endpoint and a certificate, and testing it would mean either a live call or a trust-store change.
- The HTTP-date form of `Retry-After`, which `backends.md` line 45 says is ignored. My server sends the delta-seconds form; I did not separate "ignored" from "not read".
- `meta.usage` summing and `questions_sha256`, which belong to `annotate`.
- Red how-tos (03, 06, 07, 14, 15, 16, 18, 23, 30, 37, 39, 42, 43). They describe unbuilt verbs.
