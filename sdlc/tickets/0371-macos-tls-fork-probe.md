# 0371: The TLS roots fork probe serves its own TLS and runs on macOS

Status: COMPLETE.

Opened as: 2026-10-11. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-1. Issue: `sdlc/issues/closed/2026-10-01-macos-tls-roots-fork-probe-fails.md`.

## Outcome

`conformance/consumer/fork-probe/tests/fork.rs` `a_forked_child_keeps_parsed_tls_roots_after_the_file_changes` passes on Linux and on the M5 (macOS 26.4, arm64). The fixture no longer runs the `openssl` command. It reads a fixed test CA and a `localhost` leaf from files in the test folder and serves its one HTTPS reply from a `rustls` server on a parent thread. The result no longer depends on which `openssl` is first on `PATH`, and nothing must be installed on the M5.

## Evidence

- Starts from: the issue's 20 failing runs on the M5. On 2026-10-01 a parent-only call against the same fixture failed on the M5 at `1437ae845` with "the backend refused the connection" and no request reached the responder. The certificates were made, so `req` and `x509` work under LibreSSL 3.3.6. Its `s_server` refuses the fixture's `-naccept` option, prints its usage and exits. Nothing listens on the port. The fixture's start loop does not notice the exit and waits out its five seconds. The fixture is at fault. The macOS roots path after fork works.
- Keeps: the case's proof. The parent parses the bundle, the file is overwritten, the forked child answers yes over verified TLS to `localhost`, and the responder sees exactly one `POST /systemone`. The other `fork-probe` cases are unchanged. The product code is unchanged.
- Changes: three parts of the fixture change.
  - New test fixtures `conformance/consumer/fork-probe/tests/tls/ca.pem`, `leaf.pem` and `leaf.key`: a P-256 test CA and a leaf for `DNS:localhost` signed by it, valid for 100 years. The key is test-only and guards nothing. A comment above the constants in `fork.rs` that include them gives the `openssl` commands that made them, with their 100-year validity, so a maintainer can remake them. The PEM files hold only their blocks, because the product's bundle rule refuses other text. The case copies the CA into its temporary folder, so overwriting it leaves the tracked file alone.
  - `fork-probe` gains `rustls` as a dev-dependency: `default-features = false, features = ["ring", "std"]`. The server types need `std`, so the case does not lean on ureq turning it on. Every crate rustls needs is already in the root and consumer locks at the same versions. The consumer lock gains no package; its one changed line lists `rustls` among `fork-probe`'s dependencies. The root lock is unchanged, and `policy.py` holds.
  - The `openssl` certificate generator and `s_server` responder in `fork.rs` are replaced by one in-process TLS responder. It accepts one connection, writes the fixed reply once the request's head arrives, and returns every byte the client sent. It waits at most 60 seconds for the connection and 60 seconds for each read, then fails the case, so a broken child fails the test and does not hang it.
- Proof: real runs on both machines and the repo checks.
  - Linux: `cargo test --locked --offline -p fork-probe --test fork` in `conformance/consumer` passes. The TLS case passes 10 of 10 runs.
  - M5 at the branch head, with `/usr/bin/openssl` (LibreSSL 3.3.6) first on `PATH`: the TLS case passes 10 of 10 runs, and the whole `fork` test binary passes once.
  - Before the change: on the M5 the case and the parent-only call failed at `1437ae845`, as the issue and Starts from record.
  - `git diff --stat origin/main -- Cargo.lock conformance/consumer/Cargo.lock` shows only the one consumer line. `sdlc/scripts/lint` and `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` pass.
- Defers: `crates/thinkthen/tests/library/ca_bundle.rs` uses the same `openssl` generator and `s_server -naccept` responder, so it likely fails on macOS the same way. It runs in the crate's Linux tests, and nothing runs it on the M5 now. `sdlc/issues/2026-10-01-ca-bundle-test-needs-openssl-3.md` records it. This ticket does not widen to it.

## What the build taught us

- The fault was the fixture. LibreSSL 3.3.6's `s_server` has no `-naccept` option, so it printed its usage and exited before it listened. Its `req` and `x509` worked. The macOS roots path after fork needed no change.
- A fixed CA and leaf are simpler than making them in the test. `rcgen` is not in the root lock, and `policy.py` holds the consumer lock to the root lock's versions, so making them in Rust would have added several crates to both locks. The PEM files cannot carry a comment, because the product's bundle rule refuses text outside certificate blocks; the commands that made them live in `fork.rs`.
- The responder replies once the request's head arrives and then reads until the client closes. ureq writes the whole body before it reads, so the early reply is safe.
- A plain `cargo test` in `conformance/consumer` writes `conformance/consumer/target`, and `policy.py` then counts its generated sources against the file cap. Use `--target-dir target/consumer`, as `sdlc/scripts/test` does.
- The fixture shrank by 50 nonblank lines, so the ratchet ceiling fell from 108958 to 108908.
- Nothing must be installed on the M5. The case passes with `/usr/bin/openssl` first on `PATH`.
- Reviews: ticket review returned five findings, all fixed, then ACCEPT. Code review returned ACCEPT with two optional notes: close the issue at landing, and say "a leaf for another host" in the follow-up issue. Both are done.
- Proof run: on Linux, `fork-probe` passed in full with `--locked --offline`, and the TLS case passed 10 of 10 runs; `sdlc/scripts/lint` and `policy.py` passed. On the M5 (macOS 26.4, arm64, Rust 1.95.0, LibreSSL 3.3.6 first on `PATH`), at the branch's code commit, the TLS case passed 10 of 10 runs and the whole `fork` test binary passed once (8 passed, 1 ignored). The M5 run used a scratch folder under `/tmp` that was removed afterwards.
