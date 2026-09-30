Disposition, 2026-09-27: ticket 0169 settles register 46, 56 and 116 after fresh review of `84de0a7d`. The stop reports the signal without a guessed record, SIGTERM follows SIGINT, and the second-signal escape is documented and proved. Already-sent requests still finish within their attempt timeout by the retained contract. A first-signal waiting notice and cancellable socket reads remain deferred; this umbrella issue stays open for its other findings.

Status: closed 2026-09-30. Item 1 fixed by ticket 0170 (`stopped.retryable` in run facts), item 2 by ticket 0169, and item 4 by qf-command-edges-and-prune. ADR 0111 replaces item 3: the SQLite store has no temporary entry files.

# Architect review 11: failure handling and scripting

A fresh reviewer tested exit codes, signals, partial output and reruns as an architect who runs ThinkThen from a queue worker or a scheduled job. The review ran against main `9d652bed` and the release binary. It rated the topic fair and found 0 severity 1, 2 severity 2 and 8 severity 3 issues. The full detail sits in the architect review report 273, 11.

Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

The failure design held up: a failure never becomes an answer, and standard output stays a clean prefix. The problems below make that design hard to operate at scale.

## 1. Exit codes mix transient and permanent failures, and only stderr tells them apart, which the spec forbids a script to parse (severity 2)

Evidence. Exit 4 covers 429, 503 and timeout, where a retry can pass. It also covers 401, a live 400 for an unknown model, a live `max_tokens_exceeded`, a missing key and a malformed reply, where a retry cannot pass. Exit 5 covers a full disk, and also invalid UTF-8 and a malformed question file. Exit 2 covers a flag typo and one bad record (`demos/12-keep-going/README.md:53` admits it). `channels.md:14` says stderr is "Never parsed by a script", but `diff.md:69` and demo 12 tell scripts to read it. The library has `Error::retryable()` (`public/error.rs:97`), and the command has no equivalent. `--facts` is not built.

What an integrator hits. A queue worker must choose between retrying every exit 4, which burns quota on a revoked key or a 400, and retrying none, which loses work to a 429. Matching English sentences on stderr is the only other way, and the spec reserves the right to change them.

Direction. Expose the retry signal the engine already computes. That could be a distinct exit code for a retryable backend failure, or a structured final stderr line, such as the planned `thinkthen.run/1`, with a stable cause identifier, a `retryable` field and the stop position. `2026-09-26-every-surface-should-give-back-run-facts.md` plans the run line. This item asks that the line carry the retry signal and a stable cause.

## 2. Ctrl-C waits out requests in flight and then blames the backend (severity 3, carried here for two reviews)

Reviews 01 (issue 10) and 11 (issue 8) both found this. With `--timeout 6` against a hanging server, SIGINT at 1 second exited at 6.4 seconds, and stderr read "the backend timed out; increase --timeout or try again" and then the stop line. With replies held for 25 s, one SIGINT led to exit after 24.0 s, and a second SIGINT ended the run 0.6 s after the first. `channels.md:5` documents the first behavior, and no page mentions the second. An operator reads a Ctrl-C'd run as a backend outage, and the advice runs the wrong way. Direction: say "interrupted" when cancellation caused the stop, document the second Ctrl-C, and print "finishing N requests in flight; press Ctrl-C again to stop now" on the first.

## 3. Killed writes leave temporary cache entries that nothing removes (severity 3, carried here for two reviews)

Reviews 08 (issue 12) and 11 (issue 4) both found this. Twelve SIGTERM kills of a `--cache C5` run left 82 files named `.PID.N.DIGEST.json`: 42 empty, 18 full orphans and 22 hard-linked to live entries. A full run afterwards and `thinkthen cache prune C5` left all 82, and prune reported "3000 entries". The temporary name is built in `engine/recorder.rs:366`, and that path removes a stale file only when the same PID and counter come round again. `recording.md:94` says dot-prefixed temporary files are ignored. Entries hold the evidence, so someone who prunes to remove sensitive text leaves copies behind. Direction: have prune, or the next writer, sweep dot-prefixed partials older than a bound whose PID is not running, and count them in `status`.

## 4. A huge `--timeout` panics with exit 101, outside the exit-code table (severity 3, carried here for two reviews)

Reviews 06 (I-11) and 11 (issue 7) both found this. `--timeout 9223372036854775807` on `decide` and on `check` makes the HTTP client panic with "overflow when adding duration to instant" (`ureq::timings::CallTimings::next_timeout`, through `engine/http.rs:81`), and the process exits 101. `--timeout 1000000000000000000` works. `channels.md:61` says "One function maps every error to its exit code", and a defect is 70. A templated config that passes "max u64" to mean "no timeout" produces a code no `case` handles. Direction: cap `--timeout` at parse time and exit 2.

## Carried in other files

- A blank line in `--lines` input stops the run, and the spec never says so (issue 1, severity 2). The architect review 01 file carries it with this review's evidence.
- A stored partial reply replays forever (issue 5). The architect review 08 file carries it.
- A deterministic backend refusal on one record blocks every rerun (issue 6). The architect review 06 file carries the request ceiling, and the architect review 01 file carries the skip mode.
- The default cache binds to one backend address (issue 9). The architect review 06 file carries it.

## Severity 3 titles

- SIGTERM is unspecified and ends a run abruptly. SIGTERM one second into a run with four requests in flight gave return code -15, no stderr and no stop line. `timeout(1)`, Kubernetes and systemd all send SIGTERM. Treat it like the first SIGINT, or add a `--deadline`.
- Killed runs leave temporary cache entries. Carried above as item 3.
- A stored partial reply replays forever. Carried in the architect review 08 file.
- A deterministic backend refusal on one record blocks every rerun. Carried in the architect review 06 file.
- An absurd `--timeout` panics with exit 101. Carried above as item 4.
- Ctrl-C waits out a hung request and then blames the backend. Carried above as item 2.
- The default cache binds to one backend address. Carried in the architect review 06 file.
- No catalog of error sentences with stable identifiers, so alerting and runbooks key on English text.
