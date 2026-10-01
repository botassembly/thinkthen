# ureq reuses a connection after an HTTP/1.0 reply

Status: open. A dependency bug noted for Ian. Found by ticket 0340. Owner: upstream (ureq-proto); then a Quick Fix removes the workaround, or a ticket changes `engine/http.rs`. Ticket 0361 (batch C1) extended the workaround to every Python loopback test server, and `policy.py` holds it. The issue stays open for the upstream fix and Ian's resend choice below. No upstream ureq-proto issue is filed yet; filing one is an external change, so it waits for Ian. Remove the workaround when ureq-proto treats an HTTP/1.0 reply without `keep-alive` as closing.

Kind: debt

Pay when: ureq-proto treats an HTTP/1.0 reply without `keep-alive` as closing, or a user's backend replies over HTTP/1.0.

Debt: 020

Severity: medium

Keeping it lets one send fail as "the backend did not answer" when a server closes after each reply and says nothing.

## The problem

ureq-proto 0.6.4 (`src/client/recvresp.rs`) marks a connection for closing only when the reply carries `Connection: close`. RFC 9112 section 9.3 says an HTTP/1.0 reply without `keep-alive` ends the connection. ureq returns such a connection to its pool. If the server's close has not arrived when the next send takes the connection, the send fails. With no retries allowed, the row fails.

Python's `http.server` replies over HTTP/1.0 and closes after each reply. The DuckDB case `b13c_try_details_split_denials` sends a 413 reply and then a second request on one engine. Eight runs at a time at load 16 to 20, the case failed 11 of 200 runs, each time with the alpha half as "the backend did not answer" and no second body at the listener. A scratch copy of the case failed 10 of 480 runs as written and 0 of 480 when the server added `Connection: close`.

## Our workaround

Every tracked file that defines a `BaseHTTPRequestHandler` sends `Connection: close` on every reply. `databases/duckdb/tools/verbs_budget.py` `PackedReplies` sends it in its reply code. The others override `end_headers` to send it first (ticket 0361). A `send_error` reply then carries it twice. HTTP reads the two as one list. `sdlc/scripts/policy.py` refuses a handler file without the header.

## What should happen

Either ureq-proto honors HTTP/1.0 closing, or the engine asks for no reuse after an HTTP/1.0 reply. The second is a product change to `engine/http.rs`. Until one lands, a Python fixture that serves two sends to one engine sends `Connection: close`.

## The idle case, and the choice left to Ian

Ticket 0341 asked whether the same reuse can fail a real send. It can, through a second path. ureq probes a pooled connection before reuse and skips it when the server's close has arrived. A hosted backend closes an idle connection at its keep-alive limit. When the client sends just before that close reaches it, the send fails at exit 4 with `the backend closed the connection before a reply and may have received the request; it was not sent again`. On loopback the window was a millisecond or two, and 6 of 120 sends at the edge failed. A modeled 20 ms network delay made every send inside the window fail. The server read none of those requests.

0341 set ureq's idle age to one second, down from 15. A backend that keeps idle connections longer than one second plus the round trip can no longer race. Common server defaults run 2 seconds and up. Measured: 60 of 60 edge sends failed before, and 0 of 60 after.

Left open: a backend with a keep-alive wait under one second plus the round trip still races; 119 of 120 edge sends failed at a 200 ms wait. Only a resend heals that. `specification/backends.md` and ticket 0089 forbid resending any transport failure, because the engine cannot tell an unread request from one a backend received and may bill. 0089 names this cost and leaves the lever to Ian.

Choice for Ian:

1. Keep the rule (default). A short keep-alive backend can fail a run at exit 4. The user reruns, and `thinkthen status` shows the extra request.
2. Resend once when a reused connection closes before any reply byte, within a short time of the write. This heals the race. A backend that received the request and then dropped the connection could bill it twice. It needs a spec change and a ticket.

Recommendation: keep the rule until a user reports a backend whose keep-alive wait is under a second. No hosted backend thinkthen names is known to use one.
