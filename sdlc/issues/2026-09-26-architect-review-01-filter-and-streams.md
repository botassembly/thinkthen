Status: open. Filed 2026-09-26 by the marketing lead from a fresh architect review.

# Architect review 01: `filter` and stream processing

A fresh reviewer tested `filter` and record streams as an architect who builds ThinkThen into a pipeline. The review ran against main `9d652bed` and the release binary. It rated the topic fair and found 0 severity 1, 2 severity 2 and 8 severity 3 issues. The full detail sits in the architect review report 273, 01. Work file names in that report refer to its local work folder, which stays unpushed.

Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

## 1. `filter` keeps reading and paying after the downstream reader closes the pipe (severity 2)

Evidence. `specification/records.md` says "When the program downstream closes the pipe, the tool stops reading and stops scheduling." `cli/edge.rs:278-284` detects a closed pipe only on a failed write, and `filter` writes only kept records. On loopback, with a dummy key, the input was `keep first` followed by 300 `drop N` lines:

```text
$ thinkthen filter 'Q.' --no-cache --url http://127.0.0.1:18091 < pipe300.txt | head -1
keep first
pipestatus 0 0
requests the server received: 301
$ thinkthen decide 'Q.' --lines --no-cache --url http://127.0.0.1:18091 < pipe300.txt | head -1
requests the server received: 5
```

In offline replay, a 3,002-record input ran to a replay miss at record 3,002, long after `head` had gone. The only test of the guarantee runs `decide` (`tests/backend/parallel.rs:81-92, 357-386`).

What an integrator hits. `thinkthen filter … | head -5` over a large, selective input keeps sending billed requests until the next match. That can be the whole file.

Direction. Check whether standard output is still open without writing to it, for example with a poll for an error or hang-up on the output descriptor between dispatches. Add the `head` test for `filter` with a selective question.

## 2. One blank line stops a `--lines` run at exit 2, and the spec does not say so (severity 2)

Reviews 01 and 11 both found this. This file carries it. The architect review 11 file points here.

Evidence from review 01, offline:

```text
$ printf 'keep a\nkeep b\n\nkeep c\n' | thinkthen filter 'Q.' --replay recblank --url …
keep a
keep b
thinkthen: the evidence is empty or blank
thinkthen: stopped at record 3; 2 records finished, 2 records from a recording
exit 2
```

A file ending in `\n\n` stops the same way after its last real line. `specification/records.md` says "`--lines`: Each line is one text record", and it says "No blank lines" only for `--jsonl`. The blank-string refusal appears only under `--field`. Neither `filter.md` nor `records.md` says a blank line under `--lines` fails the run. The message comes from `core/text.rs:19`.

Evidence from review 11: `printf 'a\nb\n\n' | thinkthen decide Q --lines` printed two rows, then "the evidence is empty or blank / stopped at record 3", exit 2. `printf 'a\n   \nb\n' | thinkthen filter Q` stopped at record 2 and never judged `b`. A CRLF blank line does the same. `records.md:101` says an empty stream succeeds. Blank-line rules appear only for `--jsonl` (`records.md:15`) and `relate` (`records.md:103`). `audit` skips blank lines (`audit.md:41`). The spec README says "A behavior that is absent here is absent from the tool."

What an integrator hits. "grep by meaning" over ordinary text such as logs, notes or paragraphs fails at the first paragraph break. A nightly `grep ... | thinkthen filter` over a file with a trailing blank line exits 2 after it has paid for every record. A blank line in the middle silently drops every record after it. The output is a silent prefix, the reason appears only on standard error, and exit 2 reads like a flag typo.

Direction. Pick one rule and state it on `records.md`, `filter.md` and each verb page. Either skip blank lines under `--lines` while still counting them, as `audit` does and as a grep user expects, or keep the refusal, document it with the `grep -v '^[[:space:]]*$'` workaround, and make the message say "blank line". The site's home page calls `filter` "a grep that understands", so the skip rule fits that claim.

## 3. Any single bad record ends the stream, with no skip mode (severity 3, carried here for three reviews)

Reviews 01 (issue 6), 05 (issue 3.6) and 02 (issue 19) all found this. Offline, a replay miss (exit 5), a blank line (exit 2), invalid UTF-8 (exit 5) and an over-limit line (exit 2) each stop the run. In live run L3, one over-long record ended the run at exit 4. The stop line is on standard error, which `channels.md` says a script never parses. `--facts` is "Not built yet, by ADR 0048 item 10". A pipeline over dirty data must pre-clean it perfectly or fail. After a stop, a `filter` consumer cannot tell from standard output how far the run got, because dropped records leave no trace.

The decision on a skip mode is already owed in `2026-09-25-docs-how-tos-and-spec-claims-owed.md` item 10, and `roadmap.md` holds `--on-error continue`. This entry adds the evidence that three separate reviews asked for it. Direction: ship `--facts` with the stop position, and consider an option that writes refused input records to a named file and keeps going.

## Severity 3 titles

- One request per record; the batching planner is dead code. See `2026-09-26-batching-design-review-before-0146.md`.
- Ordered output means head-of-line blocking. Carried in the architect review 07 file.
- Retries are per worker and move in lockstep; three seconds of 429 ends a run. Carried in the architect review 07 file.
- Any single bad record ends the stream. Carried above as item 3.
- No size guard for a filter record by default; memory per in-flight record is a large multiple of its size. The request ceiling part is carried in the architect review 06 file. Offline, eight 12 MiB records at `--jobs 8` used 541 MB.
- `filter` cannot serve as a request-and-reply coprocess, and the README's `coproc` advice does not say so. A service wrapper that writes a line and waits for its reply hangs on the first dropped record.
- After Ctrl-C, the stop line names a record that never arrived.
- One Ctrl-C can wait up to `--timeout` for requests on the wire, and the second Ctrl-C escape is undocumented. Carried in the architect review 11 file.
