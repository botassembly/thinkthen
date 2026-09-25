# Quick Fix qf-loopback-nodelay: answer the loopback backend without the 40 ms wait

Status: built and checked, waiting for landing. Owner: Claude. Base: `origin/main` at `f78421be`.

## Evidence it started from

The PostgreSQL port measured about 41 ms per reply on a reused connection from `conformance/backend`. `curl` saw the same delay with no engine involved.

## Cause

`write_answer` in `conformance/backend/src/listener.rs` wrote the response head and the body in two writes. With Nagle's algorithm on, the body waited until the caller acknowledged the head. The caller delays that acknowledgment by about 40 ms. The first reply on a new connection escaped the wait. Each later reply on a kept connection paid it.

## Change

`write_answer` formats the head and the body into one string and writes it once. The reply bytes are unchanged. The `content-length` still comes from `promised` when a canned reply sets it, so the length-mismatch replies stay the same.

The first version set `TCP_NODELAY` on each accepted stream, and review preferred this one. One write adds no line. It removes the cause, since no small segment is left waiting for an acknowledgment. It also keeps a whole reply in one segment, so no packet boundary falls between the head and the body. `TCP_NODELAY` left two writes in place and needed a line at each accept site.

## Proof

`curl` sent five POSTs to the `generic` arm on one reused connection. The first reply opened the connection. Times are `curl`'s `time_total` in seconds.

| Build | Reply 1, new connection | Replies 2 to 5, reused |
| --- | --- | --- |
| Before, two writes | 0.000350 | 0.040189, 0.041930, 0.040891, 0.040931 |
| After, one write, run 1 | 0.000384 | 0.000075, 0.000067, 0.000062, 0.000060 |
| After, one write, run 2 | 0.000410 | 0.000110, 0.000115, 0.000101, 0.000098 |

No test was added. A latency bound would be a timing test on a shared machine. The listener's existing tests still pin every reply byte.

## Size

`sdlc/ratchet.json` stays at 61,721.

## Checks

With `THINKTHEN_API_KEY` unset. `sdlc/scripts/live` did not run.

- `install`: exit 0.
- `lint`: exit 0. The ratchet reads 61,721.
- `test`: exit 0.
- `spec`: exit 0, `demos: 21 green, 0 red`.

## Deferred

None.
