# 0341: A backend's keep-alive close no longer fails a send

Status: in progress. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-1. Asks whether the connection reuse that 0340 found in a DuckDB test can fail a real send.

## Outcome

The pool reuses no connection that sat idle for one second or more. A backend that closes idle connections after a longer wait no longer fails the next send with exit 4. No request is sent twice, and every count and refusal stays as it was.

## Evidence

- Starts from: ticket 0340 and `sdlc/issues/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md`; the exit 4 the Beatles Bench saw once, in `sdlc/issues/2026-09-30-live-batching-flake-and-unexplained-usage-calls.md`; ticket 0089, which rules that no transport failure is sent again and names this exact cost; ticket 0142, which kept ureq's 15 second idle age; `specification/backends.md` on retries and transport failures.
- Keeps: the 0089 rule. A transport failure is never sent again, and a failed attempt still counts as one sent request. Exit codes, messages, retries, spend limits, the usage totals and `--jobs N` opening up to N connections stay the same.
- Changes: `engine/http.rs` `Client::with_roots` sets ureq's `max_idle_age` to one second, down from ureq's default of 15.
  - `specification/records.md`, section `jobs`: one sentence states the limit.
  - The dependency issue gains the idle case, its measurements and the choice left to Ian. The usage calls issue names this cause as one that fits its exit 4.
- Proof: `engine::http::tests::a_connection_idle_for_a_second_is_not_reused_and_nothing_is_sent_twice` counts connections, requests read and requests dropped at a loopback server, and the usage count. With the old 15 second age planted, it fails with `Transport(PrematureClose)`. The command-level runs below. `sdlc/scripts/test`, `spec`, workspace clippy, `policy.py`, `tickets`, `lint` in a clean checkout and the C door tests.
- Defers: a backend whose keep-alive wait is under one second plus the round trip can still close a connection under a send. Only a resend heals that, and 0089 forbids one; the choice is Ian's, recorded in the dependency issue. The HTTP/1.0 case stays in that issue too.

## Design notes

ureq probes a pooled connection before reuse and skips one whose close has arrived. The failure needs the backend's close to cross the request: the backend closes at its keep-alive limit, and the client sends before the close reaches it. Hosted servers keep idle connections for 2 to 75 seconds or more (gunicorn 2, uvicorn and Node 5, nginx 75, cloud load balancers 60 and up). Under ureq's 15 second age, any backend with a wait from 2 to 15 seconds could race. One second sits under every such default. A run with requests in flight keeps reusing its connections, since each sits idle for milliseconds. A worker idle for a second or more pays one new handshake, which costs about 60 to 90 ms on a secure connection (0142) against a wait of at least a second.

Three options were weighed.

1. Resend once on a fresh connection when a reused connection fails before any reply byte. In every failure measured here, the server never read the request. The engine cannot know that, though. A connection that closes after the request left may have reached a backend that bills it. `specification/backends.md` and ticket 0089 forbid resending any transport failure for that reason. 0089 names this cost, a pooled connection that died between ureq's probe and the write, and leaves the lever to Ian. So this ticket does not resend.
2. Upgrade ureq. ureq 3.4.2 and ureq-proto 0.6.4 are the newest in the offline registry. ureq 3.4.2 never resends a request on a stale connection.
3. Cap idle reuse. This is the change. It sends nothing more and changes no rule.

## What the build taught us

Experiment `~/workspace/experiments/417-thinkthen-stale-connection` ran the compiled command against a loopback HTTP/1.1 server. The server answers with keep-alive, closes a connection left idle for `idle_ms`, and accepts the next connection normally. A `fin_delay_ms` of 20 stands for a hosted backend's network delay: the server has closed, and a request that lands inside that window is dropped unread. Each trial ran `decide --lines --batch 1 --jobs 1 --no-cache` over two lines, paced by `THINKTHEN_REQUESTS_PER_MINUTE`, from a scratch `HOME`.

| Gap between sends | Server keep-alive | Delay | Before | After |
| --- | --- | ---: | ---: | ---: |
| 200 ms | 198 to 199 ms | 0 | 6 of 120 exit 4 | not run |
| 200 ms | 150 to 197, 200 to 250 ms | 0 | 0 of 540 | not run |
| 1.2 s | 1,185 to 1,195 ms | 20 ms | 60 of 60 exit 4 | 0 of 60 |
| 1.2 s | 1,100 or 1,250 ms | 20 ms | 0 of 40 | 0 of 40 |
| 200 ms | 185 to 195 ms | 20 ms | not run | 119 of 120 exit 4 |

- The race is real and narrow. On loopback, a close crossing the request by a millisecond or two failed 6 of 120 sends. A 20 ms network delay widens the window to 20 ms, and every send inside it failed.
- Each failure printed `the backend closed the connection before a reply and may have received the request; it was not sent again` and exited 4, the same exit the bench saw. The server read no request body for any failure, so nothing was billed twice. The server counted one request per success and none per failure: before the fix, 20 requests over 20 two-line trials at 1,190 ms; after it, 40 over 20 trials on 40 connections.
- The last row is the deferred gap: a keep-alive wait under one second still races.
