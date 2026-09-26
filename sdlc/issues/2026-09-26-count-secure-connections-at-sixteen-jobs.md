# Count secure connections at sixteen jobs

Filed 2026-09-26 by ticket 0145, the speed test.

Ticket 0142 set the connection pool to keep up to `--jobs` connections. Its deferred gap 1 says no gate test counts secure handshakes, because the loopback listener speaks plain HTTP. It left the count to the S1 speed test's live part, or to an authorized run of experiment 268's harness.

Ticket 0145 does not count connections. Its live part times a 306-title `filter` at `--jobs 16` and nothing more. Counting connections at the hosted service needs something that sees each new connection.

## The ask

Count the new secure connections a 306-title `filter` run opens at `--jobs 16` against the hosted service. Ticket 0142 expects about 16. Experiment 268 counted 121 to 198 before that ticket.

## A way to do it

Run a local CONNECT proxy on loopback, and point the command at it with `HTTPS_PROXY`. `specification/backends.md` says the client honours the proxy variables for an `https://` address. The proxy counts CONNECT requests and relays bytes it cannot read. It never sees the key, because the key travels inside the encrypted stream. The run needs Ian's authorization through `sdlc/scripts/live`, like any paid call.

## Open questions

- Ticket 0145's live mode builds each command's environment from scratch, so it drops `HTTPS_PROXY`. The count needs its own job, or a named exception in that mode.
- Whether the count belongs in `probes/speed/` as another measurement, or in its own probe folder.
