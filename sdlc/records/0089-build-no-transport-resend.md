# 0089: Never send a request again after a transport failure

Status: landed. A fresh review accepted `18221ad7` (`sdlc/records/0089-code-review.md`). Main fast-forwarded to `29578528`.

## Result

`engine/http.rs::is_retried` now returns true only for a retried status (429, 500, 502, 503, 504, 529). Every `Error::Transport` returns false, so `post_observed` returns after the one attempt. A reset, an early close, a cut-short body, a timeout, and a reply past the 1 MiB bound each fail at exit 4 after one POST. The close-before-reply message reads `thinkthen: the backend closed the connection before a reply and may have received the request; it was not sent again`. Both `--max-retries` doc comments read `How many times a retried status is sent again. A transport failure is never sent again.` `specification/backends.md` (the option sentence, the retry sentence, and the transport guidance) and the `requests_sent` row of `specification/result.md` state the new rule.

This settles items 8 and 22 of `sdlc/issues/2026-09-22-small-leftovers-from-early-reviews.md`. A cut-short or oversized body is no longer retried, and a transport failure that cannot succeed is sent once. It closes `sdlc/issues/2026-09-23-the-command-sends-a-delivered-request-again-after-a-transport-failure.md`.

## Red then green

Every test below ran red against the old rule before the fix, then green after it.

| Acceptance | Test | Red (observed) |
|---|---|---|
| 1 | `engine::http::tests::no_transport_failure_is_sent_again` | `Transport(Timeout)`: left true, right false |
| 2 close | `resend::a_close_after_the_whole_request_is_sent_once` | listener saw 3 POSTs, expected 1 |
| 2 reset | `resend::a_reset_after_the_whole_request_is_sent_once` | 3, expected 1 |
| 2 stall | `resend::a_stall_past_the_timeout_is_sent_once` (`--timeout 1`, `after(2500)`, under 2 s) | 3, expected 1 |
| 2 cut short | `resend::a_body_cut_short_is_sent_once` | 3, expected 1 |
| 2 bound | `resend::a_reply_past_the_bound_is_sent_once` | 3, expected 1 |
| 2 status | `resend::a_retried_status_is_still_sent_again` | green before and after, as a guard: 2 POSTs, `"requests_sent":2,` |
| 3 | `exchange::a_close_before_headers_is_not_sent_again` | 2, expected 1 |
| 4 | `resend::a_reset_adds_one_request_and_no_cache_answer_to_status` | `status --json` said `requests_sent` 3, expected 1 |
| 5 | `resend::a_record_run_sends_each_request_at_most_once_and_caches_no_failure` | record 2's body reached the listener 3 times, expected 1 |
| 6 | `decide_edge::the_long_help_says_a_transport_failure_is_never_sent_again` | the new line was absent from `decide --help` |
| 7 | every `resend` case through `failed_once` | no red; it guards |

The three message pins (`exchange.rs` twice, `cli/failure/tests.rs` once) failed on the old sentence and pass on the new one.

The record-mode run over `--jobs 4 --cache DIR` exits 4. Its standard error starts with the close-before-reply sentence and holds `thinkthen: stopped at record 2; `. Each request body reached the listener once. The cache folder holds an entry, named by `digest(url, body)`, for every answered body and none for record 2. The answered entries prove the naming the absence check relies on.

## Harness

`Canned::reset()` leaves the complete request unread and drops the stream. Linux answers that close with a reset. To leave the bytes unread, the listener now peeks until one whole request sits in the socket buffer, parses it from the peeked bytes, and reads past it only when it answers. Both the scripted and the keep-alive loops use this one reader, and `read_kept` is gone. A request longer than 32 KiB cannot always sit whole in the buffer. The listener reads it at once. A reset of such a request would be a plain close. No test resets a long request. The first draft had no such fallback, and two tests with long requests hung until it was added. `redirect`, `cut_short`, and `close_without_reply` now use struct-update syntax over `status`.

## Deviations from the ticket

1. Clap drops the final period of a one-paragraph doc comment. The help line therefore reads `... A transport failure is never sent again` with no period, and the help test pins that exact line on `decide` and `find`.
2. `a_response_body_past_the_bound_is_exit_four_and_never_fills_memory` pins no sentence. The bound case pins the sentence the command prints: `thinkthen: the backend could not be reached; check --url and the network`.
3. The binary prints no `Debug` output of its own. The secrecy check reads standard output, standard error, and the `Debug` form of the process `Output` on every `resend` path, including reset and stall. The unit `Debug` sweep in `cli/failure/tests.rs` is unchanged.
4. The code commit message says `resend.rs` holds 177 lines. It holds 174 nonblank lines.

## Budget

Production: 4 files (`engine/http.rs`, `cli/failure.rs`, `cli/args.rs`, `cli/args/find.rs`), 11 nonblank lines added and 13 removed outside test modules. Tests: 6 test-only files plus the unit test module in `http.rs`, 254 nonblank lines added. The ratchet rose from 43563 to 43753, then to 43754 when the record-run test pinned the whole stopped-run sentence. No dependency, no `unsafe`, no `libc`, no `rustfmt::skip`.

## Found on the way

`annotate::scheduling::one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` fails under load on main too. It is filed as `sdlc/issues/2026-09-24-the-global-queue-concurrency-check-fails-under-load.md`.

## Gates

The builder ran `install`, `lint`, `test`, and `spec` one at a time from `d2a76dd9` on this Linux machine, with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, then `git diff --check ab72203c HEAD`. Every command exited 0. `lint` reported `ratchet: crates 43753/43753` and `pages: 1 coming, 21 green`. `test` passed 724 Rust tests with 2 ignored and 0 failed, plus its script self-tests. `spec` reported `demos: 21 green, 0 red`. No live call ran. Only this paragraph changed after that run.

## Landing

The landing commits after the review pin the whole stopped-run sentence in the record-run test and raise the ratchet to 43754. They also align the site reference and catalog with the `--max-retries` help, correct this record's line counts, and fix the ticket's gate-host wording. The review's harness follow-up is `sdlc/issues/2026-09-24-the-loopback-reset-reply-has-two-silent-edges.md`.

The builder ran `install`, `lint`, `test`, and `spec` one at a time at `29578528` on this Linux machine, with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, then `git diff --check ab72203c 29578528`. `origin/main` was still `ab72203c`, so the merge added nothing. Every command exited 0. `lint` reported `ratchet: crates 43754/43754` and `pages: 1 coming, 21 green`. `test` passed 724 Rust tests with 2 ignored and 0 failed. The flaky annotate queue test passed on the first run. `spec` reported `demos: 21 green, 0 red`. No live call ran. Main fast-forwarded to `29578528`. This landing note is a doc-only commit after that run, so the gate result still applies to the code.
