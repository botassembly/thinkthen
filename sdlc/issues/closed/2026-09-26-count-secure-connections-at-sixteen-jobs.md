# Count secure connections at sixteen jobs

Status: closed 2026-09-30 by a Quick Fix, after workspace experiment 2039. Filed 2026-09-26 by ticket 0145, the speed test. Priority entry E1 of `../../planning/issue-priorities-2026-09-30.md`.

Resolution: settled by workspace experiment 2039, a loopback count. A keep-alive TLS server on `localhost`, trusted through `THINKTHEN_CA_BUNDLE`, answered `filter --batch 1 --no-cache` over 306 lines. At `--jobs 16` it accepted 16 connections and completed 16 handshakes in each of three runs. At the default `--jobs 4` it accepted 4 in each of two runs. Each connection carried about 19 or 76 requests. The count does not exceed 16, so no ticket follows. Ticket 0142's deferred gap closes for the client. The hosted server's own keep-alive policy stays unmeasured. The CONNECT-proxy run below can measure it under `sdlc/scripts/live` if a reason appears.

Ticket 0142 set the connection pool to keep up to `--jobs` connections. Its deferred gap 1 says no gate test counts secure handshakes, because the loopback listener speaks plain HTTP. It left the count to the S1 speed test's live part, or to an authorized run of experiment 268's harness.

Ticket 0145 does not count connections. Its live part times a 306-title `filter` at `--jobs 16` and nothing more. Counting connections at the hosted service needs something that sees each new connection.

## The ask

Count the new secure connections a 306-title `filter` run opens at `--jobs 16` against the hosted service. Ticket 0142 expects about 16. Experiment 268 counted 121 to 198 before that ticket.

## A way to do it

Run a local CONNECT proxy on loopback, and point the command at it with `HTTPS_PROXY`. `specification/backends.md` says the client honours the proxy variables for an `https://` address. The proxy counts CONNECT requests and relays bytes it cannot read. It never sees the key, because the key travels inside the encrypted stream. The run needs Ian's authorization through `sdlc/scripts/live`, like any paid call.

## Open questions

- Ticket 0145's live mode builds each command's environment from scratch, so it drops `HTTPS_PROXY`. The count needs its own job, or a named exception in that mode.
- Whether the count belongs in `probes/speed/` as another measurement, or in its own probe folder.

## A loopback count first

Added 2026-09-30 from the issue priorities investigation. A loopback count answers the main question with no network. `THINKTHEN_CA_BUNDLE` (ticket 0211, `specification/backends.md`) replaces the trust roots with a local PEM file, and `crates/thinkthen/tests/ca_bundle.rs` already runs a genuine localhost TLS server with an `openssl`-made certificate authority.

Start a keep-alive TLS server on `localhost` that counts accepted connections and answers each System One request after a short hold. Run `filter --jobs 16 --batch 1 --no-cache` over 306 lines with the certificate authority in `THINKTHEN_CA_BUNDLE`, and count the accepts. The client keeps up to 32 idle connections (`Width::MOST` in `engine/mod.rs`, used by `engine/http.rs`), so it should open at most 16. A count near 16 settles ticket 0142's deferred gap for the client. It becomes a ticket only if the count exceeds 16. Only the hosted server's own keep-alive policy then remains, and the CONNECT-proxy run above can measure that later under `sdlc/scripts/live`. Ruling 13 authorizes paid calls under a token cap. Neither run needs the M5.
