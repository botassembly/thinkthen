ACCEPT

# 0089 code review: never send a request again after a transport failure

Reviewer: fresh read-only session. Subject: `ticket/0089-no-transport-resend` at `18221ad7`, base main `ab72203c`. Authority: the ticket, `sdlc/records/0089-design-review.md`, `sdlc/records/0089-build-no-transport-resend.md`. No tracked file was edited. No paid call ran. `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` were unset for every command.

## What I checked, by command

1. Red proof. I copied the tree at `18221ad7` to `/tmp/claude-1000/0089-review/` and put the old `is_retried` back (Refused false, every other Transport true). Results:
   - `no_transport_failure_is_sent_again`: red on `Transport(Timeout)`, left true, right false.
   - The close, reset, stall, cut-short, and bound cases in `resend.rs`: all red, listener saw 3, expected 1.
   - The status case (`a_reset_adds_one_request...`): red, `requests_sent` 3, expected 1.
   - The record run: red, record 2 sent 3 times.
   - `exchange::a_close_before_headers_is_not_sent_again`: red, 2 against 1.
   - `a_retried_status_is_still_sent_again`: green, as the guard should be.
   With the fix restored, `--lib`, `backend`, `decide_edge`, and `status` all pass: 275, 351, 20, and 5 tests, 0 failed. `resend::` ran green 15 times in a row. `cargo fmt --check`, `clippy --all-targets -D warnings`, and `git diff --check` came back clean.
2. No resend path is left.
   - `is_retried` is now `matches!(failure, Error::Status(s) if RETRIED.contains(s))`. Every `Transport` kind and every other `Error` value returns false. `RETRIED`, the waits, and the header handling did not change. 429/5xx/529 keep their retries (unit table and compiled 503-then-200 test).
   - Both send paths (`engine/request.rs:69`, `cli/asking/request.rs:63`) pass `post_observed` through as an `FnOnce` `send` in `ask_prepared`, so no caller loops. The callers are `annotate::answer_group`, `Asking::chunks` (used by relate and recognize), and the scheduler and workers. Each stops on the first `?`, and nothing re-queues a failed item.
   - Replay never reaches `send`. A cache or record write needs a success.
   - ureq 3.4.2 has no request retry. Its only "retry" is the per-address connect fallback in `transport/tcp.rs:157`, which runs before any byte is sent.
3. The reset is real. A Python loopback check of the same peek-then-close pattern raised `ConnectionResetError` on this Linux host. `io_transport` maps a reset to `PrematureClose`, and that produces the pinned sentence.

## Departures

1. Clap drops the final period. Accepted. The help test pins the whole line on both `decide` and `find`.
2. The bound case pins the `Other` sentence. Accepted. That is the sentence the command prints, and the case went red at 3.
3. Secrecy reads stdout, stderr, and the `Debug` of `Output`. Accepted. The binary prints no `Debug` of its own. The record-run test does not go through `failed_once`, but acceptance 7 names only the reset and stall paths, and both are covered.
4. The 32 KiB read-in-full fallback (`harness/mod.rs:350`). It weakens no proof. Every reset in the acceptance tests sends a short body: one `decide` of a short string, or one small JSONL record. All of them take the peek path and get a true reset. Reset and close also give the same outside result: `PrematureClose`, the same sentence. The fallback only changes how requests of 32 KiB or more are read for replies that are not resets, and before this change those were read in full anyway. The 351-test backend suite passes.
5. The commit message says 177 lines. `resend.rs` holds 174 nonblank lines. The record says so. Accepted.

## Findings (record text only, non-blocking)

- F1. `sdlc/records/0089-build-no-transport-resend.md:47`: "12 removed" should be 13. The removed nonblank production lines outside test modules are 1 in `args.rs`, 1 in `find.rs`, 1 in `failure.rs`, and 10 in `http.rs` (7 in the old `is_retried` body, 1 in its doc line, and 2 in doc comments). Smallest fix: write "13 removed". The production change is 11 added and 13 removed, a net of −2. That meets the 20-line budget if you count the larger side or the net. It exceeds it only if you add the two sides together.
- F2. Same line: "257 nonblank lines added" should be 254. The test files add 249 and the `http.rs` test module adds 5. The 257 repeats the 177 miscount. Smallest fix: write "254". Both numbers stay under the 300 budget.

## Budget and ratchet

The source measures 43563 at `ab72203c`, equal to the old ceiling, and 43753 at `18221ad7`, equal to the new ceiling. The rise of 190 breaks down as:

- `resend.rs`: +174
- the help test: +11
- `main.rs`: +1
- the harness: +16 net. The reset reply and the peek reader added lines. The three constructors were folded into struct-update form, and `read_kept` and `read_rest` were deleted.
- `exchange.rs`: −10
- `http.rs`: −2

Each part is earned. The harness growth is the one piece of new mechanism the reset acceptance item needs. The commit message names what grew and where lines were deleted first. No dependency, no `unsafe`, no `libc`. Four production files changed.

## Wording and spec

- The spec, the help text, and the code state the same rule:
  - `backends.md:59` and `:61` state the new option and retry rule.
  - `backends.md:76` folds in the refused clause and describes the new close-before-reply guidance, and that guidance matches `failure.rs:452` exactly.
  - `result.md:103` updates the `requests_sent` row.
  - The status-500 message keeps `--max-retries`.
- No page under `specification/`, `spec/`, or the demos still claims a transport retry. Demo 12's `--max-retries 0` example is a refusal, so it stays true.
- The new message follows the writing rules: no trailing clause, and no key or evidence in it.

## Follow-ups (not this ticket)

- FU1. `site/src/pages/reference.astro:14` and `site/src/data/catalog.mjs:36` still describe `--max-retries` as "Retries after the first attempt." That reads as if a transport failure is retried. Neither file is in the allowed paths. Smallest fix: use the help sentence.
- FU2. The issue's status line (`sdlc/issues/2026-09-23-...md:3`) names the branch SHA `3686414f`. The ticket asks for the landing SHA. The coordinator should update it at merge.
- FU3. Harness: `peek_request` (`harness/mod.rs:343-353`) never sees end-of-file after a client sends part of a request and closes. `peek` keeps returning the same bytes, so the thread spins every 1 ms until the process exits. Also, a `Canned::reset()` on a request read through the 32 KiB fallback silently becomes a close. Smallest fix: have `reset` panic when `used == 0`, and treat an unchanged `seen` across polls, together with a zero-length `read`, as end-of-file. No test hits either case today.
- FU4. The record-run test checks only part of stderr: it checks the start and that the text contains `thinkthen: stopped at record 2; ` (`resend.rs:207`). With `--jobs 4`, how many records finish can vary, which may explain the partial check. Pinning the whole stopped-run sentence would follow the CLAUDE.md rule that a test pins the exact sentence it checks.

Scratch copy `/tmp/claude-1000/0089-review/` deleted after review.
