# The connection pool reopens connections above three jobs

Status: Open. Filed 2026-09-26 from workspace experiment 268.

## What happens

`specification/records.md` line 135 says one pool serves every worker, so a run "pays for one handshake rather than one for each record". `Http::new` (`crates/thinkthen/src/engine/http.rs:81`) builds the ureq 3.4.2 agent without setting its idle pool size. ureq keeps 3 idle connections per host by default. With more workers than that, a finished connection often closes, and the next request opens a new one.

## Evidence

- At `--jobs 16`, 121 to 198 of 306 requests opened a new secure connection. At `--jobs 32`, up to 231 did. At `--jobs 4`, 7 to 13 did.
- A new secure connection took 60 to 90 ms at the median. Averaged over all requests, that adds about 32 ms to each 195 ms request.

## What is asked

A run should reuse its connections up to `--jobs`, as the spec says. The design belongs to this repository.
