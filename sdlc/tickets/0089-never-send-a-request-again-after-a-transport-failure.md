---
flow: build
priority: 89
opens: crates/thinkthen/src/engine/http.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/cli/args.rs crates/thinkthen/src/cli/args/find.rs crates/thinkthen/tests specification/backends.md specification/result.md sdlc/ratchet.json sdlc/issues sdlc/records sdlc/planning
---

# 0089: Never send a request again after a transport failure

Status: built at 3686414f on its branch, rebased over 0088, and awaiting a fresh review (`sdlc/records/0089-build-no-transport-resend.md`). Owner: Claude.

## Outcome

One command call sends a paid request at most once when the transport fails. A reset, an early close, a cut-short body, and a read timeout each fail after the first attempt at exit 4. A retried status (429, 500, 502, 503, 504, 529) keeps its retries. This closes `sdlc/issues/2026-09-23-the-command-sends-a-delivered-request-again-after-a-transport-failure.md` and puts main on the rule the surfaces branch adopted in `dd8a383` (error-index row R5-4). Queue item 2 of `sdlc/planning/one-line-plan-2026-09-24.md`.

## Facts on main at 5f4fd32c

- `engine/http.rs::is_retried` returns false for `Transport(Refused)` and true for every other `Transport` kind. `post_observed` loops on it up to `--max-retries`, default 2.
- The issue measured three full 120-byte POSTs from one `decide` on a loopback stub that read the whole body and then reset, and three more when the stub stalled past `--timeout 2` (wall 9.22 s). Every copy reached the backend.
- `TransportKind` has five values: `Timeout`, `NameLookup`, `Refused`, `PrematureClose`, `Other`. `Other` holds `ureq::Error::ConnectionFailed`, TLS failures, an interrupted call, and a response body past the 1 MiB bound.
- The client sets only `timeout_global`. ureq 3.4.2 then reports every timeout as `Timeout::Global`, whatever the phase (`timings.rs`). The engine cannot tell a connect timeout from a read timeout.
- Both send paths reach `post_observed`: `engine/request.rs:69` and `cli/asking/request.rs:63`. `decide`, `choose`, `score`, `tag`, `find`, `annotate`, `recognize`, and `relate` all share it.
- `specification/recording.md:24` counts every HTTP attempt before it is sent. `thinkthen status` therefore shows three requests for the measured failure today.
- `tests/backend/exchange.rs::a_close_before_headers_follows_the_transport_retry_rule` pins the old rule (two sends, `requests_sent:2`). Records 0064 and 0072 kept it on purpose.
- `exchange.rs` holds 496 nonblank lines. The harness already serves `close_without_reply` (full read, then close), `cut_short`, and `after(ms)`.

## The retry rule

No transport failure is sent again. Status retries stay as they are.

Reasons:

1. The engine cannot prove from the error it holds that no byte left the process. `Timeout` covers every phase, and `Other` mixes connect failures with failures after the reply began.
2. The two kinds that are surely pre-delivery gain little. `Refused` already fails at once (0072). A name lookup that fails rarely heals inside the one- or two-second wait.
3. A retried status means the backend answered. A transport failure after the body left means the backend may have billed the call.
4. The surfaces bind this engine after 0086. One rule keeps the command and every surface billing the same.

Cost: a DNS blip, or a pooled connection that died between ureq's `is_open` probe and the write, now fails the call at exit 4 instead of healing. One process shares one pool (`Client::new`). A dead pooled connection can therefore stop a whole `--jobs` or record run over one pooled connection at its failed record. The user's own second run is the retry, and it is visible in `thinkthen status`. Ian can overturn this rule. The lever is follow-up 1.

## What the user sees

- Exit code 4, unchanged. Standard output stays empty. A record run still stops at the failed record with its existing stopped-run line.
- The close-before-reply message becomes exactly `thinkthen: the backend closed the connection before a reply and may have received the request; it was not sent again`. It no longer names `--max-retries`. That option no longer governs a transport failure.
- The timeout, name lookup, refused, and other-transport messages keep their exact text. The status 500 message keeps `--max-retries`, which still governs it.
- `--max-retries` long help becomes exactly `How many times a retried status is sent again. A transport failure is never sent again.`
- A transport failure ends after one attempt. It never waits a retry wait.

## Interactions

- `--jobs`: each worker calls `post_observed` for its own request. The rule holds per request, and no worker sends another worker's failed body. The scheduler, the stop rule, and in-flight handling do not change.
- Cache: a transport failure installs no entry. The 0039 lock still lets the next waiter for the same digest send after the owner fails. That second send answers a second record the user asked for, and a run with no cache sends it too. The lock rule stays.
- Record and replay: a transport failure writes no recording entry. Replay never reaches the network and does not change.
- Accounting (0063, 0064): `before_attempt` still counts each attempt before it is sent. The measured failure now adds one request to `thinkthen status`. `requests_sent` on a success counts status retries only.
- Cancellation (0073, 0074): unchanged. A transport failure returns before the retry wait. No cancellation check is added.

## Scope

- `engine/http.rs`: `is_retried` returns false for every `Error::Transport`. Update the module and `Exchange::max_retries` comments.
- `cli/failure.rs`: the one close-before-reply message.
- `cli/args.rs` and `cli/args/find.rs`: both `--max-retries` doc comments. `find.rs` says "How many transport or retried-status attempts follow the first" today.
- `specification/backends.md`: the retry sentence at line 61, the `--max-retries` sentence at line 59, and the close-before-reply guidance at line 76. The line-76 clause "A refused connection fails after its first attempt" folds into the one new rule sentence: every transport failure, a refused connection included, fails after its first attempt. `specification/result.md`: the `requests_sent` row says retries of a retried status.
- Tests as listed under acceptance, a new record `sdlc/records/0089-...md`, the issue status set to Closed with the landing SHA, and the exact ratchet.
- The record notes that item 8 of `2026-09-22-small-leftovers-from-early-reviews.md` (a cut-short or oversized body is retried) and item 22 (a transport failure that cannot succeed is retried three times) are settled here.

## Out of scope

Status retry rules and waits, `TransportKind` values, the connect-phase split, per-phase ureq timeouts, the scheduler, cache locks, recordings, cancellation and deadlines (0076), counters format, surfaces, the stand-in, dependencies, live or paid calls.

## Allowed paths

`crates/thinkthen/src/engine/http.rs`, `crates/thinkthen/src/cli/failure.rs`, `crates/thinkthen/src/cli/failure/tests.rs`, `crates/thinkthen/src/cli/args.rs`, `crates/thinkthen/src/cli/args/find.rs`, `crates/thinkthen/src/cli/interrupt/tests.rs` if a message pin lives there, `crates/thinkthen/tests/backend/{exchange,main,resend,state}.rs`, `crates/thinkthen/tests/status.rs`, `crates/thinkthen/tests/decide_edge.rs` and `crates/thinkthen/tests/find_edge.rs` for the help pin, `crates/thinkthen/tests/backend/harness/mod.rs`, `specification/backends.md`, `specification/result.md`, `sdlc/ratchet.json`, `sdlc/records/0089-*.md`, this ticket, the issue file, and `sdlc/planning/one-line-plan-2026-09-24.md` for its status mark.

## Acceptance with proof

Every "sends once" claim counts POSTs on the loopback listener. `--dry-run` and `--max-retries 0` prove nothing here.

1. Red then green, unit: `only_a_refused_transport_failure_loses_its_retry` becomes `no_transport_failure_is_sent_again`. All five `TransportKind` values return false. The six statuses return true and 401 returns false.
2. Red then green, compiled binary, new `tests/backend/resend.rs`, default `--max-retries`:
   - close after the full body (`close_without_reply`): listener counts 1 POST, exit 4, stdout empty, stderr is the exact new sentence.
   - reset after the full body: a new harness reply peeks until the whole request is in the socket buffer, records it, and drops the stream with those bytes unread. Linux and macOS send a reset for that close, and std alone does it. Listener counts 1 POST, exit 4, the same exact sentence.
   - stall past `--timeout 1` (`after(2500)`): listener counts 1 POST, exit 4, the exact timeout sentence, elapsed under 2 s. The harness sets `THINKTHEN_TEST_RETRY_WAIT_MS=1`. The old code takes about 3 s. The POST count carries the proof.
   - cut-short body after a 200 header: listener counts 1 POST, exit 4, stderr is the exact new close-before-reply sentence.
   - a 200 reply whose body passes the 1 MiB bound: listener counts 1 POST, exit 4, stderr is the exact sentence `a_response_body_past_the_bound_is_exit_four_and_never_fills_memory` already expects.
   - A 503 then 200 still sends 2 and prints `requests_sent:2`. This guards the status path.
3. Red then green: `a_close_before_headers_follows_the_transport_retry_rule` becomes `a_close_before_headers_is_not_sent_again`. It counts 1 POST and expects exit 4. The two existing message pins in `exchange.rs` and the one in `cli/failure/tests.rs` take the new sentence.
4. Red then green, accounting: with an isolated `XDG_CACHE_HOME`, one reset run adds exactly 1 to `requests_sent` in `thinkthen status` and adds no cache answer.
5. Red then green, record mode: `decide --jsonl --jobs 4 --cache DIR` over three distinct records, where the second record's request resets. Each distinct request body reaches the listener at most once. The run exits 4 with the existing stopped-run line, and the cache folder holds no entry for the second body.
6. Red then green, help: one test pins the exact `--max-retries` long help line on `decide` and on `find`.
7. A secrecy check reads stdout, stderr, and every `Debug` line on the reset and stall paths and finds neither the key nor the evidence.
8. `specification/backends.md` and `result.md` state the new rule. `sdlc/scripts/spec` passes with no page example claiming a transport retry.
9. The coordinator runs `sdlc/scripts/install`, `lint`, `test`, and `spec` in order from the exact candidate SHA on this machine (ThinkThen gates run here; `yellow.local` serves only BioMCP and BioData), with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset. Then `git diff --check`. Every command exits 0. No live call runs.

## Budget

At most 4 production Rust files and 20 changed nonblank production lines. At most 8 test-only Rust files and 300 nonblank test lines. New tests go in `resend.rs`, because `exchange.rs` sits at 496 nonblank lines. No new dependency, no `unsafe`, no `libc`. The ratchet rises by the new test file less any lines removed. A second agent reviews that raise and names what it checked.

## Dependencies and route

Implementation waits for 0088 to land. 0088 conflicts in `tests/backend/main.rs`, `sdlc/ratchet.json`, `specification/result.md`, `cli/failure.rs`, and `cli/failure/tests.rs`. Rebase over 0088 if it lands first, and re-measure the ratchet ceiling. `relate` reaches `post_observed` and inherits the rule. This ticket lands before 0082 in the queue.

Contract 1; state and timing 1; reach 1; proof 1; cost of error 2; total 6; final level 2. The cost of error is money on every surface. An independent reviewer accepted the design on 2026-09-24 (`sdlc/records/0089-design-review.md`). Build by the Opus tier. Stop and re-score if the change needs a new `TransportKind`, per-phase timeouts, a scheduler change, or more than this budget.

## Follow-ups (not this ticket)

1. Retry only failures that surely sent nothing: name lookup, `ConnectionFailed`, and a connect-phase timeout through ureq's `timeout_connect`. Open it when a user reports a failed run that one retry would have healed.
2. Whether a 500, 502, or 504 after a full delivery also bills. This needs vendor evidence through `sdlc/scripts/live` with Ian's authorization, and it may move 500 and 504 out of the retried set.
3. The stopped-run line could say the failed record may have been billed.
