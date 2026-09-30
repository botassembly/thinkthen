# ureq reuses a connection after an HTTP/1.0 reply

Status: open. A dependency bug noted for Ian. Found by ticket 0340. Owner: upstream (ureq-proto); then a Quick Fix removes the workaround, or a ticket changes `engine/http.rs`.

Kind: debt

Pay when: ureq-proto treats an HTTP/1.0 reply without `keep-alive` as closing, or a user's backend replies over HTTP/1.0.

Debt: 020

Severity: medium

Keeping it lets one send fail as "the backend did not answer" when a server closes after each reply and says nothing.

## The problem

ureq-proto 0.6.4 (`src/client/recvresp.rs`) marks a connection for closing only when the reply carries `Connection: close`. RFC 9112 section 9.3 says an HTTP/1.0 reply without `keep-alive` ends the connection. ureq returns such a connection to its pool. If the server's close has not arrived when the next send takes the connection, the send fails. With no retries allowed, the row fails.

Python's `http.server` replies over HTTP/1.0 and closes after each reply. The DuckDB case `b13c_try_details_split_denials` sends a 413 reply and then a second request on one engine. Eight runs at a time at load 16 to 20, the case failed 11 of 200 runs, each time with the alpha half as "the backend did not answer" and no second body at the listener. A scratch copy of the case failed 10 of 480 runs as written and 0 of 480 when the server added `Connection: close`.

## Our workaround

`databases/duckdb/tools/verbs_budget.py` `PackedReplies` sends `Connection: close` on every reply. The other Python loopback fixtures (`libraries/*/fixtures/backend.py`, `libraries/python/tests/*.py`, `databases/sqlite/tests/conditional_backend.py` and others found by `grep -rl BaseHTTPRequestHandler`) reply over HTTP/1.0 without the header. They risk the same failure wherever one engine sends twice to them.

## What should happen

Either ureq-proto honors HTTP/1.0 closing, or the engine asks for no reuse after an HTTP/1.0 reply. The second is a product change to `engine/http.rs`. Until one lands, a Python fixture that serves two sends to one engine sends `Connection: close`.
