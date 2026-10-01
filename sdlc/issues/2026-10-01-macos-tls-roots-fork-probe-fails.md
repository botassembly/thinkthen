Status: open. Found on the M5 on 2026-10-01 while proving ticket 0365. Owner: the queue owner.

Kind: bug

Severity: low

# The TLS roots fork probe fails on macOS

## The problem

`conformance/consumer/fork-probe/tests/fork.rs` `a_forked_child_keeps_parsed_tls_roots_after_the_file_changes` failed in every one of 20 runs on the M5 (macOS 26.4, arm64): 10 at main `f056e8cfd` and 10 at ticket 0365's branch. The child answers no instead of yes, and the test reports "forked child used the retained roots: the child exited 1". It is not the fork crash of ticket 0365. The child does not crash, and the parent asks nothing before the fork. The case passes on Linux.

The fixture makes its certificates and serves TLS with `openssl` from `PATH`. On the M5 that is `/usr/bin/openssl`, LibreSSL 3.3.6. The likely cause is a LibreSSL difference in `req`, `x509` or `s_server`. A product fault in the macOS TLS roots path remains possible until a parent-only call against the same fixture is tried.

## A fix

Run the fixture's parent call alone on the M5 first. If it fails too, the fixture needs an OpenSSL that matches Linux, or a macOS skip that gives its reason. If it passes, the fault is in the child's use of the parsed roots on macOS.
