# 0371: The TLS roots fork probe serves its own TLS and runs on macOS

Status: in progress. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-1. Issue: `sdlc/issues/2026-10-01-macos-tls-roots-fork-probe-fails.md`.

## Outcome

`conformance/consumer/fork-probe/tests/fork.rs` `a_forked_child_keeps_parsed_tls_roots_after_the_file_changes` passes on Linux and on the M5 (macOS 26.4, arm64). The fixture no longer runs the `openssl` command. It reads a fixed test CA and a `localhost` leaf from files in the test folder and serves its one HTTPS reply from a `rustls` server on a parent thread. The result no longer depends on which `openssl` is first on `PATH`, and nothing must be installed on the M5.

## Evidence

- Starts from: the issue's 20 failing runs on the M5. On 2026-10-01 a parent-only call against the same fixture failed on the M5 at `1437ae845` with "the backend refused the connection" and no request reached the responder. The certificates were made, so `req` and `x509` work under LibreSSL 3.3.6. Its `s_server` refuses the fixture's `-naccept` option, prints its usage and exits. Nothing listens on the port. The fixture's start loop does not notice the exit and waits out its five seconds. The fault is the fixture, not the macOS roots path after fork.
- Keeps: the case's proof. The parent parses the bundle, the file is overwritten, the forked child answers yes over verified TLS to `localhost`, and the responder sees exactly one `POST /systemone`. The other `fork-probe` cases are unchanged. The product code is unchanged.
- Changes: the fixture only; three parts.
  - New test fixtures `conformance/consumer/fork-probe/tests/tls/ca.pem`, `leaf.pem` and `leaf.key`: a P-256 test CA and a leaf for `DNS:localhost` signed by it, valid for 100 years. The key is test-only, guards nothing, and the file says how it was made. The case copies the CA into its temporary folder, so overwriting it leaves the tracked file alone.
  - `fork-probe` gains `rustls` as a dev-dependency, with `ring` and no default features. Both crates and their versions are already in the root and consumer locks, so the consumer lock gains no package and `policy.py` holds.
  - The `openssl` certificate generator and `s_server` responder in `fork.rs` are replaced by one in-process TLS responder. It accepts one connection, reads one HTTP request, writes the fixed reply and returns the request text.
- Proof: real runs on both machines and the repo checks.
  - Linux: `cargo test -p fork-probe --test fork` in `conformance/consumer` passes, the TLS case 10 runs of 10.
  - M5 at the branch head: the TLS case passes 10 runs of 10, and the whole `fork` test binary passes once, with `/usr/bin/openssl` (LibreSSL 3.3.6) first on `PATH`.
  - Red: on the M5 the case failed at `1437ae845` before the change, as the issue records.
  - `sdlc/scripts/lint` and `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` pass.
- Defers: `crates/thinkthen/tests/library/ca_bundle.rs` uses the same `openssl` generator and `s_server -naccept` responder, so it likely fails on macOS the same way. It runs in the crate's Linux test rung, and nothing runs it on the M5 now. A follow-up issue records it rather than widening this ticket.
