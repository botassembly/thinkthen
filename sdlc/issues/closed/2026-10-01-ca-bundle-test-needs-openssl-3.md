Status: closed 2026-10-01 by ticket 0372 "Land 0372: the CA bundle library test serves its own TLS and passes on macOS". The test now reads committed certificates and serves TLS from rustls, and it passes on the M5 with LibreSSL first on `PATH`. Found on 2026-10-01 while building ticket 0371. Owner: the queue owner.

Kind: bug

Severity: low

Milestone: later

# The CA bundle library test needs OpenSSL 3 on PATH

## The problem

`crates/thinkthen/tests/library/ca_bundle.rs` makes its certificates and serves TLS with the `openssl` command from `PATH`. Its responder runs `openssl s_server -naccept 1`. On the M5, `/usr/bin/openssl` is LibreSSL 3.3.6, whose `s_server` refuses `-naccept`, prints its usage and exits. Ticket 0371 found this in the fork probe's copy of the same fixture: nothing listens, and the client reports "the backend refused the connection". This test likely fails on macOS the same way. Its start loop does notice an early exit, so it should fail fast with "local TLS responder exited before binding". It runs in the crate's Linux tests, and nothing runs it on the M5 now.

## A fix

Do what ticket 0371 did for the fork probe: commit fixed test certificates and serve TLS from a `rustls` server in the test. This test needs several certificates (two CAs and a leaf for another host), so the fixed set is larger. Then run the test once on the M5.
