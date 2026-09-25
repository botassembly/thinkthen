# The loopback backend stalls each reply

Status: Closed on 2026-09-25 after a check against main. Quick Fix qf-loopback-nodelay merged at 365fc938 (sdlc/records/qf-loopback-nodelay.md). Earlier status: Open. Found by ticket 0086's builder, 2026-09-24.

## What happens

A request to `conformance-backend` takes about 20 to 40 ms on loopback, even on the generic arm with no hold or delay. A test that streams 100,000 records through it takes minutes. Ticket 0086's flat-memory test answers from the cache after one send to stay near 9 s.

## Likely cause

`write_answer` in `conformance/backend/src/listener.rs` writes the response head and the body in separate writes. Nagle's algorithm is on, because the listener never calls `set_nodelay`. A small second write then waits for the client's delayed acknowledgment. This is the classic stall, but nobody has measured it yet.

## Fix

Call `set_nodelay(true)` on each accepted stream, or send the head and body in one write. Measure the per-request time on the generic arm before and after. A backend test can pin an upper bound on 100 sequential requests.
